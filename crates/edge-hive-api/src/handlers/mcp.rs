//! MCP JSON-RPC handler

use crate::state::ApiState;
use axum::{extract::Extension, http::StatusCode, response::Json};
use edge_hive_auth::AuthenticatedUser;
use edge_hive_mcp::{MCPRequest, MCPResponse};

/// MCP JSON-RPC endpoint (authenticated)
pub async fn json_rpc(
    Extension(state): Extension<ApiState>,
    user: AuthenticatedUser,
    Json(payload): Json<MCPRequest>,
) -> Result<Json<MCPResponse>, StatusCode> {
    let response = state.mcp_server.handle_request(payload, user).await;
    Ok(Json(response))
}
