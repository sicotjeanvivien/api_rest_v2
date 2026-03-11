use crate::interface::http::response::status_code::StatusCode;
use std::collections::HashMap;

pub struct HttpResponse {
    status_code: StatusCode,
    headers: HashMap<String, String>,
    body: Option<String>,
}

impl HttpResponse {
    pub fn new(
        status_code: StatusCode,
        headers: HashMap<String, String>,
        body: Option<String>,
    ) -> Self {
        Self {
            status_code,
            headers,
            body,
        }
    }

    pub fn to_string(&self) -> String {
        let mut response = String::new();
        let body_len = match &self.body {
            Some(b) => b.len(),
            None => 0,
        };
        self.generate_status_line(&mut response);
        self.generate_headers(&mut response, body_len);
        self.generate_body(&mut response);
        response
    }

    fn generate_status_line(&self, response: &mut String) {
        response.push_str(&format!(
            "HTTP/1.1 {} {}\r\n",
            self.status_code.to_u16(),
            self.status_code.to_text()
        ))
    }

    fn generate_headers(&self, response: &mut String, body_len: usize) {
        self.headers
            .iter()
            .for_each(|h| response.push_str(&format!("{}: {}\r\n", h.0, h.1)));
        response.push_str(&format!("Content-Length: {}\r\n", body_len));
        response.push_str("\r\n");
    }

    fn generate_body(&self, response: &mut String) {
        if let Some(v) = &self.body {
            response.push_str(v);
        }
    }
}
