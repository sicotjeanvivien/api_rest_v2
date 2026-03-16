use crate::{
    application::{
        security::credential_hasher::{self, CredentialHasher},
        services::user_service::{self, UserService},
    },
    domain::{
        error::repository_error::RepositoryError,
        user::{
            model::{User, UserAuth, UserRegister},
            respository::UserRepository,
        },
    },
};
use std::sync::Arc;

pub struct AuthService {
    user_service: Arc<UserService>,
    credential_hasher: Arc<CredentialHasher>,
}

impl AuthService {
    pub async fn new(
        user_service: Arc<UserService>,
        credential_hasher: Arc<CredentialHasher>,
    ) -> Self {
        Self {
            user_service,
            credential_hasher,
        }
    }

    pub async fn register(&self, user_register: UserRegister)-> Result<(), RepositoryError> {
        let hash = self.credential_hasher.hash(&user_register.password)?;
        
    }

    pub async fn login(&self, user_auth: UserAuth) -> Result<User, RepositoryError> {
        let user = self
            .user_service
            .get_by_username(&user_auth.username)
            .await?;

        self.credential_hasher
            .verify(user.hash(), &user_auth.password)?
            .then_some(user)
            .ok_or(RepositoryError::InvalidCredentials)
    }
}
