//! This module implements a function per S3 op to parse the input from the HTTP request.
//! The functions take S3Request which is a wrapper around the HTTP request.
//! Currently written by hand which is difficult to maintain long term.
//! TODO This module should be generated from https://github.com/awslabs/smithy-rs.

use crate::{
    manual::{headers::*, models::*},
    http::*,
};
use aws_sdk_s3::{input::*, ByteStream};

//---------------------------------------------------//
// basic bucket ops                                  //
//---------------------------------------------------//

pub async fn head_bucket(req: &mut S3Request) -> Result<HeadBucketInput, S3Error> {
    Ok(HeadBucketInput::builder()
        .bucket(req.get_bucket())
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .build()?)
}

pub async fn create_bucket(req: &mut S3Request) -> Result<CreateBucketInput, S3Error> {
    Ok(CreateBucketInput::builder()
        .bucket(req.get_bucket())
        .set_acl(req.get_header(X_AMZ_ACL))
        .set_grant_full_control(req.get_header(X_AMZ_GRANT_FULL_CONTROL))
        .set_grant_read(req.get_header(X_AMZ_GRANT_READ))
        .set_grant_read_acp(req.get_header(X_AMZ_GRANT_READ_ACP))
        .set_grant_write(req.get_header(X_AMZ_GRANT_WRITE))
        .set_grant_write_acp(req.get_header(X_AMZ_GRANT_WRITE_ACP))
        .set_object_lock_enabled_for_bucket(req.get_header(X_AMZ_BUCKET_OBJECT_LOCK_ENABLED))
        .create_bucket_configuration(decode_create_bucket_configuration(
            req.take_body_bytes().await?,
        )?)
        .build()?)
}

pub async fn delete_bucket(req: &mut S3Request) -> Result<DeleteBucketInput, S3Error> {
    Ok(DeleteBucketInput::builder()
        .bucket(req.get_bucket())
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .build()?)
}

pub async fn get_bucket_location(req: &mut S3Request) -> Result<GetBucketLocationInput, S3Error> {
    Ok(GetBucketLocationInput::builder()
        .bucket(req.get_bucket())
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .build()?)
}

//---------------------------------------------------//
// basic object ops                                  //
//---------------------------------------------------//

pub async fn get_object(req: &mut S3Request) -> Result<GetObjectInput, S3Error> {
    Ok(GetObjectInput::builder()
        .bucket(req.get_bucket())
        .key(req.get_key())
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_if_match(req.get_header(H_IF_MATCH))
        .set_if_none_match(req.get_header(H_IF_NONE_MATCH))
        .set_if_modified_since(req.get_header(H_IF_MODIFIED_SINCE))
        .set_if_unmodified_since(req.get_header(H_IF_UNMODIFIED_SINCE))
        .set_part_number(req.get_param(P_PART_NUMBER))
        .set_range(req.get_header(H_RANGE))
        .set_request_payer(req.get_header(X_AMZ_REQUEST_PAYER))
        .set_response_cache_control(req.get_param(P_RESPONSE_CACHE_CONTROL))
        .set_response_content_disposition(req.get_param(P_RESPONSE_CONTENT_DISPOSITION))
        .set_response_content_encoding(req.get_param(P_RESPONSE_CONTENT_ENCODING))
        .set_response_content_language(req.get_param(P_RESPONSE_CONTENT_LANGUAGE))
        .set_response_content_type(req.get_param(P_RESPONSE_CONTENT_TYPE))
        .set_response_expires(req.get_param(P_RESPONSE_EXPIRES))
        .set_sse_customer_algorithm(req.get_header(X_AMZ_SSE_CUSTOMER_ALG))
        .set_sse_customer_key(req.get_header(X_AMZ_SSE_CUSTOMER_KEY))
        .set_sse_customer_key_md5(req.get_header(X_AMZ_SSE_CUSTOMER_KEY_MD5))
        .set_version_id(req.get_param(P_VERSION_ID))
        .build()?)
}

