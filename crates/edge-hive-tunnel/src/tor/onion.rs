//! Onion service setup and management

use anyhow::{anyhow, Result};
use arti_client::TorClient;
use futures::{io::{AsyncRead, AsyncWrite}, StreamExt};
use std::net::SocketAddr;
use tokio::io;
use tokio_util::compat::FuturesAsyncReadCompatExt;
use tor_hsservice::{HsNickname, OnionServiceConfig, RendRequest};
use tor_rtcompat::PreferredRuntime;
use tracing::{info, warn};
use super::TorConfig;

/// Onion service manager
pub struct OnionService {
    config: TorConfig,
    tor_client: TorClient<PreferredRuntime>,
}

trait StreamTrait: AsyncRead + AsyncWrite + Send + Unpin {}
impl<T: AsyncRead + AsyncWrite + Send + Unpin> StreamTrait for T {}

impl OnionService {
    /// Create new onion service manager
    pub fn new(
        config: TorConfig,
        tor_client: TorClient<PreferredRuntime>,
    ) -> Self {
        Self { config, tor_client }
    }

    /// Launch the onion service and return .onion address
    pub async fn launch(&self) -> Result<String> {
        info!("Launching onion service...");

        let nickname = self.config.nickname.as_ref()
            .map(|s| HsNickname::try_from(s.clone()))
            .transpose()?
            .unwrap_or_else(|| HsNickname::new("edge-hive-node".to_string()).unwrap());

        let mut config_builder = tor_hsservice::config::OnionServiceConfigBuilder::default();
        config_builder.nickname(nickname);
        let config = config_builder.build().unwrap();

        let (onion_service, receiver) = self.tor_client.launch_onion_service(config).unwrap();

        let onion_address = onion_service.onion_name().unwrap().to_string();
        info!("🧅 Onion service running at: {}.onion", onion_address);

        let local_target = SocketAddr::from(([127, 0, 0, 1], self.config.local_port));
        let runtime = PreferredRuntime::current()?;

        let mut receiver = receiver.fuse();
        let service_loop = async move {
            while let Some(request) = receiver.next().await {
                let runtime_clone = runtime.clone();
                runtime_clone.spawn(async move {
                    if let Err(e) = Self::handle_request(request, local_target).await {
                        warn!("Error handling onion request: {}", e);
                    }
                }).unwrap();
            }
        };

        tokio::spawn(service_loop);

        Ok(onion_address)
    }

    /// Handle an incoming onion service request
    async fn handle_request(request: RendRequest, target: SocketAddr) -> Result<()> {
        info!("Received onion request");

        let stream: Box<dyn StreamTrait> = Box::new(request.accept().await
            .map_err(|e| anyhow!("Failed to accept stream: {}", e))?);

        let mut target_conn = tokio::net::TcpStream::connect(target).await
            .map_err(|e| anyhow!("Failed to connect to target: {}", e))?;

        let (mut r1, mut w1) = io::split(stream.compat());
        let (mut r2, mut w2) = io::split(target_conn.compat());

        tokio::select! {
            res = io::copy(&mut r1, &mut w2) => res?,
            res = io::copy(&mut r2, &mut w1) => res?,
        };

        Ok(())
    }
}
