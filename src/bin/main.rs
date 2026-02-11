use awesome_chat::chat_peer_run;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    chat_peer_run().await
}
