---
title: "Implementar handler de login real en API Gateway"
labels:
  - enhancement
  - auth
  - api
  - jules
assignees: []
priority: critical
---

## Descripción
Reemplazar el handler placeholder de login con una implementación real que valide credenciales contra la BD y genere JWT tokens.

## Contexto
Actualmente `login()` en `edge-hive-api/src/handlers/auth.rs` retorna tokens hardcoded. Necesitamos:
1. Validar email/password contra `users` table
2. Generar JWT access token
3. Crear sesión con refresh token
4. Retornar ambos tokens

## Archivos a Modificar
- `crates/edge-hive-api/src/handlers/auth.rs`
- `crates/edge-hive-api/src/state.rs` (agregar TokenGenerator)

## Dependencias
- **Requiere:** `FEAT_user-schema-surrealdb` completado
- **Requiere:** `FEAT_session-storage-db` completado

## Implementación Requerida

### 1. Actualizar `LoginRequest` para soportar múltiples flujos

```rust
#[derive(Deserialize)]
#[serde(untagged)]
pub enum LoginRequest {
    /// Email + Password login
    Credentials {
        email: String,
        password: String,
    },
    /// OAuth2 callback
    OAuth {
        provider: String,  // "github", "google"
        code: String,
    },
}
```

### 2. Implementar handler de login real

```rust
pub async fn login(
    Extension(state): Extension<ApiState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, Json<ErrorResponse>)> {
    match payload {
        LoginRequest::Credentials { email, password } => {
            // 1. Buscar usuario por email
            let user = state.db.get_user_by_email(&email).await
                .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: "DB error".into() })))?
                .ok_or((StatusCode::UNAUTHORIZED, Json(ErrorResponse { error: "Invalid credentials".into() })))?;

            // 2. Verificar password
            if !verify_password(&password, &user.password_hash) {
                return Err((StatusCode::UNAUTHORIZED, Json(ErrorResponse { error: "Invalid credentials".into() })));
            }

            // 3. Generar access token
            let access_token = state.token_generator.generate_token(
                user.id.clone().unwrap().to_string(),
                vec!["user:read".into(), "user:write".into()],
                None,
            ).map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: "Token error".into() })))?;

            // 4. Generar refresh token y guardar sesión
            let refresh_token = generate_refresh_token();
            let session = StoredSession {
                user_id: user.id.clone().unwrap(),
                refresh_token_hash: hash_token(&refresh_token),
                expires_at: Utc::now() + Duration::days(30),
                // ...
            };
            state.db.create_session(&session).await.map_err(|_| ...)?;

            Ok(Json(LoginResponse {
                access_token,
                refresh_token,
                expires_in: 3600,
                user: Some(UserInfo { email: user.email, name: user.name }),
            }))
        }
        LoginRequest::OAuth { provider, code } => {
            // TODO: Implementar OAuth flow
            Err((StatusCode::NOT_IMPLEMENTED, Json(ErrorResponse { error: "OAuth not implemented".into() })))
        }
    }
}
```

### 3. Agregar función de verificación de password

```rust
use argon2::{Argon2, PasswordHash, PasswordVerifier};

fn verify_password(password: &str, hash: &str) -> bool {
    let parsed_hash = PasswordHash::new(hash).ok()?;
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok()
}
```

### 4. Actualizar `ApiState` para incluir `TokenGenerator`

```rust
// En state.rs
pub struct ApiState {
    pub db: Arc<DatabaseService>,
    pub cache: CacheService,
    pub token_generator: TokenGenerator,
    pub token_validator: TokenValidator,
    // ...
}
```

## Criterios de Aceptación
- [ ] Login con email/password funciona
- [ ] Password se verifica con Argon2
- [ ] Access token JWT válido generado
- [ ] Refresh token almacenado en BD
- [ ] Respuesta incluye info básica del usuario
- [ ] Errores devuelven códigos HTTP apropiados (401, 500)

## Tests Requeridos
```rust
#[tokio::test]
async fn test_login_success() {
    // Crear usuario, intentar login, verificar tokens
}

#[tokio::test]
async fn test_login_wrong_password() {
    // Verificar que devuelve 401
}

#[tokio::test]
async fn test_login_nonexistent_user() {
    // Verificar que devuelve 401
}
```

## Dependencias de Cargo
Agregar a `crates/edge-hive-api/Cargo.toml`:
```toml
argon2 = "0.5"
```
