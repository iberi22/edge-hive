use tauri::State;
use serde::{Deserialize, Serialize};
use crate::db_commands::DatabaseState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub status: String,      // todo, in_progress, done
    pub priority: String,    // low, medium, high
    pub due_date: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTaskRequest {
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<String>,
    pub due_date: Option<String>,
}

/// Get all tasks
#[tauri::command]
pub async fn get_tasks(state: State<'_, DatabaseState>) -> Result<Vec<Task>, String> {
    let result = state.service.query_json(
        "SELECT * FROM tasks ORDER BY created_at DESC"
    ).await.map_err(|e| e.to_string())?;

    serde_json::from_value(result).map_err(|e| e.to_string())
}

/// Create a new task
#[tauri::command]
pub async fn create_task(
    state: State<'_, DatabaseState>,
    title: String,
    description: Option<String>,
    priority: Option<String>,
    due_date: Option<String>,
) -> Result<Task, String> {
    let now = chrono::Utc::now().to_rfc3339();
    let id = uuid::Uuid::new_v4().to_string();

    let query = format!(
        r#"CREATE tasks SET
            id = '{}',
            title = '{}',
            description = {},
            status = 'todo',
            priority = '{}',
            due_date = {},
            created_at = '{}',
            updated_at = '{}'"#,
        id,
        title.replace("'", "''"),
        description.as_ref().map(|d| format!("'{}'", d.replace("'", "''"))).unwrap_or("NULL".to_string()),
        priority.unwrap_or("medium".to_string()),
        due_date.as_ref().map(|d| format!("'{}'", d)).unwrap_or("NULL".to_string()),
        now,
        now
    );

    state.service.query(&query).await.map_err(|e| e.to_string())?;

    Ok(Task {
        id,
        title,
        description,
        status: "todo".to_string(),
        priority: priority.unwrap_or("medium".to_string()),
        due_date,
        created_at: now.clone(),
        updated_at: now,
    })
}

/// Update task status
#[tauri::command]
pub async fn update_task(
    state: State<'_, DatabaseState>,
    task_id: String,
    status: Option<String>,
    title: Option<String>,
    priority: Option<String>,
) -> Result<(), String> {
    let mut updates = vec![];
    let now = chrono::Utc::now().to_rfc3339();

    if let Some(s) = status { updates.push(format!("status = '{}'", s)); }
    if let Some(t) = title { updates.push(format!("title = '{}'", t.replace("'", "''"))); }
    if let Some(p) = priority { updates.push(format!("priority = '{}'", p)); }
    updates.push(format!("updated_at = '{}'", now));

    let query = format!(
        "UPDATE tasks SET {} WHERE id = '{}'",
        updates.join(", "),
        task_id
    );

    state.service.query(&query).await.map_err(|e| e.to_string())?;
    Ok(())
}

/// Delete a task
#[tauri::command]
pub async fn delete_task(
    state: State<'_, DatabaseState>,
    task_id: String,
) -> Result<(), String> {
    state.service.query(&format!("DELETE tasks WHERE id = '{}'", task_id))
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}