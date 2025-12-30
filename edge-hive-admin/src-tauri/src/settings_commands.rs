//! Settings Commands
//!
//! Provides configuration management:
//! - API Keys generation and revocation
//! - Backup/restore functionality
//! - SMTP configuration
//! - Access logs

use tauri::{State, AppHandle};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;
use tokio::sync::RwLock;
use std::sync::Arc;
use tokio::fs as async_fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

/// API Key record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub id: String,
    pub name: String,
    pub key: String,  // Only shown once at creation
    pub role: String,
    pub created_at: String,
    pub last_used: Option<String>,
}

/// API Key response (hides actual key after creation)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyResponse {
    pub id: String,
    pub name: String,
    pub key_preview: String,  // Shows last 4 chars only
    pub role: String,
    pub created_at: String,
    pub last_used: Option<String>,
}

/// Backup record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Backup {
    pub id: String,
    pub name: String,
    pub size: String,
    pub created_at: String,
    pub status: String,
}

/// SMTP Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmtpConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub from_email: String,
    pub secure: bool,
}

/// Access log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessLog {
    pub id: String,
    pub timestamp: String,
    pub user: String,
    pub action: String,
    pub ip: String,
    pub success: bool,
}

/// Settings state
pub struct SettingsState {
    pub api_keys: Arc<RwLock<HashMap<String, ApiKey>>>,
    pub backups_dir: PathBuf,
    pub smtp_config: Arc<RwLock<Option<SmtpConfig>>>,
    pub access_logs: Arc<RwLock<Vec<AccessLog>>>,
}

impl SettingsState {
    pub fn new(app_data_dir: PathBuf) -> Self {
        let backups_dir = app_data_dir.join("backups");
        let _ = fs::create_dir_all(&backups_dir);

        Self {
            api_keys: Arc::new(RwLock::new(HashMap::new())),
            backups_dir,
            smtp_config: Arc::new(RwLock::new(None)),
            access_logs: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

/// Generate a secure random API key
fn generate_api_key() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let bytes: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
    format!("ehk_{}", hex::encode(bytes))
}

/// Get all API keys (with hidden actual keys)
#[tauri::command]
pub async fn get_api_keys(state: State<'_, SettingsState>) -> Result<Vec<ApiKeyResponse>, String> {
    let keys = state.api_keys.read().await;
    Ok(keys.values().map(|k| ApiKeyResponse {
        id: k.id.clone(),
        name: k.name.clone(),
        key_preview: format!("...{}", &k.key[k.key.len()-4..]),
        role: k.role.clone(),
        created_at: k.created_at.clone(),
        last_used: k.last_used.clone(),
    }).collect())
}

/// Create a new API key (returns full key only once)
#[tauri::command]
pub async fn create_api_key(
    state: State<'_, SettingsState>,
    name: String,
    role: String,
) -> Result<ApiKey, String> {
    if name.is_empty() {
        return Err("Name is required".to_string());
    }

    let id = uuid::Uuid::new_v4().to_string();
    let key = generate_api_key();
    let now = chrono::Utc::now().to_rfc3339();

    let api_key = ApiKey {
        id: id.clone(),
        name,
        key: key.clone(),
        role,
        created_at: now,
        last_used: None,
    };

    let mut keys = state.api_keys.write().await;
    keys.insert(id, api_key.clone());

    Ok(api_key)
}

/// Revoke an API key
#[tauri::command]
pub async fn revoke_api_key(
    state: State<'_, SettingsState>,
    key_id: String,
) -> Result<(), String> {
    let mut keys = state.api_keys.write().await;
    keys.remove(&key_id).ok_or("API key not found")?;
    Ok(())
}

/// Get all backups
#[tauri::command]
pub async fn get_backups(state: State<'_, SettingsState>) -> Result<Vec<Backup>, String> {
    let mut backups = Vec::new();

    if let Ok(entries) = fs::read_dir(&state.backups_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |e| e == "tar" || e == "gz" || e == "zip") {
                let name = entry.file_name().to_string_lossy().to_string();
                let metadata = entry.metadata().map_err(|e| e.to_string())?;
                let size = format_size(metadata.len());
                let created = metadata.modified()
                    .map(|t| {
                        let dt: chrono::DateTime<chrono::Utc> = t.into();
                        dt.format("%Y-%m-%d %H:%M").to_string()
                    })
                    .unwrap_or_else(|_| "Unknown".to_string());

                backups.push(Backup {
                    id: name.clone(),
                    name,
                    size,
                    created_at: created,
                    status: "completed".to_string(),
                });
            }
        }
    }

    Ok(backups)
}

