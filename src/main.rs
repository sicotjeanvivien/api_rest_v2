use std::sync::Arc;

use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
};

use crate::{
    application::services::task_service::TaskService,
    infra::stores::postgres_task_store::PostgresTaskStore,
    interface::http::{
        handlers::task_handler::TaskHandler, parser::decode_request, request::HttpMethod, response::{http_response::HttpResponse, into_http_response::IntoHttpResponse}, router::{Router, route::Route}
    },
};

mod application;
mod domain;
mod infra;
mod interface;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let handler: Arc<TaskHandler> = build_handler().await;
    let router: Arc<Router> = build_router(handler);
    let tcp_listener: TcpListener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    tracing::info!("Server starting on 127.0.0.1:8080");
    loop {
        let (stream, _addr): (tokio::net::TcpStream, _) = tcp_listener.accept().await.unwrap();
        let arc_router = Arc::clone(&router);
        tokio::spawn(async move {
            handle_connection(stream, arc_router).await;
        });
    }
}

async fn handle_connection(mut stream: TcpStream, router: Arc<Router>) {
    let response: HttpResponse = match decode_request(&mut stream).await {
        Ok(request) => router.handler(request).await,
        Err(e) => e.into_http_response(),
    };
    stream.write_all(response.to_string().as_bytes()).await.ok();
}

fn build_router(handler: Arc<TaskHandler>) -> Arc<Router> {
    let router = routes![
      GET "/tasks/:id" => {
        let handler = handler.clone();
        move |req| Box::pin({
            let handler = handler.clone();
            async move {handler.get_task(req).await}
          })
      },
      GET "/tasks" => {
        let handler = handler.clone();
        move |req| Box::pin({
            let handler = handler.clone();
            async move { handler.get_all_task(req).await }
          })
      },
      POST "/tasks" => {
        let handler = handler.clone();
        move |req| Box::pin({
            let handler = handler.clone();
            async move {handler.create_task(req).await}
          })
      },
      PATCH "/tasks" => {
        let handler = handler.clone();
        move |req| Box::pin({
            let handler = handler.clone();
            async move {handler.update_task(req).await}
          })
      },
      DELETE "/tasks/:id" => {
        let handler = handler.clone();
        move |req| Box::pin({
            let handler = handler.clone();
            async move {handler.delete_task(req).await}
          })
      },
    ];

    Arc::new(router)
}

async fn build_handler() -> Arc<TaskHandler> {
    let repository = Arc::new(PostgresTaskStore::new().await);
    let service = TaskService::new(repository).await;
    Arc::new(TaskHandler::new(service))
}
