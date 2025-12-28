---
title: "TASK: Integrate Arti Tor Client"
labels:
  - task
  - networking
  - rust
assignees: []
---

## Contexto
Implementar Tor V2 usando la librería oficial `arti` en Rust puro.

## Pasos
1. **Dependencias**
   - [ ] Agregar `arti` y `tokio` a `crates/edge-hive-tunnel`.

2. **Core Logic**
   - [ ] Implementar `TorService` que inicialice `arti::TorClient`.
   - [ ] Configurar almacenamiento persistente para identidad (keys).
   - [ ] Crear método `connect_onion` para salir a la red Tor.

3. **API Exposición**
   - [ ] Exponer estado de conexión (Building/Ready) a `edge-hive-admin`.
