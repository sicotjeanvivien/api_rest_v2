use async_trait::async_trait;
use sqlx::PgPool;
use tracing::info;

use domain::{NewTask, RepositoryError, Task, TaskRepository, UpdateTask};

pub  struct PostgresTaskStore {
    pg_pool: PgPool,
}

impl PostgresTaskStore {
    pub  fn from_pool(pg_pool: PgPool) -> Self {
        Self { pg_pool }
    }
}
#[async_trait]
impl TaskRepository for PostgresTaskStore {
    async fn get(&self, id: i32) -> Result<Task, RepositoryError> {
        let row = sqlx::query!(
            "SELECT id, title, description, done FROM tasks WHERE id = $1 ;",
            id
        )
        .fetch_one(&self.pg_pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => RepositoryError::NotFound(e.to_string()),
            _ => RepositoryError::Internal(e.to_string()),
        })?;
        info!(task_id = id, "Get task");
        Ok(Task::new(row.id, row.title, row.description, row.done))
    }

    async fn get_all(&self) -> Result<Vec<Task>, RepositoryError> {
        let rows = sqlx::query!("SELECT id, title, description, done FROM tasks ;")
            .fetch_all(&self.pg_pool)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => RepositoryError::NotFound(e.to_string()),
                _ => RepositoryError::Internal(e.to_string()),
            })?;
        info!("Get all tasks");
        Ok(rows
            .into_iter()
            .map(|row| Task::new(row.id, row.title, row.description, row.done))
            .collect())
    }

    async fn create(&self, new_task: NewTask) -> Result<(), RepositoryError> {
        sqlx::query!(
            "INSERT INTO tasks (title, description, done) VALUES ($1, $2, $3) ;",
            new_task.title,
            new_task.description,
            false
        )
        .execute(&self.pg_pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db_error) => {
                RepositoryError::BadRequest(db_error.message().to_string())
            }
            _ => RepositoryError::Internal(e.to_string()),
        })?;
        info!("New task creating");
        Ok(())
    }

    async fn update(&self, update_task: UpdateTask) -> Result<(), RepositoryError> {
        sqlx::query!(
            "UPDATE tasks
            SET 
                title = COALESCE($2, title),
                description = COALESCE($3, description),
                done = COALESCE($4, done)
            WHERE id = $1;",
            update_task.id,
            update_task.title,
            update_task.description,
            update_task.done
        )
        .execute(&self.pg_pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => RepositoryError::NotFound(e.to_string()),
            _ => RepositoryError::Internal(e.to_string()),
        })?;
        info!(task_id = update_task.id, "Updating task");
        Ok(())
    }

    async fn delete(&self, id: i32) -> Result<(), RepositoryError> {
        sqlx::query!("DELETE FROM tasks WHERE id=$1 ;", id)
            .execute(&self.pg_pool)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => RepositoryError::NotFound(e.to_string()),
                _ => RepositoryError::Internal(e.to_string()),
            })?;
        info!(task_id = id, "Deleting task");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::NewTask;
    use sqlx::PgPool;

    #[sqlx::test(migrations = "../migrations")]
    async fn test_create(pool: PgPool) {
        let store = PostgresTaskStore::from_pool(pool);

        store
            .create(NewTask {
                title: String::from("test task"),
                description: Some(String::from("description")),
            })
            .await
            .unwrap();

        let tasks = store.get_all().await.unwrap();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].title(), "test task");
        assert_eq!(tasks[0].description().unwrap(), "description");
    }

    #[sqlx::test(migrations = "../migrations")]
    async fn test_get_all(pool: PgPool) {
        let store = PostgresTaskStore::from_pool(pool);

        let tasks = store.get_all().await.unwrap();
        assert_eq!(tasks.len(), 0);

        store
            .create(NewTask {
                title: String::from("task 1"),
                description: Some(String::from("desc 1")),
            })
            .await
            .unwrap();
        store
            .create(NewTask {
                title: String::from("task 2"),
                description: None,
            })
            .await
            .unwrap();

        let tasks = store.get_all().await.unwrap();
        assert_eq!(tasks.len(), 2);
        assert_eq!(tasks[0].title(), "task 1");
        assert_eq!(tasks[1].title(), "task 2");
    }

    #[sqlx::test(migrations = "../migrations")]
    async fn test_get(pool: PgPool) {
        let store = PostgresTaskStore::from_pool(pool);

        store
            .create(NewTask {
                title: String::from("task 1"),
                description: None,
            })
            .await
            .unwrap();

        let id = store.get_all().await.unwrap()[0].id();

        let task = store.get(id).await.unwrap();
        assert_eq!(task.title(), "task 1");

        let err = store.get(99999).await.unwrap_err();
        assert!(matches!(err, RepositoryError::NotFound(_)));
    }

    #[sqlx::test(migrations = "../migrations")]
    async fn test_update(pool: PgPool) {
        let store = PostgresTaskStore::from_pool(pool);

        store
            .create(NewTask {
                title: String::from("original"),
                description: None,
            })
            .await
            .unwrap();

        let id = store.get_all().await.unwrap()[0].id();

        store
            .update(UpdateTask {
                id,
                title: Some(String::from("updated")),
                description: None,
                done: Some(true),
            })
            .await
            .unwrap();

        let task = store.get(id).await.unwrap();
        assert_eq!(task.title(), "updated");
        assert_eq!(task.done(), true);
    }

    #[sqlx::test(migrations = "../migrations")]
    async fn test_delete(pool: PgPool) {
        let store = PostgresTaskStore::from_pool(pool);

        store
            .create(NewTask {
                title: String::from("to delete"),
                description: None,
            })
            .await
            .unwrap();

        let id = store.get_all().await.unwrap()[0].id();

        store.delete(id).await.unwrap();

        let err = store.get(id).await.unwrap_err();
        assert!(matches!(err, RepositoryError::NotFound(_)));
    }
}
