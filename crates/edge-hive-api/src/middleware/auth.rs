//! Authentication middleware for JWT validation

use axum::{
    body::Body,
    http::{Request, Response, StatusCode},
    middleware::Next,
};

/// Auth middleware (placeholder for future implementation)
///
/// This middleware will:
/// 1. Extract JWT from Authorization header
/// 2. Validate token with edge-hive-auth
/// 3. Inject user context into request extensions
pub async fn auth_middleware(
    request: Request<Body>,
    next: Next,
) -> Result<Response<Body>, StatusCode> {
    // Placeholder: would validate JWT
    // For now, just pass through
    Ok(next.run(request).await)
}
