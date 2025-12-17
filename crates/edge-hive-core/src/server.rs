//! HTTP Server module for Edge Hive

use axum::{
    routing::{get, post},
    Router,
    Json,
    extract::{Extension, Path, Request},
    response::{IntoResponse, sse::{Event, Sse}},
    http::{StatusCode, header},
};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::set_status::SetStatus;
use tracing::info;
use std::sync::Arc;
use tokio::sync::RwLock;
use edge_hive_discovery::{DiscoveryService, PeerInfo};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use crate::auth::{OAuth2State, verify_bearer_token};
use futures::{stream::Stream, StreamExt};
use std::convert::Infallible;
use std::time::Duration;

async fn serve_static_file(uri: axum::http::Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');

    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let static_root = cwd.join("app/dist");

    tracing::info!("Static file request: path={}, cwd={:?}, root={:?}", path, cwd, static_root);

    // If path is empty, serve index.html
    let file_path = if path.is_empty() || path == "index.html" {
        static_root.join("index.html")
    } else {
        static_root.join(path)
    };

    match tokio::fs::read(&file_path).await {
        Ok(contents) => {
            let mime = mime_guess::from_path(&file_path)
                .first_or_octet_stream()
                .to_string();
            (
                [(header::CONTENT_TYPE, mime)],
                contents
            ).into_response()
        }
        Err(_) => {
            // Fallback to index.html for SPA routing
            match tokio::fs::read(static_root.join("index.html")).await {
                Ok(contents) => (
                    [(header::CONTENT_TYPE, "text/html")],
                    contents
                ).into_response(),
                Err(_) => (
                    StatusCode::NOT_FOUND,
                    "404 Not Found"
                ).into_response()
            }
        }
    }
}

fn spa_static_service() -> ServeDir<SetStatus<ServeFile>> {
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let from_cwd = cwd.join("app/dist");
    let from_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../app/dist");

    let static_dir = if from_cwd.exists() {
        from_cwd
    } else if from_manifest.exists() {
        from_manifest
    } else {
        // Fall back to the expected relative path.
        PathBuf::from("app/dist")
    };

    let index_html = static_dir.join("index.html");
    info!(
        "📁 Static root: {} (index.html exists: {})",
        static_dir.display(),
        index_html.exists()
    );

    ServeDir::new(static_dir)
        .append_index_html_on_directories(true)
        .not_found_service(ServeFile::new(index_html))
}

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

async fn get_peers(Extension(state): Extension<AppState>) -> Json<Vec<PeerInfo>> {
    let peers = state.discovery_svc.read().await.get_peers().await;
    Json(peers)
}

