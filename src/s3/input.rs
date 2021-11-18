/// TODO generate this file automatically with https://github.com/awslabs/smithy-rs

/// This macro defines a trait with functions for each of the S3 API methods.
macro_rules! s3_input {
    ($name:ident) => {
        paste::paste! {
            fn [<$name:snake>](
                &self,
                _req: crate::s3::types::S3Request
            ) -> Result<
                aws_sdk_s3::input::[<$name Input>],
                crate::s3::types::InputError
            > {
                Err(crate::s3::types::InputError::NotImplemented)
            }
        }
    };
}

pub trait S3Input: Sync + Send {
    s3_input!(AbortMultipartUpload);
    s3_input!(CompleteMultipartUpload);
    s3_input!(CopyObject);
    s3_input!(CreateBucket);
    s3_input!(CreateMultipartUpload);
    s3_input!(DeleteBucket);
    s3_input!(DeleteBucketAnalyticsConfiguration);
    s3_input!(DeleteBucketCors);
    s3_input!(DeleteBucketEncryption);
    s3_input!(DeleteBucketIntelligentTieringConfiguration);
    s3_input!(DeleteBucketInventoryConfiguration);
    s3_input!(DeleteBucketLifecycle);
    s3_input!(DeleteBucketMetricsConfiguration);
    s3_input!(DeleteBucketOwnershipControls);
    s3_input!(DeleteBucketPolicy);
    s3_input!(DeleteBucketReplication);
    s3_input!(DeleteBucketTagging);
    s3_input!(DeleteBucketWebsite);
    s3_input!(DeleteObject);
    s3_input!(DeleteObjects);
    s3_input!(DeleteObjectTagging);
    s3_input!(DeletePublicAccessBlock);
    s3_input!(GetBucketAccelerateConfiguration);
    s3_input!(GetBucketAcl);
    s3_input!(GetBucketAnalyticsConfiguration);
    s3_input!(GetBucketCors);
    s3_input!(GetBucketEncryption);
    s3_input!(GetBucketIntelligentTieringConfiguration);
    s3_input!(GetBucketInventoryConfiguration);
    s3_input!(GetBucketLifecycleConfiguration);
    s3_input!(GetBucketLocation);
    s3_input!(GetBucketLogging);
    s3_input!(GetBucketMetricsConfiguration);
    s3_input!(GetBucketNotificationConfiguration);
    s3_input!(GetBucketOwnershipControls);
    s3_input!(GetBucketPolicy);
    s3_input!(GetBucketPolicyStatus);
    s3_input!(GetBucketReplication);
    s3_input!(GetBucketRequestPayment);
    s3_input!(GetBucketTagging);
    s3_input!(GetBucketVersioning);
    s3_input!(GetBucketWebsite);
    s3_input!(GetObject);
    s3_input!(GetObjectAcl);
    s3_input!(GetObjectLegalHold);
    s3_input!(GetObjectLockConfiguration);
    s3_input!(GetObjectRetention);
    s3_input!(GetObjectTagging);
    s3_input!(GetObjectTorrent);
    s3_input!(GetPublicAccessBlock);
    s3_input!(HeadBucket);
    s3_input!(HeadObject);
    s3_input!(ListBucketAnalyticsConfigurations);
    s3_input!(ListBucketIntelligentTieringConfigurations);
    s3_input!(ListBucketInventoryConfigurations);
    s3_input!(ListBucketMetricsConfigurations);
    s3_input!(ListBuckets);
    s3_input!(ListMultipartUploads);
    s3_input!(ListObjects);
    s3_input!(ListObjectsV2);
    s3_input!(ListObjectVersions);
    s3_input!(ListParts);
    s3_input!(PutBucketAccelerateConfiguration);
    s3_input!(PutBucketAcl);
    s3_input!(PutBucketAnalyticsConfiguration);
    s3_input!(PutBucketCors);
    s3_input!(PutBucketEncryption);
    s3_input!(PutBucketIntelligentTieringConfiguration);
    s3_input!(PutBucketInventoryConfiguration);
    s3_input!(PutBucketLifecycleConfiguration);
    s3_input!(PutBucketLogging);
    s3_input!(PutBucketMetricsConfiguration);
    s3_input!(PutBucketNotificationConfiguration);
    s3_input!(PutBucketOwnershipControls);
    s3_input!(PutBucketPolicy);
    s3_input!(PutBucketReplication);
    s3_input!(PutBucketRequestPayment);
    s3_input!(PutBucketTagging);
    s3_input!(PutBucketVersioning);
    s3_input!(PutBucketWebsite);
    s3_input!(PutObject);
    s3_input!(PutObjectAcl);
    s3_input!(PutObjectLegalHold);
    s3_input!(PutObjectLockConfiguration);
    s3_input!(PutObjectRetention);
    s3_input!(PutObjectTagging);
    s3_input!(PutPublicAccessBlock);
    s3_input!(RestoreObject);
    s3_input!(SelectObjectContent);
    s3_input!(UploadPart);
    s3_input!(UploadPartCopy);
    s3_input!(WriteGetObjectResponse);
}
