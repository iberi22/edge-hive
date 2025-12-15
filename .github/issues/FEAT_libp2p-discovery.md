---
title: "Implement libp2p Discovery (mDNS + Kademlia DHT)"
labels:
  - enhancement
  - p2p
  - networking
  - discovery
  - priority-high
assignees: []
---

## 🎯 Objective

Enable automatic peer discovery for Edge Hive nodes using:

1. **mDNS**: Local network discovery (LAN, same WiFi)
2. **Kademlia DHT**: Global peer routing (Internet-wide)

## 📋 Context

Edge Hive uses a dual network stack:

- **Tor**: Anonymous, censorship-resistant (already implemented in `edge-hive-tunnel`)
- **libp2p**: Fast P2P for local networks + DHT for global routing

ARCHITECTURE.md specifies libp2p as the P2P discovery layer.

## 🛠️ Tasks

### Phase 1: Basic libp2p Setup

- [ ] Add `libp2p` dependency to `edge-hive-discovery/Cargo.toml`
- [ ] Generate PeerId from existing Ed25519 identity
- [ ] Create libp2p Swarm with TCP transport

### Phase 2: Local Discovery (mDNS)

- [ ] Enable mDNS protocol for LAN discovery
- [ ] Broadcast node availability on local network
- [ ] Listen for mDNS peers and store in peer list
- [ ] Test: Two nodes on same WiFi should discover each other

### Phase 3: Global Routing (Kademlia DHT)

- [ ] Initialize Kademlia DHT with bootstrap nodes
- [ ] Publish node's PeerId + Tor onion address to DHT
- [ ] Query DHT for peers by topic/namespace
- [ ] Test: Two nodes on different networks find each other via DHT

### Phase 4: Integration

- [ ] Bridge libp2p discovery with `edge-hive-core`
- [ ] Store discovered peers in SurrealDB (when DB is ready)
- [ ] CLI command: `edge-hive peers list`
- [ ] CLI command: `edge-hive peers connect <peer-id>`

## 📦 Dependencies

```toml
libp2p = { version = "0.54", features = ["tcp", "mdns", "kad", "noise", "yamux"] }
```

## ✅ Success Criteria

1. **mDNS**: Nodes on same network discover each other automatically
2. **Kademlia**: Nodes announce themselves to global DHT
3. **Querying**: `edge-hive peers list` shows discovered nodes
4. **Connecting**: Can establish connection to discovered peer

## 🔗 Related

- Architecture: `.✨/ARCHITECTURE.md` (P2P Network section)
- Crate: `crates/edge-hive-discovery/`
- Existing identity: `crates/edge-hive-identity/` (Ed25519 keys)

## 📚 References

- [libp2p Rust Docs](https://docs.rs/libp2p)
- [Kademlia DHT Tutorial](https://docs.libp2p.io/concepts/discovery-routing/kaddht/)
- [mDNS Protocol](https://docs.rs/libp2p-mdns)
