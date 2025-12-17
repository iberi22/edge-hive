use crate::LogEntry;
use tauri::State;
use std::sync::{Arc, Mutex};

// Basic in-memory log store for demo purposes
// In a real app, this would query edge-hive-logging or similar
pub struct LogState {
    pub logs: Arc<Mutex<Vec<LogEntry>>>,
}

#[derive(serde::Serialize, Clone, Debug)]
pub struct LogEntry {
    pub id: String,
    pub timestamp: String,
    pub level: String,
    pub service: String,
    pub message: String,
}

#[tauri::command]
pub async fn get_logs(state: State<'_, LogState>) -> Result<Vec<LogEntry>, String> {
    let logs = state.logs.lock().map_err(|e| e.to_string())?;
    Ok(logs.clone())
}

#[tauri::command]
pub async fn add_log(state: State<'_, LogState>, level: String, message: String) -> Result<(), String> {
    let mut logs = state.logs.lock().map_err(|e| e.to_string())?;
    logs.push(LogEntry {
        id: uuid::Uuid::new_v4().to_string(),
        timestamp: chrono::Local::now().to_rfc3339(),
        level,
        service: "admin-ui".to_string(),
        message,
    });
    // Keep last 1000 logs
    if logs.len() > 1000 {
        logs.remove(0);
    }
    Ok(())
}
