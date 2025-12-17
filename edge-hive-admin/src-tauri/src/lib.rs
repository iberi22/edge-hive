//! Edge Hive Mobile App - Tauri Backend
//!
//! Provides IPC commands for the mobile UI.

use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tauri::Manager;

mod commands;
mod mcp_commands;
mod terminal_commands;
mod db_commands;
pub mod log_commands;
pub mod function_commands;
pub mod storage_commands;
pub mod chaos_commands;
pub mod vpn_commands;

use std::sync::{Arc, Mutex};
use sysinfo::{CpuRefreshKind, RefreshKind, System};
use tauri::{Manager, Emitter}; // Added Emitter
use edge_hive_mcp::{MCPServer, MCPState};
use edge_hive_db::DatabaseService;
use edge_hive_wasm::PluginManager;
use crate::log_commands::LogState;
use crate::function_commands::FunctionState;
use crate::storage_commands::StorageState;
use crate::chaos_commands::{ChaosState, ChaosExperiment};
use mcp_commands::MCPState;
use terminal_commands::TerminalState;
use db_commands::DatabaseState;

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
    // Initialize MCP Server
    let mcp_server = Arc::new(MCPServer::new());
    let mcp_state = MCPState {
        server: mcp_server,
    };

    // Initialize Terminal State
    let terminal_state = TerminalState {
        writer: Arc::new(Mutex::new(None)),
    };

    // Initialize Log State
    let log_state = LogState {
        logs: Arc::new(Mutex::new(Vec::new())),
    };

    // Initialize Function State
    let function_state = FunctionState {
        manager: Arc::new(Mutex::new(PluginManager::new())),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
             let app_handle = app.handle();
             tauri::async_runtime::spawn(async move {
                 let db_path = app_handle.path().app_data_dir().unwrap().join("data.db");
                 // Ensure directory exists
                 if let Some(parent) = db_path.parent() {
                     let _ = std::fs::create_dir_all(parent);
                 }

                 let storage_path = parent.unwrap().join("storage");
                 let _ = std::fs::create_dir_all(&storage_path);
                 app_handle.manage(StorageState {
                    root_dir: storage_path,
                 });

                 // Initialize Chaos State
                 app_handle.manage(ChaosState {
                     experiments: Arc::new(Mutex::new(vec![
                         ChaosExperiment { id: "EXP-01".into(), type_: "node_failure".into(), target: "Node:NYC-01".into(), intensity: 80, status: "idle".into(), impact_score: 0 },
                         ChaosExperiment { id: "EXP-02".into(), type_: "latency".into(), target: "Mesh:WG-Tunnel-A".into(), intensity: 45, status: "idle".into(), impact_score: 0 },
                     ])),
                 });

                 // Start Real-time Event Loop
                 let app_handle_clone = app_handle.clone();
                 tauri::async_runtime::spawn(async move {
                     loop {
                         // Simulate System Metrics
                         let payload = serde_json::json!([{
                             "time": chrono::Local::now().format("%H:%M:%S").to_string(),
                             "cpu": (rand::random::<f32>() * 30.0 + 10.0) as u8,
                             "memory": (rand::random::<f32>() * 2000.0 + 500.0) as u32,
                             "latency": (rand::random::<f32>() * 50.0) as u32
                         }]);
                         let _ = app_handle_clone.emit("system_metrics", payload);
        .manage(mcp_state)
        .manage(terminal_state)
        .manage(DatabaseState::new())
        .manage(LogState::new())
        .manage(FunctionState::new())
        .manage(StorageState::new())
        .manage(ChaosState::new())
        .manage(AuthState::new())
        .manage(BillingState::new())
        .manage(TunnelState::new())
        .setup(|app| {
            // Async state initialization for Cache (needs async new)
            let handle = app.handle();
            tauri::async_runtime::block_on(async move {
                handle.manage(CacheState::new().await);
            });

            // Start the background emulation loop
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(2000));
                loop {
                    interval.tick().await;
                    // Emit a fake system metric event
                    let metric = edge_hive_mcp::SystemMetric {
                       time: chrono::Local::now().format("%H:%M:%S").to_string(),
                       cpu: rand::random::<f64>() * 100.0,
                       memory: rand::random::<f64>() * 100.0,
                       latency: rand::random::<f64>() * 50.0,
                    };

                    // Format: "system_metrics" payload: [metric] or metric
                    let _ = app_handle.emit("system_metrics", &metric);
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_node_status,
            commands::get_peers,
            commands::start_server,
            commands::stop_server,
            commands::get_cloud_nodes,
            commands::provision_cloud_node,
            commands::create_checkout_session,
            commands::get_system_stats,
            mcp_commands::mcp_handle_request,
            mcp_commands::mcp_update_stats,
            mcp_commands::mcp_update_nodes,
            terminal_commands::terminal_spawn,
            terminal_commands::terminal_write,
            terminal_commands::terminal_resize,
            db_commands::db_query,
            db_commands::db_execute,
            log_commands::get_logs,
            log_commands::add_log,
            function_commands::list_functions,
            function_commands::invoke_function,
            storage_commands::list_buckets,
            storage_commands::list_files,
            chaos_commands::get_experiments,
            chaos_commands::run_experiment,
            vpn_commands::get_vpn_peers,
            vpn_commands::generate_vpn_config,
            auth_commands::login,
            auth_commands::register,
            auth_commands::get_current_user,
            billing_commands::get_subscription_status,
            billing_commands::get_usage_metrics,
            billing_commands::create_checkout_session,
            cache_commands::get_cache_stats,
            cache_commands::clear_cache,
            cache_commands::get_cache_keys,
            tunnel_commands::start_tunnel,
            tunnel_commands::stop_tunnel,
            tunnel_commands::get_tunnel_status
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```
