use crate::bootstrap::container::Container;
use crate::interface::http::handlers::auth_handler::AuthHandler;
use crate::interface::http::handlers::task_handler::TaskHandler;
use crate::interface::http::request::HttpRequest;
use crate::interface::http::router::route::{Handler, HandlerResult};
use crate::interface::http::{
    request::HttpMethod,
    router::{route::Route, router::Router},
};
use crate::routes;
use std::sync::Arc;

pub async fn build_router(container: &Container) -> Arc<Router> {
    let task_handler = Arc::new(TaskHandler::new(container.task_service.clone()));
    let auth_handler = Arc::new(AuthHandler::new(
        container.credential_service.clone(),
        container.jwt_service.clone(),
    ));

    let router = routes![
      container.jwt_service.clone(),
      POST "/auth/login" => {
        let h = auth_handler.clone();
        route_handler(move |req| {
          let h = h.clone();
          async move { h.login(req).await }
        })
      },
      POST "/auth/register" => {
        let h = auth_handler.clone();
        route_handler(move |req| {
          let h = h.clone();
          async move { h.register(req).await }
        })
      },
      GET "/tasks/:id" => {
        let h = task_handler.clone();
        route_handler(move |req| {
          let h = h.clone();
          async move { h.get_task(req).await }
        })
      },
      GET "/tasks" => {
        let h = task_handler.clone();
        route_handler(move |req| {
          let h = h.clone();
          async move { h.get_all_task(req).await }
        })
      },
      POST "/tasks" => {
        let h = task_handler.clone();
        route_handler(move |req| {
          let h = h.clone();
          async move { h.create_task(req).await }
        })
      },
      PATCH "/tasks" => {
        let h = task_handler.clone();
        route_handler(move |req| {
          let h = h.clone();
          async move { h.update_task(req).await }
        })
      },
      DELETE "/tasks/:id" => {
        let h = task_handler.clone();
        route_handler(move |req| {
          let h = h.clone();
          async move { h.delete_task(req).await }
        })
      },
    ];

    Arc::new(router)
}

pub fn route_handler<F, Fut>(f: F) -> Box<Handler>
where
    F: Fn(HttpRequest) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = HandlerResult> + Send + 'static,
{
    Box::new(move |req| Box::pin(f(req)))
}
