//! Onion service setup and management

use anyhow::{Context, Result};
use ed25519_consensus::SigningKey;
use rand::rngs::OsRng;
use sha3::Digest;
use std::fs;
use tracing::{info, warn};
use super::TorConfig;

/// Onion service manager
pub struct OnionService {
    config: TorConfig,
}

impl OnionService {
    /// Create new onion service manager
    pub fn new(config: TorConfig) -> Self {
        Self { config }
    }
    
    /// Launch the onion service and return .onion address
    pub async fn launch(&self) -> Result<String> {
        info!("Launching onion service...");
        
        // Ensure keys directory exists
        let keys_dir = self.config.data_dir.join("keys");
        fs::create_dir_all(&keys_dir)
            .context("Failed to create keys directory")?;
        
        // Load or generate Ed25519 keypair for onion address
        let keypair = self.load_or_generate_keypair(&keys_dir)?;
        
        // Derive .onion address from public key
        let onion_address = self.derive_onion_address(&keypair);
        
        info!("🧅 Generated onion address: {}.onion", onion_address);
        warn!("⚠️  Full onion service launch requires tor-hsservice API");
        warn!("⚠️  Currently generating address only - service not listening");
        
        Ok(onion_address)
    }
    
    /// Load existing keypair or generate new one
    fn load_or_generate_keypair(&self, keys_dir: &std::path::Path) -> Result<SigningKey> {
        let key_file = keys_dir.join("hs_ed25519_secret_key");
        
        if key_file.exists() {
            info!("Loading existing onion service key");
            let key_bytes = fs::read(&key_file)
                .context("Failed to read key file")?;
            
            // Skip Tor's key file header (32 bytes) if present
            let key_start = if key_bytes.len() > 64 { 32 } else { 0 };
            let secret_bytes = &key_bytes[key_start..key_start + 32];
            
            SigningKey::try_from(secret_bytes)
                .context("Failed to parse Ed25519 key")
        } else {
            info!("Generating new onion service key");
            let mut rng = OsRng;
            let keypair = SigningKey::new(&mut rng);
            
            // Save key in Tor format (with header)
            let mut key_data = Vec::new();
            
            // Tor key file header (simplified - real format is more complex)
            key_data.extend_from_slice(b"== ed25519v1-secret: type0 ==\0\0\0");
            key_data.extend_from_slice(&keypair.to_bytes());
            
            fs::write(&key_file, &key_data)
                .context("Failed to save key file")?;
            
            Ok(keypair)
        }
    }
    
    /// Derive .onion v3 address from Ed25519 public key
    fn derive_onion_address(&self, keypair: &SigningKey) -> String {
        let public_key = keypair.verification_key();
        let pub_bytes = public_key.to_bytes();
        
        // Onion v3 address format:
        // base32(PUBKEY || CHECKSUM || VERSION)
        // where CHECKSUM = H(".onion checksum" || PUBKEY || VERSION)[:2]
        
        let version = [0x03u8]; // Version 3
        
        // Calculate checksum using SHA3-256
        let checksum_input = [b".onion checksum".as_slice(), &pub_bytes, &version].concat();
        let checksum_hash = sha3::Sha3_256::digest(&checksum_input);
        let checksum = &checksum_hash[..2];
        
        // Combine: PUBKEY (32) || CHECKSUM (2) || VERSION (1) = 35 bytes
        let mut address_bytes = Vec::with_capacity(35);
        address_bytes.extend_from_slice(&pub_bytes);
        address_bytes.extend_from_slice(checksum);
        address_bytes.extend_from_slice(&version);
        
        // Base32 encode (RFC4648 without padding)
        data_encoding::BASE32_NOPAD.encode(&address_bytes).to_lowercase()
    }
}
