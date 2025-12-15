#[cfg(feature = "server")]
use axum::{
    async_trait,
    extract::{FromRequestParts, Request},
    http::{request::Parts, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::sync::Arc;
use crate::{error::AuthError, jwt::{JwtClaims, TokenValidator}};

/// Extractor for validated JWT claims
#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub claims: JwtClaims,
}

#[cfg(feature = "server")]
#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Get Authorization header
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .ok_or(AuthError::MissingAuthHeader)?;

        // Extract validator from extensions (set by AuthLayer)
        let validator = parts
            .extensions
            .get::<Arc<TokenValidator>>()
            .ok_or_else(|| AuthError::Internal("TokenValidator not found in extensions".to_string()))?;

        // Validate Bearer token
        let claims = validator.validate_bearer_token(auth_header)?;

        Ok(AuthenticatedUser { claims })
    }
}

/// Authorization layer for Axum
#[cfg(feature = "server")]
#[derive(Clone)]
pub struct AuthLayer {
    validator: Arc<TokenValidator>,
}

#[cfg(feature = "server")]
impl AuthLayer {
    pub fn new(validator: TokenValidator) -> Self {
        Self {
            validator: Arc::new(validator),
        }
    }

    /// Middleware function to inject validator into request extensions
    pub async fn middleware(
        mut request: Request,
        next: Next,
    ) -> Response {
        // This middleware is applied via tower::ServiceBuilder
        next.run(request).await
    }
}

/// Require specific scopes middleware
#[cfg(feature = "server")]
pub struct RequireScopes {
    scopes: Vec<String>,
}

#[cfg(feature = "server")]
impl RequireScopes {
    pub fn new(scopes: Vec<String>) -> Self {
        Self { scopes }
    }

    pub async fn check(
        user: AuthenticatedUser,
        required_scopes: Vec<String>,
    ) -> Result<AuthenticatedUser, AuthError> {
        if !user.claims.has_all_scopes(&required_scopes) {
            return Err(AuthError::InsufficientPermissions(
                required_scopes.join(", ")
            ));
        }
        Ok(user)
    }
}

/// Convert AuthError to HTTP response
#[cfg(feature = "server")]
impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "Invalid credentials"),
            AuthError::InvalidToken(_) => (StatusCode::UNAUTHORIZED, "Invalid token"),
            AuthError::TokenExpired => (StatusCode::UNAUTHORIZED, "Token expired"),
            AuthError::MissingAuthHeader => (StatusCode::UNAUTHORIZED, "Missing authorization header"),
            AuthError::InvalidAuthHeader => (StatusCode::UNAUTHORIZED, "Invalid authorization header format"),
            AuthError::InsufficientPermissions(_) => (StatusCode::FORBIDDEN, "Insufficient permissions"),
            AuthError::ClientNotFound(_) => (StatusCode::NOT_FOUND, "Client not found"),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
        };

        let body = serde_json::json!({
            "error": message,
            "details": self.to_string(),
        });

        (status, axum::Json(body)).into_response()
    }
}
