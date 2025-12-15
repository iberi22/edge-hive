---
title: "[NET] libp2p Node Discovery (mDNS + Kademlia)"
labels:
  - networking
  - p2p
  - priority-high
assignees:
  - jules
---

## User Story

**As a** node operator
**I want** automatic peer discovery
**So that** nodes find each other without manual configuration

## Technical Specs

### Crate: `edge-hive-discovery`

```toml
[dependencies]
libp2p = { version = "0.54", features = ["tokio", "mdns", "kad", "noise", "quic", "tcp", "dns", "identify"] }
tokio = { version = "1.40", features = ["full"] }
futures = "0.3"
tracing = "0.1"
```

### Discovery Layers

| Layer | Protocol | Scope | Latency |
|-------|----------|-------|---------|
| Local | mDNS | LAN | < 30s |
| Global | Kademlia DHT | Internet | < 2min |

### Swarm Behavior

```rust
#[derive(NetworkBehaviour)]
pub struct HiveBehaviour {
    mdns: mdns::tokio::Behaviour,
    kademlia: kad::Behaviour<MemoryStore>,
    identify: identify::Behaviour,
}

pub struct DiscoveryService {
    swarm: Swarm<HiveBehaviour>,
    peers: Arc<RwLock<HashMap<PeerId, PeerInfo>>>,
}

impl DiscoveryService {
    pub async fn start(&mut self) -> Result<()>;
    pub fn peers(&self) -> Vec<PeerInfo>;
    pub async fn dial(&mut self, peer_id: PeerId) -> Result<()>;
}
```

### Bootstrap Nodes

For global discovery, connect to well-known bootstrap nodes:
- `boot1.edge-hive.io:9000`
- `boot2.edge-hive.io:9000`

## Acceptance Criteria

- [ ] mDNS discovers local peers within 30 seconds
- [ ] Kademlia connects to bootstrap nodes
- [ ] Peer list updates in real-time
- [ ] QUIC transport preferred, TCP fallback
- [ ] Noise protocol encryption verified
- [ ] Integration test with 2+ nodes

## Branch

`feat/node-discovery`
