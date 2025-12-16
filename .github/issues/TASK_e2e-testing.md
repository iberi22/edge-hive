---
title: "End-to-End Testing: CLI → Server → Tor → P2P"
labels:
  - testing
  - e2e
  - validation
  - priority-high
assignees: []
---

## 🎯 Objective

Validate the complete Edge Hive flow:

1. **CLI** generates identity
2. **Server** starts with Tor enabled
3. **Tor** publishes onion address
4. **P2P** discovers peers via libp2p
5. **Message** sent from Node A → Node B (encrypted)

## 📋 Context

All components are implemented (or will be soon):

- ✅ Identity (Ed25519 + Age) - PR #26
- ✅ CLI (7 subcommands) - Jules
- ✅ Axum Server - PR #24
- 🟡 Tor Module - Partial (needs integration)
- 🔵 libp2p Discovery - Planned
- 🔵 SurrealDB - Planned

Before Android APK build, we need E2E validation.

## 🛠️ Test Scenarios

### Scenario 5: VPS-style API Gateway + DB + Realtime + Edge Functions (Rust)

Goal: validate the "personal VPS" experience end-to-end using the Rust server:

- [ ] Boot core server and confirm both `/health` and `/api/v1/health`
- [ ] CRUD roundtrip via `/api/v1/data/:table` (uses SurrealDB)
- [ ] Realtime WS: subscribe to a topic and receive Live Query events when data changes
- [ ] Edge Functions API: list + execute via `/api/v1/edge` and `/api/v1/edge/:function`

### Scenario 1: Single Node Bootstrap

```bash
# Initialize workspace
edge-hive init my-node

# Generate identity
edge-hive keygen

# Start server with Tor
edge-hive serve --tor --port 3000

# Expected:
# - Prints onion address (e.g., abc123...onion)
# - Axum serves on localhost:3000
# - Tor hidden service is running
# - /health endpoint returns 200 OK
```

**Validation:**

- [ ] Onion address is valid v3 format (56 chars)
- [ ] Tor Browser can reach `http://<onion>/health`
- [ ] Local curl works: `curl http://localhost:3000/health`

### Scenario 2: Two Nodes Discovery (Local Network)

```bash
# Terminal 1: Node A
edge-hive init node-a
edge-hive serve --tor --mdns

# Terminal 2: Node B (same WiFi)
edge-hive init node-b
edge-hive serve --tor --mdns

# Terminal 2: Check peers
edge-hive peers list

# Expected:
# - Node B discovers Node A via mDNS
# - Peer list shows Node A's PeerId + onion address
```

**Validation:**

- [ ] mDNS discovery works within 5 seconds
- [ ] Peer list is non-empty
- [ ] Can connect to peer: `edge-hive peers connect <peer-id>`

### Scenario 3: Message Exchange (Encrypted)

```bash
# Node A sends message to Node B
edge-hive send <node-b-peer-id> "Hello, encrypted world!"

# Node B receives message
edge-hive messages list

# Expected:
# - Message is encrypted with Node B's Age public key
# - Message is routed via Tor
# - Node B can decrypt and read message
```

**Validation:**

- [ ] Message is encrypted (inspect DB)
- [ ] Message arrives at Node B
- [ ] Decryption is automatic
- [ ] Timestamp is correct

### Scenario 4: Database Persistence

```bash
# Node A: Discover peers, receive messages
edge-hive serve --tor --mdns

# Node A: Restart
# (simulate crash/reboot)
edge-hive serve --tor --mdns

# Expected:
# - Peer list survives restart
# - Messages are still in DB
# - Onion address regenerates (or persists if we add that)
```

**Validation:**

- [ ] Peers are loaded from DB on restart
- [ ] No data loss after restart

## 📦 Test Infrastructure

### Automated Tests

- [ ] Create `tests/e2e/` directory
- [ ] Use `cargo test` with `--test e2e_flow`
- [ ] Mock Tor for CI/CD (use test keypairs)
- [ ] Use `testcontainers` for isolated environments

### Manual Testing Checklist

- [ ] Linux (Ubuntu 22.04)
- [ ] Windows 11
- [ ] Termux (Android 12+)
- [ ] Docker (Alpine Linux)

## ✅ Success Criteria

1. All 4 scenarios pass on Linux, Windows, Termux
2. Automated E2E tests run in CI/CD (GitHub Actions)
3. Documentation updated with example flows
4. Performance: Discovery < 5s, Message send < 2s

## 🔗 Related

- Tor Integration: `.github/issues/FEAT_tor-core-integration.md`
- libp2p Discovery: `.github/issues/FEAT_libp2p-discovery.md`
- SurrealDB: `.github/issues/FEAT_surrealdb-integration.md`

## 🚀 Blocker For

- Android APK build (needs validation first)
- Public beta release
