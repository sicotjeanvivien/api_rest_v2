pub mod route;
mod macros;

use crate::infra::{
    http::{
        handlers::error_handler::ErrorHandler,
        request::{self, HttpMethod, HttpRequest},
        response::HttpResponse,
    },
    router::route::Route,
};

pub struct Router {
    routes: Vec<Route>,
}

impl Router {
    pub fn new(routes: Vec<Route>) -> Self {
        Router { routes }
    }

    pub fn add_route(mut self, route: Route) -> Self {
        self.routes.push(route);
        self
    }

    pub fn find_handler(
        &self,
        method: &HttpMethod,
        path: &str,
    ) -> Option<&(dyn Fn(HttpRequest) -> HttpResponse + Send + Sync)> {
        self.routes
            .iter()
            .find(|r| r.method == *method && r.path == path)
            .map(|r| r.handler.as_ref())
    }

    pub fn handler(&self, request: HttpRequest) -> HttpResponse {
        self.find_handler(&request.method, &request.path)
            .map(|handler| handler(request))
            .unwrap_or_else(|| ErrorHandler::internal_server_error())
    }
}
