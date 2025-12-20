---
title: "Auditoría y Limpieza de Dependencias (Eliminar 500+ paquetes)"
labels:
  - refactor
  - maintenance
assignees: ["@me"]
---

## Descripción
El proyecto tiene más de 500 dependencias y el árbol está actualmente roto debido a paquetes antiguos ("yanked"). Se requiere una limpieza para optimizar el tiempo de descarga y compilación.

## Tareas
- [x] Eliminar dependencias antiguas de Stripe que causan errores de "yanked version".
- [x] Identificar y remover crates no utilizados en el workspace.
- [x] Optimizar el uso de `features` en crates pesados (`libp2p`, `arti-client`, `axum`).
- [x] Unificar versiones duplicadas con `cargo tree -d`.

## Notas
Actualmente `stripe-rust v0.12.3` está bloqueando el árbol porque depende de paquetes eliminados de crates.io.
