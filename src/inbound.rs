use crate::behaviour::{ChatBehavior, ChatBehaviorEvent, MessageRequest, MessageResponse};
use libp2p::multiaddr::Protocol;
use libp2p::request_response::{Event, Message};
use libp2p::swarm::SwarmEvent;
use libp2p::{autonat, relay, Swarm};

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
            ChatBehaviorEvent::Identify(event) => identify(swarm, event),
            ChatBehaviorEvent::Kademlia(event) => kademlia(swarm, event),
            ChatBehaviorEvent::Autonat(event) => autonat(swarm, event),
            ChatBehaviorEvent::RelayServer(event) => {
                println!("Relay server: {:?}", event);
            }
            ChatBehaviorEvent::RelayClient(event) => {
                println!("Relay client: {:?}", event);
            }
            ChatBehaviorEvent::Dcutr(event) => {
                println!("Dcutr: {:?}", event);
            }
        },
        _ => {}
    }
}

fn autonat(swarm: &mut Swarm<ChatBehavior>, event: autonat::Event) {
    use autonat::Event::*;
    match event {
        InboundProbe(event) => {
            println!("Inbound Probe {event:?}")
        }
        OutboundProbe(event) => {
            println!("Outbound Probe {event:?}")
        }
        StatusChanged { old, new } => {
            println!("Status Changed from {old:?} to {new:?}")
        }
    }
}

fn kademlia(swarm: &mut Swarm<ChatBehavior>, event: libp2p::kad::Event) {
    use libp2p::kad::Event::*;
    match event {
        InboundRequest { .. } => {}
        OutboundQueryProgressed { .. } => {}
        RoutingUpdated {
            peer,
            is_new_peer,
            addresses,
            ..
        } => {
            println!("RoutingUpdated {peer:?} - {addresses:?}");
            let mut iterator = addresses.iter().cloned();
            while let Some(addr) = iterator.next() {
                if let Err(error) = swarm.dial(addr.clone()) {
                    println!("Dialing address {:?} failed: {}", addr, error);
                }
            }
        }
        UnroutablePeer { .. } => {}
        RoutablePeer { .. } => {}
        PendingRoutablePeer { .. } => {}
        ModeChanged { .. } => {}
    }
}

fn identify(swarm: &mut Swarm<ChatBehavior>, event: libp2p::identify::Event) {
    use libp2p::identify::Event::*;
    match event {
        Received {
            connection_id,
            peer_id,
            info,
        } => {
            let is_relay = info
                .protocols
                .iter()
                .any(|protocol| *protocol == relay::HOP_PROTOCOL_NAME);

            for addr in info.listen_addrs {
                swarm.behaviour_mut().kademlia.add_address(&peer_id, addr.clone());
                if is_relay {
                    let listen_addr = addr.with_p2p(peer_id).unwrap().with(Protocol::P2pCircuit);
                    println!("Trying to listening on {:?}", listen_addr);
                    if let Err(error) = swarm.listen_on(listen_addr) {
                        println!("Error while listening on {:?}", error);
                    }
                }
            }
        }
        Sent { .. } => {}
        Pushed { .. } => {}
        Error { .. } => {}
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
            Message::Response { .. } => {}
        },
        Event::OutboundFailure { .. } => {}
        Event::InboundFailure { .. } => {}
        Event::ResponseSent { .. } => {}
    }
}
