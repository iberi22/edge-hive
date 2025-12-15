//! Edge Hive DB - SurrealDB wrapper for persistent storage
//!
//! Provides embedded database functionality with RocksDB backend.

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::path::Path;
use surrealdb::engine::local::RocksDb;
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

/// Encrypted message stored in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredMessage {
    pub message_id: String,
    pub payload: Vec<u8>,
    pub received_at: chrono::DateTime<chrono::Utc>,
}

/// WASM plugin metadata stored in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredPlugin {
    pub plugin_id: String,
    pub name: String,
    pub version: String,
    pub enabled: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Database service for Edge Hive
pub struct DatabaseService {
    db: Surreal<surrealdb::engine::local::Db>,
}

impl DatabaseService {
    /// Create a new database service with RocksDB backend
    pub async fn new(path: &Path) -> Result<Self, DbError> {
        info!("📦 Opening database at {:?}", path);

        let db = Surreal::new::<RocksDb>(path)
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

        // Define messages table
        self.db
            .query(
                r#"
                DEFINE TABLE IF NOT EXISTS message SCHEMAFULL;
                DEFINE FIELD message_id ON message TYPE string;
                DEFINE FIELD payload ON message TYPE bytes;
                DEFINE FIELD received_at ON message TYPE datetime;
                DEFINE INDEX message_id_idx ON message FIELDS message_id UNIQUE;
                "#,
            )
            .await?;

        // Define plugins table
        self.db
            .query(
                r#"
                DEFINE TABLE IF NOT EXISTS plugin SCHEMAFULL;
                DEFINE FIELD plugin_id ON plugin TYPE string;
                DEFINE FIELD name ON plugin TYPE string;
                DEFINE FIELD version ON plugin TYPE string;
                DEFINE FIELD enabled ON plugin TYPE bool;
                DEFINE FIELD created_at ON plugin TYPE datetime;
                DEFINE INDEX plugin_id_idx ON plugin FIELDS plugin_id UNIQUE;
                "#,
            )
            .await?;

        info!("✅ Database schema initialized");
        Ok(())
    }

    /// Save or update a peer
    pub async fn add_peer(&self, peer: &StoredPeer) -> Result<(), DbError> {
        self.db
            .update(("peer", &peer.peer_id))
            .content(peer)
            .await?;
        Ok(())
    }

    /// Store an encrypted message
    pub async fn store_message(&self, message: &StoredMessage) -> Result<(), DbError> {
        self.db
            .create(("message", &message.message_id))
            .content(message)
            .await?;
        Ok(())
    }

    /// Get all known peers
    pub async fn get_peers(&self) -> Result<Vec<StoredPeer>, DbError> {
        let mut response = self.db.query("SELECT * FROM peer").await?;
        let peers: Vec<StoredPeer> = response.take(0)?;
        Ok(peers)
    }

    /// Get a peer by ID
    pub async fn get_peer(&self, peer_id: &str) -> Result<Option<StoredPeer>, DbError> {
        let mut response = self.db
            .query("SELECT * FROM peer WHERE peer_id = $peer_id")
            .bind(("peer_id", peer_id))
            .await?;

        let peers: Vec<StoredPeer> = response.take(0)?;
        Ok(peers.into_iter().next())
    }

    /// Save a configuration value
    pub async fn set_config<T: Serialize>(&self, key: &str, value: &T) -> Result<(), DbError> {
        let json_value = serde_json::to_value(value)
            .map_err(|e| DbError::Serialization(e.to_string()))?;

        self.db
            .query("DELETE config WHERE key = $key")
            .bind(("key", key))
            .await?;

        self.db
            .query("CREATE config CONTENT { key: $key, value: $value }")
            .bind(("key", key))
            .bind(("value", json_value))
            .await?;

        Ok(())
    }

    /// Get a configuration value
    pub async fn get_config<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>, DbError> {
        let mut response = self.db
            .query("SELECT value FROM config WHERE key = $key")
            .bind(("key", key))
            .await?;

        let configs: Vec<StoredConfig> = response.take(0)?;

        if let Some(config) = configs.into_iter().next() {
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

        db.add_peer(&peer).await.unwrap();
        let loaded = db.get_peer("test-peer-id").await.unwrap();
        assert!(loaded.is_some());
        assert_eq!(loaded.unwrap().name, Some("test-node".into()));

        // Test message
        let message = StoredMessage {
            message_id: "test-message-id".into(),
            payload: vec![1, 2, 3],
            received_at: chrono::Utc::now(),
        };

        db.store_message(&message).await.unwrap();
        let mut response = db
            .query("SELECT * FROM message WHERE message_id = 'test-message-id'")
            .await
            .unwrap();
        let messages: Vec<StoredMessage> = response.take(0).unwrap();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].payload, vec![1, 2, 3]);
    }
}
