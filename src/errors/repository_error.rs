#[derive(Debug, thiserror::Error, PartialEq)]
pub enum RepositoryError {
    #[error("Data Not Found")]
    NotFound,
    #[error("Internal server Error")]
    Internal,
}
