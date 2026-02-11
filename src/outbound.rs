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
}
