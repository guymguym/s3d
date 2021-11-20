//! TODO This module should be generated from https://github.com/awslabs/smithy-rs

use crate::types::S3Request;
use aws_sdk_s3::input::*;
use aws_smithy_http::operation::BuildError;
use std::fmt;

/// InputError are errors that can occur when parsing the input from the HTTP request
#[derive(Debug)]
pub enum InputError {
    NotImplemented(&'static str),
    BadRequest(BuildError),
    Unhandled(anyhow::Error),
}
impl std::error::Error for InputError {}
impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InputError::NotImplemented(msg) => write!(f, "NotImplemented({})", msg),
            InputError::BadRequest(err) => write!(f, "BadRequest({})", err),
            InputError::Unhandled(err) => write!(f, "Unhandled({})", err),
        }
    }
}
impl From<BuildError> for InputError {
    fn from(err: BuildError) -> Self {
        InputError::BadRequest(err)
    }
}
impl From<anyhow::Error> for InputError {
    fn from(err: anyhow::Error) -> Self {
        InputError::Unhandled(err)
    }
}

/// Implement a function per S3 operation to parse the input from the HTTP request.
/// To be exact - the methods take S3Request which is a wrapper around the HTTP request.
/// No reason to write it by hand - should be generated from smithy-rs.
pub mod parsers {
    use super::*;

    // we re-publish all the not_implemented methods as default and then override them
    // in this module to make it easier to implement it in stages.
    pub use super::not_implemented::*;

    pub fn list_buckets(_: &S3Request) -> Result<ListBucketsInput, InputError> {
        ListBucketsInput::builder().build().map_err(|e| e.into())
    }

    pub fn list_objects(req: &S3Request) -> Result<ListObjectsInput, InputError> {
        ListObjectsInput::builder()
            .set_bucket(Some(req.resource.get_bucket().to_owned()))
            .set_delimiter(req.get_param("delimiter"))
            .set_marker(req.get_param("marker"))
            .set_prefix(req.get_param("prefix"))
            .set_max_keys(req.get_param_i32("max-keys"))
            // .set_encoding_type(i.expected_bucket_owner)
            // .set_expected_bucket_owner(i.expected_bucket_owner)
            // .set_request_payer(i.request_payer)
            .build()
            .map_err(|e| e.into())
    }

    pub fn get_object(req: &S3Request) -> Result<GetObjectInput, InputError> {
        GetObjectInput::builder()
            .set_bucket(Some(req.resource.get_bucket().to_owned()))
            .set_key(Some(req.resource.get_key().to_owned()))
            .set_part_number(req.get_param_i32("partNumber"))
            .set_range(req.get_header("range"))
            .set_version_id(req.get_param("versionId"))
            .build()
            .map_err(|e| e.into())
    }
}

mod not_implemented {
    use super::*;

    /// This macro generates a default parser function per op.
    macro_rules! s3_inp {
        ($name:ident) => {
            paste::paste! {
                #[allow(dead_code)]
                pub fn [<$name:snake>](_: &S3Request) -> Result<[<$name Input>], InputError> {
                    Err(InputError::NotImplemented(stringify!(s3::input::parsers::[<$name:snake>])))
                }
            }
        };
    }

