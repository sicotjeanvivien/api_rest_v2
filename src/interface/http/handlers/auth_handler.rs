use std::{collections::HashMap, sync::Arc};

use crate::{
    application::services::auth_service::AuthService,
    domain::user::model::{User, UserAuth},
    interface::http::{
        error::http_error::HttpError, request::HttpRequest, response::{
            http_response::HttpResponse, into_http_response::IntoHttpResponse,
            status_code::StatusCode,
        }
    },
};

#[derive(Clone)]
pub struct AuthHandler {
    auth_service: Arc<AuthService>,
}

impl AuthHandler {
    pub fn new(auth_service: Arc<AuthService>) -> Self {
        Self { auth_service }
    }

    pub async fn login(&self, _request: HttpRequest) -> Result<HttpResponse, HttpResponse> {
      println!("login");
        let user_auth: UserAuth = serde_json::from_str(&_request.get_body()?)
            .map_err(|e| HttpError::BadRequest(e.to_string()).into_http_response())?;
        let user: User = self
            .auth_service
            .login(user_auth)
            .await
            .map_err(|e| e.into_http_response())?;

        Ok(HttpResponse::new(
            StatusCode::OK,
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
