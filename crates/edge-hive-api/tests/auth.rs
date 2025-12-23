use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use edge_hive_api::{create_router, ApiState};
use edge_hive_cache::CacheConfig;
use serde_json::json;
use axum::body;
use edge_hive_api::handlers::auth::RegisterResponse;
use tempfile::tempdir;
use tower::ServiceExt;

async fn setup_test_env() -> ApiState {
    let cache = edge_hive_cache::CacheService::new(CacheConfig::default()).await;
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("test.db");
    let db = std::sync::Arc::new(edge_hive_db::DatabaseService::new(&db_path).await.unwrap());
    ApiState::new_minimal(cache, db, dir.path().to_path_buf())
}

#[tokio::test]
async fn test_register_success() {
    let state = setup_test_env().await;
    let app = create_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/api/v1/auth/register")
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "email": "test@test.com",
                        "password": "password123",
                        "name": "Test User"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = body::to_bytes(response.into_body()).await.unwrap();
    let res: RegisterResponse = serde_json::from_slice(&body).unwrap();
    assert_eq!(res.email, "test@test.com");
    assert_eq!(res.message, "User registered successfully");
}

#[tokio::test]
async fn test_register_duplicate_email() {
    let state = setup_test_env().await;
    let app = create_router(state.clone());

    // First registration
    let _ = app
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/api/v1/auth/register")
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "email": "test@test.com",
                        "password": "password123",
                        "name": "Test User"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    // Second registration with the same email
    let app2 = create_router(state);
    let response = app2
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/api/v1/auth/register")
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "email": "test@test.com",
                        "password": "password123",
                        "name": "Test User"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CONFLICT);
}

#[tokio::test]
async fn test_register_weak_password() {
    let state = setup_test_env().await;
    let app = create_router(state);

    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/api/v1/auth/register")
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "email": "test@test.com",
                        "password": "123",
                        "name": "Test User"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
