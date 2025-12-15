# 🏗️ Edge Hive - Architecture Decision Records

> **Living Document** - Updated as the project evolves

---

## 🎯 Project Vision

**Edge Hive** transforms old Android phones, spare laptops, and cloud instances into a unified personal server swarm. A sovereign computing platform where users own their data and infrastructure.

### Design Philosophy (Inspired by Urbit)

1. **Identity over IP**: Nodes use cryptographic identities (Ed25519), not IP addresses
2. **Time-Travel Database**: SurrealDB with versioned state (like Urbit's Clay filesystem)
3. **Minimalist Kernel**: Rust core + WASM plugins (like Urbit's Arvo + userspace apps)
4. **Portable State**: Copy your `.edge-hive/` folder = copy your entire server

---

## 📋 CRITICAL DECISIONS

### System Architecture

| Decision | Choice | Rationale | Status |
|----------|--------|-----------|--------|
| **Language** | Rust 2024 Edition | Memory safety, zero-cost abstractions, Android compatibility | ✅ Final |
| **Async Runtime** | Tokio | Industry standard, excellent ecosystem | ✅ Final |
| **HTTP Server** | Axum | Type-safe, fast, Tokio-native | ✅ Final |
| **Database** | SurrealDB 2.0 (embedded) | Multi-model, runs in-process, sync capabilities | ✅ Final |
| **P2P Network** | libp2p | mDNS + Kademlia DHT for discovery | ✅ Final |
| **Plugin System** | WebAssembly (Wasmtime) | Sandboxed extensions, "run code not containers" | ✅ Final |

### Mobile & Desktop

| Decision | Choice | Rationale | Status |
|----------|--------|-----------|--------|
| **Framework** | Tauri 2.0 | Shared Rust backend, web UI, native performance | ✅ Final |
| **UI Framework** | Astro + Svelte | Static generation + reactive components | ✅ Final |
| **Styling** | Tailwind CSS | Utility-first, optimized for dark mode/glassmorphism | ✅ Final |
| **Target Platforms** | Android, Linux, Windows | Android via APK, desktop native | ✅ Final |

### Infrastructure

| Decision | Choice | Rationale | Status |
|----------|--------|-----------|--------|
| **Tunneling** | Cloudflare Tunnel (primary) | Fast, reliable, free tier | ✅ Final |
| **Backup Tunnel** | Tor Onion (v1.1+) | Censorship-resistant fallback | 🔄 Planned |
| **Android Runtime** | Termux | No root required, full Rust support | ✅ Final |
| **Cloud Provider** | AWS (managed tier) | Auto-provisioning via SDK | ✅ Final |
| **Billing** | Stripe | Industry standard, subscription ready | ✅ Final |

### Security

| Decision | Choice | Rationale | Status |
|----------|--------|-----------|--------|
| **Identity System** | Ed25519 keypairs | Sovereign identity, portable across IPs | ✅ Final |
| **Encryption** | TLS 1.3 | All tunnels encrypted by default | ✅ Final |
| **Plugin Isolation** | WASM sandboxing | Capability-based security (WASI) | ✅ Final |

### Business Model

| Decision | Choice | Rationale | Status |
|----------|--------|-----------|--------|
| **License** | BSL 1.1 (→ MIT after 2 years) | Source available, commercial protection | ✅ Final |
| **Monetization** | Managed cloud nodes | Supabase model: free self-host, pay for managed | ✅ Final |
| **Pricing Tiers** | Free / $25 / $100 / Enterprise | See pricing table in README | 🔄 Design |

---

## 🏛️ System Architecture

### Core Components

```
┌─────────────────────────────────────────────────────────────┐
│                   Edge Hive Node                            │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌──────────┐  ┌───────────┐  ┌────────┐  ┌──────────┐   │
│  │  Axum    │  │ SurrealDB │  │  WASM  │  │  libp2p  │   │
│  │  HTTP    │  │   2.0     │  │ Engine │  │  Swarm   │   │
│  │  API     │  │ Embedded  │  │ Wasmtm │  │  mDNS    │   │
│  └────┬─────┘  └─────┬─────┘  └───┬────┘  └────┬─────┘   │
│       │              │            │             │          │
│       └──────────────┴────────────┴─────────────┘          │
│                         │                                   │
│              ┌──────────┴──────────┐                       │
│              │  Rust Core Engine   │                       │
│              │  (Tokio Runtime)    │                       │
│              └──────────┬──────────┘                       │
│       ┌─────────────────┼─────────────────┐                │
│  ┌────┴─────┐      ┌────┴────┐      ┌────┴────┐          │
│  │ LibCFD   │      │  Arti   │      │ Identity│          │
│  │ CFTunnel │      │  Tor    │      │ Ed25519 │          │
│  └──────────┘      └─────────┘      └─────────┘          │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### Crate Structure (Rust Workspace)

```
edge-hive/
├── crates/
│   ├── edge-hive-core/        ⭐ Main binary, CLI, HTTP server
│   ├── edge-hive-identity/    🔑 Ed25519 keypair management
│   ├── edge-hive-discovery/   🌐 libp2p mDNS + Kademlia DHT
│   ├── edge-hive-tunnel/      🚇 Cloudflare Tunnel integration
│   ├── edge-hive-db/          💾 SurrealDB wrapper & migrations
│   ├── edge-hive-wasm/        ⚙️ Wasmtime plugin runtime
│   ├── edge-hive-billing/     💳 Stripe subscription logic
│   └── edge-hive-cloud/       ☁️ AWS EC2 auto-provisioning
└── app/
    ├── src-tauri/             📱 Tauri backend (uses crates above)
    └── src/                   🎨 Astro + Svelte UI
```

---

## 🔐 Security Model

### 1. Identity-Based Networking (Urbit-Inspired)

**Traditional Problem**: Nodes use IPs (192.168.1.10). IP changes = node unreachable.

**Edge Hive Solution**:
- Each node generates an **Ed25519 keypair** on first boot
- Nodes identify each other by **public key hash**, not IP
- libp2p DHT maps `node-id` → current IP/port
- Result: Phone switches from WiFi to 4G? No problem, DHT updates automatically.

### 2. Plugin Sandboxing

- All user plugins run in **WASM** with **WASI** capabilities
- Plugins can't access filesystem, network, or host memory unless explicitly granted
- Example: A "image-resize" plugin only gets access to `/tmp/uploads/`, not entire disk

### 3. Encryption Everywhere

- All inter-node communication via **TLS 1.3**
- Cloudflare Tunnel uses **Argo Tunnel encryption**
- Local database files encrypted at rest (optional, via SurrealDB encryption layer)

---

## 🌐 Network Topology

### Discovery Flow

```
1. Node boots → Generates/loads Ed25519 keypair
2. Starts libp2p with mDNS (local) + Kademlia (global)
3. Announces public key hash to DHT
4. Other nodes query DHT for "known friends" (pre-shared keys)
5. Connection established via QUIC (over UDP, faster than TCP)
```

### Multi-Node RAID (Hive Clustering)

**Use Case**: User has 2 Android phones + 1 AWS instance.

**Setup**:
1. All nodes share same **Hive ID** (derived from owner's master key)
2. SurrealDB configured in **cluster mode** (eventual consistency)
3. Data sharded: Phone A gets 33%, Phone B gets 33%, AWS gets 34%
4. Replication factor: 2 (each shard stored on 2 nodes)

**Result**:
- 1 node fails → data still available from replica
- Aggregate storage = sum of all nodes
- Reads distributed across nodes (faster)

---

## 📱 Mobile App Flow

### QR Code Pairing

```
Desktop App                          Android Phone
    │                                     │
    │ Generate QR with:                   │
    │  - Node public key                  │
    │  - IP:port                           │
    │  - Temp auth token                  │
    ├─────────────────────────────────────>│
    │                                     │ Scan QR
    │                                     │ Extract data
    │                                     │ POST /pair with token
    │<─────────────────────────────────────┤
    │ Verify token                         │
    │ Add phone to Hive                   │
    │ Return Hive config                  │
    ├─────────────────────────────────────>│
    │                                     │ Save config
    │                                     │ Join swarm
```

### Cloud Provisioning (One-Click)

```
User in App                    Edge Hive Backend              AWS
    │                                │                          │
    │ Tap "Add Cloud Node"           │                          │
    ├───────────────────────────────>│                          │
    │                                │ Create EC2 via SDK       │
    │                                ├─────────────────────────>│
    │                                │                          │ Instance boots
    │                                │<─────────────────────────┤ Returns IP
    │                                │ Install edge-hive via SSH│
    │                                │ Configure with Hive ID   │
    │                                │ Start Cloudflare Tunnel  │
    │                                │                          │
    │<───────────────────────────────┤ Return node status       │
    │ "Node online in 2 min"         │                          │
```

---

## 💡 Future Enhancements

| Feature | Priority | Complexity | Status |
|---------|----------|------------|--------|
| **RAID-5 style parity** | Medium | High | 🔄 Research |
| **Tor onion service** | High | Medium | 🔄 v1.1 |
| **iOS app** | Low | Medium | ❌ Not planned yet |
| **Plugin marketplace** | High | High | 🔄 v2.0 |
| **Multi-user auth** | Medium | Medium | 🔄 v1.5 |

---

## 📚 References

- [Urbit Architecture](https://urbit.org/docs/system/architecture/) - Identity & portability concepts
- [libp2p Specs](https://github.com/libp2p/specs) - P2P networking
- [SurrealDB Docs](https://surrealdb.com/docs) - Embedded database
- [Tauri Architecture](https://tauri.app/v1/references/architecture/) - Mobile app framework
- [Cloudflare Tunnel](https://developers.cloudflare.com/cloudflare-one/connections/connect-apps/) - Tunneling

---

**Last Updated**: December 2025  
**Protocol Version**: Git-Core 3.2.1  
**Project Phase**: MVP Development
