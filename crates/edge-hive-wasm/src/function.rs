//! Edge Function abstraction
//!
//! Provides high-level interface for loading and executing WASM edge functions.

use crate::host::HostContext;
use crate::WasmError;
use anyhow::Result;
use serde_json::Value;
use std::path::Path;
use std::sync::Arc;
use wasmtime::*;

const MEMORY_LIMIT_BYTES: usize = 10 * 1024 * 1024; // 10 MB
const DEFAULT_FUEL: u64 = 1_000_000_000;

/// Represents a loaded edge function ready for execution
pub struct EdgeFunction<H: HostContext> {
    engine: Engine,
    module: Module,
    host: Arc<H>,
}

/// Enforces memory limits on the Wasm store.
struct StoreLimits {
    remaining_memory: usize,
}

impl ResourceLimiter for StoreLimits {
    fn memory_growing(
        &mut self,
        current: usize,
        desired: usize,
        _maximum: Option<usize>,
    ) -> Result<bool> {
        let new_mem = desired.saturating_sub(current);
        if new_mem > self.remaining_memory {
            return Ok(false); // Deny the request
        }
        self.remaining_memory -= new_mem;
        Ok(true)
    }

    fn table_growing(&mut self, _current: u32, _desired: u32, _maximum: Option<u32>) -> Result<bool> {
        Ok(true) // Allow table growing for now
    }
}

/// Holds the data for the Wasm store.
struct StoreData<H: HostContext> {
    host: Arc<H>,
    limits: StoreLimits,
}

impl<H: HostContext> EdgeFunction<H> {
    /// Creates a new Wasmtime engine with the correct configuration.
    fn create_engine() -> Result<Engine, WasmError> {
        let mut config = Config::new();
        config.async_support(true);
        config.consume_fuel(true);
        Engine::new(&config).map_err(|e| WasmError::Load(e.to_string()))
    }

    /// Load an edge function from a WASM file
    ///
    /// # Arguments
    /// * `path` - Path to the WASM file
    /// * `host` - Host context for database and logging
    pub fn load(path: &Path, host: Arc<H>) -> Result<Self, WasmError> {
        let engine = Self::create_engine()?;
        let module = Module::from_file(&engine, path).map_err(|e| WasmError::Load(e.to_string()))?;

        Ok(Self {
            engine,
            module,
            host,
        })
    }

    /// Load an edge function from WASM bytes
    ///
    /// # Arguments
    /// * `bytes` - WASM module bytes
    /// * `host` - Host context for database and logging
    pub fn from_bytes(bytes: &[u8], host: Arc<H>) -> Result<Self, WasmError> {
        crate::validate_wasm_bytes(bytes)?;
        let engine = Self::create_engine()?;
        let module = Module::new(&engine, bytes).map_err(|e| WasmError::Load(e.to_string()))?;

        Ok(Self {
            engine,
            module,
            host,
        })
    }

    /// Execute the edge function with a request
    ///
    /// # Arguments
    /// * `request` - JSON request data
    ///
    /// # Returns
    /// JSON response from the function
    pub async fn execute(&self, request: Value) -> Result<Value, WasmError> {
        let store_data = StoreData {
            host: self.host.clone(),
            limits: StoreLimits {
                remaining_memory: MEMORY_LIMIT_BYTES,
            },
        };
        let mut store = Store::new(&self.engine, store_data);
        store.limiter(|data| &mut data.limits);
        store.set_fuel(DEFAULT_FUEL).map_err(|e| WasmError::Call(e.to_string()))?;


        let mut linker = Linker::new(&self.engine);
        self.link_host_functions(&mut linker)?;

        let instance = linker
            .instantiate_async(&mut store, &self.module)
            .await
            .map_err(|e| WasmError::Instantiate(e.to_string()))?;

        let memory = instance
            .get_memory(&mut store, "memory")
            .ok_or_else(|| WasmError::Call("No 'memory' export found".into()))?;

        let request_json = serde_json::to_string(&request)
            .map_err(|e| WasmError::Call(format!("Failed to serialize request: {}", e)))?;

        let allocate = instance
            .get_typed_func::<i32, i32>(&mut store, "allocate")
            .map_err(|e| WasmError::Call(format!("No 'allocate' export: {}", e)))?;

        let req_len = request_json.len() as i32;
        let req_ptr = allocate
            .call_async(&mut store, req_len)
            .await
            .map_err(|e| WasmError::Call(format!("allocate failed: {}", e)))?;

        memory.write(&mut store, req_ptr as usize, request_json.as_bytes())
            .map_err(|e| WasmError::Call(format!("Failed to write to memory: {}", e)))?;

        let handle_request = instance
            .get_typed_func::<(i32, i32), i64>(&mut store, "handle_request")
            .map_err(|e| WasmError::Call(format!("No 'handle_request' export: {}", e)))?;

        let result = handle_request
            .call_async(&mut store, (req_ptr, req_len))
            .await
            .map_err(|e| WasmError::Call(format!("handle_request failed: {}", e)))?;

        let resp_ptr = (result & 0xFFFFFFFF) as i32;
        let resp_len = (result >> 32) as i32;

        let mut buffer = vec![0u8; resp_len as usize];
        memory.read(&store, resp_ptr as usize, &mut buffer)
            .map_err(|e| WasmError::Call(format!("Failed to read from memory: {}", e)))?;

        let response_json = String::from_utf8(buffer)
            .map_err(|e| WasmError::Call(format!("Invalid UTF-8 in response: {}", e)))?;

        if let Some(deallocate) = instance.get_typed_func::<(i32, i32), ()>(&mut store, "deallocate").ok() {
            let _ = deallocate.call_async(&mut store, (req_ptr, req_len)).await;
        }

        serde_json::from_str(&response_json)
            .map_err(|e| WasmError::Call(format!("Failed to parse response JSON: {}", e)))
    }

