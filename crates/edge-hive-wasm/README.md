# edge-hive-wasm

WebAssembly runtime for Edge Hive edge functions with modular host context injection.

## Features

- **Modular Design**: No direct dependencies on edge-hive-db or other internal crates
- **Host Context Trait**: Inject any implementation for database and logging
- **wasmtime v26 Compatible**: Uses latest wasmtime API with async support
- **Type Safe**: Generic HostContext trait provides compile-time safety
- **Efficient**: Supports SIMD, bulk memory, and optimized cranelift compilation

## Architecture

```
edge-hive-wasm/
├── src/
│   ├── lib.rs           # Public exports
│   ├── runtime.rs       # WasmRuntime<H: HostContext>
│   ├── host.rs          # HostContext trait
│   ├── function.rs      # EdgeFunction abstraction
│   └── prelude.rs       # Guest SDK types (Request, Response, etc.)
├── tests/
│   └── integration_test.rs
└── examples/
    └── edge-functions/hello/  # Hello world example
```

## Usage

### 1. Implement HostContext

```rust
use edge_hive_wasm::{HostContext, LogLevel};
use serde_json::Value;

struct MyHostContext {
    // Your fields here
}

impl HostContext for MyHostContext {
    fn query(&self, sql: &str) -> Result<Value, String> {
        // Execute database query
        Ok(serde_json::json!({"result": "ok"}))
    }

    fn log(&self, level: LogLevel, msg: &str) {
        println!("[{:?}] {}", level, msg);
    }
}
```

### 2. Create Runtime and Load Function

```rust
use edge_hive_wasm::WasmRuntime;
use std::sync::Arc;
use std::path::Path;

#[tokio::main]
async fn main() {
    // Create runtime with your host context
    let host = Arc::new(MyHostContext::new());
    let runtime = WasmRuntime::new(host).unwrap();
    
    // Load edge function from WASM file
    let function = runtime.load_function(
        Path::new("path/to/function.wasm")
    ).unwrap();
    
    // Execute with JSON request
    let request = serde_json::json!({
        "name": "Edge Hive"
    });
    
    let response = function.execute(request).await.unwrap();
    println!("Response: {}", response);
}
```

## Host Functions

Edge functions can call these host functions:

### `edge_hive::db_query`
```wasm
(func $db_query (param $sql_ptr i32) (param $sql_len i32) (result i32))
```
Executes a database query and returns a pointer to the JSON result.

### `edge_hive::log`
```wasm
(func $log (param $level i32) (param $msg_ptr i32) (param $msg_len i32))
```
Logs a message. Levels: 0=Trace, 1=Debug, 2=Info, 3=Warn, 4=Error

## WASM Function Interface

Your edge function must export:

### `allocate`
```rust
#[no_mangle]
pub extern "C" fn allocate(size: i32) -> i32
```
Allocates memory and returns a pointer. Called by the runtime.

### `deallocate`
```rust
#[no_mangle]
pub extern "C" fn deallocate(ptr: i32, size: i32)
```
Deallocates memory. Called by the runtime.

### `handle_request`
```rust
#[no_mangle]
pub extern "C" fn handle_request(req_ptr: i32, req_len: i32) -> i64
```
Main entry point. Receives request as JSON string pointer/length.
Returns i64 with response pointer in lower 32 bits and length in upper 32 bits.

## Example Edge Function

See `examples/edge-functions/hello/` for a complete working example.

```rust
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Deserialize)]
struct Request {
    name: Option<String>,
}

#[derive(Serialize)]
struct Response {
    message: String,
}

#[no_mangle]
pub extern "C" fn handle_request(req_ptr: i32, req_len: i32) -> i64 {
    // Read request
    let request_json = unsafe {
        let slice = std::slice::from_raw_parts(req_ptr as *const u8, req_len as usize);
        std::str::from_utf8(slice).unwrap_or("{}")
    };
    
    // Parse and process
    let req: Value = serde_json::from_str(request_json).unwrap();
    let name = req["name"].as_str().unwrap_or("World");
    
    let response = Response {
        message: format!("Hello, {}!", name),
    };
    
    // Serialize and return
    let response_json = serde_json::to_string(&response).unwrap();
    let response_bytes = response_json.into_bytes();
    let response_len = response_bytes.len() as i32;
    
    let resp_ptr = allocate(response_len);
    
    unsafe {
        let dest = std::slice::from_raw_parts_mut(resp_ptr as *mut u8, response_len as usize);
        dest.copy_from_slice(&response_bytes);
    }
    
    ((response_len as i64) << 32) | (resp_ptr as i64)
}
```

## Building Edge Functions

```bash
# Add wasm32 target
rustup target add wasm32-unknown-unknown

# Build with optimization
cargo build --target wasm32-unknown-unknown --release

# Output will be in target/wasm32-unknown-unknown/release/*.wasm
```

### Recommended Cargo.toml

```toml
[package]
name = "my-edge-function"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
wee_alloc = "0.4"

[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
strip = true
```

## Testing

```bash
# Run all tests
cargo test -p edge-hive-wasm

# Run integration tests (requires hello-world WASM to be built)
cd examples/edge-functions/hello
cargo build --target wasm32-unknown-unknown --release
cd ../../..
cargo test -p edge-hive-wasm --test integration_test
```

## Design Principles

1. **Dependency Injection**: No direct coupling to database or logging implementations
2. **Zero Dependencies**: edge-hive-wasm doesn't depend on edge-hive-db or other internal crates
3. **Type Safety**: Compile-time guarantees via generic HostContext trait
4. **Performance**: Async support, SIMD, optimized compilation
5. **Simplicity**: Clear separation between runtime, host, and function concerns

## License

See LICENSE file in the repository root.