    s3_inp!(AbortMultipartUpload);
    s3_inp!(CompleteMultipartUpload);
    s3_inp!(CopyObject);
    s3_inp!(CreateBucket);
    s3_inp!(CreateMultipartUpload);
    s3_inp!(DeleteBucket);
    s3_inp!(DeleteBucketAnalyticsConfiguration);
    s3_inp!(DeleteBucketCors);
    s3_inp!(DeleteBucketEncryption);
    s3_inp!(DeleteBucketIntelligentTieringConfiguration);
    s3_inp!(DeleteBucketInventoryConfiguration);
    s3_inp!(DeleteBucketLifecycle);
    s3_inp!(DeleteBucketMetricsConfiguration);
    s3_inp!(DeleteBucketOwnershipControls);
    s3_inp!(DeleteBucketPolicy);
    s3_inp!(DeleteBucketReplication);
    s3_inp!(DeleteBucketTagging);
    s3_inp!(DeleteBucketWebsite);
    s3_inp!(DeleteObject);
    s3_inp!(DeleteObjects);
    s3_inp!(DeleteObjectTagging);
    s3_inp!(DeletePublicAccessBlock);
    s3_inp!(GetBucketAccelerateConfiguration);
    s3_inp!(GetBucketAcl);
    s3_inp!(GetBucketAnalyticsConfiguration);
    s3_inp!(GetBucketCors);
    s3_inp!(GetBucketEncryption);
    s3_inp!(GetBucketIntelligentTieringConfiguration);
    s3_inp!(GetBucketInventoryConfiguration);
    s3_inp!(GetBucketLifecycleConfiguration);
    s3_inp!(GetBucketLocation);
    s3_inp!(GetBucketLogging);
    s3_inp!(GetBucketMetricsConfiguration);
    s3_inp!(GetBucketNotificationConfiguration);
    s3_inp!(GetBucketOwnershipControls);
    s3_inp!(GetBucketPolicy);
    s3_inp!(GetBucketPolicyStatus);
    s3_inp!(GetBucketReplication);
    s3_inp!(GetBucketRequestPayment);
    s3_inp!(GetBucketTagging);
    s3_inp!(GetBucketVersioning);
    s3_inp!(GetBucketWebsite);
    s3_inp!(GetObject);
    s3_inp!(GetObjectAcl);
    s3_inp!(GetObjectLegalHold);
    s3_inp!(GetObjectLockConfiguration);
    s3_inp!(GetObjectRetention);
    s3_inp!(GetObjectTagging);
    s3_inp!(GetObjectTorrent);
    s3_inp!(GetPublicAccessBlock);
    s3_inp!(HeadBucket);
    s3_inp!(HeadObject);
    s3_inp!(ListBucketAnalyticsConfigurations);
    s3_inp!(ListBucketIntelligentTieringConfigurations);
    s3_inp!(ListBucketInventoryConfigurations);
    s3_inp!(ListBucketMetricsConfigurations);
    s3_inp!(ListBuckets);
    s3_inp!(ListMultipartUploads);
    s3_inp!(ListObjects);
    s3_inp!(ListObjectsV2);
    s3_inp!(ListObjectVersions);
    s3_inp!(ListParts);
    s3_inp!(PutBucketAccelerateConfiguration);
    s3_inp!(PutBucketAcl);
    s3_inp!(PutBucketAnalyticsConfiguration);
    s3_inp!(PutBucketCors);
    s3_inp!(PutBucketEncryption);
    s3_inp!(PutBucketIntelligentTieringConfiguration);
    s3_inp!(PutBucketInventoryConfiguration);
    s3_inp!(PutBucketLifecycleConfiguration);
    s3_inp!(PutBucketLogging);
    s3_inp!(PutBucketMetricsConfiguration);
    s3_inp!(PutBucketNotificationConfiguration);
    s3_inp!(PutBucketOwnershipControls);
    s3_inp!(PutBucketPolicy);
    s3_inp!(PutBucketReplication);
    s3_inp!(PutBucketRequestPayment);
    s3_inp!(PutBucketTagging);
    s3_inp!(PutBucketVersioning);
    s3_inp!(PutBucketWebsite);
    s3_inp!(PutObject);
    s3_inp!(PutObjectAcl);
    s3_inp!(PutObjectLegalHold);
    s3_inp!(PutObjectLockConfiguration);
    s3_inp!(PutObjectRetention);
    s3_inp!(PutObjectTagging);
    s3_inp!(PutPublicAccessBlock);
    s3_inp!(RestoreObject);
    s3_inp!(SelectObjectContent);
    s3_inp!(UploadPart);
    s3_inp!(UploadPartCopy);
    s3_inp!(WriteGetObjectResponse);
}
