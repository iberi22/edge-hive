use edge_hive_db::DatabaseService;
use serde_json::Value;
use std::path::Path;
use tauri::State;

pub struct DatabaseState {
    pub service: DatabaseService,
}

impl DatabaseState {
    pub async fn new(db_path: &Path) -> Result<Self, String> {
        let service = DatabaseService::new(db_path)
            .await
            .map_err(|e| e.to_string())?;
        Ok(Self { service })
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
