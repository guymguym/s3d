//! This module enumerates all S3 operations and provides macros to generate code per oepration.
//! Currently written by hand which is difficult to maintain long term.
//! TODO This module should be generated from https://github.com/awslabs/smithy-rs.

/// This enumerates all S3 operations
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum S3OpKind {
    AbortMultipartUpload,
    CompleteMultipartUpload,
    CopyObject,
    CreateBucket,
    CreateMultipartUpload,
    DeleteBucket,
    DeleteBucketAnalyticsConfiguration,
    DeleteBucketCors,
    DeleteBucketEncryption,
    DeleteBucketIntelligentTieringConfiguration,
    DeleteBucketInventoryConfiguration,
    DeleteBucketLifecycle,
    DeleteBucketMetricsConfiguration,
    DeleteBucketOwnershipControls,
    DeleteBucketPolicy,
    DeleteBucketReplication,
    DeleteBucketTagging,
    DeleteBucketWebsite,
    DeleteObject,
    DeleteObjects,
    DeleteObjectTagging,
    DeletePublicAccessBlock,
    GetBucketAccelerateConfiguration,
    GetBucketAcl,
    GetBucketAnalyticsConfiguration,
    GetBucketCors,
    GetBucketEncryption,
    GetBucketIntelligentTieringConfiguration,
    GetBucketInventoryConfiguration,
    GetBucketLifecycleConfiguration,
    GetBucketLocation,
    GetBucketLogging,
    GetBucketMetricsConfiguration,
    GetBucketNotificationConfiguration,
    GetBucketOwnershipControls,
    GetBucketPolicy,
    GetBucketPolicyStatus,
    GetBucketReplication,
    GetBucketRequestPayment,
    GetBucketTagging,
    GetBucketVersioning,
    GetBucketWebsite,
    GetObject,
    GetObjectAcl,
    GetObjectLegalHold,
    GetObjectLockConfiguration,
    GetObjectRetention,
    GetObjectTagging,
    GetObjectTorrent,
    GetPublicAccessBlock,
    HeadBucket,
    HeadObject,
    ListBucketAnalyticsConfigurations,
    ListBucketIntelligentTieringConfigurations,
    ListBucketInventoryConfigurations,
    ListBucketMetricsConfigurations,
    ListBuckets,
    ListMultipartUploads,
    ListObjects,
    ListObjectsV2,
    ListObjectVersions,
    ListParts,
    PutBucketAccelerateConfiguration,
    PutBucketAcl,
    PutBucketAnalyticsConfiguration,
    PutBucketCors,
    PutBucketEncryption,
    PutBucketIntelligentTieringConfiguration,
    PutBucketInventoryConfiguration,
    PutBucketLifecycleConfiguration,
    PutBucketLogging,
    PutBucketMetricsConfiguration,
    PutBucketNotificationConfiguration,
    PutBucketOwnershipControls,
    PutBucketPolicy,
    PutBucketReplication,
    PutBucketRequestPayment,
    PutBucketTagging,
    PutBucketVersioning,
    PutBucketWebsite,
    PutObject,
    PutObjectAcl,
    PutObjectLegalHold,
    PutObjectLockConfiguration,
    PutObjectRetention,
    PutObjectTagging,
    PutPublicAccessBlock,
    RestoreObject,
    SelectObjectContent,
    UploadPart,
    UploadPartCopy,
    WriteGetObjectResponse,
}

