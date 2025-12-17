use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use edge_hive_wasm::WasmRuntime;
use serde_json::Value;
use std::sync::Arc;

use crate::state::ApiState;

/// Execute a WASM edge function
pub async fn execute_wasm_function(
    Path(name): Path<String>,
    State(state): State<Arc<ApiState>>,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    let wasm_path = state.data_dir.join("wasm-functions").join(format!("{}.wasm", name));

    if !wasm_path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }

    let runtime = WasmRuntime::new().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let result = runtime
        .execute_wasm(&wasm_path, payload)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(result))
}

/// Compile and deploy Rust code as WASM function
pub async fn deploy_rust_function(
    Path(name): Path<String>,
    State(state): State<Arc<ApiState>>,
    Json(code): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    let rust_code = code
        .get("rust_code")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;

    // In production:
    // 1. Create temp directory
    // 2. Write Rust code to src/lib.rs
    // 3. Run: cargo build --target wasm32-unknown-unknown --release
    // 4. Copy .wasm to data_dir/wasm-functions/
    // 5. Cache compilation results in edge-hive-cache

    // For now, mock the deployment
    let wasm_dir = state.data_dir.join("wasm-functions");
    std::fs::create_dir_all(&wasm_dir).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(serde_json::json!({
        "status": "deployed",
        "name": name,
        "runtime": "wasm",
        "note": "WASM compilation will be implemented with cargo integration"
    })))
}

/// List deployed WASM functions
pub async fn list_wasm_functions(
    State(state): State<Arc<ApiState>>,
) -> Result<Json<Vec<Value>>, StatusCode> {
    let wasm_dir = state.data_dir.join("wasm-functions");

    if !wasm_dir.exists() {
        return Ok(Json(vec![]));
    }

    let mut functions = vec![];
    let entries = std::fs::read_dir(&wasm_dir).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    for entry in entries.flatten() {
        if let Some(name) = entry.file_name().to_str() {
            if name.ends_with(".wasm") {
                functions.push(serde_json::json!({
                    "name": name.trim_end_matches(".wasm"),
                    "runtime": "wasm",
                    "size": entry.metadata().ok().map(|m| m.len()).unwrap_or(0)
                }));
            }
        }
    }

    Ok(Json(functions))
}
