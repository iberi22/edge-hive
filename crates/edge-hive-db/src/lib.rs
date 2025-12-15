//! Edge Hive DB - SurrealDB wrapper for persistent storage
//!
//! Provides embedded database functionality with RocksDB backend.

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::path::Path;
use surrealdb::engine::local::Mem;
use surrealdb::Surreal;
use thiserror::Error;
use tracing::info;

/// Errors that can occur during database operations
#[derive(Debug, Error)]
pub enum DbError {
    #[error("Database connection error: {0}")]
    Connection(String),

    #[error("Query error: {0}")]
    Query(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("SurrealDB error: {0}")]
    Surreal(#[from] surrealdb::Error),
}

/// Peer information stored in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredPeer {
    pub peer_id: String,
    pub name: Option<String>,
    pub addresses: Vec<String>,
    pub last_seen: chrono::DateTime<chrono::Utc>,
    pub capabilities: u32,
}

/// Node configuration stored in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredConfig {
    pub key: String,
    pub value: serde_json::Value,
}

/// Database service for Edge Hive
pub struct DatabaseService {
    db: Surreal<surrealdb::engine::local::Db>,
}

impl DatabaseService {
    /// Create a new database service with in-memory backend
    pub async fn new(_path: &Path) -> Result<Self, DbError> {
        info!("📦 Opening in-memory database");

        let db = Surreal::new::<Mem>(())
            .await
            .map_err(|e| DbError::Connection(e.to_string()))?;

        // Select namespace and database
        db.use_ns("edge_hive").use_db("main").await?;

        let service = Self { db };
        service.initialize_schema().await?;

        Ok(service)
    }

    /// Initialize database schema
    async fn initialize_schema(&self) -> Result<(), DbError> {
        // Define peer table
        self.db
            .query(
                r#"
                DEFINE TABLE IF NOT EXISTS peer SCHEMAFULL;
                DEFINE FIELD peer_id ON peer TYPE string;
                DEFINE FIELD name ON peer TYPE option<string>;
                DEFINE FIELD addresses ON peer TYPE array;
                DEFINE FIELD last_seen ON peer TYPE datetime;
                DEFINE FIELD capabilities ON peer TYPE int;
                DEFINE INDEX peer_id_idx ON peer FIELDS peer_id UNIQUE;
                "#,
            )
            .await?;

        // Define config table
        self.db
            .query(
                r#"
                DEFINE TABLE IF NOT EXISTS config SCHEMAFULL;
                DEFINE FIELD key ON config TYPE string;
                DEFINE FIELD value ON config TYPE any;
                DEFINE INDEX key_idx ON config FIELDS key UNIQUE;
                "#,
            )
            .await?;

        info!("✅ Database schema initialized");
        Ok(())
    }

    /// Save or update a peer
    pub async fn save_peer(&self, peer: &StoredPeer) -> Result<(), DbError> {
        let _: Option<StoredPeer> = self.db
            .create(("peer", peer.peer_id.as_str()))
            .content(peer.clone())
            .await?;

        Ok(())
    }

    /// Get all known peers
    pub async fn get_peers(&self) -> Result<Vec<StoredPeer>, DbError> {
        let peers: Vec<StoredPeer> = self.db.select("peer").await?;
        Ok(peers)
    }

    /// Get a peer by ID
    pub async fn get_peer(&self, peer_id: &str) -> Result<Option<StoredPeer>, DbError> {
        let peer: Option<StoredPeer> = self.db.select(("peer", peer_id)).await?;
        Ok(peer)
    }

    /// Save a configuration value
    pub async fn set_config<T: Serialize>(&self, key: &str, value: &T) -> Result<(), DbError> {
        let json_value = serde_json::to_value(value)
            .map_err(|e| DbError::Serialization(e.to_string()))?;

        let config = StoredConfig {
            key: key.to_string(),
            value: json_value,
        };

        let _: Option<StoredConfig> = self.db
            .create(("config", key))
            .content(config)
            .await?;

        Ok(())
    }

    /// Get a configuration value
    pub async fn get_config<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>, DbError> {
        let config: Option<StoredConfig> = self.db
            .select(("config", key))
            .await?;

        if let Some(config) = config {
            let value = serde_json::from_value(config.value)
                .map_err(|e| DbError::Serialization(e.to_string()))?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    /// Execute a raw SurrealQL query
    pub async fn query(&self, sql: &str) -> Result<surrealdb::Response, DbError> {
        Ok(self.db.query(sql).await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_database_operations() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");

        let db = DatabaseService::new(&db_path).await.unwrap();

        // Test config
        db.set_config("test_key", &"test_value".to_string()).await.unwrap();
        let value: Option<String> = db.get_config("test_key").await.unwrap();
        assert_eq!(value, Some("test_value".to_string()));

        // Test peer
        let peer = StoredPeer {
            peer_id: "test-peer-id".into(),
            name: Some("test-node".into()),
            addresses: vec!["/ip4/127.0.0.1/tcp/8080".into()],
            last_seen: chrono::Utc::now(),
            capabilities: 1,
        };

        db.save_peer(&peer).await.unwrap();
        let loaded = db.get_peer("test-peer-id").await.unwrap();
        assert!(loaded.is_some());
        assert_eq!(loaded.unwrap().name, Some("test-node".into()));
    }
}
