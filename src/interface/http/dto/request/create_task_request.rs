use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateTaskRequest {
    title: String,
    description: Option<String>,
}