impl From<&str> for S3OpKind {
    fn from(s: &str) -> Self {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        match s {
            "AbortMultipartUpload" => Self::AbortMultipartUpload,
            "CompleteMultipartUpload" => Self::CompleteMultipartUpload,
            "CopyObject" => Self::CopyObject,
            "CreateBucket" => Self::CreateBucket,
            "CreateMultipartUpload" => Self::CreateMultipartUpload,
            "DeleteBucket" => Self::DeleteBucket,
            "DeleteBucketAnalyticsConfiguration" => Self::DeleteBucketAnalyticsConfiguration,
            "DeleteBucketCors" => Self::DeleteBucketCors,
            "DeleteBucketEncryption" => Self::DeleteBucketEncryption,
            "DeleteBucketIntelligentTieringConfiguration" => Self::DeleteBucketIntelligentTieringConfiguration,
            "DeleteBucketInventoryConfiguration" => Self::DeleteBucketInventoryConfiguration,
            "DeleteBucketLifecycle" => Self::DeleteBucketLifecycle,
            "DeleteBucketMetricsConfiguration" => Self::DeleteBucketMetricsConfiguration,
            "DeleteBucketOwnershipControls" => Self::DeleteBucketOwnershipControls,
            "DeleteBucketPolicy" => Self::DeleteBucketPolicy,
            "DeleteBucketReplication" => Self::DeleteBucketReplication,
            "DeleteBucketTagging" => Self::DeleteBucketTagging,
            "DeleteBucketWebsite" => Self::DeleteBucketWebsite,
            "DeleteObject" => Self::DeleteObject,
            "DeleteObjects" => Self::DeleteObjects,
            "DeleteObjectTagging" => Self::DeleteObjectTagging,
            "DeletePublicAccessBlock" => Self::DeletePublicAccessBlock,
            "GetBucketAccelerateConfiguration" => Self::GetBucketAccelerateConfiguration,
            "GetBucketAcl" => Self::GetBucketAcl,
            "GetBucketAnalyticsConfiguration" => Self::GetBucketAnalyticsConfiguration,
            "GetBucketCors" => Self::GetBucketCors,
            "GetBucketEncryption" => Self::GetBucketEncryption,
            "GetBucketIntelligentTieringConfiguration" => Self::GetBucketIntelligentTieringConfiguration,
            "GetBucketInventoryConfiguration" => Self::GetBucketInventoryConfiguration,
            "GetBucketLifecycleConfiguration" => Self::GetBucketLifecycleConfiguration,
            "GetBucketLocation" => Self::GetBucketLocation,
            "GetBucketLogging" => Self::GetBucketLogging,
            "GetBucketMetricsConfiguration" => Self::GetBucketMetricsConfiguration,
            "GetBucketNotificationConfiguration" => Self::GetBucketNotificationConfiguration,
            "GetBucketOwnershipControls" => Self::GetBucketOwnershipControls,
            "GetBucketPolicy" => Self::GetBucketPolicy,
            "GetBucketPolicyStatus" => Self::GetBucketPolicyStatus,
            "GetBucketReplication" => Self::GetBucketReplication,
            "GetBucketRequestPayment" => Self::GetBucketRequestPayment,
            "GetBucketTagging" => Self::GetBucketTagging,
            "GetBucketVersioning" => Self::GetBucketVersioning,
            "GetBucketWebsite" => Self::GetBucketWebsite,
            "GetObject" => Self::GetObject,
            "GetObjectAcl" => Self::GetObjectAcl,
            "GetObjectLegalHold" => Self::GetObjectLegalHold,
            "GetObjectLockConfiguration" => Self::GetObjectLockConfiguration,
            "GetObjectRetention" => Self::GetObjectRetention,
            "GetObjectTagging" => Self::GetObjectTagging,
            "GetObjectTorrent" => Self::GetObjectTorrent,
            "GetPublicAccessBlock" => Self::GetPublicAccessBlock,
            "HeadBucket" => Self::HeadBucket,
            "HeadObject" => Self::HeadObject,
            "ListBucketAnalyticsConfigurations" => Self::ListBucketAnalyticsConfigurations,
            "ListBucketIntelligentTieringConfigurations" => Self::ListBucketIntelligentTieringConfigurations,
            "ListBucketInventoryConfigurations" => Self::ListBucketInventoryConfigurations,
            "ListBucketMetricsConfigurations" => Self::ListBucketMetricsConfigurations,
            "ListBuckets" => Self::ListBuckets,
            "ListMultipartUploads" => Self::ListMultipartUploads,
            "ListObjects" => Self::ListObjects,
            "ListObjectsV2" => Self::ListObjectsV2,
            "ListObjectVersions" => Self::ListObjectVersions,
            "ListParts" => Self::ListParts,
            "PutBucketAccelerateConfiguration" => Self::PutBucketAccelerateConfiguration,
            "PutBucketAcl" => Self::PutBucketAcl,
            "PutBucketAnalyticsConfiguration" => Self::PutBucketAnalyticsConfiguration,
            "PutBucketCors" => Self::PutBucketCors,
            "PutBucketEncryption" => Self::PutBucketEncryption,
            "PutBucketIntelligentTieringConfiguration" => Self::PutBucketIntelligentTieringConfiguration,
            "PutBucketInventoryConfiguration" => Self::PutBucketInventoryConfiguration,
            "PutBucketLifecycleConfiguration" => Self::PutBucketLifecycleConfiguration,
            "PutBucketLogging" => Self::PutBucketLogging,
            "PutBucketMetricsConfiguration" => Self::PutBucketMetricsConfiguration,
            "PutBucketNotificationConfiguration" => Self::PutBucketNotificationConfiguration,
            "PutBucketOwnershipControls" => Self::PutBucketOwnershipControls,
            "PutBucketPolicy" => Self::PutBucketPolicy,
            "PutBucketReplication" => Self::PutBucketReplication,
            "PutBucketRequestPayment" => Self::PutBucketRequestPayment,
            "PutBucketTagging" => Self::PutBucketTagging,
            "PutBucketVersioning" => Self::PutBucketVersioning,
            "PutBucketWebsite" => Self::PutBucketWebsite,
            "PutObject" => Self::PutObject,
            "PutObjectAcl" => Self::PutObjectAcl,
            "PutObjectLegalHold" => Self::PutObjectLegalHold,
            "PutObjectLockConfiguration" => Self::PutObjectLockConfiguration,
            "PutObjectRetention" => Self::PutObjectRetention,
            "PutObjectTagging" => Self::PutObjectTagging,
            "PutPublicAccessBlock" => Self::PutPublicAccessBlock,
            "RestoreObject" => Self::RestoreObject,
            "SelectObjectContent" => Self::SelectObjectContent,
            "UploadPart" => Self::UploadPart,
            "UploadPartCopy" => Self::UploadPartCopy,
            "WriteGetObjectResponse" => Self::WriteGetObjectResponse,
            _ => panic!("Unknown S3 operation name: {}", s),
        }
    }
}

