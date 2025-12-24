use axum::{
    extract::{Multipart, Path, State},
    http::StatusCode,
    Json,
};
use edge_hive_wasm::WasmRuntime;
use serde::Deserialize;
use serde_json::Value;
use std::sync::Arc;
use tokio::fs;
use tokio::io::AsyncWriteExt;

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

async fn get_versions(function_dir: &std::path::Path) -> Result<Vec<u32>, StatusCode> {
    let mut versions = vec![];
    let mut read_dir = fs::read_dir(function_dir)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    while let Ok(Some(entry)) = read_dir.next_entry().await {
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "wasm" {
                    if let Some(stem) = path.file_stem() {
                        if let Some(stem_str) = stem.to_str() {
                            if stem_str.starts_with('v') {
                                if let Ok(version) = stem_str[1..].parse::<u32>() {
                                    versions.push(version);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(versions)
}

/// Deploy a new WASM function
pub async fn deploy_function(
    State(state): State<Arc<ApiState>>,
    Path(function_name): Path<String>,
    mut multipart: Multipart,
) -> Result<Json<Value>, StatusCode> {
    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().unwrap_or("").to_string();
        if name == "wasm_file" {
            let data = field.bytes().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            // Validate WASM magic bytes
            if data.len() < 4 || &data[0..4] != b"\0asm" {
                return Err(StatusCode::BAD_REQUEST);
            }

            let function_dir = state.data_dir.join("plugins").join(&function_name);
            fs::create_dir_all(&function_dir)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            let versions = get_versions(&function_dir).await?;
            let next_version = versions.iter().max().unwrap_or(&0) + 1;
            let wasm_path = function_dir.join(format!("v{}.wasm", next_version));
            let mut file = fs::File::create(&wasm_path)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            file.write_all(&data)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            // Update active version
            let active_path = function_dir.join("active.txt");
            fs::write(active_path, next_version.to_string())
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            return Ok(Json(serde_json::json!({
                "status": "deployed",
                "name": function_name,
                "version": next_version
            })));
        }
    }

    Err(StatusCode::BAD_REQUEST)
}

/// Get all versions of a WASM function
pub async fn get_function_versions(
    State(state): State<Arc<ApiState>>,
    Path(function_name): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    let function_dir = state.data_dir.join("plugins").join(&function_name);

    if !function_dir.exists() {
        return Err(StatusCode::NOT_FOUND);
    }

    let versions = get_versions(&function_dir).await?;

    let active_path = function_dir.join("active.txt");
    let active_version = fs::read_to_string(active_path)
        .await
        .ok()
        .and_then(|s| s.parse::<u32>().ok());

    Ok(Json(serde_json::json!({
        "name": function_name,
        "versions": versions,
        "active_version": active_version,
    })))
}

#[derive(Deserialize)]
pub struct RollbackPayload {
    pub version: u32,
}

/// Rollback a WASM function to a specific version
pub async fn rollback_function(
    State(state): State<Arc<ApiState>>,
    Path(function_name): Path<String>,
    Json(payload): Json<RollbackPayload>,
) -> Result<Json<Value>, StatusCode> {
    let function_dir = state.data_dir.join("plugins").join(&function_name);

    if !function_dir.exists() {
        return Err(StatusCode::NOT_FOUND);
    }

    let wasm_path = function_dir.join(format!("v{}.wasm", payload.version));
    if !wasm_path.exists() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let active_path = function_dir.join("active.txt");
    fs::write(active_path, payload.version.to_string())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(serde_json::json!({
        "status": "rolled_back",
        "name": function_name,
        "active_version": payload.version,
    })))
}

/// Delete a WASM function
pub async fn delete_function(
    State(state): State<Arc<ApiState>>,
    Path(function_name): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    let function_dir = state.data_dir.join("plugins").join(&function_name);

    if !function_dir.exists() {
        return Err(StatusCode::NOT_FOUND);
    }

    fs::remove_dir_all(&function_dir)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(serde_json::json!({
        "status": "deleted",
        "name": function_name,
    })))
}
