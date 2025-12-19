//! Edge Hive Mobile App - Tauri Backend
//!
//! Provides IPC commands for the mobile UI.

use std::sync::{Arc, Mutex};
use tauri::{Manager, Emitter, AppHandle};
use edge_hive_mcp::MCPServer;
use edge_hive_wasm::PluginManager;
use edge_hive_db::DatabaseService;

// Module Declarations
mod commands;
mod mcp_commands;
mod terminal_commands;
mod db_commands;
pub mod types; // Added
pub mod log_commands;
pub mod function_commands;
pub mod storage_commands;
pub mod chaos_commands;
pub mod vpn_commands;
pub mod auth_commands;
pub mod billing_commands;
pub mod cache_commands;
pub mod tunnel_commands;


// Use Commands & States
use log_commands::LogState;
use function_commands::FunctionState;
use storage_commands::StorageState;
use chaos_commands::{ChaosState, ChaosExperiment};
use terminal_commands::TerminalState;
use db_commands::DatabaseState;
use auth_commands::AuthState;
use billing_commands::BillingState;
use tunnel_commands::TunnelState;
use cache_commands::CacheState;

pub struct ServerState {
    pub pid: Mutex<Option<u32>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize Static States
    let mcp_server = Arc::new(MCPServer::new());
    let mcp_state = MCPState { server: mcp_server };

    let terminal_state = TerminalState {
        writer: Arc::new(Mutex::new(None)),
    };

    let log_state = LogState {
        logs: Arc::new(Mutex::new(Vec::new())),
    };

    let function_state = FunctionState {
        manager: Arc::new(Mutex::new(PluginManager::new())),
    };


    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(mcp_state)
        .manage(terminal_state)
        .manage(log_state)
        .manage(function_state)
        .manage(ServerState {
            pid: Mutex::new(None),
        })
        .manage(DatabaseState::new())
        .manage(AuthState::new())
        .manage(BillingState::new())
        .manage(TunnelState::new())
        .setup(|app| {
            let app_handle = app.handle().clone();

            // Initialize Dynamic States (Storage requires path)
            let db_path = app_handle.path().app_data_dir().unwrap().join("data.db");
            if let Some(parent) = db_path.parent() {
                let _ = std::fs::create_dir_all(parent);
                let storage_path = parent.join("storage");
                let _ = std::fs::create_dir_all(&storage_path);

                app_handle.manage(StorageState {
                    root_dir: storage_path,
                });
            }

            // Initialize Chaos State
            app_handle.manage(ChaosState {
                experiments: Arc::new(Mutex::new(vec![
                    ChaosExperiment { id: "EXP-01".into(), type_: "node_failure".into(), target: "Node:NYC-01".into(), intensity: 80, status: "idle".into(), impact_score: 0 },
                    ChaosExperiment { id: "EXP-02".into(), type_: "latency".into(), target: "Mesh:WG-Tunnel-A".into(), intensity: 45, status: "idle".into(), impact_score: 0 },
                ])),
            });

            // Initialize Async States (Cache)
            let handle_clone = app_handle.clone();
            tauri::async_runtime::block_on(async move {
                handle_clone.manage(CacheState::new().await);
            });

            // Start Background Loops
            spawn_metrics_loop(app_handle.clone());

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Core
            commands::get_node_status,
            commands::get_peers,
            commands::start_server,
            commands::stop_server,
            commands::get_cloud_nodes,
            commands::provision_cloud_node,
            commands::get_system_stats,
            // MCP
            mcp_commands::mcp_handle_request,
            mcp_commands::mcp_update_stats,
            mcp_commands::mcp_update_nodes,
            // Terminal
            terminal_commands::terminal_spawn,
            terminal_commands::terminal_write,
            terminal_commands::terminal_resize,
            // DB & Logs
            db_commands::db_query,
            db_commands::db_execute,
            log_commands::get_logs,
            log_commands::add_log,
            // Functions
            function_commands::list_functions,
            function_commands::invoke_function,
            function_commands::deploy_function,
            function_commands::get_function_versions,
            function_commands::rollback_function,
            function_commands::delete_function,
            // Storage
            storage_commands::list_buckets,
            storage_commands::list_files,
            // Chaos
            chaos_commands::get_experiments,
            chaos_commands::run_experiment,
            // VPN
            vpn_commands::get_vpn_peers,
            vpn_commands::generate_vpn_config,
            // Auth
            auth_commands::login,
            auth_commands::register,
            auth_commands::get_current_user,
            // Billing
            billing_commands::get_subscription_status,
            billing_commands::get_usage_metrics,
            billing_commands::create_checkout_session,
            // Cache
            cache_commands::get_cache_stats,
            cache_commands::clear_cache,
            cache_commands::get_cache_keys,
            // Tunnel
            tunnel_commands::start_tunnel,
            tunnel_commands::stop_tunnel,
            tunnel_commands::get_tunnel_status
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

use crate::mcp_commands::MCPState;

// Define SystemMetric locally or use serde_json::Value
#[derive(serde::Serialize, Clone)]
struct SystemMetric {
    time: String,
    cpu: f64,
    memory: f64,
    latency: f64,
}

fn spawn_metrics_loop(app: AppHandle) {
    tauri::async_runtime::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(2000));
        loop {
            interval.tick().await;

            // Generate fake metrics
            let metric = SystemMetric {
                time: chrono::Local::now().format("%H:%M:%S").to_string(),
                cpu: rand::random::<f64>() * 100.0,
                memory: rand::random::<f64>() * 100.0,
                latency: rand::random::<f64>() * 50.0,
            };

            // Emit event
            if let Err(e) = app.emit("system_metrics", &metric) {
                eprintln!("Failed to emit metrics: {}", e);
            }
        }
    });
}
