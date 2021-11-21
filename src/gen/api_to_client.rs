use crate::{gen::*, types::*};
use aws_sdk_s3::{error::*, input::*, output::*};
use aws_smithy_http::result::SdkError;

/// S3ApiToClient is the default implementation of S3Api that uses the client SDK to call for each function.
/// This can be connected to any remote S3 service directly.
#[derive(Debug)]
pub struct S3ApiToClient {
    pub s3c: S3C,
    pub smc: SMC,
}

/// This macro generates a default function for each op.
macro_rules! gen {
    ($name:ident) => {
        paste::paste! {
            fn [<$name:snake>](&self, i: [<$name Input>]) -> TraitFuture<[<$name Output>], [<$name Error>]> {
                Box::pin(async move {
                    self.smc
                        .call(i.make_operation(self.s3c.conf()).await.unwrap())
                        .await
                        .map_err(|err| match err {
                            SdkError::ServiceError { err, .. } => err,
                            _ => [<$name Error>]::unhandled(err),
                        })
                })
            }
        }
    };
}

impl S3Api for S3ApiToClient {
    gen!(AbortMultipartUpload);
    gen!(CompleteMultipartUpload);
    gen!(CopyObject);
    gen!(CreateBucket);
    gen!(CreateMultipartUpload);
    gen!(DeleteBucket);
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
    gen!(DeleteBucketTagging);
    gen!(DeleteBucketWebsite);
    gen!(DeleteObject);
    gen!(DeleteObjects);
    gen!(DeleteObjectTagging);
    gen!(DeletePublicAccessBlock);
    gen!(GetBucketAccelerateConfiguration);
    gen!(GetBucketAcl);
    gen!(GetBucketAnalyticsConfiguration);
    gen!(GetBucketCors);
    gen!(GetBucketEncryption);
    gen!(GetBucketIntelligentTieringConfiguration);
    gen!(GetBucketInventoryConfiguration);
    gen!(GetBucketLifecycleConfiguration);
    gen!(GetBucketLocation);
    gen!(GetBucketLogging);
    gen!(GetBucketMetricsConfiguration);
    gen!(GetBucketNotificationConfiguration);
    gen!(GetBucketOwnershipControls);
    gen!(GetBucketPolicy);
    gen!(GetBucketPolicyStatus);
    gen!(GetBucketReplication);
    gen!(GetBucketRequestPayment);
    gen!(GetBucketTagging);
    gen!(GetBucketVersioning);
    gen!(GetBucketWebsite);
    gen!(GetObject);
    gen!(GetObjectAcl);
    gen!(GetObjectLegalHold);
    gen!(GetObjectLockConfiguration);
    gen!(GetObjectRetention);
    gen!(GetObjectTagging);
    gen!(GetObjectTorrent);
    gen!(GetPublicAccessBlock);
    gen!(HeadBucket);
    gen!(HeadObject);
    gen!(ListBucketAnalyticsConfigurations);
    gen!(ListBucketIntelligentTieringConfigurations);
    gen!(ListBucketInventoryConfigurations);
    gen!(ListBucketMetricsConfigurations);
    gen!(ListBuckets);
    gen!(ListMultipartUploads);
    gen!(ListObjects);
    gen!(ListObjectsV2);
    gen!(ListObjectVersions);
    gen!(ListParts);
    gen!(PutBucketAccelerateConfiguration);
    gen!(PutBucketAcl);
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
    gen!(PutBucketTagging);
    gen!(PutBucketVersioning);
    gen!(PutBucketWebsite);
    gen!(PutObject);
    gen!(PutObjectAcl);
    gen!(PutObjectLegalHold);
    gen!(PutObjectLockConfiguration);
    gen!(PutObjectRetention);
    gen!(PutObjectTagging);
    gen!(PutPublicAccessBlock);
    gen!(RestoreObject);
    gen!(SelectObjectContent);
    gen!(UploadPart);
    gen!(UploadPartCopy);
    gen!(WriteGetObjectResponse);
}
