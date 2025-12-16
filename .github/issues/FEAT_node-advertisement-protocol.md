---
title: "Mejorar Discovery Protocol: Node Advertisement y Capabilities"
labels:
  - enhancement
  - discovery
  - p2p
  - networking
assignees: []
---

## 🎯 Objetivo

Mejorar el protocolo de descubrimiento P2P para que los nodos anuncien sus capacidades, endpoints MCP, y metadata de forma robusta en redes locales y globales.

## 📋 Problema Actual

El discovery actual con libp2p (mDNS + Kademlia) solo descubre peer IDs y direcciones, pero no comunica:

- ✅ Qué servicios ofrece el nodo (MCP, API, plugins)
- ✅ Endpoints HTTPS para AI agents
- ✅ Capacidades (tools disponibles, recursos, versión MCP)
- ✅ Metadata (nombre, versión, uptime, load)
- ✅ Métodos de autenticación soportados

## 🏗️ Propuesta: Node Advertisement Protocol

### Advertisement Message Structure

```json
{
  "node_id": "swift-fox-0dd9b9",
  "peer_id": "12D3KooWAMv3tyFoXG38hteiVk9ugVVh8pp63BmY8q3Wz7oLDqJi",
  "version": "0.1.0",
  "capabilities": {
    "mcp": {
      "enabled": true,
      "version": "2024-11-05",
      "https_endpoint": "https://192.168.1.100:8080/mcp",
      "tools": [
        {"name": "get_status", "description": "Get node status"},
        {"name": "provision_node", "description": "Provision cloud node"}
      ],
      "resources": [
        {"uri": "edge-hive://logs/*", "type": "text/plain"}
      ]
    },
    "p2p": {
      "protocols": ["/libp2p/kad/1.0.0", "/libp2p/noise/1.0.0"],
      "listen_addrs": [
        "/ip4/192.168.1.100/tcp/4001",
        "/ip4/192.168.1.100/udp/4001/quic"
      ]
    },
    "api": {
      "http_endpoint": "https://192.168.1.100:8080",
      "websocket_endpoint": "wss://192.168.1.100:8080/ws"
    }
  },
  "metadata": {
    "name": "swift-fox-0dd9b9",
    "platform": "linux-x86_64",
    "uptime_seconds": 3600,
    "load": {
      "cpu_percent": 45,
      "memory_percent": 60,
      "disk_percent": 30
    }
  },
  "authentication": {
    "oauth2": {
      "enabled": true,
      "token_endpoint": "/mcp/auth/token",
      "flows": ["client_credentials"]
    },
    "p2p": {
      "method": "ed25519-signature"
    }
  },
  "timestamp": "2025-12-15T22:00:00Z",
  "signature": "..."
}
```

## 📝 Tareas de Implementación

### Fase 1: Advertisement Data Structure

- [ ] Crear struct `NodeAdvertisement` en `edge-hive-discovery`
  - [ ] Incluir capabilities, metadata, authentication
  - [ ] Serializable a JSON
  - [ ] Firma Ed25519 para autenticidad
- [ ] Agregar método `DiscoveryService::advertise()`
  - [ ] Generar advertisement del nodo local
  - [ ] Actualizar cada 30 segundos
  - [ ] Incrementar timestamp

### Fase 2: mDNS Enhancement

- [ ] Usar TXT records de mDNS para metadata básica

  ```rust
  use mdns_sd::{ServiceDaemon, ServiceInfo};

  let properties = vec![
      ("node_id", "swift-fox-0dd9b9"),
      ("version", "0.1.0"),
      ("mcp_enabled", "true"),
      ("mcp_endpoint", "https://192.168.1.100:8080/mcp"),
      ("oauth2_enabled", "true"),
  ];
  ```

- [ ] Escuchar anuncios mDNS de otros nodos
- [ ] Parsear TXT records y actualizar peer info

### Fase 3: Kademlia DHT Enhancement

