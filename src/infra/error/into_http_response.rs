use crate::infra::http::response::HttpResponse;

pub trait IntoHttpResponse {
    fn into_http_response(self) -> HttpResponse;
}