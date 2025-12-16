---
title: "Implement Unified API Gateway"
labels:
  - enhancement
  - architecture
  - ai-plan
assignees: []
---

## Description

Create a unified API Gateway to centralize all HTTP routes, middleware, and service integrations. This replaces the current scattered route definitions with a single, coherent API layer.

## Architecture

```
Client Request
     ↓
[CORS Middleware]
     ↓
[Auth Middleware] (JWT validation)
     ↓
[Cache Middleware] (check L1/L2)
     ↓
[Router]
     ├─ /api/v1/data/*     → Database Service
     ├─ /api/v1/auth/*     → OAuth2 Service
     ├─ /api/v1/edge/*     → WASM Runtime
     ├─ /api/v1/realtime   → WebSocket Upgrade
     └─ /api/v1/mcp/*      → MCP Server
     ↓
[Response]
```

## Routes to Implement

### Core Routes

- `GET /health` - Health check
- `GET /api/v1/info` - Node info

### Database Routes (with auto-cache)

- `GET /api/v1/data/:table` - Query records
- `POST /api/v1/data/:table` - Insert record
- `PUT /api/v1/data/:table/:id` - Update record
- `DELETE /api/v1/data/:table/:id` - Delete record

### Auth Routes

- `POST /api/v1/auth/login` - OAuth2 login
- `POST /api/v1/auth/refresh` - Refresh token
- `GET /api/v1/auth/logout` - Logout

### Edge Functions Routes

- `POST /api/v1/edge/:function` - Execute WASM function
- `GET /api/v1/edge` - List available functions

### Real-time Routes

- `GET /api/v1/realtime` - WebSocket upgrade
- `POST /api/v1/realtime/subscribe` - Subscribe to topic

### MCP Routes (existing)

- `POST /api/v1/mcp` - MCP JSON-RPC endpoint

## Technical Requirements

### Dependencies

```toml
[dependencies]
axum = { workspace = true }
tower = "0.5"
tower-http = { version = "0.6", features = ["cors", "trace"] }
serde = { workspace = true }
serde_json = { workspace = true }
```

### Middleware Stack

1. **CORS** - Permissive for development, configurable for production
2. **Tracing** - Request/response logging
3. **Auth** - JWT validation (optional per route)
4. **Cache** - Automatic cache check/update
5. **Rate Limiting** - Prevent abuse (future)

## Implementation Tasks

- [ ] Create `crates/edge-hive-api/` crate structure
- [ ] Implement router with all routes
- [ ] Add CORS middleware
- [ ] Add authentication middleware
- [ ] Add cache middleware integration
- [ ] Add request tracing
- [ ] Implement database route handlers
- [ ] Implement auth route handlers
- [ ] Implement edge function route handlers
- [ ] Implement real-time WebSocket upgrade
- [ ] Add error handling and standard error responses
- [ ] Write unit tests for handlers
- [ ] Write integration tests for full request flow
- [ ] Document all API endpoints (OpenAPI spec)

## Integration Points

- `edge-hive-db`: Database operations
- `edge-hive-cache`: Response caching
- `edge-hive-auth`: OAuth2 + JWT
- `edge-hive-wasm`: Edge function execution
- `edge-hive-realtime`: WebSocket connections
- `edge-hive-mcp`: MCP server

## Success Criteria

- [ ] All routes functional and tested
- [ ] Response time < 10ms for cached responses
- [ ] Proper error handling (4xx/5xx)
- [ ] OpenAPI documentation generated
- [ ] All tests passing

## References

- Axum docs: <https://docs.rs/axum>
- Tower middleware: <https://docs.rs/tower>
- VPS Migration Plan: `docs/VPS_MIGRATION_PLAN.md`