/// This macro calls a provided $macro for each S3 operation to generate code per op.
macro_rules! generate_code_for_each_s3_op {
    ($macro:ident) => {
        $macro!(AbortMultipartUpload);
        $macro!(CompleteMultipartUpload);
        $macro!(CopyObject);
        $macro!(CreateBucket);
        $macro!(CreateMultipartUpload);
        $macro!(DeleteBucket);
        $macro!(DeleteBucketAnalyticsConfiguration);
        $macro!(DeleteBucketCors);
        $macro!(DeleteBucketEncryption);
        $macro!(DeleteBucketIntelligentTieringConfiguration);
        $macro!(DeleteBucketInventoryConfiguration);
        $macro!(DeleteBucketLifecycle);
        $macro!(DeleteBucketMetricsConfiguration);
        $macro!(DeleteBucketOwnershipControls);
        $macro!(DeleteBucketPolicy);
        $macro!(DeleteBucketReplication);
        $macro!(DeleteBucketTagging);
        $macro!(DeleteBucketWebsite);
        $macro!(DeleteObject);
        $macro!(DeleteObjects);
        $macro!(DeleteObjectTagging);
        $macro!(DeletePublicAccessBlock);
        $macro!(GetBucketAccelerateConfiguration);
        $macro!(GetBucketAcl);
        $macro!(GetBucketAnalyticsConfiguration);
        $macro!(GetBucketCors);
        $macro!(GetBucketEncryption);
        $macro!(GetBucketIntelligentTieringConfiguration);
        $macro!(GetBucketInventoryConfiguration);
        $macro!(GetBucketLifecycleConfiguration);
        $macro!(GetBucketLocation);
        $macro!(GetBucketLogging);
        $macro!(GetBucketMetricsConfiguration);
        $macro!(GetBucketNotificationConfiguration);
        $macro!(GetBucketOwnershipControls);
        $macro!(GetBucketPolicy);
        $macro!(GetBucketPolicyStatus);
        $macro!(GetBucketReplication);
        $macro!(GetBucketRequestPayment);
        $macro!(GetBucketTagging);
        $macro!(GetBucketVersioning);
        $macro!(GetBucketWebsite);
        $macro!(GetObject);
        $macro!(GetObjectAcl);
        $macro!(GetObjectLegalHold);
        $macro!(GetObjectLockConfiguration);
        $macro!(GetObjectRetention);
        $macro!(GetObjectTagging);
        $macro!(GetObjectTorrent);
        $macro!(GetPublicAccessBlock);
        $macro!(HeadBucket);
        $macro!(HeadObject);
        $macro!(ListBucketAnalyticsConfigurations);
        $macro!(ListBucketIntelligentTieringConfigurations);
        $macro!(ListBucketInventoryConfigurations);
        $macro!(ListBucketMetricsConfigurations);
        $macro!(ListBuckets);
        $macro!(ListMultipartUploads);
        $macro!(ListObjects);
        $macro!(ListObjectsV2);
        $macro!(ListObjectVersions);
        $macro!(ListParts);
        $macro!(PutBucketAccelerateConfiguration);
        $macro!(PutBucketAcl);
        $macro!(PutBucketAnalyticsConfiguration);
        $macro!(PutBucketCors);
        $macro!(PutBucketEncryption);
        $macro!(PutBucketIntelligentTieringConfiguration);
        $macro!(PutBucketInventoryConfiguration);
        $macro!(PutBucketLifecycleConfiguration);
        $macro!(PutBucketLogging);
        $macro!(PutBucketMetricsConfiguration);
        $macro!(PutBucketNotificationConfiguration);
        $macro!(PutBucketOwnershipControls);
        $macro!(PutBucketPolicy);
        $macro!(PutBucketReplication);
        $macro!(PutBucketRequestPayment);
        $macro!(PutBucketTagging);
        $macro!(PutBucketVersioning);
        $macro!(PutBucketWebsite);
        $macro!(PutObject);
        $macro!(PutObjectAcl);
        $macro!(PutObjectLegalHold);
        $macro!(PutObjectLockConfiguration);
        $macro!(PutObjectRetention);
        $macro!(PutObjectTagging);
        $macro!(PutPublicAccessBlock);
        $macro!(RestoreObject);
        $macro!(SelectObjectContent);
        $macro!(UploadPart);
        $macro!(UploadPartCopy);
        $macro!(WriteGetObjectResponse);
    };
}

