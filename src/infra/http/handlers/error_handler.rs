use std::collections::HashMap;

use crate::infra::http::response::{HttpResponse, StatusCode};

pub struct ErrorHandler {}

impl ErrorHandler {
    pub fn internal_server_error() -> HttpResponse {
        eprintln!("internal_server_error");
        HttpResponse::new(StatusCode::InternalServerError, HashMap::new(), None)
    }

    pub fn not_found() -> HttpResponse {
        eprintln!("not_found");
        HttpResponse::new(StatusCode::NotFound, HashMap::new(), None)
    }

    pub fn bad_request() -> HttpResponse {
        eprintln!("bad_request");
        HttpResponse::new(StatusCode::BadRequest, HashMap::new(), None)
    }

    pub fn unprocessable_entity() -> HttpResponse {
        eprintln!("unprocessable_entity");
        HttpResponse::new(StatusCode::UnprocessableEntity, HashMap::new(), None)
    }
}
