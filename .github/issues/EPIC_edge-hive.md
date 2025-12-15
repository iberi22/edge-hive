---
title: "[EPIC] Edge Hive - Distributed Edge Computing Platform"
labels:
  - epic
  - ai-plan
  - architecture
assignees: []
---

# 🐝 Edge Hive - Epic

## Vision

Build a distributed edge computing platform in Rust that transforms old Android phones, spare laptops, and cloud instances into a unified personal server swarm.

## Goals

1. **Cross-Platform Server**: Single Rust binary for Android (Termux), Linux, Windows
2. **Auto-Discovery**: Nodes automatically find each other on local network and globally
3. **Internet Exposure**: Cloudflare Tunnel + Tor onion for public access
4. **Mobile App**: Native Android APK with Astro + Svelte UI
5. **Monetization**: Supabase-style tiered pricing with AWS-backed managed tier
6. **One-Click Cloud**: Users can provision AWS nodes directly from the app UI

## Child Issues

### Phase 1: Core
- [x] `[CORE]` Rust workspace setup with Tokio + Axum
- [ ] `[CORE]` CLI interface with clap

### Phase 2: Identity & Discovery
- [x] `[NET]` Ed25519 cryptographic identity system
- [x] `[NET]` libp2p mDNS local discovery
- [ ] `[NET]` Kademlia DHT global discovery

### Phase 3: Database
- [ ] `[DATA]` SurrealDB 2.0 embedded integration
- [ ] `[DATA]` Time-travel versioning system

### Phase 4: Tunneling
- [x] `[INFRA]` LibCFD Cloudflare Tunnel integration
- [ ] `[INFRA]` Arti Tor onion service (v1.1)
- [ ] `[INFRA]` Termux installation script

### Phase 5: Mobile App
- [ ] `[APP]` Tauri 2.0 project setup
- [ ] `[APP]` Astro + Svelte UI components
- [ ] `[APP]` Android APK build pipeline
- [ ] `[APP]` Cloud dashboard UI

### Phase 6: Cloud Auto-Provisioning (NEW)
- [x] `[CLOUD]` Stripe billing integration (crate created)
- [x] `[CLOUD]` AWS EC2 provisioning (crate created)
- [ ] `[CLOUD]` One-click node provisioning from app
- [ ] `[CLOUD]` Auto-configure Cloudflare Tunnel on new nodes
- [ ] `[CLOUD]` Usage metrics and billing dashboard

## Success Criteria

- [ ] Binary runs on Android Termux, Linux, and Windows
- [ ] Two nodes on same network discover each other within 30 seconds
- [ ] HTTP endpoint accessible via Cloudflare Tunnel
- [ ] APK installs and connects to local node
- [ ] **User can provision a cloud node from app in < 5 minutes**
- [ ] Managed tier deployed with working billing

## Architecture

See [.✨/ARCHITECTURE.md](/.✨/ARCHITECTURE.md)

## Crates (8 total)

| Crate | Status | Description |
|-------|--------|-------------|
| `edge-hive-core` | ⏳ | Main binary, CLI, HTTP server |
| `edge-hive-identity` | ✅ | Ed25519 keypair management |
| `edge-hive-discovery` | ✅ | P2P node discovery |
| `edge-hive-tunnel` | ✅ | Cloudflare Tunnel |
| `edge-hive-db` | ⏳ | SurrealDB wrapper |
| `edge-hive-wasm` | ✅ | Wasmtime plugins |
| `edge-hive-billing` | ✅ | Stripe integration |
| `edge-hive-cloud` | ✅ | AWS EC2 provisioning |

## Timeline

- **MVP (Core + Discovery + Tunnel)**: 4-6 months
- **Mobile App**: +2 months
- **Cloud Auto-Provisioning**: +1 month
- **Full Product**: ~12 months
