use crate::interface::http::response::http_response::HttpResponse;

pub trait IntoHttpResponse {
    fn into_http_response(self) -> HttpResponse;
}