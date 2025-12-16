# 🚀 Edge Hive VPS Migration Plan

> **De Supabase Jamstack a Edge Hive VPS con Rust Edge Functions**

## 📊 Comparativa: Supabase vs Edge Hive

| Feature | Supabase | Edge Hive | Ventaja |
|---------|----------|-----------|---------|
| **Database** | PostgreSQL | SurrealDB | Graph queries, vector search |
| **Real-time** | PostgreSQL LISTEN/NOTIFY | SurrealDB Live Queries | Nativo, más rápido |
| **Edge Functions** | Deno (JavaScript) | Rust WASM | 10-100x más rápido |
| **Auth** | GoTrue (Go) | OAuth2 nativo Rust | Integrado, sin servicios externos |
| **Storage** | S3-compatible | Local + opcional S3 | Sin latencia, offline-first |
| **Caching** | **No incluido** | **Redis-compatible** | Implementar con `moka` |
| **Hosting** | Cloud only | Self-hosted VPS | Control total, costo fijo |
| **Cost** | $25/mes + uso | $0 (VPS ya existe) | Ahorro 100% |

---

## 🎯 Plan de Implementación

### Fase 1: Cache Layer (Redis-compatible) ⚡

**Opciones de Cache en Rust:**

| Crate | Tipo | Ventaja |
|-------|------|---------|
| **moka** | In-memory LRU cache | TTL, eviction policies, sync/async |
| **mini-redis** | Redis server completo | Compatible con clientes Redis |
| **redis-rs** | Cliente Redis nativo | Conectar a Redis externo |
| **sled** | Persistent cache | KV store embebido, fast |

**Recomendación:** `moka` + `mini-redis`

- **moka**: Cache L1 ultrarrápido (nanosegundos)
- **mini-redis**: Cache L2 compatible con Redis (microsegundos)

#### Implementación

```toml
# Cargo.toml
[dependencies]
moka = { version = "0.12", features = ["future"] }
mini-redis = "0.4"
```

```rust
// crates/edge-hive-cache/src/lib.rs
use moka::future::Cache;
use std::time::Duration;

pub struct CacheService {
    // L1: In-memory ultra-fast
    l1_cache: Cache<String, Vec<u8>>,
    // L2: Redis-compatible (opcional)
    l2_cache: Option<mini_redis::client::Client>,
}

impl CacheService {
    pub fn new(max_capacity: u64, ttl_secs: u64) -> Self {
        let l1_cache = Cache::builder()
            .max_capacity(max_capacity)
            .time_to_live(Duration::from_secs(ttl_secs))
            .build();

        Self {
            l1_cache,
            l2_cache: None,
        }
    }

    pub async fn get(&self, key: &str) -> Option<Vec<u8>> {
        // Try L1 first (nanoseconds)
        if let Some(value) = self.l1_cache.get(key).await {
            return Some(value);
        }

        // Try L2 if configured (microseconds)
        if let Some(ref client) = self.l2_cache {
            if let Ok(value) = client.get(key).await {
                // Populate L1
                self.l1_cache.insert(key.to_string(), value.clone()).await;
                return Some(value);
            }
        }

        None
    }

    pub async fn set(&self, key: String, value: Vec<u8>) {
        // Write to both layers
        self.l1_cache.insert(key.clone(), value.clone()).await;

        if let Some(ref mut client) = self.l2_cache {
            let _ = client.set(&key, value).await;
        }
    }
}
```

### Fase 2: Real-time con SurrealDB Live Queries 📡

**Supabase Real-time:**

```javascript
// Supabase - PostgreSQL LISTEN/NOTIFY
supabase
  .from('messages')
  .on('INSERT', payload => {
    console.log('New message:', payload.new)
  })
  .subscribe()
```

**Edge Hive - SurrealDB Live Queries:**

```rust
// Rust backend - MUCHO más rápido
use surrealdb::sql::Uuid;
use futures::StreamExt;

pub async fn subscribe_to_messages(db: &Surreal<Any>) -> Result<(), DbError> {
    let mut stream = db
        .query("LIVE SELECT * FROM messages")
        .await?
        .stream::<Message>(0)?;

    while let Some(result) = stream.next().await {
        match result {
            Ok(notification) => {
                // Broadcast via WebSocket
                broadcast_to_clients(&notification.data).await;
            }
            Err(e) => error!("Live query error: {}", e),
        }
    }

    Ok(())
}
```

