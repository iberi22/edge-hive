# 🌐 Edge Hive - Network Architecture (Tor + libp2p)

> **Multi-platform deployment: Android, PC, Docker, VPS**  
> **Dual network stack: Tor (anonymous) + libp2p (fast) + Cloudflare (public)**

---

## 🎯 Network Design Goals

| Goal | Solution | Status |
|------|----------|--------|
| **Zero Config** | Works behind any NAT/firewall | ✅ Tor auto-NAT traversal |
| **Privacy First** | IP addresses hidden by default | ✅ Tor onion services |
| **Fast Local** | <1ms latency on same LAN | ✅ libp2p mDNS |
| **Global Mesh** | No central server dependency | ✅ Kademlia DHT |
| **Public Optional** | Expose services to clearnet | ✅ Cloudflare Tunnel |

---

## 📊 Network Stack Comparison

| Feature | Tor Onion | libp2p | Cloudflare Tunnel |
|---------|-----------|--------|-------------------|
| **NAT Traversal** | ✅ Auto | ✅ STUN/TURN | ✅ Auto |
| **Latency** | ~200ms | ~50ms | ~30ms |
| **Anonymity** | ✅ IP hidden | ❌ IP visible | ❌ IP logged |
| **Censorship Resistance** | ✅ Yes | ⚠️ Depends | ❌ Can be blocked |
| **Bandwidth** | ~1-5 Mbps | Unlimited | Unlimited |
| **Setup** | Zero config | Zero config | Requires domain |
| **Cost** | Free | Free | Free (limits) |

**Recommendation:**
- **Default:** Tor (privacy + NAT traversal)
- **Optimization:** libp2p for LAN + known peers
- **Public:** CF Tunnel for web dashboards only

---

## 🏗️ Network Stack Layers

```
┌──────────────────────────────────────────────────────────────┐
│  Layer 4: Application Protocol                              │
│  ┌─────────┐  ┌──────────┐  ┌─────────┐  ┌────────────┐    │
│  │  HTTP   │  │ GraphQL  │  │  gRPC   │  │  Custom    │    │
│  │  REST   │  │ Websock  │  │ Streams │  │  Binary    │    │
│  └────┬────┘  └────┬─────┘  └────┬────┘  └─────┬──────┘    │
├───────┴─────────────┴──────────────┴─────────────┴───────────┤
│  Layer 3: Transport Selection (Auto)                        │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │ Tor Hidden   │  │ libp2p QUIC  │  │ CF Tunnel    │      │
│  │ Service v3   │  │ (UDP-based)  │  │ (HTTP/2)     │      │
│  │ (.onion)     │  │ Noise Proto  │  │ (TLS 1.3)    │      │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘      │
├─────────┴──────────────────┴──────────────────┴──────────────┤
│  Layer 2: Encryption (Always On)                            │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │ Tor Circuits │  │ Noise XX     │  │ TLS 1.3      │      │
│  │ (3-hop mix)  │  │ (ChaCha20)   │  │ (AES-GCM)    │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
├───────────────────────────────────────────────────────────────┤
│  Layer 1: Physical Network                                   │
│  Internet: WiFi, 4G/5G, Ethernet, Satellite, LoRa          │
└──────────────────────────────────────────────────────────────┘
```

---

## 🧅 Tor Integration (Primary Network)

### Why Tor as Default?

1. **Zero Port Forwarding**: Works behind CGNAT, corporate firewalls
2. **IP Anonymity**: Physical location of nodes is hidden
3. **Censorship Proof**: Works in China, Iran, Russia (with bridges)
4. **Free Forever**: No infrastructure cost, no rate limits
5. **Battle Tested**: Used by millions daily (Tor Browser)

### Technical Stack

