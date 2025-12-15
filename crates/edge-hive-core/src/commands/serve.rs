//! Start the Edge Hive server

use edge_hive_core::server;
use edge_hive_identity::NodeIdentity;
use edge_hive_tunnel::{TunnelBackend, TunnelService};
use futures::StreamExt;
use libp2p::{
    identify, identity, kad, mdns, noise,
    swarm::{Swarm, SwarmEvent},
    tcp, yamux, SwarmBuilder,
};
use std::path::Path;
use tracing::{info, warn, debug};
use clap::Args;

#[derive(Args, Debug)]
pub struct ServeArgs {
    /// Port to listen on
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,

    /// Enable Cloudflare tunnel
    #[arg(long)]
    pub tunnel: bool,

    /// Enable discovery service
    #[arg(long)]
    pub discovery: bool,
}

/// The network behaviour for the discovery service.
#[derive(libp2p::swarm::NetworkBehaviour)]
#[behaviour(out_event = "BehaviourEvent")]
struct Behaviour {
    identify: identify::Behaviour,
    mdns: mdns::tokio::Behaviour,
    kad: kad::Behaviour<kad::store::MemoryStore>,
}

#[derive(Debug)]
enum BehaviourEvent {
    Identify(identify::Event),
    Mdns(mdns::Event),
    Kad(kad::Event),
}

impl From<identify::Event> for BehaviourEvent {
    fn from(event: identify::Event) -> Self {
        BehaviourEvent::Identify(event)
    }
}

impl From<mdns::Event> for BehaviourEvent {
    fn from(event: mdns::Event) -> Self {
        BehaviourEvent::Mdns(event)
    }
}

impl From<kad::Event> for BehaviourEvent {
    fn from(event: kad::Event) -> Self {
        BehaviourEvent::Kad(event)
    }
}

/// Run the serve command
pub async fn run(
    args: ServeArgs,
    data_dir: &Path,
) -> anyhow::Result<()> {
    let identity_path = data_dir.join("identity.key");

    // Load identity
    let identity = if identity_path.exists() {
        NodeIdentity::load(&identity_path, None)?
    } else {
        warn!("No identity found, generating new one...");
        let identity = NodeIdentity::generate()?;
        identity.save(&identity_path, None)?;
        identity
    };

    println!("🐝 Edge Hive Node");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("   Name:    {}", identity.name());
    println!("   Peer ID: {}...", &identity.peer_id()[..16]);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    // Initialize discovery service
    if args.discovery {
        info!("🔍 Starting discovery service...");

        let mut secret_bytes = identity.secret_key_bytes();
        let secret_key = identity::ed25519::SecretKey::try_from_bytes(&mut secret_bytes)?;
        let keypair = identity::Keypair::from(identity::ed25519::Keypair::from(secret_key));
        let peer_id = keypair.public().to_peer_id();

        let mut behaviour = Behaviour {
            identify: identify::Behaviour::new(identify::Config::new(
                "/edge-hive/1.0.0".to_string(),
                keypair.public(),
            )),
            mdns: mdns::tokio::Behaviour::new(mdns::Config::default(), peer_id)?,
            kad: kad::Behaviour::new(peer_id, kad::store::MemoryStore::new(peer_id)),
        };

        let bootnodes = [
            (
                "QmNnooDu7bfjPFoTZYxMNLWUQJyrVwtbZg5gBMjTezGAJN",
                "/dnsaddr/bootstrap.libp2p.io/p2p/QmNnooDu7bfjPFoTZYxMNLWUQJyrVwtbZg5gBMjTezGAJN",
            ),
            (
                "QmQCU2EcMqAqQPR2i9bChDtGNJchTf5CMDuiCC5H4hrF4k",
                "/dnsaddr/bootstrap.libp2p.io/p2p/QmQCU2EcMqAqQPR2i9bChDtGNJchTf5CMDuiCC5H4hrF4k",
            ),
        ];

        for (peer, addr) in bootnodes {
            behaviour
                .kad
                .add_address(&peer.parse()?, addr.parse()?);
        }

        let mut swarm = SwarmBuilder::with_existing_identity(keypair)
            .with_tokio()
            .with_tcp(
                tcp::Config::default(),
                noise::Config::new,
                yamux::Config::default,
            )?
            .with_behaviour(|_| behaviour)?
            .build();

        swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
        swarm.behaviour_mut().kad.bootstrap()?;

        tokio::spawn(async move {
            loop {
                match swarm.select_next_some().await {
                    SwarmEvent::Behaviour(event) => {
                        debug!("Discovery event: {:?}", event);
                    }
                    _ => {}
                }
            }
        });

        println!("✅ Discovery: Enabled (mDNS + DHT)");
    } else {
        println!("⏸️  Discovery: Disabled");
        None
    };

    // Initialize tunnel service
    let tunnel = if args.tunnel {
        info!("🚇 Starting tunnel service...");
        let mut tunnel = TunnelService::new(TunnelBackend::Cloudflared);

        match tunnel.start_quick(args.port).await {
            Ok(url) => {
                println!("✅ Tunnel: {}", url);
                Some(tunnel)
            }
            Err(e) => {
                warn!("Failed to start tunnel: {}", e);
                println!("⚠️  Tunnel: Failed ({})", e);
                None
            }
        }
    } else {
        println!("⏸️  Tunnel: Disabled (use --tunnel to enable)");
        None
    };

    println!();
    println!("🌐 HTTP Server: http://0.0.0.0:{}", args.port);
    println!();
    println!("Press Ctrl+C to stop");
    println!();

    // Run the HTTP server
    server::run(args.port).await?;

    // Cleanup
    if let Some(mut t) = tunnel {
        info!("Stopping tunnel...");
        t.stop().await?;
    }

    Ok(())
}
