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

    //---------------------------------------------------//
    // basic bucket ops                                  //
    //---------------------------------------------------//

    //-------------------------------//
    // gen!(HeadBucket);
    // gen!(CreateBucket);
    // gen!(DeleteBucket);
    // gen!(GetBucketLocation);
    //-------------------------------//

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
            // .set_create_bucket_configuration(req.get_body_parse("CreateBucketConfiguration"))
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

    //---------------------------------------------------//
    // basic list ops                                    //
    //---------------------------------------------------//

    //-------------------------------//
    // gen!(ListBuckets);
    // gen!(ListObjects);
    // gen!(ListObjectsV2);
    // gen!(ListObjectVersions);
    //-------------------------------//

    pub fn list_buckets(_: &S3Request) -> Result<ListBucketsInput, InputError> {
        ListBucketsInput::builder().build().map_err(|e| e.into())
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

    pub fn list_objects_v2(req: &S3Request) -> Result<ListObjectsV2Input, InputError> {
        ListObjectsV2Input::builder()
            .bucket(req.get_bucket())
            .set_continuation_token(req.get_param(P_CONTINUATION_TOKEN))
            .set_delimiter(req.get_param(P_DELIMITER))
            .set_encoding_type(req.get_param_parse(P_ENCODING_TYPE))
            .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
            .set_fetch_owner(req.get_param_parse(P_FETCH_OWNER))
            .set_max_keys(req.get_param_parse(P_MAX_KEYS))
            .set_prefix(req.get_param(P_PREFIX))
            .set_request_payer(req.get_header_parse(X_AMZ_REQUEST_PAYER))
            .set_start_after(req.get_param(P_START_AFTER))
            .build()
            .map_err(|e| e.into())
    }

    pub fn list_object_versions(req: &S3Request) -> Result<ListObjectVersionsInput, InputError> {
        ListObjectVersionsInput::builder()
            .bucket(req.get_bucket())
            .set_delimiter(req.get_param(P_DELIMITER))
            .set_encoding_type(req.get_param_parse(P_ENCODING_TYPE))
            .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
            .set_key_marker(req.get_param(P_KEY_MARKER))
            .set_max_keys(req.get_param_parse(P_MAX_KEYS))
            .set_prefix(req.get_param(P_PREFIX))
            .set_version_id_marker(req.get_param(P_VERSION_ID_MARKER))
            .build()
            .map_err(|e| e.into())
    }

    //---------------------------------------------------//
    // basic object ops                                  //
    //---------------------------------------------------//

    //-------------------------------//
    // gen!(GetObject);
    // gen!(HeadObject);
    // gen!(PutObject);
    // gen!(CopyObject);
    // gen!(DeleteObject);
    // gen!(DeleteObjects);
    //-------------------------------//

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

    pub fn head_object(req: &S3Request) -> Result<HeadObjectInput, InputError> {
        HeadObjectInput::builder()
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
            .set_sse_customer_algorithm(req.get_header(X_AMZ_SSE_CUSTOMER_ALG))
            .set_sse_customer_key_md5(req.get_header(X_AMZ_SSE_CUSTOMER_KEY_MD5))
            .set_sse_customer_key(req.get_header(X_AMZ_SSE_AWS_KMS_KEY_ID))
            .set_version_id(req.get_param(P_VERSION_ID))
            .build()
            .map_err(|e| e.into())
    }

    pub fn put_object(req: &S3Request) -> Result<PutObjectInput, InputError> {
        PutObjectInput::builder()
            .bucket(req.get_bucket())
            .key(req.get_key())
            .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
            .set_request_payer(req.get_header_parse(X_AMZ_REQUEST_PAYER))
            .set_sse_customer_algorithm(req.get_header(X_AMZ_SSE_CUSTOMER_ALG))
            .set_sse_customer_key_md5(req.get_header(X_AMZ_SSE_CUSTOMER_KEY_MD5))
            .set_sse_customer_key(req.get_header(X_AMZ_SSE_AWS_KMS_KEY_ID))
            .build()
            .map_err(|e| e.into())
    }

    pub fn copy_object(req: &S3Request) -> Result<CopyObjectInput, InputError> {
        CopyObjectInput::builder()
            .bucket(req.get_bucket())
            .key(req.get_key())
            .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
            .set_request_payer(req.get_header_parse(X_AMZ_REQUEST_PAYER))
            .set_sse_customer_algorithm(req.get_header(X_AMZ_SSE_CUSTOMER_ALG))
            .set_sse_customer_key_md5(req.get_header(X_AMZ_SSE_CUSTOMER_KEY_MD5))
            .set_sse_customer_key(req.get_header(X_AMZ_SSE_AWS_KMS_KEY_ID))
            .build()
            .map_err(|e| e.into())
    }

    pub fn delete_object(req: &S3Request) -> Result<DeleteObjectInput, InputError> {
        DeleteObjectInput::builder()
            .bucket(req.get_bucket())
            .key(req.get_key())
            .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
            .set_request_payer(req.get_header_parse(X_AMZ_REQUEST_PAYER))
            .build()
            .map_err(|e| e.into())
    }

    pub fn delete_objects(req: &S3Request) -> Result<DeleteObjectsInput, InputError> {
        DeleteObjectsInput::builder()
            .bucket(req.get_bucket())
            .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
            .set_request_payer(req.get_header_parse(X_AMZ_REQUEST_PAYER))
            .build()
            .map_err(|e| e.into())
    }

    //---------------------------------------------------//
    // multipart upload ops                              //
    //---------------------------------------------------//

    //-------------------------------//
    gen!(ListMultipartUploads);
    gen!(CreateMultipartUpload);
    gen!(AbortMultipartUpload);
    gen!(CompleteMultipartUpload);
    gen!(ListParts);
    gen!(UploadPart);
    gen!(UploadPartCopy);
    //-------------------------------//

    //---------------------------------------------------//
    // advanced object ops                               //
    //---------------------------------------------------//

    //-------------------------------//
    // gen!(GetObjectAcl);
    // gen!(PutObjectAcl);
    // gen!(GetObjectTagging);
    // gen!(PutObjectTagging);
    // gen!(DeleteObjectTagging);
    //-------------------------------//
    gen!(GetObjectRetention);
    gen!(PutObjectRetention);
    gen!(GetObjectLegalHold);
    gen!(PutObjectLegalHold);
    gen!(GetObjectLockConfiguration);
    gen!(PutObjectLockConfiguration);
    //-------------------------------//
    gen!(RestoreObject);
    gen!(GetObjectTorrent);
    gen!(SelectObjectContent);
    gen!(WriteGetObjectResponse);
    //-------------------------------//

    pub fn get_object_acl(req: &S3Request) -> Result<GetObjectAclInput, InputError> {
        GetObjectAclInput::builder()
            .bucket(req.get_bucket())
            .key(req.get_key())
            .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
            .set_request_payer(req.get_header_parse(X_AMZ_REQUEST_PAYER))
            .set_version_id(req.get_param(P_VERSION_ID))
            .build()
            .map_err(|e| e.into())
    }

    pub fn put_object_acl(req: &S3Request) -> Result<PutObjectAclInput, InputError> {
        PutObjectAclInput::builder()
            .bucket(req.get_bucket())
            .key(req.get_key())
            // .set_access_control_policy(req.get_body_parse("AccessControlPolicy")) // TODO parse xml body
            .set_acl(req.get_header_parse(X_AMZ_ACL))
            .set_content_md5(req.get_header(H_CONTENT_MD5))
            .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
            .set_grant_full_control(req.get_header(X_AMZ_GRANT_FULL_CONTROL))
            .set_grant_read(req.get_header(X_AMZ_GRANT_READ))
            .set_grant_read_acp(req.get_header(X_AMZ_GRANT_READ_ACP))
            .set_grant_write(req.get_header(X_AMZ_GRANT_WRITE))
            .set_grant_write_acp(req.get_header(X_AMZ_GRANT_WRITE_ACP))
            .set_request_payer(req.get_header_parse(X_AMZ_REQUEST_PAYER))
            .set_version_id(req.get_param(P_VERSION_ID))
            .build()
            .map_err(|e| e.into())
    }

    pub fn get_object_tagging(req: &S3Request) -> Result<GetObjectTaggingInput, InputError> {
        GetObjectTaggingInput::builder()
            .bucket(req.get_bucket())
            .key(req.get_key())
            .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
            .set_request_payer(req.get_header_parse(X_AMZ_REQUEST_PAYER))
            .set_version_id(req.get_param(P_VERSION_ID))
            .build()
            .map_err(|e| e.into())
    }

    pub fn put_object_tagging(req: &S3Request) -> Result<PutObjectTaggingInput, InputError> {
        PutObjectTaggingInput::builder()
            .bucket(req.get_bucket())
            .key(req.get_key())
            .set_content_md5(req.get_header(H_CONTENT_MD5))
            .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
            // .set_tagging(req.get_body_parse("Tagging")) // TODO parse xml body
            .set_request_payer(req.get_header_parse(X_AMZ_REQUEST_PAYER))
            .set_version_id(req.get_param(P_VERSION_ID))
            .build()
            .map_err(|e| e.into())
    }

    pub fn delete_object_tagging(req: &S3Request) -> Result<DeleteObjectTaggingInput, InputError> {
        DeleteObjectTaggingInput::builder()
            .bucket(req.get_bucket())
            .key(req.get_key())
            .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
            .set_version_id(req.get_param(P_VERSION_ID))
            .build()
            .map_err(|e| e.into())
    }

    //---------------------------------------------------//
    // advanced bucket ops                               //
    //---------------------------------------------------//

    //-------------------------------//
    // gen!(GetBucketAcl);
    // gen!(PutBucketAcl);
    // gen!(GetBucketTagging);
    // gen!(PutBucketTagging);
    // gen!(DeleteBucketTagging);
    //-------------------------------//
    gen!(GetBucketCors);
    gen!(PutBucketCors);
    gen!(DeleteBucketCors);
    //-------------------------------//
    gen!(GetBucketPolicy);
    gen!(PutBucketPolicy);
    gen!(DeleteBucketPolicy);
    gen!(GetBucketPolicyStatus);
    //-------------------------------//
    gen!(GetBucketReplication);
    gen!(PutBucketReplication);
    gen!(DeleteBucketReplication);
    //-------------------------------//
    gen!(GetBucketEncryption);
    gen!(PutBucketEncryption);
    gen!(DeleteBucketEncryption);
    //-------------------------------//
    gen!(GetBucketWebsite);
    gen!(PutBucketWebsite);
    gen!(DeleteBucketWebsite);
    //-------------------------------//
    gen!(GetBucketLifecycleConfiguration);
    gen!(PutBucketLifecycleConfiguration);
    gen!(DeleteBucketLifecycle);
    //-------------------------------//
    gen!(GetBucketLogging);
    gen!(PutBucketLogging);
    gen!(GetBucketVersioning);
    gen!(PutBucketVersioning);
    gen!(GetBucketRequestPayment);
    gen!(PutBucketRequestPayment);
    gen!(GetBucketAccelerateConfiguration);
    gen!(PutBucketAccelerateConfiguration);
    gen!(GetBucketNotificationConfiguration);
    gen!(PutBucketNotificationConfiguration);
    //-------------------------------//
    gen!(GetBucketAnalyticsConfiguration);
    gen!(PutBucketAnalyticsConfiguration);
    gen!(DeleteBucketAnalyticsConfiguration);
    gen!(ListBucketAnalyticsConfigurations);
    //-------------------------------//
    gen!(GetBucketIntelligentTieringConfiguration);
    gen!(PutBucketIntelligentTieringConfiguration);
    gen!(DeleteBucketIntelligentTieringConfiguration);
    gen!(ListBucketIntelligentTieringConfigurations);
    //-------------------------------//
    gen!(GetBucketInventoryConfiguration);
    gen!(PutBucketInventoryConfiguration);
    gen!(DeleteBucketInventoryConfiguration);
    gen!(ListBucketInventoryConfigurations);
    //-------------------------------//
    gen!(GetBucketMetricsConfiguration);
    gen!(PutBucketMetricsConfiguration);
    gen!(DeleteBucketMetricsConfiguration);
    gen!(ListBucketMetricsConfigurations);
    //-------------------------------//
    gen!(GetBucketOwnershipControls);
    gen!(PutBucketOwnershipControls);
    gen!(DeleteBucketOwnershipControls);
    //-------------------------------//
    gen!(GetPublicAccessBlock);
    gen!(PutPublicAccessBlock);
    gen!(DeletePublicAccessBlock);
    //-------------------------------//

    pub fn get_bucket_acl(req: &S3Request) -> Result<GetBucketAclInput, InputError> {
        GetBucketAclInput::builder()
            .bucket(req.get_bucket())
            .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
            .build()
            .map_err(|e| e.into())
    }

    pub fn put_bucket_acl(req: &S3Request) -> Result<PutBucketAclInput, InputError> {
        PutBucketAclInput::builder()
            .bucket(req.get_bucket())
            // .set_access_control_policy(req.get_body_parse("AccessControlPolicy")) // TODO parse xml body
            .set_acl(req.get_header_parse(X_AMZ_ACL))
            .set_content_md5(req.get_header(H_CONTENT_MD5))
            .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
            .set_grant_full_control(req.get_header(X_AMZ_GRANT_FULL_CONTROL))
            .set_grant_read(req.get_header(X_AMZ_GRANT_READ))
            .set_grant_read_acp(req.get_header(X_AMZ_GRANT_READ_ACP))
            .set_grant_write(req.get_header(X_AMZ_GRANT_WRITE))
            .set_grant_write_acp(req.get_header(X_AMZ_GRANT_WRITE_ACP))
            .build()
            .map_err(|e| e.into())
    }

    pub fn get_bucket_tagging(req: &S3Request) -> Result<GetBucketTaggingInput, InputError> {
        GetBucketTaggingInput::builder()
            .bucket(req.get_bucket())
            .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
            .build()
            .map_err(|e| e.into())
    }

    pub fn put_bucket_tagging(req: &S3Request) -> Result<PutBucketTaggingInput, InputError> {
        PutBucketTaggingInput::builder()
            .bucket(req.get_bucket())
            .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
            .set_content_md5(req.get_header(H_CONTENT_MD5))
            // .set_tagging(req.get_body_parse("Tagging")) // TODO parse xml body
            .build()
            .map_err(|e| e.into())
    }

    pub fn delete_bucket_tagging(req: &S3Request) -> Result<DeleteBucketTaggingInput, InputError> {
        DeleteBucketTaggingInput::builder()
            .bucket(req.get_bucket())
            .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
            .build()
            .map_err(|e| e.into())
    }
}