async fn send_message(
    Extension(state): Extension<AppState>,
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
    Extension(state): Extension<AppState>,
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

/// SSE streaming endpoint for MCP notifications
async fn mcp_stream(
    Extension(oauth_state): Extension<OAuth2State>,
    request: Request,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, StatusCode> {
    // Verify Bearer token
    let auth_header = request.headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if !auth_header.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = &auth_header[7..];
    let claims = verify_bearer_token(token, &oauth_state.jwt_secret)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let client_id = claims.sub.clone(); // Clone for move into closure
    info!("SSE stream established for client: {}", client_id);

    // Create SSE stream with keep-alive
    let stream = futures::stream::iter(0..)
        .then(move |_| {
            let client_id = client_id.clone();
            async move {
                tokio::time::sleep(Duration::from_secs(30)).await;
                let event_data = serde_json::json!({
                    "type": "notification",
                    "timestamp": chrono::Utc::now().to_rfc3339(),
                    "data": {
                        "status": "connected",
                        "client_id": client_id,
                    }
                });
                Event::default()
                    .event("mcp-notification")
                    .data(event_data.to_string())
            }
        })
        .map(Ok);

    Ok(Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text("keep-alive"),
    ))
}

/// MCP tool call endpoint
async fn mcp_tool_call(
    Extension(state): Extension<AppState>,
    Extension(oauth_state): Extension<OAuth2State>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Verify Bearer token
    let auth_header = headers
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if !auth_header.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = &auth_header[7..];
    verify_bearer_token(token, &oauth_state.jwt_secret)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Handle tool call based on method
    let method = payload.get("method").and_then(|m| m.as_str()).unwrap_or("");
    let params = payload.get("params");

    let result = match method {
        "tools/call" => {
            let name = params
                .and_then(|p| p.get("name"))
                .and_then(|n| n.as_str())
                .unwrap_or("");

            match name {
                "get_status" => serde_json::json!({
                    "content": [{
                        "type": "text",
                        "text": "Node: edge-hive-node\nStatus: Running\nUptime: 1234s"
                    }]
                }),
                "edge_function_create" => {
                    let name = params
                        .and_then(|p| p.get("arguments"))
                        .and_then(|a| a.get("name"))
                        .and_then(|n| n.as_str())
                        .unwrap_or("");

                    let template = params
                        .and_then(|p| p.get("arguments"))
                        .and_then(|a| a.get("template"))
                        .cloned()
                        .unwrap_or_else(|| serde_json::json!({}));

                    let valid_name = !name.is_empty()
                        && name
                            .chars()
                            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_');

                    if !valid_name {
                        serde_json::json!({
                            "error": {
                                "code": -32602,
                                "message": "Invalid function name (use [A-Za-z0-9_-])"
                            }
                        })
                    } else {
                        let functions_dir = state.data_dir.join("edge-functions");
                        if let Err(e) = fs::create_dir_all(&functions_dir) {
                            serde_json::json!({
                                "error": {"code": -32000, "message": format!("Failed to create edge-functions dir: {}", e)}
                            })
                        } else {
                            let path = functions_dir.join(format!("{}.json", name));
                            let bytes = match serde_json::to_vec_pretty(&template) {
                                Ok(b) => b,
                                Err(e) => {
                                    return Ok(Json(serde_json::json!({
                                        "error": {"code": -32000, "message": format!("Failed to serialize template: {}", e)}
                                    })));
                                }
                            };

                            match fs::write(&path, bytes) {
                                Ok(_) => serde_json::json!({
                                    "content": [{
                                        "type": "text",
                                        "text": format!("Edge function '{}' created at {}", name, path.display())
                                    }]
                                }),
                                Err(e) => serde_json::json!({
                                    "error": {"code": -32000, "message": format!("Failed to write function file: {}", e)}
                                }),
                            }
                        }
                    }
                }
                "edge_function_list" => {
                    let functions_dir = state.data_dir.join("edge-functions");
                    let mut names: Vec<String> = Vec::new();

                    if let Ok(entries) = fs::read_dir(&functions_dir) {
                        for entry in entries.flatten() {
                            let path = entry.path();
                            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                                continue;
                            }
                            if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                                names.push(stem.to_string());
                            }
                        }
                    }
                    names.sort();

                    serde_json::json!({
                        "content": [{
                            "type": "text",
                            "text": serde_json::json!({"functions": names}).to_string()
                        }]
                    })
                }
                "provision_node" => {
                    let node_name = params
                        .and_then(|p| p.get("arguments"))
                        .and_then(|a| a.get("name"))
                        .and_then(|n| n.as_str())
                        .unwrap_or("unknown-node");

                    serde_json::json!({
                        "content": [{
                            "type": "text",
                            "text": format!("Provisioning node: {}", node_name)
                        }]
                    })
                },
                _ => serde_json::json!({
                    "error": {
                        "code": -32601,
                        "message": "Tool not found"
                    }
                }),
            }
        },
        _ => serde_json::json!({
            "error": {
                "code": -32601,
                "message": "Method not found"
            }
        }),
    };

    Ok(Json(result))
}

/// MCP resource read endpoint
async fn mcp_resource_read(
    Extension(oauth_state): Extension<OAuth2State>,
    Path(resource_uri): Path<String>,
    headers: axum::http::HeaderMap,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Verify Bearer token
    let auth_header = headers
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if !auth_header.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = &auth_header[7..];
    verify_bearer_token(token, &oauth_state.jwt_secret)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Handle resource read
    let result = match resource_uri.as_str() {
        "edge-hive://status" => serde_json::json!({
            "uri": "edge-hive://status",
            "mimeType": "application/json",
            "text": serde_json::to_string(&serde_json::json!({
                "status": "running",
                "version": env!("CARGO_PKG_VERSION"),
                "uptime": 1234
            })).unwrap()
        }),
        "edge-hive://logs/last" => serde_json::json!({
            "uri": "edge-hive://logs/last",
            "mimeType": "text/plain",
            "text": "[INFO] Server started\n[INFO] MCP server ready"
        }),
        _ => serde_json::json!({
            "error": {
                "code": -32602,
                "message": "Resource not found"
            }
        }),
    };

    Ok(Json(result))
}


