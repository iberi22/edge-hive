---
title: "Implement Edge Functions WASM Runtime"
labels:
  - enhancement
  - wasm
  - performance
  - ai-plan
assignees: []
---

## Description

Extend the existing WASM plugin system to support user-defined edge functions. This replaces Supabase Edge Functions (Deno) with native Rust WASM execution that is 10-100x faster with zero cold start.

## Performance Comparison

| Metric | Supabase (Deno) | Edge Hive (WASM) | Improvement |
|--------|-----------------|------------------|-------------|
| Execution Time | 50-200ms | 1-5ms | 40-200x |
| Cold Start | 1-3 seconds | 0ms | ∞ |
| Memory | ~50MB (Deno runtime) | ~5MB (Wasmtime) | 10x |
| Binary Size | ~80MB | ~100KB | 800x |

## Architecture

```
HTTP Request
     ↓
[API Gateway] /api/v1/edge/:function
     ↓
[WASM Runtime]
     ├─ Load function from registry
     ├─ Instantiate WASM module
     ├─ Call exported function
     └─ Return response
     ↓
[Response to client]
```

## Edge Function Example

**Rust WASM Function:**

```rust
// functions/hello/src/lib.rs
use edge_hive_wasm::prelude::*;

#[edge_function]
pub fn handler(req: Request) -> Result<Response, Error> {
    let body: serde_json::Value = serde_json::from_str(&req.body)?;
    let name = body["name"].as_str().unwrap_or("World");

    Ok(Response {
        status: 200,
        body: json!({
            "message": format!("Hello, {}!", name),
            "timestamp": chrono::Utc::now().to_rfc3339()
        }).to_string(),
        headers: vec![
            ("Content-Type".into(), "application/json".into())
        ]
    })
}
```

**Compile:**

```bash
cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/hello.wasm functions/
```

**Deploy:**

```bash
curl -X POST http://localhost:8443/api/v1/edge/deploy \
  -F "function=hello" \
  -F "wasm=@functions/hello.wasm"
```

**Invoke:**

```bash
curl -X POST http://localhost:8443/api/v1/edge/hello \
  -H "Content-Type: application/json" \
  -d '{"name": "Edge Hive"}'

# Response:
# {"message": "Hello, Edge Hive!", "timestamp": "2025-12-15T19:30:00Z"}
```

## Technical Requirements

### Dependencies

```toml
[dependencies]
wasmtime = { workspace = true }
tokio = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
anyhow = { workspace = true }
```

### Features

1. **Function Registry**: Store and manage WASM modules
2. **Hot Reload**: Deploy new versions without downtime
3. **Sandboxing**: WASI isolation for security
4. **Timeout**: Configurable execution timeout
5. **Resource Limits**: CPU/memory constraints
6. **Metrics**: Invocations, errors, latency

## Implementation Tasks

### Core Runtime

- [ ] Extend `edge-hive-wasm` for edge functions
- [ ] Add function registry (filesystem + database)
- [ ] Implement WASM module loading and caching
- [ ] Add request/response marshaling
- [ ] Implement timeout enforcement
- [ ] Add resource limits (CPU, memory)

### API Integration

- [ ] Add route: `POST /api/v1/edge/:function`
- [ ] Add route: `POST /api/v1/edge/deploy` (upload WASM)
- [ ] Add route: `GET /api/v1/edge` (list functions)
- [ ] Add route: `DELETE /api/v1/edge/:function`

### Developer Tools

- [ ] Create function template generator
- [ ] Add CLI command: `edge-hive function new <name>`
- [ ] Add CLI command: `edge-hive function deploy <name>`
- [ ] Add CLI command: `edge-hive function invoke <name>`

### Testing & Docs

- [ ] Write unit tests for runtime
- [ ] Write integration tests for deployment
- [ ] Create example functions (hello, auth, webhook)
- [ ] Document function API (Request/Response types)
- [ ] Document deployment process
- [ ] Add performance benchmarks

## Function Template

```toml
# functions/my-function/Cargo.toml
[package]
name = "my-function"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib"]

[dependencies]
edge-hive-wasm = { path = "../../crates/edge-hive-wasm" }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
chrono = "0.4"

[profile.release]
opt-level = 'z'     # Optimize for size
lto = true          # Link-time optimization
codegen-units = 1   # Single codegen unit
strip = true        # Strip symbols
```

## Security Features

- [ ] WASI sandbox (no filesystem/network by default)
- [ ] Configurable permissions per function
- [ ] Memory limits (default: 128MB)
- [ ] CPU limits (default: 100ms timeout)
- [ ] Rate limiting per function

## Integration Points

- `edge-hive-wasm`: Base WASM runtime (already exists)
- `edge-hive-api`: HTTP endpoints for function invocation
- `edge-hive-db`: Function metadata storage
- `edge-hive-auth`: Function authentication/authorization

## Success Criteria

- [ ] Functions execute in < 5ms (p99)
- [ ] Zero cold start penalty
- [ ] Hot reload without downtime
- [ ] Sandbox prevents malicious code
- [ ] All tests passing
- [ ] Documentation complete
- [ ] 3+ example functions

## Migration from Supabase

**Supabase Edge Function:**

```typescript
// Deno runtime
Deno.serve(async (req) => {
  const { name } = await req.json()
  return new Response(JSON.stringify({ message: `Hello ${name}` }))
})
```

**Edge Hive equivalent:**

```rust
// Native Rust (much faster)
#[edge_function]
pub fn handler(req: Request) -> Result<Response, Error> {
    let body: Value = serde_json::from_str(&req.body)?;
    let name = body["name"].as_str().unwrap_or("World");
    Ok(Response::json(json!({ "message": format!("Hello {}", name) })))
}
```

## References

- Wasmtime docs: <https://docs.rs/wasmtime>
- WASI spec: <https://wasi.dev>
- VPS Migration Plan: `docs/VPS_MIGRATION_PLAN.md`
