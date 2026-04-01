use async_trait::async_trait;

use crate::{NewUser, RepositoryError, User};

#[async_trait]
pub trait UserRepository {
    async fn get_by_username(&self, username: &str) -> Result<User, RepositoryError>;
    async fn create(&self, user_register: NewUser) -> Result<(), RepositoryError>;
}