| Component | Library | Purpose |
|-----------|---------|---------|
| **Tor Client** | `arti` (Rust) | Pure Rust Tor implementation |
| **Onion Service** | Arti Hidden Services | v3 .onion addresses (56 chars) |
| **Bridges** | `obfs4` + `snowflake` | Bypass censorship |
| **Directory** | Built-in consensus | No custom infrastructure |

### Implementation

```rust
// crates/edge-hive-tunnel/src/tor.rs
use arti_client::{TorClient, config::TorClientConfigBuilder};
use tor_hsservice::{HsIdKeypair, OnionServiceConfig};

pub struct TorNode {
    client: TorClient<PreferredRuntime>,
    onion_address: String, // abc123xyz456.onion (v3)
    keypair: HsIdKeypair,  // Persistent identity
}

impl TorNode {
    pub async fn bootstrap() -> Result<Self> {
        // 1. Configurar cliente Tor
        let config = TorClientConfigBuilder::default()
            .storage_dir("~/.edge-hive/tor")
            .build()?;
        
        // 2. Bootstrap Tor network (descargar consensus)
        let client = TorClient::create_bootstrapped(config).await?;
        info!("✅ Tor bootstrapped ({} relays)", client.relay_count());
        
        // 3. Cargar o generar keypair de Onion Service
        let keypair = HsIdKeypair::load_or_generate("~/.edge-hive/onion.key")?;
        
        // 4. Crear Onion Service v3
        let onion_config = OnionServiceConfig::builder()
            .nickname("edge-hive-node")
            .port(80, SocketAddr::from(([127, 0, 0, 1], 8080))) // .onion:80 → localhost:8080
            .keypair(keypair.clone())
            .build()?;
        
        let onion_service = client.launch_onion_service(onion_config).await?;
        let onion_addr = onion_service.onion_name().to_string();
        
        info!("🧅 Node accessible at: http://{}.onion", onion_addr);
        
        Ok(Self {
            client,
            onion_address: onion_addr,
            keypair,
        })
    }
    
    /// Conectar a otro nodo via Tor
    pub async fn connect(&self, peer_onion: &str) -> Result<TcpStream> {
        let stream = self.client
            .connect((peer_onion, 80))
            .await?;
        
        info!("🔗 Connected to {} via Tor", peer_onion);
        Ok(stream.into_std()?)
    }
}
```

### Tor Discovery via DHT

```rust
// Publicar dirección .onion en Kademlia DHT
pub async fn announce_to_dht(&self, dht: &mut KademliaDHT) -> Result<()> {
    let node_id = self.identity.public_key_hash(); // Ed25519 hash
    let record = DHTRecord {
        node_id,
        onion_address: self.tor.onion_address.clone(),
        last_seen: Utc::now(),
    };
    
    dht.put_value(node_id, serde_json::to_vec(&record)?).await?;
    info!("📢 Announced {} → {}.onion to DHT", node_id, self.tor.onion_address);
    Ok(())
}

// Buscar peer en DHT y conectar via Tor
pub async fn connect_to_peer(&self, peer_id: &str) -> Result<TcpStream> {
    // 1. Query DHT por node_id
    let record: DHTRecord = self.dht.get_value(peer_id).await?;
    
    // 2. Conectar via Tor usando .onion del peer
    self.tor.connect(&record.onion_address).await
}
```

### Tor Performance Optimizations

| Optimization | Improvement | Implementation |
|--------------|-------------|----------------|
| **Onion Service v3** | 2x faster than v2 | Default in Arti |
| **Circuit Preemption** | -50ms latency | Pre-build circuits |
| **Connection Pooling** | Reuse circuits | `tower` middleware |
| **Vanguards** | Protect against attacks | Arti config |

### Censorship Resistance (Bridges)

```rust
// Configurar bridges si Tor está bloqueado
let config = TorClientConfigBuilder::default()
    .bridge("obfs4 192.0.2.3:1234 FINGERPRINT cert=...")
    .bridge("snowflake ...")
    .build()?;
```

