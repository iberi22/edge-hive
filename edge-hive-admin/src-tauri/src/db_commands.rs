use edge_hive_db::DatabaseService;
use serde_json::Value;
use std::sync::Arc;
use tauri::State;

pub struct DatabaseState {
    // Stub service
    pub service: DatabaseServiceStub,
}

pub struct DatabaseServiceStub;

impl DatabaseServiceStub {
    pub async fn query(&self, _query: &str) -> Result<Vec<()>, String> {
        Ok(vec![])
    }
    // Assuming query_json is also needed for the stub to compile with db_query
    pub async fn query_json(&self, _query: &str) -> Result<Vec<Value>, String> {
        Ok(vec![])
    }
}

impl DatabaseState {
    pub fn new() -> Self {
        Self { service: DatabaseServiceStub }
    }
}

#[tauri::command]
pub async fn db_query(
    state: State<'_, DatabaseState>,
    sql: String,
) -> Result<Vec<Value>, String> {
    state
        .service
        .query_json(&sql)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_execute(
    state: State<'_, DatabaseState>,
    sql: String,
) -> Result<String, String> {
    state
        .service
        .query(&sql)
        .await
        .map(|_| "OK".to_string())
        .map_err(|e| e.to_string())
}
