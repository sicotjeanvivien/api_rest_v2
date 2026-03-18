use chrono::Utc;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};

use crate::application::security::{
    claims::{self, Claims},
    error::security_error::SecurityError,
};

pub struct JwtService {}

impl JwtService {
    pub fn generate(username: &str) -> Result<String, SecurityError> {
        let secret: String = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let claims: Claims = Claims {
            sub: username.to_string(),
            exp: (Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
        };

        let header: Header = Header::default();
        let key: &EncodingKey = &EncodingKey::from_secret(secret.as_bytes());
        jsonwebtoken::encode(&header, &claims, key)
            .map_err(|e| SecurityError::InvalidCredential(e.to_string()))
    }

    pub fn verify(token: &str) -> Result<Claims, SecurityError> {
        let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let key = &DecodingKey::from_secret(secret.as_bytes());
        let validation = &Validation::new(Algorithm::HS256);

        jsonwebtoken::decode(token, key, validation)
            .map(|token| token.claims)
            .map_err(|e| SecurityError::InvalidCredential(e.to_string()))
    }
}
