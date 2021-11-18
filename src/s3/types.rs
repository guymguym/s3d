use crate::s3::kind::*;
use hyper::Body;
use hyper::{HeaderMap, Method};
use std::{collections::HashMap, net::SocketAddr};
use url::Url;

pub type HttpRequest = hyper::Request<hyper::Body>;
pub type HttpResponse = hyper::Response<Body>;
pub type HttpResult = anyhow::Result<HttpResponse>;
pub type S3Result = Result<HttpResponse, S3Error>;
pub type S3ResultNull = Result<(), S3Error>;
pub type S3Errors = aws_sdk_s3::Error;
pub type S3Error = aws_smithy_types::Error;
pub type S3ErrorMeta = aws_smithy_types::Error;
pub type S3C = aws_sdk_s3::Client;
pub type AC = aws_smithy_client::Client<aws_hyper::DynConnector, aws_hyper::AwsMiddleware>;

pub fn responder() -> hyper::http::response::Builder {
    hyper::http::response::Builder::new()
}

#[async_trait::async_trait]
pub trait S3Op {
    const KIND: S3OpKind;
    type Input;
    type Output;
    type Error;
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

#[derive(Debug, PartialEq)]
pub enum S3BucketSubResource {
    None,
    Accelerate,
    Acl,
    Analytics,
    Cors,
    Encryption,
    IntelligentTiering,
    Inventory,
    Lifecycle,
    Location,
    Logging,
    Metrics,
    Notification,
    OwnershipControls,
    Policy,
    PolicyStatus,
    PublicAccessBlock,
    Replication,
    RequestPayment,
    Tagging,
    Versioning,
    Website,
}

#[derive(Debug, PartialEq)]
pub enum S3ObjectSubResource {
    None,
    Acl,
    LegalHold,
    ObjectLock,
    Restore,
    Retention,
    SelectObjectContent,
    Tagging,
    Torrent,
    Uploads,
    UploadId,
    Versions,
}

impl From<&str> for S3BucketSubResource {
    fn from(s: &str) -> Self {
        match s {
            "accelerate" => Self::Accelerate,
            "acl" => Self::Acl,
            "analytics" => Self::Analytics,
            "cors" => Self::Cors,
            "encryption" => Self::Encryption,
            "intelligent-tiering" => Self::IntelligentTiering,
            "inventory" => Self::Inventory,
            "lifecycle" => Self::Lifecycle,
            "location" => Self::Location,
            "logging" => Self::Logging,
            "metrics" => Self::Metrics,
            "notification" => Self::Notification,
            "ownershipControls" => Self::OwnershipControls,
            "policy" => Self::Policy,
            "policyStatus" => Self::PolicyStatus,
            "publicAccessBlock" => Self::PublicAccessBlock,
            "replication" => Self::Replication,
            "requestPayment" => Self::RequestPayment,
            "tagging" => Self::Tagging,
            "versioning" => Self::Versioning,
            "website" => Self::Website,
            _ => Self::None,
        }
    }
}

impl From<&str> for S3ObjectSubResource {
    fn from(s: &str) -> Self {
        match s {
            "acl" => Self::Acl,
            "legal-hold" => Self::LegalHold,
            "object-lock" => Self::ObjectLock,
            "restore" => Self::Restore,
            "retention" => Self::Retention,
            "select" => Self::SelectObjectContent,
            "tagging" => Self::Tagging,
            "torrent" => Self::Torrent,
            "uploads" => Self::Uploads,
            "uploadId" => Self::UploadId,
            "versions" => Self::Versions,
            _ => Self::None,
        }
    }
}

pub enum InputError {
    Unhandled(anyhow::Error),
    NotImplemented,
}
pub enum OutputError {
    Unknown,
    NotImplemented,
}
pub enum ServeError {
    InputError(InputError),
    S3Error(S3Error),
    OutputError(OutputError),
}
impl From<InputError> for ServeError {
    fn from(e: InputError) -> Self {
        ServeError::InputError(e)
    }
}
impl From<S3Error> for ServeError {
    fn from(e: S3Error) -> Self {
        ServeError::S3Error(e)
    }
}
impl From<OutputError> for ServeError {
    fn from(e: OutputError) -> Self {
        ServeError::OutputError(e)
    }
}
