//! Edge Hive API Gateway
//!
//! Unified API layer for all Edge Hive services with middleware stack:
//! - CORS: Cross-origin resource sharing
//! - Auth: JWT validation
//! - Cache: Automatic cache integration
//! - Tracing: Request/response logging
//!
//! ## Architecture
//!
//! ```text
//! Client → CORS → Auth → Cache → Router → Services
//! ```
//!
//! ## Routes
//!
//! - `/api/v1/health` - Health check
//! - `/api/v1/data/*` - Database operations
//! - `/api/v1/auth/*` - Authentication
//! - `/api/v1/edge/*` - Edge functions (WASM)
//! - `/api/v1/realtime` - WebSocket upgrade
//! - `/api/v1/mcp` - MCP JSON-RPC
//! - `/api/v1/mcp/auth/token` - MCP OAuth2 token endpoint

use axum::{
    routing::{get, post, put, delete},
    Router,
};
use tower::ServiceBuilder;
use tower_http::{
    cors::{CorsLayer, Any},
    trace::TraceLayer,
};
use tracing::info;
use edge_hive_auth::AuthLayer;

pub mod handlers;
pub mod middleware;
pub mod state;
pub mod handlers;

pub use state::ApiState;

/// Create the API router with all routes and middleware
pub fn create_router(state: ApiState) -> Router {
    info!("Creating API Gateway router");

    let auth_layer = AuthLayer::new(state.token_validator.as_ref().clone());

    // Core routes
    let health_routes = Router::new()
        .route("/api/v1/health", get(handlers::health::health_check))
        .route("/api/v1/info", get(handlers::health::node_info));

    // Database routes (auto-cached)
    let data_routes = Router::new()
        .route("/api/v1/data/:table", get(handlers::data::query_records))
        .route("/api/v1/data/:table", post(handlers::data::insert_record))
        .route("/api/v1/data/:table/:id", put(handlers::data::update_record))
        .route("/api/v1/data/:table/:id", delete(handlers::data::delete_record));

    // Auth routes
    let auth_routes = Router::new()
        .route("/api/v1/auth/register", post(handlers::auth::register))
        .route("/api/v1/auth/login", post(handlers::auth::login))
        .route("/api/v1/auth/refresh", post(handlers::auth::refresh_token))
        .route("/api/v1/auth/logout", get(handlers::auth::logout));

    // Edge functions routes (placeholder for future WASM integration)
    let wasm_routes = Router::new()
        .route("/api/v1/edge", get(handlers::wasm::list_wasm_functions))
        .route("/api/v1/edge/:function_name", post(handlers::wasm::deploy_function))
        .route("/api/v1/edge/:function_name/versions", get(handlers::wasm::get_function_versions))
        .route("/api/v1/edge/:function_name/rollback", post(handlers::wasm::rollback_function))
        .route("/api/v1/edge/:function_name", delete(handlers::wasm::delete_function))
        .route("/api/v1/edge/:function_name/execute", post(handlers::wasm::execute_wasm_function));

    // Real-time routes (placeholder for WebSocket)
    let realtime_routes = Router::new()
        .route("/api/v1/realtime", get(handlers::realtime::ws_upgrade))
        .route("/api/v1/realtime/subscribe", post(handlers::realtime::subscribe));

    // MCP routes (with OAuth2 token endpoint)
    let mcp_routes = Router::new()
        .route("/api/v1/mcp", post(handlers::mcp::json_rpc))
        .route_layer(
            ServiceBuilder::new()
                .layer(axum::middleware::from_fn(AuthLayer::middleware))
        )
        .route("/api/v1/mcp/auth/token", post(handlers::mcp_auth::issue_mcp_token));


    // Combine all routes
    Router::new()
        .merge(health_routes)
        .merge(data_routes)
        .merge(auth_routes)
        .merge(wasm_routes)
        .merge(realtime_routes)
        .merge(mcp_routes)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use edge_hive_cache::CacheConfig;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_router_creation() {
        let cache = edge_hive_cache::CacheService::new(CacheConfig::default()).await;
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let db = std::sync::Arc::new(edge_hive_db::DatabaseService::new(&db_path).await.unwrap());
        let state = ApiState::new_minimal(cache, db, dir.path().to_path_buf());
        let router = create_router(state);

        // Router should be created without panic
        assert!(true);
    }
}
