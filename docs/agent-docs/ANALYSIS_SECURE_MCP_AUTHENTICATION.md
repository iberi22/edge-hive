---
title: "Análisis: MCP sobre HTTPS con Autenticación Segura para Agentes AI"
type: ANALYSIS
id: "analysis-mcp-auth"
created: 2025-12-15
agent: copilot
model: claude-sonnet-4
requested_by: user
summary: |
  Diseño de sistema de autenticación seguro para agentes AI que se conectan
  a nodos Edge Hive vía MCP sobre HTTPS, inspirado en la implementación de Supabase.
keywords: [mcp, https, oauth2, authentication, security, agents, ai]
tags: ["#mcp", "#security", "#oauth2", "#architecture"]
topics: [mcp-server, authentication, p2p-discovery]
priority: high
status: proposal
---

# 🔐 MCP sobre HTTPS con Autenticación Segura para Agentes AI

> **Objetivo:** Permitir que agentes AI (Claude, GPT, Jules) se conecten de forma segura a nodos Edge Hive usando MCP sobre HTTPS con autenticación OAuth2/JWT.

---

## 📊 Análisis de la Implementación de Supabase

### ¿Cómo Supabase implementó MCP sobre HTTPS?

Basándome en la captura compartida y la investigación:

```
URL: https://mcp.supabase.com/mcp/project_ref=tzmrgvtpdtsjcuqewyq
```

**Arquitectura de Supabase MCP:**

```
┌─────────────────┐      HTTPS (TLS 1.3)     ┌──────────────────────┐
│  AI Agent       │◄────────────────────────►│  Supabase MCP Server │
│  (Claude/GPT)   │   OAuth2 Bearer Token    │  (edge functions)    │
└─────────────────┘                          └──────────────────────┘
                                                        │
                                                        ▼
                                              ┌──────────────────┐
                                              │  Postgres DB     │
                                              │  + Auth          │
                                              └──────────────────┘
```

**Componentes clave:**

1. **HTTPS Endpoint**: `https://mcp.supabase.com/mcp/project_ref={id}`
2. **Autenticación**: Bearer token (probablemente JWT)
3. **Transporte**: HTTP/2 + Server-Sent Events (SSE) para streaming
4. **Identificación**: `project_ref` como identificador del proyecto

---

## 🏗️ Propuesta de Arquitectura para Edge Hive

### Arquitectura Multi-Capa

```
┌─────────────────────────────────────────────────────────────────────┐
│                        EDGE HIVE NODE                               │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  ┌──────────────────┐      ┌──────────────────┐                   │
│  │  MCP/HTTPS       │      │  libp2p P2P      │                   │
│  │  (AI Agents)     │      │  (Node-to-Node)  │                   │
│  └────────┬─────────┘      └────────┬─────────┘                   │
│           │                         │                             │
│           │                         │                             │
│  ┌────────▼─────────────────────────▼─────────┐                   │
│  │         Authentication Layer               │                   │
│  │  • OAuth2 for AI Agents                    │                   │
│  │  • Ed25519 for P2P Nodes                   │                   │
│  └────────────────┬───────────────────────────┘                   │
│                   │                                                │
│  ┌────────────────▼───────────────────────────┐                   │
│  │         Core Edge Hive Service             │                   │
│  │  • Identity Management                     │                   │
│  │  • Resource Discovery                      │                   │
│  │  • State Management (SurrealDB)            │                   │
│  └────────────────────────────────────────────┘                   │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 🔑 Sistema de Autenticación Dual

### 1. Para AI Agents (MCP sobre HTTPS)

**OAuth2 + JWT Flow:**

```rust
// Pseudocódigo simplificado
async fn authenticate_agent(req: Request) -> Result<AgentSession> {
    // 1. Extraer Bearer token
    let token = req.headers().get("Authorization")?
        .strip_prefix("Bearer ")?;

    // 2. Validar JWT
    let claims = verify_jwt(token, &public_key)?;

    // 3. Verificar scopes
    ensure!(claims.scopes.contains("mcp:read"));

    // 4. Crear sesión
    Ok(AgentSession {
        agent_id: claims.sub,
        node_id: claims.aud,
        scopes: claims.scopes,
        expires_at: claims.exp,
    })
}
```

**JWT Claims Structure:**

```json
{
  "sub": "agent-claude-opus-4",
  "aud": "node-swift-fox-0dd9b9",
  "iss": "https://edge-hive.io",
  "exp": 1702915200,
  "iat": 1702828800,
  "scopes": ["mcp:read", "mcp:call", "mcp:resources"],
  "agent_type": "claude",
  "agent_version": "opus-4.0"
}
```

### 2. Para Nodos P2P (libp2p)

**Ed25519 Mutual Authentication:**

```rust
// Los nodos se autentican usando claves Ed25519
async fn authenticate_peer(peer_id: PeerId) -> Result<PeerSession> {
    // 1. Verificar firma Ed25519
    let public_key = peer_id.to_public_key()?;

    // 2. Challenge-response
    let challenge = generate_challenge();
    let signature = peer.sign(challenge).await?;

    public_key.verify(challenge, signature)?;

    // 3. Crear sesión P2P
    Ok(PeerSession {
        peer_id,
        capabilities: peer.advertise_capabilities(),
        trust_score: calculate_trust(peer_id),
    })
}
```

---

## 🌐 Endpoint Structure

### MCP HTTPS Endpoints

```
GET  /mcp/node/{node_id}/info
  → Información del nodo (capabilities, version)

