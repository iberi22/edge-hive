
use libp2p::{identity, PeerId};
use libp2p::futures::StreamExt;
use libp2p::kad::{store::MemoryStore, Behaviour, Config};
use std::time::Duration;
use tokio::time;

#[tokio::test]
async fn test_libp2p_discovery() {
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());

    let mut swarm = libp2p::SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            Default::default(),
            libp2p::noise::Config::new,
            libp2p::yamux::Config::default,
        )
        .unwrap()
        .with_behaviour(|_| {
            let mut cfg = Config::default();
            cfg.set_query_timeout(Duration::from_secs(5 * 60));
            let store = MemoryStore::new(local_peer_id);
            Behaviour::with_config(local_peer_id, store, cfg)
        })
        .unwrap()
        .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(30)))
        .build();

    swarm.listen_on("/ip4/127.0.0.1/tcp/0".parse().unwrap()).unwrap();

    // Wait for the swarm to start listening
    time::sleep(Duration::from_secs(1)).await;

    let mut swarm2 = libp2p::SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            Default::default(),
            libp2p::noise::Config::new,
            libp2p::yamux::Config::default,
        )
        .unwrap()
        .with_behaviour(|key| {
            let peer_id = key.public().to_peer_id();
            let mut cfg = Config::default();
            cfg.set_query_timeout(Duration::from_secs(5 * 60));
            let store = MemoryStore::new(peer_id);
            Behaviour::with_config(peer_id, store, cfg)
        })
        .unwrap()
        .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(30)))
        .build();

    let addr = swarm.listeners().next().unwrap();
    swarm2.behaviour_mut().add_address(&local_peer_id, addr.clone());
    swarm2.behaviour_mut().bootstrap().unwrap();

    let mut discovered = false;
    for _ in 0..10 {
        let event = swarm2.select_next_some().await;
        if let libp2p::swarm::SwarmEvent::Behaviour(libp2p::kad::Event::OutboundQueryProgressed { result, .. }) = event {
            if let libp2p::kad::QueryResult::GetClosestPeers(Ok(ok)) = result {
                for peer in ok.peers {
                    if peer == local_peer_id {
                        discovered = true;
                        break;
                    }
                }
            }
        }
        if discovered {
            break;
        }
        time::sleep(Duration::from_secs(1)).await;
    }

    assert!(discovered);
}
