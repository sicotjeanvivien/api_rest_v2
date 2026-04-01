use crate::bootstrap::Server;

mod bootstrap;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server: Server = Server::init().await?;
    server.run().await;
    Ok(())
}
