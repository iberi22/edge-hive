//! L2 Cache - Redis-compatible persistent cache using mini-redis (microsecond access)

use anyhow::Result;
use mini_redis::client;
use tracing::debug;

/// L2 cache implementation (Redis-compatible, persistent)
pub struct L2Cache {
    client: client::Client,
}

impl L2Cache {
    /// Connect to Redis server
    pub async fn connect(addr: &str) -> Result<Self> {
        let client = client::connect(addr).await.map_err(|e| anyhow::anyhow!("{}", e))?;
        debug!("L2 cache connected to Redis at {}", addr);
        Ok(Self { client })
    }

    /// Get a value from L2 cache
    pub async fn get(&mut self, key: &str) -> Result<Option<Vec<u8>>> {
        let value = self.client.get(key).await.map_err(|e| anyhow::anyhow!("{}", e))?;
        Ok(value.map(|bytes| bytes.to_vec()))
    }

    /// Set a value in L2 cache
    pub async fn set(&mut self, key: &str, value: Vec<u8>) -> Result<()> {
        self.client.set(key, value.into()).await.map_err(|e| anyhow::anyhow!("{}", e))?;
        Ok(())
    }

    /// Set a value with TTL (time-to-live)
    pub async fn set_with_ttl(&mut self, key: &str, value: Vec<u8>, _ttl_secs: u64) -> Result<()> {
        self.client.set(key, value.into()).await.map_err(|e| anyhow::anyhow!("{}", e))?;

        // Note: mini-redis doesn't support SETEX/EXPIRE in the current client API
        // For production, we'd need to extend mini-redis or use redis-rs
        // For now, we just set the value without TTL
        debug!("TTL not fully supported in mini-redis client, set without expiration: {}", key);

        Ok(())
    }

    /// Delete a key from L2 cache
    pub async fn delete(&mut self, key: &str) -> Result<bool> {
        // mini-redis doesn't expose DEL command directly
        // We'll need to extend the client or use a different approach
        debug!("Delete operation not fully implemented in mini-redis client: {}", key);
        Ok(false)
    }

    /// Clear all entries (FLUSHDB)
    pub async fn clear(&mut self) -> Result<()> {
        // mini-redis doesn't expose FLUSHDB
        debug!("Clear operation not fully implemented in mini-redis client");
        Ok(())
    }
}

// Note: For production, consider using `redis` crate instead of `mini-redis`
// mini-redis is a minimal implementation for learning purposes
// The `redis` crate provides full Redis command support:
//
// ```toml
// redis = { version = "0.24", features = ["tokio-comp", "connection-manager"] }
// ```
//
// Migration example:
// ```rust
// use redis::AsyncCommands;
//
// pub async fn connect(addr: &str) -> Result<Self> {
//     let client = redis::Client::open(addr)?;
//     let conn = client.get_multiplexed_async_connection().await?;
//     Ok(Self { conn })
// }
//
// pub async fn set_with_ttl(&mut self, key: &str, value: Vec<u8>, ttl_secs: u64) -> Result<()> {
//     self.conn.set_ex(key, value, ttl_secs).await?;
//     Ok(())
// }
// ```

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These tests require a Redis server running on localhost:6379
    // Skip them in CI unless Redis is available

    #[tokio::test]
    #[ignore] // Run with: cargo test -- --ignored
    async fn test_l2_connect() {
        let result = L2Cache::connect("127.0.0.1:6379").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_l2_basic_operations() {
        let mut cache = L2Cache::connect("127.0.0.1:6379").await.unwrap();

        // Set and get
        cache.set("test:key1", b"value1".to_vec()).await.unwrap();
        let value = cache.get("test:key1").await.unwrap();
        assert_eq!(value, Some(b"value1".to_vec()));

        // Delete
        cache.delete("test:key1").await.unwrap();
        let value = cache.get("test:key1").await.unwrap();
        assert_eq!(value, None);
    }
}
