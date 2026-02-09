use crate::domain::{ChatBehavior, MessageRequest};
use libp2p::{PeerId, Swarm};

pub fn handle(swarm: &mut Swarm<ChatBehavior>, line: String, target_peer_id: &Option<PeerId>) {
    if let Some(peer_id) = target_peer_id {
        swarm.behaviour_mut().messaging.send_request(
            peer_id,
            MessageRequest {
                message: line.clone(),
            },
        );
        println!("{} {line:?}", swarm.local_peer_id());
    }
}
