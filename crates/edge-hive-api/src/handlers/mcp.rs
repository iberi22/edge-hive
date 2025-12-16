//! MCP JSON-RPC handler

use axum::{
    extract::Extension,
    http::StatusCode,
    response::Json,
};
use serde_json::Value;
use crate::state::ApiState;

/// MCP JSON-RPC endpoint
pub async fn json_rpc(
    Extension(_state): Extension<ApiState>,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    // Placeholder: would forward to edge-hive-mcp
    Ok(Json(serde_json::json!({
        "jsonrpc": "2.0",
        "id": payload.get("id"),
        "result": {
            "message": "MCP placeholder",
        },
    })))
}
