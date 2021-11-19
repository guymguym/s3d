use crate::s3::*;
use hyper::Body;
use hyper::{HeaderMap, Method};
use std::future::Future;
use std::pin::Pin;
use std::{collections::HashMap, net::SocketAddr};
use url::Url;

/// Why we need this TraitFuture:
/// https://smallcultfollowing.com/babysteps/blog/2019/10/26/async-fn-in-traits-are-hard/
pub type TraitFuture<'a, O, E> = Pin<Box<dyn Future<Output = Result<O, E>> + Send + 'a>>;

pub type HttpRequest = hyper::Request<hyper::Body>;
pub type HttpResponse = hyper::Response<hyper::Body>;

pub type S3Errors = aws_sdk_s3::Error;
pub type S3Error = aws_smithy_types::Error;
pub type S3ErrorMeta = aws_smithy_types::Error;

pub type S3Result = Result<HttpResponse, S3Error>;
pub type S3ResultNull = Result<(), S3Error>;

pub type S3C = aws_sdk_s3::Client;
pub type AC = aws_smithy_client::Client<aws_hyper::DynConnector, aws_hyper::AwsMiddleware>;

pub fn responder() -> hyper::http::response::Builder {
    hyper::Response::builder()
}

#[derive(Debug)]
pub struct S3Request {
    // http request info
    pub remote_addr: SocketAddr,
    pub url: Url,
    pub body: Body,
    pub method: Method,
    pub headers: HeaderMap,

    /// reqid is a generated unique id for each request
    pub reqid: String,

    // parsed fields
    pub params: HashMap<String, String>,
    pub bucket: String,
    pub key: String,
    pub bucket_subresource: S3BucketSubResource,
    pub object_subresource: S3ObjectSubResource,
    pub op_kind: Option<S3OpKind>,
}

impl Default for S3Request {
    fn default() -> S3Request {
        S3Request {
            remote_addr: SocketAddr::from(([127, 0, 0, 1], 0)),
            url: Url::parse("").unwrap(),
            body: Body::empty(),
            method: Method::GET,
            headers: HeaderMap::new(),
            reqid: "".to_string(),
            params: HashMap::new(),
            bucket: "".to_string(),
            key: "".to_string(),
            bucket_subresource: S3BucketSubResource::None,
            object_subresource: S3ObjectSubResource::None,
            op_kind: None,
        }
    }
}

impl S3Request {
    pub fn get_param(&self, name: &str) -> Option<String> {
        self.params.get(name).map(|x| x.clone())
    }
    pub fn get_param_i32(&self, name: &str) -> Option<i32> {
        self.params.get(name).map(|x| x.parse().unwrap())
    }
    pub fn get_header(&self, name: &str) -> Option<String> {
        self.headers
            .get(name)
            .map_or(None, |x| x.to_str().map_or(None, |s| Some(s.to_string())))
    }
}
