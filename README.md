# 🐝 Edge Hive

> **Your Personal VPS Swarm - Sovereign Computing at the Edge**

[![Rust](https://img.shields.io/badge/rust-2024_edition-orange?logo=rust)](https://www.rust-lang.org/)
[![SurrealDB](https://img.shields.io/badge/SurrealDB-2.0-pink?logo=surrealdb)](https://surrealdb.com/)
[![Tauri](https://img.shields.io/badge/Tauri-2.0-blue?logo=tauri)](https://tauri.app/)
[![License](https://img.shields.io/badge/license-BSL--1.1-blue)](LICENSE)

---

## 🎯 What is Edge Hive?

Edge Hive is a **distributed edge computing platform** that transforms old Android phones, spare laptops, and cloud instances into a unified personal server swarm. Think of it as **your own Supabase/Firebase**, but:

- 🦀 **Built in Rust** for maximum performance and safety
- 📱 **Runs on Android** via Termux or native APK
- 🌐 **Cross-platform** (Android, Linux, Windows)
- 🔗 **Auto-discovers** other nodes on your network
- 🚀 **Exposes to internet** via Cloudflare Tunnel or Tor

## ✨ Features

| Feature | Description |
|---------|-------------|
| **Node Discovery** | Auto-find other Edge Hive nodes on local network (mDNS) and globally (Kademlia DHT) |
| **Cryptographic Identity** | Ed25519 keypairs replace IP addresses - your node is portable |
| **SurrealDB Embedded** | Full database with realtime, graphs, and vector search - offline-first |
| **WASM Plugins** | Extend functionality with sandboxed WebAssembly modules |
| **Dual Tunneling** | Cloudflare Tunnel (fast) + Tor Onion (censorship-resistant) |
| **Cross-Platform** | One codebase → Android APK, Linux binary, Windows EXE |

## 🚀 Quick Start

### Termux (Android)

```bash
# Install from script
curl -sSL https://edge-hive.io/install.sh | bash

# Or manually
pkg install rust
cargo install edge-hive
edge-hive init
edge-hive serve
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