pub async fn head_object(req: &mut S3Request) -> Result<HeadObjectInput, S3Error> {
    Ok(HeadObjectInput::builder()
        .bucket(req.get_bucket())
        .key(req.get_key())
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_if_match(req.get_header(H_IF_MATCH))
        .set_if_none_match(req.get_header(H_IF_NONE_MATCH))
        .set_if_modified_since(req.get_header(H_IF_MODIFIED_SINCE))
        .set_if_unmodified_since(req.get_header(H_IF_UNMODIFIED_SINCE))
        .set_part_number(req.get_param(P_PART_NUMBER))
        .set_range(req.get_header(H_RANGE))
        .set_request_payer(req.get_header(X_AMZ_REQUEST_PAYER))
        .set_sse_customer_algorithm(req.get_header(X_AMZ_SSE_CUSTOMER_ALG))
        .set_sse_customer_key(req.get_header(X_AMZ_SSE_CUSTOMER_KEY))
        .set_sse_customer_key_md5(req.get_header(X_AMZ_SSE_CUSTOMER_KEY_MD5))
        .set_version_id(req.get_param(P_VERSION_ID))
        .build()?)
}

pub async fn put_object(req: &mut S3Request) -> Result<PutObjectInput, S3Error> {
    Ok(PutObjectInput::builder()
        .bucket(req.get_bucket())
        .key(req.get_key())
        .set_acl(req.get_header(X_AMZ_ACL))
        .set_bucket_key_enabled(req.get_header(X_AMZ_SSE_BUCKET_KEY_ENABLED))
        .set_cache_control(req.get_header(H_CACHE_CONTROL))
        .set_content_disposition(req.get_header(H_CONTENT_DISPOSITION))
        .set_content_encoding(req.get_header(H_CONTENT_ENCODING))
        .set_content_language(req.get_header(H_CONTENT_LANGUAGE))
        .set_content_length(req.get_header(H_CONTENT_LENGTH))
        .set_content_md5(req.get_header(H_CONTENT_MD5))
        .set_content_type(req.get_header(H_CONTENT_TYPE))
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_expires(req.get_header(H_EXPIRES))
        .set_grant_full_control(req.get_header(X_AMZ_GRANT_FULL_CONTROL))
        .set_grant_read(req.get_header(X_AMZ_GRANT_READ))
        .set_grant_read_acp(req.get_header(X_AMZ_GRANT_READ_ACP))
        .set_grant_write_acp(req.get_header(X_AMZ_GRANT_WRITE_ACP))
        .set_metadata(req.get_header_map(X_AMZ_META_PREFIX))
        .set_object_lock_legal_hold_status(req.get_header(X_AMZ_OBJECT_LOCK_LEGAL_HOLD))
        .set_object_lock_mode(req.get_header(X_AMZ_OBJECT_LOCK_MODE))
        .set_object_lock_retain_until_date(req.get_header(X_AMZ_OBJECT_LOCK_RETAIN_UNTIL_DATE))
        .set_request_payer(req.get_header(X_AMZ_REQUEST_PAYER))
        .set_server_side_encryption(req.get_header(X_AMZ_SSE))
        .set_sse_customer_algorithm(req.get_header(X_AMZ_SSE_CUSTOMER_ALG))
        .set_sse_customer_key(req.get_header(X_AMZ_SSE_CUSTOMER_KEY))
        .set_sse_customer_key_md5(req.get_header(X_AMZ_SSE_CUSTOMER_KEY_MD5))
        .set_ssekms_encryption_context(req.get_header(X_AMZ_SSE_CONTEXT))
        .set_ssekms_key_id(req.get_header(X_AMZ_SSE_AWS_KMS_KEY_ID))
        .set_storage_class(req.get_header(X_AMZ_STORAGE_CLASS))
        .set_tagging(req.get_header(X_AMZ_TAGGING))
        .set_website_redirect_location(req.get_header(X_AMZ_WEBSITE_REDIRECT_LOCATION))
        .set_body(Some(ByteStream::new(req.take_body().into())))
        .build()?)
}

