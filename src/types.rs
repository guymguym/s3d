use crate::gen::*;
use hyper::Body;
use hyper::{header, HeaderMap, Method};
use std::future::Future;
use std::pin::Pin;
use std::{collections::HashMap, net::SocketAddr};
use url::Url;
use uuid::Uuid;

pub type HttpRequest = hyper::Request<hyper::Body>;
pub type HttpResponse = hyper::Response<hyper::Body>;

pub type S3Error = aws_smithy_types::Error;
pub type S3ClientError = aws_sdk_s3::Error;
pub type S3Result = Result<HttpResponse, S3Error>;
pub type S3ResultNull = Result<(), S3Error>;

pub type S3C = aws_sdk_s3::Client;
pub type SMC = aws_smithy_client::Client<aws_hyper::DynConnector, aws_hyper::AwsMiddleware>;

/// Why we need this TraitFuture:
/// https://smallcultfollowing.com/babysteps/blog/2019/10/26/async-fn-in-traits-are-hard/
pub type TraitFuture<'a, O, E> = Pin<Box<dyn Future<Output = Result<O, E>> + Send + 'a>>;

pub fn responder() -> hyper::http::response::Builder {
    hyper::Response::builder()
}

#[derive(Debug)]
pub struct S3Request {
    // http request info
    pub url: Url,
    pub body: Body,
    pub method: Method,
    pub headers: HeaderMap,
    pub params: HashMap<String, String>,
    pub remote_addr: SocketAddr,

    /// reqid is a generated unique id for each request
    pub reqid: String,
    /// hostid is a an opaque id that can be used to find the host in the server that handled this request
    pub hostid: String,

    // parsed fields
    pub bucket: String,
    pub key: String,
    pub bucket_subresource: S3BucketSubResource,
    pub object_subresource: S3ObjectSubResource,
    pub op_kind: Option<S3OpKind>,
}

impl S3Request {
    pub fn new(http_req: HttpRequest, remote_addr: SocketAddr) -> S3Request {
        let (parts, body) = http_req.into_parts();
        let host_hdr = parts
            .headers
            .get(header::HOST)
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();
        let host_url = Url::parse(&format!("http://{}", host_hdr)).unwrap();
        let url = host_url.join(&parts.uri.to_string()).unwrap();
        let params = url.query_pairs().into_owned().collect();
        let reqid = Uuid::new_v4().to_string(); // unique id for each request
        S3Request {
            url,
            body,
            method: parts.method,
            headers: parts.headers,
            params,
            remote_addr,
            reqid,
            hostid: host_hdr,
            // init parsed fields to empty values for now
            bucket: "".to_string(),
            key: "".to_string(),
            bucket_subresource: S3BucketSubResource::None,
            object_subresource: S3ObjectSubResource::None,
            op_kind: None,
        }
    }

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
