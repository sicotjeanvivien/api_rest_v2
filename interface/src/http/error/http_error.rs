#[derive(Debug, thiserror::Error, PartialEq)]
#[allow(dead_code)]
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