pub async fn copy_object(req: &mut S3Request) -> Result<CopyObjectInput, S3Error> {
    Ok(CopyObjectInput::builder()
        .bucket(req.get_bucket())
        .key(req.get_key())
        .set_acl(req.get_header(X_AMZ_ACL))
        .set_bucket_key_enabled(req.get_header(X_AMZ_SSE_BUCKET_KEY_ENABLED))
        .set_cache_control(req.get_header(H_CACHE_CONTROL))
        .set_content_disposition(req.get_header(H_CONTENT_DISPOSITION))
        .set_content_encoding(req.get_header(H_CONTENT_ENCODING))
        .set_content_language(req.get_header(H_CONTENT_LANGUAGE))
        .set_content_type(req.get_header(H_CONTENT_TYPE))
        .set_copy_source(req.get_header(X_AMZ_COPY_SOURCE))
        .set_copy_source_if_match(req.get_header(X_AMZ_COPY_SOURCE_IF_MATCH))
        .set_copy_source_if_modified_since(req.get_header(X_AMZ_COPY_SOURCE_IF_MODIFIED_SINCE))
        .set_copy_source_if_none_match(req.get_header(X_AMZ_COPY_SOURCE_IF_NONE_MATCH))
        .set_copy_source_if_unmodified_since(
            req.get_header(X_AMZ_COPY_SOURCE_IF_UNMODIFIED_SINCE),
        )
        .set_copy_source_sse_customer_algorithm(req.get_header(X_AMZ_COPY_SOURCE_SSE_CUSTOMER_ALG))
        .set_copy_source_sse_customer_key(req.get_header(X_AMZ_COPY_SOURCE_SSE_CUSTOMER_KEY))
        .set_copy_source_sse_customer_key_md5(
            req.get_header(X_AMZ_COPY_SOURCE_SSE_CUSTOMER_KEY_MD5),
        )
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_expected_source_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_expires(req.get_header(H_EXPIRES))
        .set_grant_full_control(req.get_header(X_AMZ_GRANT_FULL_CONTROL))
        .set_grant_read(req.get_header(X_AMZ_GRANT_READ))
        .set_grant_read_acp(req.get_header(X_AMZ_GRANT_READ_ACP))
        .set_grant_write_acp(req.get_header(X_AMZ_GRANT_WRITE_ACP))
        .set_metadata(req.get_header_map(X_AMZ_META_PREFIX))
        .set_metadata_directive(req.get_header(X_AMZ_METADATA_DIRECTIVE))
        .set_object_lock_legal_hold_status(req.get_header(X_AMZ_OBJECT_LOCK_LEGAL_HOLD))
        .set_object_lock_mode(req.get_header(X_AMZ_OBJECT_LOCK_MODE))
        .set_object_lock_retain_until_date(req.get_header(X_AMZ_OBJECT_LOCK_RETAIN_UNTIL_DATE))
        .set_request_payer(req.get_header(X_AMZ_REQUEST_PAYER))
        .set_server_side_encryption(req.get_header(X_AMZ_SSE))
        .set_sse_customer_algorithm(req.get_header(X_AMZ_SSE_CUSTOMER_ALG))
        .set_sse_customer_key(req.get_header(X_AMZ_SSE_CUSTOMER_KEY))
        .set_sse_customer_key_md5(req.get_header(X_AMZ_SSE_CUSTOMER_KEY_MD5))
        .set_ssekms_encryption_context(req.get_header(X_AMZ_SSE_CONTEXT))
        .set_ssekms_key_id(req.get_header(X_AMZ_SSE_AWS_KMS_KEY_ID))
        .set_storage_class(req.get_header(X_AMZ_STORAGE_CLASS))
        .set_tagging(req.get_header(X_AMZ_TAGGING))
        .set_tagging_directive(req.get_header(X_AMZ_TAGGING_DIRECTIVE))
        .set_website_redirect_location(req.get_header(X_AMZ_WEBSITE_REDIRECT_LOCATION))
        .build()?)
}

pub async fn delete_object(req: &mut S3Request) -> Result<DeleteObjectInput, S3Error> {
    Ok(DeleteObjectInput::builder()
        .bucket(req.get_bucket())
        .key(req.get_key())
        .set_bypass_governance_retention(req.get_header(X_AMZ_BYPASS_GOVERNANCE_RETENTION))
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_mfa(req.get_header(X_AMZ_MFA))
        .set_request_payer(req.get_header(X_AMZ_REQUEST_PAYER))
        .set_version_id(req.get_param(P_VERSION_ID))
        .build()?)
}

pub async fn delete_objects(req: &mut S3Request) -> Result<DeleteObjectsInput, S3Error> {
    Ok(DeleteObjectsInput::builder()
        .bucket(req.get_bucket())
        .set_bypass_governance_retention(req.get_header(X_AMZ_BYPASS_GOVERNANCE_RETENTION))
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_mfa(req.get_header(X_AMZ_MFA))
        .set_request_payer(req.get_header(X_AMZ_REQUEST_PAYER))
        .delete(decode_delete_objects(req.take_body_bytes().await?)?)
        .build()?)
}

