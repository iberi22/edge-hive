use edge_hive_db::DatabaseService;
use serde_json::Value;
use std::sync::Arc;
use tauri::State;

pub struct DatabaseState {
    pub service: Arc<DatabaseService>,
}

impl DatabaseState {
    pub fn new(service: DatabaseService) -> Self {
        Self {
            service: Arc::new(service),
        }
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
