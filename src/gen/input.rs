//! This module implements a function per S3 op to parse the input from the HTTP request.
//! The functions take S3Request which is a wrapper around the HTTP request.
//! Currently written by hand which is difficult to maintain long term.
//! TODO This module should be generated from https://github.com/awslabs/smithy-rs.

use crate::{gen::headers::*, http::*};
use aws_sdk_s3::{input::*, ByteStream};

/// This macro generates a default parser function per op.
/// to make it possible to implement it in stages.
macro_rules! gen {
    ($op:ident) => {
        paste::paste! {
            pub fn [<$op:snake>](_: &mut S3Request) -> Result<[<$op Input>], S3Error> {
                Err(S3Error::builder().
                    code("NotImplemented")
                    .message(format!(
                        "Not implemented {}",
                        stringify!(s3::input::parsers::[<$op:snake>])
                    ))
                    .build()
                    .into())
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

pub fn head_bucket(req: &mut S3Request) -> Result<HeadBucketInput, S3Error> {
    HeadBucketInput::builder()
        .bucket(req.get_bucket())
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .build()
        .map_err(|e| req.from_build_err(e))
}

pub fn create_bucket(req: &mut S3Request) -> Result<CreateBucketInput, S3Error> {
    CreateBucketInput::builder()
        .bucket(req.get_bucket())
        .set_acl(req.get_header_parse(X_AMZ_ACL))
        // .set_create_bucket_configuration(req.get_body_parse("CreateBucketConfiguration"))
        .set_grant_full_control(req.get_header(X_AMZ_GRANT_FULL_CONTROL))
        .set_grant_read(req.get_header(X_AMZ_GRANT_READ))
        .set_grant_read_acp(req.get_header(X_AMZ_GRANT_READ_ACP))
        .set_grant_write(req.get_header(X_AMZ_GRANT_WRITE))
        .set_grant_write_acp(req.get_header(X_AMZ_GRANT_WRITE_ACP))
        .set_object_lock_enabled_for_bucket(req.get_header_parse(X_AMZ_BUCKET_OBJECT_LOCK_ENABLED))
        .build()
        .map_err(|e| req.from_build_err(e))
}

pub fn delete_bucket(req: &mut S3Request) -> Result<DeleteBucketInput, S3Error> {
    DeleteBucketInput::builder()
        .bucket(req.get_bucket())
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .build()
        .map_err(|e| req.from_build_err(e))
}

pub fn get_bucket_location(req: &mut S3Request) -> Result<GetBucketLocationInput, S3Error> {
    GetBucketLocationInput::builder()
        .bucket(req.get_bucket())
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .build()
        .map_err(|e| req.from_build_err(e))
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

pub fn get_object(req: &mut S3Request) -> Result<GetObjectInput, S3Error> {
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
        .set_sse_customer_key(req.get_header(X_AMZ_SSE_CUSTOMER_KEY))
        .set_sse_customer_key_md5(req.get_header(X_AMZ_SSE_CUSTOMER_KEY_MD5))
        .set_version_id(req.get_param(P_VERSION_ID))
        .build()
        .map_err(|e| req.from_build_err(e))
}

pub fn head_object(req: &mut S3Request) -> Result<HeadObjectInput, S3Error> {
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
        .set_sse_customer_key(req.get_header(X_AMZ_SSE_CUSTOMER_KEY))
        .set_sse_customer_key_md5(req.get_header(X_AMZ_SSE_CUSTOMER_KEY_MD5))
        .set_version_id(req.get_param(P_VERSION_ID))
        .build()
        .map_err(|e| req.from_build_err(e))
}

pub fn put_object(req: &mut S3Request) -> Result<PutObjectInput, S3Error> {
    PutObjectInput::builder()
        .bucket(req.get_bucket())
        .key(req.get_key())
        .set_acl(req.get_header_parse(X_AMZ_ACL))
        .set_bucket_key_enabled(req.get_header_parse(X_AMZ_SSE_BUCKET_KEY_ENABLED))
        .set_cache_control(req.get_header(H_CACHE_CONTROL))
        .set_content_disposition(req.get_header(H_CONTENT_DISPOSITION))
        .set_content_encoding(req.get_header(H_CONTENT_ENCODING))
        .set_content_language(req.get_header(H_CONTENT_LANGUAGE))
        .set_content_length(req.get_header_parse(H_CONTENT_LENGTH))
        .set_content_md5(req.get_header(H_CONTENT_MD5))
        .set_content_type(req.get_header(H_CONTENT_TYPE))
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_expires(req.get_header_date(H_EXPIRES))
        .set_grant_full_control(req.get_header(X_AMZ_GRANT_FULL_CONTROL))
        .set_grant_read(req.get_header(X_AMZ_GRANT_READ))
        .set_grant_read_acp(req.get_header(X_AMZ_GRANT_READ_ACP))
        .set_grant_write_acp(req.get_header(X_AMZ_GRANT_WRITE_ACP))
        .set_metadata(req.get_header_map(X_AMZ_META_PREFIX))
        .set_object_lock_legal_hold_status(req.get_header_parse(X_AMZ_OBJECT_LOCK_LEGAL_HOLD))
        .set_object_lock_mode(req.get_header_parse(X_AMZ_OBJECT_LOCK_MODE))
        .set_object_lock_retain_until_date(req.get_header_date(X_AMZ_OBJECT_LOCK_RETAIN_UNTIL_DATE))
        .set_request_payer(req.get_header_parse(X_AMZ_REQUEST_PAYER))
        .set_server_side_encryption(req.get_header_parse(X_AMZ_SSE))
        .set_sse_customer_algorithm(req.get_header(X_AMZ_SSE_CUSTOMER_ALG))
        .set_sse_customer_key(req.get_header(X_AMZ_SSE_CUSTOMER_KEY))
        .set_sse_customer_key_md5(req.get_header(X_AMZ_SSE_CUSTOMER_KEY_MD5))
        .set_ssekms_encryption_context(req.get_header(X_AMZ_SSE_CONTEXT))
        .set_ssekms_key_id(req.get_header(X_AMZ_SSE_AWS_KMS_KEY_ID))
        .set_storage_class(req.get_header_parse(X_AMZ_STORAGE_CLASS))
        .set_tagging(req.get_header(X_AMZ_TAGGING))
        .set_website_redirect_location(req.get_header(X_AMZ_WEBSITE_REDIRECT_LOCATION))
        .set_body(Some(ByteStream::new(req.take_body().into())))
        .build()
        .map_err(|e| req.from_build_err(e))
}

pub fn copy_object(req: &mut S3Request) -> Result<CopyObjectInput, S3Error> {
    CopyObjectInput::builder()
        .bucket(req.get_bucket())
        .key(req.get_key())
        .set_acl(req.get_header_parse(X_AMZ_ACL))
        .set_bucket_key_enabled(req.get_header_parse(X_AMZ_SSE_BUCKET_KEY_ENABLED))
        .set_cache_control(req.get_header(H_CACHE_CONTROL))
        .set_content_disposition(req.get_header(H_CONTENT_DISPOSITION))
        .set_content_encoding(req.get_header(H_CONTENT_ENCODING))
        .set_content_language(req.get_header(H_CONTENT_LANGUAGE))
        .set_content_type(req.get_header(H_CONTENT_TYPE))
        .set_copy_source(req.get_header(X_AMZ_COPY_SOURCE))
        .set_copy_source_if_match(req.get_header(X_AMZ_COPY_SOURCE_IF_MATCH))
        .set_copy_source_if_modified_since(req.get_header_date(X_AMZ_COPY_SOURCE_IF_MODIFIED_SINCE))
        .set_copy_source_if_none_match(req.get_header(X_AMZ_COPY_SOURCE_IF_NONE_MATCH))
        .set_copy_source_if_unmodified_since(
            req.get_header_date(X_AMZ_COPY_SOURCE_IF_UNMODIFIED_SINCE),
        )
        .set_copy_source_sse_customer_algorithm(req.get_header(X_AMZ_COPY_SOURCE_SSE_CUSTOMER_ALG))
        .set_copy_source_sse_customer_key(req.get_header(X_AMZ_COPY_SOURCE_SSE_CUSTOMER_KEY))
        .set_copy_source_sse_customer_key_md5(
            req.get_header(X_AMZ_COPY_SOURCE_SSE_CUSTOMER_KEY_MD5),
        )
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_expected_source_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_expires(req.get_header_date(H_EXPIRES))
        .set_grant_full_control(req.get_header(X_AMZ_GRANT_FULL_CONTROL))
        .set_grant_read(req.get_header(X_AMZ_GRANT_READ))
        .set_grant_read_acp(req.get_header(X_AMZ_GRANT_READ_ACP))
        .set_grant_write_acp(req.get_header(X_AMZ_GRANT_WRITE_ACP))
        .set_metadata(req.get_header_map(X_AMZ_META_PREFIX))
        .set_metadata_directive(req.get_header_parse(X_AMZ_METADATA_DIRECTIVE))
        .set_object_lock_legal_hold_status(req.get_header_parse(X_AMZ_OBJECT_LOCK_LEGAL_HOLD))
        .set_object_lock_mode(req.get_header_parse(X_AMZ_OBJECT_LOCK_MODE))
        .set_object_lock_retain_until_date(req.get_header_date(X_AMZ_OBJECT_LOCK_RETAIN_UNTIL_DATE))
        .set_request_payer(req.get_header_parse(X_AMZ_REQUEST_PAYER))
        .set_server_side_encryption(req.get_header_parse(X_AMZ_SSE))
        .set_sse_customer_algorithm(req.get_header(X_AMZ_SSE_CUSTOMER_ALG))
        .set_sse_customer_key(req.get_header(X_AMZ_SSE_CUSTOMER_KEY))
        .set_sse_customer_key_md5(req.get_header(X_AMZ_SSE_CUSTOMER_KEY_MD5))
        .set_ssekms_encryption_context(req.get_header(X_AMZ_SSE_CONTEXT))
        .set_ssekms_key_id(req.get_header(X_AMZ_SSE_AWS_KMS_KEY_ID))
        .set_storage_class(req.get_header_parse(X_AMZ_STORAGE_CLASS))
        .set_tagging(req.get_header(X_AMZ_TAGGING))
        .set_tagging_directive(req.get_header_parse(X_AMZ_TAGGING_DIRECTIVE))
        .set_website_redirect_location(req.get_header(X_AMZ_WEBSITE_REDIRECT_LOCATION))
        .build()
        .map_err(|e| req.from_build_err(e))
}

pub fn delete_object(req: &mut S3Request) -> Result<DeleteObjectInput, S3Error> {
    DeleteObjectInput::builder()
        .bucket(req.get_bucket())
        .key(req.get_key())
        .set_bypass_governance_retention(req.get_header_parse(X_AMZ_BYPASS_GOVERNANCE_RETENTION))
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_mfa(req.get_header(X_AMZ_MFA))
        .set_request_payer(req.get_header_parse(X_AMZ_REQUEST_PAYER))
        .set_version_id(req.get_param(P_VERSION_ID))
        .build()
        .map_err(|e| req.from_build_err(e))
}

pub fn delete_objects(req: &mut S3Request) -> Result<DeleteObjectsInput, S3Error> {
    DeleteObjectsInput::builder()
        .bucket(req.get_bucket())
        .set_bypass_governance_retention(req.get_header_parse(X_AMZ_BYPASS_GOVERNANCE_RETENTION))
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_mfa(req.get_header(X_AMZ_MFA))
        .set_request_payer(req.get_header_parse(X_AMZ_REQUEST_PAYER))
        // TODO parse body xml for delete_objects
        // .set_delete(req.get_body_parse(B_DELETE))
        .build()
        .map_err(|e| req.from_build_err(e))
}

//---------------------------------------------------//
// multipart upload ops                              //
//---------------------------------------------------//
//-------------------------------//
// gen!(CreateMultipartUpload);
// gen!(CompleteMultipartUpload);
// gen!(AbortMultipartUpload);
// gen!(UploadPart);
// gen!(UploadPartCopy);
//-------------------------------//

pub fn create_multipart_upload(req: &mut S3Request) -> Result<CreateMultipartUploadInput, S3Error> {
    CreateMultipartUploadInput::builder()
        .bucket(req.get_bucket())
        .key(req.get_key())
        .set_acl(req.get_header_parse(X_AMZ_ACL))
        .set_bucket_key_enabled(req.get_header_parse(X_AMZ_SSE_BUCKET_KEY_ENABLED))
        .set_cache_control(req.get_header(H_CACHE_CONTROL))
        .set_content_disposition(req.get_header(H_CONTENT_DISPOSITION))
        .set_content_encoding(req.get_header(H_CONTENT_ENCODING))
        .set_content_language(req.get_header(H_CONTENT_LANGUAGE))
        .set_content_type(req.get_header(H_CONTENT_TYPE))
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_expires(req.get_header_date(H_EXPIRES))
        .set_grant_full_control(req.get_header(X_AMZ_GRANT_FULL_CONTROL))
        .set_grant_read(req.get_header(X_AMZ_GRANT_READ))
        .set_grant_read_acp(req.get_header(X_AMZ_GRANT_READ_ACP))
        .set_grant_write_acp(req.get_header(X_AMZ_GRANT_WRITE_ACP))
        .set_metadata(req.get_header_map(X_AMZ_META_PREFIX))
        .set_object_lock_legal_hold_status(req.get_header_parse(X_AMZ_OBJECT_LOCK_LEGAL_HOLD))
        .set_object_lock_mode(req.get_header_parse(X_AMZ_OBJECT_LOCK_MODE))
        .set_object_lock_retain_until_date(req.get_header_date(X_AMZ_OBJECT_LOCK_RETAIN_UNTIL_DATE))
        .set_request_payer(req.get_header_parse(X_AMZ_REQUEST_PAYER))
        .set_server_side_encryption(req.get_header_parse(X_AMZ_SSE))
        .set_sse_customer_algorithm(req.get_header(X_AMZ_SSE_CUSTOMER_ALG))
        .set_sse_customer_key(req.get_header(X_AMZ_SSE_CUSTOMER_KEY))
        .set_sse_customer_key_md5(req.get_header(X_AMZ_SSE_CUSTOMER_KEY_MD5))
        .set_ssekms_encryption_context(req.get_header(X_AMZ_SSE_CONTEXT))
        .set_ssekms_key_id(req.get_header(X_AMZ_SSE_AWS_KMS_KEY_ID))
        .set_storage_class(req.get_header_parse(X_AMZ_STORAGE_CLASS))
        .set_tagging(req.get_header(X_AMZ_TAGGING))
        .set_website_redirect_location(req.get_header(X_AMZ_WEBSITE_REDIRECT_LOCATION))
        .build()
        .map_err(|e| req.from_build_err(e))
}

pub fn complete_multipart_upload(
    req: &mut S3Request,
) -> Result<CompleteMultipartUploadInput, S3Error> {
    CompleteMultipartUploadInput::builder()
        .bucket(req.get_bucket())
        .key(req.get_key())
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_request_payer(req.get_header_parse(X_AMZ_REQUEST_PAYER))
        .set_upload_id(req.get_param(P_UPLOAD_ID))
        // TODO parse body xml for complete_multipart_upload
        // .set_multipart_upload(req.get_body_parse(B_CompleteMultipartUpload))
        .build()
        .map_err(|e| req.from_build_err(e))
}

pub fn abort_multipart_upload(req: &mut S3Request) -> Result<AbortMultipartUploadInput, S3Error> {
    AbortMultipartUploadInput::builder()
        .bucket(req.get_bucket())
        .key(req.get_key())
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_request_payer(req.get_header_parse(X_AMZ_REQUEST_PAYER))
        .set_upload_id(req.get_param(P_UPLOAD_ID))
        .build()
        .map_err(|e| req.from_build_err(e))
}

pub fn upload_part(req: &mut S3Request) -> Result<UploadPartInput, S3Error> {
    UploadPartInput::builder()
        .bucket(req.get_bucket())
        .key(req.get_key())
        .set_content_length(req.get_header_parse(H_CONTENT_LENGTH))
        .set_content_md5(req.get_header(H_CONTENT_MD5))
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_part_number(req.get_param_parse(P_PART_NUMBER))
        .set_request_payer(req.get_header_parse(X_AMZ_REQUEST_PAYER))
        .set_sse_customer_algorithm(req.get_header(X_AMZ_SSE_CUSTOMER_ALG))
        .set_sse_customer_key(req.get_header(X_AMZ_SSE_CUSTOMER_KEY))
        .set_sse_customer_key_md5(req.get_header(X_AMZ_SSE_CUSTOMER_KEY_MD5))
        .set_upload_id(req.get_param(P_UPLOAD_ID))
        .set_body(Some(ByteStream::new(req.take_body().into())))
        .build()
        .map_err(|e| req.from_build_err(e))
}

pub fn upload_part_copy(req: &mut S3Request) -> Result<UploadPartCopyInput, S3Error> {
    UploadPartCopyInput::builder()
        .bucket(req.get_bucket())
        .key(req.get_key())
        .set_copy_source(req.get_header(X_AMZ_COPY_SOURCE))
        .set_copy_source_range(req.get_header(X_AMZ_COPY_SOURCE_RANGE))
        .set_copy_source_if_match(req.get_header(X_AMZ_COPY_SOURCE_IF_MATCH))
        .set_copy_source_if_modified_since(req.get_header_date(X_AMZ_COPY_SOURCE_IF_MODIFIED_SINCE))
        .set_copy_source_if_none_match(req.get_header(X_AMZ_COPY_SOURCE_IF_NONE_MATCH))
        .set_copy_source_if_unmodified_since(
            req.get_header_date(X_AMZ_COPY_SOURCE_IF_UNMODIFIED_SINCE),
        )
        .set_copy_source_sse_customer_algorithm(req.get_header(X_AMZ_COPY_SOURCE_SSE_CUSTOMER_ALG))
        .set_copy_source_sse_customer_key(req.get_header(X_AMZ_COPY_SOURCE_SSE_CUSTOMER_KEY))
        .set_copy_source_sse_customer_key_md5(
            req.get_header(X_AMZ_COPY_SOURCE_SSE_CUSTOMER_KEY_MD5),
        )
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_expected_source_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_part_number(req.get_param_parse(P_PART_NUMBER))
        .set_request_payer(req.get_header_parse(X_AMZ_REQUEST_PAYER))
        .set_sse_customer_algorithm(req.get_header(X_AMZ_SSE_CUSTOMER_ALG))
        .set_sse_customer_key(req.get_header(X_AMZ_SSE_CUSTOMER_KEY))
        .set_sse_customer_key_md5(req.get_header(X_AMZ_SSE_CUSTOMER_KEY_MD5))
        .set_upload_id(req.get_param(P_UPLOAD_ID))
        .build()
        .map_err(|e| req.from_build_err(e))
}

//---------------------------------------------------//
// list ops                                          //
//---------------------------------------------------//
//-------------------------------//
// gen!(ListBuckets);
// gen!(ListObjects);
// gen!(ListObjectsV2);
// gen!(ListObjectVersions);
// gen!(ListMultipartUploads);
// gen!(ListParts);
//-------------------------------//

pub fn list_buckets(req: &mut S3Request) -> Result<ListBucketsInput, S3Error> {
    ListBucketsInput::builder()
        .build()
        .map_err(|e| req.from_build_err(e))
}

pub fn list_objects(req: &mut S3Request) -> Result<ListObjectsInput, S3Error> {
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
        .map_err(|e| req.from_build_err(e))
}

pub fn list_objects_v2(req: &mut S3Request) -> Result<ListObjectsV2Input, S3Error> {
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
        .map_err(|e| req.from_build_err(e))
}

pub fn list_object_versions(req: &mut S3Request) -> Result<ListObjectVersionsInput, S3Error> {
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
        .map_err(|e| req.from_build_err(e))
}

pub fn list_multipart_uploads(req: &mut S3Request) -> Result<ListMultipartUploadsInput, S3Error> {
    ListMultipartUploadsInput::builder()
        .bucket(req.get_bucket())
        .set_delimiter(req.get_param(P_DELIMITER))
        .set_encoding_type(req.get_param_parse(P_ENCODING_TYPE))
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_key_marker(req.get_param(P_KEY_MARKER))
        .set_max_uploads(req.get_param_parse(P_MAX_UPLOADS))
        .set_prefix(req.get_param(P_PREFIX))
        .set_upload_id_marker(req.get_header_parse(P_UPLOAD_ID_MARKER))
        .build()
        .map_err(|e| req.from_build_err(e))
}

pub fn list_parts(req: &mut S3Request) -> Result<ListPartsInput, S3Error> {
    ListPartsInput::builder()
        .bucket(req.get_bucket())
        .key(req.get_key())
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_max_parts(req.get_param_parse(P_MAX_PARTS))
        .set_part_number_marker(req.get_param(P_PART_NUMBER_MARKER))
        .set_request_payer(req.get_header_parse(X_AMZ_REQUEST_PAYER))
        .set_upload_id(req.get_header(P_UPLOAD_ID))
        .build()
        .map_err(|e| req.from_build_err(e))
}

//---------------------------------------------------//
// advanced object ops                               //
//---------------------------------------------------//
//-------------------------------//
// gen!(GetObjectAcl);
// gen!(PutObjectAcl);
//-------------------------------//
// gen!(GetObjectTagging);
// gen!(PutObjectTagging);
// gen!(DeleteObjectTagging);
//-------------------------------//
// gen!(GetObjectRetention);
// gen!(PutObjectRetention);
//-------------------------------//
// gen!(GetObjectLegalHold);
// gen!(PutObjectLegalHold);
//-------------------------------//
// gen!(RestoreObject);
// gen!(GetObjectTorrent);
// gen!(SelectObjectContent);
//-------------------------------//

pub fn get_object_acl(req: &mut S3Request) -> Result<GetObjectAclInput, S3Error> {
    GetObjectAclInput::builder()
        .bucket(req.get_bucket())
        .key(req.get_key())
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_request_payer(req.get_header_parse(X_AMZ_REQUEST_PAYER))
        .set_version_id(req.get_param(P_VERSION_ID))
        .build()
        .map_err(|e| req.from_build_err(e))
}

pub fn put_object_acl(req: &mut S3Request) -> Result<PutObjectAclInput, S3Error> {
    PutObjectAclInput::builder()
        .bucket(req.get_bucket())
        .key(req.get_key())
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
        // TODO parse body xml for put_object_acl
        // .set_access_control_policy(req.get_body_parse("AccessControlPolicy"))
        .build()
        .map_err(|e| req.from_build_err(e))
}

pub fn get_object_tagging(req: &mut S3Request) -> Result<GetObjectTaggingInput, S3Error> {
    GetObjectTaggingInput::builder()
        .bucket(req.get_bucket())
        .key(req.get_key())
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_request_payer(req.get_header_parse(X_AMZ_REQUEST_PAYER))
        .set_version_id(req.get_param(P_VERSION_ID))
        .build()
        .map_err(|e| req.from_build_err(e))
}

pub fn put_object_tagging(req: &mut S3Request) -> Result<PutObjectTaggingInput, S3Error> {
    PutObjectTaggingInput::builder()
        .bucket(req.get_bucket())
        .key(req.get_key())
        .set_content_md5(req.get_header(H_CONTENT_MD5))
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_request_payer(req.get_header_parse(X_AMZ_REQUEST_PAYER))
        .set_version_id(req.get_param(P_VERSION_ID))
        // TODO parse xml body for put_object_tagging
        // .set_tagging(req.get_body_parse("Tagging"))
        .build()
        .map_err(|e| req.from_build_err(e))
}

pub fn delete_object_tagging(req: &mut S3Request) -> Result<DeleteObjectTaggingInput, S3Error> {
    DeleteObjectTaggingInput::builder()
        .bucket(req.get_bucket())
        .key(req.get_key())
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_version_id(req.get_param(P_VERSION_ID))
        .build()
        .map_err(|e| req.from_build_err(e))
}

pub fn get_object_retention(req: &mut S3Request) -> Result<GetObjectRetentionInput, S3Error> {
    GetObjectRetentionInput::builder()
        .bucket(req.get_bucket())
        .key(req.get_key())
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_request_payer(req.get_header_parse(X_AMZ_REQUEST_PAYER))
        .set_version_id(req.get_param(P_VERSION_ID))
        .build()
        .map_err(|e| req.from_build_err(e))
}

pub fn put_object_retention(req: &mut S3Request) -> Result<PutObjectRetentionInput, S3Error> {
    PutObjectRetentionInput::builder()
        .bucket(req.get_bucket())
        .key(req.get_key())
        .set_bypass_governance_retention(req.get_header_parse(X_AMZ_BYPASS_GOVERNANCE_RETENTION))
        .set_content_md5(req.get_header(H_CONTENT_MD5))
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_request_payer(req.get_header_parse(X_AMZ_REQUEST_PAYER))
        .set_version_id(req.get_param(P_VERSION_ID))
        // TODO parse xml body for put_object_retention
        // .set_retention(req.get_body_parse("Retention"))
        .build()
        .map_err(|e| req.from_build_err(e))
}

pub fn get_object_legal_hold(req: &mut S3Request) -> Result<GetObjectLegalHoldInput, S3Error> {
    GetObjectLegalHoldInput::builder()
        .bucket(req.get_bucket())
        .key(req.get_key())
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_request_payer(req.get_header_parse(X_AMZ_REQUEST_PAYER))
        .set_version_id(req.get_param(P_VERSION_ID))
        .build()
        .map_err(|e| req.from_build_err(e))
}

pub fn put_object_legal_hold(req: &mut S3Request) -> Result<PutObjectLegalHoldInput, S3Error> {
    PutObjectLegalHoldInput::builder()
        .bucket(req.get_bucket())
        .key(req.get_key())
        .set_content_md5(req.get_header(H_CONTENT_MD5))
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_request_payer(req.get_header_parse(X_AMZ_REQUEST_PAYER))
        .set_version_id(req.get_param(P_VERSION_ID))
        // TODO parse xml body for put_object_legal_hold
        // .set_legal_hold(req.get_body_parse("LegalHold"))
        .build()
        .map_err(|e| req.from_build_err(e))
}

pub fn restore_object(req: &mut S3Request) -> Result<RestoreObjectInput, S3Error> {
    RestoreObjectInput::builder()
        .bucket(req.get_bucket())
        .key(req.get_key())
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_request_payer(req.get_header_parse(X_AMZ_REQUEST_PAYER))
        .set_version_id(req.get_param(P_VERSION_ID))
        // TODO parse xml body for restore_object
        // .set_restore_request(input)
        .build()
        .map_err(|e| req.from_build_err(e))
}
pub fn get_object_torrent(req: &mut S3Request) -> Result<GetObjectTorrentInput, S3Error> {
    GetObjectTorrentInput::builder()
        .bucket(req.get_bucket())
        .key(req.get_key())
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_request_payer(req.get_header_parse(X_AMZ_REQUEST_PAYER))
        .build()
        .map_err(|e| req.from_build_err(e))
}
pub fn select_object_content(req: &mut S3Request) -> Result<SelectObjectContentInput, S3Error> {
    SelectObjectContentInput::builder()
        .bucket(req.get_bucket())
        .key(req.get_key())
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_sse_customer_algorithm(req.get_header(X_AMZ_SSE_CUSTOMER_ALG))
        .set_sse_customer_key(req.get_header(X_AMZ_SSE_CUSTOMER_KEY))
        .set_sse_customer_key_md5(req.get_header(X_AMZ_SSE_CUSTOMER_KEY_MD5))
        // TODO parse xml body for select_object_content
        // .set_expression(input)
        // .set_expression_type(input)
        // .set_input_serialization(input)
        // .set_output_serialization(input)
        // .set_request_progress(input)
        // .set_scan_range(input)
        .build()
        .map_err(|e| req.from_build_err(e))
}

//---------------------------------------------------//
// advanced bucket ops                               //
//---------------------------------------------------//
//-------------------------------//
// gen!(GetBucketAcl);
// gen!(PutBucketAcl);
//-------------------------------//
// gen!(GetBucketTagging);
// gen!(PutBucketTagging);
// gen!(DeleteBucketTagging);
//-------------------------------//
// gen!(GetObjectLockConfiguration);
// gen!(PutObjectLockConfiguration);
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
//-------------------------------//
gen!(GetBucketVersioning);
gen!(PutBucketVersioning);
//-------------------------------//
gen!(GetBucketRequestPayment);
gen!(PutBucketRequestPayment);
//-------------------------------//
gen!(GetBucketAccelerateConfiguration);
gen!(PutBucketAccelerateConfiguration);
//-------------------------------//
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
gen!(WriteGetObjectResponse);
//-------------------------------//

pub fn get_bucket_acl(req: &mut S3Request) -> Result<GetBucketAclInput, S3Error> {
    GetBucketAclInput::builder()
        .bucket(req.get_bucket())
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .build()
        .map_err(|e| req.from_build_err(e))
}

pub fn put_bucket_acl(req: &mut S3Request) -> Result<PutBucketAclInput, S3Error> {
    PutBucketAclInput::builder()
        .bucket(req.get_bucket())
        .set_acl(req.get_header_parse(X_AMZ_ACL))
        .set_content_md5(req.get_header(H_CONTENT_MD5))
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_grant_full_control(req.get_header(X_AMZ_GRANT_FULL_CONTROL))
        .set_grant_read(req.get_header(X_AMZ_GRANT_READ))
        .set_grant_read_acp(req.get_header(X_AMZ_GRANT_READ_ACP))
        .set_grant_write(req.get_header(X_AMZ_GRANT_WRITE))
        .set_grant_write_acp(req.get_header(X_AMZ_GRANT_WRITE_ACP))
        // TODO parse xml body for put_bucket_acl
        // .set_access_control_policy(req.get_body_parse("AccessControlPolicy"))
        .build()
        .map_err(|e| req.from_build_err(e))
}

pub fn get_bucket_tagging(req: &mut S3Request) -> Result<GetBucketTaggingInput, S3Error> {
    GetBucketTaggingInput::builder()
        .bucket(req.get_bucket())
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .build()
        .map_err(|e| req.from_build_err(e))
}

pub fn put_bucket_tagging(req: &mut S3Request) -> Result<PutBucketTaggingInput, S3Error> {
    PutBucketTaggingInput::builder()
        .bucket(req.get_bucket())
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_content_md5(req.get_header(H_CONTENT_MD5))
        // TODO parse xml body for put_bucket_tagging
        // .set_tagging(req.get_body_parse("Tagging"))
        .build()
        .map_err(|e| req.from_build_err(e))
}

pub fn delete_bucket_tagging(req: &mut S3Request) -> Result<DeleteBucketTaggingInput, S3Error> {
    DeleteBucketTaggingInput::builder()
        .bucket(req.get_bucket())
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .build()
        .map_err(|e| req.from_build_err(e))
}

pub fn get_object_lock_configuration(
    req: &mut S3Request,
) -> Result<GetObjectLockConfigurationInput, S3Error> {
    GetObjectLockConfigurationInput::builder()
        .bucket(req.get_bucket())
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .build()
        .map_err(|e| req.from_build_err(e))
}

pub fn put_object_lock_configuration(
    req: &mut S3Request,
) -> Result<PutObjectLockConfigurationInput, S3Error> {
    PutObjectLockConfigurationInput::builder()
        .bucket(req.get_bucket())
        .set_content_md5(req.get_header(H_CONTENT_MD5))
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_request_payer(req.get_header_parse(X_AMZ_REQUEST_PAYER))
        .set_token(req.get_header(X_AMZ_BUCKET_OBJECT_LOCK_TOKEN))
        // TODO parse xml body for put_object_lock_configuration
        // .set_object_lock_configuration(req.get_body_parse("ObjectLockConfiguration"))
        .build()
        .map_err(|e| req.from_build_err(e))
}
