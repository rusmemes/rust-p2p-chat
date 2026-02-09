use libp2p::futures::StreamExt;
use libp2p::ping::Config;
use libp2p::request_response::{Event, Message, json};
use libp2p::swarm::{NetworkBehaviour, SwarmEvent};
use libp2p::{Multiaddr, PeerId, StreamProtocol, Swarm, noise, ping, request_response, tcp, yamux};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::select;

#[derive(Serialize, Debug, Deserialize, Clone)]
struct MessageRequest {
    message: String,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
struct MessageResponse {
    ack: bool,
}

#[derive(NetworkBehaviour)]
struct ChatBehavior {
    ping: ping::Behaviour,
    messaging: json::Behaviour<MessageRequest, MessageResponse>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let port = std::env::var("CHAT_P2P_PORT")
        .unwrap_or_else(|_| "9999".to_owned())
        .parse::<u16>()?;

    let peer: Multiaddr = std::env::var("CHAT_PEER")?.parse()?;

    let mut swarm: Swarm<ChatBehavior> = libp2p::SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_behaviour(|_key_pair| {
            Ok(ChatBehavior {
                ping: ping::Behaviour::new(Config::new().with_interval(Duration::from_secs(10))),
                messaging: json::Behaviour::new(
                    [(
                        StreamProtocol::new("/awesome-chat/1"),
                        request_response::ProtocolSupport::Full,
                    )],
                    request_response::Config::default(),
                ),
            })
        })?
        .with_swarm_config(|config| config.with_idle_connection_timeout(Duration::from_secs(30)))
        .build();

    swarm.listen_on(format!("/ip4/0.0.0.0/tcp/{}", port).parse()?)?;
    swarm.dial(peer.clone())?;

    println!("Peer ID: {:?}", swarm.local_peer_id());

    let mut stdin = BufReader::new(tokio::io::stdin()).lines();
    let mut target_peer_id = None;

    loop {
        select! {
            event = swarm.select_next_some() => inbound(&mut swarm, event, &mut target_peer_id),
            Ok(Some(line)) = stdin.next_line() => outbound(&mut swarm, line, &target_peer_id)
        }
    }
}

fn inbound(
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
            ChatBehaviorEvent::Messaging(event) => match event {
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
                    Message::Response { request_id, response } => {
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
            },
        },
        _ => {}
    }
}

fn outbound(swarm: &mut Swarm<ChatBehavior>, line: String, target_peer_id: &Option<PeerId>) {
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
