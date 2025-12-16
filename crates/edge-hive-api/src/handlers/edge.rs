//! Edge functions handlers (WASM runtime placeholder)

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
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
    State(_state): State<ApiState>,
    Path(function): Path<String>,
    Json(payload): Json<Value>,
) -> Result<Json<ExecutionResult>, StatusCode> {
    // Placeholder: would execute WASM function from edge-hive-wasm
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
    State(_state): State<ApiState>,
) -> Json<Vec<FunctionInfo>> {
    // Placeholder: would list from edge-hive-wasm registry
    Json(vec![
        FunctionInfo {
            name: "example-function".to_string(),
            runtime: "wasm".to_string(),
        },
    ])
}
