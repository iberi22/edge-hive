//! HTTP Server module for Edge Hive

use axum::{
    routing::get,
    Router,
    Json,
};
use serde::Serialize;
use tower_http::cors::CorsLayer;
use tracing::info;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    version: &'static str,
}

#[derive(Serialize)]
struct NodeInfo {
    peer_id: String,
    name: String,
    version: &'static str,
    uptime_seconds: u64,
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        version: env!("CARGO_PKG_VERSION"),
    })
}

async fn node_info() -> Json<NodeInfo> {
    // TODO: Get real node info from identity service
    Json(NodeInfo {
        peer_id: "placeholder".into(),
        name: "edge-hive-node".into(),
        version: env!("CARGO_PKG_VERSION"),
        uptime_seconds: 0,
    })
}

/// Build the Axum router
pub fn build_router() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/api/v1/node", get(node_info))
        .layer(CorsLayer::permissive())
}

/// Run the HTTP server
pub async fn run(port: u16) -> anyhow::Result<()> {
    let app = build_router();

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    info!("🌐 HTTP server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
