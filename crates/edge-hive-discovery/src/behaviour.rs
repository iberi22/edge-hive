
use libp2p::{
    identify, kad,
    kad::{store::MemoryStore, Behaviour as KadBehaviour},
    mdns,
    swarm::NetworkBehaviour,
};
use tracing::debug;

/// Combined network behaviour for the Hive node
///
/// This struct combines all the network behaviours required for node discovery
/// and peer management.
#[derive(NetworkBehaviour)]
#[behaviour(to_swarm = "HiveBehaviourEvent")]
pub struct HiveBehaviour {
    /// mDNS for local peer discovery
    pub mdns: mdns::tokio::Behaviour,

    /// Kademlia for global peer discovery in the DHT
    pub kademlia: KadBehaviour<MemoryStore>,

    /// Identify for getting peer information
    pub identify: identify::Behaviour,
}

/// Events emitted by the HiveBehaviour to the Swarm
#[derive(Debug)]
pub enum HiveBehaviourEvent {
    Mdns(mdns::Event),
    Kademlia(kad::Event),
    Identify(identify::Event),
}

impl From<mdns::Event> for HiveBehaviourEvent {
    fn from(event: mdns::Event) -> Self {
        debug!("mDNS event: {:?}", event);
        HiveBehaviourEvent::Mdns(event)
    }
}

impl From<kad::Event> for HiveBehaviourEvent {
    fn from(event: kad::Event) -> Self {
        debug!("Kademlia event: {:?}", event);
        HiveBehaviourEvent::Kademlia(event)
    }
}

impl From<identify::Event> for HiveBehaviourEvent {
    fn from(event: identify::Event) -> Self {
        debug!("Identify event: {:?}", event);
        HiveBehaviourEvent::Identify(event)
    }
}
