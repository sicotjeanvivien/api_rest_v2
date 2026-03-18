use crate::domain::{
    error::repository_error::RepositoryError,
    user::{
        model::{NewUser, User},
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
                sqlx::Error::Database(db_error) => {
                    RepositoryError::BadRequest(db_error.message().to_string())
                }
                _ => RepositoryError::Internal(e.to_string()),
            })?;
        info!(user_username = username, "Get task");
        Ok(User::new(row.id, row.username, row.hash, row.created_at))
    }

    async fn create(&self, new_user: NewUser) -> Result<(), RepositoryError> {
        sqlx::query!(
            "INSERT INTO users (username, hash) VALUES ($1, $2) ;",
            new_user.username,
            new_user.hash
        )
        .execute(&self.pg_pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db_error) => {
                RepositoryError::BadRequest(db_error.message().to_string())
            }
            _ => RepositoryError::Internal(e.to_string()),
        })?;
        info!("New user creating.");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;

    #[sqlx::test(migrations = "./migrations")]
    async fn test_create_and_get_by_user(pg_pool: PgPool) {
        let store = PostgresUserStore::from_pool(pg_pool);
        store.create(NewUser { username: "john_doe".to_string(), hash: "$argon2id$v=19$m=19456,t=2,p=1$dBPCpdhnbQNatTIOs9ilcA$USnB2zX124wzh0wSAOqIvpHIV9TWOmuK15OMnBrYYLc".to_string() } ).await.unwrap();
        let user = store.get_by_username("john_doe").await.unwrap();
        assert_eq!(user.username(), "john_doe");
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_get_by_username(pg_pool: PgPool) {
        let store = PostgresUserStore::from_pool(pg_pool);

        store
            .create(NewUser {
                username: "john_doe".to_string(),
                hash: "hash123".to_string(),
            })
            .await
            .unwrap();

        let user = store.get_by_username("john_doe").await.unwrap();
        assert_eq!(user.username(), "john_doe");

        let err = store.get_by_username("unknown").await.unwrap_err();
        assert!(matches!(err, RepositoryError::Internal(_)));
    }
}
