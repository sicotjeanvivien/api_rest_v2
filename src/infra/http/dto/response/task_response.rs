use crate::domain::task::model::Task;
use serde::Serialize;

#[derive(Serialize)]
struct TaskResponse {
    id: u32,
    title: String,
    description: Option<String>,
    done: bool,
}

impl From<Task> for TaskResponse {
    fn from(task: Task) -> TaskResponse {
        TaskResponse {
            id: task.id(),
            title: task.title().to_string(),
            description: task.description().map(|s| s.to_string()),
            done: task.done(),
        }
    }
}
