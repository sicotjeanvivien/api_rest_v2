mod macros;
pub mod route;

use std::collections::HashMap;

use crate::infra::{
    http::{
        handlers::error_handler::ErrorHandler,
        request::{ HttpMethod, HttpRequest},
        response::HttpResponse,
    },
    router::route::{Handler, Route},
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
    ) -> Option<(&Handler, HashMap<String, String>)> {
        for route in self.routes.iter() {
            if route.method != *method {
                continue;
            }

            let path_segments: Vec<&str> = path.split('/').collect();
            let route_segments: Vec<&str> = route.path.split('/').collect();

            if path_segments.len() != route_segments.len() {
                continue;
            }

            let mut params = HashMap::new();
            let mut matched = true;

            for (route_seg, path_seg) in route_segments.iter().zip(path_segments.iter()) {
                if route_seg.starts_with(':') {
                    params.insert(route_seg[1..].to_string(), path_seg.to_string());
                } else if route_seg != path_seg {
                    matched = false;
                    break;
                }
            }

            if matched {
                return Some((&route.handler, params));
            }
        }
        None
    }

    pub fn handler(&self, mut request: HttpRequest) -> HttpResponse {
        self.find_handler(&request.method, &request.path)
            .map(|(handler, params)| {
                request.params.extend(params);
                handler(request).unwrap_or_else(|err| err)
            })
            .unwrap_or_else(|| ErrorHandler::not_found())
    }
}
