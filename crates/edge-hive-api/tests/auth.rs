use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware,
    routing::get,
    Router,
};
use edge_hive_api::{middleware::auth::auth_middleware, state::ApiState};
use edge_hive_auth::jwt::{JwtClaims, TokenGenerator, TokenValidator};
use edge_hive_db::{DatabaseService, StoredUser};
use std::sync::Arc;
use tempfile::tempdir;
use tower::ServiceExt;

async fn setup_test_env() -> (ApiState, TokenGenerator) {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("test.db");
    let db = Arc::new(DatabaseService::new(&db_path).await.unwrap());

    let secret = b"secret";
    let issuer = "test".to_string();

    let token_generator = TokenGenerator::new(secret, issuer.clone());
    let token_validator = TokenValidator::new(secret, issuer);
    let state = ApiState::new_minimal(
        edge_hive_cache::CacheService::new(Default::default()).await,
        db,
        Arc::new(token_validator),
        dir.path().to_path_buf(),
    );
    (state, token_generator)
}

#[tokio::test]
async fn test_auth_middleware_valid_token() {
    let (state, token_generator) = setup_test_env().await;
    let user_id = "test_user".to_string();

    let user = StoredUser {
        id: user_id.clone(),
        email: "test@test.com".to_string(),
        name: None,
        created_at: chrono::Utc::now().into(),
        updated_at: chrono::Utc::now().into(),
        deleted_at: None,
    };
    state.db.save_user(&user).await.unwrap();

    let claims = JwtClaims {
        sub: user_id,
        iss: "test".to_string(),
        aud: "mcp".to_string(),
        exp: (chrono::Utc::now() + chrono::Duration::minutes(5)).timestamp(),
        ..Default::default()
    };
    let token = token_generator
        .generate_token_from_claims(&claims)
        .unwrap();

    let app = Router::new()
        .route("/", get(|| async { "OK" }))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    let response = app
        .oneshot(
            Request::builder()
                .uri("/")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_auth_middleware_deleted_user() {
    let (state, token_generator) = setup_test_env().await;
    let user_id = "deleted_user".to_string();

    let user = StoredUser {
        id: user_id.clone(),
        email: "deleted@test.com".to_string(),
        name: None,
        created_at: chrono::Utc::now().into(),
        updated_at: chrono::Utc::now().into(),
        deleted_at: Some(chrono::Utc::now().into()),
    };
    state.db.save_user(&user).await.unwrap();

    let claims = JwtClaims {
        sub: user_id,
        iss: "test".to_string(),
        aud: "mcp".to_string(),
        exp: (chrono::Utc::now() + chrono::Duration::minutes(5)).timestamp(),
        ..Default::default()
    };
    let token = token_generator
        .generate_token_from_claims(&claims)
        .unwrap();

    let app = Router::new()
        .route("/", get(|| async { "OK" }))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    let response = app
        .oneshot(
            Request::builder()
                .uri("/")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_auth_middleware_expired_token() {
    let (state, token_generator) = setup_test_env().await;
    let user_id = "test_user_expired".to_string();

    let user = StoredUser {
        id: user_id.clone(),
        email: "expired@test.com".to_string(),
        name: None,
        created_at: chrono::Utc::now().into(),
        updated_at: chrono::Utc::now().into(),
        deleted_at: None,
    };
    state.db.save_user(&user).await.unwrap();

    let claims = JwtClaims {
        sub: user_id,
        iss: "test".to_string(),
        aud: "mcp".to_string(),
        exp: (chrono::Utc::now() - chrono::Duration::minutes(5)).timestamp(),
        ..Default::default()
    };
    let token = token_generator
        .generate_token_from_claims(&claims)
        .unwrap();

    let app = Router::new()
        .route("/", get(|| async { "OK" }))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    let response = app
        .oneshot(
            Request::builder()
                .uri("/")
                .header("Authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
