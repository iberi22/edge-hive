use tauri::State;
use edge_hive_tunnel::{TunnelService, TunnelBackend};
use tokio::sync::Mutex;
use std::sync::Arc;
use serde::{Deserialize, Serialize};

pub struct TunnelState {
    pub service: Arc<Mutex<TunnelService>>,
}

impl TunnelState {
    pub fn new() -> Self {
        // Default to cloudflared
        Self {
            service: Arc::new(Mutex::new(TunnelService::new(TunnelBackend::Cloudflared))),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TunnelStatus {
    pub is_running: bool,
    pub public_url: Option<String>,
}

#[tauri::command]
pub async fn start_tunnel(
    state: State<'_, TunnelState>,
    port: u16
) -> Result<String, String> {
    let mut service = state.service.lock().await;
    service.start_quick(port).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn stop_tunnel(
    state: State<'_, TunnelState>
) -> Result<(), String> {
    let mut service = state.service.lock().await;
    service.stop().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_tunnel_status(
    state: State<'_, TunnelState>
) -> Result<TunnelStatus, String> {
    let service = state.service.lock().await;
    Ok(TunnelStatus {
        is_running: service.is_running(),
        public_url: service.public_url().map(|s| s.to_string()),
    })
}
