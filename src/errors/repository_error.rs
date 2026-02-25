#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("Data Not Found")]
    NotFound,
    #[error("Internal server Error")]
    Internal,
}
