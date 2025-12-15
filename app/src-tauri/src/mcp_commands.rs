use edge_hive_mcp::{DashboardStats, MCPRequest, MCPResponse, MCPServer, Node};
use std::sync::Arc;
use tauri::State;
use tokio::sync::RwLock;

pub struct MCPState {
    pub server: Arc<MCPServer>,
}

#[tauri::command]
pub async fn mcp_handle_request(
    state: State<'_, MCPState>,
    request: MCPRequest,
) -> Result<MCPResponse, String> {
    Ok(state.server.handle_request(request).await)
}

#[tauri::command]
pub async fn mcp_update_stats(
    state: State<'_, MCPState>,
    stats: DashboardStats,
) -> Result<(), String> {
    state.server.update_stats(stats).await;
    Ok(())
}

#[tauri::command]
pub async fn mcp_update_nodes(state: State<'_, MCPState>, nodes: Vec<Node>) -> Result<(), String> {
    state.server.update_nodes(nodes).await;
    Ok(())
}
