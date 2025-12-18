use tauri::State;
use std::sync::{Arc, Mutex};
use edge_hive_wasm::PluginManager;
use serde::{Serialize, Deserialize};

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
