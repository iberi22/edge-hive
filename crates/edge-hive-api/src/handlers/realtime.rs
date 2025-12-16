//! Real-time WebSocket handlers (placeholder)

use axum::{
    extract::ws::WebSocketUpgrade,
    extract::Extension,
    http::StatusCode,
    response::IntoResponse,
    response::Json,
};
use serde::{Deserialize, Serialize};
use crate::state::ApiState;

#[derive(Deserialize)]
pub struct SubscribeRequest {
    topic: String,
}

#[derive(Serialize)]
pub struct SubscribeResponse {
    subscription_id: String,
}

/// WebSocket upgrade handler
pub async fn ws_upgrade(
    Extension(state): Extension<ApiState>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    let realtime = state.realtime.clone();
    ws.on_upgrade(move |socket| async move {
        realtime.handle_axum_socket(socket).await;
    })
}

/// Subscribe to a real-time topic
pub async fn subscribe(
    Extension(_state): Extension<ApiState>,
    Json(payload): Json<SubscribeRequest>,
) -> Result<Json<SubscribeResponse>, StatusCode> {
    // Placeholder: would create SurrealDB Live Query subscription
    Ok(Json(SubscribeResponse {
        subscription_id: format!("sub_{}", payload.topic),
    }))
}
