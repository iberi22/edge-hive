---
title: "Agent & Skill Index - Edge Hive Project"
type: INDEX
id: "index-agents-edge-hive"
created: 2025-12-14
updated: 2025-12-14
agent: antigravity
model: gemini-3-pro
requested_by: user
summary: |
  Index of specialized agent roles for Edge Hive development,
  including Rust systems, P2P networking, and mobile experts.
keywords: [agents, index, skills, roles, rust, distributed]
tags: ["#index", "#agents", "#skills", "#rust"]
project: Edge-Hive
---

# 🧠 Agent & Skill Index - Edge Hive

## 🚦 Routing Logic

When working on Edge Hive, identify the **Domain** and **Role**.
Then, run the `equip-agent` script to load that persona.

---

## 📂 Domain: Rust Systems

| Role | Description | Source Recipe | Recommended Skills |
|------|-------------|---------------|-------------------|
| **Rust Core Engineer** | Core engine, async Rust, Tokio | `rust/core-engineer.md` | `tokio`, `axum`, `error-handling` |
| **Rust Crypto Specialist** | Ed25519, encryption, identity | `rust/crypto-specialist.md` | `ed25519-dalek`, `noise-protocol` |
| **Rust WASM Developer** | Plugin system, Wasmtime | `rust/wasm-developer.md` | `wasmtime`, `wit-bindgen` |
| **Cross-Compiler** | Android NDK, multi-target builds | `rust/cross-compiler.md` | `cargo-ndk`, `cross` |

---

## 📂 Domain: Distributed Systems

| Role | Description | Source Recipe | Recommended Skills |
|------|-------------|---------------|-------------------|
| **P2P Network Architect** | libp2p, DHT, node discovery | `p2p/network-architect.md` | `libp2p`, `kademlia`, `mdns` |
| **Sync Engineer** | Data replication, CRDT | `p2p/sync-engineer.md` | `surrealdb-sync`, `crdt` |
| **Tunnel Specialist** | Cloudflare, Tor integration | `p2p/tunnel-specialist.md` | `libcfd`, `arti`, `quic` |

---

## 📂 Domain: Database

| Role | Description | Source Recipe | Recommended Skills |
|------|-------------|---------------|-------------------|
| **SurrealDB Expert** | Embedded DB, SurrealQL | `database/surrealdb-expert.md` | `surrealdb`, `surql`, `rocksdb` |
| **Schema Designer** | Data modeling, indexes | `database/schema-designer.md` | `graph-modeling`, `vector-search` |

---

## 📂 Domain: Mobile & Frontend

| Role | Description | Source Recipe | Recommended Skills |
|------|-------------|---------------|-------------------|
| **Tauri Mobile Dev** | Android/iOS builds, IPC | `mobile/tauri-developer.md` | `tauri-2`, `cargo-mobile` |
| **Astro Developer** | Static site generation | `mobile/astro-developer.md` | `astro`, `islands` |
| **Svelte UI Engineer** | Reactive components | `mobile/svelte-engineer.md` | `svelte-5`, `runes` |
| **Termux Specialist** | Android shell, installation | `mobile/termux-specialist.md` | `termux-api`, `proot` |

---

## 📂 Domain: Infrastructure

| Role | Description | Source Recipe | Recommended Skills |
|------|-------------|---------------|-------------------|
| **AWS Architect** | Cloud deployment, pricing | `infra/aws-architect.md` | `ec2`, `s3`, `cloudfront` |
| **DevOps Automator** | CI/CD, cross-compilation | `infra/devops-automator.md` | `github-actions`, `docker` |
| **Security Auditor** | Threat modeling, hardening | `infra/security-auditor.md` | `threat-model`, `sbom` |

---

## 📂 Domain: Product & Business

| Role | Description | Source Recipe | Recommended Skills |
|------|-------------|---------------|-------------------|
| **Product Strategist** | Roadmap, monetization | `product/strategist.md` | `baas-model`, `pricing` |
| **Technical Writer** | Docs, API reference | `product/tech-writer.md` | `mdbook`, `openapi` |
| **Growth Hacker** | User acquisition, community | `product/growth-hacker.md` | `discord`, `oss-marketing` |

---

## 🛠️ Skill Definitions (Capabilities)

*Skills are appended to the agent's context when loaded.*

### Rust Skills
- **tokio**: Async runtime patterns, spawn, select
- **axum**: Router, extractors, middleware
- **ed25519-dalek**: Key generation, signing, verification
- **wasmtime**: WASM instantiation, memory management
- **cargo-ndk**: Android NDK integration

### P2P Skills
- **libp2p**: Swarm, transport, protocols
- **kademlia**: DHT operations, peer routing
- **mdns**: Local discovery, service announcement

### Mobile Skills
- **tauri-2**: Mobile commands, IPC, plugins
- **astro**: Islands, SSG, integrations
- **svelte-5**: Runes, reactivity, stores

### Infrastructure Skills
- **github-actions**: Cross-compilation workflows
- **cross**: Cross-compilation with Docker
- **aws-cdk**: Infrastructure as code

---

## 🎯 Task-to-Agent Mapping

| Task Type | Recommended Agent |
|-----------|-------------------|
| Core engine development | Rust Core Engineer |
| Node discovery system | P2P Network Architect |
| Cryptographic identity | Rust Crypto Specialist |
| SurrealDB integration | SurrealDB Expert |
| Plugin system | Rust WASM Developer |
| Tunneling (CF/Tor) | Tunnel Specialist |
| Android APK build | Tauri Mobile Dev + Cross-Compiler |
| UI components | Svelte UI Engineer |
| Termux installation | Termux Specialist |
| AWS deployment | AWS Architect |
| CI/CD pipelines | DevOps Automator |
| Monetization strategy | Product Strategist |

---

## 🚀 How to Activate

Run in terminal:

```powershell
./scripts/equip-agent.ps1 -Role "Rust Core Engineer"
```

Or for this project specifically:

```powershell
./scripts/equip-agent.ps1 -Role "P2P Network Architect" -Project "Edge-Hive"
```

---

*Last updated by AI Agent: 2025-12-14*
