//! TODO This module should be generated from https://github.com/awslabs/smithy-rs

use crate::{gen::constants::*, types::S3Request};
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

    pub fn list_buckets(_: &S3Request) -> Result<ListBucketsInput, InputError> {
        ListBucketsInput::builder().build().map_err(|e| e.into())
    }

    pub fn head_bucket(req: &S3Request) -> Result<HeadBucketInput, InputError> {
        HeadBucketInput::builder()
            .bucket(req.get_bucket())
            .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
            .build()
            .map_err(|e| e.into())
    }

    pub fn create_bucket(req: &S3Request) -> Result<CreateBucketInput, InputError> {
        CreateBucketInput::builder()
            .bucket(req.get_bucket())
            .set_acl(req.get_header_parse(X_AMZ_ACL))
            // .set_create_bucket_configuration(req.get_body_parse(CreateBucketConfiguration))
            .set_grant_full_control(req.get_header(X_AMZ_GRANT_FULL_CONTROL))
            .set_grant_read(req.get_header(X_AMZ_GRANT_READ))
            .set_grant_read_acp(req.get_header(X_AMZ_GRANT_READ_ACP))
            .set_grant_write(req.get_header(X_AMZ_GRANT_WRITE))
            .set_grant_write_acp(req.get_header(X_AMZ_GRANT_WRITE_ACP))
            .set_object_lock_enabled_for_bucket(
                req.get_header_parse(X_AMZ_BUCKET_OBJECT_LOCK_ENABLED),
            )
            .build()
            .map_err(|e| e.into())
    }

    pub fn delete_bucket(req: &S3Request) -> Result<DeleteBucketInput, InputError> {
        DeleteBucketInput::builder()
            .bucket(req.get_bucket())
            .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
            .build()
            .map_err(|e| e.into())
    }

    pub fn get_bucket_location(req: &S3Request) -> Result<GetBucketLocationInput, InputError> {
        GetBucketLocationInput::builder()
            .bucket(req.get_bucket())
            .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
            .build()
            .map_err(|e| e.into())
    }

    pub fn list_objects(req: &S3Request) -> Result<ListObjectsInput, InputError> {
        ListObjectsInput::builder()
            .bucket(req.get_bucket())
            .set_delimiter(req.get_param(P_DELIMITER))
            .set_encoding_type(req.get_param_parse(P_ENCODING_TYPE))
            .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
            .set_marker(req.get_param(P_MARKER))
            .set_max_keys(req.get_param_parse(P_MAX_KEYS))
            .set_prefix(req.get_param(P_PREFIX))
            .set_request_payer(req.get_header_parse(X_AMZ_REQUEST_PAYER))
            .build()
            .map_err(|e| e.into())
    }

    pub fn get_object(req: &S3Request) -> Result<GetObjectInput, InputError> {
        GetObjectInput::builder()
            .bucket(req.get_bucket())
            .key(req.get_key())
            .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
            .set_if_match(req.get_header(H_IF_MATCH))
            .set_if_none_match(req.get_header(H_IF_NONE_MATCH))
            .set_if_modified_since(req.get_header_date(H_IF_MODIFIED_SINCE))
            .set_if_unmodified_since(req.get_header_date(H_IF_UNMODIFIED_SINCE))
            .set_part_number(req.get_param_parse(P_PART_NUMBER))
            .set_range(req.get_header(H_RANGE))
            .set_request_payer(req.get_header_parse(X_AMZ_REQUEST_PAYER))
            .set_response_cache_control(req.get_param(P_RESPONSE_CACHE_CONTROL))
            .set_response_content_disposition(req.get_param(P_RESPONSE_CONTENT_DISPOSITION))
            .set_response_content_encoding(req.get_param(P_RESPONSE_CONTENT_ENCODING))
            .set_response_content_language(req.get_param(P_RESPONSE_CONTENT_LANGUAGE))
            .set_response_content_type(req.get_param(P_RESPONSE_CONTENT_TYPE))
            .set_response_expires(req.get_param_date(P_RESPONSE_EXPIRES))
            .set_sse_customer_algorithm(req.get_header(X_AMZ_SSE_CUSTOMER_ALG))
            .set_sse_customer_key_md5(req.get_header(X_AMZ_SSE_CUSTOMER_KEY_MD5))
            .set_sse_customer_key(req.get_header(X_AMZ_SSE_AWS_KMS_KEY_ID))
            .set_version_id(req.get_param(P_VERSION_ID))
            .build()
            .map_err(|e| e.into())
    }

    /// This macro generates a default parser function per op.
    /// to make it possible to implement it in stages.
    macro_rules! gen {
        ($name:ident) => {
            paste::paste! {
                pub fn [<$name:snake>](_: &S3Request) -> Result<[<$name Input>], InputError> {
                    Err(InputError::NotImplemented(stringify!(s3::input::parsers::[<$name:snake>])))
                }
            }
        };
    }

    // basic bucket ops
    // gen!(ListBuckets);
    // gen!(HeadBucket);
    // gen!(CreateBucket);
    // gen!(DeleteBucket);
    // gen!(GetBucketLocation);
    gen!(GetBucketAcl);
    gen!(PutBucketAcl);
    gen!(GetBucketTagging);
    gen!(PutBucketTagging);
    gen!(DeleteBucketTagging);

    // basic list ops
    // gen!(ListObjects);
    gen!(ListObjectsV2);
    gen!(ListObjectVersions);

    // basic object ops
    // gen!(GetObject);
    gen!(HeadObject);
    gen!(PutObject);
    gen!(CopyObject);
    gen!(DeleteObject);
    gen!(DeleteObjects);
    gen!(GetObjectAcl);
    gen!(PutObjectAcl);
    gen!(GetObjectTagging);
    gen!(PutObjectTagging);
    gen!(DeleteObjectTagging);

    // multipart upload
    gen!(ListMultipartUploads);
    gen!(CreateMultipartUpload);
    gen!(AbortMultipartUpload);
    gen!(CompleteMultipartUpload);
    gen!(ListParts);
    gen!(UploadPart);
    gen!(UploadPartCopy);

    // advanced object ops
    gen!(GetObjectLegalHold);
    gen!(GetObjectLockConfiguration);
    gen!(GetObjectRetention);
    gen!(GetObjectTorrent);
    gen!(PutObjectLegalHold);
    gen!(PutObjectLockConfiguration);
    gen!(PutObjectRetention);
    gen!(RestoreObject);
    gen!(SelectObjectContent);
    gen!(WriteGetObjectResponse);

    // advanced bucket ops
    gen!(GetBucketAccelerateConfiguration);
    gen!(GetBucketAnalyticsConfiguration);
    gen!(GetBucketCors);
    gen!(GetBucketEncryption);
    gen!(GetBucketIntelligentTieringConfiguration);
    gen!(GetBucketInventoryConfiguration);
    gen!(GetBucketLifecycleConfiguration);
    gen!(GetBucketLogging);
    gen!(GetBucketMetricsConfiguration);
    gen!(GetBucketNotificationConfiguration);
    gen!(GetBucketOwnershipControls);
    gen!(GetBucketPolicy);
    gen!(GetBucketPolicyStatus);
    gen!(GetBucketReplication);
    gen!(GetBucketRequestPayment);
    gen!(GetBucketVersioning);
    gen!(GetBucketWebsite);
    gen!(GetPublicAccessBlock);

    gen!(PutBucketAccelerateConfiguration);
    gen!(PutBucketAnalyticsConfiguration);
    gen!(PutBucketCors);
    gen!(PutBucketEncryption);
    gen!(PutBucketIntelligentTieringConfiguration);
    gen!(PutBucketInventoryConfiguration);
    gen!(PutBucketLifecycleConfiguration);
    gen!(PutBucketLogging);
    gen!(PutBucketMetricsConfiguration);
    gen!(PutBucketNotificationConfiguration);
    gen!(PutBucketOwnershipControls);
    gen!(PutBucketPolicy);
    gen!(PutBucketReplication);
    gen!(PutBucketRequestPayment);
    gen!(PutBucketVersioning);
    gen!(PutBucketWebsite);
    gen!(PutPublicAccessBlock);

    gen!(DeleteBucketAnalyticsConfiguration);
    gen!(DeleteBucketCors);
    gen!(DeleteBucketEncryption);
    gen!(DeleteBucketIntelligentTieringConfiguration);
    gen!(DeleteBucketInventoryConfiguration);
    gen!(DeleteBucketLifecycle);
    gen!(DeleteBucketMetricsConfiguration);
    gen!(DeleteBucketOwnershipControls);
    gen!(DeleteBucketPolicy);
    gen!(DeleteBucketReplication);
    gen!(DeleteBucketWebsite);
    gen!(DeletePublicAccessBlock);

    gen!(ListBucketAnalyticsConfigurations);
    gen!(ListBucketIntelligentTieringConfigurations);
    gen!(ListBucketInventoryConfigurations);
    gen!(ListBucketMetricsConfigurations);
}
