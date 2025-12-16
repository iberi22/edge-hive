//! API Gateway shared state

use edge_hive_cache::CacheService;
use edge_hive_realtime::{RealtimeServer, RealtimeServerConfig};
use edge_hive_db::DatabaseService;
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
}

impl ApiState {
    /// Create a new API state
    pub fn new(cache: CacheService, db: Arc<DatabaseService>, realtime: RealtimeServer) -> Self {
        Self {
            cache: Arc::new(Mutex::new(cache)),
            db,
            realtime,
        }
    }

    /// Convenience constructor for tests / minimal setups.
    pub fn new_minimal(cache: CacheService, db: Arc<DatabaseService>) -> Self {
        Self::new(
            cache,
            db.clone(),
            RealtimeServer::new(RealtimeServerConfig::default()).with_db(db),
        )
    }
}
