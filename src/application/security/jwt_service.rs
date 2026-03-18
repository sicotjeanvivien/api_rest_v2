use chrono::Utc;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};

use crate::application::security::{claims::Claims, error::security_error::SecurityError};

pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl JwtService {
    pub fn new(secret: String) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
        }
    }

    pub fn generate(&self, username: &str) -> Result<String, SecurityError> {
        let claims: Claims = Claims {
            sub: username.to_string(),
            exp: (Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
        };

        let header: Header = Header::default();

        jsonwebtoken::encode(&header, &claims, &self.encoding_key)
            .map_err(|e| SecurityError::InvalidCredential(e.to_string()))
    }

    pub fn verify(&self, token: &str) -> Result<Claims, SecurityError> {
        let validation = &Validation::new(Algorithm::HS256);

        jsonwebtoken::decode(token, &self.decoding_key, validation)
            .map(|token| token.claims)
            .map_err(|e| SecurityError::InvalidCredential(e.to_string()))
    }
}
