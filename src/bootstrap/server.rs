use crate::{
    bootstrap::{self, container::{self, Container}},
    interface::http::{
        parser::decode_request,
        response::{http_response::HttpResponse, into_http_response::IntoHttpResponse},
        router::router::Router,
    },
};
use std::sync::Arc;
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
        let tcp_listener: TcpListener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
        info!("Server starting on 127.0.0.1:8080");
        Server {
            router,
            tcp_listener,
        }
    }

    pub async fn run(self) {
        loop {
            let (stream, _addr): (TcpStream, _) = self.tcp_listener.accept().await.unwrap();
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
        Err(e) => e.into_http_response(),
    };
    if let Err(e) = stream.write_all(response.to_string().as_bytes()).await {
        tracing::error!("Failed to write response: {}", e);
    }
}
