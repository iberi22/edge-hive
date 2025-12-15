//! Edge Hive Discovery - P2P node discovery using libp2p
//!
//! Implements mDNS for local discovery and Kademlia DHT for global discovery.

pub mod behaviour;

use anyhow::Result;
use behaviour::HiveBehaviour;
use futures::StreamExt;
use libp2p::{
    identity::Keypair,
    kad::{store::MemoryStore, Behaviour as KadBehaviour},
    mdns, noise, tcp, yamux, PeerId, Swarm, SwarmBuilder,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc, time::Duration};
use thiserror::Error;
use tokio::sync::{mpsc, RwLock};
use tracing::{error, info};

/// Errors that can occur during discovery operations
#[derive(Debug, Error)]
pub enum DiscoveryError {
    #[error("Failed to build swarm: {0}")]
    SwarmBuild(String),
    #[error("Transport error: {0}")]
    Transport(String),
    #[error("Discovery failed: {0}")]
    Discovery(String),
    #[error("Command channel closed")]
    ChannelClosed,
}

/// Information about a discovered peer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerInfo {
    pub peer_id: String,
    pub addresses: Vec<String>,
    pub last_seen: chrono::DateTime<chrono::Utc>,
    pub source: DiscoverySource,
}

/// How the peer was discovered
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum DiscoverySource {
    Mdns,
    Kademlia,
    Direct,
}

#[derive(Debug)]
enum Command {
    Dial(PeerId),
}

/// Discovery service managing peer discovery
pub struct DiscoveryService {
    swarm: Option<Swarm<HiveBehaviour>>,
    peers: Arc<RwLock<HashMap<PeerId, PeerInfo>>>,
    command_sender: mpsc::Sender<Command>,
    command_receiver: Option<mpsc::Receiver<Command>>,
    local_peer_id: PeerId,
}

impl DiscoveryService {
    /// Create a new discovery service
    pub fn new() -> Result<Self, DiscoveryError> {
        let keypair = Keypair::generate_ed25519();
        let local_peer_id = keypair.public().to_peer_id();
        info!("🔍 Local peer ID: {}", local_peer_id);

        let (command_sender, command_receiver) = mpsc::channel(10);
        let peers = Arc::new(RwLock::new(HashMap::new()));

        let swarm = SwarmBuilder::with_existing_identity(keypair)
            .with_tokio()
            .with_tcp(
                tcp::Config::default(),
                noise::Config::new,
                yamux::Config::default,
            )
            .map_err(|e| DiscoveryError::Transport(e.to_string()))?
            .with_quic()
            .with_behaviour(|key| {
                let mdns = mdns::tokio::Behaviour::new(mdns::Config::default(), key.public().to_peer_id()).unwrap();

                let mut kad_config = libp2p::kad::Config::default();
                kad_config.set_query_timeout(Duration::from_secs(5 * 60));
                let store = MemoryStore::new(key.public().to_peer_id());
                let kademlia = KadBehaviour::with_config(key.public().to_peer_id(), store, kad_config);

                let identify = libp2p::identify::Behaviour::new(libp2p::identify::Config::new(
                    "/edge-hive/1.0.0".to_string(),
                    key.public(),
                ));

                Ok(HiveBehaviour {
                    mdns,
                    kademlia,
                    identify,
                })
            })
            .map_err(|e: libp2p::swarm::SwarmBuilderError| DiscoveryError::SwarmBuild(e.to_string()))?
            .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
            .build();

        Ok(Self {
            swarm: Some(swarm),
            peers,
            command_sender,
            command_receiver: Some(command_receiver),
            local_peer_id,
        })
    }

    /// Start the discovery service event loop
    pub fn start(&mut self) -> Result<()> {
        let mut swarm = self.swarm.take().ok_or_else(|| DiscoveryError::SwarmBuild("Swarm already started".to_string()))?;
        let mut command_receiver = self.command_receiver.take().unwrap();

        swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
        swarm.listen_on("/ip4/0.0.0.0/udp/0/quic-v1".parse()?)?;

        let bootstrap_nodes: [(&str, &str); 0] = [];

        for (addr, peer_id) in bootstrap_nodes {
            if let Ok(peer_id) = peer_id.parse() {
                if let Ok(addr) = format!("/dns4/{}/tcp/9000", addr).parse() {
                    swarm
                        .behaviour_mut()
                        .kademlia
                        .add_address(&peer_id, addr);
                }
            }
        }

        if !bootstrap_nodes.is_empty() {
             swarm.behaviour_mut().kademlia.bootstrap()?;
        }

        let peers = self.peers.clone();

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    event = swarm.select_next_some() => {
                        // Event handling logic will be in the next step
                    }
                    Some(command) = command_receiver.recv() => {
                        match command {
                            Command::Dial(peer_id) => {
                                if let Err(e) = swarm.dial(peer_id) {
                                    error!("Failed to dial peer: {}", e);
                                }
                            }
                        }
                    }
                }
.                }
        });

        Ok(())
    }

    /// Get a list of discovered peers
    pub async fn peers(&self) -> Vec<PeerInfo> {
        self.peers.read().await.values().cloned().collect()
    }

    /// Dial a specific peer
    pub async fn dial(&self, peer_id: PeerId) -> Result<()> {
        self.command_sender
            .send(Command::Dial(peer_id))
            .await
            .map_err(|_| DiscoveryError::ChannelClosed)?;
        Ok(())
    }

    pub fn local_peer_id(&self) -> &PeerId {
        &self.local_peer_id
    }
}
