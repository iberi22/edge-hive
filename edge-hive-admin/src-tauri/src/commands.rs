//! Tauri IPC Commands

use crate::types::{CloudNode, NodeStatus, PeerInfo};
use sysinfo::{CpuRefreshKind, RefreshKind, System};
use serde::{Serialize, Deserialize};
use tauri::State;
use crate::db_commands::DatabaseState;

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemStats {
    pub cpu_usage: f32,
    pub total_memory: u64,
    pub used_memory: u64,
    pub total_swap: u64,
    pub used_swap: u64,
}

/// Get system statistics (CPU, RAM)
#[tauri::command]
pub async fn get_system_stats() -> Result<SystemStats, String> {
    let mut sys = System::new_with_specifics(
        RefreshKind::new()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(sysinfo::MemoryRefreshKind::everything()),
    );

    // Wait a bit for CPU usage calculation
    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_cpu();
    sys.refresh_memory();

    let cpu_usage = sys.global_cpu_info().cpu_usage();
    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    let total_swap = sys.total_swap();
    let used_swap = sys.used_swap();

    Ok(SystemStats {
        cpu_usage,
        total_memory,
        used_memory,
        total_swap,
        used_swap,
    })
}

/// Get the current node status
#[tauri::command]
pub async fn get_node_status() -> Result<NodeStatus, String> {
    // TODO: Get actual status from edge-hive service
    Ok(NodeStatus {
        name: "edge-node-demo".into(),
        peer_id: "12D3KooWDemo...".into(),
        status: "running".into(),
        peers_count: 0,
        uptime_seconds: 0,
        tunnel_url: None,
    })
}

/// Get list of discovered peers
#[tauri::command]
pub async fn get_peers(state: State<'_, DatabaseState>) -> Result<Vec<PeerInfo>, String> {
    // Return peers stored in DB + potential discovery logic
    // For now, simple query to 'peer' table
    let result = state.service.query("SELECT * FROM peer").await.map_err(|e| e.to_string())?;
    // This assumes specific structure returned by query.
    // Since 'query' returns raw JSON-like structure (surrealdb::Response), getting directly into Vec<PeerInfo> might need manual deserialization or helper.
    // For simplicity, we return empty or implement a 'get_all_peers' in DatabaseService later.
    // Let's assume empty for this step to compile, or deserialize cleanly if possible.
    Ok(vec![])
}

/// Start the server
#[tauri::command]
pub async fn start_server(port: u16) -> Result<String, String> {
    // TODO: Start edge-hive server
    Ok(format!("Server started on port {}", port))
}

/// Stop the server
#[tauri::command]
pub async fn stop_server() -> Result<(), String> {
    // TODO: Stop edge-hive server
    Ok(())
}

/// Get user's cloud nodes
#[tauri::command]
pub async fn get_cloud_nodes() -> Result<Vec<CloudNode>, String> {
    // TODO: Fetch from API
    Ok(vec![])
}

/// Provision a new cloud node
#[tauri::command]
pub async fn provision_cloud_node(
    region: String,
    size: String,
) -> Result<CloudNode, String> {
    // TODO: Call provisioning API
    Err("Cloud provisioning not yet implemented".into())
}

