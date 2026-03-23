use crate::{
    bootstrap::{self, container::Container},
    interface::http::{
        parser::decode_request, response::http_response::HttpResponse, router::router::Router,
    },
};
use std::{env, sync::Arc};
use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
};
use tracing::info;

pub struct Server {
    pub router: Arc<Router>,
    pub tcp_listener: TcpListener,
}

impl Server {
    pub async fn init() -> Self {
        dotenvy::dotenv().ok();
        tracing_subscriber::fmt::init();

        let container = Container::build().await;
        let router = bootstrap::router::build_router(&container).await;
        let app_url = env::var("APP_URL").expect("APP_URL must be set in .env");
        let tcp_listener: TcpListener = TcpListener::bind(app_url.clone())
            .await
            .expect("app_url is invalid ");
        info!("Server starting on {}", app_url);
        Server {
            router,
            tcp_listener,
        }
    }

    pub async fn run(self) {
        loop {
            let (stream, _addr): (TcpStream, _) = self
                .tcp_listener
                .accept()
                .await
                .expect("couldn't get client");
            let arc_router = Arc::clone(&self.router);
            tokio::spawn(async move {
                handle_connection(stream, arc_router).await;
            });
        }
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
