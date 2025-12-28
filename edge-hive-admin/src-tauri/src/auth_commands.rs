use tauri::State;
use edge_hive_auth::TokenGenerator;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub provider: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: User,
}

pub struct AuthState {
    pub token_gen: TokenGenerator,
}

impl AuthState {
    pub fn new() -> Self {
        // In a real app we would load keys from disk or env
        // Using arbitrary secret for now
        // Or if JwtKeys::generate_secret() exists:
        // let secret = JwtKeys::generate_secret();
        // But simpler to just use a fixed buffer since this is a stub/demo
        let secret = "stub_secret_key_change_me".as_bytes();
        Self {
            token_gen: TokenGenerator::new(secret, "edge-hive-admin".to_string()),
        }
    }
}

#[tauri::command]
pub async fn login(
    _state: State<'_, AuthState>,
    email: String,
    _password: String,
) -> Result<AuthResponse, String> {
    // Stub implementation - accepts any password
    // In real implementation: verify password hash from DB

    let user = User {
        id: "user_123".to_string(),
        email: email.clone(),
        provider: "local".to_string(),
    };

    // Generate real JWT
    // let token = state.token_gen.generate(&user.id).map_err(|e| e.to_string())?;
    let token = "mock_jwt_token_signed".to_string();

    Ok(AuthResponse { token, user })
}

#[tauri::command]
pub async fn register(
    _state: State<'_, AuthState>,
    email: String,
    _password: String,
) -> Result<AuthResponse, String> {
    // Stub implementation
    let user = User {
        id: "user_new_456".to_string(),
        email,
        provider: "local".to_string(),
    };

    let token = "mock_jwt_token_registered".to_string();
    Ok(AuthResponse { token, user })
}

#[tauri::command]
pub async fn get_current_user() -> Result<User, String> {
    // Stub - usually we'd parse the token from headers/state
    Ok(User {
        id: "user_123".to_string(),
        email: "admin@localhost".to_string(),
        provider: "local".to_string(),
    })
}
