//! API Gateway shared state

use edge_hive_auth::{TokenGenerator, TokenValidator};
use edge_hive_cache::CacheService;
use edge_hive_db::DatabaseService;
use edge_hive_mcp::AuthenticatedMCPServer;
use edge_hive_realtime::{RealtimeServer, RealtimeServerConfig};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Shared state for all API handlers
#[derive(Clone)]
pub struct ApiState {
    /// Cache service for automatic response caching (wrapped in Arc<Mutex> for mutability)
    pub cache: Arc<Mutex<CacheService>>,

    /// Database service (SurrealDB)
    pub db: Arc<DatabaseService>,

    /// Real-time WebSocket hub
    pub realtime: RealtimeServer,

    /// Node data directory (used for loading edge function artifacts)
    pub data_dir: PathBuf,

    /// JWT token generator
    pub token_generator: Arc<TokenGenerator>,

    /// JWT token validator
    pub token_validator: Arc<TokenValidator>,

    /// MCP server
    pub mcp_server: Arc<AuthenticatedMCPServer>,
}

impl ApiState {
    /// Create a new API state
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        cache: CacheService,
        db: Arc<DatabaseService>,
        realtime: RealtimeServer,
        data_dir: PathBuf,
        token_generator: TokenGenerator,
        token_validator: TokenValidator,
    ) -> Self {
        let mcp_server = Arc::new(AuthenticatedMCPServer::new(token_validator.clone()));
        Self {
            cache: Arc::new(Mutex::new(cache)),
            db,
            realtime,
            data_dir,
            token_generator: Arc::new(token_generator),
            token_validator: Arc::new(token_validator),
            mcp_server,
        }
    }

    /// Convenience constructor for tests / minimal setups.
    pub fn new_minimal(cache: CacheService, db: Arc<DatabaseService>, data_dir: PathBuf) -> Self {
        let token_secret = "some-secret-for-testing";
        let token_generator =
            TokenGenerator::new(token_secret.as_bytes(), "edge-hive-test".to_string());
        let token_validator =
            TokenValidator::new(token_secret.as_bytes(), "edge-hive-test".to_string());

        Self::new(
            cache,
            db.clone(),
            RealtimeServer::new(RealtimeServerConfig::default()).with_db(db),
            data_dir,
            token_generator,
            token_validator,
        )
    }
}
