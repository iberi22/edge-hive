use tauri::State;
use std::sync::{Arc, Mutex};
use edge_hive_wasm::{PluginManager, validate_wasm_bytes};
use serde::{Serialize, Deserialize};
use std::path::{Path, PathBuf};
use std::fs;
use base64::{Engine as _, engine::general_purpose};

pub struct FunctionState {
    pub manager: Arc<Mutex<PluginManager>>,
    pub plugins_dir: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EdgeFunctionDTO {
    pub id: String,
    pub name: String,
    pub status: String,
    pub invocations: u32,
    pub last_run: String,
    pub description: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionVersion {
    pub version: String,
    pub created_at: String,
    pub size_bytes: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionInfo {
    pub id: String,
    pub name: String,
    pub version: String,
}

/// Helper to load existing plugins on startup
pub fn load_existing_plugins(plugins_dir: &Path) -> PluginManager {
    let mut manager = PluginManager::new();

    if let Ok(entries) = fs::read_dir(plugins_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                // Find latest version
                let mut wasm_files = Vec::new();
                if let Ok(files) = fs::read_dir(&path) {
                    for f in files.flatten() {
                         let fp = f.path();
                         if fp.extension().and_then(|s| s.to_str()) == Some("wasm") {
                             wasm_files.push(fp);
                         }
                    }
                }
                // Sort by name (v{timestamp}) implies string sort works for timestamps if same length,
                // but timestamps vary in length over decades.
                // Assuming v{timestamp} where timestamp is seconds. Standard length mostly.
                wasm_files.sort();

                if let Some(latest) = wasm_files.last() {
                    let _ = manager.load(latest); // Ignore errors for now
                }
            }
        }
    }
    manager
}

#[tauri::command]
pub async fn list_functions(state: State<'_, FunctionState>) -> Result<Vec<EdgeFunctionDTO>, String> {
    let manager = state.manager.lock().map_err(|e| e.to_string())?;
    let plugins = manager.list();

    let functions = plugins.into_iter().enumerate().map(|(i, p)| EdgeFunctionDTO {
        id: format!("fn-{}", i),
        name: p.name.clone(),
        status: "active".to_string(),
        invocations: 0,
        last_run: "never".to_string(),
        description: p.description.clone(),
        version: p.version.clone(),
    }).collect();

    Ok(functions)
}

#[tauri::command]
pub async fn invoke_function(
    state: State<'_, FunctionState>,
    id: String,
    _payload: serde_json::Value
) -> Result<serde_json::Value, String> {
    // Parsing ID "fn-{index}"
    let index_str = id.strip_prefix("fn-").ok_or("Invalid ID format")?;
    let index: usize = index_str.parse().map_err(|_| "Invalid ID format")?;

    let mut manager = state.manager.lock().map_err(|e| e.to_string())?;

    if let Some(plugin) = manager.get(index) {
        Ok(serde_json::json!({
            "status": 200,
            "result": format!("Executed plugin {}", plugin.info().name)
        }))
    } else {
        Err("Function not found".into())
    }
}

#[tauri::command]
pub async fn deploy_function(
    state: State<'_, FunctionState>,
    name: String,
    wasm_base64: String,
) -> Result<FunctionInfo, String> {
    // 1. Decode Base64
    let bytes = general_purpose::STANDARD
        .decode(&wasm_base64)
        .map_err(|e| format!("Base64 decode failed: {}", e))?;

    // 2. Validate Magic Bytes
    validate_wasm_bytes(&bytes).map_err(|e| e.to_string())?;

    // 3. Ensure directory exists
    let func_dir = state.plugins_dir.join(&name);
    fs::create_dir_all(&func_dir).map_err(|e| e.to_string())?;

    // 4. Determine version (timestamp)
    let version = chrono::Utc::now().timestamp().to_string();
    let file_path = func_dir.join(format!("v{}.wasm", version));

    // 5. Save file
    fs::write(&file_path, &bytes).map_err(|e| e.to_string())?;

    // 6. Update Runtime (Hot Reload)
    let mut manager = state.manager.lock().map_err(|e| e.to_string())?;
    manager.unload(&name);
    manager.load(&file_path).map_err(|e| e.to_string())?;

    Ok(FunctionInfo {
        id: format!("fn-{}", manager.plugins().len() - 1),
        name,
        version,
    })
}

#[tauri::command]
pub async fn get_function_versions(
    state: State<'_, FunctionState>,
    name: String,
) -> Result<Vec<FunctionVersion>, String> {
    let func_dir = state.plugins_dir.join(&name);
    if !func_dir.exists() {
        return Ok(vec![]);
    }

    let mut versions = Vec::new();
    let entries = fs::read_dir(func_dir).map_err(|e| e.to_string())?;

    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("wasm") {
             if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                 if let Some(ver) = stem.strip_prefix("v") {
                     let metadata = entry.metadata().map_err(|e| e.to_string())?;
                     versions.push(FunctionVersion {
                         version: ver.to_string(),
                         created_at: ver.to_string(), // Timestamp is the version
                         size_bytes: metadata.len(),
                     });
                 }
             }
        }
    }

    // Sort desc
    versions.sort_by(|a, b| b.version.cmp(&a.version));

    Ok(versions)
}

#[tauri::command]
pub async fn delete_function(
    state: State<'_, FunctionState>,
    name: String,
) -> Result<(), String> {
    // Unload from runtime
    let mut manager = state.manager.lock().map_err(|e| e.to_string())?;
    manager.unload(&name);

    // Delete files
    let func_dir = state.plugins_dir.join(&name);
    if func_dir.exists() {
        fs::remove_dir_all(func_dir).map_err(|e| e.to_string())?;
    }
    Ok(())
}

