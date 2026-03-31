use std::collections::HashMap;

use tokio::{io::AsyncReadExt, net::TcpStream};

use crate::interface::{HttpError, HttpMethod, HttpRequest};

pub async fn decode_request(stream: &mut TcpStream) -> Result<HttpRequest, HttpError> {
    let mut buffer = [0; 2048];
    let bytes_read = stream.read(&mut buffer).await.unwrap();

    let request_str = String::from_utf8_lossy(&buffer[..bytes_read]);

    let mut lines = request_str.lines();

    // ---- 1️⃣ Request line ----
    let request_line: &str = lines.next().unwrap_or("");
    let mut request_parts: std::str::SplitWhitespace<'_> = request_line.split_whitespace();

    let method: HttpMethod = parse_method(request_parts.next().unwrap_or(""))?;
    let path: &str = request_parts.next().unwrap_or("");
    let http_version: &str = request_parts.next().unwrap_or("");
    let params: HashMap<String, String> = HashMap::new();

    // ---- 2️⃣ Headers ----
    let mut headers: HashMap<String, String> = HashMap::new();

    for line in &mut lines {
        if line.is_empty() {
            break;
        }

        if let Some((key, value)) = line.split_once(":") {
            headers.insert(key.trim().to_string(), value.trim().to_string());
        }
    }

    // ---- 3️⃣ Body ----
    let body: String = lines.collect::<Vec<&str>>().join("\n");

    Ok(HttpRequest::new(
        method,
        path.into(),
        params,
        http_version.into(),
        headers,
        if body.is_empty() { None } else { Some(body) },
    ))
}

fn parse_method(request_method: &str) -> Result<HttpMethod, HttpError> {
    match request_method {
        "GET" => Ok(HttpMethod::GET),
        "POST" => Ok(HttpMethod::POST),
        "PUT" => Ok(HttpMethod::PUT),
        "PATCH" => Ok(HttpMethod::PATCH),
        "HEAD" => Ok(HttpMethod::HEAD),
        "DELETE" => Ok(HttpMethod::DELETE),
        "CONNECT" => Ok(HttpMethod::CONNECT),
        "OPTIONS" => Ok(HttpMethod::OPTIONS),
        "TRACE" => Ok(HttpMethod::TRACE),
        e => Err(HttpError::MethodNotFound(e.to_string())),
    }
}
