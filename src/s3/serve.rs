/// TODO generate this file automatically with https://github.com/awslabs/smithy-rs
use crate::{
    s3::{api::*, input::*, kind::*, output::*},
    types::*,
};
use paste::paste;

/// This macro generates the serve code for each operation kind.
macro_rules! s3_serve {
    ($name:ident, $req:ident, $inp:ident, $api:ident, $out:ident) => {
        paste! {
            {
                let input = $inp.[<$name:snake>]($req)?;
                let output = $api.[<$name:snake>](input).await.map_err(|err| err.meta().clone())?;
                let response = $out.[<$name:snake>](output)?;
                Ok(response)
            }
        }
    };
}

pub enum ServeError {
    InputError(InputError),
    S3Error(S3Error),
    OutputError(OutputError),
}
impl From<InputError> for ServeError {
    fn from(e: InputError) -> Self {
        ServeError::InputError(e)
    }
}
impl From<S3Error> for ServeError {
    fn from(e: S3Error) -> Self {
        ServeError::S3Error(e)
    }
}
impl From<OutputError> for ServeError {
    fn from(e: OutputError) -> Self {
        ServeError::OutputError(e)
    }
}

pub async fn serve_http<I: S3Input, A: S3Api, O: S3Output>(
    req: HttpRequest,
    op: S3OpKind,
    inp: &I,
    api: &A,
    out: &O,
) -> Result<HttpResponse, ServeError> {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    match op {
        S3OpKind::AbortMultipartUpload => s3_serve!(AbortMultipartUpload, req, inp, api, out),
        S3OpKind::CompleteMultipartUpload => s3_serve!(CompleteMultipartUpload, req, inp, api, out),
        S3OpKind::CopyObject => s3_serve!(CopyObject, req, inp, api, out),
        S3OpKind::CreateBucket => s3_serve!(CreateBucket, req, inp, api, out),
        S3OpKind::CreateMultipartUpload => s3_serve!(CreateMultipartUpload, req, inp, api, out),
        S3OpKind::DeleteBucket => s3_serve!(DeleteBucket, req, inp, api, out),
        S3OpKind::DeleteBucketAnalyticsConfiguration => s3_serve!(DeleteBucketAnalyticsConfiguration, req, inp, api, out),
        S3OpKind::DeleteBucketCors => s3_serve!(DeleteBucketCors, req, inp, api, out),
        S3OpKind::DeleteBucketEncryption => s3_serve!(DeleteBucketEncryption, req, inp, api, out),
        S3OpKind::DeleteBucketIntelligentTieringConfiguration => s3_serve!(DeleteBucketIntelligentTieringConfiguration, req, inp, api, out),
        S3OpKind::DeleteBucketInventoryConfiguration => s3_serve!(DeleteBucketInventoryConfiguration, req, inp, api, out),
        S3OpKind::DeleteBucketLifecycle => s3_serve!(DeleteBucketLifecycle, req, inp, api, out),
        S3OpKind::DeleteBucketMetricsConfiguration => s3_serve!(DeleteBucketMetricsConfiguration, req, inp, api, out),
        S3OpKind::DeleteBucketOwnershipControls => s3_serve!(DeleteBucketOwnershipControls, req, inp, api, out),
        S3OpKind::DeleteBucketPolicy => s3_serve!(DeleteBucketPolicy, req, inp, api, out),
        S3OpKind::DeleteBucketReplication => s3_serve!(DeleteBucketReplication, req, inp, api, out),
        S3OpKind::DeleteBucketTagging => s3_serve!(DeleteBucketTagging, req, inp, api, out),
        S3OpKind::DeleteBucketWebsite => s3_serve!(DeleteBucketWebsite, req, inp, api, out),
        S3OpKind::DeleteObject => s3_serve!(DeleteObject, req, inp, api, out),
        S3OpKind::DeleteObjects => s3_serve!(DeleteObjects, req, inp, api, out),
        S3OpKind::DeleteObjectTagging => s3_serve!(DeleteObjectTagging, req, inp, api, out),
        S3OpKind::DeletePublicAccessBlock => s3_serve!(DeletePublicAccessBlock, req, inp, api, out),
        S3OpKind::GetBucketAccelerateConfiguration => s3_serve!(GetBucketAccelerateConfiguration, req, inp, api, out),
        S3OpKind::GetBucketAcl => s3_serve!(GetBucketAcl, req, inp, api, out),
        S3OpKind::GetBucketAnalyticsConfiguration => s3_serve!(GetBucketAnalyticsConfiguration, req, inp, api, out),
        S3OpKind::GetBucketCors => s3_serve!(GetBucketCors, req, inp, api, out),
        S3OpKind::GetBucketEncryption => s3_serve!(GetBucketEncryption, req, inp, api, out),
        S3OpKind::GetBucketIntelligentTieringConfiguration => s3_serve!(GetBucketIntelligentTieringConfiguration, req, inp, api, out),
        S3OpKind::GetBucketInventoryConfiguration => s3_serve!(GetBucketInventoryConfiguration, req, inp, api, out),
        S3OpKind::GetBucketLifecycleConfiguration => s3_serve!(GetBucketLifecycleConfiguration, req, inp, api, out),
        S3OpKind::GetBucketLocation => s3_serve!(GetBucketLocation, req, inp, api, out),
        S3OpKind::GetBucketLogging => s3_serve!(GetBucketLogging, req, inp, api, out),
        S3OpKind::GetBucketMetricsConfiguration => s3_serve!(GetBucketMetricsConfiguration, req, inp, api, out),
        S3OpKind::GetBucketNotificationConfiguration => s3_serve!(GetBucketNotificationConfiguration, req, inp, api, out),
        S3OpKind::GetBucketOwnershipControls => s3_serve!(GetBucketOwnershipControls, req, inp, api, out),
        S3OpKind::GetBucketPolicy => s3_serve!(GetBucketPolicy, req, inp, api, out),
        S3OpKind::GetBucketPolicyStatus => s3_serve!(GetBucketPolicyStatus, req, inp, api, out),
        S3OpKind::GetBucketReplication => s3_serve!(GetBucketReplication, req, inp, api, out),
        S3OpKind::GetBucketRequestPayment => s3_serve!(GetBucketRequestPayment, req, inp, api, out),
        S3OpKind::GetBucketTagging => s3_serve!(GetBucketTagging, req, inp, api, out),
        S3OpKind::GetBucketVersioning => s3_serve!(GetBucketVersioning, req, inp, api, out),
        S3OpKind::GetBucketWebsite => s3_serve!(GetBucketWebsite, req, inp, api, out),
        S3OpKind::GetObject => s3_serve!(GetObject, req, inp, api, out),
        S3OpKind::GetObjectAcl => s3_serve!(GetObjectAcl, req, inp, api, out),
        S3OpKind::GetObjectLegalHold => s3_serve!(GetObjectLegalHold, req, inp, api, out),
        S3OpKind::GetObjectLockConfiguration => s3_serve!(GetObjectLockConfiguration, req, inp, api, out),
        S3OpKind::GetObjectRetention => s3_serve!(GetObjectRetention, req, inp, api, out),
        S3OpKind::GetObjectTagging => s3_serve!(GetObjectTagging, req, inp, api, out),
        S3OpKind::GetObjectTorrent => s3_serve!(GetObjectTorrent, req, inp, api, out),
        S3OpKind::GetPublicAccessBlock => s3_serve!(GetPublicAccessBlock, req, inp, api, out),
        S3OpKind::HeadBucket => s3_serve!(HeadBucket, req, inp, api, out),
        S3OpKind::HeadObject => s3_serve!(HeadObject, req, inp, api, out),
        S3OpKind::ListBucketAnalyticsConfigurations => s3_serve!(ListBucketAnalyticsConfigurations, req, inp, api, out),
        S3OpKind::ListBucketIntelligentTieringConfigurations => s3_serve!(ListBucketIntelligentTieringConfigurations, req, inp, api, out),
        S3OpKind::ListBucketInventoryConfigurations => s3_serve!(ListBucketInventoryConfigurations, req, inp, api, out),
        S3OpKind::ListBucketMetricsConfigurations => s3_serve!(ListBucketMetricsConfigurations, req, inp, api, out),
        S3OpKind::ListBuckets => s3_serve!(ListBuckets, req, inp, api, out),
        S3OpKind::ListMultipartUploads => s3_serve!(ListMultipartUploads, req, inp, api, out),
        S3OpKind::ListObjects => s3_serve!(ListObjects, req, inp, api, out),
        S3OpKind::ListObjectsV2 => s3_serve!(ListObjectsV2, req, inp, api, out),
        S3OpKind::ListObjectVersions => s3_serve!(ListObjectVersions, req, inp, api, out),
        S3OpKind::ListParts => s3_serve!(ListParts, req, inp, api, out),
        S3OpKind::PutBucketAccelerateConfiguration => s3_serve!(PutBucketAccelerateConfiguration, req, inp, api, out),
        S3OpKind::PutBucketAcl => s3_serve!(PutBucketAcl, req, inp, api, out),
        S3OpKind::PutBucketAnalyticsConfiguration => s3_serve!(PutBucketAnalyticsConfiguration, req, inp, api, out),
        S3OpKind::PutBucketCors => s3_serve!(PutBucketCors, req, inp, api, out),
        S3OpKind::PutBucketEncryption => s3_serve!(PutBucketEncryption, req, inp, api, out),
        S3OpKind::PutBucketIntelligentTieringConfiguration => s3_serve!(PutBucketIntelligentTieringConfiguration, req, inp, api, out),
        S3OpKind::PutBucketInventoryConfiguration => s3_serve!(PutBucketInventoryConfiguration, req, inp, api, out),
        S3OpKind::PutBucketLifecycleConfiguration => s3_serve!(PutBucketLifecycleConfiguration, req, inp, api, out),
        S3OpKind::PutBucketLogging => s3_serve!(PutBucketLogging, req, inp, api, out),
        S3OpKind::PutBucketMetricsConfiguration => s3_serve!(PutBucketMetricsConfiguration, req, inp, api, out),
        S3OpKind::PutBucketNotificationConfiguration => s3_serve!(PutBucketNotificationConfiguration, req, inp, api, out),
        S3OpKind::PutBucketOwnershipControls => s3_serve!(PutBucketOwnershipControls, req, inp, api, out),
        S3OpKind::PutBucketPolicy => s3_serve!(PutBucketPolicy, req, inp, api, out),
        S3OpKind::PutBucketReplication => s3_serve!(PutBucketReplication, req, inp, api, out),
        S3OpKind::PutBucketRequestPayment => s3_serve!(PutBucketRequestPayment, req, inp, api, out),
        S3OpKind::PutBucketTagging => s3_serve!(PutBucketTagging, req, inp, api, out),
        S3OpKind::PutBucketVersioning => s3_serve!(PutBucketVersioning, req, inp, api, out),
        S3OpKind::PutBucketWebsite => s3_serve!(PutBucketWebsite, req, inp, api, out),
        S3OpKind::PutObject => s3_serve!(PutObject, req, inp, api, out),
        S3OpKind::PutObjectAcl => s3_serve!(PutObjectAcl, req, inp, api, out),
        S3OpKind::PutObjectLegalHold => s3_serve!(PutObjectLegalHold, req, inp, api, out),
        S3OpKind::PutObjectLockConfiguration => s3_serve!(PutObjectLockConfiguration, req, inp, api, out),
        S3OpKind::PutObjectRetention => s3_serve!(PutObjectRetention, req, inp, api, out),
        S3OpKind::PutObjectTagging => s3_serve!(PutObjectTagging, req, inp, api, out),
        S3OpKind::PutPublicAccessBlock => s3_serve!(PutPublicAccessBlock, req, inp, api, out),
        S3OpKind::RestoreObject => s3_serve!(RestoreObject, req, inp, api, out),
        S3OpKind::SelectObjectContent => s3_serve!(SelectObjectContent, req, inp, api, out),
        S3OpKind::UploadPart => s3_serve!(UploadPart, req, inp, api, out),
        S3OpKind::UploadPartCopy => s3_serve!(UploadPartCopy, req, inp, api, out),
        S3OpKind::WriteGetObjectResponse => s3_serve!(WriteGetObjectResponse, req, inp, api, out),
        _ => Err(ServeError::InputError(InputError::Unhandled(anyhow!("Unhandled operation")))),
    }
}
