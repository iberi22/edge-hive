//! HTTP Server module for Edge Hive

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde::Serialize;
use axum::extract::State;
use edge_hive_db::DatabaseService;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing::{error, info};

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
pub fn build_router(db: Arc<DatabaseService>) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/api/v1/node", get(node_info))
        .route("/api/v1/peers", get(get_peers))
        .with_state(db)
        .layer(CorsLayer::permissive())
}

async fn get_peers(
    State(db): State<Arc<DatabaseService>>,
) -> Result<Json<Vec<edge_hive_db::StoredPeer>>, AppError> {
    match db.get_peers().await {
        Ok(peers) => Ok(Json(peers)),
        Err(e) => {
            error!("Failed to get peers from DB: {}", e);
            Err(AppError::InternalServerError)
        }
    }
}

enum AppError {
    InternalServerError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::InternalServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
            }
        }
    }
}


/// Run the HTTP server
pub async fn run(port: u16, db: Arc<DatabaseService>) -> anyhow::Result<()> {
    let app = build_router(db);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    info!("🌐 HTTP server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
