# OAuth2 + JWT Authentication System - Quick Start

Sistema de autenticación OAuth2 con JWT implementado para Edge Hive MCP (Model Context Protocol).

## 🚀 Características Implementadas

✅ **OAuth2 Client Credentials Flow** (RFC 6749)
✅ **JWT Bearer Tokens** con scopes (mcp:read, mcp:call, mcp:resources)
✅ **Client Credentials Management** (create, list, revoke)
✅ **Token Validation Middleware** para Axum
✅ **Secure Secret Storage** (SHA-256 hashed secrets)
✅ **Docker Integration** con docker-compose multi-node

## 📦 Estructura del Código

```
crates/edge-hive-auth/
├── src/
│   ├── lib.rs              # Public API
│   ├── jwt.rs              # JWT generation & validation
│   ├── oauth2.rs           # OAuth2 Client Credentials flow
│   ├── client.rs           # Client credentials storage (in-memory → SurrealDB later)
│   ├── middleware.rs       # Axum authentication middleware
│   └── error.rs            # Error types
└── Cargo.toml

crates/edge-hive-core/
├── src/
│   ├── auth.rs             # OAuth2 endpoints integration
│   └── server.rs           # Updated with auth routes
```

## 🔧 Endpoints Implementados

### POST /mcp/auth/token

**OAuth2 Client Credentials Token Endpoint**

Request:

```json
{
  "grant_type": "client_credentials",
  "client_id": "cli_abc123...",
  "client_secret": "secret_xyz789...",
  "scope": "mcp:read mcp:call"
}
```

Response:

```json
{
  "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "token_type": "Bearer",
  "expires_in": 3600,
  "scope": "mcp:read mcp:call"
}
```

### POST /mcp/auth/clients

**Create New OAuth2 Client (Admin)**

Request:

```json
{
  "name": "vscode-client",
  "scopes": ["mcp:read", "mcp:call", "mcp:resources"]
}
```

Response:

```json
{
  "client_id": "cli_abc123...",
  "client_secret": "secret_xyz789...",  // ⚠️ Only shown ONCE!
  "name": "vscode-client",
  "scopes": ["mcp:read", "mcp:call", "mcp:resources"]
}
```

### GET /mcp/auth/clients

**List All OAuth2 Clients**

Response:

```json
{
  "clients": [
    {
      "client_id": "cli_abc123...",
      "name": "vscode-client",
      "scopes": ["mcp:read", "mcp:call"],
      "created_at": 1734230400,
      "revoked": false
    }
  ]
}
```

### DELETE /mcp/auth/clients/:client_id

**Revoke OAuth2 Client**

Response: 204 No Content

## 🧪 Testing

### 1. Start Server

```bash
# Build first
cargo build --release

# Run server
./target/release/edge-hive serve --port 8080
```

### 2. Run OAuth2 Tests

```powershell
# Windows PowerShell
./scripts/test-oauth2.ps1

# Linux/macOS
# (TODO: create test-oauth2.sh)
```

### 3. Manual Testing

```bash
# Create client
curl -X POST http://localhost:8080/mcp/auth/clients \
  -H "Content-Type: application/json" \
  -d '{"name":"test-client","scopes":["mcp:read","mcp:call"]}'

# Get token
curl -X POST http://localhost:8080/mcp/auth/token \
  -H "Content-Type: application/json" \
  -d '{
    "grant_type":"client_credentials",
    "client_id":"YOUR_CLIENT_ID",
    "client_secret":"YOUR_CLIENT_SECRET"
  }'

# Use token
curl http://localhost:8080/api/v1/node \
  -H "Authorization: Bearer YOUR_ACCESS_TOKEN"
```

## 🐳 Docker Testing

```bash
# Build and run 3-node network
docker-compose up --build

# Test node 1 (bootstrap)
curl http://localhost:8080/health

# Test OAuth2 on node 1
./scripts/test-oauth2.ps1 -BaseUrl "http://localhost:8080"

# Test node 2
curl http://localhost:8081/health

# Test node 3
curl http://localhost:8082/health
```

## 🔐 Security Features

| Feature | Status |
|---------|--------|
| **SHA-256 Secret Hashing** | ✅ Implemented |
| **Constant-Time Comparison** | ✅ Implemented |
| **JWT Signature (HS256)** | ✅ Implemented |
| **Token Expiration** | ✅ 1 hour default |
| **Scope Validation** | ✅ Implemented |
| **Client Revocation** | ✅ Implemented |
| **TLS/HTTPS** | ⏳ Next Phase |
| **Rate Limiting** | ⏳ Next Phase |

## 📝 JWT Claims Structure

```json
{
  "sub": "cli_abc123...",          // Subject (client_id)
  "iss": "https://localhost:8080",  // Issuer (node URL)
  "aud": "mcp",                     // Audience
  "exp": 1734234000,                // Expiration (Unix timestamp)
  "iat": 1734230400,                // Issued At
  "jti": "uuid-here",               // JWT ID (unique)
  "scopes": ["mcp:read", "mcp:call"], // Granted scopes
  "node_id": "node-123"             // Optional: Node identifier
}
```

## 🎯 Scopes

| Scope | Permission |
|-------|------------|
| `mcp:read` | Read node status, resources |
| `mcp:call` | Execute MCP tools/commands |
| `mcp:resources` | Access MCP resources (logs, etc.) |
| `mcp:admin` | Manage OAuth2 clients (future) |

## 📋 Next Steps (Issue #1 Checklist)

- [x] Phase 1: OAuth2 Infrastructure
  - [x] Create edge-hive-auth crate
  - [x] Implement JWT generation/validation
  - [x] Implement OAuth2 Client Credentials flow
  - [x] In-memory client store

- [x] Phase 2: MCP HTTPS Integration
  - [x] Add auth routes to Axum server
  - [x] Token endpoint (POST /mcp/auth/token)
  - [x] Client management endpoints

- [ ] Phase 3: Client Management (CLI)
  - [ ] `edge-hive auth client create` command
  - [ ] `edge-hive auth client list` command
  - [ ] `edge-hive auth client revoke` command

- [ ] Phase 4: TLS/HTTPS Setup
  - [ ] Integrate rustls for TLS 1.3
  - [ ] Generate self-signed certificates for testing
  - [ ] Production certificate support (Let's Encrypt)

- [ ] Phase 5: Testing & Integration
  - [ ] Unit tests for all auth modules ✅ (partial - jwt.rs has tests)
  - [ ] Integration tests with real HTTP requests
  - [ ] VS Code MCP client configuration
  - [ ] Docker multi-node auth testing

## 🔗 Related Files

- `.github/issues/FEAT_oauth2-mcp-authentication.md` - Full implementation plan
- `docs/agent-docs/ANALYSIS_SECURE_MCP_AUTHENTICATION.md` - Architecture analysis
- `vscode_mcp_client_config.json` - VS Code client configuration
- `docker-compose.yml` - Multi-node testing setup

## 📚 References

- [OAuth 2.0 RFC 6749](https://datatracker.ietf.org/doc/html/rfc6749)
- [JWT RFC 7519](https://datatracker.ietf.org/doc/html/rfc7519)
- [Model Context Protocol](https://spec.modelcontextprotocol.io/)
- [Supabase MCP Implementation](https://supabase.com/docs/guides/functions/ai-models)
