use crate::domain::{NewUser, RepositoryError, User, UserRepository};
use std::sync::Arc;

pub(crate)  struct UserService {
    repository: Arc<dyn UserRepository + Send + Sync>,
}

impl UserService {
    pub(crate)  async fn new(repository: Arc<dyn UserRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub(crate)  async fn get_by_username(&self, username: &str) -> Result<User, RepositoryError> {
        self.repository.get_by_username(username).await
    }
    pub(crate)  async fn create(&self, new_user: NewUser) -> Result<(), RepositoryError> {
        self.repository.create(new_user).await
    }
}
