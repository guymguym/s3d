#![allow(unused)]
#![allow(non_camel_case_types)]

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Ops {
    DeleteObject,
    GetBucketAccelerateConfiguration,
    DeleteBucketInventoryConfiguration,
    GetBucketAcl,
    GetBucketCors,
    GetBucketWebsite,
    PutBucketWebsite,
    UploadPartCopy,
    WriteGetObjectResponse,
    PutBucketAccelerateConfiguration,
    UploadPart,
    GetBucketTagging,
    PutBucketIntelligentTieringConfiguration,
    CreateMultipartUpload,
    DeleteBucketMetricsConfiguration,
    PutBucketVersioning,
    DeleteBucketReplication,
    GetBucketRequestPayment,
    GetBucketLogging,
    DeleteObjects,
    ListBucketMetricsConfigurations,
    PutBucketReplication,
    PutBucketAcl,
    GetBucketOwnershipControls,
    PutObjectLockConfiguration,
    DeleteBucketPolicy,
    ListMultipartUploads,
    DeleteBucketOwnershipControls,
    GetObjectAcl,
    PutBucketLifecycleConfiguration,
    GetBucketVersioning,
    GetPublicAccessBlock,
    GetObjectRetention,
    GetBucketPolicyStatus,
    PutBucketLogging,
    PutBucketEncryption,
    PutBucketTagging,
    DeleteBucket,
    ListBucketIntelligentTieringConfigurations,
    DeleteBucketWebsite,
    GetObject,
    PutBucketCors,
    PutObject,
    ListBucketAnalyticsConfigurations,
    PutBucketAnalyticsConfiguration,
    PutBucketMetricsConfiguration,
    GetObjectTagging,
    GetBucketIntelligentTieringConfiguration,
    CopyObject,
    PutBucketInventoryConfiguration,
    DeleteBucketIntelligentTieringConfiguration,
    PutBucketOwnershipControls,
    SelectObjectContent,
    GetBucketEncryption,
    PutBucketRequestPayment,
    ListObjects,
    DeleteBucketLifecycle,
    RestoreObject,
    GetObjectLegalHold,
    DeleteBucketEncryption,
    AbortMultipartUpload,
    GetObjectLockConfiguration,
    ListParts,
    GetBucketReplication,
    PutObjectRetention,
    DeleteObjectTagging,
    GetBucketInventoryConfiguration,
    HeadObject,
    ListBuckets,
    CreateBucket,
    ListObjectVersions,
    DeleteBucketCors,
    PutObjectLegalHold,
    GetBucketMetricsConfiguration,
    HeadBucket,
    GetBucketLocation,
    DeletePublicAccessBlock,
    DeleteBucketAnalyticsConfiguration,
    DeleteBucketTagging,
    GetObjectAttributes,
    GetBucketNotificationConfiguration,
    CompleteMultipartUpload,
    ListObjectsV2,
    PutBucketPolicy,
    PutObjectAcl,
    PutPublicAccessBlock,
    GetObjectTorrent,
    GetBucketLifecycleConfiguration,
    GetBucketPolicy,
    GetBucketAnalyticsConfiguration,
    ListBucketInventoryConfigurations,
    PutBucketNotificationConfiguration,
    PutObjectTagging,
}
#[doc = r" This macro calls a provided $macro for each operation to generate code per op."]
macro_rules! generate_ops_code {
    ($ macro : ident) => {
        $macro!(DeleteObject);
        $macro!(GetBucketAccelerateConfiguration);
        $macro!(DeleteBucketInventoryConfiguration);
        $macro!(GetBucketAcl);
        $macro!(GetBucketCors);
        $macro!(GetBucketWebsite);
        $macro!(PutBucketWebsite);
        $macro!(UploadPartCopy);
        $macro!(WriteGetObjectResponse);
        $macro!(PutBucketAccelerateConfiguration);
        $macro!(UploadPart);
        $macro!(GetBucketTagging);
        $macro!(PutBucketIntelligentTieringConfiguration);
        $macro!(CreateMultipartUpload);
        $macro!(DeleteBucketMetricsConfiguration);
        $macro!(PutBucketVersioning);
        $macro!(DeleteBucketReplication);
        $macro!(GetBucketRequestPayment);
        $macro!(GetBucketLogging);
        $macro!(DeleteObjects);
        $macro!(ListBucketMetricsConfigurations);
        $macro!(PutBucketReplication);
        $macro!(PutBucketAcl);
        $macro!(GetBucketOwnershipControls);
        $macro!(PutObjectLockConfiguration);
        $macro!(DeleteBucketPolicy);
        $macro!(ListMultipartUploads);
        $macro!(DeleteBucketOwnershipControls);
        $macro!(GetObjectAcl);
        $macro!(PutBucketLifecycleConfiguration);
        $macro!(GetBucketVersioning);
        $macro!(GetPublicAccessBlock);
        $macro!(GetObjectRetention);
        $macro!(GetBucketPolicyStatus);
        $macro!(PutBucketLogging);
        $macro!(PutBucketEncryption);
        $macro!(PutBucketTagging);
        $macro!(DeleteBucket);
        $macro!(ListBucketIntelligentTieringConfigurations);
        $macro!(DeleteBucketWebsite);
        $macro!(GetObject);
        $macro!(PutBucketCors);
        $macro!(PutObject);
        $macro!(ListBucketAnalyticsConfigurations);
        $macro!(PutBucketAnalyticsConfiguration);
        $macro!(PutBucketMetricsConfiguration);
        $macro!(GetObjectTagging);
        $macro!(GetBucketIntelligentTieringConfiguration);
        $macro!(CopyObject);
        $macro!(PutBucketInventoryConfiguration);
        $macro!(DeleteBucketIntelligentTieringConfiguration);
        $macro!(PutBucketOwnershipControls);
        $macro!(SelectObjectContent);
        $macro!(GetBucketEncryption);
        $macro!(PutBucketRequestPayment);
        $macro!(ListObjects);
        $macro!(DeleteBucketLifecycle);
        $macro!(RestoreObject);
        $macro!(GetObjectLegalHold);
        $macro!(DeleteBucketEncryption);
        $macro!(AbortMultipartUpload);
        $macro!(GetObjectLockConfiguration);
        $macro!(ListParts);
        $macro!(GetBucketReplication);
        $macro!(PutObjectRetention);
        $macro!(DeleteObjectTagging);
        $macro!(GetBucketInventoryConfiguration);
        $macro!(HeadObject);
        $macro!(ListBuckets);
        $macro!(CreateBucket);
        $macro!(ListObjectVersions);
        $macro!(DeleteBucketCors);
        $macro!(PutObjectLegalHold);
        $macro!(GetBucketMetricsConfiguration);
        $macro!(HeadBucket);
        $macro!(GetBucketLocation);
        $macro!(DeletePublicAccessBlock);
        $macro!(DeleteBucketAnalyticsConfiguration);
        $macro!(DeleteBucketTagging);
        $macro!(GetObjectAttributes);
        $macro!(GetBucketNotificationConfiguration);
        $macro!(CompleteMultipartUpload);
        $macro!(ListObjectsV2);
        $macro!(PutBucketPolicy);
        $macro!(PutObjectAcl);
        $macro!(PutPublicAccessBlock);
        $macro!(GetObjectTorrent);
        $macro!(GetBucketLifecycleConfiguration);
        $macro!(GetBucketPolicy);
        $macro!(GetBucketAnalyticsConfiguration);
        $macro!(ListBucketInventoryConfigurations);
        $macro!(PutBucketNotificationConfiguration);
        $macro!(PutObjectTagging);
    };
}
#[doc = r" This macro calls a provided $macro for each operation to generate item per op."]
macro_rules ! generate_ops_items { ($ macro : ident) => { $ macro ! (DeleteObject) , $ macro ! (GetBucketAccelerateConfiguration) , $ macro ! (DeleteBucketInventoryConfiguration) , $ macro ! (GetBucketAcl) , $ macro ! (GetBucketCors) , $ macro ! (GetBucketWebsite) , $ macro ! (PutBucketWebsite) , $ macro ! (UploadPartCopy) , $ macro ! (WriteGetObjectResponse) , $ macro ! (PutBucketAccelerateConfiguration) , $ macro ! (UploadPart) , $ macro ! (GetBucketTagging) , $ macro ! (PutBucketIntelligentTieringConfiguration) , $ macro ! (CreateMultipartUpload) , $ macro ! (DeleteBucketMetricsConfiguration) , $ macro ! (PutBucketVersioning) , $ macro ! (DeleteBucketReplication) , $ macro ! (GetBucketRequestPayment) , $ macro ! (GetBucketLogging) , $ macro ! (DeleteObjects) , $ macro ! (ListBucketMetricsConfigurations) , $ macro ! (PutBucketReplication) , $ macro ! (PutBucketAcl) , $ macro ! (GetBucketOwnershipControls) , $ macro ! (PutObjectLockConfiguration) , $ macro ! (DeleteBucketPolicy) , $ macro ! (ListMultipartUploads) , $ macro ! (DeleteBucketOwnershipControls) , $ macro ! (GetObjectAcl) , $ macro ! (PutBucketLifecycleConfiguration) , $ macro ! (GetBucketVersioning) , $ macro ! (GetPublicAccessBlock) , $ macro ! (GetObjectRetention) , $ macro ! (GetBucketPolicyStatus) , $ macro ! (PutBucketLogging) , $ macro ! (PutBucketEncryption) , $ macro ! (PutBucketTagging) , $ macro ! (DeleteBucket) , $ macro ! (ListBucketIntelligentTieringConfigurations) , $ macro ! (DeleteBucketWebsite) , $ macro ! (GetObject) , $ macro ! (PutBucketCors) , $ macro ! (PutObject) , $ macro ! (ListBucketAnalyticsConfigurations) , $ macro ! (PutBucketAnalyticsConfiguration) , $ macro ! (PutBucketMetricsConfiguration) , $ macro ! (GetObjectTagging) , $ macro ! (GetBucketIntelligentTieringConfiguration) , $ macro ! (CopyObject) , $ macro ! (PutBucketInventoryConfiguration) , $ macro ! (DeleteBucketIntelligentTieringConfiguration) , $ macro ! (PutBucketOwnershipControls) , $ macro ! (SelectObjectContent) , $ macro ! (GetBucketEncryption) , $ macro ! (PutBucketRequestPayment) , $ macro ! (ListObjects) , $ macro ! (DeleteBucketLifecycle) , $ macro ! (RestoreObject) , $ macro ! (GetObjectLegalHold) , $ macro ! (DeleteBucketEncryption) , $ macro ! (AbortMultipartUpload) , $ macro ! (GetObjectLockConfiguration) , $ macro ! (ListParts) , $ macro ! (GetBucketReplication) , $ macro ! (PutObjectRetention) , $ macro ! (DeleteObjectTagging) , $ macro ! (GetBucketInventoryConfiguration) , $ macro ! (HeadObject) , $ macro ! (ListBuckets) , $ macro ! (CreateBucket) , $ macro ! (ListObjectVersions) , $ macro ! (DeleteBucketCors) , $ macro ! (PutObjectLegalHold) , $ macro ! (GetBucketMetricsConfiguration) , $ macro ! (HeadBucket) , $ macro ! (GetBucketLocation) , $ macro ! (DeletePublicAccessBlock) , $ macro ! (DeleteBucketAnalyticsConfiguration) , $ macro ! (DeleteBucketTagging) , $ macro ! (GetObjectAttributes) , $ macro ! (GetBucketNotificationConfiguration) , $ macro ! (CompleteMultipartUpload) , $ macro ! (ListObjectsV2) , $ macro ! (PutBucketPolicy) , $ macro ! (PutObjectAcl) , $ macro ! (PutPublicAccessBlock) , $ macro ! (GetObjectTorrent) , $ macro ! (GetBucketLifecycleConfiguration) , $ macro ! (GetBucketPolicy) , $ macro ! (GetBucketAnalyticsConfiguration) , $ macro ! (ListBucketInventoryConfigurations) , $ macro ! (PutBucketNotificationConfiguration) , $ macro ! (PutObjectTagging) , } }
#[doc = r" This macro matches a variable of Ops type and expands the provided $macro"]
#[doc = r" for each operation to generate code handler per op."]
macro_rules! generate_ops_match {
    ($ macro : ident , $ op : expr) => {
        match ($op) {
            ops::Ops::DeleteObject => $macro!(DeleteObject),
            ops::Ops::GetBucketAccelerateConfiguration => $macro!(GetBucketAccelerateConfiguration),
            ops::Ops::DeleteBucketInventoryConfiguration => {
                $macro!(DeleteBucketInventoryConfiguration)
            }
            ops::Ops::GetBucketAcl => $macro!(GetBucketAcl),
            ops::Ops::GetBucketCors => $macro!(GetBucketCors),
            ops::Ops::GetBucketWebsite => $macro!(GetBucketWebsite),
            ops::Ops::PutBucketWebsite => $macro!(PutBucketWebsite),
            ops::Ops::UploadPartCopy => $macro!(UploadPartCopy),
            ops::Ops::WriteGetObjectResponse => $macro!(WriteGetObjectResponse),
            ops::Ops::PutBucketAccelerateConfiguration => $macro!(PutBucketAccelerateConfiguration),
            ops::Ops::UploadPart => $macro!(UploadPart),
            ops::Ops::GetBucketTagging => $macro!(GetBucketTagging),
            ops::Ops::PutBucketIntelligentTieringConfiguration => {
                $macro!(PutBucketIntelligentTieringConfiguration)
            }
            ops::Ops::CreateMultipartUpload => $macro!(CreateMultipartUpload),
            ops::Ops::DeleteBucketMetricsConfiguration => $macro!(DeleteBucketMetricsConfiguration),
            ops::Ops::PutBucketVersioning => $macro!(PutBucketVersioning),
            ops::Ops::DeleteBucketReplication => $macro!(DeleteBucketReplication),
            ops::Ops::GetBucketRequestPayment => $macro!(GetBucketRequestPayment),
            ops::Ops::GetBucketLogging => $macro!(GetBucketLogging),
            ops::Ops::DeleteObjects => $macro!(DeleteObjects),
            ops::Ops::ListBucketMetricsConfigurations => $macro!(ListBucketMetricsConfigurations),
            ops::Ops::PutBucketReplication => $macro!(PutBucketReplication),
            ops::Ops::PutBucketAcl => $macro!(PutBucketAcl),
            ops::Ops::GetBucketOwnershipControls => $macro!(GetBucketOwnershipControls),
            ops::Ops::PutObjectLockConfiguration => $macro!(PutObjectLockConfiguration),
            ops::Ops::DeleteBucketPolicy => $macro!(DeleteBucketPolicy),
            ops::Ops::ListMultipartUploads => $macro!(ListMultipartUploads),
            ops::Ops::DeleteBucketOwnershipControls => $macro!(DeleteBucketOwnershipControls),
            ops::Ops::GetObjectAcl => $macro!(GetObjectAcl),
            ops::Ops::PutBucketLifecycleConfiguration => $macro!(PutBucketLifecycleConfiguration),
            ops::Ops::GetBucketVersioning => $macro!(GetBucketVersioning),
            ops::Ops::GetPublicAccessBlock => $macro!(GetPublicAccessBlock),
            ops::Ops::GetObjectRetention => $macro!(GetObjectRetention),
            ops::Ops::GetBucketPolicyStatus => $macro!(GetBucketPolicyStatus),
            ops::Ops::PutBucketLogging => $macro!(PutBucketLogging),
            ops::Ops::PutBucketEncryption => $macro!(PutBucketEncryption),
            ops::Ops::PutBucketTagging => $macro!(PutBucketTagging),
            ops::Ops::DeleteBucket => $macro!(DeleteBucket),
            ops::Ops::ListBucketIntelligentTieringConfigurations => {
                $macro!(ListBucketIntelligentTieringConfigurations)
            }
            ops::Ops::DeleteBucketWebsite => $macro!(DeleteBucketWebsite),
            ops::Ops::GetObject => $macro!(GetObject),
            ops::Ops::PutBucketCors => $macro!(PutBucketCors),
            ops::Ops::PutObject => $macro!(PutObject),
            ops::Ops::ListBucketAnalyticsConfigurations => {
                $macro!(ListBucketAnalyticsConfigurations)
            }
            ops::Ops::PutBucketAnalyticsConfiguration => $macro!(PutBucketAnalyticsConfiguration),
            ops::Ops::PutBucketMetricsConfiguration => $macro!(PutBucketMetricsConfiguration),
            ops::Ops::GetObjectTagging => $macro!(GetObjectTagging),
            ops::Ops::GetBucketIntelligentTieringConfiguration => {
                $macro!(GetBucketIntelligentTieringConfiguration)
            }
            ops::Ops::CopyObject => $macro!(CopyObject),
            ops::Ops::PutBucketInventoryConfiguration => $macro!(PutBucketInventoryConfiguration),
            ops::Ops::DeleteBucketIntelligentTieringConfiguration => {
                $macro!(DeleteBucketIntelligentTieringConfiguration)
            }
            ops::Ops::PutBucketOwnershipControls => $macro!(PutBucketOwnershipControls),
            ops::Ops::SelectObjectContent => $macro!(SelectObjectContent),
            ops::Ops::GetBucketEncryption => $macro!(GetBucketEncryption),
            ops::Ops::PutBucketRequestPayment => $macro!(PutBucketRequestPayment),
            ops::Ops::ListObjects => $macro!(ListObjects),
            ops::Ops::DeleteBucketLifecycle => $macro!(DeleteBucketLifecycle),
            ops::Ops::RestoreObject => $macro!(RestoreObject),
            ops::Ops::GetObjectLegalHold => $macro!(GetObjectLegalHold),
            ops::Ops::DeleteBucketEncryption => $macro!(DeleteBucketEncryption),
            ops::Ops::AbortMultipartUpload => $macro!(AbortMultipartUpload),
            ops::Ops::GetObjectLockConfiguration => $macro!(GetObjectLockConfiguration),
            ops::Ops::ListParts => $macro!(ListParts),
            ops::Ops::GetBucketReplication => $macro!(GetBucketReplication),
            ops::Ops::PutObjectRetention => $macro!(PutObjectRetention),
            ops::Ops::DeleteObjectTagging => $macro!(DeleteObjectTagging),
            ops::Ops::GetBucketInventoryConfiguration => $macro!(GetBucketInventoryConfiguration),
            ops::Ops::HeadObject => $macro!(HeadObject),
            ops::Ops::ListBuckets => $macro!(ListBuckets),
            ops::Ops::CreateBucket => $macro!(CreateBucket),
            ops::Ops::ListObjectVersions => $macro!(ListObjectVersions),
            ops::Ops::DeleteBucketCors => $macro!(DeleteBucketCors),
            ops::Ops::PutObjectLegalHold => $macro!(PutObjectLegalHold),
            ops::Ops::GetBucketMetricsConfiguration => $macro!(GetBucketMetricsConfiguration),
            ops::Ops::HeadBucket => $macro!(HeadBucket),
            ops::Ops::GetBucketLocation => $macro!(GetBucketLocation),
            ops::Ops::DeletePublicAccessBlock => $macro!(DeletePublicAccessBlock),
            ops::Ops::DeleteBucketAnalyticsConfiguration => {
                $macro!(DeleteBucketAnalyticsConfiguration)
            }
            ops::Ops::DeleteBucketTagging => $macro!(DeleteBucketTagging),
            ops::Ops::GetObjectAttributes => $macro!(GetObjectAttributes),
            ops::Ops::GetBucketNotificationConfiguration => {
                $macro!(GetBucketNotificationConfiguration)
            }
            ops::Ops::CompleteMultipartUpload => $macro!(CompleteMultipartUpload),
            ops::Ops::ListObjectsV2 => $macro!(ListObjectsV2),
            ops::Ops::PutBucketPolicy => $macro!(PutBucketPolicy),
            ops::Ops::PutObjectAcl => $macro!(PutObjectAcl),
            ops::Ops::PutPublicAccessBlock => $macro!(PutPublicAccessBlock),
            ops::Ops::GetObjectTorrent => $macro!(GetObjectTorrent),
            ops::Ops::GetBucketLifecycleConfiguration => $macro!(GetBucketLifecycleConfiguration),
            ops::Ops::GetBucketPolicy => $macro!(GetBucketPolicy),
            ops::Ops::GetBucketAnalyticsConfiguration => $macro!(GetBucketAnalyticsConfiguration),
            ops::Ops::ListBucketInventoryConfigurations => {
                $macro!(ListBucketInventoryConfigurations)
            }
            ops::Ops::PutBucketNotificationConfiguration => {
                $macro!(PutBucketNotificationConfiguration)
            }
            ops::Ops::PutObjectTagging => $macro!(PutObjectTagging),
        }
    };
}
pub(crate) use generate_ops_code;
pub(crate) use generate_ops_items;
pub(crate) use generate_ops_match;
