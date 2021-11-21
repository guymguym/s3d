//! TODO This module should be generated from https://github.com/awslabs/smithy-rs

use crate::{gen::*, types::*};
use hyper::Method;

pub fn match_op(req: &S3Request) -> Option<S3OpKind> {
    match &req.resource {
        S3Resource::Service => match_service(req),
        S3Resource::Bucket(b) => match b.sub_resource {
            S3BucketSubResource::None => match_bucket(req),
            S3BucketSubResource::Accelerate => match_bucket_accelerate(req),
            S3BucketSubResource::Acl => match_bucket_acl(req),
            S3BucketSubResource::Analytics => match_bucket_analytics(req),
            S3BucketSubResource::Cors => match_bucket_cors(req),
            S3BucketSubResource::Encryption => match_bucket_encryption(req),
            S3BucketSubResource::IntelligentTiering => match_bucket_intelligent_tiering(req),
            S3BucketSubResource::Inventory => match_bucket_inventory(req),
            S3BucketSubResource::Lifecycle => match_bucket_lifecycle(req),
            S3BucketSubResource::Location => match_bucket_location(req),
            S3BucketSubResource::Logging => match_bucket_logging(req),
            S3BucketSubResource::Metrics => match_bucket_metrics(req),
            S3BucketSubResource::Notification => match_bucket_notification(req),
            S3BucketSubResource::OwnershipControls => match_bucket_ownership_controls(req),
            S3BucketSubResource::Policy => match_bucket_policy(req),
            S3BucketSubResource::PolicyStatus => match_bucket_policy_status(req),
            S3BucketSubResource::PublicAccessBlock => match_bucket_public_access_block(req),
            S3BucketSubResource::Replication => match_bucket_replication(req),
            S3BucketSubResource::RequestPayment => match_bucket_request_payment(req),
            S3BucketSubResource::Tagging => match_bucket_tagging(req),
            S3BucketSubResource::Versioning => match_bucket_versioning(req),
            S3BucketSubResource::Website => match_bucket_website(req),
        },
        S3Resource::Object(o) => match o.sub_resource {
            S3ObjectSubResource::None => match_object(req),
            S3ObjectSubResource::Acl => match_object_acl(req),
            S3ObjectSubResource::LegalHold => match_object_legal_hold(req),
            S3ObjectSubResource::ObjectLock => match_object_object_lock(req),
            S3ObjectSubResource::Restore => match_object_restore(req),
            S3ObjectSubResource::Retention => match_object_retention(req),
            S3ObjectSubResource::SelectObjectContent => match_object_select_object_content(req),
            S3ObjectSubResource::Tagging => match_object_tagging(req),
            S3ObjectSubResource::Torrent => match_object_torrent(req),
            S3ObjectSubResource::Uploads => match_object_uploads(req),
            S3ObjectSubResource::UploadId => match_object_upload_id(req),
            S3ObjectSubResource::Versions => match_object_versions(req),
        },
    }
}

pub fn match_service(req: &S3Request) -> Option<S3OpKind> {
    match req.method {
        Method::GET => Some(S3OpKind::ListBuckets),
        _ => None,
    }
}

///////////////////////////////////////////////////////////////////////////////
// Bucket                                                                    //
///////////////////////////////////////////////////////////////////////////////

