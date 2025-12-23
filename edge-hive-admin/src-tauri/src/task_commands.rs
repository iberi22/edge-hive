use tauri::State;
use crate::db_commands::DatabaseState;
use edge_hive_db::StoredTask;
use serde::{Deserialize, Serialize};

/// Frontend-friendly task representation with string ID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskDto {
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: String,
    pub priority: String,
    pub due_date: String,
    pub created_at: String,
    pub assignee: Option<String>,
}

impl From<StoredTask> for TaskDto {
    fn from(task: StoredTask) -> Self {
        TaskDto {
            id: task.id.map(|t| t.id.to_string()).unwrap_or_default(),
            title: task.title,
            description: task.description,
            status: task.status,
            priority: task.priority,
            due_date: task.due_date.to_string(),
            created_at: task.created_at.to_string(),
            assignee: task.assignee,
        }
    }
}

impl From<TaskDto> for StoredTask {
    fn from(dto: TaskDto) -> Self {
        StoredTask {
            id: if dto.id.is_empty() {
                None
            } else {
                Some(surrealdb::sql::Thing::from(("task", dto.id.as_str())))
            },
            title: dto.title,
            description: dto.description,
            status: dto.status,
            priority: dto.priority,
            due_date: surrealdb::sql::Datetime::default(), // Frontend should send proper datetime
            created_at: surrealdb::sql::Datetime::default(),
            assignee: dto.assignee,
        }
    }
}

#[tauri::command]
pub async fn get_tasks(
    state: State<'_, DatabaseState>,
) -> Result<Vec<TaskDto>, String> {
    let tasks = state.service.get_tasks().await.map_err(|e| e.to_string())?;
    Ok(tasks.into_iter().map(TaskDto::from).collect())
}

#[tauri::command]
pub async fn create_task(
    state: State<'_, DatabaseState>,
    task: TaskDto,
) -> Result<TaskDto, String> {
    let stored_task: StoredTask = task.into();
    let created = state.service.save_task(&stored_task).await.map_err(|e| e.to_string())?;
    Ok(TaskDto::from(created))
}

#[tauri::command]
pub async fn update_task(
    state: State<'_, DatabaseState>,
    task: TaskDto,
) -> Result<TaskDto, String> {
    let stored_task: StoredTask = task.into();
    let updated = state.service.save_task(&stored_task).await.map_err(|e| e.to_string())?;
    Ok(TaskDto::from(updated))
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
    task: TaskDto,
) -> Result<(), String> {
    let stored_task: StoredTask = task.into();
    state.service.save_task(&stored_task).await.map_err(|e| e.to_string())?;
    Ok(())
}
