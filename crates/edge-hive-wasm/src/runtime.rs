use anyhow::{Context, Result};
use serde_json::Value;
use std::path::Path;
use std::sync::Arc;
use wasmtime::*;
use edge_hive_db::DatabaseService;
use tracing::{info, warn, error};

/// State accessible from host functions
pub struct HostState {
    pub db: Arc<DatabaseService>,
}

pub struct HostFunctions;

impl HostFunctions {
    pub fn add_to_linker(linker: &mut Linker<HostState>) -> Result<()> {
        // Function to query the database
        linker.func_wrap_async("edge_hive", "db_query", |mut caller: Caller<'_, HostState>, sql_ptr: i32, sql_len: i32| {
            Box::new(async move {
                let mem = match caller.get_export("memory") {
                    Some(Extern::Memory(mem)) => mem,
                    _ => {
                        error!("WASM module has no memory export");
                        return -1;
                    }
                };

                let sql_bytes = &mem.data(&caller)[sql_ptr as usize .. (sql_ptr + sql_len) as usize];
                let sql = match std::str::from_utf8(sql_bytes) {
                    Ok(s) => s.to_string(),
                    Err(_) => {
                        error!("Invalid UTF-8 in SQL string from WASM");
                        return -1;
                    }
                };

                info!("Running DB query from WASM: {}", sql);
                let result = caller.data().db.query(&sql).await;
                match result {
                    Ok(_response) => {
                        // TODO: Serialize the response and write it to WASM memory.
                        // For now, we just return a success code.
                        0
                    }
                    Err(e) => {
                        error!("DB query from WASM failed: {}", e);
                        -1
                    }
                }
            })
        })?;

        // Function for logging
        linker.func_wrap("edge_hive", "log", |mut caller: Caller<'_, HostState>, level: i32, msg_ptr: i32, msg_len: i32| {
             let mem = match caller.get_export("memory") {
                Some(Extern::Memory(mem)) => mem,
                _ => {
                    warn!("WASM module has no memory export");
                    return;
                }
            };

            let msg_bytes = &mem.data(&caller)[msg_ptr as usize .. (msg_ptr + msg_len) as usize];
            let msg = match std::str::from_utf8(msg_bytes) {
                Ok(s) => s,
                Err(_) => {
                    error!("Invalid UTF-8 in log message from WASM");
                    return;
                }
            };

            match level {
                1 => error!("WASM: {}", msg),
                2 => warn!("WASM: {}", msg),
                3 => info!("WASM: {}", msg),
                _ => tracing::debug!("WASM: {}", msg),
            }
        })?;

        Ok(())
    }
}

/// Edge function runtime using WebAssembly
pub struct WasmRuntime {
    engine: Engine,
    linker: Linker<HostState>,
}

impl WasmRuntime {
    pub fn new() -> Result<Self> {
        let mut config = Config::new();
        config.wasm_simd(true);
        config.wasm_bulk_memory(true);
        config.cranelift_opt_level(OptLevel::Speed);

        let engine = Engine::new(&config)
            .context("Failed to create Wasmtime engine")?;

        let mut linker = Linker::new(&engine);
        HostFunctions::add_to_linker(&mut linker)?;

        Ok(Self { engine, linker })
    }

    /// Execute pre-compiled WASM module
    pub async fn execute_wasm(&self, wasm_path: &Path, input: Value, host_state: HostState) -> Result<Value> {
        let module = Module::from_file(&self.engine, wasm_path)
            .context("Failed to load WASM module")?;

        let mut store = Store::new(&self.engine, host_state);
        let instance = self.linker.instantiate(&mut store, &module)
            .context("Failed to instantiate WASM module")?;

        // Call exported "handle" function
        let handle = instance
            .get_typed_func::<(), i32>(&mut store, "handle")
            .context("WASM module missing 'handle' export")?;

        let result = handle.call(&mut store, ())
            .context("WASM execution failed")?;

        Ok(serde_json::json!({
            "result": result,
            "input": input
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use edge_hive_db::DatabaseService;
    use std::sync::Arc;
    use wat::parse_str;

    async fn create_mock_db() -> Arc<DatabaseService> {
        // In-memory database for testing
        Arc::new(DatabaseService::new(std::path::Path::new("memory://")).await.unwrap())
    }

    #[tokio::test]
    async fn test_wasm_runtime_init() {
        let runtime = WasmRuntime::new();
        assert!(runtime.is_ok());
    }

    #[tokio::test]
    async fn test_host_function_linking() {
        let db = create_mock_db().await;
        let runtime = WasmRuntime::new().unwrap();

        // A simple WAT module that imports and calls our host functions
        let wat = r#"
        (module
            (import "edge_hive" "log" (func $log (param i32 i32 i32)))
            (import "edge_hive" "db_query" (func $db_query (param i32 i32) (result i32)))
            (memory (export "memory") 1)
            (data (i32.const 0) "hello from wasm")
            (data (i32.const 20) "SELECT * FROM users")

            (func (export "run_test")
                ;; Log a message
                i32.const 3  ;; level
                i32.const 0  ;; msg_ptr
                i32.const 15 ;; msg_len
                call $log

                ;; Query the database
                i32.const 20 ;; sql_ptr
                i32.const 18 ;; sql_len
                call $db_query
                drop ;; drop the result
            )
        )
        "#;

        let wasm_bytes = parse_str(wat).unwrap();

        let module = Module::new(&runtime.engine, &wasm_bytes).unwrap();
        let mut store = Store::new(&runtime.engine, HostState { db });
        let instance = runtime.linker.instantiate(&mut store, &module).unwrap();

        let run_test = instance.get_typed_func::<(), ()>(&mut store, "run_test").unwrap();

        // This will call the host functions, which will log to the console.
        // In a real test, we would capture the logs and assert on them.
        let result = run_test.call(&mut store, ());
        assert!(result.is_ok());
    }
}
