use std::collections::HashMap;

pub struct HttpRequest {
    pub method: HttpMethod,
    pub path: String,
    pub http_version: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

impl HttpRequest {
    pub fn new(
        method: HttpMethod,
        path: String,
        http_version: String,
        headers: HashMap<String, String>,
        body: Option<String>,
    ) -> Self {
        HttpRequest {
            method,
            path,
            http_version,
            headers,
            body,
        }
    }
}

#[derive(PartialEq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    HEAD,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
}
