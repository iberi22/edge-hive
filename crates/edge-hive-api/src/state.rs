//! API Gateway shared state

use edge_hive_cache::CacheService;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Shared state for all API handlers
#[derive(Clone)]
pub struct ApiState {
    /// Cache service for automatic response caching (wrapped in Arc<Mutex> for mutability)
    pub cache: Arc<Mutex<CacheService>>,
}

impl ApiState {
    /// Create a new API state
    pub fn new(cache: CacheService) -> Self {
        Self {
            cache: Arc::new(Mutex::new(cache)),
        }
    }
}
