use crate::{
    error::WasmError,
    host::{HostContext, LogLevel},
    EdgeFunction,
};
use anyhow::{anyhow, Result};
use std::sync::Arc;
use wasmtime::*;

/// The state required by the host, shared with WASM guests.
pub struct State<H: HostContext> {
    pub host: Arc<H>,
    pub memory: Option<Memory>,
}

/// A sandboxed runtime for executing Edge Functions compiled to WebAssembly.
pub struct WasmRuntime<H: HostContext> {
    engine: Engine,
    linker: Linker<State<H>>,
    host: Arc<H>,
}

impl<H: HostContext + 'static> WasmRuntime<H> {
    /// Creates a new `WasmRuntime`.
    pub fn new(host: Arc<H>) -> Result<Self> {
        let mut config = Config::new();
        config.async_support(true);

        let engine = Engine::new(&config)?;
        let mut linker = Linker::new(&engine);

        // Define host functions
        Self::define_host_functions(&mut linker)?;

        Ok(Self {
            engine,
            linker,
            host,
        })
    }

    /// Loads a WASM module and prepares it for execution.
    pub async fn load(&self, wasm_bytes: &[u8]) -> Result<EdgeFunction<H>, WasmError> {
        let module = Module::from_binary(&self.engine, wasm_bytes)
            .map_err(|e| WasmError::Load(e.to_string()))?;

        let state = State {
            host: self.host.clone(),
            memory: None,
        };
        let mut store = Store::new(&self.engine, state);

        let instance = self
            .linker
            .instantiate_async(&mut store, &module)
            .await
            .map_err(|e| WasmError::Instantiate(e.to_string()))?;

        // Extract memory and store it in the state for host functions to use.
        let memory = instance
            .get_memory(&mut store, "memory")
            .ok_or_else(|| WasmError::Instantiate("Memory not found".to_string()))?;
        store.data_mut().memory = Some(memory);

        EdgeFunction::new(store, instance)
    }

    /// Defines the host functions that will be available to the WASM guest.
    fn define_host_functions(linker: &mut Linker<State<H>>) -> Result<()> {
        // `db_query`
        linker.func_wrap_async(
            "edge_hive",
            "db_query",
            |mut caller: Caller<'_, State<H>>, (ptr, len): (i32, i32)| {
                Box::new(async move {
                    let mem = match caller.data().memory {
                        Some(mem) => mem,
                        None => return 0,
                    };
                    let sql_bytes = &mem.data(&caller)[ptr as usize..(ptr + len) as usize];
                    let sql = match std::str::from_utf8(sql_bytes) {
                        Ok(s) => s,
                        Err(_) => return 0,
                    };

                    let result = caller.data().host.query(sql).await;
                    let result_json = match serde_json::to_string(&result) {
                        Ok(s) => s,
                        Err(_) => return 0,
                    };
                    let result_bytes = result_json.as_bytes();

                    // Allocate memory for the result and write it.
                    match Self::allocate_and_write(&mut caller, result_bytes).await {
                        Ok(ptr) => ptr as i32,
                        Err(_) => 0,
                    }
                })
            },
        )?;

        // `log`
        linker.func_wrap(
            "edge_hive",
            "log",
            |caller: Caller<'_, State<H>>, (level, ptr, len): (u32, i32, i32)| {
                if let Some(mem) = caller.data().memory {
                    if let Ok(msg) =
                        std::str::from_utf8(&mem.data(&caller)[ptr as usize..(ptr + len) as usize])
                    {
                        caller.data().host.log(LogLevel::from(level), msg);
                    }
                }
            },
        )?;

        Ok(())
    }

    /// Helper to allocate memory in the guest and write data to it.
    async fn allocate_and_write(
        caller: &mut Caller<'_, State<H>>,
        data: &[u8],
    ) -> Result<i32> {
        let size = data.len() as i32;
        let alloc_fn = caller
            .get_export("allocate")
            .and_then(|e| e.into_func())
            .ok_or_else(|| anyhow!("`allocate` function not found in guest"))?;

        let mut result = Val::I32(0);
        alloc_fn
            .call_async(
                caller.as_context_mut(),
                &[Val::I32(size)],
                std::slice::from_mut(&mut result),
            )
            .await?;

        let ptr = result.unwrap_i32();
        let mem = caller
            .data()
            .memory
            .ok_or_else(|| anyhow!("memory not set"))?;
        mem.write(caller.as_context_mut(), ptr as usize, data)?;

        Ok(ptr)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::Request;
    use async_trait::async_trait;
    use serde_json::Value;
    use std::sync::Mutex;

    // A mock HostContext for testing.
    struct MockHostContext {
        queries: Mutex<Vec<String>>,
        logs: Mutex<Vec<(LogLevel, String)>>,
    }

    impl MockHostContext {
        fn new() -> Self {
            Self {
                queries: Mutex::new(Vec::new()),
                logs: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl HostContext for MockHostContext {
        async fn query(&self, sql: &str) -> Result<Value, String> {
            self.queries.lock().unwrap().push(sql.to_string());
            Ok(Value::Null)
        }

        fn log(&self, level: LogLevel, msg: &str) {
            self.logs.lock().unwrap().push((level, msg.to_string()));
        }
    }

    #[tokio::test]
    async fn test_wasm_runtime_hello_world() {
        // Compile the hello-world example.
        let wasm_bytes = std::fs::read(
            "../../target/wasm32-unknown-unknown/release/hello_world_edge_function.wasm",
        )
        .expect("Failed to read wasm file");

        let host = Arc::new(MockHostContext::new());
        let runtime = WasmRuntime::new(host.clone()).expect("Failed to create runtime");
        let mut function = runtime
            .load(&wasm_bytes)
            .await
            .expect("Failed to load function");

        let req = Request {
            method: "GET".to_string(),
            path: "/hello".to_string(),
            headers: Default::default(),
            body: Value::Null,
        };

        let resp = function.call(req).await.expect("Failed to call function");
        assert_eq!(resp.status, 200);
        assert_eq!(
            resp.body,
            serde_json::json!({ "message": "Hello from WASM!" })
        );
    }
}
