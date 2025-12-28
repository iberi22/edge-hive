---
title: "EPIC: Core Systems Evolution (Tor & WASM)"
labels:
  - epic
  - backend
  - rust
  - high-priority
assignees: []
---

## Descripción
Este Epic consolida las tareas de infraestructura crítica de bajo nivel para evitar conflictos con el desarrollo de UI. Se enfoca en la privacidad (Tor) y el runtime de ejecución (WASM).

## Sub-tareas (Backend Focus)

### Tor Implementation V2 (Privacy)
- [x] Dise\u00f1o Arquitect\u00f3nico: Definir c\u00f3mo `edge-hive-tunnel` gestionar\u00e1 el ciclo de vida de `arti` (Tor client en Rust).
- [x] Implementaci\u00f3n Base: Integrar crate `arti` para establecer conexiones onion salientes.
- [x] Gesti\u00f3n de Identidad: Implementar persistencia segura de llaves privadas (Hidden Service).
- [x] Pruebas de Red: Crear tests de integraci\u00f3n para verificar conectividad Tor sin exponer IP real.

### ⚡ Edge Functions V2 (Runtime)
- [ ] **Upgrade Wasmtime**: Actualizar dependencia `wasmtime` en `crates/edge-hive-wasm` a la última versión estable.
- [ ] **Resource Limits**: Implementar límites estrictos de CPU y Memoria (Memory/Fuel consumption) para evitar DoS por funciones de usuario.
- [ ] **Host Functions**: Exponer nuevas funciones nativas seguras (ej. HTTP fetch limitado) al runtime WASM.
