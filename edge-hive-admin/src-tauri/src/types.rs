use serde::{Deserialize, Serialize};

/// Node status for the UI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeStatus {
    pub name: String,
    pub peer_id: String,
    pub status: String,
    pub peers_count: u32,
    pub uptime_seconds: u64,
    pub tunnel_url: Option<String>,
}

/// Peer info for the UI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerInfo {
    pub peer_id: String,
    pub name: Option<String>,
    pub addresses: Vec<String>,
    pub source: String,
    pub last_seen: String,
}

/// Cloud node info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudNode {
    pub id: String,
    pub name: String,
    pub region: String,
    pub status: String,
    pub tunnel_url: Option<String>,
    pub monthly_cost: u32,
}
