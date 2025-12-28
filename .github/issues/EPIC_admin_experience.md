---
title: "EPIC: Admin Experience & Real Integration"
labels:
  - epic
  - frontend
  - tauri
  - ux
assignees: []
---

## Descripción
Este Epic se enfoca en la experiencia de usuario y la integración real entre el frontend (Tauri/React) y el sistema operativo, reemplazando simulaciones con datos reales.

## Sub-tareas (Frontend/Integration Focus)

### 📊 Real System Metrics (No More Fakes)
- [x] **Dependencia Sysinfo**: Agregar crate `sysinfo` a `edge-hive-admin/src-tauri`.
- [x] **Refactor Backend**: Reemplazar `spawn_metrics_loop` en `lib.rs` para leer uso real de CPU, RAM y Disco.
- [x] **Frontend Binding**: Asegurar que los componentes de React (`DashboardView`) consuman y muestren estos datos en tiempo real.

### 🎨 UX & Branding "Free Tier"
- [x] **Stripe Cleanup Visual**: Verificar que no queden botones de "Upgrade" o "Checkout" visibles en la UI.
- [x] **Badge Free Tier**: Agregar indicador visual "Community Edition / Free" en el header.
- [x] **Onboarding Flow**: Ajustar el flujo de bienvenida para no solicitar configuración de pagos.

### 🔧 Config & Settings
- [ ] **Editor de Configuración**: Implementar UI para editar `config.toml` del nodo desde la app.
- [ ] **Logs Viewer**: Conectar el visualizador de logs del frontend a la salida estándar real del daemon `edge-hive-core`.
