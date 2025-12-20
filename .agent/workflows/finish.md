---
description: Finaliza la sesión actual siguiendo el Git-Core Protocol (Sincronización, Limpieza, Commits Atómicos y Reporte AI).
---

// turbo-all

1. Ejecutar el script maestro de finalización:
`pwsh -File ./scripts/gc-finish.ps1`

2. Verificar que los issues cerrados se hayan eliminado de `.github/issues/`.
`Get-ChildItem -Path .github/issues/*.md`
