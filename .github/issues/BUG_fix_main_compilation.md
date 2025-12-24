---
title: "Fix main branch compilation"
labels:
  - bug
  - critical
assignees: []
---

## Descripción
El código en la rama `main` tiene errores de compilación que impiden el despliegue.

## Tareas
- [ ] Ejecutar `cargo check --workspace` para identificar errores
- [ ] Corregir errores de dependencias
- [ ] Corregir errores de tipos y borrow checker
- [ ] Verificar que `cargo test` pase