//---------------------------------------------------//
// multipart upload ops                              //
//---------------------------------------------------//

pub async fn create_multipart_upload(
    req: &mut S3Request,
) -> Result<CreateMultipartUploadInput, S3Error> {
    Ok(CreateMultipartUploadInput::builder()
        .bucket(req.get_bucket())
        .key(req.get_key())
        .set_acl(req.get_header(X_AMZ_ACL))
        .set_bucket_key_enabled(req.get_header(X_AMZ_SSE_BUCKET_KEY_ENABLED))
        .set_cache_control(req.get_header(H_CACHE_CONTROL))
        .set_content_disposition(req.get_header(H_CONTENT_DISPOSITION))
        .set_content_encoding(req.get_header(H_CONTENT_ENCODING))
        .set_content_language(req.get_header(H_CONTENT_LANGUAGE))
        .set_content_type(req.get_header(H_CONTENT_TYPE))
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_expires(req.get_header(H_EXPIRES))
        .set_grant_full_control(req.get_header(X_AMZ_GRANT_FULL_CONTROL))
        .set_grant_read(req.get_header(X_AMZ_GRANT_READ))
        .set_grant_read_acp(req.get_header(X_AMZ_GRANT_READ_ACP))
        .set_grant_write_acp(req.get_header(X_AMZ_GRANT_WRITE_ACP))
        .set_metadata(req.get_header_map(X_AMZ_META_PREFIX))
        .set_object_lock_legal_hold_status(req.get_header(X_AMZ_OBJECT_LOCK_LEGAL_HOLD))
        .set_object_lock_mode(req.get_header(X_AMZ_OBJECT_LOCK_MODE))
        .set_object_lock_retain_until_date(req.get_header(X_AMZ_OBJECT_LOCK_RETAIN_UNTIL_DATE))
        .set_request_payer(req.get_header(X_AMZ_REQUEST_PAYER))
        .set_server_side_encryption(req.get_header(X_AMZ_SSE))
        .set_sse_customer_algorithm(req.get_header(X_AMZ_SSE_CUSTOMER_ALG))
        .set_sse_customer_key(req.get_header(X_AMZ_SSE_CUSTOMER_KEY))
        .set_sse_customer_key_md5(req.get_header(X_AMZ_SSE_CUSTOMER_KEY_MD5))
        .set_ssekms_encryption_context(req.get_header(X_AMZ_SSE_CONTEXT))
        .set_ssekms_key_id(req.get_header(X_AMZ_SSE_AWS_KMS_KEY_ID))
        .set_storage_class(req.get_header(X_AMZ_STORAGE_CLASS))
        .set_tagging(req.get_header(X_AMZ_TAGGING))
        .set_website_redirect_location(req.get_header(X_AMZ_WEBSITE_REDIRECT_LOCATION))
        .build()?)
}

pub async fn complete_multipart_upload(
    req: &mut S3Request,
) -> Result<CompleteMultipartUploadInput, S3Error> {
    Ok(CompleteMultipartUploadInput::builder()
        .bucket(req.get_bucket())
        .key(req.get_key())
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_request_payer(req.get_header(X_AMZ_REQUEST_PAYER))
        .set_upload_id(req.get_param(P_UPLOAD_ID))
        .multipart_upload(decode_complete_multipart_upload(
            req.take_body_bytes().await?,
        )?)
        .build()?)
}

pub async fn abort_multipart_upload(
    req: &mut S3Request,
) -> Result<AbortMultipartUploadInput, S3Error> {
    Ok(AbortMultipartUploadInput::builder()
        .bucket(req.get_bucket())
        .key(req.get_key())
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_request_payer(req.get_header(X_AMZ_REQUEST_PAYER))
        .set_upload_id(req.get_param(P_UPLOAD_ID))
        .build()?)
}

