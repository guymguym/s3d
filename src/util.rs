use hyper;
use std::error::Error;

pub type AnyError = Box<dyn Error + Send + Sync>;
pub type ResultOrAnyErr<T> = Result<T, AnyError>;

pub type HttpRequest = hyper::Request<hyper::Body>;
pub type HttpResponse = hyper::Response<hyper::Body>;
pub type HttpResult = ResultOrAnyErr<HttpResponse>;
pub type HttpResultOrErr<E: Error> = Result<HttpResponse, E>;

// #[macro_export]
// macro_rules! log {
//     ($fmt:expr) => {
//         println!($fmt)
//     };
//     ($fmt:expr, $($arg:tt)*) => {
//         println!($fmt, $($arg)*)
//     }
// }

pub fn http_response() -> hyper::http::response::Builder {
    hyper::http::response::Builder::new()
}

