#![allow(unused)]
#![allow(non_camel_case_types)]

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Ops {
    PutBucketLifecycleConfiguration,
    GetBucketAnalyticsConfiguration,
    PutBucketMetricsConfiguration,
    GetObjectRetention,
    PutBucketAnalyticsConfiguration,
    PutPublicAccessBlock,
    PutObject,
    CreateMultipartUpload,
    WriteGetObjectResponse,
    PutBucketVersioning,
    GetBucketWebsite,
    GetBucketOwnershipControls,
    GetObjectLockConfiguration,
    ListObjectsV2,
    GetBucketNotificationConfiguration,
    ListParts,
    DeleteBucket,
    DeleteBucketMetricsConfiguration,
    DeleteBucketPolicy,
    GetObject,
    GetBucketAccelerateConfiguration,
    SelectObjectContent,
    UploadPartCopy,
    PutObjectLegalHold,
    PutObjectRetention,
    GetBucketReplication,
    GetBucketTagging,
    PutObjectLockConfiguration,
    GetBucketPolicyStatus,
    GetBucketPolicy,
    GetObjectAcl,
    GetObjectAttributes,
    PutBucketTagging,
    CopyObject,
    ListObjects,
    DeleteBucketAnalyticsConfiguration,
    DeleteObjects,
    ListBucketMetricsConfigurations,
    DeleteBucketEncryption,
    PutBucketLogging,
    ListBucketAnalyticsConfigurations,
    PutBucketWebsite,
    PutBucketNotificationConfiguration,
    GetBucketRequestPayment,
    PutBucketIntelligentTieringConfiguration,
    GetBucketIntelligentTieringConfiguration,
    DeleteBucketWebsite,
    DeleteObject,
    ListBucketIntelligentTieringConfigurations,
    PutBucketReplication,
    GetPublicAccessBlock,
    DeleteBucketLifecycle,
    PutObjectAcl,
    DeleteObjectTagging,
    ListBucketInventoryConfigurations,
    GetObjectTagging,
    DeleteBucketCors,
    PutBucketRequestPayment,
    GetBucketLocation,
    ListBuckets,
    DeleteBucketIntelligentTieringConfiguration,
    DeletePublicAccessBlock,
    GetBucketCors,
    GetBucketAcl,
    RestoreObject,
    PutObjectTagging,
    GetBucketInventoryConfiguration,
    GetBucketEncryption,
    AbortMultipartUpload,
    DeleteBucketTagging,
    PutBucketOwnershipControls,
    DeleteBucketOwnershipControls,
    GetBucketMetricsConfiguration,
    UploadPart,
    GetBucketVersioning,
    GetObjectTorrent,
    PutBucketEncryption,
    CreateBucket,
    DeleteBucketInventoryConfiguration,
    GetBucketLifecycleConfiguration,
    PutBucketAcl,
    PutBucketPolicy,
    ListMultipartUploads,
    CompleteMultipartUpload,
    HeadBucket,
    PutBucketAccelerateConfiguration,
    HeadObject,
    PutBucketCors,
    ListObjectVersions,
    GetObjectLegalHold,
    DeleteBucketReplication,
    GetBucketLogging,
    PutBucketInventoryConfiguration,
}
#[doc = r" This macro calls a provided $macro for each operation to generate code per op."]
macro_rules! generate_ops_code {
    ($ macro : ident) => {
        $macro!(PutBucketLifecycleConfiguration);
        $macro!(GetBucketAnalyticsConfiguration);
        $macro!(PutBucketMetricsConfiguration);
        $macro!(GetObjectRetention);
        $macro!(PutBucketAnalyticsConfiguration);
        $macro!(PutPublicAccessBlock);
        $macro!(PutObject);
        $macro!(CreateMultipartUpload);
        $macro!(WriteGetObjectResponse);
        $macro!(PutBucketVersioning);
        $macro!(GetBucketWebsite);
        $macro!(GetBucketOwnershipControls);
        $macro!(GetObjectLockConfiguration);
        $macro!(ListObjectsV2);
        $macro!(GetBucketNotificationConfiguration);
        $macro!(ListParts);
        $macro!(DeleteBucket);
        $macro!(DeleteBucketMetricsConfiguration);
        $macro!(DeleteBucketPolicy);
        $macro!(GetObject);
        $macro!(GetBucketAccelerateConfiguration);
        $macro!(SelectObjectContent);
        $macro!(UploadPartCopy);
        $macro!(PutObjectLegalHold);
        $macro!(PutObjectRetention);
        $macro!(GetBucketReplication);
        $macro!(GetBucketTagging);
        $macro!(PutObjectLockConfiguration);
        $macro!(GetBucketPolicyStatus);
        $macro!(GetBucketPolicy);
        $macro!(GetObjectAcl);
        $macro!(GetObjectAttributes);
        $macro!(PutBucketTagging);
        $macro!(CopyObject);
        $macro!(ListObjects);
        $macro!(DeleteBucketAnalyticsConfiguration);
        $macro!(DeleteObjects);
        $macro!(ListBucketMetricsConfigurations);
        $macro!(DeleteBucketEncryption);
        $macro!(PutBucketLogging);
        $macro!(ListBucketAnalyticsConfigurations);
        $macro!(PutBucketWebsite);
        $macro!(PutBucketNotificationConfiguration);
        $macro!(GetBucketRequestPayment);
        $macro!(PutBucketIntelligentTieringConfiguration);
        $macro!(GetBucketIntelligentTieringConfiguration);
        $macro!(DeleteBucketWebsite);
        $macro!(DeleteObject);
        $macro!(ListBucketIntelligentTieringConfigurations);
        $macro!(PutBucketReplication);
        $macro!(GetPublicAccessBlock);
        $macro!(DeleteBucketLifecycle);
        $macro!(PutObjectAcl);
        $macro!(DeleteObjectTagging);
        $macro!(ListBucketInventoryConfigurations);
        $macro!(GetObjectTagging);
        $macro!(DeleteBucketCors);
        $macro!(PutBucketRequestPayment);
        $macro!(GetBucketLocation);
        $macro!(ListBuckets);
        $macro!(DeleteBucketIntelligentTieringConfiguration);
        $macro!(DeletePublicAccessBlock);
        $macro!(GetBucketCors);
        $macro!(GetBucketAcl);
        $macro!(RestoreObject);
        $macro!(PutObjectTagging);
        $macro!(GetBucketInventoryConfiguration);
        $macro!(GetBucketEncryption);
        $macro!(AbortMultipartUpload);
        $macro!(DeleteBucketTagging);
        $macro!(PutBucketOwnershipControls);
        $macro!(DeleteBucketOwnershipControls);
        $macro!(GetBucketMetricsConfiguration);
        $macro!(UploadPart);
        $macro!(GetBucketVersioning);
        $macro!(GetObjectTorrent);
        $macro!(PutBucketEncryption);
        $macro!(CreateBucket);
        $macro!(DeleteBucketInventoryConfiguration);
        $macro!(GetBucketLifecycleConfiguration);
        $macro!(PutBucketAcl);
        $macro!(PutBucketPolicy);
        $macro!(ListMultipartUploads);
        $macro!(CompleteMultipartUpload);
        $macro!(HeadBucket);
        $macro!(PutBucketAccelerateConfiguration);
        $macro!(HeadObject);
        $macro!(PutBucketCors);
        $macro!(ListObjectVersions);
        $macro!(GetObjectLegalHold);
        $macro!(DeleteBucketReplication);
        $macro!(GetBucketLogging);
        $macro!(PutBucketInventoryConfiguration);
    };
}
#[doc = r" This macro calls a provided $macro for each operation to generate item per op."]
macro_rules ! generate_ops_items { ($ macro : ident) => { $ macro ! (PutBucketLifecycleConfiguration) , $ macro ! (GetBucketAnalyticsConfiguration) , $ macro ! (PutBucketMetricsConfiguration) , $ macro ! (GetObjectRetention) , $ macro ! (PutBucketAnalyticsConfiguration) , $ macro ! (PutPublicAccessBlock) , $ macro ! (PutObject) , $ macro ! (CreateMultipartUpload) , $ macro ! (WriteGetObjectResponse) , $ macro ! (PutBucketVersioning) , $ macro ! (GetBucketWebsite) , $ macro ! (GetBucketOwnershipControls) , $ macro ! (GetObjectLockConfiguration) , $ macro ! (ListObjectsV2) , $ macro ! (GetBucketNotificationConfiguration) , $ macro ! (ListParts) , $ macro ! (DeleteBucket) , $ macro ! (DeleteBucketMetricsConfiguration) , $ macro ! (DeleteBucketPolicy) , $ macro ! (GetObject) , $ macro ! (GetBucketAccelerateConfiguration) , $ macro ! (SelectObjectContent) , $ macro ! (UploadPartCopy) , $ macro ! (PutObjectLegalHold) , $ macro ! (PutObjectRetention) , $ macro ! (GetBucketReplication) , $ macro ! (GetBucketTagging) , $ macro ! (PutObjectLockConfiguration) , $ macro ! (GetBucketPolicyStatus) , $ macro ! (GetBucketPolicy) , $ macro ! (GetObjectAcl) , $ macro ! (GetObjectAttributes) , $ macro ! (PutBucketTagging) , $ macro ! (CopyObject) , $ macro ! (ListObjects) , $ macro ! (DeleteBucketAnalyticsConfiguration) , $ macro ! (DeleteObjects) , $ macro ! (ListBucketMetricsConfigurations) , $ macro ! (DeleteBucketEncryption) , $ macro ! (PutBucketLogging) , $ macro ! (ListBucketAnalyticsConfigurations) , $ macro ! (PutBucketWebsite) , $ macro ! (PutBucketNotificationConfiguration) , $ macro ! (GetBucketRequestPayment) , $ macro ! (PutBucketIntelligentTieringConfiguration) , $ macro ! (GetBucketIntelligentTieringConfiguration) , $ macro ! (DeleteBucketWebsite) , $ macro ! (DeleteObject) , $ macro ! (ListBucketIntelligentTieringConfigurations) , $ macro ! (PutBucketReplication) , $ macro ! (GetPublicAccessBlock) , $ macro ! (DeleteBucketLifecycle) , $ macro ! (PutObjectAcl) , $ macro ! (DeleteObjectTagging) , $ macro ! (ListBucketInventoryConfigurations) , $ macro ! (GetObjectTagging) , $ macro ! (DeleteBucketCors) , $ macro ! (PutBucketRequestPayment) , $ macro ! (GetBucketLocation) , $ macro ! (ListBuckets) , $ macro ! (DeleteBucketIntelligentTieringConfiguration) , $ macro ! (DeletePublicAccessBlock) , $ macro ! (GetBucketCors) , $ macro ! (GetBucketAcl) , $ macro ! (RestoreObject) , $ macro ! (PutObjectTagging) , $ macro ! (GetBucketInventoryConfiguration) , $ macro ! (GetBucketEncryption) , $ macro ! (AbortMultipartUpload) , $ macro ! (DeleteBucketTagging) , $ macro ! (PutBucketOwnershipControls) , $ macro ! (DeleteBucketOwnershipControls) , $ macro ! (GetBucketMetricsConfiguration) , $ macro ! (UploadPart) , $ macro ! (GetBucketVersioning) , $ macro ! (GetObjectTorrent) , $ macro ! (PutBucketEncryption) , $ macro ! (CreateBucket) , $ macro ! (DeleteBucketInventoryConfiguration) , $ macro ! (GetBucketLifecycleConfiguration) , $ macro ! (PutBucketAcl) , $ macro ! (PutBucketPolicy) , $ macro ! (ListMultipartUploads) , $ macro ! (CompleteMultipartUpload) , $ macro ! (HeadBucket) , $ macro ! (PutBucketAccelerateConfiguration) , $ macro ! (HeadObject) , $ macro ! (PutBucketCors) , $ macro ! (ListObjectVersions) , $ macro ! (GetObjectLegalHold) , $ macro ! (DeleteBucketReplication) , $ macro ! (GetBucketLogging) , $ macro ! (PutBucketInventoryConfiguration) , } }
#[doc = r" This macro matches a variable of Ops type and expands the provided $macro"]
#[doc = r" for each operation to generate code handler per op."]
macro_rules! generate_ops_match {
    ($ macro : ident , $ op : expr) => {
        match ($op) {
            ops::Ops::PutBucketLifecycleConfiguration => $macro!(PutBucketLifecycleConfiguration),
            ops::Ops::GetBucketAnalyticsConfiguration => $macro!(GetBucketAnalyticsConfiguration),
            ops::Ops::PutBucketMetricsConfiguration => $macro!(PutBucketMetricsConfiguration),
            ops::Ops::GetObjectRetention => $macro!(GetObjectRetention),
            ops::Ops::PutBucketAnalyticsConfiguration => $macro!(PutBucketAnalyticsConfiguration),
            ops::Ops::PutPublicAccessBlock => $macro!(PutPublicAccessBlock),
            ops::Ops::PutObject => $macro!(PutObject),
            ops::Ops::CreateMultipartUpload => $macro!(CreateMultipartUpload),
            ops::Ops::WriteGetObjectResponse => $macro!(WriteGetObjectResponse),
            ops::Ops::PutBucketVersioning => $macro!(PutBucketVersioning),
            ops::Ops::GetBucketWebsite => $macro!(GetBucketWebsite),
            ops::Ops::GetBucketOwnershipControls => $macro!(GetBucketOwnershipControls),
            ops::Ops::GetObjectLockConfiguration => $macro!(GetObjectLockConfiguration),
            ops::Ops::ListObjectsV2 => $macro!(ListObjectsV2),
            ops::Ops::GetBucketNotificationConfiguration => {
                $macro!(GetBucketNotificationConfiguration)
            }
            ops::Ops::ListParts => $macro!(ListParts),
            ops::Ops::DeleteBucket => $macro!(DeleteBucket),
            ops::Ops::DeleteBucketMetricsConfiguration => $macro!(DeleteBucketMetricsConfiguration),
            ops::Ops::DeleteBucketPolicy => $macro!(DeleteBucketPolicy),
            ops::Ops::GetObject => $macro!(GetObject),
            ops::Ops::GetBucketAccelerateConfiguration => $macro!(GetBucketAccelerateConfiguration),
            ops::Ops::SelectObjectContent => $macro!(SelectObjectContent),
            ops::Ops::UploadPartCopy => $macro!(UploadPartCopy),
            ops::Ops::PutObjectLegalHold => $macro!(PutObjectLegalHold),
            ops::Ops::PutObjectRetention => $macro!(PutObjectRetention),
            ops::Ops::GetBucketReplication => $macro!(GetBucketReplication),
            ops::Ops::GetBucketTagging => $macro!(GetBucketTagging),
            ops::Ops::PutObjectLockConfiguration => $macro!(PutObjectLockConfiguration),
            ops::Ops::GetBucketPolicyStatus => $macro!(GetBucketPolicyStatus),
            ops::Ops::GetBucketPolicy => $macro!(GetBucketPolicy),
            ops::Ops::GetObjectAcl => $macro!(GetObjectAcl),
            ops::Ops::GetObjectAttributes => $macro!(GetObjectAttributes),
            ops::Ops::PutBucketTagging => $macro!(PutBucketTagging),
            ops::Ops::CopyObject => $macro!(CopyObject),
            ops::Ops::ListObjects => $macro!(ListObjects),
            ops::Ops::DeleteBucketAnalyticsConfiguration => {
                $macro!(DeleteBucketAnalyticsConfiguration)
            }
            ops::Ops::DeleteObjects => $macro!(DeleteObjects),
            ops::Ops::ListBucketMetricsConfigurations => $macro!(ListBucketMetricsConfigurations),
            ops::Ops::DeleteBucketEncryption => $macro!(DeleteBucketEncryption),
            ops::Ops::PutBucketLogging => $macro!(PutBucketLogging),
            ops::Ops::ListBucketAnalyticsConfigurations => {
                $macro!(ListBucketAnalyticsConfigurations)
            }
            ops::Ops::PutBucketWebsite => $macro!(PutBucketWebsite),
            ops::Ops::PutBucketNotificationConfiguration => {
                $macro!(PutBucketNotificationConfiguration)
            }
            ops::Ops::GetBucketRequestPayment => $macro!(GetBucketRequestPayment),
            ops::Ops::PutBucketIntelligentTieringConfiguration => {
                $macro!(PutBucketIntelligentTieringConfiguration)
            }
            ops::Ops::GetBucketIntelligentTieringConfiguration => {
                $macro!(GetBucketIntelligentTieringConfiguration)
            }
            ops::Ops::DeleteBucketWebsite => $macro!(DeleteBucketWebsite),
            ops::Ops::DeleteObject => $macro!(DeleteObject),
            ops::Ops::ListBucketIntelligentTieringConfigurations => {
                $macro!(ListBucketIntelligentTieringConfigurations)
            }
            ops::Ops::PutBucketReplication => $macro!(PutBucketReplication),
            ops::Ops::GetPublicAccessBlock => $macro!(GetPublicAccessBlock),
            ops::Ops::DeleteBucketLifecycle => $macro!(DeleteBucketLifecycle),
            ops::Ops::PutObjectAcl => $macro!(PutObjectAcl),
            ops::Ops::DeleteObjectTagging => $macro!(DeleteObjectTagging),
            ops::Ops::ListBucketInventoryConfigurations => {
                $macro!(ListBucketInventoryConfigurations)
            }
            ops::Ops::GetObjectTagging => $macro!(GetObjectTagging),
            ops::Ops::DeleteBucketCors => $macro!(DeleteBucketCors),
            ops::Ops::PutBucketRequestPayment => $macro!(PutBucketRequestPayment),
            ops::Ops::GetBucketLocation => $macro!(GetBucketLocation),
            ops::Ops::ListBuckets => $macro!(ListBuckets),
            ops::Ops::DeleteBucketIntelligentTieringConfiguration => {
                $macro!(DeleteBucketIntelligentTieringConfiguration)
            }
            ops::Ops::DeletePublicAccessBlock => $macro!(DeletePublicAccessBlock),
            ops::Ops::GetBucketCors => $macro!(GetBucketCors),
            ops::Ops::GetBucketAcl => $macro!(GetBucketAcl),
            ops::Ops::RestoreObject => $macro!(RestoreObject),
            ops::Ops::PutObjectTagging => $macro!(PutObjectTagging),
            ops::Ops::GetBucketInventoryConfiguration => $macro!(GetBucketInventoryConfiguration),
            ops::Ops::GetBucketEncryption => $macro!(GetBucketEncryption),
            ops::Ops::AbortMultipartUpload => $macro!(AbortMultipartUpload),
            ops::Ops::DeleteBucketTagging => $macro!(DeleteBucketTagging),
            ops::Ops::PutBucketOwnershipControls => $macro!(PutBucketOwnershipControls),
            ops::Ops::DeleteBucketOwnershipControls => $macro!(DeleteBucketOwnershipControls),
            ops::Ops::GetBucketMetricsConfiguration => $macro!(GetBucketMetricsConfiguration),
            ops::Ops::UploadPart => $macro!(UploadPart),
            ops::Ops::GetBucketVersioning => $macro!(GetBucketVersioning),
            ops::Ops::GetObjectTorrent => $macro!(GetObjectTorrent),
            ops::Ops::PutBucketEncryption => $macro!(PutBucketEncryption),
            ops::Ops::CreateBucket => $macro!(CreateBucket),
            ops::Ops::DeleteBucketInventoryConfiguration => {
                $macro!(DeleteBucketInventoryConfiguration)
            }
            ops::Ops::GetBucketLifecycleConfiguration => $macro!(GetBucketLifecycleConfiguration),
            ops::Ops::PutBucketAcl => $macro!(PutBucketAcl),
            ops::Ops::PutBucketPolicy => $macro!(PutBucketPolicy),
            ops::Ops::ListMultipartUploads => $macro!(ListMultipartUploads),
            ops::Ops::CompleteMultipartUpload => $macro!(CompleteMultipartUpload),
            ops::Ops::HeadBucket => $macro!(HeadBucket),
            ops::Ops::PutBucketAccelerateConfiguration => $macro!(PutBucketAccelerateConfiguration),
            ops::Ops::HeadObject => $macro!(HeadObject),
            ops::Ops::PutBucketCors => $macro!(PutBucketCors),
            ops::Ops::ListObjectVersions => $macro!(ListObjectVersions),
            ops::Ops::GetObjectLegalHold => $macro!(GetObjectLegalHold),
            ops::Ops::DeleteBucketReplication => $macro!(DeleteBucketReplication),
            ops::Ops::GetBucketLogging => $macro!(GetBucketLogging),
            ops::Ops::PutBucketInventoryConfiguration => $macro!(PutBucketInventoryConfiguration),
        }
    };
}
pub(crate) use generate_ops_code;
pub(crate) use generate_ops_items;
pub(crate) use generate_ops_match;
