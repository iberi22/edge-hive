//! Tor onion service integration

use anyhow::Result;
use arti_client::{TorClient, TorClientConfig, OnionServiceExt};
use futures::StreamExt;
use std::net::SocketAddr;
use std::path::Path;
use tor_hscrypto::pk::HsIdKeypair;
use tor_hsservice::{OnionServiceConfigBuilder, RendRequest};
use tor_rtcompat::PreferredRuntime;
use tracing::info;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use ed25519_consensus::SigningKey;

/// A running Tor client, connected to an onion service.
pub struct TorNode {
    /// The Arti Tor client.
    pub client: TorClient<PreferredRuntime>,
    /// The onion address of our service.
    pub onion_address: String,
}

impl TorNode {
    /// Bootstrap a new Tor client and launch an onion service.
    pub async fn bootstrap(data_dir: &Path, local_port: u16) -> Result<Self> {
        let tor_dir = data_dir.join("tor");
        std::fs::create_dir_all(&tor_dir)?;

        let mut config_builder = TorClientConfig::builder();
        config_builder.storage().storage_dir(tor_dir.clone());
        let config = config_builder.build()?;

        info!("Bootstrapping Tor client...");
        let client = TorClient::create_bootstrapped(config).await?;

        let nickname = "edge-hive-node".into();
        let keypair = load_or_generate_identity(&tor_dir.join("onion_keys"))?;

        let onion_service_config = OnionServiceConfigBuilder::default()
            .nickname(nickname)
            .build()?;

        let (onion_service, mut rend_requests) =
            client.launch_onion_service(onion_service_config, keypair)?;

        let onion_address = onion_service.onion_name().to_string();
        info!("🧅 Onion address: http://{}.onion", onion_address);

        let local_socket = SocketAddr::from(([127, 0, 0, 1], local_port));

        // Start a task to handle incoming rendezvous requests.
        tokio::spawn(async move {
            while let Some(request) = rend_requests.next().await {
                handle_rend_request(request, local_socket).await;
            }
        });

        Ok(Self {
            client,
            onion_address,
        })
    }
}

/// Handle a single rendezvous request.
async fn handle_rend_request(request: RendRequest, local_socket: SocketAddr) {
    info!("Received onion service request; connecting to local service.");
    let stream = match request.accept().await {
        Ok(stream) => stream,
        Err(e) => {
            tracing::warn!("Failed to accept onion service request: {}", e);
            return;
        }
    };

    let mut local_conn = match tokio::net::TcpStream::connect(local_socket).await {
        Ok(conn) => conn,
        Err(e) => {
            tracing::warn!("Failed to connect to local service: {}", e);
            return;
        }
    };

    let (mut client_read, mut client_write) = tokio::io::split(stream);
    let (mut local_read, mut local_write) = tokio::io::split(local_conn);

    tokio::select! {
        res = tokio::io::copy(&mut client_read, &mut local_write) => {
            if let Err(e) = res {
                tracing::warn!("Error copying from onion service to local: {}", e);
            }
        }
        res = tokio::io::copy(&mut local_read, &mut client_write) => {
            if let Err(e) = res {
                tracing::warn!("Error copying from local to onion service: {}", e);
            }
        }
    }
}

/// Load an onion service identity keypair from a file, or generate a new one.
pub fn load_or_generate_identity(path: &Path) -> Result<HsIdKeypair> {
    if path.exists() {
        let bytes = std::fs::read(path)?;
        let key_bytes: [u8; 32] = bytes
            .try_into()
            .map_err(|_| anyhow::anyhow!("invalid key file"))?;
        let sk = SigningKey::from(key_bytes);
        Ok(HsIdKeypair::from(sk))
    } else {
        info!("Generating new onion service identity at {}", path.display());
        let mut rng = rand::thread_rng();
        let keypair = SigningKey::new(&mut rng);
        std::fs::write(path, &keypair.to_bytes())?;
        Ok(keypair.into())
    }
}
