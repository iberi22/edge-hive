---
title: "TASK: Upgrade Wasmtime Runtime"
labels:
  - task
  - wasm
  - security
assignees: []
---

## Contexto
Actualizar el motor de ejecución `wasmtime` para mejoras de seguridad y performance.

## Pasos
1. **Actualización**
   - [ ] Modificar `crates/edge-hive-wasm/Cargo.toml` a `wasmtime = "26"` (o más reciente estable).
   - [ ] Corregir breaking changes en la API de `Linker` y `Store`.

2. **Seguridad**
   - [ ] Configurar `Config::new().consume_fuel(true)` para limitar CPU.
   - [ ] Establecer límites de memoria lineal (Memory Limits).

3. **Bindings**
   - [ ] Verificar compatibilidad con `wit-bindgen` si se usa.
