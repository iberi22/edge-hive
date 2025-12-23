---
title: "Implementar esquema de usuarios en SurrealDB"
labels:
  - enhancement
  - auth
  - database
  - jules
assignees: []
priority: high
---

## Descripción
Agregar el esquema de la tabla `users` en `DatabaseService::initialize_schema()` para almacenar credenciales de usuarios.

## Contexto
El sistema de login para Jamstack necesita una tabla de usuarios con soporte para:
- Email/password authentication
- OAuth2 providers (GitHub, Google)
- Campos de perfil básico

## Archivos a Modificar
- `crates/edge-hive-db/src/lib.rs`

## Implementación Requerida

### 1. Agregar tabla `users` en `initialize_schema()`

```rust
// En DatabaseService::initialize_schema()
self.db.query(r#"
    DEFINE TABLE users SCHEMAFULL;
    DEFINE FIELD email ON users TYPE string ASSERT string::is::email($value);
    DEFINE FIELD password_hash ON users TYPE string;
    DEFINE FIELD provider ON users TYPE option<string>;  -- 'local', 'github', 'google'
    DEFINE FIELD provider_id ON users TYPE option<string>;
    DEFINE FIELD name ON users TYPE option<string>;
    DEFINE FIELD avatar_url ON users TYPE option<string>;
    DEFINE FIELD created_at ON users TYPE datetime DEFAULT time::now();
    DEFINE FIELD updated_at ON users TYPE datetime DEFAULT time::now();
    DEFINE FIELD email_verified ON users TYPE bool DEFAULT false;
    DEFINE FIELD role ON users TYPE string DEFAULT 'user';
    DEFINE INDEX users_email ON users COLUMNS email UNIQUE;
    DEFINE INDEX users_provider ON users COLUMNS provider, provider_id UNIQUE;
"#).await?;
```

### 2. Agregar struct `StoredUser`

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredUser {
    pub id: Option<surrealdb::sql::Thing>,
    pub email: String,
    pub password_hash: String,
    pub provider: Option<String>,
    pub provider_id: Option<String>,
    pub name: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub email_verified: bool,
    pub role: String,
}
```

### 3. Agregar métodos CRUD para usuarios

```rust
impl DatabaseService {
    pub async fn create_user(&self, user: &StoredUser) -> Result<StoredUser, DbError>;
    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<StoredUser>, DbError>;
    pub async fn get_user_by_provider(&self, provider: &str, provider_id: &str) -> Result<Option<StoredUser>, DbError>;
    pub async fn update_user(&self, user: &StoredUser) -> Result<(), DbError>;
    pub async fn delete_user(&self, id: &str) -> Result<(), DbError>;
}
```

## Criterios de Aceptación
- [ ] Tabla `users` se crea automáticamente en `initialize_schema()`
- [ ] `StoredUser` struct exportado desde el crate
- [ ] Métodos CRUD implementados y testeados
- [ ] Índice único en `email` funciona (no permite duplicados)
- [ ] Tests unitarios pasan

## Tests Requeridos
```rust
#[tokio::test]
async fn test_create_and_get_user() {
    // Crear usuario, verificar que se puede recuperar por email
}

#[tokio::test]
async fn test_unique_email_constraint() {
    // Intentar crear dos usuarios con el mismo email, debe fallar
}
```
