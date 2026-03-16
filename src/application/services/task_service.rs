use std::sync::Arc;

use crate::domain::{error::repository_error::RepositoryError, task::{
        model::{NewTask, Task, UpdateTask},
        repository::TaskRepository,
    }};

pub struct TaskService {
    repository: Arc<dyn TaskRepository + Send + Sync>,
}

impl TaskService {
    pub async fn new(repository: Arc<dyn TaskRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub async fn get(&self, id: i32) -> Result<Task, RepositoryError> {
        self.repository.get(id).await
    }

    pub async fn get_all(&self) -> Result<Vec<Task>, RepositoryError> {
        self.repository.get_all().await
    }

    pub async fn create(&self, new_task: NewTask) -> Result<(), RepositoryError> {
        self.repository.create(new_task).await
    }

    pub async fn update(&self, update_task: UpdateTask) -> Result<(), RepositoryError> {
        self.repository.update(update_task).await
    }

    pub async fn delete(&self, id: i32) -> Result<(), RepositoryError> {
        self.repository.delete(id).await
    }
}
