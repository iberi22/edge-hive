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
    pub last_seen: surrealdb::Datetime,
    pub capabilities: u32,
}

/// Node configuration stored in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredConfig {
    pub key: String,
    pub value: serde_json::Value,
}

/// User information stored in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredUser {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub created_at: surrealdb::Datetime,
    pub updated_at: surrealdb::Datetime,
    pub deleted_at: Option<surrealdb::Datetime>,
}

/// Task information stored in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredTask {
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: String,
    pub priority: String,
    pub due_date: surrealdb::Datetime,
    pub created_at: surrealdb::Datetime,
    pub assignee: Option<String>,
}

/// Database service for Edge Hive
pub struct DatabaseService {
    db: Surreal<surrealdb::engine::local::Db>,
}

/// Generic record shape for Live Queries.
///
/// SurrealDB live notifications include an `id` of type `Thing` plus arbitrary fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveRecord {
    pub id: surrealdb::sql::Thing,
    #[serde(flatten)]
    pub fields: serde_json::Map<String, serde_json::Value>,
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

        // Define task table
        self.db
            .query(
                r#"
                DEFINE TABLE IF NOT EXISTS task SCHEMAFULL;
                DEFINE FIELD title ON task TYPE string;
                DEFINE FIELD description ON task TYPE string;
                DEFINE FIELD status ON task TYPE string;
                DEFINE FIELD priority ON task TYPE string;
                DEFINE FIELD due_date ON task TYPE datetime;
                DEFINE FIELD created_at ON task TYPE datetime;
                DEFINE FIELD assignee ON task TYPE option<string>;
                "#,
            )
            .await?;

        // Define user table
        self.db
            .query(
                r#"
                DEFINE TABLE IF NOT EXISTS user SCHEMAFULL;
                DEFINE FIELD email ON user TYPE string;
                DEFINE FIELD name ON user TYPE option<string>;
                DEFINE FIELD created_at ON user TYPE datetime;
                DEFINE FIELD updated_at ON user TYPE datetime;
                DEFINE FIELD deleted_at ON user TYPE option<datetime>;
                DEFINE INDEX user_email_idx ON user FIELDS email UNIQUE;
                "#,
            )
            .await?;

        // Seed initial tasks if table is empty
        let mut count_resp = self.db.query("SELECT count() FROM task GROUP ALL").await?;
        let count: Option<i64> = count_resp.take("count").unwrap_or(None);

        if count.unwrap_or(0) == 0 {
            info!("🌱 Seeding initial tasks into database");
            let initial_tasks = vec![
                StoredTask {
                    id: "TSK-942".into(),
                    title: "Provision Hidden Onion Service (v3)".into(),
                    description: "Generating ED25519 identity keys and configuring Tor circuits.".into(),
                    status: "processing".into(),
                    priority: "high".into(),
                    due_date: chrono::Utc::now().into(),
                    created_at: chrono::Utc::now().into(),
                    assignee: Some("neural_agent".into()),
                },
                StoredTask {
                    id: "TSK-119".into(),
                    title: "Update VPN Mesh Peer Lattice".into(),
                    description: "Broadcasting new public keys to all peers in the WireGuard mesh.".into(),
                    status: "completed".into(),
                    priority: "critical".into(),
                    due_date: chrono::Utc::now().into(),
                    created_at: chrono::Utc::now().into(),
                    assignee: Some("wg_daemon".into()),
                },
                StoredTask {
                    id: "TSK-032".into(),
                    title: "Self-Heal Shard HN-02".into(),
                    description: "Relocating data shards post-entropy scan.".into(),
                    status: "pending".into(),
                    priority: "medium".into(),
                    due_date: chrono::Utc::now().into(),
                    created_at: chrono::Utc::now().into(),
                    assignee: Some("shard_balancer".into()),
                },
            ];

            for task in initial_tasks {
                let _ = self.save_task(&task).await;
            }
        }

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

    /// Save or update a user
    pub async fn save_user(&self, user: &StoredUser) -> Result<(), DbError> {
        self.db
            .query("CREATE user SET id = $id, email = $email, name = $name, created_at = $created_at, updated_at = $updated_at, deleted_at = $deleted_at ON DUPLICATE KEY UPDATE email = $email, name = $name, updated_at = $updated_at, deleted_at = $deleted_at")
            .bind(user.clone())
            .await?;
        Ok(())
    }

    /// Get a user by ID, returning only if not marked as deleted
    pub async fn get_user_by_id(&self, user_id: &str) -> Result<Option<StoredUser>, DbError> {
        let user: Option<StoredUser> = self.db.select(("user", user_id)).await?;

        Ok(user.filter(|u| u.deleted_at.is_none()))
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

    /// Save or update a task
    pub async fn save_task(&self, task: &StoredTask) -> Result<(), DbError> {
        let _: Option<StoredTask> = self.db
            .upsert(("task", task.id.as_str()))
            .content(task.clone())
            .await?;

        Ok(())
    }

    /// Get all tasks
    pub async fn get_tasks(&self) -> Result<Vec<StoredTask>, DbError> {
        let tasks: Vec<StoredTask> = self.db.select("task").await?;
        Ok(tasks)
    }

    /// Get a task by ID
    pub async fn get_task(&self, id: &str) -> Result<Option<StoredTask>, DbError> {
        let task: Option<StoredTask> = self.db.select(("task", id)).await?;
        Ok(task)
    }

    /// Delete a task
    pub async fn delete_task(&self, id: &str) -> Result<(), DbError> {
        let _: Option<StoredTask> = self.db.delete(("task", id)).await?;
        Ok(())
    }

    /// Execute a raw SurrealQL query
    pub async fn query(&self, sql: &str) -> Result<surrealdb::Response, DbError> {
        Ok(self.db.query(sql).await?)
    }

    /// Execute a raw SurrealQL query and return the first statement's results as JSON.
    ///
    /// This is useful for HTTP handlers that want to return `serde_json::Value` while
    /// SurrealDB may contain non-JSON-native types.
    pub async fn query_json(&self, sql: &str) -> Result<Vec<serde_json::Value>, DbError> {
        #[derive(Debug, Deserialize)]
        struct AnyRecord {
            id: surrealdb::sql::Thing,
            #[serde(flatten)]
            fields: serde_json::Map<String, serde_json::Value>,
        }

        let mut resp = self.db.query(sql).await?;
        let records: Vec<AnyRecord> = resp
            .take(0)
            .map_err(|e| DbError::Query(e.to_string()))?;

        let mut out = Vec::with_capacity(records.len());
        for record in records {
            let mut obj = record.fields;
            obj.insert(
                "id".to_string(),
                serde_json::Value::String(record.id.to_string()),
            );
            out.push(serde_json::Value::Object(obj));
        }

        Ok(out)
    }

    /// Create a Live Query stream for all changes on a table.
    ///
    /// This uses SurrealDB's Rust client live query support (embedded/local engine).
    pub async fn live_table(
        &self,
        table: &str,
    ) -> Result<surrealdb::method::Stream<Vec<LiveRecord>>, DbError> {
        Ok(self.db.select::<Vec<LiveRecord>>(table).live().await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::StreamExt;
    use tempfile::tempdir;
    use std::time::Duration;

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
            last_seen: chrono::Utc::now().into(),
            capabilities: 1,
        };

        db.save_peer(&peer).await.unwrap();
        let loaded = db.get_peer("test-peer-id").await.unwrap();
        assert!(loaded.is_some());
        assert_eq!(loaded.unwrap().name, Some("test-node".into()));
    }

    #[tokio::test]
    async fn test_query_json_create_schemaless_table() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let db = DatabaseService::new(&db_path).await.unwrap();

        let created = db
            .query_json(r#"CREATE items CONTENT {"name":"alpha"};"#)
            .await
            .unwrap();

        assert!(!created.is_empty(), "expected CREATE to return at least one record");
    }

    #[tokio::test]
    async fn test_live_table_emits_create_notification() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let db = DatabaseService::new(&db_path).await.unwrap();

        let mut stream = db.live_table("items").await.unwrap();

        let _ = db
            .query(r#"CREATE items CONTENT {"name":"live"};"#)
            .await
            .unwrap();

        let next = tokio::time::timeout(Duration::from_secs(3), async { stream.next().await })
            .await
            .expect("timeout waiting for live notification");

        let next = next.expect("stream ended").expect("notification ok");
        assert!(matches!(next.action, surrealdb::Action::Create));
    }
}