pub async fn upload_part(req: &mut S3Request) -> Result<UploadPartInput, S3Error> {
    Ok(UploadPartInput::builder()
        .bucket(req.get_bucket())
        .key(req.get_key())
        .set_content_length(req.get_header(H_CONTENT_LENGTH))
        .set_content_md5(req.get_header(H_CONTENT_MD5))
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_part_number(req.get_param(P_PART_NUMBER))
        .set_request_payer(req.get_header(X_AMZ_REQUEST_PAYER))
        .set_sse_customer_algorithm(req.get_header(X_AMZ_SSE_CUSTOMER_ALG))
        .set_sse_customer_key(req.get_header(X_AMZ_SSE_CUSTOMER_KEY))
        .set_sse_customer_key_md5(req.get_header(X_AMZ_SSE_CUSTOMER_KEY_MD5))
        .set_upload_id(req.get_param(P_UPLOAD_ID))
        .set_body(Some(ByteStream::new(req.take_body().into())))
        .build()?)
}

pub async fn upload_part_copy(req: &mut S3Request) -> Result<UploadPartCopyInput, S3Error> {
    Ok(UploadPartCopyInput::builder()
        .bucket(req.get_bucket())
        .key(req.get_key())
        .set_copy_source(req.get_header(X_AMZ_COPY_SOURCE))
        .set_copy_source_range(req.get_header(X_AMZ_COPY_SOURCE_RANGE))
        .set_copy_source_if_match(req.get_header(X_AMZ_COPY_SOURCE_IF_MATCH))
        .set_copy_source_if_modified_since(req.get_header(X_AMZ_COPY_SOURCE_IF_MODIFIED_SINCE))
        .set_copy_source_if_none_match(req.get_header(X_AMZ_COPY_SOURCE_IF_NONE_MATCH))
        .set_copy_source_if_unmodified_since(
            req.get_header(X_AMZ_COPY_SOURCE_IF_UNMODIFIED_SINCE),
        )
        .set_copy_source_sse_customer_algorithm(req.get_header(X_AMZ_COPY_SOURCE_SSE_CUSTOMER_ALG))
        .set_copy_source_sse_customer_key(req.get_header(X_AMZ_COPY_SOURCE_SSE_CUSTOMER_KEY))
        .set_copy_source_sse_customer_key_md5(
            req.get_header(X_AMZ_COPY_SOURCE_SSE_CUSTOMER_KEY_MD5),
        )
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_expected_source_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_part_number(req.get_param(P_PART_NUMBER))
        .set_request_payer(req.get_header(X_AMZ_REQUEST_PAYER))
        .set_sse_customer_algorithm(req.get_header(X_AMZ_SSE_CUSTOMER_ALG))
        .set_sse_customer_key(req.get_header(X_AMZ_SSE_CUSTOMER_KEY))
        .set_sse_customer_key_md5(req.get_header(X_AMZ_SSE_CUSTOMER_KEY_MD5))
        .set_upload_id(req.get_param(P_UPLOAD_ID))
        .build()?)
}

//---------------------------------------------------//
// list ops                                          //
//---------------------------------------------------//

pub async fn list_buckets(_req: &mut S3Request) -> Result<ListBucketsInput, S3Error> {
    Ok(ListBucketsInput::builder().build()?)
}

pub async fn list_objects(req: &mut S3Request) -> Result<ListObjectsInput, S3Error> {
    Ok(ListObjectsInput::builder()
        .bucket(req.get_bucket())
        .set_delimiter(req.get_param(P_DELIMITER))
        .set_encoding_type(req.get_param(P_ENCODING_TYPE))
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_marker(req.get_param(P_MARKER))
        .set_max_keys(req.get_param(P_MAX_KEYS))
        .set_prefix(req.get_param(P_PREFIX))
        .set_request_payer(req.get_header(X_AMZ_REQUEST_PAYER))
        .build()?)
}

pub async fn list_objects_v2(req: &mut S3Request) -> Result<ListObjectsV2Input, S3Error> {
    Ok(ListObjectsV2Input::builder()
        .bucket(req.get_bucket())
        .set_continuation_token(req.get_param(P_CONTINUATION_TOKEN))
        .set_delimiter(req.get_param(P_DELIMITER))
        .set_encoding_type(req.get_param(P_ENCODING_TYPE))
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_fetch_owner(req.get_param(P_FETCH_OWNER))
        .set_max_keys(req.get_param(P_MAX_KEYS))
        .set_prefix(req.get_param(P_PREFIX))
        .set_request_payer(req.get_header(X_AMZ_REQUEST_PAYER))
        .set_start_after(req.get_param(P_START_AFTER))
        .build()?)
}

