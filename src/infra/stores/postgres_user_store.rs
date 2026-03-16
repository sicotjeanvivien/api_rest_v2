use crate::domain::{
    error::repository_error::RepositoryError,
    user::{
        model::{User, UserAuth, UserRegister},
        respository::UserRepository,
    },
};
use async_trait::async_trait;
use sqlx::PgPool;
use tracing::info;

pub struct PostgresUserStore {
    pg_pool: PgPool,
}

impl PostgresUserStore {
    pub fn from_pool(pg_pool: PgPool) -> Self {
        Self { pg_pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserStore {
    async fn get_by_username(&self, username: &str) -> Result<User, RepositoryError> {
        let row = sqlx::query!("SELECT * FROM users WHERE username = $1 ;", username)
            .fetch_one(&self.pg_pool)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => RepositoryError::NotFound(e.to_string()),
                _ => RepositoryError::Internal(e.to_string()),
            })?;
        info!(user_username = username, "Get task");
        Ok(User::new(row.id, row.username, row.hash, row.created_at))
    }
    
    async fn create(&self, user_register: UserRegister)-> Result<(), RepositoryError>{
      todo!()
    }
}
