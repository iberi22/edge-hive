---
title: "Integrate Tor Module with edge-hive-core"
labels:
  - enhancement
  - tor
  - networking
  - priority-high
assignees: []
---

## 🎯 Objective

Connect the newly implemented Tor module (`edge-hive-tunnel::tor`) with the core Axum server to enable onion service hosting.

## 📋 Context

PR #26 integrated Identity + Age encryption (✅ merged)
PR #24 integrated Workspace + Axum server (✅ merged)
Commit `55699b6` implemented Tor module with:

- Ed25519 keypair generation
- Onion v3 address derivation
- Tor key file persistence
- **BUT**: Bootstrap is placeholder, no actual traffic forwarding yet

## 🛠️ Tasks

- [ ] Modify `src/serve.rs` to accept `--tor` flag
- [ ] Initialize `TorNode` when `--tor` is enabled
- [ ] Bridge Axum HTTP server with Tor's SOCKS5 proxy
- [ ] Implement traffic forwarding from onion address → Axum
- [ ] Add `tor_enabled: bool` to workspace configuration
- [ ] Update `TorNode::bootstrap()` placeholder with real logic
- [ ] Test: `edge-hive serve --tor` should print `.onion` address
- [ ] Test: curl via Tor should reach Axum server

## 📦 Dependencies

- Existing `edge-hive-tunnel` crate
- `tor-hsservice` API for traffic routing
- Axum server in `edge-hive-core`

## ✅ Success Criteria

1. Running `edge-hive serve --tor` starts both Axum and Tor
2. Prints onion address to console (e.g., `http://abc123...onion`)
3. External Tor Browser can reach `/health` endpoint
4. Graceful shutdown closes both servers

## 🔗 Related

- Commit: `55699b6` (Tor module implementation)
- File: `crates/edge-hive-tunnel/src/tor/mod.rs`
- File: `src/serve.rs` (needs modification)
