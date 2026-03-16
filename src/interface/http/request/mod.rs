use std::collections::HashMap;

use crate::interface::http::{
    error::http_error::HttpError,
    response::{http_response::HttpResponse, into_http_response::IntoHttpResponse},
};

#[derive(Debug)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub path: String,
    pub params: HashMap<String, String>,
    pub http_version: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

impl HttpRequest {
    pub fn new(
        method: HttpMethod,
        path: String,
        params: HashMap<String, String>,
        http_version: String,
        headers: HashMap<String, String>,
        body: Option<String>,
    ) -> Self {
        HttpRequest {
            method,
            path,
            params,
            http_version,
            headers,
            body,
        }
    }

    pub fn get_value_by_key(&self, key: String) -> Result<&String, HttpError> {
        if self.params.contains_key(&key) {
            if let Some(x) = self.params.get(&key) {
                return Ok(x);
            }
        }
        Err(HttpError::ParamNotFound(format!(
            "{} not found in path",
            key
        )))
    }

    pub fn get_body(&self) -> Result<String, HttpResponse> {
        self.body.clone().ok_or_else(|| {
            HttpError::BadRequest("body is not found".to_string()).into_http_response()
        })
    }
}

#[derive(PartialEq, Debug)]
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