/// This macro generates a match expression for a given operation kind,
/// and maps each kind to a generated code from the provided $macro.
#[cfg_attr(rustfmt, rustfmt_skip)]
macro_rules! generate_match_for_each_s3_op {
    ($macro:ident, $op:expr) => {
        match ($op) {
            S3OpKind::AbortMultipartUpload => $macro!(AbortMultipartUpload),
            S3OpKind::CompleteMultipartUpload => $macro!(CompleteMultipartUpload),
            S3OpKind::CopyObject => $macro!(CopyObject),
            S3OpKind::CreateBucket => $macro!(CreateBucket),
            S3OpKind::CreateMultipartUpload => $macro!(CreateMultipartUpload),
            S3OpKind::DeleteBucket => $macro!(DeleteBucket),
            S3OpKind::DeleteBucketAnalyticsConfiguration => $macro!(DeleteBucketAnalyticsConfiguration),
            S3OpKind::DeleteBucketCors => $macro!(DeleteBucketCors),
            S3OpKind::DeleteBucketEncryption => $macro!(DeleteBucketEncryption),
            S3OpKind::DeleteBucketIntelligentTieringConfiguration => $macro!(DeleteBucketIntelligentTieringConfiguration),
            S3OpKind::DeleteBucketInventoryConfiguration => $macro!(DeleteBucketInventoryConfiguration),
            S3OpKind::DeleteBucketLifecycle => $macro!(DeleteBucketLifecycle),
            S3OpKind::DeleteBucketMetricsConfiguration => $macro!(DeleteBucketMetricsConfiguration),
            S3OpKind::DeleteBucketOwnershipControls => $macro!(DeleteBucketOwnershipControls),
            S3OpKind::DeleteBucketPolicy => $macro!(DeleteBucketPolicy),
            S3OpKind::DeleteBucketReplication => $macro!(DeleteBucketReplication),
            S3OpKind::DeleteBucketTagging => $macro!(DeleteBucketTagging),
            S3OpKind::DeleteBucketWebsite => $macro!(DeleteBucketWebsite),
            S3OpKind::DeleteObject => $macro!(DeleteObject),
            S3OpKind::DeleteObjects => $macro!(DeleteObjects),
            S3OpKind::DeleteObjectTagging => $macro!(DeleteObjectTagging),
            S3OpKind::DeletePublicAccessBlock => $macro!(DeletePublicAccessBlock),
            S3OpKind::GetBucketAccelerateConfiguration => $macro!(GetBucketAccelerateConfiguration),
            S3OpKind::GetBucketAcl => $macro!(GetBucketAcl),
            S3OpKind::GetBucketAnalyticsConfiguration => $macro!(GetBucketAnalyticsConfiguration),
            S3OpKind::GetBucketCors => $macro!(GetBucketCors),
            S3OpKind::GetBucketEncryption => $macro!(GetBucketEncryption),
            S3OpKind::GetBucketIntelligentTieringConfiguration => $macro!(GetBucketIntelligentTieringConfiguration),
            S3OpKind::GetBucketInventoryConfiguration => $macro!(GetBucketInventoryConfiguration),
            S3OpKind::GetBucketLifecycleConfiguration => $macro!(GetBucketLifecycleConfiguration),
            S3OpKind::GetBucketLocation => $macro!(GetBucketLocation),
            S3OpKind::GetBucketLogging => $macro!(GetBucketLogging),
            S3OpKind::GetBucketMetricsConfiguration => $macro!(GetBucketMetricsConfiguration),
            S3OpKind::GetBucketNotificationConfiguration => $macro!(GetBucketNotificationConfiguration),
            S3OpKind::GetBucketOwnershipControls => $macro!(GetBucketOwnershipControls),
            S3OpKind::GetBucketPolicy => $macro!(GetBucketPolicy),
            S3OpKind::GetBucketPolicyStatus => $macro!(GetBucketPolicyStatus),
            S3OpKind::GetBucketReplication => $macro!(GetBucketReplication),
            S3OpKind::GetBucketRequestPayment => $macro!(GetBucketRequestPayment),
            S3OpKind::GetBucketTagging => $macro!(GetBucketTagging),
            S3OpKind::GetBucketVersioning => $macro!(GetBucketVersioning),
            S3OpKind::GetBucketWebsite => $macro!(GetBucketWebsite),
            S3OpKind::GetObject => $macro!(GetObject),
            S3OpKind::GetObjectAcl => $macro!(GetObjectAcl),
            S3OpKind::GetObjectLegalHold => $macro!(GetObjectLegalHold),
            S3OpKind::GetObjectLockConfiguration => $macro!(GetObjectLockConfiguration),
            S3OpKind::GetObjectRetention => $macro!(GetObjectRetention),
            S3OpKind::GetObjectTagging => $macro!(GetObjectTagging),
            S3OpKind::GetObjectTorrent => $macro!(GetObjectTorrent),
            S3OpKind::GetPublicAccessBlock => $macro!(GetPublicAccessBlock),
            S3OpKind::HeadBucket => $macro!(HeadBucket),
            S3OpKind::HeadObject => $macro!(HeadObject),
            S3OpKind::ListBucketAnalyticsConfigurations => $macro!(ListBucketAnalyticsConfigurations),
            S3OpKind::ListBucketIntelligentTieringConfigurations => $macro!(ListBucketIntelligentTieringConfigurations),
            S3OpKind::ListBucketInventoryConfigurations => $macro!(ListBucketInventoryConfigurations),
            S3OpKind::ListBucketMetricsConfigurations => $macro!(ListBucketMetricsConfigurations),
            S3OpKind::ListBuckets => $macro!(ListBuckets),
            S3OpKind::ListMultipartUploads => $macro!(ListMultipartUploads),
            S3OpKind::ListObjects => $macro!(ListObjects),
            S3OpKind::ListObjectsV2 => $macro!(ListObjectsV2),
            S3OpKind::ListObjectVersions => $macro!(ListObjectVersions),
            S3OpKind::ListParts => $macro!(ListParts),
            S3OpKind::PutBucketAccelerateConfiguration => $macro!(PutBucketAccelerateConfiguration),
            S3OpKind::PutBucketAcl => $macro!(PutBucketAcl),
            S3OpKind::PutBucketAnalyticsConfiguration => $macro!(PutBucketAnalyticsConfiguration),
            S3OpKind::PutBucketCors => $macro!(PutBucketCors),
            S3OpKind::PutBucketEncryption => $macro!(PutBucketEncryption),
            S3OpKind::PutBucketIntelligentTieringConfiguration => $macro!(PutBucketIntelligentTieringConfiguration),
            S3OpKind::PutBucketInventoryConfiguration => $macro!(PutBucketInventoryConfiguration),
            S3OpKind::PutBucketLifecycleConfiguration => $macro!(PutBucketLifecycleConfiguration),
            S3OpKind::PutBucketLogging => $macro!(PutBucketLogging),
            S3OpKind::PutBucketMetricsConfiguration => $macro!(PutBucketMetricsConfiguration),
            S3OpKind::PutBucketNotificationConfiguration => $macro!(PutBucketNotificationConfiguration),
            S3OpKind::PutBucketOwnershipControls => $macro!(PutBucketOwnershipControls),
            S3OpKind::PutBucketPolicy => $macro!(PutBucketPolicy),
            S3OpKind::PutBucketReplication => $macro!(PutBucketReplication),
            S3OpKind::PutBucketRequestPayment => $macro!(PutBucketRequestPayment),
            S3OpKind::PutBucketTagging => $macro!(PutBucketTagging),
            S3OpKind::PutBucketVersioning => $macro!(PutBucketVersioning),
            S3OpKind::PutBucketWebsite => $macro!(PutBucketWebsite),
            S3OpKind::PutObject => $macro!(PutObject),
            S3OpKind::PutObjectAcl => $macro!(PutObjectAcl),
            S3OpKind::PutObjectLegalHold => $macro!(PutObjectLegalHold),
            S3OpKind::PutObjectLockConfiguration => $macro!(PutObjectLockConfiguration),
            S3OpKind::PutObjectRetention => $macro!(PutObjectRetention),
            S3OpKind::PutObjectTagging => $macro!(PutObjectTagging),
            S3OpKind::PutPublicAccessBlock => $macro!(PutPublicAccessBlock),
            S3OpKind::RestoreObject => $macro!(RestoreObject),
            S3OpKind::SelectObjectContent => $macro!(SelectObjectContent),
            S3OpKind::UploadPart => $macro!(UploadPart),
            S3OpKind::UploadPartCopy => $macro!(UploadPartCopy),
            S3OpKind::WriteGetObjectResponse => $macro!(WriteGetObjectResponse),
        }
    };
}

pub(crate) use generate_code_for_each_s3_op;
pub(crate) use generate_match_for_each_s3_op;
