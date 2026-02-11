use libp2p::kad::store::MemoryStore;
use libp2p::mdns::tokio::Tokio;
use libp2p::request_response::json;
use libp2p::swarm::behaviour::toggle;
use libp2p::swarm::NetworkBehaviour;
use libp2p::{identify, kad, mdns, ping};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct MessageRequest {
    pub message: String,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct MessageResponse {
    pub ack: bool,
}

#[derive(NetworkBehaviour)]
pub struct ChatBehavior {
    pub ping: ping::Behaviour,
    pub messaging: json::Behaviour<MessageRequest, MessageResponse>,
    pub mdns: toggle::Toggle<mdns::Behaviour<Tokio>>,
    pub identify: identify::Behaviour,
    pub kademlia: kad::Behaviour<MemoryStore>
}
