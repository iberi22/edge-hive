//! API Gateway shared state

use edge_hive_cache::CacheService;
use edge_hive_realtime::{RealtimeServer, RealtimeServerConfig};
use edge_hive_db::DatabaseService;
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
}

impl ApiState {
    /// Create a new API state
    pub fn new(
        cache: CacheService,
        db: Arc<DatabaseService>,
        realtime: RealtimeServer,
        data_dir: PathBuf,
    ) -> Self {
        Self {
            cache: Arc::new(Mutex::new(cache)),
            db,
            realtime,
            data_dir,
        }
    }

    /// Convenience constructor for tests / minimal setups.
    pub fn new_minimal(cache: CacheService, db: Arc<DatabaseService>, data_dir: PathBuf) -> Self {
        Self::new(
            cache,
            db.clone(),
            RealtimeServer::new(RealtimeServerConfig::default()).with_db(db),
            data_dir,
        )
    }
}
