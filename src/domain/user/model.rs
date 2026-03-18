use serde::Deserialize;
use sqlx::types::chrono;

#[derive(Debug)]
pub struct User {
    id: i32,
    username: String,
    hash: String,
    created_at: chrono::NaiveDateTime,
}

impl User {
    pub fn new(id: i32, username: String, hash: String, created_at: chrono::NaiveDateTime) -> Self {
        Self {
            id,
            username,
            hash,
            created_at,
        }
    }

    #[allow(dead_code)]
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn hash(&self) -> &str {
        &self.hash
    }

    #[allow(dead_code)]
    pub fn created_at(&self) -> chrono::NaiveDateTime {
        self.created_at
    }
}

pub struct NewUser {
    pub username: String,
    pub hash: String,
}

#[derive(Deserialize)]
pub struct UserAuth {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UserRegister {
    pub username: String,
    pub password: String,
}
