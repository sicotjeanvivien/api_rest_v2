use std::{collections::HashMap, sync::Arc};

use crate::{
    application::{CredentialService, JwtService},
    domain::{User, UserAuth, UserRegister},
    interface::{HttpRequest, HttpResponse, StatusCode},
};

#[derive(Clone)]
pub(crate) struct AuthHandler {
    credential_service: Arc<CredentialService>,
    jwt_service: Arc<JwtService>,
}

impl AuthHandler {
    pub(crate) fn new(
        credential_service: Arc<CredentialService>,
        jwt_service: Arc<JwtService>,
    ) -> Self {
        Self {
            credential_service,
            jwt_service,
        }
    }

    pub(crate) async fn login(&self, _request: HttpRequest) -> Result<HttpResponse, HttpResponse> {
        println!("login");
        let user_auth: UserAuth =
            serde_json::from_str(&_request.get_body()?).map_err(HttpResponse::from)?;
        let user: User = self
            .credential_service
            .login(user_auth)
            .await
            .map_err(HttpResponse::from)?;

        let token = self
            .jwt_service
            .generate(user.username())
            .map_err(HttpResponse::from)?;
        let body = format!("{{\"token\": \"{token}\"}}", token = token);

        Ok(HttpResponse::new(
            StatusCode::OK,
            Self::build_header(),
            Some(body),
        ))
    }

    pub(crate) async fn register(
        &self,
        _request: HttpRequest,
    ) -> Result<HttpResponse, HttpResponse> {
        let user_register: UserRegister =
            serde_json::from_str(&_request.get_body()?).map_err(HttpResponse::from)?;

        self.credential_service
            .register(user_register)
            .await
            .map_err(HttpResponse::from)?;

        Ok(HttpResponse::new(
            StatusCode::Created,
            Self::build_header(),
            None,
        ))
    }

    fn build_header() -> HashMap<String, String> {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("X-Content-Type-Options".to_string(), "nosniff".to_string());
        headers.insert("X-Frame-Options".to_string(), "DENY".to_string());
        headers.insert("Access-Control-Allow-Origin".to_string(), "*".to_string());
        headers.insert(
            "Access-Control-Allow-Methods".to_string(),
            "GET, POST, PATCH, DELETE".to_string(),
        );
        headers.insert(
            "Access-Control-Allow-Headers".to_string(),
            "Content-Type".to_string(),
        );
        headers.insert("Cache-Control".to_string(), "no-store".to_string());
        headers
    }
}
