
# Edge Hive Validation Plan

## Goal
Verify if the "Edge Hive" system is ready to act as a fallback VPS for moving Edge Functions and Database.

## Current Status (Hypothesis)
- **Functions (`edge-hive-wasm`)**: Likely working (Wasmtime is usually portable).
- **Database (`edge-hive-db`)**: Likely broken on Windows due to RocksDB dependency.

## Steps

### 1. Function Execution (WASM)
- [ ] Inspect `crates/edge-hive-wasm` crate.
- [ ] Run unit tests: `cargo test -p edge-hive-wasm`.
- [ ] Verify `PluginManager` can load and call a dummy WASM.

### 2. Database (SurrealDB)
- [ ] Inspect `crates/edge-hive-db`.
- [ ] Attempt build/test: `cargo test -p edge-hive-db`.
- [ ] **Fix**: If RocksDB fails, switch to `Surreal::new::<Mem>()` (In-Memory) for immediate validation.
- [ ] Verify basic CRUD operations.