/// Build the Axum router
pub fn build_router() -> Router {
    // MCP auth routes (no auth required for token endpoint)
    let auth_routes = Router::new()
        .route("/mcp/auth/token", post(crate::auth::token_endpoint))
        .route("/mcp/auth/clients", post(crate::auth::create_client))
        .route("/mcp/auth/clients", get(crate::auth::list_clients))
        .route("/mcp/auth/clients/:client_id", axum::routing::delete(crate::auth::revoke_client));

    // MCP streaming and tool routes (require OAuth2 Bearer token)
    let mcp_routes = Router::new()
        .route("/mcp/stream", get(mcp_stream))
        .route("/mcp/tools/call", post(mcp_tool_call))
        // Use a wildcard so URIs like `edge-hive://status` (contains `/`) can be passed directly.
        .route("/mcp/resources/*uri", get(mcp_resource_read));

    // Main API routes
    Router::new()
        .route("/health", get(health))
        .route("/api/v1/node", get(node_info))
        .route("/api/v1/peers", get(get_peers))
        .route("/api/v1/messages", post(send_message))
        .route("/api/v1/messages/:peer_id", get(get_messages))
        .merge(auth_routes)
        .merge(mcp_routes)
}

/// Run the HTTP/HTTPS server
pub async fn run(
    port: u16,
    discovery_svc: Arc<RwLock<DiscoveryService>>,
    message_store: MessageStore,
    data_dir: PathBuf,
    jwt_secret: Option<Vec<u8>>,
    enable_https: bool,
    hostname: String,
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

    let protocol = if enable_https { "https" } else { "http" };
    let issuer = format!("{}://{}:{}", protocol, hostname, port);
    let oauth_state = OAuth2State::load_or_new(&secret, issuer, &data_dir).await?;

    // API gateway components (shared DB + cache + realtime)
    let cache = edge_hive_cache::CacheService::new(edge_hive_cache::CacheConfig::default()).await;
    let db_path = data_dir.join("edge-hive.db");
    let db = Arc::new(edge_hive_db::DatabaseService::new(&db_path).await?);
    let realtime = edge_hive_realtime::RealtimeServer::new(edge_hive_realtime::RealtimeServerConfig::default())
        .with_db(db.clone());
    let api_state = edge_hive_api::ApiState::new(cache, db, realtime, data_dir.clone());
    let api_router = edge_hive_api::create_router(api_state);

    // MINIMAL TEST - Remove api_router temporarily to isolate issue
    let app = build_router()
        .fallback_service(spa_static_service())
        .layer(axum::Extension(state))
        .layer(axum::Extension(oauth_state));

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));

    // Friendly display host: use localhost when bound to 0.0.0.0 or :: to make URLs easy to open in a browser
    let display_host = if addr.ip().is_unspecified() {
        "localhost".to_string()
    } else {
        addr.ip().to_string()
    };
    let display_addr = format!("{}:{}", display_host, addr.port());

    if enable_https {
        // HTTPS mode with TLS
        use crate::tls::TlsCertificate;
        use axum_server::tls_rustls::RustlsConfig;

        // rustls 0.23 requires selecting a process-level CryptoProvider.
        // This must happen before any TLS config is constructed.
        let _ = rustls::crypto::ring::default_provider().install_default();

        let tls_cert = TlsCertificate::load_or_generate(&data_dir, &hostname)?;

        // Build RustlsConfig from certificate files
        let tls_config = RustlsConfig::from_pem_file(
            &tls_cert.cert_path,
            &tls_cert.key_path,
        ).await?;

        info!("🔒 HTTPS server listening on https://{} (bound to {})", display_addr, addr);
        info!("🔐 OAuth2 token endpoint: https://{}/mcp/auth/token", display_addr);
        info!("⚠️  Using self-signed certificate (for testing only)");

        axum_server::bind_rustls(addr, tls_config)
            .serve(app.into_make_service())
            .await?;
    } else {
        // HTTP mode (default)
        info!("🌐 HTTP server listening on http://{} (bound to {})", display_addr, addr);
        info!("🔐 OAuth2 token endpoint: http://{}/mcp/auth/token", display_addr);
        info!("💡 Tip: Use --https for HTTPS/TLS support");

        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, app).await?;
    }

    Ok(())
}
