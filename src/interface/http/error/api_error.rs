use serde::Serialize;

#[derive(Serialize)]
pub struct ApiError {
    pub code: u16,
    pub message: String,
}
