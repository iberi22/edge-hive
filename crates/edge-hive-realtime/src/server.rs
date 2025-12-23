//! Real-time WebSocket server (initial implementation)

use crate::protocol::{ClientMessage, ServerMessage};
use anyhow::Result;
use axum::extract::ws::{Message as AxumMessage, WebSocket};
use dashmap::DashMap;
use dashmap::mapref::entry::Entry;
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
use edge_hive_db::DatabaseService;

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
#[derive(Clone)]
pub struct RealtimeServer {
    config: RealtimeServerConfig,
    topics: Arc<DashMap<String, broadcast::Sender<ServerMessage>>>,
    db: Option<Arc<DatabaseService>>,
    live_tasks: Arc<DashMap<String, tokio::task::JoinHandle<()>>>,
}

impl RealtimeServer {
    pub fn new(config: RealtimeServerConfig) -> Self {
        Self {
            config,
            topics: Arc::new(DashMap::new()),
            db: None,
            live_tasks: Arc::new(DashMap::new()),
        }
    }

    pub fn with_db(mut self, db: Arc<DatabaseService>) -> Self {
        self.db = Some(db);
        self
    }

    pub async fn serve(self) -> Result<()> {
        let listener = TcpListener::bind(self.config.bind_addr).await?;
        info!("Realtime server listening on {}", self.config.bind_addr);

        loop {
            let (stream, addr) = listener.accept().await?;
            let topics = self.topics.clone();
            let max_subs = self.config.max_subscriptions_per_conn;
            let db = self.db.clone();
            let live_tasks = self.live_tasks.clone();

            tokio::spawn(async move {
                if let Err(e) = handle_connection(stream, addr, topics, max_subs, db, live_tasks).await {
                    warn!("Realtime connection error ({}): {}", addr, e);
                }
            });
        }
    }

