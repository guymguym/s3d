use crate::{
    s3::{api::*, kind::*},
    types::*,
};
use async_trait::async_trait;
use paste::paste;

/// This macro defines a struct for each operation with the input/output/error types
/// and functions that read/write those from http request/response and call its api method.
macro_rules! s3_op {
    ($name:ident) => {
        paste! {
            pub struct [<$name Op>] {}
            #[async_trait]
            impl S3Op for [<$name Op>] {
                const KIND: S3OpKind = S3OpKind::$name;
                type Input = aws_sdk_s3::input::[<$name Input>];
                type Output = aws_sdk_s3::output::[<$name Output>];
                type Error = aws_sdk_s3::error::[<$name Error>];
                async fn call(api: &dyn S3Api, input: Self::Input) -> Result<Self::Output, Self::Error> {
                    api.[<$name:snake>](input).await
                }
            }
        }
    };
}

#[async_trait]
pub trait S3Op {
    const KIND: S3OpKind;
    type Input;
    type Output;
    type Error;

    async fn call(api: &dyn S3Api, input: Self::Input) -> Result<Self::Output, Self::Error>;

    fn input(_req: HttpRequest) -> Result<Self::Input, InputError> {
        Err(InputError::Unknown)
    }

    fn output(_output: Self::Output) -> Result<HttpResponse, OutputError> {
        Err(OutputError::Unknown)
    }
}

pub enum InputError {
    Unknown,
}
pub enum OutputError {
    Unknown,
}

s3_op!(AbortMultipartUpload);
s3_op!(CompleteMultipartUpload);
s3_op!(CopyObject);
s3_op!(CreateBucket);
s3_op!(CreateMultipartUpload);
s3_op!(DeleteBucket);
s3_op!(DeleteBucketAnalyticsConfiguration);
s3_op!(DeleteBucketCors);
s3_op!(DeleteBucketEncryption);
s3_op!(DeleteBucketIntelligentTieringConfiguration);
s3_op!(DeleteBucketInventoryConfiguration);
s3_op!(DeleteBucketLifecycle);
s3_op!(DeleteBucketMetricsConfiguration);
s3_op!(DeleteBucketOwnershipControls);
s3_op!(DeleteBucketPolicy);
s3_op!(DeleteBucketReplication);
s3_op!(DeleteBucketTagging);
s3_op!(DeleteBucketWebsite);
s3_op!(DeleteObject);
s3_op!(DeleteObjects);
s3_op!(DeleteObjectTagging);
s3_op!(DeletePublicAccessBlock);
s3_op!(GetBucketAccelerateConfiguration);
s3_op!(GetBucketAcl);
s3_op!(GetBucketAnalyticsConfiguration);
s3_op!(GetBucketCors);
s3_op!(GetBucketEncryption);
s3_op!(GetBucketIntelligentTieringConfiguration);
s3_op!(GetBucketInventoryConfiguration);
s3_op!(GetBucketLifecycleConfiguration);
s3_op!(GetBucketLocation);
s3_op!(GetBucketLogging);
s3_op!(GetBucketMetricsConfiguration);
s3_op!(GetBucketNotificationConfiguration);
s3_op!(GetBucketOwnershipControls);
s3_op!(GetBucketPolicy);
s3_op!(GetBucketPolicyStatus);
s3_op!(GetBucketReplication);
s3_op!(GetBucketRequestPayment);
s3_op!(GetBucketTagging);
s3_op!(GetBucketVersioning);
s3_op!(GetBucketWebsite);
s3_op!(GetObject);
s3_op!(GetObjectAcl);
s3_op!(GetObjectLegalHold);
s3_op!(GetObjectLockConfiguration);
s3_op!(GetObjectRetention);
s3_op!(GetObjectTagging);
s3_op!(GetObjectTorrent);
s3_op!(GetPublicAccessBlock);
s3_op!(HeadBucket);
s3_op!(HeadObject);
s3_op!(ListBucketAnalyticsConfigurations);
s3_op!(ListBucketIntelligentTieringConfigurations);
s3_op!(ListBucketInventoryConfigurations);
s3_op!(ListBucketMetricsConfigurations);
s3_op!(ListBuckets);
s3_op!(ListMultipartUploads);
s3_op!(ListObjects);
s3_op!(ListObjectsV2);
s3_op!(ListObjectVersions);
s3_op!(ListParts);
s3_op!(PutBucketAccelerateConfiguration);
s3_op!(PutBucketAcl);
s3_op!(PutBucketAnalyticsConfiguration);
s3_op!(PutBucketCors);
s3_op!(PutBucketEncryption);
s3_op!(PutBucketIntelligentTieringConfiguration);
s3_op!(PutBucketInventoryConfiguration);
s3_op!(PutBucketLifecycleConfiguration);
s3_op!(PutBucketLogging);
s3_op!(PutBucketMetricsConfiguration);
s3_op!(PutBucketNotificationConfiguration);
s3_op!(PutBucketOwnershipControls);
s3_op!(PutBucketPolicy);
s3_op!(PutBucketReplication);
s3_op!(PutBucketRequestPayment);
s3_op!(PutBucketTagging);
s3_op!(PutBucketVersioning);
s3_op!(PutBucketWebsite);
s3_op!(PutObject);
s3_op!(PutObjectAcl);
s3_op!(PutObjectLegalHold);
s3_op!(PutObjectLockConfiguration);
s3_op!(PutObjectRetention);
s3_op!(PutObjectTagging);
s3_op!(PutPublicAccessBlock);
s3_op!(RestoreObject);
s3_op!(SelectObjectContent);
s3_op!(UploadPart);
s3_op!(UploadPartCopy);
s3_op!(WriteGetObjectResponse);