pub async fn list_object_versions(req: &mut S3Request) -> Result<ListObjectVersionsInput, S3Error> {
    Ok(ListObjectVersionsInput::builder()
        .bucket(req.get_bucket())
        .set_delimiter(req.get_param(P_DELIMITER))
        .set_encoding_type(req.get_param(P_ENCODING_TYPE))
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_key_marker(req.get_param(P_KEY_MARKER))
        .set_max_keys(req.get_param(P_MAX_KEYS))
        .set_prefix(req.get_param(P_PREFIX))
        .set_version_id_marker(req.get_param(P_VERSION_ID_MARKER))
        .build()?)
}

pub async fn list_multipart_uploads(
    req: &mut S3Request,
) -> Result<ListMultipartUploadsInput, S3Error> {
    Ok(ListMultipartUploadsInput::builder()
        .bucket(req.get_bucket())
        .set_delimiter(req.get_param(P_DELIMITER))
        .set_encoding_type(req.get_param(P_ENCODING_TYPE))
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_key_marker(req.get_param(P_KEY_MARKER))
        .set_max_uploads(req.get_param(P_MAX_UPLOADS))
        .set_prefix(req.get_param(P_PREFIX))
        .set_upload_id_marker(req.get_header(P_UPLOAD_ID_MARKER))
        .build()?)
}

pub async fn list_parts(req: &mut S3Request) -> Result<ListPartsInput, S3Error> {
    Ok(ListPartsInput::builder()
        .bucket(req.get_bucket())
        .key(req.get_key())
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_max_parts(req.get_param(P_MAX_PARTS))
        .set_part_number_marker(req.get_param(P_PART_NUMBER_MARKER))
        .set_request_payer(req.get_header(X_AMZ_REQUEST_PAYER))
        .set_upload_id(req.get_header(P_UPLOAD_ID))
        .build()?)
}

//---------------------------------------------------//
// advanced object ops                               //
//---------------------------------------------------//

pub async fn get_object_acl(req: &mut S3Request) -> Result<GetObjectAclInput, S3Error> {
    Ok(GetObjectAclInput::builder()
        .bucket(req.get_bucket())
        .key(req.get_key())
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_request_payer(req.get_header(X_AMZ_REQUEST_PAYER))
        .set_version_id(req.get_param(P_VERSION_ID))
        .build()?)
}

pub async fn put_object_acl(req: &mut S3Request) -> Result<PutObjectAclInput, S3Error> {
    Ok(PutObjectAclInput::builder()
        .bucket(req.get_bucket())
        .key(req.get_key())
        .set_acl(req.get_header(X_AMZ_ACL))
        .set_content_md5(req.get_header(H_CONTENT_MD5))
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_grant_full_control(req.get_header(X_AMZ_GRANT_FULL_CONTROL))
        .set_grant_read(req.get_header(X_AMZ_GRANT_READ))
        .set_grant_read_acp(req.get_header(X_AMZ_GRANT_READ_ACP))
        .set_grant_write(req.get_header(X_AMZ_GRANT_WRITE))
        .set_grant_write_acp(req.get_header(X_AMZ_GRANT_WRITE_ACP))
        .set_request_payer(req.get_header(X_AMZ_REQUEST_PAYER))
        .set_version_id(req.get_param(P_VERSION_ID))
        .access_control_policy(decode_access_control_policy(req.take_body_bytes().await?)?)
        .build()?)
}

pub async fn get_object_tagging(req: &mut S3Request) -> Result<GetObjectTaggingInput, S3Error> {
    Ok(GetObjectTaggingInput::builder()
        .bucket(req.get_bucket())
        .key(req.get_key())
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_request_payer(req.get_header(X_AMZ_REQUEST_PAYER))
        .set_version_id(req.get_param(P_VERSION_ID))
        .build()?)
}

pub async fn put_object_tagging(req: &mut S3Request) -> Result<PutObjectTaggingInput, S3Error> {
    Ok(PutObjectTaggingInput::builder()
        .bucket(req.get_bucket())
        .key(req.get_key())
        .set_content_md5(req.get_header(H_CONTENT_MD5))
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_request_payer(req.get_header(X_AMZ_REQUEST_PAYER))
        .set_version_id(req.get_param(P_VERSION_ID))
        .tagging(decode_tagging(req.take_body_bytes().await?)?)
        .build()?)
}

