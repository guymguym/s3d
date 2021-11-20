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
    pub resource: S3Resource,
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
            // bucket,
            // key,
            // bucket_subresource,
            // object_subresource,
            resource,
            op_kind: None::<S3OpKind>,
        };
        req.op_kind = match_op(&req);
        req
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
