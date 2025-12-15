---
title: "[NET] Ed25519 Cryptographic Identity System"
labels:
  - networking
  - crypto
  - security
assignees:
  - jules
---

## User Story

**As a** node operator
**I want** a persistent cryptographic identity
**So that** other nodes can verify my authenticity

## Technical Specs

### Crate: `edge-hive-identity`

```toml
[dependencies]
ed25519-dalek = { version = "2.1", features = ["rand_core"] }
rand = "0.8"
base64 = "0.22"
serde = { version = "1", features = ["derive"] }
zeroize = { version = "1.7", features = ["derive"] }
directories = "5.0"
```

### Key Components

```rust
pub struct NodeIdentity {
    keypair: ed25519_dalek::SigningKey,
    name: String,  // Human-readable: "hive-alpha-7x9k"
    created_at: DateTime<Utc>,
}

impl NodeIdentity {
    pub fn generate() -> Self;
    pub fn load(path: &Path) -> Result<Self>;
    pub fn save(&self, path: &Path) -> Result<()>;
    pub fn public_key(&self) -> VerifyingKey;
    pub fn sign(&self, message: &[u8]) -> Signature;
    pub fn peer_id(&self) -> String;  // Base64 of public key
}
```

### Storage

- Keys stored in `~/.edge-hive/identity.key`
- Encrypted at rest with passphrase (optional)
- Human-readable name in `~/.edge-hive/config.toml`

## Acceptance Criteria

- [ ] Key generation works
- [ ] Keys persist across restarts
- [ ] Sign/verify roundtrip passes
- [ ] Human-readable names generated (adjective-noun-hex)
- [ ] Keys zeroized on drop
- [ ] Unit tests for all operations

## Branch

`feat/identity-system`
