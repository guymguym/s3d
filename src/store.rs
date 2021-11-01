use crate::{err::*, util::*};
use hyper::{Body, HeaderMap, Method};
use std::collections::HashMap;

#[derive(Debug)]
pub struct S3Request {
    pub method: Method,
    pub path: String,
    pub query: String,
    pub headers: HeaderMap,
    pub body: Body,
    /// reqid is a generated unique id for each request
    pub reqid: String,
    // parsed fields
    pub params: HashMap<String, String>,
    pub bucket: String,
    pub key: String,
    pub bucket_sub_resource: S3BucketSubResource,
    pub object_sub_resource: S3ObjectSubResource,
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

pub async fn list_buckets(_req: &S3Request) -> S3Result {
    Ok(http_response().body(Body::from("list_buckets")).unwrap())
}

pub async fn list_objects(_req: &S3Request) -> S3Result {
    Ok(http_response().body(Body::from("list_objects")).unwrap())
}

pub async fn get_object(_req: &S3Request) -> S3Result {
    Ok(http_response().body(Body::from("get_object")).unwrap())
}

pub async fn head_bucket(_req: &S3Request) -> S3Result {
    Ok(http_response().body(Body::from("head_bucket")).unwrap())
}

pub async fn head_object(_req: &S3Request) -> S3Result {
    Ok(http_response().body(Body::from("head_object")).unwrap())
}

pub async fn put_bucket(_req: &S3Request) -> S3Result {
    Ok(http_response().body(Body::from("put_bucket")).unwrap())
}

pub async fn put_object(_req: &S3Request) -> S3Result {
    Ok(http_response().body(Body::from("put_object")).unwrap())
}

pub async fn delete_bucket(_req: &S3Request) -> S3Result {
    Ok(http_response().body(Body::from("delete_bucket")).unwrap())
}

pub async fn delete_object(_req: &S3Request) -> S3Result {
    Ok(http_response().body(Body::from("delete_object")).unwrap())
}

pub async fn post_object(_req: &S3Request) -> S3Result {
    Ok(http_response().body(Body::from("post_object")).unwrap())
}

pub async fn get_bucket_sub_resource(_req: &S3Request) -> S3Result {
    Ok(http_response()
        .body(Body::from("get_bucket_sub_resource"))
        .unwrap())
}

pub async fn get_object_sub_resource(_req: &S3Request) -> S3Result {
    Ok(http_response()
        .body(Body::from("get_object_sub_resource"))
        .unwrap())
}

pub async fn put_bucket_sub_resource(_req: &S3Request) -> S3Result {
    Ok(http_response()
        .body(Body::from("put_bucket_sub_resource"))
        .unwrap())
}

pub async fn put_object_sub_resource(_req: &S3Request) -> S3Result {
    Ok(http_response()
        .body(Body::from("put_object_sub_resource"))
        .unwrap())
}

pub async fn delete_bucket_sub_resource(_req: &S3Request) -> S3Result {
    Ok(http_response()
        .body(Body::from("delete_bucket_sub_resource"))
        .unwrap())
}

pub async fn delete_object_sub_resource(_req: &S3Request) -> S3Result {
    Ok(http_response()
        .body(Body::from("delete_object_sub_resource"))
        .unwrap())
}

pub async fn post_bucket_sub_resource(_req: &S3Request) -> S3Result {
    Ok(http_response()
        .body(Body::from("post_bucket_sub_resource"))
        .unwrap())
}

pub async fn create_multipart_upload(_req: &S3Request) -> S3Result {
    Ok(http_response()
        .body(Body::from("create_multipart_upload"))
        .unwrap())
}

pub async fn complete_multipart_upload(_req: &S3Request) -> S3Result {
    Ok(http_response()
        .body(Body::from("complete_multipart_upload"))
        .unwrap())
}

pub async fn fetch_flow(_req: &S3Request) -> S3Result {
    Ok(http_response().body(Body::from("fetch_flow")).unwrap())
}

pub async fn pull_flow(_req: &S3Request) -> S3Result {
    Ok(http_response().body(Body::from("pull_flow")).unwrap())
}

pub async fn push_flow(_req: &S3Request) -> S3Result {
    Ok(http_response().body(Body::from("push_flow")).unwrap())
}

pub async fn prune_flow(_req: &S3Request) -> S3Result {
    Ok(http_response().body(Body::from("prune_flow")).unwrap())
}

pub async fn status_flow(_req: &S3Request) -> S3Result {
    Ok(http_response().body(Body::from("status_flow")).unwrap())
}

pub async fn diff_flow(_req: &S3Request) -> S3Result {
    Ok(http_response().body(Body::from("diff_flow")).unwrap())
}

// pub async fn list_buckets() -> S3Result {
//     let s3_config = aws_config::from_env().load().await;
//     let client = aws_sdk_s3::Client::new(&s3_config);
//     // let client = S3Client::new(Region::Custom {
//     //     name: "local".to_string(),
//     //     endpoint: "http://localhost:4572".to_string(),
//     // });
//     let mut buckets = Vec::new();
//     let res = client.list_buckets().send().await.unwrap_or_else(|err| {
//         println!("{:?}", err);
//         Err(S3Error::_from(S3Errors::_BadRequest))
//     })?;
//     for bucket in res.buckets.unwrap_or_default() {
//         println!("_bucket: {:?}", bucket.name.as_deref().unwrap_or_default());
//         buckets.push(bucket.name.unwrap_or_default());
//     }
// }
