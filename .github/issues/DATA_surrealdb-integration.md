---
title: "[DATA] SurrealDB 2.0 Embedded Integration"
labels:
  - database
  - core
  - priority-high
assignees: []
---

## User Story

**As a** node operator
**I want** a persistent embedded database
**So that** data survives restarts and syncs between nodes

## Technical Specs

### Crate: `edge-hive-db`

```toml
[dependencies]
surrealdb = { version = "2.0", features = ["kv-rocksdb"] }
serde = { version = "1", features = ["derive"] }
tokio = { version = "1.40", features = ["full"] }
```

### Database Structure

```
Namespaces:
├── edge_hive/
│   ├── system       # Node config, health
│   ├── identity     # Keypairs, known peers
│   ├── plugins      # WASM registry
│   └── user_data    # Application data
```

### Database Service

```rust
pub struct DatabaseService {
    db: Surreal<RocksDb>,
}

impl DatabaseService {
    pub async fn new(path: &Path) -> Result<Self>;
    pub async fn query<T>(&self, sql: &str) -> Result<T>;
    pub async fn live<T>(&self, sql: &str) -> impl Stream<Item = T>;

    // Node operations
    pub async fn save_peer(&self, peer: &PeerInfo) -> Result<()>;
    pub async fn get_peers(&self) -> Result<Vec<PeerInfo>>;
    pub async fn save_config(&self, cfg: &NodeConfig) -> Result<()>;
}
```

### Schemas

```surql
-- Peer table
DEFINE TABLE peer SCHEMAFULL;
DEFINE FIELD peer_id ON peer TYPE string;
DEFINE FIELD name ON peer TYPE string;
DEFINE FIELD addresses ON peer TYPE array<string>;
DEFINE FIELD last_seen ON peer TYPE datetime;
DEFINE FIELD capabilities ON peer TYPE int;
DEFINE INDEX peer_id_idx ON peer FIELDS peer_id UNIQUE;

-- Config table
DEFINE TABLE config SCHEMAFULL;
DEFINE FIELD key ON config TYPE string;
DEFINE FIELD value ON config TYPE any;
DEFINE INDEX key_idx ON config FIELDS key UNIQUE;
```

## Acceptance Criteria

- [ ] RocksDB storage persists across restarts
- [ ] CRUD operations for peers and config
- [ ] Live queries work for realtime updates
- [ ] Migrations system in place
- [ ] Backup/restore functionality
- [ ] Unit tests for all operations

## Branch

`feat/surrealdb-integration`
