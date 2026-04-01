use crate::bootstrap::{self, Container};
use anyhow::Context;
use interface::{HttpResponse, Router, decode_request};
use std::{env, io::Result, net::SocketAddr, sync::Arc};
use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
};
use tracing::{error, info};

pub(crate) struct Server {
    pub(crate) router: Arc<Router>,
    pub(crate) tcp_listener: TcpListener,
}

impl Server {
    pub(crate) async fn init() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();
        tracing_subscriber::fmt::init();

        let container = Container::build().await;
        let router = bootstrap::build_router(&container).await;
        let app_url = env::var("APP_URL").context("APP_URL must be set in .env")?;
        let tcp_listener = TcpListener::bind(app_url.clone())
            .await
            .context("app_url is invalid ")?;
        info!("Server starting on {}", app_url);
        Ok(Server {
            router,
            tcp_listener,
        })
    }

    pub(crate) async fn run(self) {
        loop {
            tokio::select! {
                res = self.tcp_listener.accept() => {
                    if let Err(e) = self.handle_accept(res).await {
                        error!(error = %e, "Accept error");
                    }
                }
                _ = tokio::signal::ctrl_c() => {
                    info!("Shutdown signal received — stopping server");
                    break;
                }
            }
        }
        info!("Server stopped");
    }

    async fn handle_accept(&self, res: Result<(TcpStream, SocketAddr)>) -> Result<()> {
        let (stream, _) = res?;
        let router = Arc::clone(&self.router);

        tokio::spawn(async move {
            handle_connection(stream, router).await;
        });

        Ok(())
    }
}

async fn handle_connection(mut stream: TcpStream, router: Arc<Router>) {
    let response: HttpResponse = match decode_request(&mut stream).await {
        Ok(request) => router.handler(request).await,
        Err(e) => HttpResponse::from(e),
    };
    if let Err(e) = stream.write_all(response.to_string().as_bytes()).await {
        tracing::error!("Failed to write response: {}", e);
    }
}
