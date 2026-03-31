use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct ApiError {
    pub code: u16,
    pub message: String,
}
