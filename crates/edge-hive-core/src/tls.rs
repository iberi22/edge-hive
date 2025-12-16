//! TLS/HTTPS certificate management

use anyhow::{Context, Result};
use rcgen::{generate_simple_self_signed, CertifiedKey};
use rustls::{ServerConfig, pki_types::{CertificateDer, PrivateKeyDer}};
use rustls_pemfile::{certs, private_key};
use std::fs::{self, File};
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// TLS certificate configuration
pub struct TlsCertificate {
    pub cert_path: PathBuf,
    pub key_path: PathBuf,
}

impl TlsCertificate {
    /// Load or generate TLS certificate
    pub fn load_or_generate(data_dir: &Path, hostname: &str) -> Result<Self> {
        let cert_path = data_dir.join("tls_cert.pem");
        let key_path = data_dir.join("tls_key.pem");

        if cert_path.exists() && key_path.exists() {
            tracing::info!("🔐 Loading existing TLS certificate");
            Ok(Self { cert_path, key_path })
        } else {
            tracing::info!("🔐 Generating self-signed TLS certificate for {}", hostname);
            Self::generate_self_signed(hostname, &cert_path, &key_path)?;
            Ok(Self { cert_path, key_path })
        }
    }

    /// Generate self-signed certificate
    fn generate_self_signed(hostname: &str, cert_path: &Path, key_path: &Path) -> Result<()> {
        let subject_alt_names = vec![
            hostname.to_string(),
            "localhost".to_string(),
            "127.0.0.1".to_string(),
            "::1".to_string(),
        ];

        let CertifiedKey { cert, key_pair } = generate_simple_self_signed(subject_alt_names)
            .context("Failed to generate self-signed certificate")?;

        // Write certificate
        fs::write(cert_path, cert.pem())
            .context("Failed to write certificate file")?;

        // Write private key
        fs::write(key_path, key_pair.serialize_pem())
            .context("Failed to write private key file")?;

        tracing::info!("✅ Self-signed certificate generated");
        tracing::warn!("⚠️  Self-signed certificates are for testing only!");
        tracing::warn!("   For production, use Let's Encrypt or a trusted CA.");

        Ok(())
    }

    /// Build rustls ServerConfig
    pub fn build_server_config(&self) -> Result<Arc<ServerConfig>> {
        // rustls 0.23 requires selecting a process-level CryptoProvider.
        // Install the ring provider if not already installed.
        let _ = rustls::crypto::ring::default_provider().install_default();

        // Load certificate chain
        let cert_file = File::open(&self.cert_path)
            .context("Failed to open certificate file")?;
        let mut cert_reader = BufReader::new(cert_file);
        let certs: Vec<CertificateDer> = certs(&mut cert_reader)
            .collect::<Result<Vec<_>, _>>()
            .context("Failed to parse certificate")?;

        // Load private key
        let key_file = File::open(&self.key_path)
            .context("Failed to open private key file")?;
        let mut key_reader = BufReader::new(key_file);
        let key = private_key(&mut key_reader)
            .context("Failed to parse private key")?
            .ok_or_else(|| anyhow::anyhow!("No private key found in file"))?;

        // Build ServerConfig
        let config = ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(certs, key)
            .context("Failed to build TLS server configuration")?;

        Ok(Arc::new(config))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_generate_self_signed() {
        let dir = tempdir().unwrap();
        let cert_path = dir.path().join("cert.pem");
        let key_path = dir.path().join("key.pem");

        TlsCertificate::generate_self_signed("localhost", &cert_path, &key_path).unwrap();

        assert!(cert_path.exists());
        assert!(key_path.exists());

        // Verify we can load them
        let tls_cert = TlsCertificate {
            cert_path,
            key_path,
        };

        let config = tls_cert.build_server_config();
        assert!(config.is_ok());
    }
}
