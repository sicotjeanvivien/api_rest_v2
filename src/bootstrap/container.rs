use sqlx::PgPool;

use crate::{
    application::{CredentialHasher, CredentialService, JwtService, TaskService, UserService},
    infra::{PostgresTaskStore, PostgresUserStore},
};
use std::{env, sync::Arc};

#[allow(dead_code)]
pub(crate)  struct Container {
    pub(crate)  task_service: Arc<TaskService>,
    pub(crate)  user_service: Arc<UserService>,
    pub(crate)  credential_hasher: Arc<CredentialHasher>,
    pub(crate)  credential_service: Arc<CredentialService>,
    pub(crate)  jwt_service: Arc<JwtService>,
}
impl Container {
    pub(crate)  async fn build() -> Self {
        let pg_pool: sqlx::Pool<sqlx::Postgres> = Self::init_db().await;

        let task_service: Arc<TaskService> = Self::init_task_service(pg_pool.clone()).await;
        let user_service: Arc<UserService> = Self::init_user_service(pg_pool.clone()).await;
        let credential_hasher: Arc<CredentialHasher> = Self::init_credential_hasher().await;
        let credential_service: Arc<CredentialService> =
            Self::init_credential_service(user_service.clone(), credential_hasher.clone()).await;
        let jwt_service: Arc<JwtService> = Self::init_jwt_service();

        Self {
            task_service: task_service,
            user_service: user_service,
            credential_hasher: credential_hasher,
            credential_service: credential_service,
            jwt_service: jwt_service,
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

    fn init_jwt_service() -> Arc<JwtService> {
        let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set in .env");
        Arc::new(JwtService::new(secret))
    }
}
