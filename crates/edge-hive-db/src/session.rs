//! Session model for refresh tokens

use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Thing};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StoredSession {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Thing>,
    pub user_id: Thing,
    pub refresh_token_hash: String,
    pub expires_at: Datetime,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}
