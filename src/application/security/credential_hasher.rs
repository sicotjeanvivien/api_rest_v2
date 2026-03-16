use crate::domain::error::repository_error::RepositoryError;
use argon2::{
    Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version,
    password_hash::{
        SaltString,
        rand_core::{Error, OsRng},
    },
};

pub struct CredentialHasher {
    argon2: Argon2<'static>,
}

impl CredentialHasher {
    pub fn new() -> Self {
        Self {
            argon2: Argon2::new(
                Algorithm::Argon2id,
                Version::V0x13,
                Params::new(19456, 2, 1, None).expect("Argon2 params errors"),
            ),
        }
    }

    pub fn hash(&self, password: &str) -> Result<String, RepositoryError> {
        let salt = SaltString::generate(&mut OsRng);

        self.argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|h| h.to_string())
            .map_err(|e| RepositoryError::Internal(e.to_string()))
    }

    pub fn verify(&self, hash: &str, password: &str) -> Result<bool, RepositoryError> {
        let parsed_hash =
            PasswordHash::new(hash).map_err(|e| RepositoryError::Internal(e.to_string()))?;
        self.argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .map(|_| true)
            .map_err(|e| RepositoryError::Internal(e.to_string()))
    }
}
