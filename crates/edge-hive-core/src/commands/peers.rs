//! Manage P2P network peers

use clap::{Args, Subcommand};
use edge_hive_identity::NodeIdentity;
use futures::StreamExt;
use libp2p::{
    identify, identity, kad, mdns, noise,
    swarm::{NetworkBehaviour, Swarm, SwarmEvent},
    tcp, yamux, PeerId, SwarmBuilder,
};
use std::collections::HashSet;
use std::path::Path;
use std::time::Duration;
use tokio::time::timeout;
use tracing::info;

#[derive(Args, Debug)]
pub struct PeersArgs {
    #[command(subcommand)]
    command: PeersCommands,
}

#[derive(Subcommand, Debug)]
enum PeersCommands {
    /// List discovered peers
    List,
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

/// Run the peers command
pub async fn run(args: PeersArgs, data_dir: &Path) -> anyhow::Result<()> {
    match args.command {
        PeersCommands::List => list_peers(data_dir).await,
    }
}

/// List discovered peers
async fn list_peers(data_dir: &Path) -> anyhow::Result<()> {
    info!("🔎 Starting peer discovery...");

    let identity_path = data_dir.join("identity.key");
    let identity = NodeIdentity::load(&identity_path, None)?;
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

    info!("⏳ Waiting for peers to be discovered (10s)...");

    let discovery_future = async {
        let mut discovered_peers = HashSet::new();
        loop {
            match swarm.select_next_some().await {
                SwarmEvent::Behaviour(BehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                    for (peer, _) in list {
                        if discovered_peers.insert(peer) {
                             println!("mDNS discovered: {}", peer);
                        }
                    }
                }
                SwarmEvent::Behaviour(BehaviourEvent::Kad(kad::Event::OutboundQueryProgressed { result, .. })) => {
                    if let kad::QueryResult::GetClosestPeers(Ok(ok)) = result {
                        for peer_info in ok.peers {
                           if discovered_peers.insert(peer_info.peer_id) {
                                println!("Kademlia discovered: {:?}", peer_info.peer_id);
                           }
                        }
                    }
                }
                _ => {}
            }
        }
    };

    if timeout(Duration::from_secs(10), discovery_future).await.is_err() {
        println!("\n🏁 Discovery finished.");
    }

    Ok(())
}
