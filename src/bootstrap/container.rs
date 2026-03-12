use sqlx::PgPool;

use crate::{
    application::services::task_service::TaskService,
    infra::stores::postgres_task_store::PostgresTaskStore,
};
use std::{env, sync::Arc};

pub struct Container {
    pub task_service: Arc<TaskService>,
}
impl Container {
    const DATABASE_URL: &str = "postgres://app:azerty@127.0.0.1:5432/api_rest";

    pub async fn build() -> Self {
        let pg_pool: sqlx::Pool<sqlx::Postgres> = Self::init_db().await;
        let repository: Arc<PostgresTaskStore> = Arc::new(PostgresTaskStore::from_pool(pg_pool));
        let service: Arc<TaskService> = Arc::new(TaskService::new(repository).await);

        Self {
            task_service: service,
        }
    }

    async fn init_db() -> PgPool {
        let database_url = env::var("DATABASE_URL").unwrap();

        let pool = PgPool::connect(&database_url).await.unwrap();
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();
        tracing::info!("Migrations applied");
        pool
    }
}
