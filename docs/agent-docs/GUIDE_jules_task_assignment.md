# Guía: Asignación de Tareas a Jules

Esta guía describe el flujo de trabajo estándar para asignar tareas al agente Jules utilizando GitHub Issues.

## Flujo de Trabajo

Para que Jules inicie una tarea automáticamente, se recomienda seguir la documentación oficial de Google Cloud:

> **Referencia:** [Starting Tasks from GitHub Issues](https://jules.google/docs/running-tasks/#starting-tasks-from-github-issues)

### Pasos para Asignar

1.  **Crear un Issue:**
    Crea un nuevo issue en el repositorio describiendo la tarea.
    - Sé claro en la descripción.
    - Incluye Criterios de Aceptación si es posible.

2.  **Etiquetar el Issue:**
    Añade la etiqueta (label) `jules`.
    - Jules monitorea los issues con esta etiqueta.

3.  **Comentario de Confirmación (Opcional):**
    En algunos casos, puedes mencionar a `@google-labs-jules` en un comentario para forzar la atención, aunque la etiqueta debería ser suficiente.

### Ejemplo de Issue

```markdown
Title: Implementar endpoint de estado

Body:
Necesitamos un endpoint GET /status que devuelva el estado del servicio.
Respuesta esperada: { "status": "ok", "uptime": 123 }
```
*Labels: `enhancement`, `jules`*

---
**Nota del Proyecto:** Siempre utiliza este método para delegar tareas de implementación directa a Jules y mantener el tablero de proyecto actualizado.
