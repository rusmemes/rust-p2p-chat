use crate::domain::{ChatBehavior, ChatBehaviorEvent, MessageRequest, MessageResponse};
use libp2p::request_response::{Event, Message};
use libp2p::swarm::SwarmEvent;
use libp2p::{PeerId, Swarm};

pub fn handle(
    swarm: &mut Swarm<ChatBehavior>,
    event: SwarmEvent<ChatBehaviorEvent>,
    target_peer_id: &mut Option<PeerId>,
) {
    match event {
        SwarmEvent::NewListenAddr { address, .. } => {
            println!("Listening on {:?}", address);
        }
        SwarmEvent::ConnectionEstablished { peer_id, .. } => {
            if target_peer_id.is_none() {
                target_peer_id.replace(peer_id);
            }
        }
        SwarmEvent::ConnectionClosed { peer_id, .. } => {
            if let Some(tpi) = target_peer_id {
                if *tpi == peer_id {
                    target_peer_id.take();
                }
            }
        }
        SwarmEvent::Behaviour(event) => match event {
            ChatBehaviorEvent::Ping(event) => {
                println!("Ping: {:?}", event);
            }
            ChatBehaviorEvent::Messaging(event) => messaging(swarm, event),
        },
        _ => {}
    }
}

fn messaging(swarm: &mut Swarm<ChatBehavior>, event: Event<MessageRequest, MessageResponse>) {
    match event {
        Event::Message {
            peer,
            connection_id,
            message,
        } => match message {
            Message::Request {
                request_id,
                request,
                channel,
            } => {
                println!("{peer} {:?}", request.message);
                if let Err(error) = swarm
                    .behaviour_mut()
                    .messaging
                    .send_response(channel, MessageResponse { ack: true })
                {
                    println!("Error sending response: {:?}", error);
                }
            }
            Message::Response {
                request_id,
                response,
            } => {
                println!("{peer} {:?}", response);
            }
        },
        Event::OutboundFailure {
            peer,
            connection_id,
            request_id,
            error,
        } => {
            println!(
                "OutboundFailure from {:?} to {:?}: {:?}",
                peer, request_id, error
            );
        }
        Event::InboundFailure {
            peer,
            connection_id,
            request_id,
            error,
        } => {
            println!(
                "InboundFailure from {:?} to {:?}: {:?}",
                peer, request_id, error
            );
        }
        Event::ResponseSent { .. } => {}
    }
}
