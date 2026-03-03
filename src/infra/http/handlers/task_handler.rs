use std::collections::HashMap;

use crate::{
    domain::task::{repository::TaskRepository, service::TaskService},
    infra::http::{
        dto::response::task_response::TaskResponse,
        handlers::error_handler::ErrorHandler,
        request::HttpRequest,
        response::{HttpResponse, StatusCode},
    },
};

pub struct TaskHandler {
    task_service: TaskService,
}

impl TaskHandler {
    pub fn new(task_service: TaskService) -> Self {
        Self { task_service }
    }

    pub fn get_all_task(&self, _request: HttpRequest) -> HttpResponse {
        let tasks = match self.task_service.get_all() {
            Ok(t) => t,
            Err(_) => return ErrorHandler::internal_server_error(),
        };

        let response_tasks: Vec<TaskResponse> = tasks.into_iter().map(TaskResponse::from).collect();

        let body = match serde_json::to_string(&response_tasks) {
            Ok(s) => s,
            Err(_) => return ErrorHandler::internal_server_error(),
        };

        HttpResponse::new(StatusCode::OK, HashMap::new(), Some(body))
    }

    pub fn create_task(&self, _request: HttpRequest)-> HttpResponse {
        HttpResponse::new(StatusCode::Created, HashMap::new(), None)
    }
}
