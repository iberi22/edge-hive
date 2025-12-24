//! MCP OAuth2 Token Handler

use crate::state::ApiState;
use axum::{extract::Extension, http::StatusCode, response::Json};
use edge_hive_auth::{TokenRequest, TokenResponse};
use serde::Serialize;
use tracing::warn;

#[derive(Serialize, Debug)]
pub struct ErrorResponse {
    error: String,
    error_description: String,
}

/// OAuth2 Client Credentials Grant handler for MCP clients
pub async fn issue_mcp_token(
    Extension(state): Extension<ApiState>,
    Json(payload): Json<TokenRequest>,
) -> Result<Json<TokenResponse>, (StatusCode, Json<ErrorResponse>)> {
    // 1. Validate grant_type
    if let Err(e) = payload.validate() {
        warn!("Invalid grant type request: {}", e);
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "unsupported_grant_type".into(),
                error_description: "Only 'client_credentials' grant type is supported".into(),
            }),
        ));
    }

    // 2. Find client by client_id
    let client = state
        .db
        .get_client_by_id(&payload.client_id)
        .await
        .map_err(|e| {
            warn!("Database error while fetching client: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "server_error".into(),
                    error_description: "Database operation failed".into(),
                }),
            )
        })?
        .ok_or_else(|| {
            warn!("Client not found: {}", payload.client_id);
            (
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    error: "invalid_client".into(),
                    error_description: "Client authentication failed".into(),
                }),
            )
        })?;

    // 3. Verify client secret
    if !client.verify_secret(&payload.client_secret) {
        warn!("Invalid secret for client: {}", payload.client_id);
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                error: "invalid_client".into(),
                error_description: "Client authentication failed".into(),
            }),
        ));
    }

    // 4. Generate JWT
    let token = state
        .token_generator
        .generate_token(
            client.client_id,
            client.scopes.clone(),
            Some(state.config.oauth2.token_expiration_secs),
        )
        .map_err(|e| {
            warn!("Token generation failed: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "server_error".into(),
                    error_description: "Could not generate access token".into(),
                }),
            )
        })?;

    // 5. Create response
    let response = TokenResponse::new(
        token,
        state.config.oauth2.token_expiration_secs,
        client.scopes,
    );

    Ok(Json(response))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::ApiState;
    use axum::http::StatusCode;
    use edge_hive_auth::{ClientCredentials, TokenRequest};
    use edge_hive_cache::{CacheConfig, CacheService};
    use edge_hive_db::DatabaseService;
    use std::{path::PathBuf, sync::Arc};
    use tempfile::tempdir;

    async fn setup_test_state() -> ApiState {
        let db_dir = tempdir().unwrap();
        let db = Arc::new(
            DatabaseService::new(&db_dir.path().join("test.db"))
                .await
                .unwrap(),
        );
        let cache = CacheService::new(CacheConfig::default()).await;
        let data_dir = PathBuf::from(tempdir().unwrap().path());
        ApiState::new(cache, db, data_dir)
    }

    async fn create_test_client(
        db: &DatabaseService,
        client_id: &str,
        client_secret: &str,
        scopes: Vec<String>,
    ) {
        let client = ClientCredentials::new(
            client_id.to_string(),
            client_secret,
            scopes,
            "Test MCP Client".to_string(),
        );
        db.create_client(&client).await.unwrap();
    }

    #[tokio::test]
    async fn test_issue_mcp_token_success() {
        let state = setup_test_state().await;
        let client_id = "test_client_1";
        let client_secret = "test_secret_123";
        let scopes = vec!["mcp:read".to_string(), "mcp:call".to_string()];
        create_test_client(&state.db, client_id, client_secret, scopes.clone()).await;

        let payload = TokenRequest {
            grant_type: "client_credentials".to_string(),
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            scope: None,
        };

        let result = issue_mcp_token(Extension(state), Json(payload)).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(!response.0.access_token.is_empty());
        assert_eq!(response.0.token_type, "Bearer");
        assert_eq!(response.0.scope, Some(scopes.join(" ")));
    }

    #[tokio::test]
    async fn test_issue_mcp_token_invalid_secret() {
        let state = setup_test_state().await;
        let client_id = "test_client_2";
        let client_secret = "test_secret_123";
        create_test_client(&state.db, client_id, client_secret, vec![]).await;

        let payload = TokenRequest {
            grant_type: "client_credentials".to_string(),
            client_id: client_id.to_string(),
            client_secret: "wrong_secret".to_string(),
            scope: None,
        };

        let result = issue_mcp_token(Extension(state), Json(payload)).await;
        assert!(result.is_err());

        let (status, error) = result.unwrap_err();
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        assert_eq!(error.0.error, "invalid_client");
    }
}
