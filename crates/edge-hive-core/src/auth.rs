use axum::{
    extract::Extension,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use edge_hive_auth::{
    oauth2::{TokenRequest, TokenResponse, ClientCredentials},
    jwt::TokenGenerator,
    client::ClientStore,
    error::AuthError,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::path::{Path, PathBuf};

/// Shared OAuth2 state
#[derive(Clone)]
pub struct OAuth2State {
    pub token_generator: Arc<TokenGenerator>,
    pub client_store: Arc<ClientStore>,
    pub jwt_secret: Vec<u8>,
    pub issuer: String,
    pub clients_file: PathBuf,
}

impl OAuth2State {
    pub async fn load_or_new(jwt_secret: &[u8], issuer: String, data_dir: &Path) -> anyhow::Result<Self> {
        let client_store = Arc::new(ClientStore::new());
        let clients_file = data_dir.join("oauth_clients.json");

        if clients_file.exists() {
            let data = std::fs::read_to_string(&clients_file)?;
            let clients: Vec<ClientCredentials> = serde_json::from_str(&data)?;
            for client in clients {
                // Best-effort load; ignore duplicates.
                let _ = client_store.add_client(client).await;
            }
        }

        Ok(Self {
            token_generator: Arc::new(TokenGenerator::new(jwt_secret, issuer.clone())),
            client_store,
            jwt_secret: jwt_secret.to_vec(),
            issuer,
            clients_file,
        })
    }

    pub async fn refresh_clients_from_disk(&self) -> Result<(), AuthError> {
        if !self.clients_file.exists() {
            return Ok(());
        }

        let data = std::fs::read_to_string(&self.clients_file)
            .map_err(|e| AuthError::Internal(format!("Failed to read clients file: {e}")))?;

        let clients: Vec<ClientCredentials> = serde_json::from_str(&data)
            .map_err(|e| AuthError::Internal(format!("Failed to parse clients file: {e}")))?;

        for client in clients {
            // Best-effort load; ignore duplicates.
            let _ = self.client_store.add_client(client).await;
        }

        Ok(())
    }
}

async fn save_clients(store: &ClientStore, clients_file: &Path) -> Result<(), AuthError> {
    let clients = store.list_clients().await?;
    let json = serde_json::to_string_pretty(&clients)
        .map_err(|e| AuthError::Internal(format!("Failed to serialize clients: {e}")))?;
    std::fs::write(clients_file, json)
        .map_err(|e| AuthError::Internal(format!("Failed to write clients file: {e}")))?;
    Ok(())
}

/// POST /mcp/auth/token - OAuth2 Client Credentials token endpoint
pub async fn token_endpoint(
    Extension(state): Extension<OAuth2State>,
    Json(request): Json<TokenRequest>,
) -> Result<Json<TokenResponse>, AuthError> {
    // Validate grant type
    request.validate()?;

    // Reload clients so CLI-created clients are recognized without restarting the server.
    state.refresh_clients_from_disk().await?;

    // Verify client credentials
    let client = state
        .client_store
        .verify_credentials(&request.client_id, &request.client_secret)
        .await?;

    // Determine granted scopes (intersection of requested and allowed)
    let requested_scopes = request.requested_scopes();
    let granted_scopes: Vec<String> = if requested_scopes.is_empty() {
        // If no scopes requested, grant all client scopes
        client.scopes.clone()
    } else {
        // Grant only requested scopes that client has permission for
        requested_scopes
            .into_iter()
            .filter(|scope| client.scopes.contains(scope))
            .collect()
    };

    if granted_scopes.is_empty() {
        return Err(AuthError::InsufficientPermissions(
            "No valid scopes granted".to_string()
        ));
    }

    // Generate JWT access token
    let token = state
        .token_generator
        .generate_token(
            client.client_id.clone(),
            granted_scopes.clone(),
            None, // node_id will be added later
        )?;

    // Return token response
    Ok(Json(TokenResponse::new(token, 3600, granted_scopes)))
}

/// Client creation request
#[derive(Debug, Deserialize)]
pub struct CreateClientRequest {
    pub name: String,
    pub scopes: Vec<String>,
}

/// Client creation response
#[derive(Debug, Serialize)]
pub struct CreateClientResponse {
    pub client_id: String,
    pub client_secret: String, // Only returned once!
    pub name: String,
    pub scopes: Vec<String>,
}

/// POST /mcp/auth/clients - Create new OAuth2 client (admin only)
pub async fn create_client(
    Extension(state): Extension<OAuth2State>,
    Json(request): Json<CreateClientRequest>,
) -> Result<Json<CreateClientResponse>, AuthError> {
    // Generate credentials
    let client_id = ClientCredentials::generate_client_id();
    let client_secret = ClientCredentials::generate_client_secret();

    // Create client
    let credentials = ClientCredentials::new(
        client_id.clone(),
        &client_secret,
        request.scopes.clone(),
        request.name.clone(),
    );

    // Store in database
    state.client_store.add_client(credentials).await?;
    save_clients(&state.client_store, &state.clients_file).await?;

    // Return credentials (client_secret only shown once!)
    Ok(Json(CreateClientResponse {
        client_id,
        client_secret,
        name: request.name,
        scopes: request.scopes,
    }))
}

/// Client list response
#[derive(Debug, Serialize)]
pub struct ClientListResponse {
    pub clients: Vec<ClientInfo>,
}

#[derive(Debug, Serialize)]
pub struct ClientInfo {
    pub client_id: String,
    pub name: String,
    pub scopes: Vec<String>,
    pub created_at: i64,
    pub revoked: bool,
}

/// GET /mcp/auth/clients - List all OAuth2 clients (admin only)
pub async fn list_clients(
    Extension(state): Extension<OAuth2State>,
) -> Result<Json<ClientListResponse>, AuthError> {
    let clients = state.client_store.list_clients().await?;

    let client_info: Vec<ClientInfo> = clients
        .into_iter()
        .map(|c| ClientInfo {
            client_id: c.client_id,
            name: c.name,
            scopes: c.scopes,
            created_at: c.created_at,
            revoked: c.revoked,
        })
        .collect();

    Ok(Json(ClientListResponse { clients: client_info }))
}

/// DELETE /mcp/auth/clients/:client_id - Revoke OAuth2 client (admin only)
pub async fn revoke_client(
    Extension(state): Extension<OAuth2State>,
    axum::extract::Path(client_id): axum::extract::Path<String>,
) -> Result<StatusCode, AuthError> {
    state.client_store.revoke_client(&client_id).await?;
    save_clients(&state.client_store, &state.clients_file).await?;
    Ok(StatusCode::NO_CONTENT)
}
