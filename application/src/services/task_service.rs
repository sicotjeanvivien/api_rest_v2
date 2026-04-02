use std::sync::Arc;

use domain::{NewTask, RepositoryError, Task, TaskRepository, UpdateTask};

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

#[cfg(test)]
mod tests {
    use super::*;
    use domain::task::repository::MockTaskRepository;

    #[tokio::test]
    async fn test_get() {
        let mut mock = MockTaskRepository::new();

        let task = Task::new(1, "title".to_string(), None, false);
        mock.expect_get()
            .times(1)
            .returning(move |_| Ok(task.clone()));

        let service = TaskService::new(Arc::new(mock)).await;
        let result = service.get(1).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().id(), 1);
    }

    #[tokio::test]
    async fn test_get_all() {
        let mut mock = MockTaskRepository::new();

        // Définir le comportement attendu
        mock.expect_get_all()
            .times(1) // doit être appelé exactement une fois
            .returning(|| Ok(vec![]));

        let service = TaskService::new(Arc::new(mock)).await;
        let result = service.get_all().await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[tokio::test]
    async fn test_create() {
        let mut mock = MockTaskRepository::new();
        mock.expect_create().times(1).returning(|_| Ok(()));

        let new_task = NewTask {
            title: "title".to_string(),
            description: None,
        };
        let service = TaskService::new(Arc::new(mock)).await;
        let result = service.create(new_task).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update() {
        let mut mock = MockTaskRepository::new();
        mock.expect_update().times(1).returning(|_| Ok(()));

        let update_task = UpdateTask {
            id: 1,
            title: Some("title".to_string()),
            description: None,
            done: None,
        };
        let service = TaskService::new(Arc::new(mock)).await;
        let result = service.update(update_task).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete() {
        let mut mock = MockTaskRepository::new();
        mock.expect_delete().times(1).returning(|_| Ok(()));

        let service = TaskService::new(Arc::new(mock)).await;
        let result = service.delete(1).await;

        assert!(result.is_ok());
    }
}
