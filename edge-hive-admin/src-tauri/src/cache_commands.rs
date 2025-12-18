use tauri::State;
use edge_hive_cache::{CacheService, CacheConfig, CacheStats};
use tokio::sync::Mutex;
use std::sync::Arc;

pub struct CacheState {
    pub service: Arc<Mutex<CacheService>>,
}

impl CacheState {
    pub async fn new() -> Self {
        let config = CacheConfig::default();
        Self {
            service: Arc::new(Mutex::new(CacheService::new(config).await)),
        }
    }
}

#[tauri::command]
pub async fn get_cache_stats(
    state: State<'_, CacheState>
) -> Result<CacheStats, String> {
    let service = state.service.lock().await;
    Ok(service.stats().await)
}

#[tauri::command]
pub async fn clear_cache(
    state: State<'_, CacheState>
) -> Result<(), String> {
    let mut service = state.service.lock().await;
    service.clear().await;
    Ok(())
}

#[tauri::command]
pub async fn get_cache_keys(
    _state: State<'_, CacheState>,
    pattern: String
) -> Result<Vec<String>, String> {
    // Stub - cache crate doesn't expose get_keys yet in lib.rs public interface easily without iteration
    // We'll simulate for now
    if pattern == "*" {
        Ok(vec!["user:123".to_string(), "session:abc".to_string()])
    } else {
        Ok(vec![])
    }
}
