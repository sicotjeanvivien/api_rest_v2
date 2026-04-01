use std::collections::HashMap;
use tracing::error;

use crate::{ApiError, HttpResponse, StatusCode};

pub struct ErrorHandler {}

impl ErrorHandler {
    pub fn internal_server_error(message: &str) -> HttpResponse {
        error!(code = StatusCode::InternalServerError.to_string(), message);
        HttpResponse::new(
            StatusCode::InternalServerError,
            Self::build_header(),
            Self::build_body(StatusCode::InternalServerError, message),
        )
    }

    pub fn not_found(message: &str) -> HttpResponse {
        error!(code = StatusCode::NotFound.to_string(), message);
        HttpResponse::new(
            StatusCode::NotFound,
            Self::build_header(),
            Self::build_body(StatusCode::NotFound, message),
        )
    }

    pub fn bad_request(message: &str) -> HttpResponse {
        error!(code = StatusCode::BadRequest.to_string(), message);
        HttpResponse::new(
            StatusCode::BadRequest,
            Self::build_header(),
            Self::build_body(StatusCode::BadRequest, message),
        )
    }

    #[allow(dead_code)]
    pub fn unprocessable_entity(message: &str) -> HttpResponse {
        error!(code = StatusCode::UnprocessableEntity.to_string(), message);
        HttpResponse::new(
            StatusCode::UnprocessableEntity,
            Self::build_header(),
            Self::build_body(StatusCode::UnprocessableEntity, message),
        )
    }

    pub fn method_not_found(message: &str) -> HttpResponse {
        error!(code = StatusCode::UnprocessableEntity.to_string(), message);
        HttpResponse::new(
            StatusCode::UnprocessableEntity,
            Self::build_header(),
            Self::build_body(StatusCode::UnprocessableEntity, message),
        )
    }

    pub fn unauthorized(message: &str) -> HttpResponse {
        error!(code = StatusCode::Unauthorized.to_string(), message);
        HttpResponse::new(
            StatusCode::Unauthorized,
            Self::build_header(),
            Self::build_body(StatusCode::Unauthorized, message),
        )
    }

    fn build_header() -> HashMap<String, String> {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("X-Content-Type-Options".to_string(), "nosniff".to_string());
        headers.insert("X-Frame-Options".to_string(), "DENY".to_string());
        headers.insert("Access-Control-Allow-Origin".to_string(), "*".to_string());
        headers.insert(
            "Access-Control-Allow-Methods".to_string(),
            "GET, POST, PATCH, DELETE".to_string(),
        );
        headers.insert(
            "Access-Control-Allow-Headers".to_string(),
            "Content-Type".to_string(),
        );
        headers.insert("Cache-Control".to_string(), "no-store".to_string());
        headers
    }

    fn build_body(status_code: StatusCode, message: &str) -> Option<String> {
        serde_json::to_string(&ApiError {
            code: status_code.to_u16(),
            message: message.to_string(),
        })
        .ok()
    }
}
