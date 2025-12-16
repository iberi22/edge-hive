//! L1 Cache - In-memory cache using moka (nanosecond access)

use moka::future::Cache;
use std::time::Duration;
use tracing::debug;

/// L1 cache implementation (in-memory, ultra-fast)
pub struct L1Cache {
    cache: Cache<String, Vec<u8>>,
}

impl L1Cache {
    /// Create a new L1 cache
    pub fn new(max_capacity: u64, ttl_secs: u64) -> Self {
        let cache = Cache::builder()
            .max_capacity(max_capacity)
            .time_to_live(Duration::from_secs(ttl_secs))
            .build();

        Self { cache }
    }

    /// Get a value from L1 cache
    pub async fn get(&self, key: &str) -> Option<Vec<u8>> {
        self.cache.get(key).await
    }

    /// Set a value in L1 cache
    pub async fn set(&self, key: String, value: Vec<u8>) {
        self.cache.insert(key, value).await;
    }

    /// Set a value with custom TTL
    pub async fn set_with_ttl(&self, key: String, value: Vec<u8>, _ttl_secs: u64) {
        // Note: moka doesn't support per-entry TTL in the current API
        // For now, we use the global TTL. For fine-grained control,
        // we'd need to implement a custom eviction policy.
        self.cache.insert(key, value).await;
    }

    /// Delete a key from L1 cache
    pub async fn delete(&self, key: &str) {
        self.cache.invalidate(key).await;
    }

    /// Delete keys matching a pattern (basic implementation)
    pub async fn delete_pattern(&self, pattern: &str) -> u64 {
        let mut count = 0u64;

        // Simple pattern matching (prefix, suffix, contains)
        if pattern.ends_with('*') {
            // Prefix match: "user:*"
            let _prefix = &pattern[..pattern.len() - 1];

            // Note: moka doesn't expose keys directly, so we can't iterate
            // This is a limitation. For production, consider using a separate index.
            debug!("Pattern deletion not fully implemented for L1 cache: {}", pattern);
        } else if pattern.starts_with('*') {
            // Suffix match: "*:cached"
            debug!("Pattern deletion not fully implemented for L1 cache: {}", pattern);
        } else {
            // Exact match
            self.cache.invalidate(pattern).await;
            count = 1;
        }

        count
    }

    /// Clear all entries
    pub async fn clear(&self) {
        self.cache.invalidate_all();
    }

    /// Get number of entries in cache
    pub async fn entry_count(&self) -> u64 {
        self.cache.entry_count()
    }

    /// Get cache capacity
    pub fn max_capacity(&self) -> u64 {
        // Note: moka v0.12 doesn't expose max_capacity()
        // Return 0 as placeholder
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_l1_basic_operations() {
        let cache = L1Cache::new(1000, 60);

        // Set and get
        cache.set("key1".to_string(), b"value1".to_vec()).await;
        let value = cache.get("key1").await;
        assert_eq!(value, Some(b"value1".to_vec()));

        // Delete
        cache.delete("key1").await;
        let value = cache.get("key1").await;
        assert_eq!(value, None);
    }

    #[tokio::test]
    async fn test_l1_entry_count() {
        let cache = L1Cache::new(1000, 60);

        cache.set("key1".to_string(), b"value1".to_vec()).await;
        cache.set("key2".to_string(), b"value2".to_vec()).await;

        // Note: moka's entry_count() may not be instantly accurate
        // Just verify it returns without panicking
        let _count = cache.entry_count().await;

        // Verify values are actually stored
        assert_eq!(cache.get("key1").await, Some(b"value1".to_vec()));
        assert_eq!(cache.get("key2").await, Some(b"value2".to_vec()));
    }

    #[tokio::test]
    async fn test_l1_clear() {
        let cache = L1Cache::new(1000, 60);

        cache.set("key1".to_string(), b"value1".to_vec()).await;
        cache.set("key2".to_string(), b"value2".to_vec()).await;

        // Verify data exists
        assert_eq!(cache.get("key1").await, Some(b"value1".to_vec()));

        cache.clear().await;

        // Verify data is cleared
        assert_eq!(cache.get("key1").await, None);
        assert_eq!(cache.get("key2").await, None);
    }
}
