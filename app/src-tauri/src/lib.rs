//! Edge Hive Mobile App - Tauri Backend
//!
//! Provides IPC commands for the mobile UI.

use serde::{Deserialize, Serialize};
use tauri::Manager;

mod commands;

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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::get_node_status,
            commands::get_peers,
            commands::start_server,
            commands::stop_server,
            commands::get_cloud_nodes,
            commands::provision_cloud_node,
            commands::create_checkout_session,
            commands::get_system_stats,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
