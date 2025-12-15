---
title: "Design and implement web-based admin dashboard (Astro + Svelte)"
labels:
  - enhancement
  - ui
  - admin
  - astro
  - svelte
assignees:
  - jules
---

## 📋 Context

Edge Hive necesita un panel de administración web para gestionar nodos, monitorear red P2P, configurar Tor, y administrar servicios. El stack ya está inicializado en `app/` con Astro + Svelte + TailwindCSS.

**Investigación de Stack:**
- ✅ **Astro**: Framework optimizado para contenido SSR/SSG, 63% Core Web Vitals pass rate (vs Next.js 27%)
- ✅ **Svelte**: Compilador UI reactivo, usado por Spotify, NYT, Apple, Stack Overflow
- ✅ **TailwindCSS**: Utility-first CSS ya integrado
- ⚠️ **WebAssembly**: Considerar para embeber Rust core en el navegador (SurrealDB client, crypto)

**Proyecto actual:**
```
app/
├── astro.config.mjs     ✅ Astro 5.16 + Svelte integration
├── package.json         ✅ Dependencies configuradas
├── tailwind.config.mjs  ✅ TailwindCSS setup
└── src/
    ├── components/      📝 Crear componentes Svelte aquí
    ├── layouts/         📝 Layouts Astro para admin
    └── pages/           📝 Rutas del dashboard
```

---

## 🎯 Objetivos

### 1. **Investigación de Arquitectura UI** (1 día)

**Pregunta clave:** ¿Astro SSR + Svelte Islands vs Full Svelte SPA vs Hybrid con WASM?

**Comparar:**
| Enfoque | Pros | Contras | Ideal Para |
|---------|------|---------|-----------|
| **Astro SSR + Svelte Islands** | SEO, performance, server-first | Requiere Node.js runtime | Dashboards con mucho contenido |
| **Svelte SPA** | Offline-first, PWA | No SEO, bundle size | Apps embebidas (Tauri) |
| **Hybrid (Astro + WASM)** | Rust core en browser, máxima performance | Complejidad, debugging difícil | Apps críticas (crypto, DB local) |

**Investigar:**
- [ ] ¿Cómo compilar Rust crates a WASM y usarlos en Svelte?
  - Considerar: `wasm-pack`, `wasm-bindgen`
  - Target: `wasm32-unknown-unknown` o `wasm32-wasi`
  - Crates compatibles: `surrealdb-wasm`, `ed25519-dalek` (crypto)
  
- [ ] ¿Astro Static Site Generation (SSG) es viable para admin?
  - Pros: Deploy a GitHub Pages, S3, Netlify
  - Contras: Datos dinámicos requieren fetch client-side
  
- [ ] ¿Tauri WebView (src-tauri/) puede servir Astro app?
  - `src-tauri/tauri.conf.json` → `"devUrl": "../app/dist"`
  - Permite app desktop/mobile con mismo código

**Recomendación preliminar:**
```
✅ Astro SSR/SSG + Svelte Islands + WASM modules (Rust core)

Razones:
1. Astro compila a HTML estático → deploy anywhere (GitHub Pages, Docker, VPS)
2. Svelte Islands para interactividad (botones, formularios, gráficos)
3. WASM para lógica crítica (crypto, SurrealDB client, Tor connections)
4. Mismo código sirve en:
   - Web (https://admin.edgehive.local)
   - Desktop (Tauri app)
   - Mobile (Tauri Android/iOS)
```

---

### 2. **Diseño de UI/UX** (2 días)

**Páginas principales:**

