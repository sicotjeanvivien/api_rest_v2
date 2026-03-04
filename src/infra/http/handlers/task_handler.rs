use std::collections::HashMap;

use crate::{
    domain::task::{
        model::{NewTask, UpdateTask},
        repository::TaskRepository,
        service::TaskService,
    },
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

    pub fn get_task(&self, _request: HttpRequest) -> Result<HttpResponse, HttpResponse> {
        let id = parse_id(&_request)?;

        let task = self
            .task_service
            .get(id)
            .map_err(|_| ErrorHandler::not_found())?;

        let task_response = TaskResponse::from(task);
        let body = serde_json::to_string(&task_response)
            .map_err(|_| ErrorHandler::internal_server_error())?;

        Ok(HttpResponse::new(
            StatusCode::OK,
            HashMap::new(),
            Some(body),
        ))
    }

    pub fn get_all_task(&self, _request: HttpRequest) -> Result<HttpResponse, HttpResponse> {
        let tasks = self
            .task_service
            .get_all()
            .map_err(|_| ErrorHandler::internal_server_error())?;

        let response_tasks: Vec<TaskResponse> = tasks.into_iter().map(TaskResponse::from).collect();

        let body = serde_json::to_string(&response_tasks)
            .map_err(|_| ErrorHandler::internal_server_error())?;

        Ok(HttpResponse::new(
            StatusCode::OK,
            HashMap::new(),
            Some(body),
        ))
    }

    pub fn create_task(&self, _request: HttpRequest) -> Result<HttpResponse, HttpResponse> {
        let body = _request.body.ok_or_else(ErrorHandler::bad_request)?;

        let new_task: NewTask =
            serde_json::from_str(&body).map_err(|_| ErrorHandler::unprocessable_entity())?;

        self.task_service
            .create(new_task)
            .map_err(|_| ErrorHandler::unprocessable_entity())?;

        Ok(HttpResponse::new(StatusCode::Created, HashMap::new(), None))
    }

    pub fn update_task(&self, _request: HttpRequest) -> Result<HttpResponse, HttpResponse> {
        let body = _request.body.ok_or_else(ErrorHandler::bad_request)?;
        let update_task =
            serde_json::from_str(&body).map_err(|_| ErrorHandler::unprocessable_entity())?;

        self.task_service
            .update(update_task)
            .map_err(|_| return ErrorHandler::unprocessable_entity())?;
        Ok(HttpResponse::new(
            StatusCode::Accepted,
            HashMap::new(),
            None,
        ))
    }

    pub fn delete_task(&self, _request: HttpRequest) -> Result<HttpResponse, HttpResponse> {
        let id = parse_id(&_request)?;
        self.task_service
            .delete(id)
            .map_err(|_| ErrorHandler::unprocessable_entity())?;
        Ok(HttpResponse::new(
            StatusCode::Accepted,
            HashMap::new(),
            None,
        ))
    }
}

fn parse_id(request: &HttpRequest) -> Result<u32, HttpResponse> {
    request
        .get_value_by_key("id".to_string())
        .map_err(|_| ErrorHandler::internal_server_error())?
        .parse()
        .map_err(|_| ErrorHandler::bad_request())
}
