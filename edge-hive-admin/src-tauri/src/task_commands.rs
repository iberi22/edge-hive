use tauri::State;
use crate::db_commands::DatabaseState;
use edge_hive_db::StoredTask;

#[tauri::command]
pub async fn get_tasks(
    state: State<'_, DatabaseState>,
) -> Result<Vec<StoredTask>, String> {
    state.service.get_tasks().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_task(
    state: State<'_, DatabaseState>,
    task: StoredTask,
) -> Result<(), String> {
    state.service.save_task(&task).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_task(
    state: State<'_, DatabaseState>,
    task: StoredTask,
) -> Result<(), String> {
    state.service.save_task(&task).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_task(
    state: State<'_, DatabaseState>,
    id: String,
) -> Result<(), String> {
    state.service.delete_task(&id).await.map_err(|e| e.to_string())
}

// Keep save_task for backward compatibility
#[tauri::command]
pub async fn save_task(
    state: State<'_, DatabaseState>,
    task: StoredTask,
) -> Result<(), String> {
    state.service.save_task(&task).await.map_err(|e| e.to_string())
}