#### **Dashboard Principal** (`/`)
```
┌─────────────────────────────────────────────────────────────┐
│ Edge Hive Admin                    [user]  [settings]  [?]  │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  📊 Network Status                                           │
│  ┌──────────────┬──────────────┬──────────────┐            │
│  │ Nodes Online │ Tor Circuits │ Bandwidth    │            │
│  │     12       │      3       │  1.2 MB/s    │            │
│  └──────────────┴──────────────┴──────────────┘            │
│                                                              │
│  🌐 P2P Network Topology                                     │
│  [Graph visualization: libp2p peer connections]             │
│                                                              │
│  🔐 Recent Activity                                          │
│  • Node 0xabc... connected via Tor                          │
│  • File sync completed: dataset.db (2.3 MB)                 │
│  • New identity discovered: peer-xyz                        │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

**Componentes Svelte necesarios:**
- `<NetworkStatus.svelte>` - Cards con métricas real-time (WebSocket)
- `<TopologyGraph.svelte>` - Visualización de red P2P (d3.js o cytoscape.js)
- `<ActivityLog.svelte>` - Stream de eventos con auto-scroll

---

#### **Nodes Manager** (`/nodes`)
```
┌─────────────────────────────────────────────────────────────┐
│ Nodes                                          [+ Add Node]  │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  Search: [________________]  Filter: [All] [Online] [Tor]   │
│                                                              │
│  ┌────────────────────────────────────────────────────────┐ │
│  │ ● node-android-1       192.168.1.10  Tor: ✅  Online   │ │
│  │   Ed25519: 0xabc123...              libp2p: ✅         │ │
│  │   [Edit] [Delete] [SSH] [Logs]                         │ │
│  └────────────────────────────────────────────────────────┘ │
│                                                              │
│  ┌────────────────────────────────────────────────────────┐ │
│  │ ○ node-vps-1           vps.example.com  Tor: ✅  Offline│ │
│  │   Ed25519: 0xdef456...              libp2p: ❌         │ │
│  │   [Edit] [Delete] [SSH] [Logs]                         │ │
│  └────────────────────────────────────────────────────────┘ │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

**Componentes:**
- `<NodeCard.svelte>` - Tarjeta individual de nodo
- `<NodeForm.svelte>` - Modal para agregar/editar nodo
- `<SSHTerminal.svelte>` - Terminal web (xterm.js + WebSocket)

---

#### **Tor Configuration** (`/tor`)
```
┌─────────────────────────────────────────────────────────────┐
│ Tor Network                                                  │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  Status: ✅ Connected to Tor                                │
│  Onion Service: http://abc123xyz.onion:8080                 │
│                                                              │
│  ⚙️ Configuration                                            │
│  ┌────────────────────────────────────────────────────────┐ │
│  │ ☑ Enable Tor on startup                                │ │
│  │ ☑ Use bridges (for censored networks)                  │ │
│  │ ☐ Enable HS v3 directory (experimental)                │ │
│  │                                                          │ │
│  │ Bandwidth Limit: [Unlimited ▼]                          │ │
│  │ Circuit Timeout:  [30] seconds                          │ │
│  │                                                          │ │
│  │ [Save Changes]                                           │ │
│  └────────────────────────────────────────────────────────┘ │
│                                                              │
│  📜 Circuit Logs                                             │
│  • 12:34:56 - Circuit built: 3 hops (DE → FR → US)         │
│  • 12:35:12 - HS published to directory                     │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

**Componentes:**
- `<TorStatus.svelte>` - Estado de conexión Tor (polling cada 5s)
- `<TorSettings.svelte>` - Formulario de configuración
- `<CircuitVisualization.svelte>` - Mapa de saltos Tor

---

#### **Database Explorer** (`/database`)
```
┌─────────────────────────────────────────────────────────────┐
│ SurrealDB Explorer                    [New Query]  [Export] │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  📂 Tables                                                   │
│  ├─ users (12 records)                                      │
│  ├─ nodes (5 records)                                       │
│  └─ files (248 records)                                     │
│                                                              │
│  💻 Query Editor                                             │
│  ┌────────────────────────────────────────────────────────┐ │
│  │ SELECT * FROM nodes WHERE status = 'online';            │ │
│  │                                                          │ │
│  │ [Run Query]                                              │ │
│  └────────────────────────────────────────────────────────┘ │
│                                                              │
│  📊 Results (5 rows)                                         │
│  ┌────────────────────────────────────────────────────────┐ │
│  │ id              │ name            │ status   │ ...      │ │
│  │ nodes:abc123    │ node-android-1  │ online   │ ...      │ │
│  │ nodes:def456    │ node-vps-1      │ online   │ ...      │ │
│  └────────────────────────────────────────────────────────┘ │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

**Componentes:**
- `<DBExplorer.svelte>` - Navegador de tablas (tree view)
- `<QueryEditor.svelte>` - Editor de SQL (CodeMirror)
- `<ResultsTable.svelte>` - Tabla paginada de resultados