**Frontend (Svelte):**

```typescript
// app/src/lib/realtime.ts
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

export async function subscribeToMessages(callback: (msg: any) => void) {
  // Start live query in backend
  await invoke('start_live_query', { table: 'messages' });

  // Listen for events
  const unlisten = await listen('live-query-update', (event) => {
    callback(event.payload);
  });

  return unlisten;
}
```

### Fase 3: Rust Edge Functions 🦀

**Supabase Edge Functions (Deno):**

```typescript
// Supabase - JavaScript en Deno
Deno.serve(async (req) => {
  const { name } = await req.json()
  return new Response(JSON.stringify({ message: `Hello ${name}!` }))
})
```

**Edge Hive - Rust WASM Edge Functions:**

```rust
// crates/edge-hive-wasm/examples/hello.rs
use edge_hive_wasm::prelude::*;

#[export_function]
pub fn process_request(input: String) -> Result<String, String> {
    let data: serde_json::Value = serde_json::from_str(&input)
        .map_err(|e| e.to_string())?;

    let name = data["name"].as_str().unwrap_or("World");

    Ok(format!(r#"{{"message": "Hello {}!"}}"#, name))
}
```

**Ventajas:**

- ⚡ **10-100x más rápido** que JavaScript
- 🔒 **Sandboxed** con WASM (seguro)
- 📦 **Tamaño:** ~100KB vs 5MB (Deno runtime)
- 🔥 **Sin cold start**

### Fase 4: API Gateway + Router 🌐

```rust
// crates/edge-hive-core/src/api/mod.rs
use axum::{Router, routing::{get, post}, Extension};
use tower_http::cors::CorsLayer;

pub fn build_api_router(
    db: Arc<DatabaseService>,
    cache: Arc<CacheService>,
    wasm_runtime: Arc<WasmRuntime>,
) -> Router {
    Router::new()
        // Static routes
        .route("/health", get(health))
        .route("/api/auth/*path", post(auth_handler))

        // Real-time WebSocket
        .route("/api/realtime", get(websocket_handler))

        // Edge functions (WASM)
        .route("/api/edge/:function", post(edge_function_handler))

        // Database queries (con cache automático)
        .route("/api/data/:table", get(query_handler))
        .route("/api/data/:table", post(insert_handler))

        // Middleware
        .layer(CorsLayer::permissive())
        .layer(Extension(db))
        .layer(Extension(cache))
        .layer(Extension(wasm_runtime))
}

async fn edge_function_handler(
    Path(function_name): Path<String>,
    Extension(wasm): Extension<Arc<WasmRuntime>>,
    body: String,
) -> Result<String, StatusCode> {
    // Execute WASM function
    wasm.call(&function_name, &body)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn query_handler(
    Path(table): Path<String>,
    Extension(db): Extension<Arc<DatabaseService>>,
    Extension(cache): Extension<Arc<CacheService>>,
) -> Result<Json<Vec<Value>>, StatusCode> {
    let cache_key = format!("query:{}", table);

    // Try cache first
    if let Some(cached) = cache.get(&cache_key).await {
        return Ok(Json(serde_json::from_slice(&cached).unwrap()));
    }

    // Query database
    let query = format!("SELECT * FROM {}", table);
    let result = db.query(&query).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Cache result (TTL: 60s)
    let json = serde_json::to_vec(&result).unwrap();
    cache.set(cache_key, json).await;

    Ok(Json(result))
}
```

---

## 🔧 Estructura de Archivos

