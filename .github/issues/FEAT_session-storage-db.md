---
title: "Implementar almacenamiento de sesiones/tokens en DB"
labels:
  - enhancement
  - auth
  - database
  - jules
assignees: []
priority: high
---

## Descripción
Agregar tabla `sessions` para almacenar tokens de refresco y sesiones activas, permitiendo invalidación manual y TTL automático.

## Contexto
Sin almacenamiento de sesiones:
- No se pueden invalidar tokens individuales
- No hay forma de "cerrar sesión en todos los dispositivos"
- No hay refresh tokens persistentes

## Archivos a Modificar
- `crates/edge-hive-db/src/lib.rs`

## Implementación Requerida

### 1. Agregar tabla `sessions` en `initialize_schema()`

```rust
self.db.query(r#"
    DEFINE TABLE sessions SCHEMAFULL;
    DEFINE FIELD user_id ON sessions TYPE record<users>;
    DEFINE FIELD refresh_token_hash ON sessions TYPE string;
    DEFINE FIELD device_info ON sessions TYPE option<string>;
    DEFINE FIELD ip_address ON sessions TYPE option<string>;
    DEFINE FIELD created_at ON sessions TYPE datetime DEFAULT time::now();
    DEFINE FIELD expires_at ON sessions TYPE datetime;
    DEFINE FIELD revoked ON sessions TYPE bool DEFAULT false;
    DEFINE INDEX sessions_user ON sessions COLUMNS user_id;
    DEFINE INDEX sessions_token ON sessions COLUMNS refresh_token_hash UNIQUE;
"#).await?;
```

### 2. Agregar struct `StoredSession`

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredSession {
    pub id: Option<surrealdb::sql::Thing>,
    pub user_id: surrealdb::sql::Thing,
    pub refresh_token_hash: String,
    pub device_info: Option<String>,
    pub ip_address: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub revoked: bool,
}
```

### 3. Agregar métodos de sesión

```rust
impl DatabaseService {
    /// Crear nueva sesión
    pub async fn create_session(&self, session: &StoredSession) -> Result<StoredSession, DbError>;

    /// Obtener sesión por refresh token hash
    pub async fn get_session_by_token(&self, token_hash: &str) -> Result<Option<StoredSession>, DbError>;

    /// Revocar una sesión específica
    pub async fn revoke_session(&self, session_id: &str) -> Result<(), DbError>;

    /// Revocar TODAS las sesiones de un usuario (logout everywhere)
    pub async fn revoke_all_user_sessions(&self, user_id: &str) -> Result<u64, DbError>;

    /// Limpiar sesiones expiradas (llamar periódicamente)
    pub async fn cleanup_expired_sessions(&self) -> Result<u64, DbError>;
}
```

## Criterios de Aceptación
- [ ] Tabla `sessions` se crea automáticamente
- [ ] `StoredSession` struct exportado
- [ ] Métodos de sesión implementados
- [ ] `revoke_all_user_sessions` funciona correctamente
- [ ] Tests unitarios pasan

## Tests Requeridos
```rust
#[tokio::test]
async fn test_session_lifecycle() {
    // Crear sesión, verificar, revocar, verificar revocación
}

#[tokio::test]
async fn test_revoke_all_sessions() {
    // Crear múltiples sesiones, revocar todas, verificar
}
```
