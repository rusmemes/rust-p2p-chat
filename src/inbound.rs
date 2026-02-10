use crate::domain::{ChatBehavior, ChatBehaviorEvent, MessageRequest, MessageResponse};
use libp2p::request_response::{Event, Message};
use libp2p::swarm::SwarmEvent;
use libp2p::Swarm;

pub fn handle(swarm: &mut Swarm<ChatBehavior>, event: SwarmEvent<ChatBehaviorEvent>) {
    match event {
        SwarmEvent::NewListenAddr { address, .. } => {
            println!("Listening on {:?}", address);
        }
        SwarmEvent::ConnectionEstablished { peer_id, .. } => {
            println!("{peer_id:?} connected")
        }
        SwarmEvent::ConnectionClosed { peer_id, cause, .. } => {
            println!("{peer_id:?} disconnected, cause {cause:?}")
        }
        SwarmEvent::Behaviour(event) => match event {
            ChatBehaviorEvent::Ping(event) => {
                println!("Ping: {:?}", event);
            }
            ChatBehaviorEvent::Messaging(event) => messaging(swarm, event),
            ChatBehaviorEvent::Mdns(event) => {
                use libp2p::mdns::Event::{Discovered, Expired};
                use libp2p::swarm::dial_opts::DialOpts;

                match event {
                    Discovered(peers) => {
                        for (id, addr) in peers {

                            let opts = DialOpts::peer_id(id)
                                .addresses(vec![addr])
                                .build();

                            if let Err(e) = swarm.dial(opts) {
                                println!("Dial failed: {e}");
                            } else {
                                println!("Dialing peer {} succeeded", id);
                            }
                        }
                    }
                    Expired(_) => {}
                }
            }
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
