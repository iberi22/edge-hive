use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiKey {
    pub id: String,
    pub key: String,
    pub name: String,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Backup {
    pub id: String,
    pub status: String,
    pub created_at: String,
}

#[tauri::command]
pub async fn create_api_key(name: String, role: String) -> Result<ApiKey, String> {
    Ok(ApiKey {
        id: "key1".into(),
        key: "ehk_12345".into(),
        name,
        role,
    })
}

#[tauri::command]
pub async fn get_api_keys() -> Result<Vec<ApiKey>, String> {
    Ok(vec![
        ApiKey {
            id: "key1".into(),
            key: "ehk_12345".into(),
            name: "test-key".into(),
            role: "admin".into(),
        }
    ])
}

#[tauri::command]
pub async fn revoke_api_key(id: String) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub async fn create_backup() -> Result<Backup, String> {
    Ok(Backup {
        id: "backup1".into(),
        status: "completed".into(),
        created_at: "2024-01-01".into(),
    })
}

#[tauri::command]
pub async fn get_backups() -> Result<Vec<Backup>, String> {
    Ok(vec![
        Backup {
            id: "backup1".into(),
            status: "completed".into(),
            created_at: "2024-01-01".into(),
        }
    ])
}
