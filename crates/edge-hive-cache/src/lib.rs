//! Edge Hive Cache - High-performance caching layer
//!
//! Provides a two-tier caching system:
//! - L1: In-memory cache (moka) - nanosecond access
//! - L2: Redis-compatible cache (mini-redis) - microsecond access
//!
//! # Performance
//!
//! - L1 Cache: < 1ms latency
//! - L2 Cache: < 5ms latency
//! - 100x faster than database queries
//!
//! # Example
//!
//! ```rust
//! use edge_hive_cache::{CacheService, CacheConfig};
//!
//! #[tokio::main]
//! async fn main() {
//!     let config = CacheConfig::default();
//!     let cache = CacheService::new(config);
//!
//!     // Write to cache
//!     cache.set("user:1".to_string(), b"John Doe".to_vec()).await;
//!
//!     // Read from cache
//!     let value = cache.get("user:1").await;
//!     assert_eq!(value, Some(b"John Doe".to_vec()));
//! }
//! ```

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;
use tracing::{debug, info, warn};

mod l1_cache;
mod l2_cache;
mod metrics;

pub use l1_cache::L1Cache;
pub use l2_cache::L2Cache;
pub use metrics::CacheMetrics;

/// Cache errors
#[derive(Debug, Error)]
pub enum CacheError {
    #[error("Cache initialization error: {0}")]
    Init(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("L2 cache error: {0}")]
    L2Error(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
}

/// Cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// Maximum number of entries in L1 cache
    pub l1_max_capacity: u64,

    /// Default TTL for L1 cache entries (seconds)
    pub l1_ttl_secs: u64,

    /// Enable L2 cache (mini-redis)
    pub l2_enabled: bool,

    /// L2 cache port (default: 6379)
    pub l2_port: u16,

    /// L2 cache host
    pub l2_host: String,

    /// Enable metrics collection
    pub metrics_enabled: bool,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            l1_max_capacity: 10_000,
            l1_ttl_secs: 300, // 5 minutes
            l2_enabled: false,
            l2_port: 6379,
            l2_host: "127.0.0.1".to_string(),
            metrics_enabled: true,
        }
    }
}

/// Main cache service with two-tier architecture
pub struct CacheService {
    l1: L1Cache,
    l2: Option<L2Cache>,
    metrics: Arc<CacheMetrics>,
}

impl CacheService {
    /// Create a new cache service
    pub async fn new(config: CacheConfig) -> Self {
        info!(
            "Initializing cache service (L1: {} entries, TTL: {}s, L2: {})",
            config.l1_max_capacity, config.l1_ttl_secs, config.l2_enabled
        );

        let l1 = L1Cache::new(config.l1_max_capacity, config.l1_ttl_secs);

        let l2 = if config.l2_enabled {
            let addr = format!("{}:{}", config.l2_host, config.l2_port);
            match L2Cache::connect(&addr).await {
                Ok(cache) => {
                    info!("L2 cache connected: {}", addr);
                    Some(cache)
                }
                Err(e) => {
                    warn!("Failed to connect L2 cache: {}", e);
                    None
                }
            }
        } else {
            None
        };

        let metrics = if config.metrics_enabled {
            Arc::new(CacheMetrics::new())
        } else {
            Arc::new(CacheMetrics::disabled())
        };

        Self { l1, l2, metrics }
    }

    /// Get a value from cache (tries L1 first, then L2)
    pub async fn get(&mut self, key: &str) -> Option<Vec<u8>> {
        // Try L1 first
        if let Some(value) = self.l1.get(key).await {
            debug!("L1 cache hit: {}", key);
            self.metrics.record_hit(1);
            return Some(value);
        }

        // Try L2 if enabled
        if let Some(ref mut l2) = self.l2 {
            if let Ok(Some(value)) = l2.get(key).await {
                debug!("L2 cache hit: {}", key);
                // Populate L1 for faster access next time
                self.l1.set(key.to_string(), value.clone()).await;
                self.metrics.record_hit(2);
                return Some(value);
            }
        }

        debug!("Cache miss: {}", key);
        self.metrics.record_miss();
        None
    }