**User Experience:**
1. App detecta bloqueo de Tor (timeout en consensus)
2. Muestra QR code con bridge config
3. Usuario escanea con otro dispositivo en país libre
4. Config se sincroniza → Tor funciona

---

## ⚡ libp2p Integration (Fast Path)

### When to Use libp2p?

| Scenario | Network | Reason |
|----------|---------|--------|
| **Same LAN** (WiFi) | libp2p mDNS | <1ms latency vs. 200ms Tor |
| **Large file sync** (GB) | libp2p QUIC | Parallel streams, no Tor bandwidth limit |
| **Known trusted peers** | libp2p direct | No anonymity needed |
| **Public IP available** | libp2p Kademlia | Faster than Tor circuits |

### libp2p Stack

| Component | Purpose |
|-----------|---------|
| **mDNS** | Local network discovery (Bonjour-like) |
| **Kademlia DHT** | Global peer discovery |
| **QUIC** | Fast UDP transport (HTTP/3) |
| **Noise Protocol** | Encryption (ChaCha20-Poly1305) |
| **Yamux** | Stream multiplexing |

### Implementation

```rust
// crates/edge-hive-discovery/src/libp2p.rs
use libp2p::{
    identity, mdns, noise, quic, tcp, yamux,
    kad::{Kademlia, KademliaConfig, store::MemoryStore},
    swarm::{NetworkBehaviour, Swarm, SwarmBuilder},
};

#[derive(NetworkBehaviour)]
struct EdgeHiveBehaviour {
    mdns: mdns::tokio::Behaviour,
    kademlia: Kademlia<MemoryStore>,
}

pub struct LibP2PNode {
    swarm: Swarm<EdgeHiveBehaviour>,
}

impl LibP2PNode {
    pub async fn start(identity: identity::Keypair) -> Result<Self> {
        // 1. Crear transporte QUIC (UDP)
        let transport = quic::tokio::Transport::new(quic::Config::new(&identity));
        
        // 2. Configurar mDNS (local discovery)
        let mdns = mdns::tokio::Behaviour::new(
            mdns::Config::default(),
            identity.public().to_peer_id()
        )?;
        
        // 3. Configurar Kademlia DHT
        let store = MemoryStore::new(identity.public().to_peer_id());
        let mut kademlia = Kademlia::new(identity.public().to_peer_id(), store);
        kademlia.set_mode(Some(kad::Mode::Server)); // Siempre ser DHT server
        
        // 4. Bootstrap DHT con nodos conocidos
        kademlia.add_address(
            &"/ip4/104.131.131.82/tcp/4001/p2p/QmaCpDMGvV2BGHeYERUEnRQAwe3N8SzbUtfsmvsqQLuvuJ".parse()?,
            "/dnsaddr/bootstrap.libp2p.io".parse()?
        );
        
        // 5. Crear Swarm
        let behaviour = EdgeHiveBehaviour { mdns, kademlia };
        let mut swarm = SwarmBuilder::with_tokio_executor(transport, behaviour, identity.public().to_peer_id())
            .build();
        
        swarm.listen_on("/ip4/0.0.0.0/udp/0/quic-v1".parse()?)?;
        
        info!("⚡ libp2p listening on QUIC");
        Ok(Self { swarm })
    }
}
```

### Hybrid Connection Strategy

```rust
/// Auto-select fastest route: libp2p (fast) vs. Tor (secure)
pub async fn smart_connect(&self, peer_id: &str) -> Result<Stream> {
    // Race: primer winner se usa
    tokio::select! {
        // Opción 1: libp2p (si peer está en LAN o tiene IP pública)
        stream = self.libp2p.dial(peer_id) => {
            info!("⚡ Connected via libp2p (fast path)");
            stream
        }
        
        // Opción 2: Tor (siempre funciona, más lento)
        stream = self.tor.connect_onion(peer_id) => {
            info!("🧅 Connected via Tor (secure path)");
            stream
        }
        
        // Timeout después de 5 segundos
        _ = tokio::time::sleep(Duration::from_secs(5)) => {
            Err("Connection timeout")
        }
    }
}
```

