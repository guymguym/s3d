use crate::gen::S3Ops;
use crate::resources::*;
use aws_smithy_http::operation::BuildError;
use aws_smithy_types::date_time::{DateTime, Format};
use aws_smithy_xml::decode::XmlError;
use hyper::{
    body::{to_bytes, Bytes},
    header::{HeaderName, HeaderValue},
    Body, HeaderMap, Method,
};
use std::future::Future;
use std::pin::Pin;
use std::str::FromStr;
use std::{collections::HashMap, net::SocketAddr};
use url::Url;
use uuid::Uuid;

pub type HttpRequest = hyper::Request<Body>;
pub type HttpResponse = hyper::Response<Body>;

pub type S3InnerError = aws_smithy_types::Error;
pub type S3InnerBuilder = aws_smithy_types::error::Builder;
pub type S3ClientError = aws_sdk_s3::Error;
pub type S3Result = Result<HttpResponse, S3Error>;

pub fn responder() -> hyper::http::response::Builder {
    hyper::Response::builder()
}

/// Why we need this TraitFuture:
/// We can't use async_trait macro inside our macro so we use the same thing it does
/// which is this pin-box-dyn-future - see long explanation here:
/// https://smallcultfollowing.com/babysteps/blog/2019/10/26/async-fn-in-traits-are-hard/
pub type TraitFuture<'a, O, E> = Pin<Box<dyn Future<Output = Result<O, E>> + Send + 'a>>;

pub trait ServerOperation {
    type Input;
    type Output;
    type Error;
    const OP: S3Ops;
    const IS_INPUT_BODY_STREAMING: bool;
    const IS_OUTPUT_BODY_STREAMING: bool;
    fn decode_input(req: &mut S3Request) -> TraitFuture<Self::Input, S3Error>;
    fn encode_output(o: Self::Output) -> TraitFuture<'static, HttpResponse, S3Error>;
}

#[derive(Debug)]
pub struct S3Error {
    pub inner: aws_smithy_types::Error,
}
impl std::error::Error for S3Error {}
impl std::fmt::Display for S3Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}
impl S3Error {
    pub fn builder() -> S3InnerBuilder {
        S3InnerError::builder()
    }
    pub fn bad_request(message: impl Into<String>) -> S3Error {
        S3Error {
            inner: S3InnerError::builder()
                .code("BadRequest")
                .message(message)
                .build(),
        }
    }
}
impl From<S3InnerError> for S3Error {
    fn from(inner: S3InnerError) -> Self {
        S3Error { inner }
    }
}
impl From<hyper::Error> for S3Error {
    fn from(e: hyper::Error) -> Self {
        S3Error::bad_request(e.to_string())
    }
}
impl From<BuildError> for S3Error {
    fn from(e: BuildError) -> Self {
        S3Error::bad_request(e.to_string())
    }
}
impl From<XmlError> for S3Error {
    fn from(e: XmlError) -> Self {
        S3Error::bad_request(e.to_string())
    }
}
impl From<hyper::http::Error> for S3Error {
    fn from(e: hyper::http::Error) -> Self {
        S3Error::bad_request(e.to_string())
    }
}

#[derive(Debug)]
pub struct S3Request {
    // use take_body() to consume the body
    body: Body,

    // http request info
    pub url: Url,
    pub method: Method,
    pub headers: HeaderMap,
    pub params: HashMap<String, String>,
    pub remote_addr: SocketAddr,

    /// reqid is a generated unique id for each request
    pub reqid: String,
    /// hostid is a an opaque id that can be used to find the host in the server that handled this request
    pub hostid: String,

    // parsed fields
    pub resource: S3Resource,
    pub op_kind: Option<S3Ops>,
}

impl S3Request {
    pub fn new(http_req: HttpRequest, remote_addr: SocketAddr) -> S3Request {
        let (parts, body) = http_req.into_parts();
        let host_hdr = parts
            .headers
            .get(hyper::header::HOST)
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();
        let host_url = Url::parse(&format!("http://{}", host_hdr)).unwrap();
        let url = host_url.join(&parts.uri.to_string()).unwrap();
        let mut params = HashMap::<String, String>::new();
        // unique id for each request
        let reqid = Uuid::new_v4().to_string();
        // parse path-style addressing for bucket names
        // TODO: add support for host-style addressing
        assert!(url.path().starts_with("/"));
        let path_items: Vec<&str> = url.path()[1..].splitn(2, "/").collect();
        let bucket = String::from(*path_items.get(0).unwrap_or(&""));
        let key = String::from(*path_items.get(1).unwrap_or(&""));
        let mut resource = S3Resource::Service;
        if bucket.is_empty() && key.is_empty() {
            for (key, val) in url.query_pairs() {
                params.insert(String::from(key), String::from(val));
            }
        } else if !bucket.is_empty() && key.is_empty() {
            let mut sub = S3BucketSubResource::None;
            for (key, val) in url.query_pairs() {
                if sub == S3BucketSubResource::None {
                    sub = S3BucketSubResource::from(key.as_ref());
                }
                params.insert(String::from(key), String::from(val));
            }
            resource = S3Resource::Bucket(S3BucketResource {
                bucket,
                sub_resource: sub,
            });
        } else {
            let mut sub = S3ObjectSubResource::None;
            for (key, val) in url.query_pairs() {
                if sub == S3ObjectSubResource::None {
                    sub = S3ObjectSubResource::from(key.as_ref());
                }
                params.insert(String::from(key), String::from(val));
            }
            resource = S3Resource::Object(S3ObjectResource {
                bucket,
                key,
                sub_resource: sub,
            });
        }
        let mut req = S3Request {
            url,
            body,
            method: parts.method,
            headers: parts.headers,
            params,
            remote_addr,
            reqid,
            hostid: host_hdr,
            resource,
            op_kind: None::<S3Ops>,
        };
        req.op_kind = resolve_op_kind(&req);
        req
    }

