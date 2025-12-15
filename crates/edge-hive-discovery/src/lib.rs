//! Edge Hive Discovery - P2P node discovery using libp2p
//!
//! Implements mDNS for local discovery and Kademlia DHT for global discovery.

use libp2p::PeerId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;
use tracing::info;

/// Errors that can occur during discovery operations
#[derive(Debug, Error)]
pub enum DiscoveryError {
    #[error("Failed to build swarm: {0}")]
    SwarmBuild(String),

    #[error("Transport error: {0}")]
    Transport(String),

    #[error("Discovery failed: {0}")]
    Discovery(String),
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

/// Discovery service managing peer discovery
pub struct DiscoveryService {
    local_peer_id: PeerId,
    peers: Arc<RwLock<HashMap<PeerId, PeerInfo>>>,
}

impl DiscoveryService {
    /// Create a new discovery service
    pub fn new() -> Result<Self, DiscoveryError> {
        // Generate a random peer ID for now
        let keypair = libp2p::identity::Keypair::generate_ed25519();
        let peer_id = keypair.public().to_peer_id();

        info!("🔍 Discovery service created with peer ID: {}", peer_id);

        Ok(Self {
            local_peer_id: peer_id,
            peers: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Get the local peer ID
    pub fn local_peer_id(&self) -> PeerId {
        self.local_peer_id
    }

    /// Get a clone of the peers map for reading
    pub fn peers(&self) -> Arc<RwLock<HashMap<PeerId, PeerInfo>>> {
        Arc::clone(&self.peers)
    }

    /// Add a discovered peer
    pub async fn add_peer(&self, peer_id: PeerId, address: String, source: DiscoverySource) {
        let mut peers = self.peers.write().await;

        let peer_info = peers.entry(peer_id).or_insert_with(|| PeerInfo {
            peer_id: peer_id.to_string(),
            addresses: vec![],
            last_seen: chrono::Utc::now(),
            source,
        });

        if !peer_info.addresses.contains(&address) {
            peer_info.addresses.push(address);
        }
        peer_info.last_seen = chrono::Utc::now();

        info!("📡 Peer added/updated: {} ({:?})", peer_id, source);
    }

    /// Get list of all known peers
    pub async fn get_peers(&self) -> Vec<PeerInfo> {
        let peers = self.peers.read().await;
        peers.values().cloned().collect()
    }

    /// Get peer count
    pub async fn peer_count(&self) -> usize {
        let peers = self.peers.read().await;
        peers.len()
    }
}

impl Default for DiscoveryService {
    fn default() -> Self {
        Self::new().expect("Failed to create discovery service")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_discovery_service() {
        let service = DiscoveryService::new();
        assert!(service.is_ok());
    }

    #[tokio::test]
    async fn test_add_peer() {
        let service = DiscoveryService::new().unwrap();
        let peer_id = PeerId::random();

        service.add_peer(peer_id, "/ip4/127.0.0.1/tcp/8080".into(), DiscoverySource::Mdns).await;

        let count = service.peer_count().await;
        assert_eq!(count, 1);
    }
}
