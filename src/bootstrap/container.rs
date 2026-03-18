use sqlx::PgPool;

use crate::{
    application::{
        security::credential_hasher::{self, CredentialHasher},
        services::{
            credential_service::{self, CredentialService},
            task_service::{self, TaskService},
            user_service::{self, UserService},
        },
    },
    infra::stores::{
        postgres_task_store::PostgresTaskStore, postgres_user_store::PostgresUserStore,
    },
};
use std::{env, sync::Arc};

pub struct Container {
    pub task_service: Arc<TaskService>,
    pub user_service: Arc<UserService>,
    pub credential_hasher: Arc<CredentialHasher>,
    pub credential_service: Arc<CredentialService>,
}
impl Container {
    pub async fn build() -> Self {
        let pg_pool: sqlx::Pool<sqlx::Postgres> = Self::init_db().await;

        let task_service: Arc<TaskService> = Self::init_task_service(pg_pool.clone()).await;
        let user_service: Arc<UserService> = Self::init_user_service(pg_pool.clone()).await;
        let credential_hasher: Arc<CredentialHasher> = Self::init_credential_hasher().await;
        let credential_service: Arc<CredentialService> =
            Self::init_credential_service(user_service.clone(), credential_hasher.clone()).await;

        Self {
            task_service: task_service,
            user_service: user_service,
            credential_hasher: credential_hasher,
            credential_service: credential_service,
        }
    }

    async fn init_db() -> PgPool {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

        let pool = PgPool::connect(&database_url)
            .await
            .expect("Failed to connect to PgPool");
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Failed to migrat in folder ./migration");
        tracing::info!("Migrations applied");
        pool
    }

    async fn init_task_service(pg_pool: PgPool) -> Arc<TaskService> {
        let task_repository: Arc<PostgresTaskStore> =
            Arc::new(PostgresTaskStore::from_pool(pg_pool));
        Arc::new(TaskService::new(task_repository).await)
    }

    async fn init_user_service(pg_pool: PgPool) -> Arc<UserService> {
        let user_repository: Arc<PostgresUserStore> =
            Arc::new(PostgresUserStore::from_pool(pg_pool));
        Arc::new(UserService::new(user_repository).await)
    }

    async fn init_credential_hasher() -> Arc<CredentialHasher> {
        Arc::new(CredentialHasher::new())
    }

    async fn init_credential_service(
        user_service: Arc<UserService>,
        credential_hasher: Arc<CredentialHasher>,
    ) -> Arc<CredentialService> {
        Arc::new(CredentialService::new(user_service, credential_hasher).await)
    }
}
