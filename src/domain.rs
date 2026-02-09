use libp2p::ping;
use libp2p::request_response::json;
use libp2p::swarm::NetworkBehaviour;
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
}
