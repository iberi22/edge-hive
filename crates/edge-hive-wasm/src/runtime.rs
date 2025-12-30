use crate::function::EdgeFunction;
use crate::host::HostContext;
use anyhow::{Context, Result};
use serde_json::Value;
use std::path::Path;
use std::sync::Arc;
use wasmtime::*;

/// Edge function runtime using WebAssembly with generic host context
pub struct WasmRuntime<H: HostContext> {
    engine: Engine,
    host: Arc<H>,
}

impl<H: HostContext> WasmRuntime<H> {
    /// Create a new WASM runtime with the given host context
    ///
    /// # Arguments
    /// * `host` - Host context implementation for database and logging
    pub fn new(host: Arc<H>) -> Result<Self> {
        let mut config = Config::new();
        config.wasm_simd(true);
        config.wasm_bulk_memory(true);
        config.cranelift_opt_level(OptLevel::Speed);
        config.async_support(true);
        config.consume_fuel(true); // Security: Limit CPU usage

        let engine = Engine::new(&config)
            .context("Failed to create Wasmtime engine")?;

        Ok(Self { engine, host })
    }

    /// Get the engine instance
    pub fn engine(&self) -> &Engine {
        &self.engine
    }

    /// Load an edge function from a WASM file
    ///
    /// # Arguments
    /// * `path` - Path to the WASM file
    pub fn load_function(&self, path: &Path) -> Result<EdgeFunction<H>> {
        EdgeFunction::load(&self.engine, path, self.host.clone())
            .map_err(|e| anyhow::anyhow!(e))
    }

    /// Load an edge function from WASM bytes
    ///
    /// # Arguments
    /// * `bytes` - WASM module bytes
    pub fn load_function_from_bytes(&self, bytes: &[u8]) -> Result<EdgeFunction<H>> {
        EdgeFunction::from_bytes(&self.engine, bytes, self.host.clone())
            .map_err(|e| anyhow::anyhow!(e))
    }

    /// Compile Rust code to WASM and execute
    pub async fn execute_rust(&self, code: &str, input: Value) -> Result<Value> {
        // In production, this would:
        // 1. Compile Rust to WASM using cargo/rustc
        // 2. Cache compiled WASM in edge-hive-cache
        // 3. Execute with Wasmtime
        // 4. Return result

        // For now, return mock result
        Ok(serde_json::json!({
            "result": "WASM execution not yet implemented",
            "input": input,
            "code_hash": format!("{:x}", md5::compute(code))
        }))
    }

    /// Execute pre-compiled WASM module
    pub async fn execute_wasm(&self, wasm_path: &Path, input: Value) -> Result<Value> {
        let function = self.load_function(wasm_path)?;
        function.execute(input).await.map_err(|e| anyhow::anyhow!(e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::host::NoOpHostContext;

    #[tokio::test]
    async fn test_wasm_runtime_init() {
        let host = Arc::new(NoOpHostContext);
        let runtime = WasmRuntime::new(host);
        assert!(runtime.is_ok());
    }

    #[tokio::test]
    async fn test_execute_rust_mock() {
        let host = Arc::new(NoOpHostContext);
        let runtime = WasmRuntime::new(host).unwrap();
        let code = r#"
            pub fn handle() -> i32 {
                42
            }
        "#;
        let input = serde_json::json!({"test": true});

        let result = runtime.execute_rust(code, input).await;
        assert!(result.is_ok());
    }
}
