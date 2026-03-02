use std::collections::HashMap;

use crate::{
    domain::task::service::TaskService,
    infra::http::{
        dto::response::task_response::TaskResponse,
        handlers::error_handler::ErrorHandler,
        request::Request,
        response::{Response, StatusCode},
    },
};

pub struct TaskHandler {
    request: Request,
    task_service: TaskService,
}

impl TaskHandler {
    pub fn get_all_task(&self) -> Response {
        let tasks = match self.task_service.get_all() {
            Ok(t) => t,
            Err(_) => return ErrorHandler::internal_server_error(),
        };

        let response_tasks: Vec<TaskResponse> = tasks
            .iter()
            .map(|t| TaskResponse::from(t.clone()))
            .collect();
        let body = match serde_json::to_string(&response_tasks) {
            Ok(s) => s,
            Err(_) => return ErrorHandler::internal_server_error(),
        };

        Response::new(StatusCode::OK, HashMap::new(), Some(body))
    }
}
