use serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;
use crate::db_commands::DatabaseState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Option<String>,
    pub title: String,
    pub description: String,
    pub status: String,
    pub priority: String,
    pub due_date: Datetime,
}

#[tauri::command]
pub async fn get_tasks(state: tauri::State<'_, DatabaseState>) -> Result<Vec<Task>, String> {
    let tasks = state.db_service.get_tasks().await.map_err(|e| e.to_string())?;
    Ok(tasks.into_iter().map(|t| Task {
        id: Some(t.id),
        title: t.title,
        description: t.description,
        status: t.status,
        priority: t.priority,
        due_date: t.due_date,
    }).collect())
}

use edge_hive_db::StoredTask;

#[tauri::command]
pub async fn create_task(task: Task, state: tauri::State<'_, DatabaseState>) -> Result<Task, String> {
    let new_task = StoredTask {
        id: task.id.unwrap_or_else(|| "".to_string()),
        title: task.title,
        description: task.description,
        status: task.status,
        priority: task.priority,
        due_date: task.due_date,
        created_at: chrono::Utc::now().into(),
        assignee: None,
    };
    state.db_service.save_task(&new_task).await.map_err(|e| e.to_string())?;
    Ok(Task {
        id: Some(new_task.id),
        title: new_task.title,
        description: new_task.description,
        status: new_task.status,
        priority: new_task.priority,
        due_date: new_task.due_date,
    })
}

#[tauri::command]
pub async fn update_task(task: Task, state: tauri::State<'_, DatabaseState>) -> Result<Task, String> {
    let id = task.id.clone().ok_or("Task ID is required for update".to_string())?;
    let stored_task = StoredTask {
        id,
        title: task.title,
        description: task.description,
        status: task.status,
        priority: task.priority,
        due_date: task.due_date,
        created_at: chrono::Utc::now().into(), // This should be preserved, not updated
        assignee: None,
    };
    state.db_service.save_task(&stored_task).await.map_err(|e| e.to_string())?;
    Ok(task)
}

#[tauri::command]
pub async fn delete_task(id: String, state: tauri::State<'_, DatabaseState>) -> Result<(), String> {
    state.db_service.delete_task(&id).await.map_err(|e| e.to_string())
}