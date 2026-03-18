use crate::{
    application::{
        security::credential_hasher::CredentialHasher, services::user_service::UserService,
    },
    domain::{
        error::repository_error::RepositoryError,
        user::model::{NewUser, User, UserAuth, UserRegister},
    },
};
use std::sync::Arc;

pub struct CredentialService {
    user_service: Arc<UserService>,
    credential_hasher: Arc<CredentialHasher>,
}

impl CredentialService {
    pub async fn new(
        user_service: Arc<UserService>,
        credential_hasher: Arc<CredentialHasher>,
    ) -> Self {
        Self {
            user_service,
            credential_hasher,
        }
    }

    pub async fn register(&self, user_register: UserRegister) -> Result<(), RepositoryError> {
        let hash = self.credential_hasher.hash(&user_register.password)?;

        let new_user: NewUser = NewUser {
            username: user_register.username,
            hassh: hash,
        };
        self.user_service.create(new_user).await
    }

    pub async fn login(&self, user_auth: UserAuth) -> Result<User, RepositoryError> {
        let user = self
            .user_service
            .get_by_username(&user_auth.username)
            .await?;

        self.credential_hasher
            .verify(user.hash(), &user_auth.password)?
            .then_some(user)
            .ok_or(RepositoryError::InvalidCredentials("".to_string()))
    }
}
