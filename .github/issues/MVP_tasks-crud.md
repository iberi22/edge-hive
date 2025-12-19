---
title: "[MVP] Tasks CRUD with SurrealDB Persistence"
labels:
  - enhancement
  - backend
  - jules
  - P2
assignees: []
---

## Description

Implement task management with SurrealDB persistence.

## Tasks

- [ ] Create `tasks_commands.rs` module
- [ ] Define `Task` struct with id, title, description, status, priority, due_date
- [ ] Implement CRUD commands:
  - `get_tasks` - List all tasks
  - `create_task` - Create new task
  - `update_task` - Update task status/details
  - `delete_task` - Remove task
- [ ] Integrate with `DatabaseService` from `edge-hive-db`
- [ ] Update `lib.rs` to register commands
- [ ] Update `tauriClient.ts` with frontend bindings

## Technical Details

```rust
// tasks_commands.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub status: String, // todo, in_progress, done
    pub priority: String, // low, medium, high
    pub due_date: Option<String>,
    pub created_at: String,
}

#[tauri::command]
pub async fn get_tasks(state: State<'_, DatabaseState>) -> Result<Vec<Task>, String> {
    // SELECT * FROM tasks ORDER BY created_at DESC
}

#[tauri::command]
pub async fn create_task(title: String, description: Option<String>) -> Result<Task, String> {
    // CREATE tasks SET title = $title, status = 'todo', ...
}
```

## Acceptance Criteria

- [ ] Tasks persist across app restarts
- [ ] CRUD operations work correctly
- [ ] Tasks page displays real data

## Estimated Effort
3-4 hours
