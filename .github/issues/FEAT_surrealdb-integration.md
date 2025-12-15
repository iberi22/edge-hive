---
title: "Integrate SurrealDB as Embedded Database"
labels:
  - enhancement
  - database
  - storage
  - priority-medium
assignees: []
---

## 🎯 Objective

Add SurrealDB as the embedded database for Edge Hive, storing:

- **Nodes**: Discovered peers (PeerId, onion address, metadata)
- **Messages**: Encrypted messages between nodes
- **Workspace State**: Configuration, plugins, logs

## 📋 Context

ARCHITECTURE.md specifies SurrealDB 2.0 (embedded mode) as the database:

- **Multi-model**: Documents, graphs, key-value
- **Runs in-process**: No separate server needed
- **Sync capabilities**: Time-travel, versioned state (like Urbit's Clay)

Storage location: `.edge-hive/db/` (portable, can be rsync'd)

## 🛠️ Tasks

### Phase 1: Setup

- [ ] Add `surrealdb = { version = "2.0", default-features = false, features = ["kv-rocksdb"] }` to `edge-hive-db/Cargo.toml`
- [ ] Initialize database at `.edge-hive/db/` on first run
- [ ] Create schema migration system (versioned)

### Phase 2: Schemas

- [ ] **Nodes Table**: `{ id, peer_id, onion_address, public_key, last_seen, metadata }`
- [ ] **Messages Table**: `{ id, from, to, encrypted_content, timestamp, signature }`
- [ ] **Plugins Table**: `{ id, name, version, wasm_hash, enabled, config }`
- [ ] **Logs Table**: `{ id, level, message, timestamp, source }`

### Phase 3: API Layer

- [ ] `DbConnection::new()` - Initialize connection
- [ ] `DbConnection::add_peer()` - Store discovered peer
- [ ] `DbConnection::get_peers()` - Query all known peers
- [ ] `DbConnection::store_message()` - Save encrypted message
- [ ] `DbConnection::get_messages()` - Retrieve messages

### Phase 4: Integration

- [ ] Use DB in `edge-hive-discovery` to persist peers
- [ ] Use DB in `edge-hive-core` for message storage
- [ ] CLI command: `edge-hive db status` (show stats)
- [ ] CLI command: `edge-hive db backup <path>` (export DB)
- [ ] CLI command: `edge-hive db restore <path>` (import DB)

## 📦 Dependencies

```toml
[dependencies]
surrealdb = { version = "2.0", default-features = false, features = ["kv-rocksdb"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
```

## ✅ Success Criteria

1. Database initializes at `.edge-hive/db/` on first run
2. Peers discovered via libp2p are persisted
3. Database survives restarts (data is durable)
4. Backup/restore works correctly
5. DB operations are async (non-blocking)

## 🔗 Related

- Architecture: `.✨/ARCHITECTURE.md` (Database section)
- Crate: `crates/edge-hive-db/`
- Discovery: Will use this for peer storage
- Core: Will use this for message storage

## 📚 References

- [SurrealDB Embedded Mode](https://surrealdb.com/docs/embedding)
- [RocksDB Backend](https://surrealdb.com/docs/deployment/storage-engines#rocksdb)
