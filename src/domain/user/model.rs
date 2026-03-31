use serde::Deserialize;
use sqlx::types::chrono;

#[derive(Debug)]
pub(crate) struct User {
    id: i32,
    username: String,
    hash: String,
    created_at: chrono::NaiveDateTime,
}

impl User {
    pub(crate) fn new(id: i32, username: String, hash: String, created_at: chrono::NaiveDateTime) -> Self {
        Self {
            id,
            username,
            hash,
            created_at,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn id(&self) -> i32 {
        self.id
    }

    pub(crate) fn username(&self) -> &str {
        &self.username
    }
    pub(crate) fn hash(&self) -> &str {
        &self.hash
    }

    #[allow(dead_code)]
    pub(crate) fn created_at(&self) -> chrono::NaiveDateTime {
        self.created_at
    }
}

pub(crate) struct NewUser {
    pub(crate) username: String,
    pub(crate) hash: String,
}

#[derive(Deserialize)]
pub(crate) struct UserAuth {
    pub(crate) username: String,
    pub(crate) password: String,
}

#[derive(Deserialize)]
pub(crate) struct UserRegister {
    pub(crate) username: String,
    pub(crate) password: String,
}