    /// Link host functions to the linker
    fn link_host_functions(&self, linker: &mut Linker<StoreData<H>>) -> Result<(), WasmError> {
        linker.func_wrap2_async(
            "edge_hive",
            "db_query",
            |mut caller: Caller<'_, StoreData<H>>, sql_ptr: i32, sql_len: i32| {
                Box::new(async move {
                    let host = caller.data().host.clone();
                    let memory = caller.get_export("memory").and_then(|e| e.into_memory()).ok_or_else(|| anyhow::anyhow!("No memory export"))?;

                    let mut buffer = vec![0u8; sql_len as usize];
                    memory.read(&caller, sql_ptr as usize, &mut buffer)?;
                    let sql = std::str::from_utf8(&buffer)?;

                    let result = host.query(sql);
                    let result_json = match result {
                        Ok(data) => serde_json::json!({ "ok": data }),
                        Err(e) => serde_json::json!({ "error": e.to_string() }),
                    };
                    let result_str = result_json.to_string();
                    let result_len = result_str.len() as i32;

                    let allocate = caller.get_export("allocate").and_then(|e| e.into_func()).ok_or_else(|| anyhow::anyhow!("No allocate export"))?;

                    let mut results = [Val::I32(0)];
                    allocate.call_async(&mut caller, &[Val::I32(result_len + 4)], &mut results).await?;
                    let result_ptr = results[0].unwrap_i32();

                    memory.write(&mut caller, result_ptr as usize, &result_len.to_le_bytes())?;
                    memory.write(&mut caller, (result_ptr + 4) as usize, result_str.as_bytes())?;

                    Ok(result_ptr + 4)
                })
            },
        ).map_err(|e| WasmError::Instantiate(e.to_string()))?;

        linker.func_wrap3_async(
            "edge_hive",
            "log",
            |mut caller: Caller<'_, StoreData<H>>, level: i32, msg_ptr: i32, msg_len: i32| {
                Box::new(async move {
                    let host = caller.data().host.clone();
                    let memory = caller.get_export("memory").and_then(|e| e.into_memory()).ok_or_else(|| anyhow::anyhow!("No memory export"))?;

                    let mut buffer = vec![0u8; msg_len as usize];
                    memory.read(&caller, msg_ptr as usize, &mut buffer)?;
                    let msg = std::str::from_utf8(&buffer)?;

                    let log_level = match level {
                        0 => crate::host::LogLevel::Trace,
                        1 => crate::host::LogLevel::Debug,
                        2 => crate::host::LogLevel::Info,
                        3 => crate::host::LogLevel::Warn,
                        _ => crate::host::LogLevel::Error,
                    };

                    host.log(log_level, msg);

                    Ok(())
                })
            },
        ).map_err(|e| WasmError::Instantiate(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::host::NoOpHostContext;

    #[test]
    fn test_edge_function_from_invalid_bytes() {
        let host = Arc::new(NoOpHostContext::new());
        let result = EdgeFunction::from_bytes(&[0x00, 0x01, 0x02], host);
        assert!(result.is_err());
    }
}
