//! HTTP Server module for Edge Hive

use axum::{
    routing::{get, post},
    Router,
    Json,
    extract::{State, Path},
};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;
use tracing::info;
use std::sync::Arc;
use tokio::sync::RwLock;
use edge_hive_discovery::{DiscoveryService, PeerInfo};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use crate::auth::OAuth2State;

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

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Message {
    pub to: String,
    pub from: String,
    pub body: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

pub type MessageStore = Arc<RwLock<HashMap<String, Vec<Message>>>>;

#[derive(Clone)]
pub struct AppState {
    pub discovery_svc: Arc<RwLock<DiscoveryService>>,
    pub message_store: MessageStore,
    pub data_dir: PathBuf,
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

async fn get_peers(State(state): State<AppState>) -> Json<Vec<PeerInfo>> {
    let peers = state.discovery_svc.read().await.get_peers().await;
    Json(peers)
}

async fn send_message(
    State(state): State<AppState>,
    Json(message): Json<Message>,
) -> Json<serde_json::Value> {
    let mut store = state.message_store.write().await;
    let messages = store.entry(message.to.clone()).or_default();
    messages.push(message);
    let messages_json = serde_json::to_string(&*store).unwrap();
    fs::write(state.data_dir.join("messages.json"), messages_json).unwrap();
    Json(serde_json::json!({ "status": "ok" }))
}

async fn get_messages(
    State(state): State<AppState>,
    Path(peer_id): Path<String>,
) -> Json<Vec<Message>> {
    let messages_path = state.data_dir.join("messages.json");
    let messages = if messages_path.exists() {
        let messages_json = fs::read_to_string(messages_path).unwrap();
        let store: HashMap<String, Vec<Message>> = serde_json::from_str(&messages_json).unwrap();
        store.get(&peer_id).cloned().unwrap_or_default()
    } else {
        vec![]
    };
    Json(messages)
}


/// Build the Axum router
pub fn build_router(state: AppState, oauth_state: OAuth2State) -> Router {
    // MCP auth routes
    let auth_routes = Router::new()
        .route("/mcp/auth/token", post(crate::auth::token_endpoint))
        .route("/mcp/auth/clients", post(crate::auth::create_client))
        .route("/mcp/auth/clients", get(crate::auth::list_clients))
        .route("/mcp/auth/clients/:client_id", axum::routing::delete(crate::auth::revoke_client))
        .with_state(oauth_state);

    // Main API routes
    Router::new()
        .route("/health", get(health))
        .route("/api/v1/node", get(node_info))
        .route("/api/v1/peers", get(get_peers))
        .route("/api/v1/messages", post(send_message))
        .route("/api/v1/messages/:peer_id", get(get_messages))
        .merge(auth_routes)
        .with_state(state)
        .layer(CorsLayer::permissive())
}

/// Run the HTTP server
pub async fn run(
    port: u16,
    discovery_svc: Arc<RwLock<DiscoveryService>>,
    message_store: MessageStore,
    data_dir: PathBuf,
    jwt_secret: Option<Vec<u8>>,
) -> anyhow::Result<()> {
    let state = AppState {
        discovery_svc,
        message_store,
        data_dir: data_dir.clone(),
    };

    // Initialize OAuth2 state
    let secret = jwt_secret.unwrap_or_else(|| {
        use edge_hive_auth::jwt::JwtKeys;
        JwtKeys::generate_secret()
    });
    let issuer = format!("https://localhost:{}", port);
    let oauth_state = OAuth2State::new(&secret, issuer);

    let app = build_router(state, oauth_state);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    info!("🌐 HTTP server listening on http://{}", addr);
    info!("🔐 OAuth2 token endpoint: http://{}/mcp/auth/token", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