pub fn match_bucket(req: &S3Request) -> Option<S3OpKind> {
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
pub fn match_bucket_accelerate(req: &S3Request) -> Option<S3OpKind> {
    match req.method {
        Method::GET => Some(S3OpKind::GetBucketAccelerateConfiguration),
        Method::PUT => Some(S3OpKind::PutBucketAccelerateConfiguration),
        _ => None,
    }
}
pub fn match_bucket_acl(req: &S3Request) -> Option<S3OpKind> {
    match req.method {
        Method::GET => Some(S3OpKind::GetBucketAcl),
        Method::PUT => Some(S3OpKind::PutBucketAcl),
        _ => None,
    }
}
pub fn match_bucket_analytics(req: &S3Request) -> Option<S3OpKind> {
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
pub fn match_bucket_cors(req: &S3Request) -> Option<S3OpKind> {
    match req.method {
        Method::GET => Some(S3OpKind::GetBucketCors),
        Method::PUT => Some(S3OpKind::PutBucketCors),
        Method::DELETE => Some(S3OpKind::DeleteBucketCors),
        _ => None,
    }
}
pub fn match_bucket_encryption(req: &S3Request) -> Option<S3OpKind> {
    match req.method {
        Method::GET => Some(S3OpKind::GetBucketEncryption),
        Method::PUT => Some(S3OpKind::PutBucketEncryption),
        Method::DELETE => Some(S3OpKind::DeleteBucketEncryption),
        _ => None,
    }
}
pub fn match_bucket_intelligent_tiering(req: &S3Request) -> Option<S3OpKind> {
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
pub fn match_bucket_inventory(req: &S3Request) -> Option<S3OpKind> {
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
pub fn match_bucket_lifecycle(req: &S3Request) -> Option<S3OpKind> {
    match req.method {
        Method::GET => Some(S3OpKind::GetBucketLifecycleConfiguration),
        Method::PUT => Some(S3OpKind::PutBucketLifecycleConfiguration),
        Method::DELETE => Some(S3OpKind::DeleteBucketLifecycle),
        _ => None,
    }
}
pub fn match_bucket_location(req: &S3Request) -> Option<S3OpKind> {
    match req.method {
        Method::GET => Some(S3OpKind::GetBucketLocation),
        _ => None,
    }
}
pub fn match_bucket_logging(req: &S3Request) -> Option<S3OpKind> {
    match req.method {
        Method::GET => Some(S3OpKind::GetBucketLogging),
        Method::PUT => Some(S3OpKind::PutBucketLogging),
        _ => None,
    }
}
pub fn match_bucket_metrics(req: &S3Request) -> Option<S3OpKind> {
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
pub fn match_bucket_notification(req: &S3Request) -> Option<S3OpKind> {
    match req.method {
        Method::GET => Some(S3OpKind::GetBucketNotificationConfiguration),
        Method::PUT => Some(S3OpKind::PutBucketNotificationConfiguration),
        _ => None,
    }
}
pub fn match_bucket_ownership_controls(req: &S3Request) -> Option<S3OpKind> {
    match req.method {
        Method::GET => Some(S3OpKind::GetBucketOwnershipControls),
        Method::PUT => Some(S3OpKind::PutBucketOwnershipControls),
        Method::DELETE => Some(S3OpKind::DeleteBucketOwnershipControls),
        _ => None,
    }
}
pub fn match_bucket_policy(req: &S3Request) -> Option<S3OpKind> {
    match req.method {
        Method::GET => Some(S3OpKind::GetBucketPolicy),
        Method::PUT => Some(S3OpKind::PutBucketPolicy),
        Method::DELETE => Some(S3OpKind::DeleteBucketPolicy),
        _ => None,
    }
}
pub fn match_bucket_policy_status(req: &S3Request) -> Option<S3OpKind> {
    match req.method {
        Method::GET => Some(S3OpKind::GetBucketPolicyStatus),
        _ => None,
    }
}
pub fn match_bucket_public_access_block(req: &S3Request) -> Option<S3OpKind> {
    match req.method {
        Method::GET => Some(S3OpKind::GetPublicAccessBlock),
        Method::PUT => Some(S3OpKind::PutPublicAccessBlock),
        Method::DELETE => Some(S3OpKind::DeletePublicAccessBlock),
        _ => None,
    }
}
pub fn match_bucket_replication(req: &S3Request) -> Option<S3OpKind> {
    match req.method {
        Method::GET => Some(S3OpKind::GetBucketReplication),
        Method::PUT => Some(S3OpKind::PutBucketReplication),
        Method::DELETE => Some(S3OpKind::DeleteBucketReplication),
        _ => None,
    }
}
pub fn match_bucket_request_payment(req: &S3Request) -> Option<S3OpKind> {
    match req.method {
        Method::GET => Some(S3OpKind::GetBucketRequestPayment),
        Method::PUT => Some(S3OpKind::PutBucketRequestPayment),
        _ => None,
    }
}
pub fn match_bucket_tagging(req: &S3Request) -> Option<S3OpKind> {
    match req.method {
        Method::GET => Some(S3OpKind::GetBucketTagging),
        Method::PUT => Some(S3OpKind::PutBucketTagging),
        Method::DELETE => Some(S3OpKind::DeleteBucketTagging),
        _ => None,
    }
}
pub fn match_bucket_versioning(req: &S3Request) -> Option<S3OpKind> {
    match req.method {
        Method::GET => Some(S3OpKind::GetBucketVersioning),
        Method::PUT => Some(S3OpKind::PutBucketVersioning),
        _ => None,
    }
}
pub fn match_bucket_website(req: &S3Request) -> Option<S3OpKind> {
    match req.method {
        Method::GET => Some(S3OpKind::GetBucketWebsite),
        Method::PUT => Some(S3OpKind::PutBucketWebsite),
        Method::DELETE => Some(S3OpKind::DeleteBucketWebsite),
        _ => None,
    }
}

///////////////////////////////////////////////////////////////////////////////
// Object                                                                    //
///////////////////////////////////////////////////////////////////////////////

pub fn match_object(req: &S3Request) -> Option<S3OpKind> {
    match req.method {
        Method::GET => Some(S3OpKind::GetObject),
        Method::HEAD => Some(S3OpKind::HeadObject),
        Method::PUT => {
            if req.has_header("x-amz-copy-source") {
                Some(S3OpKind::CopyObject)
            } else {
                Some(S3OpKind::PutObject)
            }
        }
        Method::DELETE => Some(S3OpKind::DeleteObject),
        _ => None,
    }
}
pub fn match_object_acl(req: &S3Request) -> Option<S3OpKind> {
    match req.method {
        Method::GET => Some(S3OpKind::GetObjectAcl),
        Method::PUT => Some(S3OpKind::PutObjectAcl),
        _ => None,
    }
}
pub fn match_object_legal_hold(req: &S3Request) -> Option<S3OpKind> {
    match req.method {
        Method::GET => Some(S3OpKind::GetObjectLegalHold),
        Method::PUT => Some(S3OpKind::PutObjectLegalHold),
        _ => None,
    }
}
pub fn match_object_object_lock(req: &S3Request) -> Option<S3OpKind> {
    match req.method {
        Method::GET => Some(S3OpKind::GetObjectLockConfiguration),
        Method::PUT => Some(S3OpKind::PutObjectLockConfiguration),
        _ => None,
    }
}
pub fn match_object_restore(req: &S3Request) -> Option<S3OpKind> {
    match req.method {
        Method::POST => Some(S3OpKind::RestoreObject),
        _ => None,
    }
}
pub fn match_object_retention(req: &S3Request) -> Option<S3OpKind> {
    match req.method {
        Method::GET => Some(S3OpKind::GetObjectRetention),
        Method::PUT => Some(S3OpKind::PutObjectRetention),
        _ => None,
    }
}
pub fn match_object_select_object_content(req: &S3Request) -> Option<S3OpKind> {
    match req.method {
        Method::POST => Some(S3OpKind::SelectObjectContent),
        _ => None,
    }
}
pub fn match_object_tagging(req: &S3Request) -> Option<S3OpKind> {
    match req.method {
        Method::GET => Some(S3OpKind::GetObjectTagging),
        Method::PUT => Some(S3OpKind::PutObjectTagging),
        Method::DELETE => Some(S3OpKind::DeleteObjectTagging),
        _ => None,
    }
}
pub fn match_object_torrent(req: &S3Request) -> Option<S3OpKind> {
    match req.method {
        Method::GET => Some(S3OpKind::GetObjectTorrent),
        _ => None,
    }
}
pub fn match_object_uploads(req: &S3Request) -> Option<S3OpKind> {
    match req.method {
        Method::GET => Some(S3OpKind::ListMultipartUploads),
        Method::POST => Some(S3OpKind::CreateMultipartUpload),
        _ => None,
    }
}
pub fn match_object_upload_id(req: &S3Request) -> Option<S3OpKind> {
    match req.method {
        Method::GET => Some(S3OpKind::ListParts),
        Method::PUT => {
            if req.has_header("x-amz-copy-source") {
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
pub fn match_object_versions(req: &S3Request) -> Option<S3OpKind> {
    match req.method {
        Method::GET => Some(S3OpKind::ListObjectVersions),
        _ => None,
    }
}
