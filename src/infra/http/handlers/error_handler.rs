use std::collections::HashMap;

use crate::infra::http::response::{Response, StatusCode};

pub struct ErrorHandler {}

impl ErrorHandler {
  pub fn internal_server_error()-> Response {
      Response::new(StatusCode::InternalServerError, HashMap::new(), None)
  }
}