---
title: "Conectar auth middleware con validación de BD"
labels:
  - enhancement
  - auth
  - security
  - jules
assignees: []
priority: high
---

## Descripción
Actualizar el middleware de autenticación para validar sesiones contra la BD, permitiendo invalidación de tokens y logout real.

## Contexto
Actualmente el middleware solo valida JWT signature. Necesitamos:
- Verificar que la sesión no esté revocada
- Permitir "logout everywhere"
- Denegar tokens de usuarios eliminados

## Archivos a Modificar
- `crates/edge-hive-auth/src/middleware.rs`
- `crates/edge-hive-api/src/middleware.rs`

## Dependencias
- **Requiere:** `FEAT_session-storage-db` completado

## Implementación Requerida

### 1. Actualizar middleware para verificar sesión en BD

```rust
pub async fn auth_middleware<B>(
    State(state): State<ApiState>,
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // 1. Validar JWT signature
    let claims = state.token_validator
        .validate_bearer_token(auth_header)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // 2. Verificar que el usuario existe y no está eliminado
    let user = state.db.get_user_by_id(&claims.sub).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // 3. (Opcional) Verificar que el token no está en blacklist
    // Esto es útil para logout individual de access tokens

    // 4. Agregar claims al request para uso en handlers
    request.extensions_mut().insert(claims);
    request.extensions_mut().insert(user);

    Ok(next.run(request).await)
}
```

### 2. Crear extractor de usuario autenticado

```rust
pub struct AuthenticatedUser(pub StoredUser);

#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<StoredUser>()
            .cloned()
            .map(AuthenticatedUser)
            .ok_or(StatusCode::UNAUTHORIZED)
    }
}
```

### 3. Uso en handlers

```rust
pub async fn get_profile(
    AuthenticatedUser(user): AuthenticatedUser,
) -> Json<UserProfile> {
    Json(UserProfile {
        email: user.email,
        name: user.name,
        // ...
    })
}
```

## Criterios de Aceptación
- [ ] Middleware valida JWT + verifica usuario en BD
- [ ] Usuarios eliminados no pueden usar tokens existentes
- [ ] `AuthenticatedUser` extractor funciona
- [ ] Tests de seguridad pasan

## Tests Requeridos
```rust
#[tokio::test]
async fn test_auth_middleware_valid_token() {
    // Verificar acceso con token válido
}

#[tokio::test]
async fn test_auth_middleware_deleted_user() {
    // Crear user, generar token, eliminar user, verificar 401
}

#[tokio::test]
async fn test_auth_middleware_expired_token() {
    // Verificar rechazo de token expirado
}
```
