use std::collections::HashMap;

use crate::{
    application::services::task_service::TaskService,
    domain::task::model::{NewTask, UpdateTask},
    interface::http::{
        dto::response::task_response::TaskResponse,
        error::HttpError,
        request::HttpRequest,
        response::{http_response::HttpResponse, into_http_response::IntoHttpResponse, status_code::StatusCode},
    },
};

pub struct TaskHandler {
    task_service: TaskService,
}

impl TaskHandler {
    pub fn new(task_service: TaskService) -> Self {
        Self { task_service }
    }

    pub async fn get_task(&self, _request: HttpRequest) -> Result<HttpResponse, HttpResponse> {
        let id = parse_id(&_request)?;

        let task = self
            .task_service
            .get(id)
            .await
            .map_err(|e| e.into_http_response())?;

        let task_response = TaskResponse::from(task);
        let body = serde_json::to_string(&task_response)
            .map_err(|e| HttpError::InternalServerError(e.to_string()).into_http_response())?;

        Ok(HttpResponse::new(
            StatusCode::OK,
            Self::build_header(),
            Some(body),
        ))
    }

    pub async fn get_all_task(&self, _request: HttpRequest) -> Result<HttpResponse, HttpResponse> {
        let tasks = self
            .task_service
            .get_all()
            .await
            .map_err(|e| e.into_http_response())?;

        let response_tasks: Vec<TaskResponse> = tasks.into_iter().map(TaskResponse::from).collect();

        let body = serde_json::to_string(&response_tasks)
            .map_err(|e| HttpError::InternalServerError(e.to_string()).into_http_response())?;

        Ok(HttpResponse::new(
            StatusCode::OK,
            Self::build_header(),
            Some(body),
        ))
    }

    pub async fn create_task(&self, _request: HttpRequest) -> Result<HttpResponse, HttpResponse> {
        let body = _request.body.ok_or_else(|| {
            HttpError::BadRequest("body is not found".to_string()).into_http_response()
        })?;

        let new_task: NewTask = serde_json::from_str(&body)
            .map_err(|e| HttpError::BadRequest(e.to_string()).into_http_response())?;

        self.task_service
            .create(new_task)
            .await
            .map_err(|e| e.into_http_response())?;

        Ok(HttpResponse::new(
            StatusCode::Created,
            Self::build_header(),
            None,
        ))
    }

    pub async fn update_task(&self, _request: HttpRequest) -> Result<HttpResponse, HttpResponse> {
        let body = _request.body.ok_or_else(|| {
            HttpError::BadRequest("body is not found".to_string()).into_http_response()
        })?;
        let update_task: UpdateTask = serde_json::from_str(&body)
            .map_err(|e| HttpError::BadRequest(e.to_string()).into_http_response())?;

        self.task_service
            .update(update_task)
            .await
            .map_err(|e| e.into_http_response())?;
        Ok(HttpResponse::new(
            StatusCode::Accepted,
            Self::build_header(),
            None,
        ))
    }

    pub async fn delete_task(&self, _request: HttpRequest) -> Result<HttpResponse, HttpResponse> {
        let id = parse_id(&_request)?;
        self.task_service
            .delete(id)
            .await
            .map_err(|e| e.into_http_response())?;

        Ok(HttpResponse::new(
            StatusCode::Accepted,
            Self::build_header(),
            None,
        ))
    }

    fn build_header() -> HashMap<String, String> {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("X-Content-Type-Options".to_string(), "nosniff".to_string());
        headers.insert("X-Frame-Options".to_string(), "DENY".to_string());
        headers.insert("Access-Control-Allow-Origin".to_string(), "*".to_string());
        headers.insert(
            "Access-Control-Allow-Methods".to_string(),
            "GET, POST, PATCH, DELETE".to_string(),
        );
        headers.insert(
            "Access-Control-Allow-Headers".to_string(),
            "Content-Type".to_string(),
        );
        headers.insert("Cache-Control".to_string(), "no-store".to_string());
        headers
    }
}

fn parse_id(request: &HttpRequest) -> Result<i32, HttpResponse> {
    request
        .get_value_by_key("id".to_string())
        .map_err(|e| e.into_http_response())?
        .parse()
        .map_err(|e: std::num::ParseIntError| {
            HttpError::BadRequest(e.to_string()).into_http_response()
        })
}
