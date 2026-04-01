use std::future::Future;
use std::pin::Pin;

use crate::{HttpMethod, HttpRequest, HttpResponse};

pub  type HandlerResult = Result<HttpResponse, HttpResponse>;
pub  type Handler = dyn Fn(HttpRequest) -> Pin<Box<dyn Future<Output = HandlerResult> + Send>>
    + Send
    + Sync
    + 'static;
pub  struct Route {
    pub  method: HttpMethod,
    pub  path: String,
    pub  handler: Box<Handler>,
}

impl Route {
    pub  fn new(method: HttpMethod, path: String, handler: Box<Handler>) -> Self {
        Self {
            method,
            path,
            handler,
        }
    }
}
