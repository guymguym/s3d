/// TODO generate this file automatically with https://github.com/awslabs/smithy-rs
use crate::s3::{input::*, types::*};
use aws_sdk_s3::input::*;

pub struct S3InputImpl {}

impl S3Input for S3InputImpl {
    fn list_buckets(&self, req: S3Request) -> Result<ListBucketsInput, InputError> {
        ListBucketsInput::builder()
            .build()
            .map_err(|err| InputError::Unhandled(err.into()))
    }

    fn list_objects(&self, req: S3Request) -> Result<ListObjectsInput, InputError> {
        ListObjectsInput::builder()
            .set_bucket(Some(req.bucket.clone()))
            .set_delimiter(req.get_param("delimiter"))
            .set_marker(req.get_param("marker"))
            .set_prefix(req.get_param("prefix"))
            .set_max_keys(req.get_param_i32("max-keys"))
            // .set_encoding_type(i.expected_bucket_owner)
            // .set_expected_bucket_owner(i.expected_bucket_owner)
            // .set_request_payer(i.request_payer)
            .build()
            .map_err(|err| InputError::Unhandled(err.into()))
    }

    fn get_object(&self, req: S3Request) -> Result<GetObjectInput, InputError> {
        GetObjectInput::builder()
            .set_bucket(Some(req.bucket.clone()))
            .set_key(Some(req.key.clone()))
            .set_part_number(req.get_param_i32("partNumber"))
            .set_range(req.get_header("range"))
            .set_version_id(req.get_param("versionId"))
            .build()
            .map_err(|err| InputError::Unhandled(err.into()))
    }
}