---

#### **Settings** (`/settings`)
- API Keys (Cloudflare, AWS, Google Cloud)
- Billing configuration
- User management
- Security (2FA, API tokens)

---

### 3. **Implementación - Fase 1: Fundamentos** (3 días)

**Estructura de archivos:**
```
app/src/
├── components/
│   ├── layout/
│   │   ├── Header.svelte
│   │   ├── Sidebar.svelte
│   │   └── Footer.svelte
│   ├── dashboard/
│   │   ├── NetworkStatus.svelte
│   │   ├── TopologyGraph.svelte
│   │   └── ActivityLog.svelte
│   ├── nodes/
│   │   ├── NodeCard.svelte
│   │   ├── NodeForm.svelte
│   │   └── SSHTerminal.svelte
│   ├── tor/
│   │   ├── TorStatus.svelte
│   │   ├── TorSettings.svelte
│   │   └── CircuitVisualization.svelte
│   └── database/
│       ├── DBExplorer.svelte
│       ├── QueryEditor.svelte
│       └── ResultsTable.svelte
├── layouts/
│   ├── AdminLayout.astro      # Layout principal con sidebar
│   └── PublicLayout.astro     # Layout para login/public pages
├── pages/
│   ├── index.astro            # Dashboard
│   ├── nodes.astro            # Nodes manager
│   ├── tor.astro              # Tor config
│   ├── database.astro         # DB explorer
│   ├── settings.astro         # Settings
│   └── api/                   # Astro API endpoints (SSR)
│       ├── nodes.ts           # GET/POST/DELETE nodes
│       ├── tor.ts             # Tor status/config
│       └── db.ts              # SurrealDB queries
└── lib/
    ├── api.ts                 # API client (fetch wrappers)
    ├── websocket.ts           # WebSocket client (real-time)
    └── wasm/
        └── edgehive.wasm      # 🦀 Compiled Rust core
```

**Dependencias a agregar:**
```json
{
  "dependencies": {
    "@astrojs/svelte": "^5.0.0",
    "@astrojs/tailwind": "^5.0.0",
    "svelte": "^5.0.0",
    "tailwindcss": "^3.4.0",
    
    // UI Libraries
    "d3": "^7.9.0",               // Network graphs
    "cytoscape": "^3.30.0",       // Alternative graph lib
    "xterm": "^5.3.0",            // Terminal emulator
    "xterm-addon-fit": "^0.8.0",  // Terminal auto-resize
    "codemirror": "^6.0.0",       // Code editor
    
    // Real-time
    "ws": "^8.18.0",              // WebSocket client
    
    // Icons
    "@tabler/icons-svelte": "^3.0.0"
  },
  "devDependencies": {
    "vite": "^5.4.0",
    "vite-plugin-wasm": "^3.3.0"  // 🦀 WASM support
  }
}
```

**Tareas:**
- [ ] Configurar `vite-plugin-wasm` en `astro.config.mjs`
- [ ] Crear `AdminLayout.astro` con sidebar navigation
- [ ] Implementar routing básico (5 páginas)
- [ ] Setup TailwindCSS theme (dark mode opcional)
- [ ] Crear componentes base (Header, Sidebar, Footer)

---

### 4. **Implementación - Fase 2: Integración con Rust Backend** (4 días)

**Opciones de comunicación:**

#### **Opción A: REST API (Astro SSR endpoints)**
```typescript
// app/src/pages/api/nodes.ts
import type { APIRoute } from 'astro';

export const GET: APIRoute = async ({ request }) => {
  // Llamar a Rust backend via HTTP
  const response = await fetch('http://localhost:3000/api/nodes');
  const nodes = await response.json();
  
  return new Response(JSON.stringify(nodes), {
    headers: { 'Content-Type': 'application/json' }
  });
};
```

**Pros:** Simple, estándar
**Contras:** Requiere Rust HTTP server corriendo

---

