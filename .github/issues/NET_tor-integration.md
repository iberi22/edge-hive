---
title: "[NET] Tor Onion Services Integration (Arti)"
labels:
  - networking
  - tor
  - privacy
  - priority-high
assignees:
  - jules
---

## User Story

**As a** node operator  
**I want** my node to be accessible via Tor onion address  
**So that** I can connect from anywhere without port forwarding or exposing my IP

## Technical Specs

### Crate: `edge-hive-tunnel`

```toml
[dependencies]
arti-client = "0.20"              # Pure Rust Tor client
tor-hsservice = "0.20"            # Onion Services v3
tor-rtcompat = { version = "0.20", features = ["tokio"] }
anyhow = "1"
tracing = "0.1"
```

### Key Components

#### 1. Tor Client Bootstrap

```rust
// crates/edge-hive-tunnel/src/tor/client.rs
use arti_client::{TorClient, config::TorClientConfigBuilder};

pub struct TorNode {
    client: TorClient<PreferredRuntime>,
    onion_address: String,
}

impl TorNode {
    pub async fn bootstrap(data_dir: &Path) -> Result<Self> {
        // 1. Configurar storage
        let config = TorClientConfigBuilder::default()
            .storage_dir(data_dir.join("tor"))
            .build()?;
        
        // 2. Bootstrap Tor network
        let client = TorClient::create_bootstrapped(config).await?;
        
        // 3. Load or create onion service
        let onion_service = OnionServiceBuilder::new()
            .nickname("edge-hive-node")
            .port(80, SocketAddr::from(([127, 0, 0, 1], 8080)))
            .build()?;
        
        let service = client.launch_onion_service(onion_service).await?;
        let onion_addr = service.onion_name().to_string();
        
        info!("🧅 Onion address: http://{}.onion", onion_addr);
        
        Ok(Self {
            client,
            onion_address: onion_addr,
        })
    }
}
```

#### 2. Persistent Onion Identity

```rust
// Guardar keypair para mantener .onion address constante
use tor_hsservice::HsIdKeypair;

pub fn load_or_generate_identity(path: &Path) -> Result<HsIdKeypair> {
    if path.exists() {
        let bytes = std::fs::read(path)?;
        HsIdKeypair::from_bytes(&bytes)
    } else {
        let keypair = HsIdKeypair::generate(&mut rand::thread_rng());
        std::fs::write(path, keypair.to_bytes())?;
        Ok(keypair)
    }
}
```

#### 3. Connect to Peer via Tor

```rust
pub async fn connect_to_peer(&self, peer_onion: &str) -> Result<TcpStream> {
    let stream = self.client
        .connect((peer_onion, 80))
        .await?;
    
    info!("🔗 Connected to {} via Tor", peer_onion);
    Ok(stream.into_std()?)
}
```

### Integration with DHT

```rust
// Publicar .onion address en Kademlia DHT
pub async fn announce_onion_to_dht(&self, dht: &mut KademliaDHT) -> Result<()> {
    let node_id = self.identity.public_key_hash();
    let record = NodeRecord {
        node_id,
        onion_address: self.tor.onion_address.clone(),
        libp2p_addrs: self.libp2p.listen_addrs(),
        last_seen: Utc::now(),
    };
    
    dht.put_value(node_id, serde_json::to_vec(&record)?).await?;
    Ok(())
}
```

## Acceptance Criteria

- [ ] Tor client bootstraps on first run
- [ ] Onion service v3 created with persistent keypair
- [ ] Node accessible via `.onion` address
- [ ] Can connect to other nodes via their `.onion`
- [ ] Onion address published to Kademlia DHT
- [ ] Works on Android (Termux), Linux, Windows
- [ ] Logs show bootstrap progress

## Testing

```bash
# Unit tests
cargo test -p edge-hive-tunnel tor::

# Integration test
edge-hive start --network tor-only
# Output: 🧅 Onion address: http://abc123xyz.onion

# Conectar desde otro nodo
curl http://abc123xyz.onion/health
# {"status": "ok"}
```

## Dependencies

- Issue #7: NET_identity-system (Ed25519 keypairs)
- Issue #8: NET_node-discovery (Kademlia DHT)

## References

- [Arti Documentation](https://docs.rs/arti-client/)
- [Tor Onion Services v3](https://community.torproject.org/onion-services/)
- [NETWORK_ARCHITECTURE.md](../.✨/NETWORK_ARCHITECTURE.md)

## Performance Goals

| Metric | Target |
|--------|--------|
| Bootstrap time | < 30 seconds |
| Onion creation | < 5 seconds |
| Connection latency | < 300ms (3-hop) |
| Memory usage | < 50MB (Arti client) |

## Security Notes

- ✅ Tor circuits rotated every 10 minutes
- ✅ Onion keypair stored with 0600 permissions
- ✅ No DNS leaks (pure onion routing)
- ⚠️ Admin panel NEVER exposed via .onion (use local-only)