    /// Set a value in cache (writes to both L1 and L2)
    pub async fn set(&mut self, key: String, value: Vec<u8>) {
        // Always write to L1
        self.l1.set(key.clone(), value.clone()).await;

        // Write to L2 if enabled
        if let Some(ref mut l2) = self.l2 {
            if let Err(e) = l2.set(&key, value).await {
                warn!("Failed to write to L2 cache: {}", e);
            }
        }

        self.metrics.record_write();
    }

    /// Set a value with custom TTL
    pub async fn set_with_ttl(&mut self, key: String, value: Vec<u8>, ttl_secs: u64) {
        self.l1.set_with_ttl(key.clone(), value.clone(), ttl_secs).await;

        if let Some(ref mut l2) = self.l2 {
            if let Err(e) = l2.set_with_ttl(&key, value, ttl_secs).await {
                warn!("Failed to write to L2 cache with TTL: {}", e);
            }
        }

        self.metrics.record_write();
    }

    /// Delete a key from cache
    pub async fn delete(&mut self, key: &str) {
        self.l1.delete(key).await;

        if let Some(ref mut l2) = self.l2 {
            if let Err(e) = l2.delete(key).await {
                warn!("Failed to delete from L2 cache: {}", e);
            }
        }

        self.metrics.record_eviction();
    }

    /// Delete keys matching a pattern (L1 only for now)
    pub async fn delete_pattern(&self, pattern: &str) {
        let count = self.l1.delete_pattern(pattern).await;
        // Record each eviction individually
        for _ in 0..count {
            self.metrics.record_eviction();
        }
        debug!("Deleted {} keys matching pattern: {}", count, pattern);
    }

    /// Clear all cache entries
    pub async fn clear(&mut self) {
        self.l1.clear().await;

        if let Some(ref mut l2) = self.l2 {
            if let Err(e) = l2.clear().await {
                warn!("Failed to clear L2 cache: {}", e);
            }
        }

        info!("Cache cleared");
    }

    /// Get cache metrics
    pub fn metrics(&self) -> &CacheMetrics {
        &self.metrics
    }

    /// Get cache statistics
    pub async fn stats(&self) -> CacheStats {
        CacheStats {
            l1_entry_count: self.l1.entry_count().await,
            l1_hit_rate: self.metrics.hit_rate(),
            l2_enabled: self.l2.is_some(),
            total_hits: self.metrics.total_hits(),
            total_misses: self.metrics.total_misses(),
            total_writes: self.metrics.total_writes(),
        }
    }
}

/// Cache statistics
#[derive(Debug, Serialize, Deserialize)]
pub struct CacheStats {
    pub l1_entry_count: u64,
    pub l1_hit_rate: f64,
    pub l2_enabled: bool,
    pub total_hits: u64,
    pub total_misses: u64,
    pub total_writes: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cache_basic() {
        let config = CacheConfig {
            l2_enabled: false,
            ..Default::default()
        };
        let mut cache = CacheService::new(config).await;

        // Write
        cache.set("test:1".to_string(), b"value1".to_vec()).await;

        // Read
        let value = cache.get("test:1").await;
        assert_eq!(value, Some(b"value1".to_vec()));

        // Delete
        cache.delete("test:1").await;
        let value = cache.get("test:1").await;
        assert_eq!(value, None);
    }

    #[tokio::test]
    async fn test_cache_miss() {
        let config = CacheConfig {
            l2_enabled: false,
            ..Default::default()
        };
        let mut cache = CacheService::new(config).await;

        let value = cache.get("nonexistent").await;
        assert_eq!(value, None);
    }

    #[tokio::test]
    async fn test_cache_stats() {
        let config = CacheConfig {
            l2_enabled: false,
            metrics_enabled: true,
            ..Default::default()
        };
        let mut cache = CacheService::new(config).await;

        cache.set("test:1".to_string(), b"value1".to_vec()).await;
        let _ = cache.get("test:1").await; // hit
        let _ = cache.get("test:2").await; // miss

        let stats = cache.stats().await;
        assert_eq!(stats.total_hits, 1);
        assert_eq!(stats.total_misses, 1);
        assert_eq!(stats.total_writes, 1);
    }
}
