use crate::{err::*, util::*};
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

/// list_buckets
///
/// Request Syntax:
/// ```
/// GET / HTTP/1.1
/// ```
///
/// Response Syntax:
/// ```
/// HTTP/1.1 200
/// <?xml version="1.0" encoding="UTF-8"?>
/// <ListAllMyBucketsResult>
///    <Buckets>
///       <Bucket>
///          <CreationDate>timestamp</CreationDate>
///          <Name>string</Name>
///       </Bucket>
///    </Buckets>
///    <Owner>
///       <DisplayName>string</DisplayName>
///       <ID>string</ID>
///    </Owner>
/// </ListAllMyBucketsResult>
/// ```
pub async fn list_buckets(req: &S3Request, s3c: &aws_sdk_s3::Client) -> S3Result {
    let r = s3c.list_buckets().send().await.or_else(|err| {
        error!("{:?}", err);
        Err(S3Error::new(S3Errors::InternalError))
    })?;

    let mut w = BodyWriter::new_xml();
    w.append("<ListAllMyBucketsResult>");
    w.append("<Buckets>");
    for b in r.buckets.unwrap_or_default() {
        w.append("<Bucket>");
        w.append_xml("Name", b.name.unwrap().as_str());
        w.append_xml(
            "CreationDate",
            b.creation_date.unwrap().to_chrono().to_rfc3339().as_str(),
        );
        w.append("</Bucket>");
    }
    w.append("</Buckets>");
    let owner = r.owner.unwrap();
    w.append("<Owner>");
    w.append_xml("ID", owner.id.unwrap_or_default().as_str());
    w.append_xml(
        "DisplayName",
        owner.display_name.unwrap_or_default().as_str(),
    );
    w.append("</Owner>");
    w.append("</ListAllMyBucketsResult>");
    Ok(http_response().body(w.body()).unwrap())
}

/// list_objects
///
/// Request Syntax:
/// ```
/// GET /
///     ?delimiter=Delimiter
///     &encoding-type=EncodingType
///     &marker=Marker
///     &max-keys=MaxKeys
///     &prefix=Prefix
///     HTTP/1.1
/// Host: Bucket.s3.amazonaws.com
/// x-amz-request-payer: RequestPayer
/// x-amz-expected-bucket-owner: ExpectedBucketOwner
/// ```
///
/// Response Syntax:
/// ```
/// HTTP/1.1 200
/// <?xml version="1.0" encoding="UTF-8"?>
/// <ListBucketResult>
///    <IsTruncated>boolean</IsTruncated>
///    <Marker>string</Marker>
///    <NextMarker>string</NextMarker>
///    <Contents>
///       <ETag>string</ETag>
///       <Key>string</Key>
///       <LastModified>timestamp</LastModified>
///       <Owner>
///          <DisplayName>string</DisplayName>
///          <ID>string</ID>
///       </Owner>
///       <Size>integer</Size>
///       <StorageClass>string</StorageClass>
///    </Contents>
///    ...
///    <Name>string</Name>
///    <Prefix>string</Prefix>
///    <Delimiter>string</Delimiter>
///    <MaxKeys>integer</MaxKeys>
///    <CommonPrefixes>
///       <Prefix>string</Prefix>
///    </CommonPrefixes>
///    ...
///    <EncodingType>string</EncodingType>
/// </ListBucketResult>    
/// ```
pub async fn list_objects(req: &S3Request, s3c: &aws_sdk_s3::Client) -> S3Result {
    let r = s3c
        .list_objects()
        .set_bucket(Some(req.bucket.clone()))
        .set_delimiter(None)
        .set_encoding_type(None)
        .set_expected_bucket_owner(None)
        .set_marker(None)
        .set_max_keys(None)
        .set_prefix(None)
        .set_request_payer(None)
        .send()
        .await
        .or_else(|err| {
            error!("{:?}", err);
            Err(S3Error::new(S3Errors::InternalError))
        })?;

    let mut w = BodyWriter::new_xml();
    w.append("<ListBucketResult>");
    w.append_xml("IsTruncated", r.is_truncated.to_string().as_str());
    w.append_xml("NextMarker", r.next_marker.unwrap_or_default().as_str());
    w.append_xml("Name", r.name.unwrap_or_default().as_str());
    w.append_xml("Prefix", r.prefix.unwrap_or_default().as_str());
    w.append_xml("Marker", r.marker.unwrap_or_default().as_str());
    w.append_xml("Delimiter", r.delimiter.unwrap_or_default().as_str());
    w.append_xml("MaxKeys", r.max_keys.to_string().as_str());
    // w.append_xml("EncodingType", r.encoding_type.unwrap().as_str());
    for obj in r.contents.unwrap_or_default() {
        w.append("<Contents>");
        w.append_xml("Key", obj.key.unwrap_or_default().to_string().as_str());
        w.append_xml(
            "LastModified",
            obj.last_modified.unwrap().to_chrono().to_rfc3339().as_str(),
        );
        w.append_xml("ETag", obj.e_tag.unwrap_or_default().as_str());
        w.append_xml("Size", obj.size.to_string().as_str());
        // w.append_xml("StorageClass", obj.storage_class.unwrap().as_str());
        let owner = obj.owner.unwrap();
        w.append("<Owner>");
        w.append_xml("ID", owner.id.unwrap_or_default().as_str());
        w.append_xml(
            "DisplayName",
            owner.display_name.unwrap_or_default().as_str(),
        );
        w.append("</Owner>");
        w.append("</Contents>");
    }
    for p in r.common_prefixes.unwrap_or_default() {
        w.append("<CommonPrefixes>");
        w.append_xml("Prefix", p.prefix.unwrap_or_default().as_str());
        w.append("</CommonPrefixes>");
    }
    w.append("</ListBucketResult>");
    Ok(http_response().body(w.body()).unwrap())
}

pub async fn get_object(_req: &S3Request) -> S3Result {
    Ok(http_response().body(Body::from("get_object\n")).unwrap())
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
