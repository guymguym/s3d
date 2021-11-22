use crate::gen::{kinds::S3OpKind, resolver::resolve_op_kind, resource::*};
use aws_smithy_types::instant::{Format, Instant};
use hyper::{header, Body, HeaderMap, Method};
use std::future::Future;
use std::pin::Pin;
use std::str::FromStr;
use std::{collections::HashMap, net::SocketAddr};
use url::Url;
use uuid::Uuid;

pub type HttpRequest = hyper::Request<Body>;
pub type HttpResponse = hyper::Response<Body>;

pub type S3Error = aws_smithy_types::Error;
pub type S3ClientError = aws_sdk_s3::Error;
pub type S3Result = Result<HttpResponse, S3Error>;

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
            resource,
            op_kind: None::<S3OpKind>,
        };
        req.op_kind = resolve_op_kind(&req);
        req
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

    pub fn get_param(&self, name: &str) -> Option<String> {
        self.params.get(name).map(|x| x.clone())
    }

    pub fn get_param_str(&self, name: &str) -> &str {
        self.params.get(name).map_or("", |x| x.as_str())
    }

    pub fn get_param_parse<T: FromStr>(&self, name: &str) -> Option<T> {
        self.params.get(name).and_then(|x| x.parse().ok())
    }

    pub fn get_param_date(&self, name: &str) -> Option<Instant> {
        self.params
            .get(name)
            .and_then(|x| Instant::from_str(&x, Format::HttpDate).ok())
    }

    pub fn has_header(&self, name: &str) -> bool {
        self.headers.contains_key(name)
    }

    pub fn get_header(&self, name: &str) -> Option<String> {
        self.headers
            .get(name)
            .and_then(|x| x.to_str().ok())
            .map(|x| x.to_owned())
    }

    pub fn get_header_str(&self, name: &str) -> &str {
        self.headers
            .get(name)
            .map_or("", |x| x.to_str().unwrap_or(""))
    }

    pub fn get_header_parse<T: FromStr>(&self, name: &str) -> Option<T> {
        self.headers
            .get(name)
            .and_then(|x| x.to_str().ok())
            .and_then(|x| x.parse().ok())
    }

    pub fn get_header_date(&self, name: &str) -> Option<Instant> {
        self.headers
            .get(name)
            .and_then(|x| x.to_str().ok())
            .and_then(|x| Instant::from_str(&x, Format::HttpDate).ok())
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
