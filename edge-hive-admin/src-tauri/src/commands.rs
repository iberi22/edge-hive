//! Tauri IPC Commands

use crate::types::{NodeStatus, PeerInfo};
use crate::ServerState;
use sysinfo::{Pid, System};
use serde::{Serialize, Deserialize};
use tauri::State;
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
        sysinfo::RefreshKind::everything()
            .with_cpu(sysinfo::CpuRefreshKind::everything())
            .with_memory(sysinfo::MemoryRefreshKind::everything()),
    );

    // Wait a bit for CPU usage calculation
    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_cpu_all();
    sys.refresh_memory();

    let cpu_usage = sys.global_cpu_usage();
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
        let mut s = System::new_with_specifics(
            sysinfo::RefreshKind::everything().with_processes(sysinfo::ProcessRefreshKind::everything()),
        );
        s.refresh_processes();
        if let Some(process) = s.process(Pid::from(pid_val as usize)) {
            return Ok(NodeStatus {
                name: process.name().to_string_lossy().into_owned(),
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
    let result = state.db_service.get_peers().await.map_err(|e| e.to_string())?;
    Ok(result.into_iter().map(|p| PeerInfo {
        peer_id: p.peer_id,
        name: p.name,
        addresses: p.addresses,
        last_seen: p.last_seen.to_string(),
        source: "database".to_string(),
    }).collect())
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
        .map_err(|e| e.to_string())?;
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
                tauri_plugin_shell::process::CommandEvent::Stdout(line) => {
                    let line_str = String::from_utf8_lossy(&line);
                    println!("[sidecar] {}", line_str);
                }
                tauri_plugin_shell::process::CommandEvent::Stderr(line) => {
                    let line_str = String::from_utf8_lossy(&line);
                    eprintln!("[sidecar] {}", line_str);
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
        let mut s = System::new_with_specifics(
            sysinfo::RefreshKind::everything().with_processes(sysinfo::ProcessRefreshKind::everything()),
        );
        s.refresh_processes();
        if let Some(process) = s.process(Pid::from(pid_val as usize)) {
            if !process.kill() {
                return Err(format!("Failed to kill process with PID {}", pid_val));
            }
        }
        *pid_lock = None;
    }
    Ok(())
}