#### **Opción B: WebAssembly (Rust en el navegador)**
```rust
// crates/edge-hive-wasm/src/lib.rs
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[wasm_bindgen]
pub struct AdminClient {
    db: SurrealDB,
}

#[wasm_bindgen]
impl AdminClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        // Inicializar SurrealDB en memoria (WASM)
        Self { db: SurrealDB::new_memory() }
    }
    
    #[wasm_bindgen]
    pub async fn get_nodes(&self) -> Result<JsValue, JsValue> {
        let nodes = self.db.query("SELECT * FROM nodes").await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        
        Ok(serde_wasm_bindgen::to_value(&nodes)?)
    }
}
```

```svelte
<!-- app/src/components/nodes/NodeList.svelte -->
<script lang="ts">
  import init, { AdminClient } from '$lib/wasm/edgehive.wasm';
  
  let client: AdminClient;
  let nodes = [];
  
  async function loadNodes() {
    await init(); // Inicializar WASM
    client = new AdminClient();
    nodes = await client.get_nodes();
  }
  
  onMount(loadNodes);
</script>

{#each nodes as node}
  <NodeCard {node} />
{/each}
```

**Pros:** No requiere backend corriendo, offline-first
**Contras:** Tamaño bundle (~2-5 MB WASM), complejidad

---

#### **Opción C: Hybrid (REST + WASM)**
- WASM para crypto (Ed25519 signing, encryption)
- REST API para I/O pesado (DB queries, file uploads)
- WebSocket para eventos real-time (node status, logs)

**Recomendación:** ✅ **Opción C (Hybrid)** - Balance entre performance y simplicidad

---

### 5. **WebAssembly Integration** (3 días)

**Build WASM:**
```bash
# Compilar Rust → WASM
cd crates/edge-hive-wasm
wasm-pack build --target web --out-dir ../../app/src/lib/wasm

# Output:
# app/src/lib/wasm/
# ├── edgehive_wasm.js
# ├── edgehive_wasm_bg.wasm
# └── edgehive_wasm.d.ts
```

**Configurar Vite:**
```javascript
// app/astro.config.mjs
import { defineConfig } from 'astro/config';
import svelte from '@astrojs/svelte';
import wasm from 'vite-plugin-wasm';

export default defineConfig({
  integrations: [svelte()],
  vite: {
    plugins: [wasm()],
    optimizeDeps: {
      exclude: ['$lib/wasm/edgehive.wasm']
    }
  }
});
```

**Usar en Svelte:**
```svelte
<script lang="ts">
  import init, { encrypt_file, sign_message } from '$lib/wasm/edgehive';
  
  async function handleEncrypt(file: File) {
    await init();
    const bytes = new Uint8Array(await file.arrayBuffer());
    const encrypted = encrypt_file(bytes, 'my-secret-key');
    // ...
  }
</script>
```

**Tareas:**
- [ ] Crear `crates/edge-hive-wasm/` con `wasm-pack`
- [ ] Exportar funciones críticas (crypto, DB client)
- [ ] Integrar en Astro con `vite-plugin-wasm`
- [ ] Escribir tipos TypeScript para WASM exports

---

### 6. **Real-Time Features (WebSocket)** (2 días)

**Rust WebSocket server:**
```rust
// crates/edge-hive-core/src/api/websocket.rs
use axum::extract::ws::{WebSocket, WebSocketUpgrade};

#[derive(Serialize)]
enum Event {
    NodeConnected { id: String },
    LogEntry { message: String },
    MetricUpdate { cpu: f32, memory: u64 },
}

async fn handle_websocket(socket: WebSocket) {
    loop {
        let event = Event::NodeConnected { id: "node-1".into() };
        socket.send(serde_json::to_string(&event).unwrap()).await;
        sleep(Duration::from_secs(1)).await;
    }
}
```

**Svelte client:**
```svelte
<script lang="ts">
  import { onMount } from 'svelte';
  
  let logs = [];
  
  onMount(() => {
    const ws = new WebSocket('ws://localhost:3000/ws');
    
    ws.onmessage = (event) => {
      const data = JSON.parse(event.data);
      if (data.type === 'LogEntry') {
        logs = [...logs, data.message];
      }
    };
    
    return () => ws.close();
  });
</script>

<ul>
  {#each logs as log}
    <li>{log}</li>
  {/each}
</ul>
```

---

### 7. **Testing & QA** (2 días)

