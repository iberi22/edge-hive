---
title: "[CORE] Rust Workspace Setup with Tokio + Axum"
labels:
  - core
  - rust
  - priority-high
assignees:
  - jules
---

## User Story

**As a** developer
**I want** a properly structured Rust workspace
**So that** I can build modular crates for edge-hive components

## Technical Specs

### Workspace Structure

```
edge-hive/
├── Cargo.toml           # Workspace root
├── crates/
│   ├── edge-hive-core/      # Main binary
│   ├── edge-hive-identity/  # Crypto identity
│   ├── edge-hive-discovery/ # P2P networking
│   ├── edge-hive-tunnel/    # CF + Tor
│   ├── edge-hive-db/        # SurrealDB wrapper
│   └── edge-hive-wasm/      # Plugin runtime
```

### Dependencies (core crate)

```toml
[dependencies]
tokio = { version = "1.40", features = ["full"] }
axum = "0.7"
clap = { version = "4", features = ["derive"] }
config = "0.14"
tracing = "0.1"
tracing-subscriber = "0.3"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
anyhow = "1"
thiserror = "1"
```

### CLI Commands

- `edge-hive init` - Initialize node identity
- `edge-hive serve` - Start HTTP server
- `edge-hive status` - Show node status
- `edge-hive peers` - List discovered peers

## Acceptance Criteria

- [ ] Workspace compiles with `cargo build`
- [ ] All 6 crates initialized with basic structure
- [ ] CLI parses commands correctly
- [ ] Axum server starts on configurable port
- [ ] Tracing/logging configured
- [ ] Basic error handling with anyhow/thiserror

## Branch

`feat/core-workspace`
