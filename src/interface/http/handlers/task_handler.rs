use std::{collections::HashMap, sync::Arc};

use crate::{
    application::TaskService,
    domain::{NewTask, Task, UpdateTask},
    interface::{HttpRequest, HttpResponse, StatusCode, TaskResponse},
};

#[derive(Clone)]
pub(crate) struct TaskHandler {
    task_service: Arc<TaskService>,
}

impl TaskHandler {
    pub(crate) fn new(task_service: Arc<TaskService>) -> Self {
        Self { task_service }
    }

    pub(crate) async fn get_task(
        &self,
        _request: HttpRequest,
    ) -> Result<HttpResponse, HttpResponse> {
        let id = parse_id(&_request)?;

        let task = self
            .task_service
            .get(id)
            .await
            .map_err(HttpResponse::from)?;

        let task_response = TaskResponse::from(task);
        let body = serde_json::to_string(&task_response).map_err(HttpResponse::from)?;

        Ok(HttpResponse::new(
            StatusCode::OK,
            Self::build_header(),
            Some(body),
        ))
    }

    pub(crate) async fn get_all_task(
        &self,
        _request: HttpRequest,
    ) -> Result<HttpResponse, HttpResponse> {
        let tasks = self
            .task_service
            .get_all()
            .await
            .map_err(HttpResponse::from)?;

        let response_tasks: Vec<TaskResponse> = tasks.into_iter().map(TaskResponse::from).collect();

        let body = serde_json::to_string(&response_tasks).map_err(HttpResponse::from)?;

        Ok(HttpResponse::new(
            StatusCode::OK,
            Self::build_header(),
            Some(body),
        ))
    }

    pub(crate) async fn create_task(
        &self,
        _request: HttpRequest,
    ) -> Result<HttpResponse, HttpResponse> {
        let new_task: NewTask =
            serde_json::from_str(&_request.get_body()?).map_err(HttpResponse::from)?;

        self.task_service
            .create(new_task)
            .await
            .map_err(HttpResponse::from)?;

        Ok(HttpResponse::new(
            StatusCode::Created,
            Self::build_header(),
            None,
        ))
    }

    pub(crate) async fn update_task(
        &self,
        _request: HttpRequest,
    ) -> Result<HttpResponse, HttpResponse> {
        let update_task: UpdateTask =
            serde_json::from_str(&_request.get_body()?).map_err(HttpResponse::from)?;

        self.task_service
            .update(update_task)
            .await
            .map_err(HttpResponse::from)?;
        Ok(HttpResponse::new(
            StatusCode::Accepted,
            Self::build_header(),
            None,
        ))
    }

    pub(crate) async fn delete_task(
        &self,
        _request: HttpRequest,
    ) -> Result<HttpResponse, HttpResponse> {
        let id = parse_id(&_request)?;
        self.task_service
            .delete(id)
            .await
            .map_err(HttpResponse::from)?;

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
        .map_err(HttpResponse::from)?
        .parse()
        .map_err(HttpResponse::from)
}

#[allow(unused)]
fn parse_by_key(key: &str, request: &HttpRequest) -> Result<usize, HttpResponse> {
    request
        .get_value_by_key(key.to_string())
        .map_err(HttpResponse::from)?
        .parse()
        .map_err(HttpResponse::from)
}

#[allow(unused)]
async fn sum_ids(task_service: TaskService, _request: HttpRequest) -> Result<i32, HttpResponse> {
    let sum_task = task_service
        .get_all()
        .await
        .map_err(HttpResponse::from)?
        .into_iter()
        .filter(|t| !t.done())
        .fold(0, |acc, x| acc + x.id());

    Ok(sum_task)
}

#[allow(unused)]
fn parse_ids(ids: Vec<&str>) -> Result<Vec<i32>, HttpResponse> {
    ids.iter()
        .map(|id| id.parse::<i32>().map_err(HttpResponse::from))
        .collect()
}

#[allow(unused)]
fn filter_task_not_done(tasks: Vec<Task>) -> Vec<String> {
    tasks
        .into_iter()
        .filter_map(|t| t.description().map(|t| t.to_string()))
        .collect()
}
