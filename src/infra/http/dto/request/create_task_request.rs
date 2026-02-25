use serde::Deserialize;

#[derive(Deserialize)]
struct CreateTaskRequest {
    title: String,
    description: Option<String>,
}
