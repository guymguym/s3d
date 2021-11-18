use crate::{
    s3::{api::*, kind::*, op::*},
    types::*,
};
use paste::paste;

/// This macro generates the serve code for each operation kind.
macro_rules! s3_serve {
    ($req:ident, $api:ident, $name:ident) => {
        paste! {
            {
                let input = [<$name Op>]::input($req)?;
                let output = [<$name Op>]::call($api, input).await.map_err(|err| err.meta().clone())?;
                let response = [<$name Op>]::output(output)?;
                Ok(response)
            }
        }
    }
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


#[cfg_attr(rustfmt, rustfmt_skip)]
pub async fn serve_http(api: &dyn S3Api, op: S3OpKind, req: HttpRequest) -> Result<HttpResponse, ServeError> {
    match op {
        S3OpKind::AbortMultipartUpload => s3_serve!(req, api, AbortMultipartUpload),
        S3OpKind::CompleteMultipartUpload => s3_serve!(req, api, CompleteMultipartUpload),
        S3OpKind::CopyObject => s3_serve!(req, api, CopyObject),
        S3OpKind::CreateBucket => s3_serve!(req, api, CreateBucket),
        S3OpKind::CreateMultipartUpload => s3_serve!(req, api, CreateMultipartUpload),
        S3OpKind::DeleteBucket => s3_serve!(req, api, DeleteBucket),
        S3OpKind::DeleteBucketAnalyticsConfiguration => s3_serve!(req, api, DeleteBucketAnalyticsConfiguration),
        S3OpKind::DeleteBucketCors => s3_serve!(req, api, DeleteBucketCors),
        S3OpKind::DeleteBucketEncryption => s3_serve!(req, api, DeleteBucketEncryption),
        S3OpKind::DeleteBucketIntelligentTieringConfiguration => s3_serve!(req, api, DeleteBucketIntelligentTieringConfiguration),
        S3OpKind::DeleteBucketInventoryConfiguration => s3_serve!(req, api, DeleteBucketInventoryConfiguration),
        S3OpKind::DeleteBucketLifecycle => s3_serve!(req, api, DeleteBucketLifecycle),
        S3OpKind::DeleteBucketMetricsConfiguration => s3_serve!(req, api, DeleteBucketMetricsConfiguration),
        S3OpKind::DeleteBucketOwnershipControls => s3_serve!(req, api, DeleteBucketOwnershipControls),
        S3OpKind::DeleteBucketPolicy => s3_serve!(req, api, DeleteBucketPolicy),
        S3OpKind::DeleteBucketReplication => s3_serve!(req, api, DeleteBucketReplication),
        S3OpKind::DeleteBucketTagging => s3_serve!(req, api, DeleteBucketTagging),
        S3OpKind::DeleteBucketWebsite => s3_serve!(req, api, DeleteBucketWebsite),
        S3OpKind::DeleteObject => s3_serve!(req, api, DeleteObject),
        S3OpKind::DeleteObjects => s3_serve!(req, api, DeleteObjects),
        S3OpKind::DeleteObjectTagging => s3_serve!(req, api, DeleteObjectTagging),
        S3OpKind::DeletePublicAccessBlock => s3_serve!(req, api, DeletePublicAccessBlock),
        S3OpKind::GetBucketAccelerateConfiguration => s3_serve!(req, api, GetBucketAccelerateConfiguration),
        S3OpKind::GetBucketAcl => s3_serve!(req, api, GetBucketAcl),
        S3OpKind::GetBucketAnalyticsConfiguration => s3_serve!(req, api, GetBucketAnalyticsConfiguration),
        S3OpKind::GetBucketCors => s3_serve!(req, api, GetBucketCors),
        S3OpKind::GetBucketEncryption => s3_serve!(req, api, GetBucketEncryption),
        S3OpKind::GetBucketIntelligentTieringConfiguration => s3_serve!(req, api, GetBucketIntelligentTieringConfiguration),
        S3OpKind::GetBucketInventoryConfiguration => s3_serve!(req, api, GetBucketInventoryConfiguration),
        S3OpKind::GetBucketLifecycleConfiguration => s3_serve!(req, api, GetBucketLifecycleConfiguration),
        S3OpKind::GetBucketLocation => s3_serve!(req, api, GetBucketLocation),
        S3OpKind::GetBucketLogging => s3_serve!(req, api, GetBucketLogging),
        S3OpKind::GetBucketMetricsConfiguration => s3_serve!(req, api, GetBucketMetricsConfiguration),
        S3OpKind::GetBucketNotificationConfiguration => s3_serve!(req, api, GetBucketNotificationConfiguration),
        S3OpKind::GetBucketOwnershipControls => s3_serve!(req, api, GetBucketOwnershipControls),
        S3OpKind::GetBucketPolicy => s3_serve!(req, api, GetBucketPolicy),
        S3OpKind::GetBucketPolicyStatus => s3_serve!(req, api, GetBucketPolicyStatus),
        S3OpKind::GetBucketReplication => s3_serve!(req, api, GetBucketReplication),
        S3OpKind::GetBucketRequestPayment => s3_serve!(req, api, GetBucketRequestPayment),
        S3OpKind::GetBucketTagging => s3_serve!(req, api, GetBucketTagging),
        S3OpKind::GetBucketVersioning => s3_serve!(req, api, GetBucketVersioning),
        S3OpKind::GetBucketWebsite => s3_serve!(req, api, GetBucketWebsite),
        S3OpKind::GetObject => s3_serve!(req, api, GetObject),
        S3OpKind::GetObjectAcl => s3_serve!(req, api, GetObjectAcl),
        S3OpKind::GetObjectLegalHold => s3_serve!(req, api, GetObjectLegalHold),
        S3OpKind::GetObjectLockConfiguration => s3_serve!(req, api, GetObjectLockConfiguration),
        S3OpKind::GetObjectRetention => s3_serve!(req, api, GetObjectRetention),
        S3OpKind::GetObjectTagging => s3_serve!(req, api, GetObjectTagging),
        S3OpKind::GetObjectTorrent => s3_serve!(req, api, GetObjectTorrent),
        S3OpKind::GetPublicAccessBlock => s3_serve!(req, api, GetPublicAccessBlock),
        S3OpKind::HeadBucket => s3_serve!(req, api, HeadBucket),
        S3OpKind::HeadObject => s3_serve!(req, api, HeadObject),
        S3OpKind::ListBucketAnalyticsConfigurations => s3_serve!(req, api, ListBucketAnalyticsConfigurations),
        S3OpKind::ListBucketIntelligentTieringConfigurations => s3_serve!(req, api, ListBucketIntelligentTieringConfigurations),
        S3OpKind::ListBucketInventoryConfigurations => s3_serve!(req, api, ListBucketInventoryConfigurations),
        S3OpKind::ListBucketMetricsConfigurations => s3_serve!(req, api, ListBucketMetricsConfigurations),
        S3OpKind::ListBuckets => s3_serve!(req, api, ListBuckets),
        S3OpKind::ListMultipartUploads => s3_serve!(req, api, ListMultipartUploads),
        S3OpKind::ListObjects => s3_serve!(req, api, ListObjects),
        S3OpKind::ListObjectsV2 => s3_serve!(req, api, ListObjectsV2),
        S3OpKind::ListObjectVersions => s3_serve!(req, api, ListObjectVersions),
        S3OpKind::ListParts => s3_serve!(req, api, ListParts),
        S3OpKind::PutBucketAccelerateConfiguration => s3_serve!(req, api, PutBucketAccelerateConfiguration),
        S3OpKind::PutBucketAcl => s3_serve!(req, api, PutBucketAcl),
        S3OpKind::PutBucketAnalyticsConfiguration => s3_serve!(req, api, PutBucketAnalyticsConfiguration),
        S3OpKind::PutBucketCors => s3_serve!(req, api, PutBucketCors),
        S3OpKind::PutBucketEncryption => s3_serve!(req, api, PutBucketEncryption),
        S3OpKind::PutBucketIntelligentTieringConfiguration => s3_serve!(req, api, PutBucketIntelligentTieringConfiguration),
        S3OpKind::PutBucketInventoryConfiguration => s3_serve!(req, api, PutBucketInventoryConfiguration),
        S3OpKind::PutBucketLifecycleConfiguration => s3_serve!(req, api, PutBucketLifecycleConfiguration),
        S3OpKind::PutBucketLogging => s3_serve!(req, api, PutBucketLogging),
        S3OpKind::PutBucketMetricsConfiguration => s3_serve!(req, api, PutBucketMetricsConfiguration),
        S3OpKind::PutBucketNotificationConfiguration => s3_serve!(req, api, PutBucketNotificationConfiguration),
        S3OpKind::PutBucketOwnershipControls => s3_serve!(req, api, PutBucketOwnershipControls),
        S3OpKind::PutBucketPolicy => s3_serve!(req, api, PutBucketPolicy),
        S3OpKind::PutBucketReplication => s3_serve!(req, api, PutBucketReplication),
        S3OpKind::PutBucketRequestPayment => s3_serve!(req, api, PutBucketRequestPayment),
        S3OpKind::PutBucketTagging => s3_serve!(req, api, PutBucketTagging),
        S3OpKind::PutBucketVersioning => s3_serve!(req, api, PutBucketVersioning),
        S3OpKind::PutBucketWebsite => s3_serve!(req, api, PutBucketWebsite),
        S3OpKind::PutObject => s3_serve!(req, api, PutObject),
        S3OpKind::PutObjectAcl => s3_serve!(req, api, PutObjectAcl),
        S3OpKind::PutObjectLegalHold => s3_serve!(req, api, PutObjectLegalHold),
        S3OpKind::PutObjectLockConfiguration => s3_serve!(req, api, PutObjectLockConfiguration),
        S3OpKind::PutObjectRetention => s3_serve!(req, api, PutObjectRetention),
        S3OpKind::PutObjectTagging => s3_serve!(req, api, PutObjectTagging),
        S3OpKind::PutPublicAccessBlock => s3_serve!(req, api, PutPublicAccessBlock),
        S3OpKind::RestoreObject => s3_serve!(req, api, RestoreObject),
        S3OpKind::SelectObjectContent => s3_serve!(req, api, SelectObjectContent),
        S3OpKind::UploadPart => s3_serve!(req, api, UploadPart),
        S3OpKind::UploadPartCopy => s3_serve!(req, api, UploadPartCopy),
        S3OpKind::WriteGetObjectResponse => s3_serve!(req, api, WriteGetObjectResponse),
        _ => Err(ServeError::InputError(InputError::Unknown)),
    }
}