- [ ] Publicar advertisement en Kademlia DHT
  - [ ] Key: `node-advertisement:<peer_id>`
  - [ ] Value: Signed JSON advertisement
- [ ] Query advertisements de peers conocidos
- [ ] Cache local de advertisements recientes

### Fase 4: HTTP Discovery Endpoint

- [ ] Endpoint público para obtener advertisement:

  ```
  GET /discovery/announce
  Response: NodeAdvertisement (JSON)
  ```

- [ ] Endpoint para listar peers descubiertos:

  ```
  GET /discovery/peers
  Response: { "peers": [NodeAdvertisement] }
  ```

- [ ] Endpoint para solicitar conexión P2P:

  ```
  POST /discovery/connect/{peer_id}
  Body: { "protocol": "libp2p" }
  ```

### Fase 5: Signature Verification

- [ ] Firmar advertisements con Ed25519 node identity
- [ ] Verificar firma antes de confiar en advertisement
- [ ] Rechazar advertisements con firmas inválidas
- [ ] Trust score basado en firmas válidas

### Fase 6: CLI Commands

- [ ] `edge-hive discovery announce` - Anunciar nodo
- [ ] `edge-hive discovery peers` - Listar peers con capabilities
- [ ] `edge-hive discovery info <peer_id>` - Info detallada de peer
- [ ] `edge-hive discovery connect <peer_id>` - Conectar a peer

## 🧪 Testing Strategy

### Test 1: Local mDNS Discovery

```bash
# Terminal 1: Node 1
edge-hive serve --discovery --port 8080

# Terminal 2: Node 2
edge-hive serve --discovery --port 8081

# Terminal 3: Verificar discovery
edge-hive discovery peers
# Output esperado: Lista con node-1 mostrando capabilities
```

### Test 2: Advertisement Integrity

```bash
# Obtener advertisement
curl http://localhost:8080/discovery/announce | jq

# Verificar firma
edge-hive discovery verify <peer_id>
```

### Test 3: Docker Multi-Node

```yaml
# docker-compose.yml
services:
  node-1:
    build: .
    ports: ["8080:8080"]
    networks: ["edge-hive"]

  node-2:
    build: .
    ports: ["8081:8080"]
    networks: ["edge-hive"]

  node-3:
    build: .
    ports: ["8082:8080"]
    networks: ["edge-hive"]
```

```bash
docker-compose up -d
docker exec edge-hive-node-1 edge-hive discovery peers
# Debería mostrar node-2 y node-3 con sus capabilities
```

## 🔐 Security Considerations

1. **Signature Verification**: Todos los advertisements deben estar firmados
2. **Rate Limiting**: Limitar frecuencia de actualizaciones (max 1/30s)
3. **Size Limits**: Advertisement máximo 10KB para evitar spam
4. **TTL**: Expirar advertisements después de 5 minutos sin refresh
5. **Blocklist**: Permitir bloquear peer_ids maliciosos

## 📦 Dependencies

```toml
# Agregar al workspace Cargo.toml
mdns-sd = "0.10"  # Para mDNS TXT records
serde_json = "1"  # Ya existe, usar para JSON
```

## 📚 Referencias

- [libp2p Kademlia Spec](https://github.com/libp2p/specs/blob/master/kad-dht/README.md)
- [mDNS RFC 6763](https://datatracker.ietf.org/doc/html/rfc6763)
- [Service Discovery](https://en.wikipedia.org/wiki/Zero-configuration_networking)
- Análisis completo: `docs/agent-docs/ANALYSIS_SECURE_MCP_AUTHENTICATION.md#node-discovery-advertisement`

## ✅ Definition of Done

- [ ] Nodos anuncian capabilities vía mDNS y Kademlia DHT
- [ ] Advertisements incluyen endpoints MCP/HTTPS
- [ ] Firmas Ed25519 verificadas correctamente
- [ ] Endpoints HTTP de discovery funcionan
- [ ] CLI commands implementados
- [ ] Tests de integración multi-nodo pasan
- [ ] Documentación completa