---

## 🌍 Cloudflare Tunnel (Optional Public Access)

### Use Case

**Problema:** Quieres exponer tu dashboard en `https://my-node.example.com`  
**Solución:** Cloudflare Tunnel (antes Argo Tunnel)

### When to Enable?

| Service | Recommendation |
|---------|----------------|
| **Admin Panel** | ❌ Never (use Tor only) |
| **Public Blog** | ✅ Yes (SEO, fast) |
| **API for Apps** | ⚠️ Maybe (consider rate limits) |
| **File Sharing** | ❌ No (bandwidth cost) |

### Implementation

```rust
// crates/edge-hive-tunnel/src/cloudflare.rs
use std::process::{Command, Stdio};

pub struct CloudflareTunnel {
    process: Child,
    public_url: String,
}

impl CloudflareTunnel {
    pub async fn start(domain: &str) -> Result<Self> {
        // 1. Verificar que cloudflared esté instalado
        if !Command::new("cloudflared").arg("--version").status()?.success() {
            return Err("cloudflared not installed. Run: brew install cloudflared");
        }
        
        // 2. Crear tunnel
        let output = Command::new("cloudflared")
            .args(&["tunnel", "create", "edge-hive-node"])
            .output()?;
        
        let tunnel_id = String::from_utf8(output.stdout)?
            .lines()
            .find(|l| l.contains("Created tunnel"))
            .ok_or("Failed to create tunnel")?
            .split_whitespace()
            .last()
            .unwrap();
        
        // 3. Configurar DNS
        Command::new("cloudflared")
            .args(&["tunnel", "route", "dns", tunnel_id, domain])
            .status()?;
        
        // 4. Iniciar tunnel
        let process = Command::new("cloudflared")
            .args(&[
                "tunnel", "run",
                "--url", "http://localhost:8080",
                tunnel_id
            ])
            .stdout(Stdio::null())
            .spawn()?;
        
        info!("☁️  Cloudflare Tunnel: https://{}", domain);
        
        Ok(Self {
            process,
            public_url: format!("https://{}", domain),
        })
    }
}
```

### Security Considerations

```rust
// NUNCA exponer rutas sensibles via CF Tunnel
pub fn is_safe_to_expose(path: &str) -> bool {
    !path.starts_with("/admin") &&
    !path.starts_with("/api/internal") &&
    !path.starts_with("/.well-known/edge-hive")
}
```

---

## 🚀 Deployment Scenarios

### Scenario 1: Android Phone (Termux)

```bash
# Instalar en Termux
pkg install rust
cargo install edge-hive

# Arrancar nodo
edge-hive start --network tor,libp2p
# Output:
# 🧅 Tor: http://abc123xyz.onion
# ⚡ libp2p: /ip4/192.168.1.100/tcp/4001/p2p/QmXYZ...
```

**Connectivity:**
- ✅ Tor: Accessible desde cualquier lugar
- ✅ libp2p: Detectable en LAN via mDNS
- ❌ CF Tunnel: No recomendado (batería)

### Scenario 2: Home PC (Linux)

```bash
# Docker
docker run -d \
  -v ~/.edge-hive:/data \
  -p 8080:8080 \
  edgehive/node:latest

# Binary
curl -sSL https://edge-hive.dev/install.sh | sh
edge-hive start --network all
```

**Connectivity:**
- ✅ Tor: Siempre disponible
- ✅ libp2p: Con IP pública o UPnP
- ✅ CF Tunnel: Para servicios públicos

### Scenario 3: VPS (Hetzner, DigitalOcean)

