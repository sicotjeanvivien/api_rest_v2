use domain::{NewUser, RepositoryError, User, UserRepository};
use std::sync::Arc;

pub  struct UserService {
    repository: Arc<dyn UserRepository + Send + Sync>,
}

impl UserService {
    pub  async fn new(repository: Arc<dyn UserRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub  async fn get_by_username(&self, username: &str) -> Result<User, RepositoryError> {
        self.repository.get_by_username(username).await
    }
    pub  async fn create(&self, new_user: NewUser) -> Result<(), RepositoryError> {
        self.repository.create(new_user).await
    }
}
