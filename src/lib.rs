mod behaviour;
mod inbound;
mod outbound;
mod swarm;

use crate::swarm::create_swarm;
use libp2p::futures::StreamExt;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::select;

pub async fn chat_peer_run() -> anyhow::Result<()> {
    let mut swarm = create_swarm()?;

    println!("Peer ID: {:?}", swarm.local_peer_id());

    let mut stdin = BufReader::new(tokio::io::stdin()).lines();

    loop {
        select! {
            event = swarm.select_next_some() => inbound::handle(&mut swarm, event),
            Ok(Some(line)) = stdin.next_line() => outbound::handle(&mut swarm, line)
        }
    }
}
