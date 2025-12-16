//! Real-time WebSocket server (initial implementation)

use crate::protocol::{ClientMessage, ServerMessage};
use anyhow::Result;
use dashmap::DashMap;
use futures_util::{SinkExt, StreamExt};
use std::{
    net::SocketAddr,
    sync::Arc,
};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::broadcast,
    sync::mpsc,
};
use tokio_tungstenite::{accept_async, tungstenite::Message};
use tracing::{debug, info, warn};

#[derive(Debug, Clone)]
pub struct RealtimeServerConfig {
    pub bind_addr: SocketAddr,
    pub max_subscriptions_per_conn: usize,
}

impl Default for RealtimeServerConfig {
    fn default() -> Self {
        Self {
            bind_addr: "127.0.0.1:9001".parse().expect("valid addr"),
            max_subscriptions_per_conn: 64,
        }
    }
}

/// Real-time server with topic-based broadcast channels
///
/// Note: This is the scaffolding for SurrealDB Live Query integration.
/// Right now, it supports basic subscribe/unsubscribe and server-side broadcast.
pub struct RealtimeServer {
    config: RealtimeServerConfig,
    topics: Arc<DashMap<String, broadcast::Sender<ServerMessage>>>,
}

impl RealtimeServer {
    pub fn new(config: RealtimeServerConfig) -> Self {
        Self {
            config,
            topics: Arc::new(DashMap::new()),
        }
    }

    pub async fn serve(self) -> Result<()> {
        let listener = TcpListener::bind(self.config.bind_addr).await?;
        info!("Realtime server listening on {}", self.config.bind_addr);

        loop {
            let (stream, addr) = listener.accept().await?;
            let topics = self.topics.clone();
            let max_subs = self.config.max_subscriptions_per_conn;

            tokio::spawn(async move {
                if let Err(e) = handle_connection(stream, addr, topics, max_subs).await {
                    warn!("Realtime connection error ({}): {}", addr, e);
                }
            });
        }
    }

    /// Broadcast a synthetic event to a topic (useful for tests / internal events).
    pub fn broadcast_event(&self, topic: &str, action: &str, data: serde_json::Value) {
        let sender = self.get_or_create_topic(topic);
        let _ = sender.send(ServerMessage::Event {
            topic: topic.to_string(),
            action: action.to_string(),
            data,
        });
    }

    fn get_or_create_topic(&self, topic: &str) -> broadcast::Sender<ServerMessage> {
        if let Some(entry) = self.topics.get(topic) {
            return entry.clone();
        }

        let (tx, _rx) = broadcast::channel(1024);
        self.topics.insert(topic.to_string(), tx.clone());
        tx
    }
}

async fn handle_connection(
    stream: TcpStream,
    addr: SocketAddr,
    topics: Arc<DashMap<String, broadcast::Sender<ServerMessage>>>,
    max_subs: usize,
) -> Result<()> {
    let ws_stream = accept_async(stream).await?;
    info!("WebSocket connected: {}", addr);

    let (mut ws_tx, mut ws_rx) = ws_stream.split();

    // Single writer task (SplitSink is not Clone). Everyone else sends via mpsc.
    let (out_tx, mut out_rx) = mpsc::unbounded_channel::<Message>();

    let writer = tokio::spawn(async move {
        while let Some(msg) = out_rx.recv().await {
            if ws_tx.send(msg).await.is_err() {
                break;
            }
        }
    });

    // Active subscriptions for this connection
    let subs: Arc<DashMap<String, tokio::task::JoinHandle<()>>> = Arc::new(DashMap::new());

    while let Some(msg) = ws_rx.next().await {
        let msg = msg?;

        if msg.is_close() {
            break;
        }

        if !msg.is_text() {
            continue;
        }

        let text = msg.to_text()?;
        let parsed: Result<ClientMessage, _> = serde_json::from_str(text);

        match parsed {
            Ok(ClientMessage::Ping) => {
                let _ = out_tx.send(Message::Text(serde_json::to_string(&ServerMessage::Pong)?));
            }
            Ok(ClientMessage::Subscribe { topic, filter: _ }) => {
                if subs.len() >= max_subs {
                    let err = ServerMessage::Error {
                        message: format!("Too many subscriptions (max {max_subs})"),
                    };
                    let _ = out_tx.send(Message::Text(serde_json::to_string(&err)?));
                    continue;
                }

                if subs.contains_key(&topic) {
                    let ack = ServerMessage::Ack {
                        message: format!("Already subscribed: {topic}"),
                    };
                    let _ = out_tx.send(Message::Text(serde_json::to_string(&ack)?));
                    continue;
                }

                let sender = if let Some(entry) = topics.get(&topic) {
                    entry.clone()
                } else {
                    let (tx, _rx) = broadcast::channel(1024);
                    topics.insert(topic.clone(), tx.clone());
                    tx
                };

                let mut rx = sender.subscribe();
                let out_tx_clone = out_tx.clone();
                let topic_clone = topic.clone();

                let handle = tokio::spawn(async move {
                    loop {
                        match rx.recv().await {
                            Ok(event) => {
                                if let Ok(text) = serde_json::to_string(&event) {
                                    if out_tx_clone.send(Message::Text(text)).is_err() {
                                        break;
                                    }
                                }
                            }
                            Err(broadcast::error::RecvError::Lagged(_)) => {
                                // Client is slow; drop lagged messages.
                                continue;
                            }
                            Err(_) => break,
                        }
                    }
                    debug!("Subscription task ended for topic={}", topic_clone);
                });

                subs.insert(topic.clone(), handle);

                let ack = ServerMessage::Ack {
                    message: format!("Subscribed: {topic}"),
                };
                let _ = out_tx.send(Message::Text(serde_json::to_string(&ack)?));
            }
            Ok(ClientMessage::Unsubscribe { topic }) => {
                if let Some((_, handle)) = subs.remove(&topic) {
                    handle.abort();
                    let ack = ServerMessage::Ack {
                        message: format!("Unsubscribed: {topic}"),
                    };
                    let _ = out_tx.send(Message::Text(serde_json::to_string(&ack)?));
                } else {
                    let ack = ServerMessage::Ack {
                        message: format!("Not subscribed: {topic}"),
                    };
                    let _ = out_tx.send(Message::Text(serde_json::to_string(&ack)?));
                }
            }
            Err(e) => {
                let err = ServerMessage::Error {
                    message: format!("Invalid message: {e}"),
                };
                let _ = out_tx.send(Message::Text(serde_json::to_string(&err)?));
            }
        }
    }

    // Clean up subscription tasks
    for entry in subs.iter() {
        entry.value().abort();
    }

    // Stop writer task
    drop(out_tx);
    writer.abort();

    info!("WebSocket disconnected: {}", addr);
    Ok(())
}
