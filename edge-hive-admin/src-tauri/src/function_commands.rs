use tauri::State;
use std::sync::{Arc, Mutex};
use edge_hive_wasm::PluginManager;
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::{Path, PathBuf};

fn sanitize_function_name(name: &str) -> String {
    name.chars()
        .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
        .collect()
}

pub struct FunctionState {
    pub manager: Arc<Mutex<PluginManager>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EdgeFunctionDTO {
    pub id: String,
    pub name: String,
    pub status: String,
    pub invocations: u32,
    pub last_run: String,
    pub description: String,
}

#[tauri::command]
pub async fn list_functions(state: State<'_, FunctionState>) -> Result<Vec<EdgeFunctionDTO>, String> {
    let manager = state.manager.lock().map_err(|e| e.to_string())?;
    let plugins = manager.list();

    // Convert PluginInfo to EdgeFunctionDTO
    let functions = plugins.into_iter().enumerate().map(|(i, p)| EdgeFunctionDTO {
        id: format!("fn-{}", i), // Simple ID generation
        name: p.name.clone(),
        status: "active".to_string(), // Assume active if loaded
        invocations: 0, // Not tracked yet in PluginInfo
        last_run: "never".to_string(),
        description: p.description.clone(),
    }).collect();

    Ok(functions)
}

#[tauri::command]
pub async fn invoke_function(
    state: State<'_, FunctionState>,
    id: String,
    _payload: serde_json::Value
) -> Result<serde_json::Value, String> {
    // In a real scenario, we'd find the plugin by ID and call it.
    // Parsing ID "fn-{index}"
    let index_str = id.strip_prefix("fn-").ok_or("Invalid ID format")?;
    let index: usize = index_str.parse().map_err(|_| "Invalid ID format")?;

    let mut manager = state.manager.lock().map_err(|e| e.to_string())?;

    if let Some(plugin) = manager.get(index) {
        // For demo, just call a "main" or "handle" function if it existed.
        // Since we don't have arguments mapping yet, just return success mock.
        // plugin.call("handle", &[]).map_err(|e| e.to_string())?;
        Ok(serde_json::json!({
            "status": 200,
            "result": format!("Executed plugin {}", plugin.info().name)
        }))
    } else {
        Err("Function not found".into())
    }
}


/// WASM magic bytes: \0asm
const WASM_MAGIC: [u8; 4] = [0x00, 0x61, 0x73, 0x6D];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionVersion {
    pub version: u32,
    pub created_at: String,
    pub size_bytes: u64,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionInfo {
    pub id: String,
    pub name: String,
    pub runtime: String,
    pub memory_mb: u32,
    pub timeout_seconds: u32,
    pub invocations: u32,
    pub status: String,
}

/// Deploy a new function version
#[tauri::command]
pub async fn deploy_function(
    state: State<'_, FunctionState>,
    name: String,
    wasm_bytes: Vec<u8>,  // Base64 decoded by frontend
) -> Result<FunctionInfo, String> {
    let name = sanitize_function_name(&name);
    // Validate WASM magic bytes
    if wasm_bytes.len() < 4 || wasm_bytes[..4] != WASM_MAGIC {
        return Err("Invalid WASM file: missing magic bytes".to_string());
    }

    // Get plugins directory
    let plugins_dir = PathBuf::from("plugins").join(&name);
    fs::create_dir_all(&plugins_dir).map_err(|e| e.to_string())?;

    // Find next version number
    let versions: Vec<u32> = fs::read_dir(&plugins_dir)
        .map_err(|e| e.to_string())?
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            e.file_name()
                .to_string_lossy()
                .strip_prefix("v")
                .and_then(|s| s.strip_suffix(".wasm"))
                .and_then(|s| s.parse().ok())
        })
        .collect();

    let next_version = versions.iter().max().unwrap_or(&0) + 1;

    // Save WASM file
    let wasm_path = plugins_dir.join(format!("v{}.wasm", next_version));
    fs::write(&wasm_path, &wasm_bytes).map_err(|e| e.to_string())?;

    // Update active symlink/marker
    let active_path = plugins_dir.join("active.txt");
    fs::write(&active_path, format!("{}", next_version)).map_err(|e| e.to_string())?;

    // Reload the plugin in the manager
    let mut manager = state.manager.lock().map_err(|e| e.to_string())?;
    manager.unload(&name).ok(); // Unload existing version if any
    manager.load_plugin(&name, &wasm_path).map_err(|e| e.to_string())?;

    Ok(FunctionInfo {
        id: name.clone(),
        name,
        runtime: "wasm".to_string(),
        memory_mb: 128,
        timeout_seconds: 30,
        invocations: 0,
        status: "deployed".to_string(),
    })
}

/// Get version history for a function
#[tauri::command]
pub async fn get_function_versions(name: String) -> Result<Vec<FunctionVersion>, String> {
    let name = sanitize_function_name(&name);
    let plugins_dir = PathBuf::from("plugins").join(&name);

    if !plugins_dir.exists() {
        return Ok(vec![]);
    }

    // Read active version
    let active_path = plugins_dir.join("active.txt");
    let active_version: u32 = fs::read_to_string(&active_path)
        .ok()
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(0);

    let mut versions = Vec::new();

    for entry in fs::read_dir(&plugins_dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let filename = entry.file_name().to_string_lossy().to_string();

        if let Some(v) = filename.strip_prefix("v").and_then(|s| s.strip_suffix(".wasm")) {
            if let Ok(version) = v.parse::<u32>() {
                let metadata = entry.metadata().map_err(|e| e.to_string())?;
                let created = metadata.modified()
                    .map(|t| {
                        let dt: chrono::DateTime<chrono::Utc> = t.into();
                        dt.to_rfc3339()
                    })
                    .unwrap_or_else(|_| "Unknown".to_string());

                versions.push(FunctionVersion {
                    version,
                    created_at: created,
                    size_bytes: metadata.len(),
                    active: version == active_version,
                });
            }
        }
    }

    versions.sort_by(|a, b| b.version.cmp(&a.version));
    Ok(versions)
}

/// Rollback to a previous version
#[tauri::command]
pub async fn rollback_function(
    state: State<'_, FunctionState>,
    name: String,
    version: u32,
) -> Result<(), String> {
    let name = sanitize_function_name(&name);
    let plugins_dir = PathBuf::from("plugins").join(&name);
    let version_path = plugins_dir.join(format!("v{}.wasm", version));

    if !version_path.exists() {
        return Err(format!("Version {} not found", version));
    }

    // Update active version
    let active_path = plugins_dir.join("active.txt");
    fs::write(&active_path, format!("{}", version)).map_err(|e| e.to_string())?;

    // Reload the plugin in the manager
    let mut manager = state.manager.lock().map_err(|e| e.to_string())?;
    manager.unload(&name).ok(); // Unload existing version if any
    manager.load_plugin(&name, &version_path).map_err(|e| e.to_string())?;

    Ok(())
}

/// Delete a function entirely
#[tauri::command]
pub async fn delete_function(
    state: State<'_, FunctionState>,
    name: String,
) -> Result<(), String> {
    let name = sanitize_function_name(&name);
    let plugins_dir = PathBuf::from("plugins").join(&name);

    // Unload from manager first
    let mut manager = state.manager.lock().map_err(|e| e.to_string())?;
    manager.unload(&name).ok(); // Ignore if not loaded

    if plugins_dir.exists() {
        fs::remove_dir_all(&plugins_dir).map_err(|e| e.to_string())?;
    }

    Ok(())
}
