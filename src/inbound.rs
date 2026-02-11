use crate::behaviour::{ChatBehavior, ChatBehaviorEvent};
use libp2p::multiaddr::Protocol;
use libp2p::swarm::SwarmEvent;
use libp2p::{autonat, gossipsub, relay, Swarm};

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
            ChatBehaviorEvent::Identify(event) => identify(swarm, event),
            ChatBehaviorEvent::Kademlia(_) => {},
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
            ChatBehaviorEvent::Gossipsub(event) => {
                use gossipsub::Event::*;
                match event {
                    Message { propagation_source, message_id, message } => {
                        if let Ok(content) = String::from_utf8(message.data) {
                            println!("{propagation_source:?} {content:?}");
                        } else {
                            println!("Got message from {propagation_source:?}: {message_id:?} but could not decode utf8");
                        }
                    }
                    _ => {
                        println!("Gossipsub: {:?}", event);
                    }
                }

            }
        },
        _ => {}
    }
}

fn autonat(_swarm: &mut Swarm<ChatBehavior>, event: autonat::Event) {
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
                swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
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
