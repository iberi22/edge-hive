---
title: "[CRITICAL] Implement Tor Integration with tor-hsservice"
labels:
  - critical
  - networking
  - tor
  - privacy
---

## Context
PR #25 blocked: `arti-client 0.20` missing `hsservice` feature.

## Task
Implement Tor Onion Services using `tor-hsservice 0.20` directly (bypass arti-client).

## Implementation Plan

### 1. Dependencies
```toml
[dependencies]
tor-hsservice = "0.20"
tor-rtcompat = { version = "0.20", features = ["tokio"] }
tor-hscrypto = "0.20"
ed25519-consensus = "2"
```

### 2. Code Structure
```
crates/edge-hive-tunnel/src/tor/
├── mod.rs          # Public API
├── onion.rs        # Onion service setup
└── bootstrap.rs    # Tor client bootstrap
```

### 3. Acceptance Criteria
- [ ] Tor client bootstraps successfully
- [ ] Onion service launches and gets .onion address
- [ ] Can accept incoming connections on .onion
- [ ] Tests pass: `cargo test --package edge-hive-tunnel`

## Priority
CRITICAL - Blocks full P2P functionality

## Estimated Time
4-6 hours

## References
- https://gitlab.torproject.org/tpo/core/arti/-/issues/104
- https://docs.rs/tor-hsservice/0.20.0
