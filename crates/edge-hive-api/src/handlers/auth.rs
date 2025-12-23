//! Authentication handlers

use argon2::{Argon2, PasswordHasher, password_hash::{PasswordHash, SaltString}};
use axum::{
    extract::Extension,
    http::StatusCode,
    response::Json,
};
use edge_hive_db::StoredUser;
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use validator::Validate;

use crate::state::ApiState;


#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Internal server error")]
    InternalError,
    #[error("Database error: {0}")]
    DatabaseError(#[from] surrealdb::Error),
}

impl From<AuthError> for (StatusCode, Json<ErrorResponse>) {
    fn from(err: AuthError) -> Self {
        let status = match err {
            AuthError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            AuthError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, Json(ErrorResponse { error: err.to_string() }))
    }
}

#[derive(Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
    pub name: Option<String>,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub user_id: String,
    pub email: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    provider: String,
    code: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    access_token: String,
    refresh_token: String,
    expires_in: u64,
}

#[derive(Deserialize)]
pub struct RefreshRequest {
    refresh_token: String,
}

/// OAuth2 login handler
pub async fn login(
    Extension(_state): Extension<ApiState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    // Placeholder: would integrate with edge-hive-auth
    Ok(Json(LoginResponse {
        access_token: "placeholder_access_token".to_string(),
        refresh_token: "placeholder_refresh_token".to_string(),
        expires_in: 3600,
    }))
}

/// Refresh token handler
pub async fn refresh_token(
    Extension(_state): Extension<ApiState>,
    Json(payload): Json<RefreshRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    // Placeholder: would validate and refresh token
    Ok(Json(LoginResponse {
        access_token: "new_access_token".to_string(),
        refresh_token: "new_refresh_token".to_string(),
        expires_in: 3600,
    }))
}

/// Logout handler
pub async fn logout(Extension(_state): Extension<ApiState>) -> StatusCode {
    // Placeholder: would invalidate session
    StatusCode::OK
}

/// Register a new user
pub async fn register(
    Extension(state): Extension<ApiState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<RegisterResponse>, (StatusCode, Json<ErrorResponse>)> {
    if let Err(e) = payload.validate() {
        return Err((StatusCode::BAD_REQUEST, Json(ErrorResponse {
            error: e.to_string(),
        })));
    }

    if state.db.get_user_by_email(&payload.email).await.map_err(AuthError::from)?.is_some() {
        return Err((StatusCode::CONFLICT, Json(ErrorResponse {
            error: "Email already registered".into()
        })));
    }

    let password_hash = hash_password(&payload.password)?;

    let user = StoredUser {
        id: None,
        email: payload.email.clone(),
        password_hash: Some(password_hash),
        name: payload.name,
        provider: Some("local".into()),
        role: "user".into(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        last_login: None,
        email_verified: false,
    };

    let created = state.db.create_user(&user).await.map_err(AuthError::from)?;

    Ok(Json(RegisterResponse {
        user_id: created.id.unwrap().to_string(),
        email: payload.email,
        message: "User registered successfully".into(),
    }))
}

fn hash_password(password: &str) -> Result<String, AuthError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2.hash_password(password.as_bytes(), &salt)
        .map_err(|_| AuthError::InternalError)?
        .to_string();
    Ok(hash)
}
