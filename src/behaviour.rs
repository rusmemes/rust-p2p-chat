use libp2p::kad::store::MemoryStore;
use libp2p::request_response::json;
use libp2p::swarm::NetworkBehaviour;
use libp2p::{autonat, dcutr, gossipsub, identify, kad, ping, relay};
use serde::{Deserialize, Serialize};

pub const CHAT_TOPIC: &str = "chat";

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
    pub identify: identify::Behaviour,
    pub kademlia: kad::Behaviour<MemoryStore>,
    pub autonat: autonat::Behaviour,
    pub relay_server: relay::Behaviour,
    pub relay_client: relay::client::Behaviour,
    pub dcutr: dcutr::Behaviour,
    pub gossipsub: gossipsub::Behaviour,
}
