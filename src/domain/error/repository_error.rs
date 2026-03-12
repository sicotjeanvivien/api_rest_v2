use crate::interface::http::{
    handlers::error_handler::ErrorHandler,
    response::{http_response::HttpResponse, into_http_response::IntoHttpResponse},
};

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum RepositoryError {
    #[error("Data Not Found: {0}")]
    NotFound(String),
    #[error("Internal server Error: {0}")]
    Internal(String),
}

impl IntoHttpResponse for RepositoryError {
    fn into_http_response(self) -> HttpResponse {
        match self {
            RepositoryError::NotFound(msg) => ErrorHandler::not_found(&msg),
            RepositoryError::Internal(msg) => ErrorHandler::internal_server_error(&msg)
        }
    }
}
