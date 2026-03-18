use crate::interface::http::{
    handlers::error_handler::ErrorHandler,
    response::{http_response::HttpResponse, into_http_response::IntoHttpResponse},
};

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum RepositoryError {
    #[error("Bad Request: {0}")]
    BadRequest(String),
    #[error("Data Not Found: {0}")]
    NotFound(String),
    #[error("Internal server Error: {0}")]
    Internal(String),
    #[error("Invalid Credentials : {0}")]
    InvalidCredentials(String),
}

impl IntoHttpResponse for RepositoryError {
    fn into_http_response(self) -> HttpResponse {
        match self {
            RepositoryError::BadRequest(msg) => ErrorHandler::bad_request(&msg),
            RepositoryError::NotFound(msg) => ErrorHandler::not_found(&msg),
            RepositoryError::Internal(msg) => ErrorHandler::internal_server_error(&msg),
            RepositoryError::InvalidCredentials(msg) => ErrorHandler::unauthorized(&msg),
        }
    }
}
