//! Edge functions handlers (WASM runtime placeholder)

use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::fs as async_fs;
use crate::state::ApiState;

#[derive(Serialize)]
pub struct FunctionInfo {
    name: String,
    runtime: String,
}

#[derive(Serialize)]
pub struct ExecutionResult {
    result: Value,
    execution_time_ms: u64,
}

/// Execute a WASM edge function
pub async fn execute_function(
    Extension(state): Extension<ApiState>,
    Path(function): Path<String>,
    Json(payload): Json<Value>,
) -> Result<Json<ExecutionResult>, StatusCode> {
    // Custom functions: load JSON template from disk
    let functions_dir = state.data_dir.join("edge-functions");
    let custom_path = functions_dir.join(format!("{}.json", function));

    if let Ok(contents) = async_fs::read_to_string(&custom_path).await {
        let template: Value = serde_json::from_str(&contents).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        return Ok(Json(ExecutionResult {
            result: serde_json::json!({
                "type": "custom",
                "function": function,
                "template": template,
                "input": payload,
            }),
            execution_time_ms: 1,
        }));
    }

    // Fallback: placeholder implementation
    Ok(Json(ExecutionResult {
        result: serde_json::json!({
            "message": "Function execution placeholder",
            "function": function,
            "input": payload,
        }),
        execution_time_ms: 5,
    }))
}

/// List available edge functions
pub async fn list_functions(
    Extension(state): Extension<ApiState>,
) -> Json<Vec<FunctionInfo>> {
    let mut out = vec![FunctionInfo {
        name: "example-function".to_string(),
        runtime: "wasm".to_string(),
    }];

    let functions_dir = state.data_dir.join("edge-functions");
    if let Ok(mut entries) = async_fs::read_dir(&functions_dir).await {
        while let Ok(Some(entry)) = entries.next_entry().await {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                out.push(FunctionInfo {
                    name: stem.to_string(),
                    runtime: "custom".to_string(),
                });
            }
        }
    }

    Json(out)
}
