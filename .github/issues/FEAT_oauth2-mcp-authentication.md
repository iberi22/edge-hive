---
title: "Implementar OAuth2 Authentication para MCP sobre HTTPS"
labels:
  - enhancement
  - security
  - mcp
  - authentication
assignees: []
---

## 🎯 Objetivo

Implementar sistema de autenticación OAuth2 + JWT para permitir que agentes AI (Claude, GPT, Jules) se conecten de forma segura a nodos Edge Hive vía MCP sobre HTTPS.

## 📋 Context

Inspirado en la implementación de Supabase MCP (`https://mcp.supabase.com/mcp/project_ref={id}`), necesitamos un sistema de autenticación robusto que permita:

1. **AI Agents** conectarse vía OAuth2 Client Credentials
2. **Nodos P2P** autenticarse vía Ed25519 (ya implementado)
3. **Token-based access** con JWT
4. **Scope-based permissions** (mcp:read, mcp:call, mcp:resources)

## 🏗️ Arquitectura Propuesta

```
┌─────────────┐   HTTPS + Bearer Token   ┌──────────────────┐
│  AI Agent   │◄─────────────────────────►│  Edge Hive Node  │
│ (Claude)    │                           │  (Axum + rustls) │
└─────────────┘                           └──────────────────┘
                                                    │
                                                    ▼
                                          ┌──────────────────┐
                                          │  SurrealDB       │
                                          │  (client store)  │
                                          └──────────────────┘
```

## 📝 Tareas de Implementación

### Fase 1: OAuth2 Infrastructure

- [ ] Crear módulo `edge-hive-auth` con:
  - [ ] JWT signing/validation (usando `jsonwebtoken` crate)
  - [ ] Token endpoint (`POST /mcp/auth/token`)
  - [ ] Client credentials store (SurrealDB table)
  - [ ] Token refresh logic
- [ ] Middleware de autenticación para Axum
  - [ ] Extraer Bearer token del header `Authorization`
  - [ ] Validar JWT signature y expiration
  - [ ] Verificar scopes requeridos
- [ ] Rate limiting por client_id
  - [ ] 100 requests/min por client
  - [ ] Usar `tower::limit::RateLimitLayer`

### Fase 2: MCP HTTPS Endpoints

- [ ] Agregar autenticación a endpoints existentes:
  - [ ] `POST /mcp/tools/call` (requiere scope `mcp:call`)
  - [ ] `GET /mcp/resources/{uri}` (requiere scope `mcp:read`)
  - [ ] `GET /mcp/stream` (SSE, requiere scope `mcp:subscribe`)
- [ ] Endpoint de información pública:
  - [ ] `GET /mcp/node/{node_id}/info` (sin autenticación)
- [ ] Error responses estándar:
  - [ ] 401 Unauthorized (token inválido)
  - [ ] 403 Forbidden (scope insuficiente)
  - [ ] 429 Too Many Requests (rate limit)

### Fase 3: Client Management CLI

- [ ] Comandos para gestionar clients OAuth2:
  ```bash
  edge-hive auth client create --name "claude-opus" --scopes "mcp:read mcp:call"
  edge-hive auth client list
  edge-hive auth client revoke <client_id>
  ```
- [ ] Generar client_id y client_secret seguros
- [ ] Almacenar en SurrealDB con hash del secret

### Fase 4: TLS/HTTPS Setup

- [ ] Integrar `rustls` para HTTPS
- [ ] Auto-generar certificados self-signed para desarrollo
- [ ] Soporte para Let's Encrypt (opcional)
- [ ] TLS 1.3 only, modern ciphers

### Fase 5: Testing & Documentation

- [ ] Integration tests:
  - [ ] OAuth2 flow completo
  - [ ] Token validation
  - [ ] Scope enforcement
  - [ ] Rate limiting
- [ ] Documentación:
  - [ ] Guía de autenticación para AI agents
  - [ ] Ejemplos de curl y SDKs
  - [ ] Troubleshooting

## 🔐 JWT Claims Structure

```json
{
  "sub": "client-claude-opus-4",
  "aud": "node-swift-fox-0dd9b9",
  "iss": "https://edge-hive.io",
  "exp": 1702915200,
  "iat": 1702828800,
  "scopes": ["mcp:read", "mcp:call"],
  "client_type": "ai-agent",
  "client_name": "Claude Opus 4"
}
```

## 📦 Dependencies a Agregar

```toml
# Cargo.toml workspace dependencies
jsonwebtoken = "9"
rustls = "0.23"
tower-http = { version = "0.5", features = ["auth", "cors"] }
tower = { version = "0.5", features = ["limit"] }
base64 = "0.22"
sha2 = "0.10"
```

## 🧪 Testing Flow

```bash
# 1. Crear client
edge-hive auth client create --name test-agent
# Output: client_id=xxx client_secret=yyy

# 2. Obtener token
curl -X POST http://localhost:8080/mcp/auth/token \
  -d '{"grant_type":"client_credentials","client_id":"xxx","client_secret":"yyy"}'
# Output: {"access_token":"eyJ...","expires_in":3600}

# 3. Usar token
curl http://localhost:8080/mcp/tools/call \
  -H "Authorization: Bearer eyJ..." \
  -d '{"name":"get_status","arguments":{}}'
```

## 📚 Referencias

- [OAuth 2.0 RFC 6749](https://datatracker.ietf.org/doc/html/rfc6749)
- [JWT Best Practices RFC 8725](https://datatracker.ietf.org/doc/html/rfc8725)
- [Supabase MCP Implementation](https://supabase.com/docs/guides/functions/examples/mcp-server-mcp-lite)
- Análisis completo: `docs/agent-docs/ANALYSIS_SECURE_MCP_AUTHENTICATION.md`

## ✅ Definition of Done

- [ ] AI agents pueden obtener JWT token vía OAuth2
- [ ] MCP endpoints validan Bearer tokens correctamente
- [ ] Scopes se enforcement correctamente
- [ ] Rate limiting funciona
- [ ] TLS/HTTPS habilitado
- [ ] Tests de integración pasan
- [ ] Documentación completa
