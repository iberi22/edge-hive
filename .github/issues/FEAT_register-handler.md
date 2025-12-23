---
title: "Implementar handler de registro de usuarios"
labels:
  - enhancement
  - auth
  - api
  - jules
assignees: []
priority: high
---

## Descripción
Crear endpoint `POST /api/v1/auth/register` para registro de nuevos usuarios con validación y hashing de password.

## Contexto
El sistema de login necesita un flujo de registro que:
- Valide email único
- Hashee password con Argon2
- Cree usuario en BD
- Opcionalmente envíe email de verificación

## Archivos a Modificar
- `crates/edge-hive-api/src/handlers/auth.rs`
- `crates/edge-hive-api/src/lib.rs` (agregar ruta)

## Dependencias
- **Requiere:** `FEAT_user-schema-surrealdb` completado

## Implementación Requerida

### 1. Agregar ruta de registro

```rust
// En lib.rs
let auth_routes = Router::new()
    .route("/api/v1/auth/register", post(handlers::auth::register))
    .route("/api/v1/auth/login", post(handlers::auth::login))
    // ...
```

### 2. Crear struct de request/response

```rust
#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub name: Option<String>,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub user_id: String,
    pub email: String,
    pub message: String,
}
```

### 3. Implementar handler de registro

```rust
pub async fn register(
    Extension(state): Extension<ApiState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<RegisterResponse>, (StatusCode, Json<ErrorResponse>)> {
    // 1. Validar formato de email
    if !is_valid_email(&payload.email) {
        return Err((StatusCode::BAD_REQUEST, Json(ErrorResponse {
            error: "Invalid email format".into()
        })));
    }

    // 2. Validar password strength
    if payload.password.len() < 8 {
        return Err((StatusCode::BAD_REQUEST, Json(ErrorResponse {
            error: "Password must be at least 8 characters".into()
        })));
    }

    // 3. Verificar que email no existe
    if state.db.get_user_by_email(&payload.email).await?.is_some() {
        return Err((StatusCode::CONFLICT, Json(ErrorResponse {
            error: "Email already registered".into()
        })));
    }

    // 4. Hash password con Argon2
    let password_hash = hash_password(&payload.password)?;

    // 5. Crear usuario
    let user = StoredUser {
        email: payload.email.clone(),
        password_hash,
        name: payload.name,
        provider: Some("local".into()),
        role: "user".into(),
        // ...
    };

    let created = state.db.create_user(&user).await?;

    Ok(Json(RegisterResponse {
        user_id: created.id.unwrap().to_string(),
        email: payload.email,
        message: "User registered successfully".into(),
    }))
}
```

### 4. Función de hashing

```rust
use argon2::{Argon2, PasswordHasher, password_hash::SaltString};
use rand::rngs::OsRng;

fn hash_password(password: &str) -> Result<String, AuthError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2.hash_password(password.as_bytes(), &salt)
        .map_err(|_| AuthError::InternalError)?
        .to_string();
    Ok(hash)
}
```

## Criterios de Aceptación
- [ ] Endpoint `/api/v1/auth/register` funciona
- [ ] Email único validado (409 si duplicado)
- [ ] Password hasheado con Argon2
- [ ] Usuario creado en BD
- [ ] Respuesta incluye user_id

## Tests Requeridos
```rust
#[tokio::test]
async fn test_register_success() {
    // Registrar nuevo usuario, verificar respuesta
}

#[tokio::test]
async fn test_register_duplicate_email() {
    // Intentar registrar mismo email, verificar 409
}

#[tokio::test]
async fn test_register_weak_password() {
    // Verificar que passwords cortos son rechazados
}
```
