use crate::{s3::*, types::*};
use aws_sdk_s3::{error::*, input::*, output::*};
use aws_smithy_http::result::SdkError;

#[derive(Debug)]
pub struct S3ApiToClient {
    pub s3c: S3C,
    pub ac: AC,
}

/// This macro generates a default function for each op.
macro_rules! s3_api {
    ($name:ident) => {
        paste::paste! {
            fn [<$name:snake>](&self, i: [<$name Input>]) -> TraitFuture<[<$name Output>], [<$name Error>]> {
                Box::pin(async {
                    self.ac
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
    s3_api!(AbortMultipartUpload);
    s3_api!(CompleteMultipartUpload);
    s3_api!(CopyObject);
    s3_api!(CreateBucket);
    s3_api!(CreateMultipartUpload);
    s3_api!(DeleteBucket);
    s3_api!(DeleteBucketAnalyticsConfiguration);
    s3_api!(DeleteBucketCors);
    s3_api!(DeleteBucketEncryption);
    s3_api!(DeleteBucketIntelligentTieringConfiguration);
    s3_api!(DeleteBucketInventoryConfiguration);
    s3_api!(DeleteBucketLifecycle);
    s3_api!(DeleteBucketMetricsConfiguration);
    s3_api!(DeleteBucketOwnershipControls);
    s3_api!(DeleteBucketPolicy);
    s3_api!(DeleteBucketReplication);
    s3_api!(DeleteBucketTagging);
    s3_api!(DeleteBucketWebsite);
    s3_api!(DeleteObject);
    s3_api!(DeleteObjects);
    s3_api!(DeleteObjectTagging);
    s3_api!(DeletePublicAccessBlock);
    s3_api!(GetBucketAccelerateConfiguration);
    s3_api!(GetBucketAcl);
    s3_api!(GetBucketAnalyticsConfiguration);
    s3_api!(GetBucketCors);
    s3_api!(GetBucketEncryption);
    s3_api!(GetBucketIntelligentTieringConfiguration);
    s3_api!(GetBucketInventoryConfiguration);
    s3_api!(GetBucketLifecycleConfiguration);
    s3_api!(GetBucketLocation);
    s3_api!(GetBucketLogging);
    s3_api!(GetBucketMetricsConfiguration);
    s3_api!(GetBucketNotificationConfiguration);
    s3_api!(GetBucketOwnershipControls);
    s3_api!(GetBucketPolicy);
    s3_api!(GetBucketPolicyStatus);
    s3_api!(GetBucketReplication);
    s3_api!(GetBucketRequestPayment);
    s3_api!(GetBucketTagging);
    s3_api!(GetBucketVersioning);
    s3_api!(GetBucketWebsite);
    s3_api!(GetObject);
    s3_api!(GetObjectAcl);
    s3_api!(GetObjectLegalHold);
    s3_api!(GetObjectLockConfiguration);
    s3_api!(GetObjectRetention);
    s3_api!(GetObjectTagging);
    s3_api!(GetObjectTorrent);
    s3_api!(GetPublicAccessBlock);
    s3_api!(HeadBucket);
    s3_api!(HeadObject);
    s3_api!(ListBucketAnalyticsConfigurations);
    s3_api!(ListBucketIntelligentTieringConfigurations);
    s3_api!(ListBucketInventoryConfigurations);
    s3_api!(ListBucketMetricsConfigurations);
    s3_api!(ListBuckets);
    s3_api!(ListMultipartUploads);
    s3_api!(ListObjects);
    s3_api!(ListObjectsV2);
    s3_api!(ListObjectVersions);
    s3_api!(ListParts);
    s3_api!(PutBucketAccelerateConfiguration);
    s3_api!(PutBucketAcl);
    s3_api!(PutBucketAnalyticsConfiguration);
    s3_api!(PutBucketCors);
    s3_api!(PutBucketEncryption);
    s3_api!(PutBucketIntelligentTieringConfiguration);
    s3_api!(PutBucketInventoryConfiguration);
    s3_api!(PutBucketLifecycleConfiguration);
    s3_api!(PutBucketLogging);
    s3_api!(PutBucketMetricsConfiguration);
    s3_api!(PutBucketNotificationConfiguration);
    s3_api!(PutBucketOwnershipControls);
    s3_api!(PutBucketPolicy);
    s3_api!(PutBucketReplication);
    s3_api!(PutBucketRequestPayment);
    s3_api!(PutBucketTagging);
    s3_api!(PutBucketVersioning);
    s3_api!(PutBucketWebsite);
    s3_api!(PutObject);
    s3_api!(PutObjectAcl);
    s3_api!(PutObjectLegalHold);
    s3_api!(PutObjectLockConfiguration);
    s3_api!(PutObjectRetention);
    s3_api!(PutObjectTagging);
    s3_api!(PutPublicAccessBlock);
    s3_api!(RestoreObject);
    s3_api!(SelectObjectContent);
    s3_api!(UploadPart);
    s3_api!(UploadPartCopy);
    s3_api!(WriteGetObjectResponse);
}
