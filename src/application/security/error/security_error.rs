#[derive(Debug, thiserror::Error, PartialEq)]
#[allow(dead_code)]
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