pub async fn delete_object_tagging(
    req: &mut S3Request,
) -> Result<DeleteObjectTaggingInput, S3Error> {
    Ok(DeleteObjectTaggingInput::builder()
        .bucket(req.get_bucket())
        .key(req.get_key())
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_version_id(req.get_param(P_VERSION_ID))
        .build()?)
}

pub async fn get_object_retention(req: &mut S3Request) -> Result<GetObjectRetentionInput, S3Error> {
    Ok(GetObjectRetentionInput::builder()
        .bucket(req.get_bucket())
        .key(req.get_key())
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_request_payer(req.get_header(X_AMZ_REQUEST_PAYER))
        .set_version_id(req.get_param(P_VERSION_ID))
        .build()?)
}

pub async fn put_object_retention(req: &mut S3Request) -> Result<PutObjectRetentionInput, S3Error> {
    Ok(PutObjectRetentionInput::builder()
        .bucket(req.get_bucket())
        .key(req.get_key())
        .set_bypass_governance_retention(req.get_header(X_AMZ_BYPASS_GOVERNANCE_RETENTION))
        .set_content_md5(req.get_header(H_CONTENT_MD5))
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_request_payer(req.get_header(X_AMZ_REQUEST_PAYER))
        .set_version_id(req.get_param(P_VERSION_ID))
        .retention(decode_object_lock_retention(req.take_body_bytes().await?)?)
        .build()?)
}

pub async fn get_object_legal_hold(
    req: &mut S3Request,
) -> Result<GetObjectLegalHoldInput, S3Error> {
    Ok(GetObjectLegalHoldInput::builder()
        .bucket(req.get_bucket())
        .key(req.get_key())
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_request_payer(req.get_header(X_AMZ_REQUEST_PAYER))
        .set_version_id(req.get_param(P_VERSION_ID))
        .build()?)
}

pub async fn put_object_legal_hold(
    req: &mut S3Request,
) -> Result<PutObjectLegalHoldInput, S3Error> {
    Ok(PutObjectLegalHoldInput::builder()
        .bucket(req.get_bucket())
        .key(req.get_key())
        .set_content_md5(req.get_header(H_CONTENT_MD5))
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_request_payer(req.get_header(X_AMZ_REQUEST_PAYER))
        .set_version_id(req.get_param(P_VERSION_ID))
        .legal_hold(decode_object_lock_legal_hold(req.take_body_bytes().await?)?)
        .build()?)
}

pub async fn restore_object(req: &mut S3Request) -> Result<RestoreObjectInput, S3Error> {
    Ok(RestoreObjectInput::builder()
        .bucket(req.get_bucket())
        .key(req.get_key())
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_request_payer(req.get_header(X_AMZ_REQUEST_PAYER))
        .set_version_id(req.get_param(P_VERSION_ID))
        .restore_request(decode_restore_request(req.take_body_bytes().await?)?)
        .build()?)
}

pub async fn get_object_torrent(req: &mut S3Request) -> Result<GetObjectTorrentInput, S3Error> {
    Ok(GetObjectTorrentInput::builder()
        .bucket(req.get_bucket())
        .key(req.get_key())
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_request_payer(req.get_header(X_AMZ_REQUEST_PAYER))
        .build()?)
}

pub async fn select_object_content(
    req: &mut S3Request,
) -> Result<SelectObjectContentInput, S3Error> {
    let (select_params, scan_range, request_progress) =
        decode_select_parameters(req.take_body_bytes().await?)?;
    Ok(SelectObjectContentInput::builder()
        .bucket(req.get_bucket())
        .key(req.get_key())
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_sse_customer_algorithm(req.get_header(X_AMZ_SSE_CUSTOMER_ALG))
        .set_sse_customer_key(req.get_header(X_AMZ_SSE_CUSTOMER_KEY))
        .set_sse_customer_key_md5(req.get_header(X_AMZ_SSE_CUSTOMER_KEY_MD5))
        .set_expression(select_params.expression)
        .set_expression_type(select_params.expression_type)
        .set_input_serialization(select_params.input_serialization)
        .set_output_serialization(select_params.output_serialization)
        .set_request_progress(request_progress)
        .set_scan_range(scan_range)
        .build()?)
}

