use crate::bootstrap::server::Server;

mod application;
mod bootstrap;
mod domain;
mod infra;
mod interface;

#[tokio::main]
async fn main() {
    let server: Server = Server::init().await;
    server.run().await;
}
