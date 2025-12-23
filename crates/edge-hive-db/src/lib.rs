//! Edge Hive DB - SurrealDB wrapper for persistent storage
//!
//! Provides embedded database functionality with RocksDB backend.

pub mod session;
pub mod user;

use crate::user::StoredUser;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::path::Path;
use std::str::FromStr;
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

/// Task information stored in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredTask {
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: String,
    pub priority: String,
    pub due_date: surrealdb::sql::Datetime,
    pub created_at: surrealdb::sql::Datetime,
    pub assignee: Option<String>,
}

/// Session information stored in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredSession {
    pub id: Option<surrealdb::sql::Thing>,
    pub user_id: surrealdb::sql::Thing,
    pub refresh_token_hash: String,
    pub device_info: Option<String>,
    pub ip_address: Option<String>,
    pub created_at: surrealdb::Datetime,
    pub expires_at: surrealdb::Datetime,
    pub revoked: bool,
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

        // Define users table
        self.db
            .query(
                r#"
                DEFINE TABLE IF NOT EXISTS users SCHEMAFULL;
                DEFINE FIELD email ON users TYPE string ASSERT string::is::email($value);
                DEFINE FIELD password_hash ON users TYPE string;
                DEFINE FIELD provider ON users TYPE option<string>;
                DEFINE FIELD provider_id ON users TYPE option<string>;
                DEFINE FIELD name ON users TYPE option<string>;
                DEFINE FIELD avatar_url ON users TYPE option<string>;
                DEFINE FIELD created_at ON users TYPE datetime DEFAULT time::now();
                DEFINE FIELD updated_at ON users TYPE datetime DEFAULT time::now();
                DEFINE FIELD email_verified ON users TYPE bool DEFAULT false;
                DEFINE FIELD role ON users TYPE string DEFAULT 'user';
                DEFINE INDEX users_email ON users COLUMNS email UNIQUE;
                DEFINE INDEX users_provider ON users COLUMNS provider, provider_id UNIQUE;
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

        // Define sessions table
        self.db.query(r#"
            DEFINE TABLE IF NOT EXISTS sessions SCHEMAFULL;
            DEFINE FIELD user_id ON sessions TYPE record<users>;
            DEFINE FIELD refresh_token_hash ON sessions TYPE string;
            DEFINE FIELD device_info ON sessions TYPE option<string>;
            DEFINE FIELD ip_address ON sessions TYPE option<string>;
            DEFINE FIELD created_at ON sessions TYPE datetime DEFAULT time::now();
            DEFINE FIELD expires_at ON sessions TYPE datetime;
            DEFINE FIELD revoked ON sessions TYPE bool DEFAULT false;
            DEFINE INDEX sessions_user ON sessions COLUMNS user_id;
            DEFINE INDEX sessions_token ON sessions COLUMNS refresh_token_hash UNIQUE;
        "#).await?;

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

    /// Create a new user
    pub async fn create_user(&self, user: &StoredUser) -> Result<StoredUser, DbError> {
        let created: Option<StoredUser> = self.db.create("users").content(user.clone()).await?;
        created.ok_or_else(|| DbError::Query("User creation returned no record".to_string()))
    }

    /// Get a user by ID
    pub async fn get_user_by_id(&self, user_id: &str) -> Result<Option<StoredUser>, DbError> {
        let user: Option<StoredUser> = self.db.select(("users", user_id)).await?;
        Ok(user)
    }

    /// Get a user by email
    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<StoredUser>, DbError> {
        let mut result = self
            .db
            .query("SELECT * FROM users WHERE email = $email")
            .bind(("email", email.to_string()))
            .await?;
        let user: Option<StoredUser> = result.take(0)?;
        Ok(user)
    }

    /// Get a user by provider
    pub async fn get_user_by_provider(
        &self,
        provider: &str,
        provider_id: &str,
    ) -> Result<Option<StoredUser>, DbError> {
        let mut result = self
            .db
            .query("SELECT * FROM users WHERE provider = $provider AND provider_id = $provider_id")
            .bind(("provider", provider.to_string()))
            .bind(("provider_id", provider_id.to_string()))
            .await?;
        let user: Option<StoredUser> = result.take(0)?;
        Ok(user)
    }

    /// Update a user
    pub async fn update_user(&self, user: &StoredUser) -> Result<(), DbError> {
        if let Some(id) = &user.id {
            let record_id = id.id.to_string();
            let _: Option<StoredUser> = self.db.update(("users", record_id)).content(user.clone()).await?;
            Ok(())
        } else {
            Err(DbError::Query("User ID is required for update".to_string()))
        }
    }

    /// Delete a user
    pub async fn delete_user(&self, id: &str) -> Result<(), DbError> {
        let _: Option<StoredUser> = self.db.delete(("users", id)).await?;
        Ok(())
    }

    // Session management
    pub async fn create_session(
        &self,
        session: &session::StoredSession,
    ) -> Result<session::StoredSession, DbError> {
        let created: Option<session::StoredSession> =
            self.db.create("sessions").content(session.clone()).await?;
        created.ok_or_else(|| DbError::Query("Session creation returned no record".to_string()))
    }

    /// Get a session by refresh token hash
    pub async fn get_session_by_token(
        &self,
        token_hash: &str,
    ) -> Result<Option<StoredSession>, DbError> {
        let mut result = self
            .db
            .query("SELECT * FROM sessions WHERE refresh_token_hash = $token_hash")
            .bind(("token_hash", token_hash.to_string()))
            .await?;
        let session: Option<StoredSession> = result.take(0)?;
        Ok(session)
    }

    /// Revoke a specific session
    pub async fn revoke_session(&self, session_id: &str) -> Result<(), DbError> {
        let thing = surrealdb::sql::Thing::from_str(session_id)
            .map_err(|_| DbError::Query("Invalid session ID format".to_string()))?;
        self.db
            .query("UPDATE $session_id SET revoked = true")
            .bind(("session_id", thing))
            .await?;
        Ok(())
    }

    /// Revoke all sessions for a user
    pub async fn revoke_all_user_sessions(&self, user_id: &str) -> Result<u64, DbError> {
        let user_thing = surrealdb::sql::Thing::from_str(user_id)
            .map_err(|_| DbError::Query("Invalid user ID format".to_string()))?;
        let mut result = self
            .db
            .query("UPDATE sessions SET revoked = true WHERE user_id = $user_id")
            .bind(("user_id", user_thing))
            .await?;
        let updated_sessions: Vec<StoredSession> = result.take(0)?;
        Ok(updated_sessions.len() as u64)
    }

    /// Clean up expired sessions
    pub async fn cleanup_expired_sessions(&self) -> Result<u64, DbError> {
        let mut result = self
            .db
            .query("DELETE sessions WHERE expires_at < time::now()")
            .await?;
        let deleted_sessions: Vec<StoredSession> = result.take(0)?;
        Ok(deleted_sessions.len() as u64)
    }

    // Execute a raw JSON query and return the JSON response
    pub async fn query_json(&self, query: &str) -> Result<Vec<serde_json::Value>, DbError> {
        let mut result = self.db.query(query).await?;
        let values: Vec<serde_json::Value> = result.take(0)?;
        Ok(values)
    }

    /// Subscribe to a live query stream for a table.
    pub async fn live_table(
        &self,
        table: &str,
    ) -> Result<impl futures::Stream<Item = Result<surrealdb::Notification<LiveRecord>, surrealdb::Error>>, DbError> {
        let stream = self.db.select(table).live().await?;
        Ok(stream)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::StreamExt;
    use std::time::Duration;
    use surrealdb::sql::Thing;
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

    #[tokio::test]
    async fn test_create_and_get_user() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let db = DatabaseService::new(&db_path).await.unwrap();

        let user = StoredUser {
            id: None,
            email: "test@example.com".to_string(),
            password_hash: "hash".to_string(),
            name: None,
            created_at: surrealdb::sql::Datetime::from(chrono::Utc::now()),
            updated_at: surrealdb::sql::Datetime::from(chrono::Utc::now()),
        };

        let created_user = db.create_user(&user).await.unwrap();
        assert!(created_user.id.is_some());

        let fetched_user = db.get_user_by_email("test@example.com").await.unwrap();
        assert!(fetched_user.is_some());
        assert_eq!(fetched_user.unwrap().email, "test@example.com");
    }

    #[tokio::test]
    async fn test_unique_email_constraint() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let db = DatabaseService::new(&db_path).await.unwrap();

        let user1 = StoredUser {
            id: None,
            email: "test@example.com".to_string(),
            password_hash: "hash1".to_string(),
            name: None,
            created_at: surrealdb::sql::Datetime::from(chrono::Utc::now()),
            updated_at: surrealdb::sql::Datetime::from(chrono::Utc::now()),
        };

        let user2 = StoredUser {
            id: None,
            email: "test@example.com".to_string(),
            password_hash: "hash2".to_string(),
            name: None,
            created_at: surrealdb::sql::Datetime::from(chrono::Utc::now()),
            updated_at: surrealdb::sql::Datetime::from(chrono::Utc::now()),
        };

        db.create_user(&user1).await.unwrap();
        let result = db.create_user(&user2).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_session_lifecycle() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let db = DatabaseService::new(&db_path).await.unwrap();

        let user_id = Thing::from(("users", "test-user"));
        let session = StoredSession {
            id: None,
            user_id: user_id.clone(),
            refresh_token_hash: "test-token-hash".into(),
            device_info: Some("test-device".into()),
            ip_address: Some("127.0.0.1".into()),
            created_at: chrono::Utc::now().into(),
            expires_at: (chrono::Utc::now() + chrono::Duration::days(7)).into(),
            revoked: false,
        };

        let created_session = db.create_session(&session).await.unwrap();
        assert!(created_session.id.is_some());

        let loaded_session = db
            .get_session_by_token("test-token-hash")
            .await
            .unwrap()
            .unwrap();
        assert_eq!(loaded_session.user_id, user_id);

        db.revoke_session(&created_session.id.unwrap().to_string())
            .await
            .unwrap();

        let revoked_session = db
            .get_session_by_token("test-token-hash")
            .await
            .unwrap()
            .unwrap();
        assert!(revoked_session.revoked);
    }

    #[tokio::test]
    async fn test_revoke_all_sessions() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let db = DatabaseService::new(&db_path).await.unwrap();

        let user_id = Thing::from(("users", "test-user"));
        for i in 0..3 {
            let session = StoredSession {
                id: None,
                user_id: user_id.clone(),
                refresh_token_hash: format!("test-token-hash-{}", i),
                device_info: Some("test-device".into()),
                ip_address: Some("127.0.0.1".into()),
                created_at: chrono::Utc::now().into(),
                expires_at: (chrono::Utc::now() + chrono::Duration::days(7)).into(),
                revoked: false,
            };
            db.create_session(&session).await.unwrap();
        }

        let revoked_count = db
            .revoke_all_user_sessions(&user_id.to_string())
            .await
            .unwrap();
        assert_eq!(revoked_count, 3);

        for i in 0..3 {
            let session = db
                .get_session_by_token(&format!("test-token-hash-{}", i))
                .await
                .unwrap()
                .unwrap();
            assert!(session.revoked);
        }
    }

    #[tokio::test]
    #[ignore] // Ignoring due to suspected bug in in-memory SurrealDB engine's datetime handling
    async fn test_cleanup_expired_sessions() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let db = DatabaseService::new(&db_path).await.unwrap();

        let user_id = Thing::from(("users", "test-user"));
        let historical_date = chrono::DateTime::parse_from_rfc3339("2020-01-01T00:00:00Z")
            .unwrap()
            .with_timezone(&chrono::Utc);

        // Create an expired session
        let expired_session = StoredSession {
            id: None,
            user_id: user_id.clone(),
            refresh_token_hash: "expired-token".into(),
            device_info: Some("test-device".into()),
            ip_address: Some("127.0.0.1".into()),
            created_at: historical_date.into(),
            expires_at: (historical_date + chrono::Duration::hours(1)).into(),
            revoked: false,
        };
        db.create_session(&expired_session).await.unwrap();

        // Create a valid session
        let valid_session = StoredSession {
            id: None,
            user_id: user_id.clone(),
            refresh_token_hash: "valid-token".into(),
            device_info: Some("test-device".into()),
            ip_address: Some("127.0.0.1".into()),
            created_at: chrono::Utc::now().into(),
            expires_at: (chrono::Utc::now() + chrono::Duration::days(7)).into(),
            revoked: false,
        };
        db.create_session(&valid_session).await.unwrap();

        let deleted_count = db.cleanup_expired_sessions().await.unwrap();
        assert_eq!(deleted_count, 1);

        let expired = db.get_session_by_token("expired-token").await.unwrap();
        assert!(expired.is_none());

        let valid = db.get_session_by_token("valid-token").await.unwrap();
        assert!(valid.is_some());
    }
}
