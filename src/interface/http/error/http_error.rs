use crate::interface::http::{
    handlers::error_handler::ErrorHandler,
    response::{http_response::HttpResponse, into_http_response::IntoHttpResponse},
};

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum HttpError {
    #[error("Method Not Found: {0}")]
    MethodNotFound(String),
    #[error("Param Not Found: {0}")]
    ParamNotFound(String),
    #[error("Bad Request: {0}")]
    BadRequest(String),
    #[error("Internal server error: {0}")]
    InternalServerError(String),
}

impl IntoHttpResponse for HttpError {
    fn into_http_response(self) -> HttpResponse {
        match self {
            HttpError::BadRequest(msg) => ErrorHandler::bad_request(&msg),
            HttpError::MethodNotFound(msg) => ErrorHandler::method_not_found(&msg),
            HttpError::ParamNotFound(msg) => ErrorHandler::bad_request(&msg),
            HttpError::InternalServerError(msg) => ErrorHandler::internal_server_error(&msg),
        }
    }
}