```
edge-hive/
├── crates/
│   ├── edge-hive-cache/       # 🆕 Cache layer (moka + mini-redis)
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── l1_cache.rs    # Moka in-memory
│   │   │   └── l2_cache.rs    # Mini-redis protocol
│   │   └── Cargo.toml
│   │
│   ├── edge-hive-realtime/    # 🆕 Real-time pub/sub
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── live_query.rs  # SurrealDB live queries
│   │   │   └── websocket.rs   # WebSocket server
│   │   └── Cargo.toml
│   │
│   ├── edge-hive-functions/   # 🆕 Edge functions runtime
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── runtime.rs     # WASM runtime
│   │   │   └── registry.rs    # Function registry
│   │   └── Cargo.toml
│   │
│   └── edge-hive-api/         # 🆕 API Gateway
│       ├── src/
│       │   ├── lib.rs
│       │   ├── router.rs
│       │   └── handlers/
│       └── Cargo.toml
│
├── functions/                  # 🆕 User edge functions (WASM)
│   ├── hello.wasm
│   └── process_data.wasm
│
└── app/                        # Frontend (ya existe)
    └── src/
        └── lib/
            ├── realtime.ts    # 🆕 Real-time client
            └── cache.ts       # 🆕 Client-side cache
```

---

## 🚀 Deployment VPS

### 1. Build de Producción

```bash
# Build optimizado
cargo build --release --package edge-hive

# Output: target/release/edge-hive (~15MB)
```

### 2. Configuración VPS

```yaml
# /etc/edge-hive/config.yaml
server:
  host: "0.0.0.0"
  port: 8443
  https: true
  cert: "/etc/ssl/edge-hive.crt"
  key: "/etc/ssl/edge-hive.key"

database:
  backend: "rocksdb"
  path: "/var/lib/edge-hive/db"

cache:
  max_memory: "2GB"
  ttl_default: 300  # 5 minutos
  redis_port: 6379  # Opcional: Redis protocol

realtime:
  websocket_port: 8444
  max_connections: 10000

functions:
  runtime: "wasmtime"
  max_instances: 100
  timeout: 30s
```

### 3. Systemd Service

```ini
# /etc/systemd/system/edge-hive.service
[Unit]
Description=Edge Hive VPS Server
After=network.target

[Service]
Type=simple
User=edge-hive
ExecStart=/usr/local/bin/edge-hive serve --config /etc/edge-hive/config.yaml
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
```

```bash
# Activar
sudo systemctl enable edge-hive
sudo systemctl start edge-hive
```

### 4. Nginx Reverse Proxy

```nginx
# /etc/nginx/sites-available/edge-hive
server {
    listen 80;
    listen 443 ssl http2;
    server_name your-vps.com;

    ssl_certificate /etc/ssl/edge-hive.crt;
    ssl_certificate_key /etc/ssl/edge-hive.key;

    # Static files
    location / {
        root /var/www/edge-hive;
        try_files $uri $uri/ /index.html;
    }

    # API
    location /api/ {
        proxy_pass https://127.0.0.1:8443;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }

    # WebSocket (real-time)
    location /api/realtime {
        proxy_pass https://127.0.0.1:8444;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
    }
}
```

---

## 📈 Performance Estimado

| Métrica | Supabase | Edge Hive | Mejora |
|---------|----------|-----------|--------|
| **Edge Function** | 50-200ms | 1-5ms | **40-200x** |
| **Cache Hit** | N/A | 0.1ms | **Nuevo** |
| **Real-time Latency** | 100-500ms | 10-50ms | **10x** |
| **DB Query** | 50-200ms | 5-50ms | **4-10x** |
| **Cold Start** | 1-3s (Deno) | 0ms (native) | **∞** |

---

## ✅ Checklist de Migración

- [ ] **Fase 1:** Implementar cache layer (moka + mini-redis)
- [ ] **Fase 2:** Real-time con SurrealDB Live Queries
- [ ] **Fase 3:** WASM Edge Functions runtime
- [ ] **Fase 4:** API Gateway unificado
- [ ] **Fase 5:** Build de producción
- [ ] **Fase 6:** Deploy en VPS
- [ ] **Fase 7:** Migrar datos de Supabase
- [ ] **Fase 8:** Migrar auth (OAuth2)
- [ ] **Fase 9:** Tests de carga
- [ ] **Fase 10:** Monitoring (Prometheus + Grafana)

---

## 🎯 Próximo Paso

¿Quieres que implemente alguna fase específica?

1. **Cache layer** (moka + mini-redis)
2. **Real-time** (SurrealDB Live Queries + WebSocket)
3. **Edge Functions** (WASM runtime)
4. **API Gateway** completo
