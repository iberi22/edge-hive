//! Abstraction for a loaded Edge Function.
use crate::{
    error::WasmError,
    host::HostContext,
    prelude::{Request, Response},
    runtime::State,
};
use wasmtime::{Instance, Store, TypedFunc};

/// Represents a loaded and instantiated Edge Function.
pub struct EdgeFunction<H: HostContext> {
    store: Store<State<H>>,
    instance: Instance,
}

impl<H: HostContext> EdgeFunction<H> {
    /// Creates a new `EdgeFunction` from a `Store` and `Instance`.
    pub fn new(store: Store<State<H>>, instance: Instance) -> Result<Self, WasmError> {
        Ok(Self { store, instance })
    }

    /// Calls the `handle_request` export of the guest module.
    pub async fn call(&mut self, req: Request) -> Result<Response, WasmError> {
        // Serialize the request to JSON bytes.
        let req_bytes = serde_json::to_vec(&req).map_err(|e| WasmError::Call {
            func: "handle_request".to_string(),
            source: e.into(),
        })?;

        // Allocate memory in the guest for the request and write it.
        let req_ptr = self.allocate(req_bytes.len() as i32).await?;
        self.write_memory(req_ptr, &req_bytes)?;

        // Get the exported `handle_request` function.
        let handle_request: TypedFunc<(i32, i32), i32> = self
            .instance
            .get_typed_func(&mut self.store, "handle_request")
            .map_err(|e| WasmError::Call {
                func: "handle_request".to_string(),
                source: e.into(),
            })?;

        // Call the function. The return value is a pointer to the response length and data.
        let resp_packed_ptr = handle_request
            .call_async(&mut self.store, (req_ptr, req_bytes.len() as i32))
            .await
            .map_err(|e| WasmError::Call {
                func: "handle_request".to_string(),
                source: e.into(),
            })?;

        let (resp_ptr, resp_len) = self.unpack_ptr(resp_packed_ptr).await?;

        // Deserialize the response from JSON bytes.
        let resp_bytes = self.read_memory(resp_ptr, resp_len)?;
        let resp: Response = serde_json::from_slice(&resp_bytes).map_err(|e| WasmError::Call {
            func: "handle_request".to_string(),
            source: e.into(),
        })?;

        // Deallocate the request and response memory in the guest.
        self.deallocate(req_ptr, req_bytes.len() as i32).await?;
        self.deallocate(resp_packed_ptr, 8).await?; // Deallocate the packed pointer
        self.deallocate(resp_ptr, resp_len).await?;

        Ok(resp)
    }

    /// Allocates a block of memory in the guest.
    async fn allocate(&mut self, size: i32) -> Result<i32, WasmError> {
        let allocate: TypedFunc<i32, i32> =
            self.instance.get_typed_func(&mut self.store, "allocate").map_err(|e| {
                WasmError::FunctionNotFound(format!("'allocate': {}", e.to_string()))
            })?;

        allocate
            .call_async(&mut self.store, size)
            .await
            .map_err(|e| WasmError::Call {
                func: "allocate".to_string(),
                source: e,
            })
    }

    /// Deallocates a block of memory in the guest.
    async fn deallocate(&mut self, ptr: i32, size: i32) -> Result<(), WasmError> {
        let deallocate: TypedFunc<(i32, i32), ()> =
            self.instance.get_typed_func(&mut self.store, "deallocate").map_err(|e| {
                WasmError::FunctionNotFound(format!("'deallocate': {}", e.to_string()))
            })?;

        deallocate
            .call_async(&mut self.store, (ptr, size))
            .await
            .map_err(|e| WasmError::Call {
                func: "deallocate".to_string(),
                source: e,
            })
    }

    /// Writes data to a memory location in the guest.
    fn write_memory(&mut self, ptr: i32, data: &[u8]) -> Result<(), WasmError> {
        let memory = self
            .instance
            .get_memory(&mut self.store, "memory")
            .ok_or(WasmError::Memory("Memory not found".to_string()))?;

        memory
            .write(&mut self.store, ptr as usize, data)
            .map_err(|e| WasmError::Memory(e.to_string()))
    }

    /// Reads data from a memory location in the guest.
    fn read_memory(&mut self, ptr: i32, len: i32) -> Result<Vec<u8>, WasmError> {
        let memory = self
            .instance
            .get_memory(&mut self.store, "memory")
            .ok_or(WasmError::Memory("Memory not found".to_string()))?;

        let mut buffer = vec![0; len as usize];
        memory
            .read(&self.store, ptr as usize, &mut buffer)
            .map_err(|e| WasmError::Memory(e.to_string()))?;

        Ok(buffer)
    }

    /// Reads a (ptr, len) tuple from guest memory, which is packed as two u32 values.
    async fn unpack_ptr(&mut self, ptr: i32) -> Result<(i32, i32), WasmError> {
        let memory = self
            .instance
            .get_memory(&mut self.store, "memory")
            .ok_or(WasmError::Memory("Memory not found".to_string()))?;

        let mut packed_buffer = [0; 8];
        memory
            .read(&self.store, ptr as usize, &mut packed_buffer)
            .map_err(|e| WasmError::Memory(e.to_string()))?;

        let data_ptr = u32::from_le_bytes(
            packed_buffer[0..4]
                .try_into()
                .map_err(|e| WasmError::Memory(format!("Failed to unpack pointer: {}", e)))?,
        ) as i32;
        let len = u32::from_le_bytes(
            packed_buffer[4..8]
                .try_into()
                .map_err(|e| WasmError::Memory(format!("Failed to unpack length: {}", e)))?,
        ) as i32;

        Ok((data_ptr, len))
    }
}
