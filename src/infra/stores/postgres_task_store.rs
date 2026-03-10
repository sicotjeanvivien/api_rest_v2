use async_trait::async_trait;
use sqlx::PgPool;

use crate::{
    domain::task::{
        model::{NewTask, Task, UpdateTask},
        repository::TaskRepository,
    },
    errors::repository_error::RepositoryError,
};

pub struct PostgresTaskStore {
    pg_pool: PgPool,
}

impl PostgresTaskStore {
    const BDD_URL: &str = "postgres://app:azerty@127.0.0.1:5432/api_rest";

    pub async fn new() -> Self {
        Self {
            pg_pool: PgPool::connect(Self::BDD_URL).await.unwrap(),
        }
    }
}
#[async_trait]
impl TaskRepository for PostgresTaskStore {
    async fn get(&self, id: i32) -> Result<Task, RepositoryError> {
        let row = sqlx::query!(
            "SELECT id, title, description, done FROM tasks WHERE id = $1",
            id
        )
        .fetch_one(&self.pg_pool)
        .await
        .map_err(|_| RepositoryError::Internal)?;

        Ok(Task::new(row.id, row.title, row.description, row.done))
    }

    async fn get_all(&self) -> Result<Vec<Task>, RepositoryError> {
        let rows = sqlx::query!("SELECT id, title, description, done FROM tasks")
            .fetch_all(&self.pg_pool)
            .await
            .map_err(|_| RepositoryError::Internal)?;

        Ok(rows
            .into_iter()
            .map(|row| Task::new(row.id, row.title, row.description, row.done))
            .collect())
    }

    async fn create(&self, new_task: NewTask) -> Result<(), RepositoryError> {
        sqlx::query!(
            "INSERT INTO tasks (title, description, done) VALUES ($1, $2, $3)",
            new_task.title,
            new_task.description,
            false
        )
        .execute(&self.pg_pool)
        .await
        .map_err(|_| RepositoryError::Internal)?;
        Ok(())
    }

    async fn update(&self, update_task: UpdateTask) -> Result<(), RepositoryError> {
        sqlx::query!(
            "UPDATE tasks
            SET title = $2,  description = $3, done = $4
            WHERE id = $1;",
            update_task.id,
            update_task.title,
            update_task.description,
            update_task.done
        )
        .execute(&self.pg_pool)
        .await
        .map_err(|_| RepositoryError::Internal)?;
        Ok(())
    }

    async fn delete(&self, id: i32) -> Result<(), RepositoryError> {
        sqlx::query!("DELETE FROM tasks WHERE id=$1", id)
            .execute(&self.pg_pool)
            .await
            .map_err(|_| RepositoryError::Internal)?;
        Ok(())
    }
}
