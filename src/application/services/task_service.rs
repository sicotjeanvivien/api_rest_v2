use std::sync::Arc;

use crate::domain::{NewTask, RepositoryError, Task, TaskRepository, UpdateTask};

pub(crate)  struct TaskService {
    repository: Arc<dyn TaskRepository + Send + Sync>,
}

impl TaskService {
    pub(crate)  async fn new(repository: Arc<dyn TaskRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub(crate)  async fn get(&self, id: i32) -> Result<Task, RepositoryError> {
        self.repository.get(id).await
    }

    pub(crate)  async fn get_all(&self) -> Result<Vec<Task>, RepositoryError> {
        self.repository.get_all().await
    }

    pub(crate)  async fn create(&self, new_task: NewTask) -> Result<(), RepositoryError> {
        self.repository.create(new_task).await
    }

    pub(crate)  async fn update(&self, update_task: UpdateTask) -> Result<(), RepositoryError> {
        self.repository.update(update_task).await
    }

    pub(crate)  async fn delete(&self, id: i32) -> Result<(), RepositoryError> {
        self.repository.delete(id).await
    }
}
