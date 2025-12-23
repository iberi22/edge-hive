use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use crate::error::{AuthError, Result};

/// JWT Claims structure following OAuth2 and MCP requirements
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct JwtClaims {
    /// Subject (client_id)
    pub sub: String,
    /// Issuer (node URL)
    pub iss: String,
    /// Audience (typically "mcp")
    pub aud: String,
    /// Expiration time (Unix timestamp)
    pub exp: i64,
    /// Issued at (Unix timestamp)
    pub iat: i64,
    /// JWT ID (unique token identifier)
    pub jti: String,
    /// Scopes granted to this token
    pub scopes: Vec<String>,
    /// Custom: Node ID that issued the token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
}

impl JwtClaims {
    /// Create new JWT claims with default expiration (1 hour)
    pub fn new(
        client_id: String,
        issuer: String,
        scopes: Vec<String>,
        node_id: Option<String>,
    ) -> Self {
        let now = Utc::now();
        let exp = now + Duration::hours(1);

        Self {
            sub: client_id,
            iss: issuer,
            aud: "mcp".to_string(),
            exp: exp.timestamp(),
            iat: now.timestamp(),
            jti: uuid::Uuid::new_v4().to_string(),
            scopes,
            node_id,
        }
    }

    /// Check if token has expired
    pub fn is_expired(&self) -> bool {
        let now = Utc::now().timestamp();
        self.exp <= now
    }

    /// Check if token has required scope
    pub fn has_scope(&self, required_scope: &str) -> bool {
        self.scopes.iter().any(|s| s == required_scope)
    }

    /// Check if token has all required scopes
    pub fn has_all_scopes(&self, required_scopes: &[String]) -> bool {
        required_scopes.iter().all(|scope| self.has_scope(scope))
    }
}

/// JWT cryptographic keys
pub struct JwtKeys {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl JwtKeys {
    /// Create keys from HMAC secret (HS256)
    pub fn from_secret(secret: &[u8]) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret),
            decoding_key: DecodingKey::from_secret(secret),
        }
    }

    /// Generate random HMAC secret (32 bytes)
    pub fn generate_secret() -> Vec<u8> {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        (0..32).map(|_| rng.gen::<u8>()).collect()
    }
}

/// JWT token generator
pub struct TokenGenerator {
    keys: JwtKeys,
    issuer: String,
}

impl TokenGenerator {
    pub fn new(secret: &[u8], issuer: String) -> Self {
        Self {
            keys: JwtKeys::from_secret(secret),
            issuer,
        }
    }

    /// Generate JWT access token
    pub fn generate_token(
        &self,
        client_id: String,
        scopes: Vec<String>,
        node_id: Option<String>,
    ) -> Result<String> {
        let claims = JwtClaims::new(client_id, self.issuer.clone(), scopes, node_id);

        encode(&Header::default(), &claims, &self.keys.encoding_key)
            .map_err(AuthError::from)
    }

    /// Generate JWT access token from existing claims (useful for tests)
    pub fn generate_token_from_claims(&self, claims: &JwtClaims) -> Result<String> {
        encode(&Header::default(), claims, &self.keys.encoding_key).map_err(AuthError::from)
    }
}

/// JWT token validator
pub struct TokenValidator {
    keys: JwtKeys,
    issuer: String,
}

impl TokenValidator {
    pub fn new(secret: &[u8], issuer: String) -> Self {
        Self {
            keys: JwtKeys::from_secret(secret),
            issuer,
        }
    }

    /// Validate and decode JWT token
    pub fn validate_token(&self, token: &str) -> Result<JwtClaims> {
        let mut validation = Validation::default();
        validation.set_issuer(&[&self.issuer]);
        validation.set_audience(&["mcp"]);

        let token_data: TokenData<JwtClaims> = decode(
            token,
            &self.keys.decoding_key,
            &validation,
        )?;

        // Additional expiration check
        if token_data.claims.is_expired() {
            return Err(AuthError::TokenExpired);
        }

        Ok(token_data.claims)
    }

    /// Extract and validate Bearer token from Authorization header
    pub fn validate_bearer_token(&self, auth_header: &str) -> Result<JwtClaims> {
        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(AuthError::InvalidAuthHeader)?;

        self.validate_token(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_generation_and_validation() {
        let secret = JwtKeys::generate_secret();
        let issuer = "https://test-node:8080".to_string();

        let generator = TokenGenerator::new(&secret, issuer.clone());
        let validator = TokenValidator::new(&secret, issuer);

        let token = generator.generate_token(
            "test-client".to_string(),
            vec!["mcp:read".to_string(), "mcp:call".to_string()],
            Some("node-123".to_string()),
        ).unwrap();

        let claims = validator.validate_token(&token).unwrap();

        assert_eq!(claims.sub, "test-client");
        assert_eq!(claims.iss, "https://test-node:8080");
        assert_eq!(claims.aud, "mcp");
        assert!(claims.has_scope("mcp:read"));
        assert!(claims.has_scope("mcp:call"));
        assert!(!claims.has_scope("mcp:admin"));
    }

    #[test]
    fn test_bearer_token_validation() {
        let secret = JwtKeys::generate_secret();
        let issuer = "https://test-node:8080".to_string();

        let generator = TokenGenerator::new(&secret, issuer.clone());
        let validator = TokenValidator::new(&secret, issuer);

        let token = generator.generate_token(
            "test-client".to_string(),
            vec!["mcp:read".to_string()],
            None,
        ).unwrap();

        let auth_header = format!("Bearer {}", token);
        let claims = validator.validate_bearer_token(&auth_header).unwrap();

        assert_eq!(claims.sub, "test-client");
    }
}
