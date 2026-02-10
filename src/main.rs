mod domain;
mod inbound;
mod outbound;

use crate::domain::ChatBehavior;
use libp2p::futures::StreamExt;
use libp2p::ping::Config;
use libp2p::request_response::json;
use libp2p::{
    mdns, noise, ping, request_response, tcp, yamux, StreamProtocol, Swarm,
};
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::select;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut swarm: Swarm<ChatBehavior> = libp2p::SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_behaviour(|key_pair| {
            Ok(ChatBehavior {
                ping: ping::Behaviour::new(Config::new().with_interval(Duration::from_secs(10))),
                messaging: json::Behaviour::new(
                    [(
                        StreamProtocol::new("/awesome-chat/1"),
                        request_response::ProtocolSupport::Full,
                    )],
                    request_response::Config::default(),
                ),
                mdns: mdns::Behaviour::new(
                    mdns::Config::default(),
                    key_pair.public().to_peer_id(),
                )?,
            })
        })?
        .with_swarm_config(|config| config.with_idle_connection_timeout(Duration::from_secs(60)))
        .build();

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    println!("Peer ID: {:?}", swarm.local_peer_id());

    let mut stdin = BufReader::new(tokio::io::stdin()).lines();

    loop {
        select! {
            event = swarm.select_next_some() => inbound::handle(&mut swarm, event),
            Ok(Some(line)) = stdin.next_line() => outbound::handle(&mut swarm, line)
        }
    }
}
