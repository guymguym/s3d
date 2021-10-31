pub type HttpRequest = hyper::Request<hyper::Body>;
pub type HttpResponse = hyper::Response<hyper::Body>;
pub type HttpResult = anyhow::Result<HttpResponse>;
pub type HttpResultOrErr<E> = Result<HttpResponse, E>;

pub fn http_response() -> hyper::http::response::Builder {
    hyper::http::response::Builder::new()
}
