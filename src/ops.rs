use crate::{err::*, util::*};
use aws_sdk_s3::{ByteStream, input::*, output::*};
use aws_smithy_http::response::ParseHttpResponse;
use hyper::{Body, HeaderMap, Method};
use std::collections::HashMap;
use url::Url;

#[derive(Debug)]
pub struct S3Request {
    // http request info
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

pub async fn list_buckets(
    s3c: &aws_sdk_s3::Client,
    _: ListBucketsInput,
) -> Result<ListBucketsOutput, S3Error> {
    Ok(s3c.list_buckets().send().await.or_else(|err| {
        error!("{:?}", err);
        Err(S3Error::new(S3Errors::InternalError))
    })?)
}

pub async fn list_objects(
    s3c: &aws_sdk_s3::Client,
    i: ListObjectsInput,
) -> Result<ListObjectsOutput, S3Error> {
    let op = i.make_operation(s3c.conf()).unwrap();
    let (req, parts) = op.into_request_response();
    // let pp = ParseHttpResponse::(req, parts);
    // pp.parse_unloaded();
    // op.
    Ok(s3c
        .list_objects()
        .set_bucket(i.bucket)
        .set_delimiter(i.delimiter)
        .set_encoding_type(i.encoding_type)
        .set_expected_bucket_owner(i.expected_bucket_owner)
        .set_marker(i.marker)
        .set_max_keys(Some(i.max_keys))
        .set_prefix(i.prefix)
        .set_request_payer(i.request_payer)
        .send()
        .await
        .or_else(|err| {
            error!("{:?}", err);
            Err(S3Error::new(S3Errors::InternalError))
        })?)
}

pub async fn get_object(
    s3c: &aws_sdk_s3::Client,
    i: GetObjectInput,
) -> Result<GetObjectOutput, S3Error> {
    Ok(s3c
        .get_object()
        .set_bucket(i.bucket)
        .set_key(i.key)
        .set_part_number(Some(i.part_number))
        .set_range(i.range)
        .set_version_id(i.version_id)
        .send()
        .await
        .or_else(|err| {
            error!("{:?}", err);
            Err(S3Error::new(S3Errors::InternalError))
        })?)
}

pub async fn head_bucket(_req: &S3Request) -> S3Result {
    Ok(http_response().body(Body::from("head_bucket\n")).unwrap())
}

pub async fn head_object(_req: &S3Request) -> S3Result {
    Ok(http_response().body(Body::from("head_object\n")).unwrap())
}

pub async fn put_bucket(_req: &S3Request) -> S3Result {
    Ok(http_response().body(Body::from("put_bucket\n")).unwrap())
}

pub async fn put_object(_req: &S3Request) -> S3Result {
    Ok(http_response().body(Body::from("put_object\n")).unwrap())
}

pub async fn delete_bucket(_req: &S3Request) -> S3Result {
    Ok(http_response().body(Body::from("delete_bucket\n")).unwrap())
}

pub async fn delete_object(_req: &S3Request) -> S3Result {
    Ok(http_response().body(Body::from("delete_object\n")).unwrap())
}

pub async fn post_object(_req: &S3Request) -> S3Result {
    Ok(http_response().body(Body::from("post_object\n")).unwrap())
}

pub async fn get_bucket_subresource(req: &S3Request) -> S3Result {
    Ok(http_response()
        .body(Body::from(format!(
            "get_bucket_subresource {:?}\n",
            req.bucket_subresource
        )))
        .unwrap())
}

pub async fn get_object_subresource(req: &S3Request) -> S3Result {
    Ok(http_response()
        .body(Body::from(format!(
            "get_object_subresource {:?}\n",
            req.object_subresource
        )))
        .unwrap())
}

pub async fn put_bucket_subresource(req: &S3Request) -> S3Result {
    Ok(http_response()
        .body(Body::from(format!(
            "put_bucket_subresource {:?}\n",
            req.bucket_subresource
        )))
        .unwrap())
}

pub async fn put_object_subresource(req: &S3Request) -> S3Result {
    Ok(http_response()
        .body(Body::from(format!(
            "put_object_subresource {:?}\n",
            req.object_subresource
        )))
        .unwrap())
}

pub async fn delete_bucket_subresource(req: &S3Request) -> S3Result {
    Ok(http_response()
        .body(Body::from(format!(
            "delete_bucket_subresource {:?}\n",
            req.bucket_subresource
        )))
        .unwrap())
}

pub async fn delete_object_subresource(req: &S3Request) -> S3Result {
    Ok(http_response()
        .body(Body::from(format!(
            "delete_object_subresource {:?}\n",
            req.object_subresource
        )))
        .unwrap())
}

pub async fn post_bucket_subresource(req: &S3Request) -> S3Result {
    Ok(http_response()
        .body(Body::from(format!(
            "post_bucket_subresource {:?}\n",
            req.bucket_subresource
        )))
        .unwrap())
}

pub async fn create_multipart_upload(req: &S3Request) -> S3Result {
    Ok(http_response()
        .body(Body::from(format!(
            "create_multipart_upload {:?}\n",
            req.object_subresource
        )))
        .unwrap())
}

pub async fn complete_multipart_upload(req: &S3Request) -> S3Result {
    Ok(http_response()
        .body(Body::from(format!(
            "complete_multipart_upload {:?}\n",
            req.object_subresource
        )))
        .unwrap())
}

pub async fn fetch_flow(_req: &S3Request) -> S3Result {
    Ok(http_response().body(Body::from("fetch_flow\n")).unwrap())
}

pub async fn pull_flow(_req: &S3Request) -> S3Result {
    Ok(http_response().body(Body::from("pull_flow\n")).unwrap())
}

pub async fn push_flow(_req: &S3Request) -> S3Result {
    Ok(http_response().body(Body::from("push_flow\n")).unwrap())
}

pub async fn prune_flow(_req: &S3Request) -> S3Result {
    Ok(http_response().body(Body::from("prune_flow\n")).unwrap())
}

pub async fn status_flow(_req: &S3Request) -> S3Result {
    Ok(http_response().body(Body::from("status_flow\n")).unwrap())
}

pub async fn diff_flow(_req: &S3Request) -> S3Result {
    Ok(http_response().body(Body::from("diff_flow\n")).unwrap())
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
