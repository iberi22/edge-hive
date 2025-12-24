//! Integration tests for Edge Hive WASM Runtime
//!
//! These tests verify the complete functionality of the WASM runtime
//! with real WASM modules.

use edge_hive_wasm::{EdgeFunction, HostContext, LogLevel, WasmRuntime};
use serde_json::json;
use std::path::PathBuf;
use std::sync::Arc;

/// Test host context that tracks calls
#[derive(Debug, Clone)]
struct TestHostContext {
    queries: Arc<std::sync::Mutex<Vec<String>>>,
    logs: Arc<std::sync::Mutex<Vec<(LogLevel, String)>>>,
}

impl TestHostContext {
    fn new() -> Self {
        Self {
            queries: Arc::new(std::sync::Mutex::new(Vec::new())),
            logs: Arc::new(std::sync::Mutex::new(Vec::new())),
        }
    }

    #[allow(dead_code)]
    fn get_queries(&self) -> Vec<String> {
        self.queries.lock().unwrap().clone()
    }

    #[allow(dead_code)]
    fn get_logs(&self) -> Vec<(LogLevel, String)> {
        self.logs.lock().unwrap().clone()
    }
}

impl HostContext for TestHostContext {
    fn query(&self, sql: &str) -> Result<serde_json::Value, String> {
        self.queries.lock().unwrap().push(sql.to_string());
        Ok(json!({
            "result": "ok",
            "rows": []
        }))
    }

    fn log(&self, level: LogLevel, msg: &str) {
        self.logs.lock().unwrap().push((level, msg.to_string()));
    }
}

#[tokio::test]
async fn test_hello_world_wasm() {
    // Check if hello-world WASM exists
    let wasm_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/edge-functions/hello/target/wasm32-unknown-unknown/release/hello_edge_function.wasm");

    if !wasm_path.exists() {
        println!("⚠️  Skipping test: hello-world WASM not built");
        println!("   Build with: cd examples/edge-functions/hello && cargo build --target wasm32-unknown-unknown --release");
        return;
    }

    // Create runtime with test host
    let host = Arc::new(TestHostContext::new());
    let runtime = WasmRuntime::new(host.clone()).expect("Failed to create runtime");

    // Load function
    let function = runtime
        .load_function(&wasm_path)
        .expect("Failed to load function");

    // Test with default name
    let request = json!({});
    let response = function.execute(request).await.expect("Execution failed");

    assert!(response.is_object());
    assert_eq!(response["message"], "Hello, World!");

    // Test with custom name
    let request = json!({
        "name": "Edge Hive"
    });
    let response = function.execute(request).await.expect("Execution failed");

    assert_eq!(response["message"], "Hello, Edge Hive!");
}

#[tokio::test]
async fn test_runtime_with_custom_host() {
    let host = Arc::new(TestHostContext::new());
    let _runtime = WasmRuntime::new(host.clone()).expect("Failed to create runtime");
    // Test passes if runtime creation succeeds
}

#[tokio::test]
async fn test_invalid_wasm_bytes() {
    let host = Arc::new(TestHostContext::new());
    
    let result = EdgeFunction::from_bytes(&[0x00, 0x01, 0x02, 0x03], host);
    assert!(result.is_err());
}
