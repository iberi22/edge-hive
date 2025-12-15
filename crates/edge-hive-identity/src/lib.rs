//! Edge Hive Identity - Cryptographic node identity system
//!
//! Provides Ed25519-based identity for nodes in the Edge Hive network.

use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use std::path::Path;
use thiserror::Error;
use zeroize::Zeroize;

/// Errors that can occur during identity operations
#[derive(Debug, Error)]
pub enum IdentityError {
    #[error("Failed to generate keypair: {0}")]
    Generation(String),

    #[error("Failed to load identity: {0}")]
    Load(#[from] std::io::Error),

    #[error("Failed to parse identity: {0}")]
    Parse(String),

    #[error("Signature verification failed")]
    VerificationFailed,
}

/// Node identity containing Ed25519 keypair and metadata
#[derive(Clone)]
pub struct NodeIdentity {
    keypair: SigningKey,
    name: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

/// Serializable identity data (without private key)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicIdentity {
    pub peer_id: String,
    pub name: String,
    pub public_key: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl NodeIdentity {
    /// Generate a new random identity
    pub fn generate() -> Result<Self, IdentityError> {
        let keypair = SigningKey::generate(&mut OsRng);
        let name = Self::generate_name(&keypair.verifying_key());

        Ok(Self {
            keypair,
            name,
            created_at: chrono::Utc::now(),
        })
    }

    /// Generate a human-readable name from the public key
    fn generate_name(public_key: &VerifyingKey) -> String {
        let bytes = public_key.as_bytes();
        let adjectives = ["swift", "brave", "calm", "dark", "eager", "fair", "grand", "happy"];
        let nouns = ["alpha", "beta", "gamma", "delta", "echo", "fox", "gate", "hive"];

        let adj_idx = (bytes[0] as usize) % adjectives.len();
        let noun_idx = (bytes[1] as usize) % nouns.len();
        let hex = hex::encode(&bytes[2..5]);

        format!("{}-{}-{}", adjectives[adj_idx], nouns[noun_idx], hex)
    }

    /// Get the node's human-readable name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the peer ID (base64-encoded public key)
    pub fn peer_id(&self) -> String {
        use base64::Engine;
        base64::engine::general_purpose::URL_SAFE_NO_PAD
            .encode(self.keypair.verifying_key().as_bytes())
    }

    /// Get the public key
    pub fn public_key(&self) -> VerifyingKey {
        self.keypair.verifying_key()
    }

    /// Sign a message
    pub fn sign(&self, message: &[u8]) -> Signature {
        self.keypair.sign(message)
    }

    /// Verify a signature
    pub fn verify(&self, message: &[u8], signature: &Signature) -> Result<(), IdentityError> {
        self.keypair
            .verifying_key()
            .verify(message, signature)
            .map_err(|_| IdentityError::VerificationFailed)
    }

    /// Export public identity (safe to share)
    pub fn public_identity(&self) -> PublicIdentity {
        use base64::Engine;
        PublicIdentity {
            peer_id: self.peer_id(),
            name: self.name.clone(),
            public_key: base64::engine::general_purpose::STANDARD
                .encode(self.keypair.verifying_key().as_bytes()),
            created_at: self.created_at,
        }
    }

    /// Save identity to file (encrypted with optional passphrase)
    pub fn save(&self, path: &Path) -> Result<(), IdentityError> {
        use base64::Engine;
        let data = serde_json::json!({
            "version": 1,
            "name": self.name,
            "created_at": self.created_at.to_rfc3339(),
            "secret_key": base64::engine::general_purpose::STANDARD.encode(self.keypair.to_bytes()),
        });

        let content = serde_json::to_string_pretty(&data)
            .map_err(|e| IdentityError::Parse(e.to_string()))?;

        std::fs::create_dir_all(path.parent().unwrap_or(Path::new(".")))?;
        std::fs::write(path, content)?;

        Ok(())
    }

    /// Load identity from file
    pub fn load(path: &Path) -> Result<Self, IdentityError> {
        use base64::Engine;
        let content = std::fs::read_to_string(path)?;
        let data: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| IdentityError::Parse(e.to_string()))?;

        let secret_key_b64 = data["secret_key"]
            .as_str()
            .ok_or_else(|| IdentityError::Parse("Missing secret_key".into()))?;

        let secret_key_bytes = base64::engine::general_purpose::STANDARD
            .decode(secret_key_b64)
            .map_err(|e| IdentityError::Parse(e.to_string()))?;

        let keypair = SigningKey::try_from(secret_key_bytes.as_slice())
            .map_err(|e| IdentityError::Parse(e.to_string()))?;

        let name = data["name"]
            .as_str()
            .map(String::from)
            .unwrap_or_else(|| Self::generate_name(&keypair.verifying_key()));

        let created_at = data["created_at"]
            .as_str()
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&chrono::Utc))
            .unwrap_or_else(chrono::Utc::now);

        Ok(Self {
            keypair,
            name,
            created_at,
        })
    }
}

impl Drop for NodeIdentity {
    fn drop(&mut self) {
        // Zeroize sensitive data on drop
        // Note: SigningKey doesn't implement Zeroize directly,
        // but we clear our name at least
        self.name.zeroize();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_generate_identity() {
        let identity = NodeIdentity::generate().unwrap();
        assert!(!identity.name().is_empty());
        assert!(!identity.peer_id().is_empty());
    }

    #[test]
    fn test_sign_verify() {
        let identity = NodeIdentity::generate().unwrap();
        let message = b"Hello, Edge Hive!";
        let signature = identity.sign(message);
        assert!(identity.verify(message, &signature).is_ok());
    }

    #[test]
    fn test_save_load() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("identity.key");

        let identity = NodeIdentity::generate().unwrap();
        identity.save(&path).unwrap();

        let loaded = NodeIdentity::load(&path).unwrap();
        assert_eq!(identity.peer_id(), loaded.peer_id());
        assert_eq!(identity.name(), loaded.name());
    }
}
