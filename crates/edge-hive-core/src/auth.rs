use axum::{
    extract::State,
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

/// Shared OAuth2 state
#[derive(Clone)]
pub struct OAuth2State {
    pub token_generator: Arc<TokenGenerator>,
    pub client_store: Arc<ClientStore>,
}

impl OAuth2State {
    pub fn new(jwt_secret: &[u8], issuer: String) -> Self {
        Self {
            token_generator: Arc::new(TokenGenerator::new(jwt_secret, issuer)),
            client_store: Arc::new(ClientStore::new()),
        }
    }
}

/// POST /mcp/auth/token - OAuth2 Client Credentials token endpoint
pub async fn token_endpoint(
    State(state): State<OAuth2State>,
    Json(request): Json<TokenRequest>,
) -> Result<Json<TokenResponse>, AuthError> {
    // Validate grant type
    request.validate()?;
    
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
    State(state): State<OAuth2State>,
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
    State(state): State<OAuth2State>,
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
    State(state): State<OAuth2State>,
    axum::extract::Path(client_id): axum::extract::Path<String>,
) -> Result<StatusCode, AuthError> {
    state.client_store.revoke_client(&client_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
