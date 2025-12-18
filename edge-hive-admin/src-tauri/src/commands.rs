//! Tauri IPC Commands

use crate::types::{CloudNode, NodeStatus, PeerInfo};
use crate::ServerState;
use sysinfo::{CpuRefreshKind, Pid, RefreshKind, System};
use serde::{Serialize, Deserialize};
use tauri::{State, Manager};
use tauri_plugin_shell::ShellExt;
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
pub async fn get_node_status(state: tauri::State<'_, ServerState>) -> Result<NodeStatus, String> {
    let pid_lock = state.pid.lock().unwrap();
    if let Some(pid_val) = *pid_lock {
        let s = System::new_with_specifics(
            RefreshKind::new().with_processes(sysinfo::ProcessRefreshKind::new()),
        );
        if let Some(process) = s.process(Pid::from(pid_val as usize)) {
            return Ok(NodeStatus {
                name: process.name().to_string(),
                peer_id: format!("PID: {}", pid_val),
                status: "running".to_string(),
                peers_count: 0, // Placeholder
                uptime_seconds: process.run_time(),
                tunnel_url: None, // Placeholder
            });
        }
    }

    Ok(NodeStatus {
        name: "edge-node".into(),
        peer_id: "N/A".into(),
        status: "stopped".into(),
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
pub async fn start_server(
    app: tauri::AppHandle,
    state: tauri::State<'_, ServerState>,
    port: u16,
) -> Result<String, String> {
    let sidecar = app
        .shell()
        .sidecar("edge-hive-core")
        .map_err(|e| e.to-string())?;
    let (mut rx, child) = sidecar
        .args(["--port", &port.to_string()])
        .spawn()
        .map_err(|e| e.to_string())?;
    let pid = child.pid();
    *state.pid.lock().unwrap() = Some(pid);
    // Optional: Log output from sidecar
    tokio::spawn(async move {
        while let Some(event) = rx.recv().await {
            match event {
                tauri_plugin_shell::Event::Stdout(line) => {
                    println!("[sidecar] {}", line);
                }
                tauri_plugin_shell::Event::Stderr(line) => {
                    eprintln!("[sidecar] {}", line);
                }
                _ => {}
            }
        }
    });
    Ok(format!("Server started with PID {}", pid))
}

/// Stop the server
#[tauri::command]
pub async fn stop_server(state: tauri::State<'_, ServerState>) -> Result<(), String> {
    let mut pid_lock = state.pid.lock().unwrap();
    if let Some(pid_val) = *pid_lock {
        let s =
            System::new_with_specifics(RefreshKind::new().with_processes(sysinfo::ProcessRefreshKind::new()));
        if let Some(process) = s.process(Pid::from(pid_val as usize)) {
            if !process.kill() {
                return Err(format!("Failed to kill process with PID {}", pid_val));
            }
        }
        *pid_lock = None;
    }
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