/// Create a new backup (snapshot of storage)
#[tauri::command]
pub async fn create_backup(
    state: State<'_, SettingsState>,
    storage_state: State<'_, crate::storage_commands::StorageState>,
) -> Result<Backup, String> {
    let now = chrono::Utc::now();
    let backup_name = format!("backup_{}.tar.gz", now.format("%Y%m%d_%H%M%S"));
    let backup_path = state.backups_dir.join(&backup_name);

    // Create a simple tar.gz of the storage directory
    let storage_root = &storage_state.root_dir;

    // Use flate2 and tar for compression
    let tar_gz = fs::File::create(&backup_path).map_err(|e| e.to_string())?;
    let enc = flate2::write::GzEncoder::new(tar_gz, flate2::Compression::default());
    let mut tar = tar::Builder::new(enc);

    if storage_root.exists() {
        tar.append_dir_all("storage", storage_root).map_err(|e| e.to_string())?;
    }

    tar.finish().map_err(|e| e.to_string())?;

    let metadata = fs::metadata(&backup_path).map_err(|e| e.to_string())?;

    Ok(Backup {
        id: backup_name.clone(),
        name: backup_name,
        size: format_size(metadata.len()),
        created_at: now.format("%Y-%m-%d %H:%M").to_string(),
        status: "completed".to_string(),
    })
}

/// Restore from a backup
#[tauri::command]
pub async fn restore_backup(
    state: State<'_, SettingsState>,
    storage_state: State<'_, crate::storage_commands::StorageState>,
    backup_id: String,
) -> Result<(), String> {
    let backup_path = state.backups_dir.join(&backup_id);

    if !backup_path.exists() {
        return Err("Backup not found".to_string());
    }

    let storage_root = &storage_state.root_dir;

    // Clear storage directory
    if storage_root.exists() {
        fs::remove_dir_all(storage_root).map_err(|e| e.to_string())?;
    }
    fs::create_dir_all(storage_root).map_err(|e| e.to_string())?;

    // Extract backup
    let tar_gz = fs::File::open(&backup_path).map_err(|e| e.to_string())?;
    let dec = flate2::read::GzDecoder::new(tar_gz);
    let mut archive = tar::Archive::new(dec);

    archive.unpack(storage_root.parent().unwrap_or(storage_root))
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// Save SMTP configuration
#[tauri::command]
pub async fn save_smtp_config(
    state: State<'_, SettingsState>,
    host: String,
    port: u16,
    username: String,
    password: String,
    from_email: String,
    secure: bool,
) -> Result<(), String> {
    let config = SmtpConfig {
        host,
        port,
        username,
        password,
        from_email,
        secure,
    };

    let mut smtp = state.smtp_config.write().await;
    *smtp = Some(config);

    Ok(())
}

/// Get SMTP configuration (hides password)
#[tauri::command]
pub async fn get_smtp_config(state: State<'_, SettingsState>) -> Result<Option<SmtpConfig>, String> {
    let smtp = state.smtp_config.read().await;
    Ok(smtp.clone().map(|mut c| {
        c.password = "********".to_string();
        c
    }))
}

/// Send test email
#[tauri::command]
pub async fn send_test_email(
    state: State<'_, SettingsState>,
    to: String,
) -> Result<(), String> {
    let smtp = state.smtp_config.read().await;
    let config = smtp.as_ref().ok_or("SMTP not configured")?;

    // For now, just validate config exists
    // Real implementation would use lettre crate
    if config.host.is_empty() {
        return Err("SMTP host not configured".to_string());
    }

    // Log the attempt
    let mut logs = state.access_logs.write().await;
    logs.push(AccessLog {
        id: uuid::Uuid::new_v4().to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        user: "system".to_string(),
        action: format!("Test email sent to {}", to),
        ip: "127.0.0.1".to_string(),
        success: true,
    });

    Ok(())
}

/// Get access logs
#[tauri::command]
pub async fn get_access_logs(state: State<'_, SettingsState>) -> Result<Vec<AccessLog>, String> {
    let logs = state.access_logs.read().await;
    Ok(logs.iter().rev().take(100).cloned().collect())
}

/// Add access log entry
#[tauri::command]
pub async fn add_access_log(
    state: State<'_, SettingsState>,
    user: String,
    action: String,
    success: bool,
) -> Result<(), String> {
    let mut logs = state.access_logs.write().await;
    logs.push(AccessLog {
        id: uuid::Uuid::new_v4().to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        user,
        action,
        ip: "127.0.0.1".to_string(),
        success,
    });
    Ok(())
}

/// Format bytes to human readable
fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

#[tauri::command]
pub async fn get_config(app_handle: AppHandle) -> Result<String, String> {
    let config_path = app_handle.path().app_config_dir()
        .ok_or_else(|| "Failed to resolve config directory".to_string())?
        .join("config.toml");

    let mut contents = String::new();
    async_fs::File::open(config_path)
        .await
        .map_err(|e| e.to_string())?
        .read_to_string(&mut contents)
        .await
        .map_err(|e| e.to_string())?;
    Ok(contents)
}

#[tauri::command]
pub async fn save_config(app_handle: AppHandle, contents: String) -> Result<(), String> {
    let config_path = app_handle.path().app_config_dir()
        .ok_or_else(|| "Failed to resolve config directory".to_string())?
        .join("config.toml");

    async_fs::File::create(config_path)
        .await
        .map_err(|e| e.to_string())?
        .write_all(contents.as_bytes())
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}
