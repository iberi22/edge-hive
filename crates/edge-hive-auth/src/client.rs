use serde::{Deserialize, Serialize};
use crate::oauth2::ClientCredentials;
use crate::error::{AuthError, Result};

/// In-memory client store (temporary - will be replaced with SurrealDB)
#[derive(Debug, Clone)]
pub struct ClientStore {
    clients: std::sync::Arc<tokio::sync::RwLock<std::collections::HashMap<String, ClientCredentials>>>,
}

impl ClientStore {
    pub fn new() -> Self {
        Self {
            clients: std::sync::Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new())),
        }
    }

    /// Add a new client
    pub async fn add_client(&self, credentials: ClientCredentials) -> Result<()> {
        let mut clients = self.clients.write().await;
        clients.insert(credentials.client_id.clone(), credentials);
        Ok(())
    }

    /// Get client by ID
    pub async fn get_client(&self, client_id: &str) -> Result<ClientCredentials> {
        let clients = self.clients.read().await;
        clients
            .get(client_id)
            .cloned()
            .ok_or_else(|| AuthError::ClientNotFound(client_id.to_string()))
    }

    /// Verify client credentials
    pub async fn verify_credentials(&self, client_id: &str, client_secret: &str) -> Result<ClientCredentials> {
        let client = self.get_client(client_id).await?;

        if !client.verify_secret(client_secret) {
            return Err(AuthError::InvalidCredentials);
        }

        if client.revoked {
            return Err(AuthError::InvalidCredentials);
        }

        Ok(client)
    }

    /// List all clients
    pub async fn list_clients(&self) -> Result<Vec<ClientCredentials>> {
        let clients = self.clients.read().await;
        Ok(clients.values().cloned().collect())
    }

    /// Revoke a client
    pub async fn revoke_client(&self, client_id: &str) -> Result<()> {
        let mut clients = self.clients.write().await;

        if let Some(client) = clients.get_mut(client_id) {
            client.revoked = true;
            Ok(())
        } else {
            Err(AuthError::ClientNotFound(client_id.to_string()))
        }
    }

    /// Delete a client permanently
    pub async fn delete_client(&self, client_id: &str) -> Result<()> {
        let mut clients = self.clients.write().await;
        clients
            .remove(client_id)
            .ok_or_else(|| AuthError::ClientNotFound(client_id.to_string()))?;
        Ok(())
    }
}

impl Default for ClientStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_client_store_operations() {
        let store = ClientStore::new();

        let client = ClientCredentials::new(
            "test_client".to_string(),
            "test_secret",
            vec!["mcp:read".to_string()],
            "Test Client".to_string(),
        );

        // Add client
        store.add_client(client.clone()).await.unwrap();

        // Verify credentials
        let verified = store.verify_credentials("test_client", "test_secret").await.unwrap();
        assert_eq!(verified.client_id, "test_client");

        // Invalid secret should fail
        assert!(store.verify_credentials("test_client", "wrong_secret").await.is_err());

        // Revoke client
        store.revoke_client("test_client").await.unwrap();

        // Revoked client should fail verification
        assert!(store.verify_credentials("test_client", "test_secret").await.is_err());
    }
}
