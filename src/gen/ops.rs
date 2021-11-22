//! TODO This module should be generated from https://github.com/awslabs/smithy-rs

use crate::{err::*, gen::kinds::S3OpKind, types::*};

/// This module is currently unused - perhaps we can delete it if we don't need it.
pub trait S3Op {
    type Input;
    type Output;
    type Error;

    const KIND: S3OpKind;
    const INPUT_PARSER: fn(&S3Request) -> Result<Self::Input, InputError>;
    const OUTPUT_PARSER: fn(Self::Output) -> Result<HttpResponse, OutputError>;
}

/// This macro generates a struct for each operation with the input/output/error types
/// and functions that read/write those from http request/response and call its api method.
macro_rules! gen {
    ($name:ident) => {
        paste::paste! {
            pub struct [<$name Op>] {}
            impl S3Op for [<$name Op>] {
                type Input = aws_sdk_s3::input::[<$name Input>];
                type Output = aws_sdk_s3::output::[<$name Output>];
                type Error = aws_sdk_s3::error::[<$name Error>];

                const KIND: S3OpKind = S3OpKind::$name;
                const INPUT_PARSER: fn(&S3Request) -> Result<Self::Input, InputError> =
                    crate::gen::input::[<$name:snake>];
                const OUTPUT_PARSER: fn(Self::Output) -> Result<HttpResponse, OutputError> =
                    crate::gen::output::[<$name:snake>];
            }
        }
    };
}

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