**Playwright E2E:**
```typescript
// app/tests/dashboard.spec.ts
import { test, expect } from '@playwright/test';

test('dashboard loads with network status', async ({ page }) => {
  await page.goto('/');
  
  await expect(page.locator('h1')).toContainText('Edge Hive Admin');
  await expect(page.locator('[data-testid="nodes-online"]')).toBeVisible();
  
  const nodesCount = await page.locator('[data-testid="nodes-online"]').textContent();
  expect(parseInt(nodesCount)).toBeGreaterThan(0);
});

test('can add new node', async ({ page }) => {
  await page.goto('/nodes');
  await page.click('text=Add Node');
  
  await page.fill('input[name="name"]', 'test-node');
  await page.fill('input[name="address"]', '192.168.1.100');
  await page.click('button:has-text("Save")');
  
  await expect(page.locator('text=test-node')).toBeVisible();
});
```

**Vitest Unit Tests:**
```typescript
// app/src/lib/api.test.ts
import { describe, it, expect, vi } from 'vitest';
import { getNodes } from './api';

describe('API Client', () => {
  it('fetches nodes from backend', async () => {
    global.fetch = vi.fn(() =>
      Promise.resolve({
        json: () => Promise.resolve([{ id: '1', name: 'node-1' }])
      })
    );
    
    const nodes = await getNodes();
    expect(nodes).toHaveLength(1);
    expect(nodes[0].name).toBe('node-1');
  });
});
```

---

## 📦 Deliverables

- [ ] **Documento de investigación** (`docs/agent-docs/RESEARCH_UI_ARCHITECTURE.md`)
  - Comparación Astro SSR vs Svelte SPA vs Hybrid WASM
  - Decisión final con justificación técnica
  
- [ ] **Diseño UI/UX** (Figma o similar)
  - Mockups de 5 páginas principales
  - Sistema de diseño (colores, tipografía, componentes)
  
- [ ] **Código funcional:**
  - Astro app con routing completo
  - Componentes Svelte base implementados
  - Integración WASM funcional (crypto operations)
  - WebSocket client conectado a Rust backend
  
- [ ] **Tests:**
  - E2E tests (Playwright): 10+ scenarios
  - Unit tests (Vitest): 80%+ coverage en utils/lib
  
- [ ] **Documentación:**
  - README en `app/` con instrucciones de desarrollo
  - Guía de contribución para nuevos componentes

---

## 🎯 Definition of Done

- ✅ Investigación documentada con decisión arquitectónica clara
- ✅ Figma mockups aprobados (o equivalente)
- ✅ `npm run dev` inicia Astro dev server sin errores
- ✅ Dashboard muestra datos reales desde Rust backend (REST o WASM)
- ✅ WebSocket real-time funciona (logs o metrics actualizando)
- ✅ Al menos 1 componente Svelte usa WASM (ej: crypto signing)
- ✅ Tests E2E pasan en CI (GitHub Actions)
- ✅ Build production (`npm run build`) genera static site deployable

---

## 🔗 Related Issues

- #0 Architecture foundation (completed)
- #7 Identity system (Ed25519) - Usaremos en crypto WASM
- #8 Node discovery (libp2p) - Visualizar en TopologyGraph.svelte
- Pending: INFRA_termux-deployment (blocking APK native UI)

---

## 📚 References

- [Astro Docs](https://docs.astro.build/)
- [Svelte Tutorial](https://svelte.dev/tutorial)
- [wasm-pack Book](https://rustwasm.github.io/wasm-pack/)
- [wasm-bindgen Guide](https://rustwasm.github.io/wasm-bindgen/)
- [Tauri + Astro Guide](https://tauri.app/develop/frontend-frameworks/)

---

## 💬 Notes

**Jules:** Este issue es perfecto para investigar arquitectura híbrida Astro+WASM. Prioriza:
1. Investigación de viabilidad WASM (2 días)
2. Mockups UI (1 día)
3. Implementación incremental (dashboard → nodes → tor → db)

Si WASM resulta muy complejo, fallback a REST API puro está bien. El objetivo es tener un admin funcional, no bleeding-edge tech.

**Timeline:** 2 semanas (14 días) - Puede paralelizar con otras tareas si Jules lo desea.