    /// Handle an Axum-upgraded WebSocket connection (used by the API gateway).
    pub async fn handle_axum_socket(&self, socket: WebSocket) {
        let addr: SocketAddr = "0.0.0.0:0".parse().expect("valid addr");
        let topics = self.topics.clone();
        let max_subs = self.config.max_subscriptions_per_conn;
        let db = self.db.clone();
        let live_tasks = self.live_tasks.clone();

        if let Err(e) = handle_axum_connection(socket, addr, topics, max_subs, db, live_tasks).await {
            warn!("Realtime connection error (axum): {}", e);
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
    db: Option<Arc<DatabaseService>>,
    live_tasks: Arc<DashMap<String, tokio::task::JoinHandle<()>>>,
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
                let _ = out_tx.send(Message::Text(
                    serde_json::to_string(&ServerMessage::Pong)?.into(),
                ));
            }
            Ok(ClientMessage::Subscribe { topic, filter: _ }) => {
                if subs.len() >= max_subs {
                    let err = ServerMessage::Error {
                        message: format!("Too many subscriptions (max {max_subs})"),
                    };
                    let _ = out_tx.send(Message::Text(serde_json::to_string(&err)?.into()));
                    continue;
                }

                if subs.contains_key(&topic) {
                    let ack = ServerMessage::Ack {
                        message: format!("Already subscribed: {topic}"),
                    };
                    let _ = out_tx.send(Message::Text(serde_json::to_string(&ack)?.into()));
                    continue;
                }

                let sender = if let Some(entry) = topics.get(&topic) {
                    entry.clone()
                } else {
                    let (tx, _rx) = broadcast::channel(1024);
                    topics.insert(topic.clone(), tx.clone());
                    tx
                };

                // If DB is available, ensure a single Live Query pump exists for this topic.
                if db.is_some() {
                    match live_tasks.entry(topic.clone()) {
                        Entry::Occupied(_) => {}
                        Entry::Vacant(entry) => {
                            let db = db.clone();
                            let sender = sender.clone();
                            let topic_for_task = topic.clone();

                            let handle = tokio::spawn(async move {
                                let Some(db) = db else {
                                    return;
                                };
                                match db.live_table(&topic_for_task).await {
                                    Ok(mut stream) => {
                                        while let Some(notification_result) = stream.next().await {
                                            let notification = match notification_result {
                                                Ok(n) => n,
                                                Err(e) => {
                                                    warn!(
                                                        "Live query notification error for topic={}: {}",
                                                        topic_for_task, e
                                                    );
                                                    continue;
                                                }
                                            };

                                            let action = match notification.action {
                                                surrealdb::Action::Create => "create",
                                                surrealdb::Action::Update => "update",
                                                surrealdb::Action::Delete => "delete",
                                                _ => "unknown",
                                            };

                                            let data = serde_json::to_value(&notification.data)
                                                .unwrap_or(serde_json::Value::Null);

                                            let _ = sender.send(ServerMessage::Event {
                                                topic: topic_for_task.clone(),
                                                action: action.to_string(),
                                                data,
                                            });
                                        }
                                    }
                                    Err(e) => {
                                        warn!(
                                            "Live query failed for topic={}: {}",
                                            topic_for_task, e
                                        );
                                    }
                                }
                            });

                            entry.insert(handle);
                        }
                    }
                }

                let mut rx = sender.subscribe();
                let out_tx_clone = out_tx.clone();
                let topic_clone = topic.clone();

                let handle = tokio::spawn(async move {
                    loop {
                        match rx.recv().await {
                            Ok(event) => {
                                if let Ok(text) = serde_json::to_string(&event) {
                                    if out_tx_clone.send(Message::Text(text.into())).is_err() {
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
                let _ = out_tx.send(Message::Text(serde_json::to_string(&ack)?.into()));
            }
            Ok(ClientMessage::Unsubscribe { topic }) => {
                if let Some((_, handle)) = subs.remove(&topic) {
                    handle.abort();
                    let ack = ServerMessage::Ack {
                        message: format!("Unsubscribed: {topic}"),
                    };
                    let _ = out_tx.send(Message::Text(serde_json::to_string(&ack)?.into()));
                } else {
                    let ack = ServerMessage::Ack {
                        message: format!("Not subscribed: {topic}"),
                    };
                    let _ = out_tx.send(Message::Text(serde_json::to_string(&ack)?.into()));
                }
            }
            Err(e) => {
                let err = ServerMessage::Error {
                    message: format!("Invalid message: {e}"),
                };
                let _ = out_tx.send(Message::Text(serde_json::to_string(&err)?.into()));
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

async fn handle_axum_connection(
    socket: WebSocket,
    addr: SocketAddr,
    topics: Arc<DashMap<String, broadcast::Sender<ServerMessage>>>,
    max_subs: usize,
    db: Option<Arc<DatabaseService>>,
    live_tasks: Arc<DashMap<String, tokio::task::JoinHandle<()>>>,
) -> Result<()> {
    info!("WebSocket connected (axum): {}", addr);

    let (mut ws_tx, mut ws_rx) = socket.split();

    // Single writer task (SplitSink is not Clone). Everyone else sends via mpsc.
    let (out_tx, mut out_rx) = mpsc::unbounded_channel::<AxumMessage>();

    let writer = tokio::spawn(async move {
        while let Some(msg) = out_rx.recv().await {
            if ws_tx.send(msg).await.is_err() {
                break;
            }
        }
    });

    let subs: Arc<DashMap<String, tokio::task::JoinHandle<()>>> = Arc::new(DashMap::new());

    while let Some(msg) = ws_rx.next().await {
        let msg = match msg {
            Ok(m) => m,
            Err(_) => break,
        };

        if let AxumMessage::Close(_) = msg {
            break;
        }

        let text = match msg {
            AxumMessage::Text(t) => t,
            _ => continue,
        };

        let parsed: Result<ClientMessage, _> = serde_json::from_str(&text);

        match parsed {
            Ok(ClientMessage::Ping) => {
                let _ = out_tx.send(AxumMessage::Text(
                    serde_json::to_string(&ServerMessage::Pong)?,
                ));
            }
            Ok(ClientMessage::Subscribe { topic, filter: _ }) => {
                if subs.len() >= max_subs {
                    let err = ServerMessage::Error {
                        message: format!("Too many subscriptions (max {max_subs})"),
                    };
                    let _ = out_tx.send(AxumMessage::Text(serde_json::to_string(&err)?));
                    continue;
                }

                if subs.contains_key(&topic) {
                    let ack = ServerMessage::Ack {
                        message: format!("Already subscribed: {topic}"),
                    };
                    let _ = out_tx.send(AxumMessage::Text(serde_json::to_string(&ack)?));
                    continue;
                }

                let sender = if let Some(entry) = topics.get(&topic) {
                    entry.clone()
                } else {
                    let (tx, _rx) = broadcast::channel(1024);
                    topics.insert(topic.clone(), tx.clone());
                    tx
                };

                if db.is_some() {
                    match live_tasks.entry(topic.clone()) {
                        Entry::Occupied(_) => {}
                        Entry::Vacant(entry) => {
                            let db = db.clone();
                            let sender = sender.clone();
                            let topic_for_task = topic.clone();

                            let handle = tokio::spawn(async move {
                                let Some(db) = db else {
                                    return;
                                };
                                match db.live_table(&topic_for_task).await {
                                    Ok(mut stream) => {
                                        while let Some(notification_result) = stream.next().await {
                                            let notification = match notification_result {
                                                Ok(n) => n,
                                                Err(e) => {
                                                    warn!(
                                                        "Live query notification error for topic={}: {}",
                                                        topic_for_task, e
                                                    );
                                                    continue;
                                                }
                                            };

                                            let action = match notification.action {
                                                surrealdb::Action::Create => "create",
                                                surrealdb::Action::Update => "update",
                                                surrealdb::Action::Delete => "delete",
                                                _ => "unknown",
                                            };

                                            let data = serde_json::to_value(&notification.data)
                                                .unwrap_or(serde_json::Value::Null);

                                            let _ = sender.send(ServerMessage::Event {
                                                topic: topic_for_task.clone(),
                                                action: action.to_string(),
                                                data,
                                            });
                                        }
                                    }
                                    Err(e) => {
                                        warn!(
                                            "Live query failed for topic={}: {}",
                                            topic_for_task, e
                                        );
                                    }
                                }
                            });

                            entry.insert(handle);
                        }
                    }
                }

                let mut rx = sender.subscribe();
                let out_tx_clone = out_tx.clone();
                let topic_clone = topic.clone();

                let handle = tokio::spawn(async move {
                    loop {
                        match rx.recv().await {
                            Ok(event) => {
                                if let Ok(text) = serde_json::to_string(&event) {
                                    if out_tx_clone.send(AxumMessage::Text(text)).is_err() {
                                        break;
                                    }
                                }
                            }
                            Err(broadcast::error::RecvError::Lagged(_)) => continue,
                            Err(_) => break,
                        }
                    }
                    debug!("Subscription task ended for topic={}", topic_clone);
                });

                subs.insert(topic.clone(), handle);

                let ack = ServerMessage::Ack {
                    message: format!("Subscribed: {topic}"),
                };
                let _ = out_tx.send(AxumMessage::Text(serde_json::to_string(&ack)?));
            }
            Ok(ClientMessage::Unsubscribe { topic }) => {
                if let Some((_, handle)) = subs.remove(&topic) {
                    handle.abort();
                    let ack = ServerMessage::Ack {
                        message: format!("Unsubscribed: {topic}"),
                    };
                    let _ = out_tx.send(AxumMessage::Text(serde_json::to_string(&ack)?));
                } else {
                    let ack = ServerMessage::Ack {
                        message: format!("Not subscribed: {topic}"),
                    };
                    let _ = out_tx.send(AxumMessage::Text(serde_json::to_string(&ack)?));
                }
            }
            Err(e) => {
                let err = ServerMessage::Error {
                    message: format!("Invalid message: {e}"),
                };
                let _ = out_tx.send(AxumMessage::Text(serde_json::to_string(&err)?));
            }
        }
    }

    for entry in subs.iter() {
        entry.value().abort();
    }

    drop(out_tx);
    writer.abort();

    info!("WebSocket disconnected (axum): {}", addr);
    Ok(())
}
