---
title: "[INFRA] Cloudflare Tunnel (LibCFD) Integration"
labels:
  - infrastructure
  - networking
  - tunneling
assignees: []
---

## User Story

**As a** node operator
**I want** to expose my node to the internet
**So that** external clients can access my services securely

## Technical Specs

### Crate: `edge-hive-tunnel`

```toml
[dependencies]
# Primary: Native Rust CF Tunnel
libcfd = "0.1"

# Fallback: cloudflared binary wrapper
tokio = { version = "1.40", features = ["full", "process"] }

# Tor support (v1.1)
arti-client = "0.22"
tor-hsservice = "0.22"
```

### Tunnel Service

```rust
pub enum TunnelBackend {
    LibCFD,           // Native Rust (preferred)
    Cloudflared,      // Fallback: spawn binary
    Tor,              // Censorship-resistant (v1.1)
}

pub struct TunnelService {
    backend: TunnelBackend,
    public_url: Option<String>,
}

impl TunnelService {
    pub async fn start(&mut self, local_port: u16) -> Result<String>;
    pub async fn stop(&mut self) -> Result<()>;
    pub fn public_url(&self) -> Option<&str>;
}
```

### Configuration

```toml
# ~/.edge-hive/config.toml
[tunnel]
enabled = true
backend = "libcfd"  # or "cloudflared", "tor"
token = "${CF_TUNNEL_TOKEN}"  # Named tunnel
```

### Quick Tunnel (Development)

For development, use TryCloudflare (no account needed):
```rust
let url = tunnel.start_quick(8080).await?;
// Returns: https://random-slug.trycloudflare.com
```

## Acceptance Criteria

- [ ] LibCFD connects to Cloudflare edge
- [ ] Named tunnel works with token
- [ ] Quick tunnel works without account
- [ ] Fallback to cloudflared binary works
- [ ] Public URL accessible from internet
- [ ] Graceful shutdown

## Branch

`feat/cloudflare-tunnel`