POST /mcp/node/{node_id}/auth/token
  → Obtener JWT token (OAuth2 Client Credentials)

POST /mcp/node/{node_id}/tools/call
  → Ejecutar tool MCP (requiere Bearer token)

GET  /mcp/node/{node_id}/resources/{uri}
  → Obtener resource MCP

GET  /mcp/node/{node_id}/stream
  → SSE stream para notificaciones
```

### Discovery Endpoints (para redes locales)

```
GET  /discovery/announce
  → Anunciar presencia en red local

GET  /discovery/peers
  → Lista de peers descubiertos (mDNS + Kademlia)

POST /discovery/connect/{peer_id}
  → Solicitar conexión P2P
```

---

## 🐳 Docker Configuration

### Dockerfile Optimizado

```dockerfile
# Multi-stage build optimizado
FROM rust:1.85-alpine AS builder

# Dependencias de compilación
RUN apk add --no-cache \
    musl-dev openssl-dev openssl-libs-static pkgconfig

WORKDIR /build

# Cache de dependencias (layer caching)
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p crates/edge-hive-core/src && \
    echo "fn main() {}" > crates/edge-hive-core/src/main.rs
RUN cargo build --release --target x86_64-unknown-linux-musl

# Build real
COPY crates ./crates
RUN touch crates/edge-hive-core/src/main.rs && \
    cargo build --release --target x86_64-unknown-linux-musl

# Runtime stage: Alpine mínimo
FROM alpine:3.19

# Solo runtime essentials
RUN apk add --no-cache ca-certificates libgcc && \
    addgroup -g 1000 edgehive && \
    adduser -D -u 1000 -G edgehive edgehive

# Binary
COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/edge-hive /usr/local/bin/

# Data directory
RUN mkdir -p /data/.edge-hive && \
    chown -R edgehive:edgehive /data

USER edgehive
WORKDIR /data

# Puertos
EXPOSE 8080/tcp   # HTTPS MCP
EXPOSE 4001/udp   # libp2p QUIC
EXPOSE 5353/udp   # mDNS

# Healthcheck
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s \
  CMD ["/usr/local/bin/edge-hive", "status", "--json"]

ENTRYPOINT ["/usr/local/bin/edge-hive"]
CMD ["serve", "--port", "8080", "--discovery"]
```

### docker-compose.yml para Testing

```yaml
version: '3.8'

