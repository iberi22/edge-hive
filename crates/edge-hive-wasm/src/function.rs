//! Edge Function abstraction
//!
//! Provides high-level interface for loading and executing WASM edge functions.

use crate::host::HostContext;
use crate::WasmError;
use serde_json::Value;
use std::path::Path;
use std::sync::Arc;
use wasmtime::*;

/// Represents a loaded edge function ready for execution
pub struct EdgeFunction<H: HostContext> {
    engine: Engine,
    module: Module,
    host: Arc<H>,
}

impl<H: HostContext> EdgeFunction<H> {
    /// Load an edge function from a WASM file
    ///
    /// # Arguments
    /// * `path` - Path to the WASM file
    /// * `host` - Host context for database and logging
    pub fn load(path: &Path, host: Arc<H>) -> Result<Self, WasmError> {
        let engine = Engine::default();
        let module = Module::from_file(&engine, path)
            .map_err(|e| WasmError::Load(e.to_string()))?;

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

        let engine = Engine::default();
        let module = Module::new(&engine, bytes)
            .map_err(|e| WasmError::Load(e.to_string()))?;

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
        // Create store with host context state
        let mut store = Store::new(&self.engine, self.host.clone());

        // Create linker with host functions
        let mut linker = Linker::new(&self.engine);
        self.link_host_functions(&mut linker)?;

        // Instantiate the module
        let instance = linker
            .instantiate_async(&mut store, &self.module)
            .await
            .map_err(|e| WasmError::Instantiate(e.to_string()))?;

        // Get memory for string passing
        let memory = instance
            .get_memory(&mut store, "memory")
            .ok_or_else(|| WasmError::Call("No 'memory' export found".into()))?;

        // Serialize request to JSON string
        let request_json = serde_json::to_string(&request)
            .map_err(|e| WasmError::Call(format!("Failed to serialize request: {}", e)))?;

        // Allocate memory for request in WASM
        let allocate = instance
            .get_typed_func::<i32, i32>(&mut store, "allocate")
            .map_err(|e| WasmError::Call(format!("No 'allocate' export: {}", e)))?;

        let req_len = request_json.len() as i32;
        let req_ptr = allocate
            .call_async(&mut store, req_len)
            .await
            .map_err(|e| WasmError::Call(format!("allocate failed: {}", e)))?;

        // Write request data to WASM memory
        let mem_data = memory.data_mut(&mut store);
        mem_data[req_ptr as usize..(req_ptr as usize + req_len as usize)]
            .copy_from_slice(request_json.as_bytes());

        // Call the handle_request function
        let handle_request = instance
            .get_typed_func::<(i32, i32), i32>(&mut store, "handle_request")
            .map_err(|e| WasmError::Call(format!("No 'handle_request' export: {}", e)))?;

        let resp_ptr = handle_request
            .call_async(&mut store, (req_ptr, req_len))
            .await
            .map_err(|e| WasmError::Call(format!("handle_request failed: {}", e)))?;

        // Read response length (stored at resp_ptr - 4)
        let resp_len = {
            let mem_data = memory.data(&store);
            i32::from_le_bytes([
                mem_data[resp_ptr as usize - 4],
                mem_data[resp_ptr as usize - 3],
                mem_data[resp_ptr as usize - 2],
                mem_data[resp_ptr as usize - 1],
            ])
        };

        // Read response data
        let response_json = {
            let mem_data = memory.data(&store);
            let response_bytes = &mem_data[resp_ptr as usize..(resp_ptr as usize + resp_len as usize)];
            std::str::from_utf8(response_bytes)
                .map_err(|e| WasmError::Call(format!("Invalid UTF-8 in response: {}", e)))?
                .to_string()
        };

        // Deallocate request memory
        let deallocate = instance
            .get_typed_func::<(i32, i32), ()>(&mut store, "deallocate")
            .ok();

        if let Some(dealloc) = deallocate {
            let _ = dealloc.call_async(&mut store, (req_ptr, req_len)).await;
        }

        // Parse and return response
        serde_json::from_str(&response_json)
            .map_err(|e| WasmError::Call(format!("Failed to parse response JSON: {}", e)))
    }

    /// Link host functions to the linker
    fn link_host_functions(&self, linker: &mut Linker<Arc<H>>) -> Result<(), WasmError> {
        // db_query(sql_ptr: i32, sql_len: i32) -> result_ptr: i32
        linker
            .func_wrap_async(
                "edge_hive",
                "db_query",
                |mut caller: Caller<'_, Arc<H>>, (sql_ptr, sql_len): (i32, i32)| {
                    Box::new(async move {
                        let host = caller.data().clone();

                        // Read SQL from memory
                        let memory = caller
                            .get_export("memory")
                            .and_then(|e| e.into_memory())
                            .ok_or_else(|| anyhow::anyhow!("No memory export"))?;

                        let mem_data = memory.data(&caller);
                        let sql_bytes =
                            &mem_data[sql_ptr as usize..(sql_ptr as usize + sql_len as usize)];
                        let sql = std::str::from_utf8(sql_bytes)?;

                        // Execute query via host
                        let result = host.query(sql);

                        // Serialize result
                        let result_json = serde_json::to_string(&result)?;
                        let result_len = result_json.len() as i32;

                        // Allocate memory for result
                        let allocate = caller
                            .get_export("allocate")
                            .and_then(|e| e.into_func())
                            .ok_or_else(|| anyhow::anyhow!("No allocate export"))?;

                        let mut results = [Val::I32(0)];
                        allocate
                            .call_async(&mut caller, &[Val::I32(result_len + 4)], &mut results)
                            .await?;

                        let result_ptr = results[0].unwrap_i32();

                        // Write result length and data
                        let memory = caller
                            .get_export("memory")
                            .and_then(|e| e.into_memory())
                            .ok_or_else(|| anyhow::anyhow!("No memory export"))?;

                        let mem_data = memory.data_mut(&mut caller);
                        mem_data[result_ptr as usize..result_ptr as usize + 4]
                            .copy_from_slice(&result_len.to_le_bytes());
                        mem_data[result_ptr as usize + 4
                            ..result_ptr as usize + 4 + result_len as usize]
                            .copy_from_slice(result_json.as_bytes());

                        Ok(result_ptr + 4)
                    })
                },
            )
            .map_err(|e| WasmError::Instantiate(e.to_string()))?;

        // log(level: i32, msg_ptr: i32, msg_len: i32)
        linker
            .func_wrap_async(
                "edge_hive",
                "log",
                |mut caller: Caller<'_, Arc<H>>, (level, msg_ptr, msg_len): (i32, i32, i32)| {
                    Box::new(async move {
                        let host = caller.data().clone();

                        // Read message from memory
                        let memory = caller
                            .get_export("memory")
                            .and_then(|e| e.into_memory())
                            .ok_or_else(|| anyhow::anyhow!("No memory export"))?;

                        let mem_data = memory.data(&caller);
                        let msg_bytes =
                            &mem_data[msg_ptr as usize..(msg_ptr as usize + msg_len as usize)];
                        let msg = std::str::from_utf8(msg_bytes)?;

                        // Convert level and log
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
            )
            .map_err(|e| WasmError::Instantiate(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::host::NoOpHostContext;

    #[test]
    fn test_edge_function_from_invalid_bytes() {
        let host = Arc::new(NoOpHostContext);
        let result = EdgeFunction::from_bytes(&[0x00, 0x01, 0x02], host);
        assert!(result.is_err());
    }
}
