//! Health check handlers

use axum::{extract::Extension, response::Json};
use serde::{Deserialize, Serialize};
use crate::state::ApiState;

#[derive(Serialize)]
pub struct HealthResponse {
    status: String,
    version: String,
}

#[derive(Serialize)]
pub struct NodeInfo {
    version: String,
    cache_enabled: bool,
    features: Vec<String>,
}

/// Health check endpoint
pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

/// Node information endpoint
pub async fn node_info(Extension(_state): Extension<ApiState>) -> Json<NodeInfo> {
    Json(NodeInfo {
        version: env!("CARGO_PKG_VERSION").to_string(),
        cache_enabled: true,
        features: vec![
            "cache".to_string(),
            "database".to_string(),
            "auth".to_string(),
        ],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_check() {
        let response = health_check().await;
        assert_eq!(response.0.status, "ok");
    }
}
