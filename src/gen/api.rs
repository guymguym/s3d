//! TODO This module should be generated from https://github.com/awslabs/smithy-rs

use crate::types::*;
use aws_sdk_s3::{error::*, input::*, output::*};

/// This macro generates a default function for each op.
/// We can't use async_trait macro inside our macro so we use the same thing it does
/// which is this pin-box-dyn-future - see long explanation here:
/// https://smallcultfollowing.com/babysteps/blog/2019/10/26/async-fn-in-traits-are-hard/
macro_rules! gen {
    ($name:ident) => {
        paste::paste! {
            fn [<$name:snake>](&self, _: [<$name Input>]) -> TraitFuture<[<$name Output>], [<$name Error>]> {
                Box::pin(async move { Err([<$name Error>]::generic(not_implemented())) })
            }
        }
    };
}

fn not_implemented() -> S3Error {
    S3Error::builder()
        .code("NotImplemented")
        .message("The requested action is not implemented.")
        .build()
}

pub trait S3Api: Sync + Send {
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
