use crate::bootstrap::container::Container;
use crate::interface::http::handlers::task_handler::TaskHandler;
use crate::interface::http::{
    request::HttpMethod,
    router::{route::Route, router::Router},
};
use crate::routes;
use std::sync::Arc;

pub async fn build_router(container: &Container) -> Arc<Router> {
    let handler = Arc::new(TaskHandler::new(container.task_service.clone()));
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
