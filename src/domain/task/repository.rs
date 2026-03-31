use async_trait::async_trait;

use crate::domain::{NewTask, RepositoryError, Task, UpdateTask};

#[async_trait]
pub(crate) trait TaskRepository {
    async fn get(&self, id: i32) -> Result<Task, RepositoryError>;
    async fn get_all(&self) -> Result<Vec<Task>, RepositoryError>;
    async fn create(&self, new_task: NewTask) -> Result<(), RepositoryError>;
    async fn update(&self, update_task: UpdateTask) -> Result<(), RepositoryError>;
    async fn delete(&self, id: i32) -> Result<(), RepositoryError>;
}
