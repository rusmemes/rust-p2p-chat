use libp2p::kad::store::MemoryStore;
use libp2p::swarm::NetworkBehaviour;
use libp2p::{autonat, dcutr, gossipsub, identify, kad, ping, relay};

pub const CHAT_TOPIC: &str = "chat";

#[derive(NetworkBehaviour)]
pub struct ChatBehavior {
    pub ping: ping::Behaviour,
    pub identify: identify::Behaviour,
    pub kademlia: kad::Behaviour<MemoryStore>,
    pub autonat: autonat::Behaviour,
    pub relay_server: relay::Behaviour,
    pub relay_client: relay::client::Behaviour,
    pub dcutr: dcutr::Behaviour,
    pub gossipsub: gossipsub::Behaviour,
}
