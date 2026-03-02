use std::collections::HashMap;

pub struct Request {
    method: HttpMethod,
    path: String,
    http_version: String,
    headers: HashMap<String, String>,
    body: Option<String>,
}

impl Request {
    pub fn new(
        method: HttpMethod,
        path: String,
        http_version: String,
        headers: HashMap<String, String>,
        body: Option<String>,
    ) -> Self {
        Request {
            method,
            path,
            http_version,
            headers,
            body,
        }
    }
}

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
