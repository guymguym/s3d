//! TODO This module should be generated from https://github.com/awslabs/smithy-rs

use crate::{
    err::*,
    gen::{api::S3Api, kinds::S3OpKind},
    types::*,
};

/// serve_s3_request parses the input, dispatches the request to the appropriate handler,
/// and writes the response.
pub async fn serve_s3_request<API: S3Api>(
    req: &S3Request,
    api: &API,
) -> Result<HttpResponse, ServerError> {
    if req.op_kind.is_none() {
        return Err(InputError::Unhandled(anyhow!("missing op_kind")).into());
    }
    let op_kind = req.op_kind.unwrap();

    /// This macro generates the serve code for each operation kind.
    macro_rules! gen {
        ($name:ident, $req:ident, $api:ident) => {
            paste::paste! {
                {
                    let input = crate::gen::input::[<$name:snake>]($req)?;
                    debug!("input {:?}", input);
                    let output = $api.[<$name:snake>](input).await.map_err(|err| err.meta().clone())?;
                    debug!("output {:?}", output);
                    let response = crate::gen::output::[<$name:snake>](output)?;
                    debug!("response {:?}", response);
                    Ok(response)
                }
            }
        };
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    match op_kind {
        S3OpKind::AbortMultipartUpload => gen!(AbortMultipartUpload, req, api),
        S3OpKind::CompleteMultipartUpload => gen!(CompleteMultipartUpload, req, api),
        S3OpKind::CopyObject => gen!(CopyObject, req, api),
        S3OpKind::CreateBucket => gen!(CreateBucket, req, api),
        S3OpKind::CreateMultipartUpload => gen!(CreateMultipartUpload, req, api),
        S3OpKind::DeleteBucket => gen!(DeleteBucket, req, api),
        S3OpKind::DeleteBucketAnalyticsConfiguration => gen!(DeleteBucketAnalyticsConfiguration, req, api),
        S3OpKind::DeleteBucketCors => gen!(DeleteBucketCors, req, api),
        S3OpKind::DeleteBucketEncryption => gen!(DeleteBucketEncryption, req, api),
        S3OpKind::DeleteBucketIntelligentTieringConfiguration => gen!(DeleteBucketIntelligentTieringConfiguration, req, api),
        S3OpKind::DeleteBucketInventoryConfiguration => gen!(DeleteBucketInventoryConfiguration, req, api),
        S3OpKind::DeleteBucketLifecycle => gen!(DeleteBucketLifecycle, req, api),
        S3OpKind::DeleteBucketMetricsConfiguration => gen!(DeleteBucketMetricsConfiguration, req, api),
        S3OpKind::DeleteBucketOwnershipControls => gen!(DeleteBucketOwnershipControls, req, api),
        S3OpKind::DeleteBucketPolicy => gen!(DeleteBucketPolicy, req, api),
        S3OpKind::DeleteBucketReplication => gen!(DeleteBucketReplication, req, api),
        S3OpKind::DeleteBucketTagging => gen!(DeleteBucketTagging, req, api),
        S3OpKind::DeleteBucketWebsite => gen!(DeleteBucketWebsite, req, api),
        S3OpKind::DeleteObject => gen!(DeleteObject, req, api),
        S3OpKind::DeleteObjects => gen!(DeleteObjects, req, api),
        S3OpKind::DeleteObjectTagging => gen!(DeleteObjectTagging, req, api),
        S3OpKind::DeletePublicAccessBlock => gen!(DeletePublicAccessBlock, req, api),
        S3OpKind::GetBucketAccelerateConfiguration => gen!(GetBucketAccelerateConfiguration, req, api),
        S3OpKind::GetBucketAcl => gen!(GetBucketAcl, req, api),
        S3OpKind::GetBucketAnalyticsConfiguration => gen!(GetBucketAnalyticsConfiguration, req, api),
        S3OpKind::GetBucketCors => gen!(GetBucketCors, req, api),
        S3OpKind::GetBucketEncryption => gen!(GetBucketEncryption, req, api),
        S3OpKind::GetBucketIntelligentTieringConfiguration => gen!(GetBucketIntelligentTieringConfiguration, req, api),
        S3OpKind::GetBucketInventoryConfiguration => gen!(GetBucketInventoryConfiguration, req, api),
        S3OpKind::GetBucketLifecycleConfiguration => gen!(GetBucketLifecycleConfiguration, req, api),
        S3OpKind::GetBucketLocation => gen!(GetBucketLocation, req, api),
        S3OpKind::GetBucketLogging => gen!(GetBucketLogging, req, api),
        S3OpKind::GetBucketMetricsConfiguration => gen!(GetBucketMetricsConfiguration, req, api),
        S3OpKind::GetBucketNotificationConfiguration => gen!(GetBucketNotificationConfiguration, req, api),
        S3OpKind::GetBucketOwnershipControls => gen!(GetBucketOwnershipControls, req, api),
        S3OpKind::GetBucketPolicy => gen!(GetBucketPolicy, req, api),
        S3OpKind::GetBucketPolicyStatus => gen!(GetBucketPolicyStatus, req, api),
        S3OpKind::GetBucketReplication => gen!(GetBucketReplication, req, api),
        S3OpKind::GetBucketRequestPayment => gen!(GetBucketRequestPayment, req, api),
        S3OpKind::GetBucketTagging => gen!(GetBucketTagging, req, api),
        S3OpKind::GetBucketVersioning => gen!(GetBucketVersioning, req, api),
        S3OpKind::GetBucketWebsite => gen!(GetBucketWebsite, req, api),
        S3OpKind::GetObject => gen!(GetObject, req, api),
        S3OpKind::GetObjectAcl => gen!(GetObjectAcl, req, api),
        S3OpKind::GetObjectLegalHold => gen!(GetObjectLegalHold, req, api),
        S3OpKind::GetObjectLockConfiguration => gen!(GetObjectLockConfiguration, req, api),
        S3OpKind::GetObjectRetention => gen!(GetObjectRetention, req, api),
        S3OpKind::GetObjectTagging => gen!(GetObjectTagging, req, api),
        S3OpKind::GetObjectTorrent => gen!(GetObjectTorrent, req, api),
        S3OpKind::GetPublicAccessBlock => gen!(GetPublicAccessBlock, req, api),
        S3OpKind::HeadBucket => gen!(HeadBucket, req, api),
        S3OpKind::HeadObject => gen!(HeadObject, req, api),
        S3OpKind::ListBucketAnalyticsConfigurations => gen!(ListBucketAnalyticsConfigurations, req, api),
        S3OpKind::ListBucketIntelligentTieringConfigurations => gen!(ListBucketIntelligentTieringConfigurations, req, api),
        S3OpKind::ListBucketInventoryConfigurations => gen!(ListBucketInventoryConfigurations, req, api),
        S3OpKind::ListBucketMetricsConfigurations => gen!(ListBucketMetricsConfigurations, req, api),
        S3OpKind::ListBuckets => gen!(ListBuckets, req, api),
        S3OpKind::ListMultipartUploads => gen!(ListMultipartUploads, req, api),
        S3OpKind::ListObjects => gen!(ListObjects, req, api),
        S3OpKind::ListObjectsV2 => gen!(ListObjectsV2, req, api),
        S3OpKind::ListObjectVersions => gen!(ListObjectVersions, req, api),
        S3OpKind::ListParts => gen!(ListParts, req, api),
        S3OpKind::PutBucketAccelerateConfiguration => gen!(PutBucketAccelerateConfiguration, req, api),
        S3OpKind::PutBucketAcl => gen!(PutBucketAcl, req, api),
        S3OpKind::PutBucketAnalyticsConfiguration => gen!(PutBucketAnalyticsConfiguration, req, api),
        S3OpKind::PutBucketCors => gen!(PutBucketCors, req, api),
        S3OpKind::PutBucketEncryption => gen!(PutBucketEncryption, req, api),
        S3OpKind::PutBucketIntelligentTieringConfiguration => gen!(PutBucketIntelligentTieringConfiguration, req, api),
        S3OpKind::PutBucketInventoryConfiguration => gen!(PutBucketInventoryConfiguration, req, api),
        S3OpKind::PutBucketLifecycleConfiguration => gen!(PutBucketLifecycleConfiguration, req, api),
        S3OpKind::PutBucketLogging => gen!(PutBucketLogging, req, api),
        S3OpKind::PutBucketMetricsConfiguration => gen!(PutBucketMetricsConfiguration, req, api),
        S3OpKind::PutBucketNotificationConfiguration => gen!(PutBucketNotificationConfiguration, req, api),
        S3OpKind::PutBucketOwnershipControls => gen!(PutBucketOwnershipControls, req, api),
        S3OpKind::PutBucketPolicy => gen!(PutBucketPolicy, req, api),
        S3OpKind::PutBucketReplication => gen!(PutBucketReplication, req, api),
        S3OpKind::PutBucketRequestPayment => gen!(PutBucketRequestPayment, req, api),
        S3OpKind::PutBucketTagging => gen!(PutBucketTagging, req, api),
        S3OpKind::PutBucketVersioning => gen!(PutBucketVersioning, req, api),
        S3OpKind::PutBucketWebsite => gen!(PutBucketWebsite, req, api),
        S3OpKind::PutObject => gen!(PutObject, req, api),
        S3OpKind::PutObjectAcl => gen!(PutObjectAcl, req, api),
        S3OpKind::PutObjectLegalHold => gen!(PutObjectLegalHold, req, api),
        S3OpKind::PutObjectLockConfiguration => gen!(PutObjectLockConfiguration, req, api),
        S3OpKind::PutObjectRetention => gen!(PutObjectRetention, req, api),
        S3OpKind::PutObjectTagging => gen!(PutObjectTagging, req, api),
        S3OpKind::PutPublicAccessBlock => gen!(PutPublicAccessBlock, req, api),
        S3OpKind::RestoreObject => gen!(RestoreObject, req, api),
        S3OpKind::SelectObjectContent => gen!(SelectObjectContent, req, api),
        S3OpKind::UploadPart => gen!(UploadPart, req, api),
        S3OpKind::UploadPartCopy => gen!(UploadPartCopy, req, api),
        S3OpKind::WriteGetObjectResponse => gen!(WriteGetObjectResponse, req, api),
    }
}
