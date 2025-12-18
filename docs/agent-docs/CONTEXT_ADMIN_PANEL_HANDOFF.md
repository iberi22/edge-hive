# 🧠 Jules Master Context: Edge Hive Admin Panel

> **Protocolo de Asignación Masiva**
> **Versión**: 1.0
> **Fecha**: 2025-12-17
> **Objetivo**: Completar el 100% de la funcionalidad del Admin Panel y desplegar en producción.

## 1. Estado Actual (The "Baseline")
El **Edge Hive Admin Panel** (`edge-hive-admin/`) ha sido migrado a una arquitectura React/Tauri sólida.
- **Frontend**: Vite + React + TypeScript + Tailwind. Totalmente funcional visualmente.
- **Backend**: Rust (`src-tauri/`). Compila bien (`cargo check` passing).
- **Integración**: Los comandos Tauri (`tauriClient.ts`) están conectados, pero muchos son "Stubs" (devuelven datos falsos).

### ✅ Lo que YA funciona:
- Navegación, Layout, Autenticación (UI), Billing (UI).
- Event Loop de métricas del sistema (Rust -> Frontend).
- Logs en memoria.
- Configuración de Tauri (`tauri.conf.json`) correcta para Escritorio.

### 🚧 Lo que FALTA (Tu Misión):
Necesitamos reemplazar la "magia falsa" con "lógica real". Tienes asignadas ~100 micro-tareas agrupadas en los siguientes épicos.

## 2. Tus Misiones (The "Missing 100")

### Misión A: Control Real del Nodo (Backend)
**Contexto**: `src-tauri/src/commands.rs` tiene `TODOs` para iniciar/parar el servidor.
**Tareas**:
1. Implementar `start_server`: Debe invocar el binario `edge-hive-node` o usar la crate `edge-hive-core` en un thread separado.
2. Implementar `get_node_status`: Leer el estado real del nodo (peers, uptime, ancho de banda).
3. Implementar `get_peers`: Consultar la DHT de `libp2p`.

### Misión B: Infraestructura Cloud (Terraform/AWS)
**Contexto**: El dashboard tiene un botón "Provision Node".
**Tareas**:
1. Conectar `provision_cloud_node` con scripts de Terraform (ubicados en `infra/`).
2. Implementar polling de estado para mostrar "Provisioning..." -> "Running".

### Misión C: Facturación Real (Stripe)
**Contexto**: `billing_commands.rs` devuelve URLs falsas.
**Tareas**:
1. Integrar crate `async-stripe`.
2. Generar `checkout_session` real con los precios definidos.
3. Validar webhooks de Stripe para actualizar el estado de suscripción en `SurrealDB`.

### Misión D: Portabilidad Android (Termux)
**Contexto**: El objetivo final es correr esto en un celular.
**Tareas**:
1. Ajustar el build de Rust para `aarch64-linux-android`.
2. Verificar que la UI sea responsive en pantallas pequeñas.

## 3. Protocolo de Ejecución
Para CADA tarea que tomes:
1. **Analiza**: Lee el archivo `.github/issues/FEAT_*.md` correspondiente.
2. **Implementa**: Escribe el código en Rust/TypeScript.
3. **Verifica**: Usa `cargo check` y `npm run dev`.
4. **Deploy**: Si es estable, haz commit y push.

**Nota**: Tienes autonomía total para refactorizar si encuentras deuda técnica.