services:
  edge-hive-node-1:
    build: .
    container_name: edge-hive-node-1
    hostname: node-1
    ports:
      - "8080:8080"  # HTTPS MCP
      - "4001:4001/udp"  # libp2p
    environment:
      - EDGE_HIVE_NODE_NAME=node-1
      - EDGE_HIVE_MCP_ENABLED=true
      - EDGE_HIVE_OAUTH2_ISSUER=https://edge-hive.io
      - RUST_LOG=info,edge_hive=debug
    volumes:
      - node1-data:/data
    networks:
      - edge-hive-net

  edge-hive-node-2:
    build: .
    container_name: edge-hive-node-2
    hostname: node-2
    ports:
      - "8081:8080"
      - "4002:4001/udp"
    environment:
      - EDGE_HIVE_NODE_NAME=node-2
      - EDGE_HIVE_MCP_ENABLED=true
      - RUST_LOG=info
    volumes:
      - node2-data:/data
    networks:
      - edge-hive-net
    depends_on:
      - edge-hive-node-1

volumes:
  node1-data:
  node2-data:

networks:
  edge-hive-net:
    driver: bridge
    ipam:
      config:
        - subnet: 172.28.0.0/16
```

---

## 🔐 OAuth2 Implementation

### Grant Types Soportados

| Grant Type | Uso | Seguridad |
|------------|-----|-----------|
| **Client Credentials** | Agent-to-Node | ✅ Alta |
| **Authorization Code** | User consent (futuro) | ✅ Muy Alta |
| **Device Code** | CLI tools | ✅ Alta |

### Token Endpoint

```rust
#[derive(Deserialize)]
struct TokenRequest {
    grant_type: String,
    client_id: String,
    client_secret: String,
    scope: Option<String>,
}

async fn token_endpoint(
    State(state): State<AppState>,
    Json(req): Json<TokenRequest>,
) -> Result<Json<TokenResponse>, AuthError> {
    // Validar client credentials
    let client = state.validate_client(&req.client_id, &req.client_secret).await?;

    // Generar JWT
    let claims = Claims {
        sub: req.client_id,
        aud: state.node_id.clone(),
        iss: "https://edge-hive.io",
        exp: (Utc::now() + Duration::hours(1)).timestamp(),
        iat: Utc::now().timestamp(),
        scopes: parse_scopes(&req.scope.unwrap_or_default()),
    };

    let token = encode_jwt(&claims, &state.jwt_secret)?;

    Ok(Json(TokenResponse {
        access_token: token,
        token_type: "Bearer".into(),
        expires_in: 3600,
        scope: claims.scopes.join(" "),
    }))
}
```

---

## 📡 Node Discovery & Advertisement

### Advertisement Protocol

Cada nodo debe anunciar:

```json
{
  "node_id": "swift-fox-0dd9b9",
  "peer_id": "12D3KooWAMv3tyFoXG38hteiVk9ugVVh8pp63BmY8q3Wz7oLDqJi",
  "capabilities": {
    "mcp": {
      "version": "2024-11-05",
      "https_endpoint": "https://192.168.1.100:8080/mcp",
      "supported_tools": ["get_status", "provision_node"],
      "supported_resources": ["edge-hive://logs/*"]
    },
    "p2p": {
      "protocols": ["/libp2p/kad/1.0.0", "/libp2p/noise/1.0.0"],
      "listen_addrs": [
        "/ip4/192.168.1.100/tcp/4001",
        "/ip4/192.168.1.100/udp/4001/quic"
      ]
    }
  },
  "metadata": {
    "name": "swift-fox-0dd9b9",
    "version": "0.1.0",
    "platform": "linux-x86_64",
    "uptime": 3600,
    "load": {"cpu": 45, "memory": 60}
  },
  "authentication": {
    "oauth2_enabled": true,
    "token_endpoint": "/mcp/auth/token",
    "supported_flows": ["client_credentials"]
  }
}
```

### mDNS Advertisement

```rust
use mdns_sd::{ServiceDaemon, ServiceInfo};

async fn advertise_on_mdns(node_info: &NodeInfo) -> Result<()> {
    let mdns = ServiceDaemon::new()?;

    let service_type = "_edge-hive._tcp.local.";
    let instance_name = format!("{}.{}", node_info.node_id, service_type);

    let properties = vec![
        ("node_id", node_info.node_id.as_str()),
        ("version", "0.1.0"),
        ("mcp_enabled", "true"),
        ("oauth2_enabled", "true"),
    ];

    let service = ServiceInfo::new(
        service_type,
        &instance_name,
        &node_info.hostname,
        node_info.mcp_port,
        properties,
    )?;

    mdns.register(service)?;

    Ok(())
}
```

---

## 🛡️ Security Best Practices

### 1. TLS Configuration

```rust
use rustls::{ServerConfig, Certificate, PrivateKey};

