//! Authentication handlers

use crate::state::ApiState;
use axum::{extract::Extension, http::StatusCode, response::Json};
use chrono::{Duration, Utc};
use edge_hive_db::session::StoredSession;
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

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
        provider: String, // "github", "google"
        code: String,
    },
}

#[derive(Serialize, Debug)]
pub struct UserInfo {
    pub email: String,
    pub name: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct LoginResponse {
    access_token: String,
    refresh_token: String,
    expires_in: i64,
    user: Option<UserInfo>,
}

#[derive(Serialize, Debug)]
pub struct ErrorResponse {
    error: String,
}

#[derive(Deserialize)]
pub struct RefreshRequest {
    refresh_token: String,
}

/// OAuth2 login handler
pub async fn login(
    Extension(state): Extension<ApiState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, Json<ErrorResponse>)> {
    match payload {
        LoginRequest::Credentials { email, password } => {
            // 1. Buscar usuario por email
            let user = state
                .db
                .get_user_by_email(&email)
                .await
                .map_err(|_| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse {
                            error: "DB error".into(),
                        }),
                    )
                })?
                .ok_or((
                    StatusCode::UNAUTHORIZED,
                    Json(ErrorResponse {
                        error: "Invalid credentials".into(),
                    }),
                ))?;

            // 2. Verificar password
            if !verify_password(&password, &user.password_hash) {
                return Err((
                    StatusCode::UNAUTHORIZED,
                    Json(ErrorResponse {
                        error: "Invalid credentials".into(),
                    }),
                ));
            }

            // 3. Generar access token
            let access_token = state
                .token_generator
                .generate_token(
                    user.id.clone().unwrap().to_string(),
                    vec!["user:read".into(), "user:write".into()],
                    None,
                )
                .map_err(|_| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse {
                            error: "Token error".into(),
                        }),
                    )
                })?;

            // 4. Generar refresh token y guardar sesión
            let refresh_token = generate_refresh_token();
            let session = StoredSession {
                id: None,
                user_id: user.id.clone().unwrap(),
                refresh_token_hash: hash_token(&refresh_token),
                expires_at: (Utc::now() + Duration::days(30)).into(),
                created_at: Utc::now().into(),
                updated_at: Utc::now().into(),
            };
            state.db.create_session(&session).await.map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: "DB error".into(),
                    }),
                )
            })?;

            Ok(Json(LoginResponse {
                access_token,
                refresh_token,
                expires_in: 3600,
                user: Some(UserInfo {
                    email: user.email,
                    name: user.name,
                }),
            }))
        }
        LoginRequest::OAuth { .. } => {
            // TODO: Implementar OAuth flow
            Err((
                StatusCode::NOT_IMPLEMENTED,
                Json(ErrorResponse {
                    error: "OAuth not implemented".into(),
                }),
            ))
        }
    }
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
        user: None,
    }))
}

/// Logout handler
pub async fn logout(Extension(_state): Extension<ApiState>) -> StatusCode {
    // Placeholder: would invalidate session
    StatusCode::OK
}

use argon2::{Argon2, PasswordHash, PasswordVerifier};

fn verify_password(password: &str, hash: &str) -> bool {
    if let Ok(parsed_hash) = PasswordHash::new(hash) {
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
    } else {
        false
    }
}

fn generate_refresh_token() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(64)
        .map(char::from)
        .collect()
}

fn hash_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::ApiState;
    use axum::http::StatusCode;
    use edge_hive_cache::{CacheConfig, CacheService};
    use edge_hive_db::{
        user::{HashedPassword, StoredUser},
        DatabaseService,
    };
    use std::{path::PathBuf, sync::Arc};
    use tempfile::tempdir;

    async fn setup_test_state() -> ApiState {
        let db_dir = tempdir().unwrap();
        let db = Arc::new(
            DatabaseService::new(&db_dir.path().join("test.db"))
                .await
                .unwrap(),
        );
        let cache = CacheService::new(CacheConfig {
            l1_max_capacity: 100,
            l1_ttl_secs: 60,
            l2_enabled: false,
            l2_host: "127.0.0.1".to_string(),
            l2_port: 6379,
            metrics_enabled: false,
        })
        .await;
        let data_dir = PathBuf::from(tempdir().unwrap().path());
        ApiState::new_minimal(cache, db, data_dir)
    }

    async fn create_test_user(db: &DatabaseService, email: &str, password: &str) {
        let user = StoredUser {
            id: None,
            email: email.to_string(),
            name: Some("Test User".to_string()),
            password_hash: HashedPassword::new(password).unwrap().to_string(),
            created_at: Utc::now().into(),
            updated_at: Utc::now().into(),
        };
        db.create_user(&user).await.unwrap();
    }

    #[tokio::test]
    async fn test_login_success() {
        let state = setup_test_state().await;
        create_test_user(&state.db, "test@example.com", "password123").await;

        let payload = LoginRequest::Credentials {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };

        let result = login(Extension(state), Json(payload)).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(!response.0.access_token.is_empty());
        assert!(!response.0.refresh_token.is_empty());
        assert_eq!(response.0.user.as_ref().unwrap().email, "test@example.com");
    }

    #[tokio::test]
    async fn test_login_wrong_password() {
        let state = setup_test_state().await;
        create_test_user(&state.db, "test@example.com", "password123").await;

        let payload = LoginRequest::Credentials {
            email: "test@example.com".to_string(),
            password: "wrongpassword".to_string(),
        };

        let result = login(Extension(state), Json(payload)).await;
        assert!(result.is_err());

        let (status, error) = result.unwrap_err();
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        assert_eq!(error.0.error, "Invalid credentials");
    }

    #[tokio::test]
    async fn test_login_nonexistent_user() {
        let state = setup_test_state().await;

        let payload = LoginRequest::Credentials {
            email: "nonexistent@example.com".to_string(),
            password: "password123".to_string(),
        };

        let result = login(Extension(state), Json(payload)).await;
        assert!(result.is_err());

        let (status, error) = result.unwrap_err();
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        assert_eq!(error.0.error, "Invalid credentials");
    }
}
