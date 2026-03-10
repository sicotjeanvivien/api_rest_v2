use std::sync::Arc;

use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
};

use crate::{
    domain::task::service::TaskService,
    infra::{
        http::{handlers::task_handler::TaskHandler, parser::decode_request, request::HttpMethod},
        router::{Router, route::Route},
        stores::{in_memory_task_store::InMemoryTaskStore, postgres_task_store::PostgresTaskStore},
    },
};

mod domain;
mod errors;
mod infra;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let handler: Arc<TaskHandler> = build_handler().await;
    let router: Arc<Router> = build_router(handler);
    let tcp_listener: TcpListener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    loop {
        let (stream, _addr): (tokio::net::TcpStream, _) = tcp_listener.accept().await.unwrap();
        let arc_router = Arc::clone(&router);
        tokio::spawn(async move {
            let _ = handle_connection(stream, arc_router).await;
        });
    }
}

async fn handle_connection(mut stream: TcpStream, router: Arc<Router>) -> std::io::Result<()> {
    let request = decode_request(&mut stream)
        .await
        .map_err(|_| std::io::ErrorKind::Other)?;
    let response = router.handler(request);

    stream
        .write_all(response.await.to_string().as_bytes())
        .await?;
    Ok(())
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
    // let repository = Arc::new(InMemoryTaskStore::new());
    let repository = Arc::new(PostgresTaskStore::new().await);
    let service = TaskService::new(repository).await;
    Arc::new(TaskHandler::new(service))
}
