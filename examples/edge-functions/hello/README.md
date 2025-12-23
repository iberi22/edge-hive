# Hello World Edge Function

A simple example edge function that demonstrates the Edge Hive WASM runtime.

## Building

```bash
cd examples/edge-functions/hello
cargo build --target wasm32-unknown-unknown --release
```

The compiled WASM will be at:
```
target/wasm32-unknown-unknown/release/hello_edge_function.wasm
```

## Usage

```rust
use edge_hive_wasm::{WasmRuntime, NoOpHostContext};
use serde_json::json;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // Create runtime with host context
    let host = Arc::new(NoOpHostContext);
    let runtime = WasmRuntime::new(host).unwrap();
    
    // Load the function
    let function = runtime.load_function(
        Path::new("examples/edge-functions/hello/target/wasm32-unknown-unknown/release/hello_edge_function.wasm")
    ).unwrap();
    
    // Execute with request
    let request = json!({ "name": "Edge Hive" });
    let response = function.execute(request).await.unwrap();
    
    println!("Response: {}", response);
}
```

## Request Format

```json
{
  "name": "YourName"
}
```

## Response Format

```json
{
  "message": "Hello, YourName!",
  "timestamp": 0
}
```

## Memory Management

The function implements simple memory management:
- `allocate(size)` - Allocates memory and returns a pointer
- `deallocate(ptr, size)` - Deallocates memory
- `handle_request(req_ptr, req_len)` - Main entry point

The response length is stored at `response_ptr - 4` bytes.
