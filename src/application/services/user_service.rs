use crate::domain::{
    error::repository_error::RepositoryError,
    user::{
        model::{User, UserAuth, UserRegister},
        respository::UserRepository,
    },
};
use core::sync;
use std::sync::Arc;

pub struct UserService {
    repository: Arc<dyn UserRepository + Send + Sync>,
}

impl UserService {
    pub async fn new(repository: Arc<dyn UserRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub async fn get_by_username(&self, username: &str) -> Result<User, RepositoryError> {
        self.repository.get_by_username(username).await
    }
    pub async fn create(&self, user_register: UserRegister)->Result<(), RepositoryError> {
        self.repository.create(user_register).await
    }
}
