use crate::domain::{
    error::repository_error::RepositoryError,
    user::model::{NewUser, User},
};
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository {
    async fn get_by_username(&self, username: &str) -> Result<User, RepositoryError>;
    async fn create(&self, user_register: NewUser) -> Result<(), RepositoryError>;
}
