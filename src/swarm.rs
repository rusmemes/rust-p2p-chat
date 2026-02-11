use crate::behaviour::ChatBehavior;
use anyhow::{anyhow, Context};
use libp2p::kad::store::MemoryStore;
use libp2p::kad::Mode;
use libp2p::multiaddr::Protocol;
use libp2p::ping::Config;
use libp2p::request_response::json;
use libp2p::{
    autonat, dcutr, identify, kad, noise, ping, relay, request_response, tcp, yamux,
    Multiaddr, StreamProtocol, Swarm,
};
use std::env;
use std::time::Duration;

pub fn create_swarm() -> anyhow::Result<Swarm<ChatBehavior>> {
    let bootstrap_peers = bootstrap_peers();

    let mut swarm: Swarm<ChatBehavior> = libp2p::SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_relay_client(noise::Config::new, yamux::Config::default)?
        .with_behaviour(|key_pair, relay_client| {
            let mut kad_conf = kad::Config::new(StreamProtocol::new("/awesome-chat/kad/1.0.0"));
            kad_conf.set_periodic_bootstrap_interval(Some(Duration::from_secs(10)));

            Ok(ChatBehavior {
                ping: ping::Behaviour::new(Config::new().with_interval(Duration::from_secs(10))),
                messaging: json::Behaviour::new(
                    [(
                        StreamProtocol::new("/awesome-chat/1"),
                        request_response::ProtocolSupport::Full,
                    )],
                    request_response::Config::default(),
                ),
                identify: identify::Behaviour::new(identify::Config::new(
                    "1.0.0".to_string(),
                    key_pair.public(),
                )),
                kademlia: kad::Behaviour::with_config(
                    key_pair.public().to_peer_id(),
                    MemoryStore::new(key_pair.public().to_peer_id()),
                    kad_conf,
                ),
                autonat: autonat::Behaviour::new(
                    key_pair.public().to_peer_id(),
                    autonat::Config::default(),
                ),
                relay_server: relay::Behaviour::new(
                    key_pair.public().to_peer_id(),
                    relay::Config::default(),
                ),
                relay_client,
                dcutr: dcutr::Behaviour::new(key_pair.public().to_peer_id()),
            })
        })?
        .with_swarm_config(|config| config.with_idle_connection_timeout(Duration::from_secs(60)))
        .build();

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
    swarm.behaviour_mut().kademlia.set_mode(Some(Mode::Server));

    if let Some(bootstrap_peers) = bootstrap_peers {
        for bootstrap_peer_str in bootstrap_peers {
            let addr: Multiaddr = bootstrap_peer_str
                .parse()
                .context("No Peer ID found in address!")?;

            if let Some(Protocol::P2p(peer_id)) = addr.iter().last() {
                swarm.behaviour_mut().kademlia.add_address(&peer_id, addr);
            } else {
                return Err(anyhow!(anyhow!(
                    "Peer ID does not exist in {bootstrap_peer_str}!"
                )));
            }
        }
    }

    Ok(swarm)
}

fn bootstrap_peers() -> Option<Vec<String>> {
    env::var("CHAT_BOOTSTRAP_PEERS")
        .ok()
        .map(|s| s.split(',').map(|p| p.to_string()).collect::<Vec<String>>())
}
