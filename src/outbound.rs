use crate::behaviour::{ChatBehavior, CHAT_TOPIC};
use libp2p::gossipsub::IdentTopic;
use libp2p::Swarm;

pub fn handle(swarm: &mut Swarm<ChatBehavior>, line: String) {
    match swarm
        .behaviour_mut()
        .gossipsub
        .publish(IdentTopic::new(CHAT_TOPIC), line.as_bytes())
    {
        Ok(_) => {}
        Err(error) => {
            println!("Failed to publish message: {}", error);
        }
    };

    // let peers = swarm
    //     .connected_peers()
    //     .map(|it| it.clone())
    //     .collect::<Vec<_>>();
    //
    // for peer_id in peers {
    //     swarm.behaviour_mut().messaging.send_request(
    //         &peer_id,
    //         MessageRequest {
    //             message: line.clone(),
    //         },
    //     );
    //     println!("{peer_id:?} {line:?}");
    // }
}
