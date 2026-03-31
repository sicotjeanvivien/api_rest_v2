use async_trait::async_trait;

use crate::domain::{NewUser, RepositoryError, User};

#[async_trait]
pub(crate) trait UserRepository {
    async fn get_by_username(&self, username: &str) -> Result<User, RepositoryError>;
    async fn create(&self, user_register: NewUser) -> Result<(), RepositoryError>;
}
