---
title: "TASK: Replace Mock Metrics with Real Data"
labels:
  - task
  - rust
  - frontend
assignees: []
---

## Contexto
`edge-hive-admin` usa métricas falsas. Se requiere integración real con el sistema operativo.

## Pasos
1. **Backend (Rust)**
   - [ ] Agregar `sysinfo` a `Cargo.toml`.
   - [ ] Implementar `SystemMonitor` con `sysinfo::System`.
   - [ ] Reemplazar `spawn_metrics_loop` en `lib.rs` con datos reales (`refresh_cpu`, `refresh_memory`).
   - [ ] Mapear evento `system_metrics`.

2. **Frontend (Dashboard)**
   - [ ] Validar renderizado en `DashboardView` (0-100%).
   - [ ] Ajustar unidades (GB/MB, %).
