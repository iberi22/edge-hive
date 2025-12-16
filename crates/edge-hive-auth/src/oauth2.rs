use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use crate::error::{AuthError, Result};

/// OAuth2 configuration
#[derive(Debug, Clone)]
pub struct OAuth2Config {
    pub issuer: String,
    pub token_endpoint: String,
    pub token_expiration_secs: i64,
}

impl Default for OAuth2Config {
    fn default() -> Self {
        Self {
            issuer: "https://localhost:8080".to_string(),
            token_endpoint: "/mcp/auth/token".to_string(),
            token_expiration_secs: 3600, // 1 hour
        }
    }
}

/// Client credentials (stored securely)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientCredentials {
    pub client_id: String,
    pub client_secret_hash: String, // SHA-256 hash, never store plaintext
    pub scopes: Vec<String>,
    pub name: String,
    pub created_at: i64,
    pub revoked: bool,
}

impl ClientCredentials {
    /// Create new client credentials with hashed secret
    pub fn new(client_id: String, client_secret: &str, scopes: Vec<String>, name: String) -> Self {
        Self {
            client_id,
            client_secret_hash: Self::hash_secret(client_secret),
            scopes,
            name,
            created_at: chrono::Utc::now().timestamp(),
            revoked: false,
        }
    }

    /// Hash client secret using SHA-256
    pub fn hash_secret(secret: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(secret.as_bytes());
        hex::encode(hasher.finalize())
    }

    /// Verify client secret against stored hash
    pub fn verify_secret(&self, secret: &str) -> bool {
        if self.revoked {
            return false;
        }

        let provided_hash = Self::hash_secret(secret);

        // Constant-time comparison to prevent timing attacks
        use std::cmp::Ordering;
        let mut result = Ordering::Equal;

        for (a, b) in self.client_secret_hash.bytes().zip(provided_hash.bytes()) {
            if a != b {
                result = Ordering::Less;
            }
        }

        result == Ordering::Equal &&
        self.client_secret_hash.len() == provided_hash.len()
    }

    /// Generate random client_id
    pub fn generate_client_id() -> String {
        format!("cli_{}", uuid::Uuid::new_v4().simple())
    }

    /// Generate random client_secret (32 bytes, hex encoded)
    pub fn generate_client_secret() -> String {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let bytes: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
        hex::encode(bytes)
    }
}

/// OAuth2 Access Token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub scope: String,
}

/// Token response (RFC 6749 compliant)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
}

impl TokenResponse {
    pub fn new(access_token: String, expires_in: i64, scopes: Vec<String>) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
            expires_in,
            scope: Some(scopes.join(" ")),
            refresh_token: None, // Not implementing refresh tokens yet
        }
    }
}

/// Token request parameters (Client Credentials grant)
#[derive(Debug, Clone, Deserialize)]
pub struct TokenRequest {
    pub grant_type: String,
    pub client_id: String,
    pub client_secret: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

impl TokenRequest {
    /// Validate grant type (must be "client_credentials")
    pub fn validate(&self) -> Result<()> {
        if self.grant_type != "client_credentials" {
            return Err(AuthError::InvalidCredentials);
        }
        Ok(())
    }

    /// Parse requested scopes
    pub fn requested_scopes(&self) -> Vec<String> {
        self.scope
            .as_ref()
            .map(|s| s.split_whitespace().map(String::from).collect())
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_credentials_creation() {
        let client_id = ClientCredentials::generate_client_id();
        let client_secret = ClientCredentials::generate_client_secret();

        let creds = ClientCredentials::new(
            client_id.clone(),
            &client_secret,
            vec!["mcp:read".to_string()],
            "Test Client".to_string(),
        );

        assert_eq!(creds.client_id, client_id);
        assert!(creds.verify_secret(&client_secret));
        assert!(!creds.verify_secret("wrong_secret"));
    }

    #[test]
    fn test_secret_hashing() {
        let secret = "my_secret_key_123";
        let hash1 = ClientCredentials::hash_secret(secret);
        let hash2 = ClientCredentials::hash_secret(secret);

        // Same secret should produce same hash
        assert_eq!(hash1, hash2);

        // Different secret should produce different hash
        let hash3 = ClientCredentials::hash_secret("different_secret");
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_token_request_validation() {
        let valid_request = TokenRequest {
            grant_type: "client_credentials".to_string(),
            client_id: "test_client".to_string(),
            client_secret: "test_secret".to_string(),
            scope: Some("mcp:read mcp:call".to_string()),
        };

        assert!(valid_request.validate().is_ok());

        let scopes = valid_request.requested_scopes();
        assert_eq!(scopes, vec!["mcp:read", "mcp:call"]);
    }
}
