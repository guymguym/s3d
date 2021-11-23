//! This module resolves S3 resources to ops.
//! Currently written by hand which is difficult to maintain long term.
//! TODO This module should be generated from https://github.com/awslabs/smithy-rs.

use crate::{gen::ops::S3OpKind, http::*};
use hyper::Method;

#[derive(Debug)]
pub enum S3Resource {
    Service,
    Bucket(S3BucketResource),
    Object(S3ObjectResource),
}

impl S3Resource {
    pub fn get_bucket(&self) -> &str {
        match self {
            S3Resource::Bucket(b) => b.bucket.as_str(),
            S3Resource::Object(o) => o.bucket.as_str(),
            _ => panic!("Expected bucket resource type: {:?}", self),
        }
    }
    pub fn get_key(&self) -> &str {
        match self {
            S3Resource::Object(o) => o.key.as_str(),
            _ => panic!("Expected object resource type: {:?}", self),
        }
    }
}

#[derive(Debug)]
pub struct S3BucketResource {
    pub bucket: String,
    pub sub_resource: S3BucketSubResource,
}

#[derive(Debug)]
pub struct S3ObjectResource {
    pub bucket: String,
    pub key: String,
    pub sub_resource: S3ObjectSubResource,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
    ObjectLock,
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum S3ObjectSubResource {
    None,
    Acl,
    LegalHold,
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
            "object-lock" => Self::ObjectLock,
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

pub fn resolve_op_kind(req: &S3Request) -> Option<S3OpKind> {
    match &req.resource {
        S3Resource::Service => service::resolve(req),
        S3Resource::Bucket(b) => match b.sub_resource {
            S3BucketSubResource::None => bucket::resolve(req),
            S3BucketSubResource::Accelerate => bucket::resolve_accelerate(req),
            S3BucketSubResource::Acl => bucket::resolve_acl(req),
            S3BucketSubResource::Analytics => bucket::resolve_analytics(req),
            S3BucketSubResource::Cors => bucket::resolve_cors(req),
            S3BucketSubResource::Encryption => bucket::resolve_encryption(req),
            S3BucketSubResource::IntelligentTiering => bucket::resolve_intelligent_tiering(req),
            S3BucketSubResource::Inventory => bucket::resolve_inventory(req),
            S3BucketSubResource::Lifecycle => bucket::resolve_lifecycle(req),
            S3BucketSubResource::Location => bucket::resolve_location(req),
            S3BucketSubResource::Logging => bucket::resolve_logging(req),
            S3BucketSubResource::Metrics => bucket::resolve_metrics(req),
            S3BucketSubResource::Notification => bucket::resolve_notification(req),
            S3BucketSubResource::ObjectLock => bucket::resolve_object_lock(req),
            S3BucketSubResource::OwnershipControls => bucket::resolve_ownership_controls(req),
            S3BucketSubResource::Policy => bucket::resolve_policy(req),
            S3BucketSubResource::PolicyStatus => bucket::resolve_policy_status(req),
            S3BucketSubResource::PublicAccessBlock => bucket::resolve_public_access_block(req),
            S3BucketSubResource::Replication => bucket::resolve_replication(req),
            S3BucketSubResource::RequestPayment => bucket::resolve_request_payment(req),
            S3BucketSubResource::Tagging => bucket::resolve_tagging(req),
            S3BucketSubResource::Versioning => bucket::resolve_versioning(req),
            S3BucketSubResource::Website => bucket::resolve_website(req),
        },
        S3Resource::Object(o) => match o.sub_resource {
            S3ObjectSubResource::None => object::match_object(req),
            S3ObjectSubResource::Acl => object::resolve_acl(req),
            S3ObjectSubResource::LegalHold => object::resolve_legal_hold(req),
            S3ObjectSubResource::Restore => object::resolve_restore(req),
            S3ObjectSubResource::Retention => object::resolve_retention(req),
            S3ObjectSubResource::SelectObjectContent => object::resolve_select_object_content(req),
            S3ObjectSubResource::Tagging => object::resolve_tagging(req),
            S3ObjectSubResource::Torrent => object::resolve_torrent(req),
            S3ObjectSubResource::Uploads => object::resolve_uploads(req),
            S3ObjectSubResource::UploadId => object::resolve_upload_id(req),
            S3ObjectSubResource::Versions => object::resolve_versions(req),
        },
    }
}

pub mod service {
    use super::*;

    pub fn resolve(req: &S3Request) -> Option<S3OpKind> {
        match req.method {
            Method::GET => Some(S3OpKind::ListBuckets),
            _ => None,
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
// Bucket                                                                    //
///////////////////////////////////////////////////////////////////////////////
pub mod bucket {
    use super::*;

    pub fn resolve(req: &S3Request) -> Option<S3OpKind> {
        match req.method {
            Method::GET => {
                if req.get_param_str("list-type") == "2" {
                    Some(S3OpKind::ListObjectsV2)
                } else {
                    Some(S3OpKind::ListObjects)
                }
            }
            Method::HEAD => Some(S3OpKind::HeadBucket),
            Method::PUT => Some(S3OpKind::CreateBucket),
            Method::DELETE => Some(S3OpKind::DeleteBucket),
            _ => None,
        }
    }
    pub fn resolve_accelerate(req: &S3Request) -> Option<S3OpKind> {
        match req.method {
            Method::GET => Some(S3OpKind::GetBucketAccelerateConfiguration),
            Method::PUT => Some(S3OpKind::PutBucketAccelerateConfiguration),
            _ => None,
        }
    }
    pub fn resolve_acl(req: &S3Request) -> Option<S3OpKind> {
        match req.method {
            Method::GET => Some(S3OpKind::GetBucketAcl),
            Method::PUT => Some(S3OpKind::PutBucketAcl),
            _ => None,
        }
    }
    pub fn resolve_analytics(req: &S3Request) -> Option<S3OpKind> {
        match req.method {
            Method::GET => {
                if req.has_param("id") {
                    Some(S3OpKind::GetBucketAnalyticsConfiguration)
                } else {
                    Some(S3OpKind::ListBucketAnalyticsConfigurations)
                }
            }
            Method::PUT => Some(S3OpKind::PutBucketAnalyticsConfiguration),
            Method::DELETE => Some(S3OpKind::DeleteBucketAnalyticsConfiguration),
            _ => None,
        }
    }
    pub fn resolve_cors(req: &S3Request) -> Option<S3OpKind> {
        match req.method {
            Method::GET => Some(S3OpKind::GetBucketCors),
            Method::PUT => Some(S3OpKind::PutBucketCors),
            Method::DELETE => Some(S3OpKind::DeleteBucketCors),
            _ => None,
        }
    }
    pub fn resolve_encryption(req: &S3Request) -> Option<S3OpKind> {
        match req.method {
            Method::GET => Some(S3OpKind::GetBucketEncryption),
            Method::PUT => Some(S3OpKind::PutBucketEncryption),
            Method::DELETE => Some(S3OpKind::DeleteBucketEncryption),
            _ => None,
        }
    }
    pub fn resolve_intelligent_tiering(req: &S3Request) -> Option<S3OpKind> {
        match req.method {
            Method::GET => {
                if req.has_param("id") {
                    Some(S3OpKind::GetBucketIntelligentTieringConfiguration)
                } else {
                    Some(S3OpKind::ListBucketIntelligentTieringConfigurations)
                }
            }
            Method::PUT => Some(S3OpKind::PutBucketIntelligentTieringConfiguration),
            Method::DELETE => Some(S3OpKind::DeleteBucketIntelligentTieringConfiguration),
            _ => None,
        }
    }
    pub fn resolve_inventory(req: &S3Request) -> Option<S3OpKind> {
        match req.method {
            Method::GET => {
                if req.has_param("id") {
                    Some(S3OpKind::GetBucketInventoryConfiguration)
                } else {
                    Some(S3OpKind::ListBucketInventoryConfigurations)
                }
            }
            Method::PUT => Some(S3OpKind::PutBucketInventoryConfiguration),
            Method::DELETE => Some(S3OpKind::DeleteBucketInventoryConfiguration),
            _ => None,
        }
    }
    pub fn resolve_lifecycle(req: &S3Request) -> Option<S3OpKind> {
        match req.method {
            Method::GET => Some(S3OpKind::GetBucketLifecycleConfiguration),
            Method::PUT => Some(S3OpKind::PutBucketLifecycleConfiguration),
            Method::DELETE => Some(S3OpKind::DeleteBucketLifecycle),
            _ => None,
        }
    }
    pub fn resolve_location(req: &S3Request) -> Option<S3OpKind> {
        match req.method {
            Method::GET => Some(S3OpKind::GetBucketLocation),
            _ => None,
        }
    }
    pub fn resolve_logging(req: &S3Request) -> Option<S3OpKind> {
        match req.method {
            Method::GET => Some(S3OpKind::GetBucketLogging),
            Method::PUT => Some(S3OpKind::PutBucketLogging),
            _ => None,
        }
    }
    pub fn resolve_metrics(req: &S3Request) -> Option<S3OpKind> {
        match req.method {
            Method::GET => {
                if req.has_param("id") {
                    Some(S3OpKind::GetBucketMetricsConfiguration)
                } else {
                    Some(S3OpKind::ListBucketMetricsConfigurations)
                }
            }
            Method::PUT => Some(S3OpKind::PutBucketMetricsConfiguration),
            Method::DELETE => Some(S3OpKind::DeleteBucketMetricsConfiguration),
            _ => None,
        }
    }
    pub fn resolve_notification(req: &S3Request) -> Option<S3OpKind> {
        match req.method {
            Method::GET => Some(S3OpKind::GetBucketNotificationConfiguration),
            Method::PUT => Some(S3OpKind::PutBucketNotificationConfiguration),
            _ => None,
        }
    }
    pub fn resolve_object_lock(req: &S3Request) -> Option<S3OpKind> {
        match req.method {
            Method::GET => Some(S3OpKind::GetObjectLockConfiguration),
            Method::PUT => Some(S3OpKind::PutObjectLockConfiguration),
            _ => None,
        }
    }
    pub fn resolve_ownership_controls(req: &S3Request) -> Option<S3OpKind> {
        match req.method {
            Method::GET => Some(S3OpKind::GetBucketOwnershipControls),
            Method::PUT => Some(S3OpKind::PutBucketOwnershipControls),
            Method::DELETE => Some(S3OpKind::DeleteBucketOwnershipControls),
            _ => None,
        }
    }
    pub fn resolve_policy(req: &S3Request) -> Option<S3OpKind> {
        match req.method {
            Method::GET => Some(S3OpKind::GetBucketPolicy),
            Method::PUT => Some(S3OpKind::PutBucketPolicy),
            Method::DELETE => Some(S3OpKind::DeleteBucketPolicy),
            _ => None,
        }
    }
    pub fn resolve_policy_status(req: &S3Request) -> Option<S3OpKind> {
        match req.method {
            Method::GET => Some(S3OpKind::GetBucketPolicyStatus),
            _ => None,
        }
    }
    pub fn resolve_public_access_block(req: &S3Request) -> Option<S3OpKind> {
        match req.method {
            Method::GET => Some(S3OpKind::GetPublicAccessBlock),
            Method::PUT => Some(S3OpKind::PutPublicAccessBlock),
            Method::DELETE => Some(S3OpKind::DeletePublicAccessBlock),
            _ => None,
        }
    }
    pub fn resolve_replication(req: &S3Request) -> Option<S3OpKind> {
        match req.method {
            Method::GET => Some(S3OpKind::GetBucketReplication),
            Method::PUT => Some(S3OpKind::PutBucketReplication),
            Method::DELETE => Some(S3OpKind::DeleteBucketReplication),
            _ => None,
        }
    }
    pub fn resolve_request_payment(req: &S3Request) -> Option<S3OpKind> {
        match req.method {
            Method::GET => Some(S3OpKind::GetBucketRequestPayment),
            Method::PUT => Some(S3OpKind::PutBucketRequestPayment),
            _ => None,
        }
    }
    pub fn resolve_tagging(req: &S3Request) -> Option<S3OpKind> {
        match req.method {
            Method::GET => Some(S3OpKind::GetBucketTagging),
            Method::PUT => Some(S3OpKind::PutBucketTagging),
            Method::DELETE => Some(S3OpKind::DeleteBucketTagging),
            _ => None,
        }
    }
    pub fn resolve_versioning(req: &S3Request) -> Option<S3OpKind> {
        match req.method {
            Method::GET => Some(S3OpKind::GetBucketVersioning),
            Method::PUT => Some(S3OpKind::PutBucketVersioning),
            _ => None,
        }
    }
    pub fn resolve_website(req: &S3Request) -> Option<S3OpKind> {
        match req.method {
            Method::GET => Some(S3OpKind::GetBucketWebsite),
            Method::PUT => Some(S3OpKind::PutBucketWebsite),
            Method::DELETE => Some(S3OpKind::DeleteBucketWebsite),
            _ => None,
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
// Object                                                                    //
///////////////////////////////////////////////////////////////////////////////
pub mod object {
    use crate::gen::headers::X_AMZ_COPY_SOURCE;

    use super::*;

    pub fn match_object(req: &S3Request) -> Option<S3OpKind> {
        match req.method {
            Method::GET => Some(S3OpKind::GetObject),
            Method::HEAD => Some(S3OpKind::HeadObject),
            Method::PUT => {
                if req.has_header(X_AMZ_COPY_SOURCE) {
                    Some(S3OpKind::CopyObject)
                } else {
                    Some(S3OpKind::PutObject)
                }
            }
            Method::DELETE => Some(S3OpKind::DeleteObject),
            _ => None,
        }
    }
    pub fn resolve_acl(req: &S3Request) -> Option<S3OpKind> {
        match req.method {
            Method::GET => Some(S3OpKind::GetObjectAcl),
            Method::PUT => Some(S3OpKind::PutObjectAcl),
            _ => None,
        }
    }
    pub fn resolve_legal_hold(req: &S3Request) -> Option<S3OpKind> {
        match req.method {
            Method::GET => Some(S3OpKind::GetObjectLegalHold),
            Method::PUT => Some(S3OpKind::PutObjectLegalHold),
            _ => None,
        }
    }
    pub fn resolve_restore(req: &S3Request) -> Option<S3OpKind> {
        match req.method {
            Method::POST => Some(S3OpKind::RestoreObject),
            _ => None,
        }
    }
    pub fn resolve_retention(req: &S3Request) -> Option<S3OpKind> {
        match req.method {
            Method::GET => Some(S3OpKind::GetObjectRetention),
            Method::PUT => Some(S3OpKind::PutObjectRetention),
            _ => None,
        }
    }
    pub fn resolve_select_object_content(req: &S3Request) -> Option<S3OpKind> {
        match req.method {
            Method::POST => Some(S3OpKind::SelectObjectContent),
            _ => None,
        }
    }
    pub fn resolve_tagging(req: &S3Request) -> Option<S3OpKind> {
        match req.method {
            Method::GET => Some(S3OpKind::GetObjectTagging),
            Method::PUT => Some(S3OpKind::PutObjectTagging),
            Method::DELETE => Some(S3OpKind::DeleteObjectTagging),
            _ => None,
        }
    }
    pub fn resolve_torrent(req: &S3Request) -> Option<S3OpKind> {
        match req.method {
            Method::GET => Some(S3OpKind::GetObjectTorrent),
            _ => None,
        }
    }
    pub fn resolve_uploads(req: &S3Request) -> Option<S3OpKind> {
        match req.method {
            Method::GET => Some(S3OpKind::ListMultipartUploads),
            Method::POST => Some(S3OpKind::CreateMultipartUpload),
            _ => None,
        }
    }
    pub fn resolve_upload_id(req: &S3Request) -> Option<S3OpKind> {
        match req.method {
            Method::GET => Some(S3OpKind::ListParts),
            Method::PUT => {
                if req.has_header(X_AMZ_COPY_SOURCE) {
                    Some(S3OpKind::UploadPartCopy)
                } else {
                    Some(S3OpKind::UploadPart)
                }
            }
            Method::POST => Some(S3OpKind::CompleteMultipartUpload),
            Method::DELETE => Some(S3OpKind::AbortMultipartUpload),
            _ => None,
        }
    }
    pub fn resolve_versions(req: &S3Request) -> Option<S3OpKind> {
        match req.method {
            Method::GET => Some(S3OpKind::ListObjectVersions),
            _ => None,
        }
    }
}
