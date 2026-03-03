use std::collections::HashMap;

use crate::infra::http::response::{HttpResponse, StatusCode};

pub struct ErrorHandler {}

impl ErrorHandler {
  pub fn internal_server_error()-> HttpResponse {
      HttpResponse::new(StatusCode::InternalServerError, HashMap::new(), None)
  }
}