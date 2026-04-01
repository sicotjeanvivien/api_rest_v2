use crate::{Claims, SecurityError};
use chrono::Utc;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};

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
        let token = jsonwebtoken::encode(&header, &claims, &self.encoding_key)?;
        Ok(token)
    }

    pub fn verify(&self, token: &str) -> Result<Claims, SecurityError> {
        let validation = Validation::new(Algorithm::HS256);
        let token_data = jsonwebtoken::decode(token, &self.decoding_key, &validation)?;
        Ok(token_data.claims)
    }
}
