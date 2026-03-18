use std::{collections::HashMap, sync::Arc};

use tracing::info;

use crate::{
    application::security::jwt_service::JwtService,
    interface::http::{
        handlers::error_handler::ErrorHandler,
        request::{HttpMethod, HttpRequest},
        response::http_response::HttpResponse,
        router::route::{Handler, Route},
    },
};

pub struct Router {
    routes: Vec<Route>,
    jwt_service: Arc<JwtService>,
}

impl Router {
    pub fn new(routes: Vec<Route>, jwt_service: Arc<JwtService>) -> Self {
        Self {
            routes,
            jwt_service,
        }
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

    pub async fn handler(&self, mut request: HttpRequest) -> HttpResponse {
        if !request.path.starts_with("/auth") {
            let Some(auth_header) = request.headers.get("Authorization") else {
                return ErrorHandler::unauthorized("Missing Authorization header");
            };

            let Some(token) = auth_header.strip_prefix("Bearer ") else {
                return ErrorHandler::unauthorized("Invalid Authorization format");
            };

            if let Err(err) = self.jwt_service.verify(token) {
                info!("JWT verification failed: {:?}", err);
                return ErrorHandler::unauthorized("Invalid token");
            }
        }
        match self.find_handler(&request.method, &request.path) {
            Some((handler, params)) => {
                request.params.extend(params);
                handler(request).await.unwrap_or_else(|err| err)
            }
            None => ErrorHandler::not_found("Page Not Found."),
        }
    }
}
