---
title: "Eliminar cache de Rust para liberar espacio"
labels:
  - chore
  - maintenance
assignees: ["@me"]
---

## Descripción
El directorio `target` ocupa más de 84GB. Se requiere realizar una limpieza para liberar espacio en disco.

## Tareas
- [x] Ejecutar `cargo clean` (completado manualmente borrando `target` tras error de manifest)
- [x] Verificar espacio liberado
