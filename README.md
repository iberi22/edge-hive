# 🐝 Edge Hive

> **Run Your Server Anywhere: Android, PC, Docker, VPS**  
> **Sovereign Computing with Tor + P2P Networking**

[![Rust](https://img.shields.io/badge/rust-2024_edition-orange?logo=rust)](https://www.rust-lang.org/)
[![SurrealDB](https://img.shields.io/badge/SurrealDB-2.0-pink?logo=surrealdb)](https://surrealdb.com/)
[![Tauri](https://img.shields.io/badge/Tauri-2.0-blue?logo=tauri)](https://tauri.app/)
[![License](https://img.shields.io/badge/license-BUSL--1.1-blue)](LICENSE)

---

## 🎯 What is Edge Hive?

Edge Hive transforms **any device** into a sovereign compute node. Deploy the same Rust binary on:

- 📱 **Android phones** (via Termux, no root)
- 💻 **PCs** (Linux, Windows, macOS)
- 🐳 **Docker** containers (Alpine-based, 15MB image)
- ☁️ **VPS** servers (any provider, generic SSH)
- 🍓 **Raspberry Pi** and ARM devices

All nodes communicate through **Tor** (anonymous, NAT-proof) + **libp2p** (fast P2P), forming a unified swarm where:

- 🔐 **Identity > IP**: Nodes use Ed25519 keypairs, not IP addresses
- 🌐 **Works Everywhere**: Behind firewalls, CGNAT, corporate proxies
- 🧅 **Privacy First**: Tor onion services by default (optional Cloudflare Tunnel)
- 📊 **One Database**: SurrealDB syncs across all your devices
- 🚀 **Zero Config**: Works out-of-the-box, no port forwarding

## ✨ Features

| Feature | Description |
|---------|-------------|
| **Multi-Platform Binary** | Single Rust codebase → 8 compilation targets (Android, Linux, Windows, Docker) |
| **Tor Integration** | Anonymous `.onion` addresses, censorship-resistant, NAT traversal |
| **libp2p Mesh** | Fast local sync (mDNS), global discovery (Kademlia DHT) |
| **Cryptographic Identity** | Ed25519 keypairs replace IP addresses - identity is portable |
| **SurrealDB Embedded** | Offline-first database with real-time sync, graphs, vector search |
| **WASM Plugins** | Extend functionality with sandboxed WebAssembly modules |
| **Cloudflare Tunnel** | Optional public HTTPS endpoints (for web services) |

## 🚀 Quick Start

### Android (Termux - No Root Required)

For detailed instructions on how to install and configure Edge Hive on Termux, please see the comprehensive [Termux Guide](docs/agent-docs/GUIDE_TERMUX.md).

```bash
# Quick Install
bash <(curl -fsSL https://edgehive.dev/install-termux.sh)
```

### Linux/Windows

```bash
# Download binary
curl -sSL https://github.com/your-org/edge-hive/releases/latest/download/edge-hive-$(uname -m) -o edge-hive
chmod +x edge-hive

# Initialize and run
./edge-hive init
./edge-hive serve
```

### Android APK

Download the APK from [Releases](https://github.com/your-org/edge-hive/releases) and install.

## 📐 Architecture

```
┌─────────────────────────────────────────────────────────┐
│                     Edge Hive Node                       │
├─────────────────────────────────────────────────────────┤
│  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐    │
│  │ Axum    │  │ Surreal │  │ WASM    │  │ libp2p  │    │
│  │ HTTP    │  │ DB 2.0  │  │ Plugins │  │ P2P     │    │
│  └────┬────┘  └────┬────┘  └────┬────┘  └────┬────┘    │
│       └────────────┴────────────┴────────────┘          │
│                         │                                │
│              ┌──────────┴──────────┐                    │
│              │   Rust Core Engine   │                    │
│              └──────────┬──────────┘                    │
│       ┌─────────────────┼─────────────────┐             │
│  ┌────┴────┐       ┌────┴────┐       ┌────┴────┐       │
│  │ LibCFD  │       │  Arti   │       │  mDNS   │       │
│  │ Tunnel  │       │  Onion  │       │ + DHT   │       │
│  └─────────┘       └─────────┘       └─────────┘       │
└─────────────────────────────────────────────────────────┘
```

## 💰 Pricing (Coming Soon)

| Tier | Price | Features |
|------|-------|----------|
| **Open Source** | Free Forever | Self-host unlimited nodes |
| **Pro** | $25/mo | 1 managed cloud node, 10GB storage |
| **Team** | $100/mo | 5 managed nodes, 100GB, team auth |
| **Enterprise** | Custom | Dedicated infra, SLA, support |

## 🛠️ Development

```bash
# Clone
git clone https://github.com/your-org/edge-hive.git
cd edge-hive

# Build core
cargo build --release

# Build Android APK
cd app && npm run tauri android build

# Run tests
cargo test --all
```

## 📚 Documentation

- [Architecture Decision Records](/.✨/ARCHITECTURE.md)
- [Agent Index](/.✨/AGENT_INDEX.md)
- [API Reference](/docs/api/)
- [Plugin Development](/docs/plugins/)

## 🤝 Contributing

We follow the **Git-Core Protocol**. See [CONTRIBUTING.md](CONTRIBUTING.md).

1. Check existing [Issues](https://github.com/your-org/edge-hive/issues)
2. Create issue first, then branch
3. Submit PR with tests
4. Pass CI checks

## 📄 License

**Edge Hive** is licensed under the **Business Source License 1.1 (BSL)**.

> **Source Available**: You can view, modify, and use the code for personal or internal business purposes.
> **Commercial Restriction**: You may NOT provide "Edge Hive" as a managed service to third parties.
> **Open Source Transition**: The code automatically converts to Apache 2.0 after 4 years.

See [LICENSE](LICENSE) for details.

---

**Built with 🦀 Rust and ❤️ by the Edge Hive community**
