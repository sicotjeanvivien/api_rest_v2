use std::{
    io::Write,
    net::{TcpListener, TcpStream},
    sync::Arc,
};

use crate::{
    domain::task::service::TaskService,
    infra::{
        http::{handlers::task_handler::TaskHandler, parser::decode_request, request::HttpMethod},
        router::{Router, route::Route},
        stores::in_memory_task_store::InMemoryTaskStore,
    },
};

mod domain;
mod errors;
mod infra;

fn main() {
    let handler = build_handler();
    let router = build_router(handler);
    let tcp_listener: TcpListener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in tcp_listener.incoming() {
        println!("new connexion");
        match stream {
            Ok(stream) => {
                let arc_router = Arc::clone(&router);
                std::thread::spawn(move || {
                    if let Err(e) = handle_connection(stream, arc_router) {
                        eprintln!("Connection error: {}", e);
                    }
                });
            }
            Err(e) => {
                eprintln!("Échec de la connexion : {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream, router: Arc<Router>) -> std::io::Result<()> {
    let request = decode_request(&mut stream).map_err(|_| std::io::ErrorKind::Other)?;
    let response = router.handler(request);
    stream.write_all(response.to_string().as_bytes())?;
    Ok(())
}

fn build_router(handler: Arc<TaskHandler>) -> Arc<Router> {
    let router = routes![
      GET "/tasks/:id" => {
          let handler = handler.clone();
          move |req| handler.get_task(req)
      },
      GET "/tasks" => {
          let handler = handler.clone();
          move |req| handler.get_all_task(req)
      },
      POST "/tasks" => {
          let handler = handler.clone();
          move |req| handler.create_task(req)
      },
      PATCH "/tasks" => {
        let handler = handler.clone();
        move |req| handler.update_task(req)
      },
      DELETE "/tasks/:id" => {
        let handler = handler.clone();
        move |req| handler.delete_task(req)
      },

    ];

    Arc::new(router)
}

fn build_handler() -> Arc<TaskHandler> {
    let repository = Arc::new(InMemoryTaskStore::new());
    let service = TaskService::new(repository);
    Arc::new(TaskHandler::new(service))
}
