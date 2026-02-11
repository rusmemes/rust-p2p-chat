use crate::domain::{ChatBehavior, MessageRequest};
use libp2p::Swarm;

pub fn handle(swarm: &mut Swarm<ChatBehavior>, line: String) {
    let peers = swarm
        .connected_peers()
        .map(|it| it.clone())
        .collect::<Vec<_>>();

    for peer_id in peers {
        swarm.behaviour_mut().messaging.send_request(
            &peer_id,
            MessageRequest {
                message: line.clone(),
            },
        );
        println!("{peer_id:?} {line:?}");
    }
}
