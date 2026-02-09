mod domain;
mod inbound;
mod outbound;

use crate::domain::ChatBehavior;
use libp2p::futures::StreamExt;
use libp2p::ping::Config;
use libp2p::request_response::json;
use libp2p::{noise, ping, request_response, tcp, yamux, Multiaddr, StreamProtocol, Swarm};
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::select;

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
            event = swarm.select_next_some() => inbound::handle(&mut swarm, event, &mut target_peer_id),
            Ok(Some(line)) = stdin.next_line() => outbound::handle(&mut swarm, line, &target_peer_id)
        }
    }
}
