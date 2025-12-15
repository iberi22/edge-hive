---
title: "Edge Hive Development Context"
type: RESEARCH
id: "research-edge-hive-stack"
created: 2025-12-14
updated: 2025-12-14
agent: antigravity
model: gemini-3-pro
requested_by: user
summary: |
  Living research context for Edge Hive development, including current
  stack versions, known issues, and intelligent patterns for AI agents.
keywords: [rust, surrealdb, libp2p, tauri, edge-computing]
tags: ["#research", "#context", "#stack"]
project: Edge-Hive
---

# 📚 Edge Hive Stack Research Context

> This document is the **Living Context** for AI agents working on Edge Hive.
> Read this BEFORE implementing any feature.

## Current Stack (December 2024)

| Component | Version | Status | Notes |
|-----------|---------|--------|-------|
| Rust | 2024 Edition (1.85+) | ✅ Stable | Use `edition = "2024"` in Cargo.toml |
| Tokio | 1.40+ | ✅ Stable | Full async runtime |
| Axum | 0.7+ | ✅ Stable | HTTP framework |
| SurrealDB | 2.0.x | ✅ Stable | Use embedded RocksDB backend |
| libp2p | 0.54+ | ✅ Stable | P2P networking |
| libp2p-mdns | 0.48+ | ✅ Stable | Local discovery |
| libp2p-kad | 0.47+ | ✅ Stable | Kademlia DHT |
| Tauri | 2.0+ | ✅ Stable | Mobile support (Oct 2024) |
| LibCFD | 0.1.x | ⚠️ New | Rust CF Tunnel (Feb 2024) |
| Arti | 1.4+ | ⚠️ Maturing | Tor client, onion services improving |
| Wasmtime | 25+ | ✅ Stable | WASM plugin runtime |

## Known Anomalies

### SurrealDB 2.0

- **Issue**: New query parser may have edge cases
- **Mitigation**: Keep queries simple, test in CI
- **Reference**: [SurrealDB 2.0 Release Notes](https://surrealdb.com/releases)

### LibCFD (Cloudflare Tunnel Rust)

- **Issue**: New library (Feb 2024), less battle-tested
- **Mitigation**: Have fallback to spawn `cloudflared` binary
- **Alternative**: Use `cloudflared` crate on crates.io

### Arti (Tor Rust)

- **Issue**: Onion service support still maturing
- **Mitigation**: Delay Tor integration to v1.1, focus on CF Tunnel first
- **Reference**: [Arti Releases](https://blog.torproject.org/arti/)

### Android Cross-Compilation

- **Issue**: Requires NDK setup, can be complex on CI
- **Mitigation**: Use `cargo-ndk` or `cross` crate
- **CI**: GitHub Actions with `setup-android` action

## Intelligent Patterns

### libp2p Setup

```rust
use libp2p::{
    identity, mdns, kad, noise, quic, swarm,
    SwarmBuilder, Transport,
};

// Recommended pattern for node identity
let keypair = identity::Keypair::generate_ed25519();
let peer_id = keypair.public().to_peer_id();

// Build swarm with QUIC transport
let swarm = SwarmBuilder::with_new_identity()
    .with_tokio()
    .with_quic()
    .with_behaviour(|key| {
        let mdns = mdns::tokio::Behaviour::new(
            mdns::Config::default(),
            key.public().to_peer_id(),
        )?;
        let kademlia = kad::Behaviour::new(
            key.public().to_peer_id(),
            kad::store::MemoryStore::new(key.public().to_peer_id()),
        );
        Ok(MyBehaviour { mdns, kademlia })
    })?
    .build();
```

### SurrealDB Embedded

```rust
use surrealdb::Surreal;
use surrealdb::engine::local::RocksDb;

// Recommended: RocksDB for persistent storage
let db = Surreal::new::<RocksDb>("./data").await?;
db.use_ns("edge_hive").use_db("main").await?;

// For testing: Use in-memory
use surrealdb::engine::local::Mem;
let db = Surreal::new::<Mem>(()).await?;
```

### Tauri 2.0 Mobile Commands

```rust
// src-tauri/src/lib.rs
#[tauri::command]
async fn get_node_status() -> Result<NodeStatus, String> {
    // Use invoke() from frontend, not fetch()
    Ok(NodeStatus {
        peer_id: "...".into(),
        peers: vec![],
        uptime: 3600,
    })
}
```

### Astro + Svelte Bridge

```svelte
<!-- src/components/Dashboard.svelte -->
<script>
  import { invoke } from '@tauri-apps/api/core';

  let status = $state(null);

  async function loadStatus() {
    status = await invoke('get_node_status');
  }
</script>
```

## Quarantine Status

| Dependency | Added | Quarantine Until | Notes |
|------------|-------|------------------|-------|
| surrealdb 2.1+ | Future | +14 days | Wait for community feedback |
| libp2p 0.55+ | Future | +14 days | Major version bump |
| tauri 2.1+ | Future | +14 days | Mobile-related changes |

## Security Checklist

- [ ] All keys stored encrypted at rest (AES-256-GCM)
- [ ] Ed25519 for identity (ed25519-dalek crate)
- [ ] Noise protocol for P2P encryption (libp2p default)
- [ ] WASM plugins run in Wasmtime sandbox
- [ ] No hardcoded secrets - use env vars or config file
- [ ] `cargo audit` passes before each release

---

*Auto-updated by Context Research Agent*
