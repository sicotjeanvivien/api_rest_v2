use crate::infra::http::{
    request::{HttpMethod, HttpRequest},
    response::HttpResponse,
};

type Handler = dyn Fn(HttpRequest) -> HttpResponse + Send + Sync + 'static;
pub struct Route {
    pub method: HttpMethod,
    pub path: String,
    pub handler: Box<Handler>,
}

impl Route {
    pub fn new(method: HttpMethod, path: String, handler: Box<Handler>) -> Self {
        Self {
            method,
            path,
            handler,
        }
    }
}
