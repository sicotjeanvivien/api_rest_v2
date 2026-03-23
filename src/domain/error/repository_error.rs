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