use crate::interface::http::{
    handlers::error_handler::ErrorHandler, response::into_http_response::IntoHttpResponse,
};

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum SecurityError {
    #[error("Invalid Credential: {0}")]
    InvalidCredential(String),
    #[error("Token CreationFailed: {0}")]
    TokenCreationFailed(String),
    #[error("Invalid Token: {0}")]
    InvalidToken(String),
    #[error("Missing Jwt Secret: {0}")]
    MissingJwtSecret(String),
    #[error("TokenExpired: {0}")]
    TokenExpired(String),
}

impl IntoHttpResponse for SecurityError {
    fn into_http_response(self) -> crate::interface::http::response::http_response::HttpResponse {
        match self {
            SecurityError::InvalidCredential(msg) => ErrorHandler::bad_request(&msg),
            SecurityError::TokenCreationFailed(msg) => ErrorHandler::bad_request(&msg),
            SecurityError::InvalidToken(msg) => ErrorHandler::bad_request(&msg),
            SecurityError::MissingJwtSecret(msg) => ErrorHandler::bad_request(&msg),
            SecurityError::TokenExpired(msg) => ErrorHandler::bad_request(&msg),
        }
    }
}