```bash
# Single command deploy
curl -sSL https://edge-hive.dev/install.sh | sh
edge-hive start --public --domain my-hive.example.com

# Expone:
# - Tor: abc.onion (privado)
# - libp2p: [IP]:4001 (P2P)
# - HTTPS: my-hive.example.com (público)
```

### Scenario 4: Multi-Node Swarm

```
┌────────────────────────────────────────────────────────────┐
│                    User's Hive                             │
├────────────────────────────────────────────────────────────┤
│                                                            │
│  📱 Android (Termux)     💻 Home PC (Docker)              │
│  ├─ Tor: abc.onion      ├─ Tor: def.onion                │
│  ├─ libp2p (WiFi)       ├─ libp2p (LAN + public IP)      │
│  └─ Storage: 32GB       └─ Storage: 500GB                 │
│                                                            │
│  ☁️  AWS t4g.micro       🏠 Raspberry Pi 4                │
│  ├─ Tor: ghi.onion      ├─ Tor: jkl.onion                │
│  ├─ CF: api.x.com       ├─ libp2p (LAN only)             │
│  └─ Storage: 20GB       └─ Storage: 128GB                 │
│                                                            │
│  Total Storage: 680GB (replicated 2x = 340GB usable)      │
│  Access: Any node via Tor, fast sync via libp2p LAN       │
└────────────────────────────────────────────────────────────┘
```

---

## 📊 Performance Benchmarks

### Latency (Ping)

| Connection Type | Latency | Use Case |
|-----------------|---------|----------|
| libp2p mDNS (LAN) | 0.5ms | File sync same WiFi |
| libp2p QUIC (direct) | 20ms | Known peer, public IP |
| Tor (3-hop) | 200ms | Anonymous messaging |
| CF Tunnel | 30ms | Public API |

### Throughput (File Transfer)

| Connection Type | Speed | Limit |
|-----------------|-------|-------|
| libp2p (LAN) | 1 Gbps | Network card |
| libp2p (internet) | 500 Mbps | ISP |
| Tor | 5 Mbps | Tor network |
| CF Tunnel | 100 Mbps | CF free tier |

**Recommendation:** Use libp2p for large files, Tor for privacy-critical data.

---

## 🔐 Security Model

### Network Isolation

```rust
// Cada red tiene permisos diferentes
pub enum NetworkZone {
    Tor,      // Full access (anonymous)
    LibP2P,   // Trusted peers only
    CF,       // Public, rate-limited
}

impl AccessControl {
    pub fn check(&self, path: &str, zone: NetworkZone) -> bool {
        match zone {
            NetworkZone::Tor => true, // All endpoints
            NetworkZone::LibP2P => !path.starts_with("/admin"),
            NetworkZone::CF => self.public_routes.contains(path),
        }
    }
}
```

### End-to-End Encryption

```
┌────────────────────────────────────────────────┐
│  Data at Rest: AES-256-GCM (SurrealDB)        │
├────────────────────────────────────────────────┤
│  Transport:                                    │
│  ├─ Tor: ChaCha20 (3 layers)                  │
│  ├─ libp2p: Noise XX (ChaCha20-Poly1305)     │
│  └─ CF: TLS 1.3 (AES-GCM)                     │
└────────────────────────────────────────────────┘
```

---

## 🎯 Summary

### Network Selection Matrix

| Your Priority | Recommended Stack |
|---------------|-------------------|
| **Maximum privacy** | Tor only |
| **Best performance** | libp2p (LAN) + Tor (WAN) |
| **Public service** | CF Tunnel (web) + Tor (admin) |
| **Offline-first** | libp2p mDNS only |
| **Censorship bypass** | Tor + bridges |

### Next Steps

1. ✅ Implement Tor integration (`arti` crate)
2. ✅ Implement libp2p discovery (mDNS + Kademlia)
3. ⚠️ Add hybrid connection strategy
4. 🔄 Cloudflare Tunnel (optional, Phase 2)

---

**Questions? See:** `docs/agent-docs/NETWORK_FAQ.md`
