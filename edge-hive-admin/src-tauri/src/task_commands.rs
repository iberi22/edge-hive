use serde::{Deserialize, Serialize};
use tauri::State;
use crate::db_commands::DatabaseState;
use edge_hive_db::StoredTask;

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
            id: task.id.map(|t| t.id.to_raw()).unwrap_or_else(|| "".to_string()),
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
        // Parse datetime strings or use current time as fallback
        let due_date = chrono::DateTime::parse_from_rfc3339(&dto.due_date)
            .or_else(|_| chrono::DateTime::parse_from_str(&dto.due_date, "%+"))
            .map(|dt| surrealdb::sql::Datetime::from(dt.with_timezone(&chrono::Utc)))
            .unwrap_or_else(|_| surrealdb::sql::Datetime::from(chrono::Utc::now()));

        let created_at = chrono::DateTime::parse_from_rfc3339(&dto.created_at)
            .or_else(|_| chrono::DateTime::parse_from_str(&dto.created_at, "%+"))
            .map(|dt| surrealdb::sql::Datetime::from(dt.with_timezone(&chrono::Utc)))
            .unwrap_or_else(|_| surrealdb::sql::Datetime::from(chrono::Utc::now()));

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
            due_date,
            created_at,
            assignee: dto.assignee,
        }
    }
}

#[tauri::command]
pub async fn get_tasks(
    state: State<'_, DatabaseState>,
) -> Result<Vec<TaskDto>, String> {
    let tasks = state.db_service.get_tasks().await.map_err(|e| e.to_string())?;
    Ok(tasks.into_iter().map(TaskDto::from).collect())
}

/// Helper function to save a task and return the DTO
async fn save_task_internal(
    state: &State<'_, DatabaseState>,
    task: TaskDto,
) -> Result<TaskDto, String> {
    let stored_task: StoredTask = task.into();
    let saved = state.db_service.save_task(&stored_task).await.map_err(|e| e.to_string())?;
    Ok(TaskDto::from(saved))
}

#[tauri::command]
pub async fn create_task(
    state: State<'_, DatabaseState>,
    task: TaskDto,
) -> Result<TaskDto, String> {
    save_task_internal(&state, task).await
}

#[tauri::command]
pub async fn update_task(
    state: State<'_, DatabaseState>,
    task: TaskDto,
) -> Result<TaskDto, String> {
    save_task_internal(&state, task).await
}

#[tauri::command]
pub async fn delete_task(
    state: State<'_, DatabaseState>,
    id: String,
) -> Result<(), String> {
    state.db_service.delete_task(&id).await.map_err(|e| e.to_string())
}

// Keep save_task for backward compatibility
#[tauri::command]
pub async fn save_task(
    state: State<'_, DatabaseState>,
    task: TaskDto,
) -> Result<(), String> {
    save_task_internal(&state, task).await?;
    Ok(())
}