    pub fn take_body(&mut self) -> Body {
        std::mem::take(&mut self.body)
    }

    pub async fn take_body_bytes(&mut self) -> Result<Bytes, S3Error> {
        Ok(to_bytes(self.take_body()).await?)
    }

    pub fn get_bucket(&self) -> &str {
        self.resource.get_bucket()
    }

    pub fn get_key(&self) -> &str {
        self.resource.get_key()
    }

    pub fn has_param(&self, name: &str) -> bool {
        self.params.contains_key(name)
    }

    pub fn get_param_str(&self, name: &str) -> &str {
        self.params.get(name).map_or("", |x| x.as_str())
    }

    pub fn get_param<T: FromHttp>(&self, name: &str) -> Option<T> {
        self.params.get(name).and_then(|x| T::from_http(x))
    }

    pub fn has_header(&self, name: &str) -> bool {
        self.headers.contains_key(name)
    }

    pub fn get_header<T: FromHttp>(&self, name: &str) -> Option<T> {
        self.headers
            .get(name)
            .and_then(|x| x.to_str().ok())
            .and_then(|x| T::from_http(x))
    }

    pub fn get_header_map(&self, prefix: &str) -> Option<HashMap<String, String>> {
        let mut map = HashMap::<String, String>::new();
        for (key, val) in self.headers.iter() {
            let mut key = key.to_string();
            if key.starts_with(prefix) {
                map.insert(
                    key.split_off(prefix.len()),
                    val.to_str().unwrap_or("").to_string(),
                );
            }
        }
        Some(map)
    }
}

pub trait FromHttp: Sized {
    fn from_http(v: &str) -> Option<Self>;
}
impl FromHttp for String {
    fn from_http(v: &str) -> Option<Self> {
        v.parse().ok()
    }
}
impl FromHttp for bool {
    fn from_http(v: &str) -> Option<Self> {
        v.parse().ok()
    }
}
impl FromHttp for i32 {
    fn from_http(v: &str) -> Option<Self> {
        v.parse().ok()
    }
}
impl FromHttp for i64 {
    fn from_http(v: &str) -> Option<Self> {
        v.parse().ok()
    }
}
impl FromHttp for DateTime {
    fn from_http(v: &str) -> Option<Self> {
        DateTime::from_str(&v, Format::HttpDate).ok()
    }
}

pub trait ToHeader: Sized {
    fn to_header(self) -> Option<HeaderValue>;
    fn set_header(self, h: &mut HeaderMap<HeaderValue>, key: &'static str) {
        if let Some(value) = self.to_header() {
            h.insert(key, value);
        }
    }
    fn set_header_non_static(self, h: &mut HeaderMap<HeaderValue>, key: &str) {
        if let Some(value) = self.to_header() {
            let k = HeaderName::from_str(key).unwrap();
            h.insert(k, value);
        }
    }
}

impl ToHeader for HeaderValue {
    fn to_header(self) -> Option<HeaderValue> {
        Some(self)
    }
}
impl ToHeader for &str {
    fn to_header(self) -> Option<HeaderValue> {
        HeaderValue::from_str(self).ok()
    }
}
impl ToHeader for String {
    fn to_header(self) -> Option<HeaderValue> {
        self.as_str().to_header()
    }
}
impl ToHeader for &String {
    fn to_header(self) -> Option<HeaderValue> {
        self.as_str().to_header()
    }
}
impl ToHeader for &DateTime {
    fn to_header(self) -> Option<HeaderValue> {
        self.fmt(Format::DateTime).ok().to_header()
    }
}
impl<T: ToHeader> ToHeader for Option<T> {
    fn to_header(self) -> Option<HeaderValue> {
        self.and_then(|s| s.to_header())
    }
}

macro_rules! to_header_to_string {
    ($t:ty) => {
        impl ToHeader for $t {
            fn to_header(self) -> Option<HeaderValue> {
                self.to_string().to_header()
            }
        }
    };
}

to_header_to_string!(i64);
to_header_to_string!(i32);
to_header_to_string!(bool);

// macro_rules! to_header_as_str {
//     ($t:ty) => {
//         impl ToHeader for &$t {
//             fn to_header(self) -> Option<HeaderValue> {
//                 self.as_str().to_header()
//             }
//         }
//     };
// }

// to_header_as_str!(aws_sdk_s3::model::StorageClass);
// to_header_as_str!(aws_sdk_s3::model::ArchiveStatus);
// to_header_as_str!(aws_sdk_s3::model::RequestCharged);
// to_header_as_str!(aws_sdk_s3::model::ReplicationStatus);
// to_header_as_str!(aws_sdk_s3::model::ServerSideEncryption);
// to_header_as_str!(aws_sdk_s3::model::ObjectLockMode);
// to_header_as_str!(aws_sdk_s3::model::ObjectLockLegalHoldStatus);