//---------------------------------------------------//
// advanced bucket ops                               //
//---------------------------------------------------//

pub async fn get_bucket_acl(req: &mut S3Request) -> Result<GetBucketAclInput, S3Error> {
    Ok(GetBucketAclInput::builder()
        .bucket(req.get_bucket())
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .build()?)
}

pub async fn put_bucket_acl(req: &mut S3Request) -> Result<PutBucketAclInput, S3Error> {
    Ok(PutBucketAclInput::builder()
        .bucket(req.get_bucket())
        .set_acl(req.get_header(X_AMZ_ACL))
        .set_content_md5(req.get_header(H_CONTENT_MD5))
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_grant_full_control(req.get_header(X_AMZ_GRANT_FULL_CONTROL))
        .set_grant_read(req.get_header(X_AMZ_GRANT_READ))
        .set_grant_read_acp(req.get_header(X_AMZ_GRANT_READ_ACP))
        .set_grant_write(req.get_header(X_AMZ_GRANT_WRITE))
        .set_grant_write_acp(req.get_header(X_AMZ_GRANT_WRITE_ACP))
        .access_control_policy(decode_access_control_policy(req.take_body_bytes().await?)?)
        .build()?)
}

pub async fn get_bucket_tagging(req: &mut S3Request) -> Result<GetBucketTaggingInput, S3Error> {
    Ok(GetBucketTaggingInput::builder()
        .bucket(req.get_bucket())
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .build()?)
}

pub async fn put_bucket_tagging(req: &mut S3Request) -> Result<PutBucketTaggingInput, S3Error> {
    Ok(PutBucketTaggingInput::builder()
        .bucket(req.get_bucket())
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_content_md5(req.get_header(H_CONTENT_MD5))
        .tagging(decode_tagging(req.take_body_bytes().await?)?)
        .build()?)
}

pub async fn delete_bucket_tagging(
    req: &mut S3Request,
) -> Result<DeleteBucketTaggingInput, S3Error> {
    Ok(DeleteBucketTaggingInput::builder()
        .bucket(req.get_bucket())
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .build()?)
}

pub async fn get_object_lock_configuration(
    req: &mut S3Request,
) -> Result<GetObjectLockConfigurationInput, S3Error> {
    Ok(GetObjectLockConfigurationInput::builder()
        .bucket(req.get_bucket())
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .build()?)
}

pub async fn put_object_lock_configuration(
    req: &mut S3Request,
) -> Result<PutObjectLockConfigurationInput, S3Error> {
    Ok(PutObjectLockConfigurationInput::builder()
        .bucket(req.get_bucket())
        .set_content_md5(req.get_header(H_CONTENT_MD5))
        .set_expected_bucket_owner(req.get_header(X_AMZ_EXPECTED_BUCKET_OWNER))
        .set_request_payer(req.get_header(X_AMZ_REQUEST_PAYER))
        .set_token(req.get_header(X_AMZ_BUCKET_OBJECT_LOCK_TOKEN))
        .object_lock_configuration(decode_object_lock_configuration(
            req.take_body_bytes().await?,
        )?)
        .build()?)
}

/// This macro generates a default parser function per op,
/// to make it possible to implement it in stages.
macro_rules! gen {
    ($op:ident) => {
        paste::paste! {
            pub async fn [<$op:snake>](_: &mut S3Request) -> Result<[<$op Input>], S3Error> {
                Err(S3Error::builder()
                    .code("NotImplemented")
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

//-------------------------------//
// gen!(HeadBucket);
// gen!(CreateBucket);
// gen!(DeleteBucket);
// gen!(GetBucketLocation);
//-------------------------------//

//-------------------------------//
// gen!(GetObject);
// gen!(HeadObject);
// gen!(PutObject);
// gen!(CopyObject);
// gen!(DeleteObject);
// gen!(DeleteObjects);
//-------------------------------//

//-------------------------------//
// gen!(CreateMultipartUpload);
// gen!(CompleteMultipartUpload);
// gen!(AbortMultipartUpload);
// gen!(UploadPart);
// gen!(UploadPartCopy);
//-------------------------------//

//-------------------------------//
// gen!(ListBuckets);
// gen!(ListObjects);
// gen!(ListObjectsV2);
// gen!(ListObjectVersions);
// gen!(ListMultipartUploads);
// gen!(ListParts);
//-------------------------------//

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