fn create_tls_config() -> Result<ServerConfig> {
    let cert = load_certificate("certs/node.crt")?;
    let key = load_private_key("certs/node.key")?;

    let mut config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(vec![cert], key)?;

    // Modern ciphers only
    config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];

    Ok(config)
}
```

### 2. Rate Limiting

```rust
use tower::limit::RateLimitLayer;

let rate_limit = RateLimitLayer::new(
    100,  // 100 requests
    Duration::from_secs(60),  // per minute
);

app.layer(rate_limit)
```

### 3. CORS para AI Agents

```rust
use tower_http::cors::{CorsLayer, Any};

let cors = CorsLayer::new()
    .allow_origin(Any)  // Para development
    .allow_methods([Method::GET, Method::POST])
    .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE]);
```

---

## 🧪 Testing Strategy

### 1. Docker Compose Test

```bash
# Iniciar cluster de 3 nodos
docker-compose up -d

# Verificar discovery
docker exec edge-hive-node-1 edge-hive peers list

# Test MCP connection
curl https://localhost:8080/mcp/node/node-1/info
```

### 2. OAuth2 Flow Test

```bash
# Obtener token
TOKEN=$(curl -X POST https://localhost:8080/mcp/auth/token \
  -H "Content-Type: application/json" \
  -d '{
    "grant_type": "client_credentials",
    "client_id": "test-agent",
    "client_secret": "test-secret",
    "scope": "mcp:read mcp:call"
  }' | jq -r '.access_token')

# Usar token
curl https://localhost:8080/mcp/tools/call \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "get_status",
    "arguments": {}
  }'
```

### 3. P2P Discovery Test

```bash
# Node 1 anuncia
docker exec edge-hive-node-1 edge-hive discovery announce

# Node 2 descubre
docker exec edge-hive-node-2 edge-hive discovery peers

# Conectar
docker exec edge-hive-node-2 edge-hive peers connect <peer_id>
```

---

## 📋 Implementation Checklist

### Phase 1: OAuth2 Infrastructure (Week 1)

- [ ] Implementar token endpoint (`/mcp/auth/token`)
- [ ] JWT signing y validación
- [ ] Client credentials store (SurrealDB)
- [ ] Middleware de autenticación Bearer token
- [ ] Rate limiting

### Phase 2: MCP HTTPS Server (Week 2)

- [ ] HTTPS server con TLS 1.3
- [ ] MCP protocol handlers (tools, resources)
- [ ] SSE streaming para notificaciones
- [ ] CORS configuration
- [ ] Error handling

### Phase 3: Node Discovery Enhancement (Week 3)

- [ ] mDNS service advertisement
- [ ] Capability advertisement protocol
- [ ] Peer info API (`/discovery/peers`)
- [ ] Health checks
- [ ] Metadata refresh

### Phase 4: Docker & Testing (Week 4)

- [ ] Dockerfile optimizado (<20MB)
- [ ] docker-compose multi-node setup
- [ ] Integration tests (OAuth + MCP + P2P)
- [ ] Performance benchmarks
- [ ] Documentation

---

## 🚀 Next Steps

1. **Crear issue para OAuth2 implementation**
2. **Prototipo de MCP/HTTPS server** (Axum + rustls)
3. **Docker multi-stage build** refinado
4. **Integration tests** con 3 nodos locales
5. **VS Code extension** para conectar a nodos

---

## 📚 Referencias

- [MCP Specification](https://spec.modelcontextprotocol.io/)
- [OAuth 2.0 RFC 6749](https://datatracker.ietf.org/doc/html/rfc6749)
- [libp2p Specs](https://github.com/libp2p/specs)
- [Supabase MCP Example](https://supabase.com/docs/guides/functions/examples/mcp-server-mcp-lite)
- [JWT Best Practices](https://datatracker.ietf.org/doc/html/rfc8725)
