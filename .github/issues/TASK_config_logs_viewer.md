---
title: "TASK: Config Editor & Real-time Logs Viewer"
labels:
  - enhancement
  - jules
assignees: []
---

## Contexto
Implementar la capacidad de persistir configuraciones y visualizar logs reales desde la UI administrativa.

## Pasos de Implementaci\u00f3n
1. **Logs Viewer**
   - [ ] Backend: Crear comando Tauri para hacer streaming de los logs de `edge-hive-core`.
   - [ ] Frontend: Implementar consola de logs en tiempo real.

2. **Config Editor**
   - [ ] Backend: Comandos para leer y escribir el archivo `config.toml`.
   - [ ] Frontend: Formulario para editar variables de entorno y configuraci\u00f3n del nodo.

Related Epic: #116
