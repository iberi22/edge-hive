//! Authentication handlers

use axum::{
    extract::{Extension, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use crate::state::ApiState;
use edge_hive_wasm::{PluginManager, WasmError};
use std::sync::Arc;
use serde_json::json;

#[derive(Serialize)]
pub struct ErrorResponse {
    error: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    // Ignoring password for now as we are focusing on edge function
    // password: String,
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
    State(state): State<Arc<ApiState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Before validating password, execute edge function if it exists
    if let Some(plugin_manager) = &state.plugin_manager {
        let mut plugins = plugin_manager.lock().await;
        if let Some(validator) = plugins.plugins().iter_mut().find(|p| p.info().name == "login_validator") {
            let email = payload.email.as_bytes();
            let email_len = email.len() as i32;

            // Allocate memory in the WASM module
            let allocate_result = validator.call("allocate", &[wasmtime::Val::I32(email_len)]);
            let email_ptr = match allocate_result {
                Ok(mut results) => match results.pop() {
                    Some(wasmtime::Val::I32(ptr)) => ptr,
                    _ => return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: "Failed to allocate memory in WASM module".into() }))),
                },
                Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("Error executing allocate function: {}", e) }))),
            };

            // Write the email to the WASM module's memory
            let memory = validator.instance().get_memory(&mut validator.store_mut(), "memory").unwrap();
            memory.write(&mut validator.store_mut(), email_ptr as usize, email).unwrap();

            // Call the validation function
            let result = validator.call("validate_login", &[wasmtime::Val::I32(email_ptr), wasmtime::Val::I32(email_len)]);

            // Deallocate the memory
            let _ = validator.call("deallocate", &[wasmtime::Val::I32(email_ptr), wastime::Val::I32(email_len)]);

            match result {
                Ok(results) => {
                    if let Some(wasmtime::Val::I32(0)) = results.get(0) {
                        return Err((StatusCode::FORBIDDEN, Json(ErrorResponse {
                            error: "Login blocked by custom rule".into()
                        })));
                    }
                }
                Err(e) => {
                    // Log the error and deny login just in case
                    return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse {
                        error: format!("Error executing edge function: {}", e),
                    })));
                }
            }
        }
    }

    // Continue with normal login...
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::ApiState;
    use axum::extract::State;
    use edge-hive_cache::CacheService;
    use edge_hive_db::DatabaseService;
    use edge-hive_wasm::PluginManager;
    use std::path::PathBuf;
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use wat::parse_str;

    async fn setup_test_env() -> Arc<ApiState> {
        let db = Arc::new(
            DatabaseService::new(std::path::Path::new("memory://"))
                .await
                .unwrap(),
        );
        let cache = CacheService::new_in_memory();

        let wat = r#"
        (module
            (memory (export "memory") 1)
            (func (export "allocate") (param i32) (result i32)
                i32.const 0
            )
            (func (export "deallocate") (param i32 i32))
            (func (export "validate_login") (param i32 i32) (result i32)
                get_local 0
                i32.load8_u
                i32.const 64
                i32.eq
            )
        )
        "#;

        let wasm_bytes = parse_str(wat).unwrap();
        let mut plugin_manager = PluginManager::new();
        plugin_manager.load_from_bytes(&wasm_bytes, "login_validator").unwrap();
        let plugin_manager = Arc::new(Mutex::new(plugin_manager));

        Arc::new(ApiState::new(
            cache,
            db,
            edge_hive_realtime::RealtimeServer::new(Default::default()),
            PathBuf::from("/tmp"),
            Some(plugin_manager),
        ))
    }

    #[tokio::test]
    async fn test_login_with_edge_validation_allowed() {
        let state = setup_test_env().await;

        let payload = LoginRequest {
            email: "test@mycompany.com".to_string(),
        };

        let result = login(State(state.clone()), Json(payload)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_login_with_edge_validation_blocked() {
        let (state, _temp_dir) = setup_test_env().await;

        let payload = LoginRequest {
            email: "test@otherdomain.com".to_string(),
        };

        let result = login(State(state.clone()), Json(payload)).await;
        assert!(result.is_err());
        if let Err((status, _)) = result {
            assert_eq!(status, StatusCode::FORBIDDEN);
        }
    }
}
