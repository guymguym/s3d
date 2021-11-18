/// TODO generate this file automatically with https://github.com/awslabs/smithy-rs

/// This macro defines a trait with functions for each of the S3 API methods.
macro_rules! s3_output {
    ($name:ident) => {
        paste::paste! {
            fn [<$name:snake>](
                &self,
                _output: aws_sdk_s3::output::[<$name Output>]
            ) -> Result<
                crate::s3::types::HttpResponse,
                crate::s3::types::OutputError
            > {
                Err(crate::s3::types::OutputError::NotImplemented)
            }
        }
    };
}

pub trait S3Output: Sync + Send {
    s3_output!(AbortMultipartUpload);
    s3_output!(CompleteMultipartUpload);
    s3_output!(CopyObject);
    s3_output!(CreateBucket);
    s3_output!(CreateMultipartUpload);
    s3_output!(DeleteBucket);
    s3_output!(DeleteBucketAnalyticsConfiguration);
    s3_output!(DeleteBucketCors);
    s3_output!(DeleteBucketEncryption);
    s3_output!(DeleteBucketIntelligentTieringConfiguration);
    s3_output!(DeleteBucketInventoryConfiguration);
    s3_output!(DeleteBucketLifecycle);
    s3_output!(DeleteBucketMetricsConfiguration);
    s3_output!(DeleteBucketOwnershipControls);
    s3_output!(DeleteBucketPolicy);
    s3_output!(DeleteBucketReplication);
    s3_output!(DeleteBucketTagging);
    s3_output!(DeleteBucketWebsite);
    s3_output!(DeleteObject);
    s3_output!(DeleteObjects);
    s3_output!(DeleteObjectTagging);
    s3_output!(DeletePublicAccessBlock);
    s3_output!(GetBucketAccelerateConfiguration);
    s3_output!(GetBucketAcl);
    s3_output!(GetBucketAnalyticsConfiguration);
    s3_output!(GetBucketCors);
    s3_output!(GetBucketEncryption);
    s3_output!(GetBucketIntelligentTieringConfiguration);
    s3_output!(GetBucketInventoryConfiguration);
    s3_output!(GetBucketLifecycleConfiguration);
    s3_output!(GetBucketLocation);
    s3_output!(GetBucketLogging);
    s3_output!(GetBucketMetricsConfiguration);
    s3_output!(GetBucketNotificationConfiguration);
    s3_output!(GetBucketOwnershipControls);
    s3_output!(GetBucketPolicy);
    s3_output!(GetBucketPolicyStatus);
    s3_output!(GetBucketReplication);
    s3_output!(GetBucketRequestPayment);
    s3_output!(GetBucketTagging);
    s3_output!(GetBucketVersioning);
    s3_output!(GetBucketWebsite);
    s3_output!(GetObject);
    s3_output!(GetObjectAcl);
    s3_output!(GetObjectLegalHold);
    s3_output!(GetObjectLockConfiguration);
    s3_output!(GetObjectRetention);
    s3_output!(GetObjectTagging);
    s3_output!(GetObjectTorrent);
    s3_output!(GetPublicAccessBlock);
    s3_output!(HeadBucket);
    s3_output!(HeadObject);
    s3_output!(ListBucketAnalyticsConfigurations);
    s3_output!(ListBucketIntelligentTieringConfigurations);
    s3_output!(ListBucketInventoryConfigurations);
    s3_output!(ListBucketMetricsConfigurations);
    s3_output!(ListBuckets);
    s3_output!(ListMultipartUploads);
    s3_output!(ListObjects);
    s3_output!(ListObjectsV2);
    s3_output!(ListObjectVersions);
    s3_output!(ListParts);
    s3_output!(PutBucketAccelerateConfiguration);
    s3_output!(PutBucketAcl);
    s3_output!(PutBucketAnalyticsConfiguration);
    s3_output!(PutBucketCors);
    s3_output!(PutBucketEncryption);
    s3_output!(PutBucketIntelligentTieringConfiguration);
    s3_output!(PutBucketInventoryConfiguration);
    s3_output!(PutBucketLifecycleConfiguration);
    s3_output!(PutBucketLogging);
    s3_output!(PutBucketMetricsConfiguration);
    s3_output!(PutBucketNotificationConfiguration);
    s3_output!(PutBucketOwnershipControls);
    s3_output!(PutBucketPolicy);
    s3_output!(PutBucketReplication);
    s3_output!(PutBucketRequestPayment);
    s3_output!(PutBucketTagging);
    s3_output!(PutBucketVersioning);
    s3_output!(PutBucketWebsite);
    s3_output!(PutObject);
    s3_output!(PutObjectAcl);
    s3_output!(PutObjectLegalHold);
    s3_output!(PutObjectLockConfiguration);
    s3_output!(PutObjectRetention);
    s3_output!(PutObjectTagging);
    s3_output!(PutPublicAccessBlock);
    s3_output!(RestoreObject);
    s3_output!(SelectObjectContent);
    s3_output!(UploadPart);
    s3_output!(UploadPartCopy);
    s3_output!(WriteGetObjectResponse);
}
