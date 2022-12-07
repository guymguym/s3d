#![allow(unused)]
#![allow(non_camel_case_types)]
use aws_smithy_http_server::operation::OperationShape;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;
use std::sync::Arc;

#[derive(Debug, Default, Clone)]
pub struct PutObjectTaggingOutput {
    pub version_id: Option<ObjectVersionId>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum RestoreRequestType {
    SELECT,
}
impl AsRef<str> for RestoreRequestType {
    fn as_ref(&self) -> &str {
        match self {
            Self::SELECT => "SELECT",
        }
    }
}
impl TryFrom<&str> for RestoreRequestType {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "SELECT" => Ok(Self::SELECT),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Grantee {
    pub r#type: Option<Type>,
    pub uri: Option<Uri>,
    pub display_name: Option<DisplayName>,
    pub id: Option<Id>,
    pub email_address: Option<EmailAddress>,
}

pub type Marker = String;

#[derive(Debug, Default, Clone)]
pub struct AnalyticsConfiguration {
    pub id: Option<AnalyticsId>,
    pub storage_class_analysis: Option<StorageClassAnalysis>,
    pub filter: Option<AnalyticsFilter>,
}

#[derive(Debug, Default, Clone)]
pub struct HeadBucketRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum RequestPayer {
    requester,
}
impl AsRef<str> for RequestPayer {
    fn as_ref(&self) -> &str {
        match self {
            Self::requester => "requester",
        }
    }
}
impl TryFrom<&str> for RequestPayer {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "requester" => Ok(Self::requester),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct MetricsAndOperator {
    pub prefix: Option<Prefix>,
    pub tags: Option<TagSet>,
    pub access_point_arn: Option<AccessPointArn>,
}

pub type Prefix = String;

#[derive(Debug, Default, Clone)]
pub struct HeadBucket {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for HeadBucket {
    const NAME: &'static str = "HeadBucket";
    type Input = HeadBucketRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct PutBucketCorsRequest {
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub cors_configuration: Option<CorsConfiguration>,
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
    pub content_md5: Option<ContentMd5>,
}

#[derive(Debug, Default, Clone)]
pub struct UploadPartCopy {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for UploadPartCopy {
    const NAME: &'static str = "UploadPartCopy";
    type Input = UploadPartCopyRequest;
    type Output = UploadPartCopyOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct Tiering {
    pub days: Option<IntelligentTieringDays>,
    pub access_tier: Option<IntelligentTieringAccessTier>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ChecksumAlgorithm {
    CRC32,
    CRC32C,
    SHA1,
    SHA256,
}
impl AsRef<str> for ChecksumAlgorithm {
    fn as_ref(&self) -> &str {
        match self {
            Self::CRC32 => "CRC32",
            Self::CRC32C => "CRC32C",
            Self::SHA1 => "SHA1",
            Self::SHA256 => "SHA256",
        }
    }
}
impl TryFrom<&str> for ChecksumAlgorithm {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "CRC32" => Ok(Self::CRC32),
            "CRC32C" => Ok(Self::CRC32C),
            "SHA1" => Ok(Self::SHA1),
            "SHA256" => Ok(Self::SHA256),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

pub type CopySourceSseCustomerKey = String;

#[derive(Debug, Default, Clone)]
pub struct CorsConfiguration {
    pub cors_rules: Option<CorsRules>,
}

pub type ErrorCode = String;

#[derive(Debug, Default, Clone)]
pub struct GetObjectAclRequest {
    pub version_id: Option<ObjectVersionId>,
    pub bucket: Option<BucketName>,
    pub request_payer: Option<RequestPayer>,
    pub expected_bucket_owner: Option<AccountId>,
    pub key: Option<ObjectKey>,
}

#[derive(Debug, Default, Clone)]
pub struct BucketLifecycleConfiguration {
    pub rules: Option<LifecycleRules>,
}

pub type SkipValidation = bool;

pub type Comments = String;

#[derive(Debug, Default, Clone)]
pub struct GetObjectAclOutput {
    pub request_charged: Option<RequestCharged>,
    pub grants: Option<Grants>,
    pub owner: Option<Owner>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BucketVersioningStatus {
    Enabled,
    Suspended,
}
impl AsRef<str> for BucketVersioningStatus {
    fn as_ref(&self) -> &str {
        match self {
            Self::Enabled => "Enabled",
            Self::Suspended => "Suspended",
        }
    }
}
impl TryFrom<&str> for BucketVersioningStatus {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "Enabled" => Ok(Self::Enabled),
            "Suspended" => Ok(Self::Suspended),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetObjectAcl {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for GetObjectAcl {
    const NAME: &'static str = "GetObjectAcl";
    type Input = GetObjectAclRequest;
    type Output = GetObjectAclOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketWebsiteRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum JsonType {
    DOCUMENT,
    LINES,
}
impl AsRef<str> for JsonType {
    fn as_ref(&self) -> &str {
        match self {
            Self::DOCUMENT => "DOCUMENT",
            Self::LINES => "LINES",
        }
    }
}
impl TryFrom<&str> for JsonType {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "DOCUMENT" => Ok(Self::DOCUMENT),
            "LINES" => Ok(Self::LINES),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct PutObjectLegalHold {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for PutObjectLegalHold {
    const NAME: &'static str = "PutObjectLegalHold";
    type Input = PutObjectLegalHoldRequest;
    type Output = PutObjectLegalHoldOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct PutObjectLegalHoldRequest {
    pub content_md5: Option<ContentMd5>,
    pub expected_bucket_owner: Option<AccountId>,
    pub key: Option<ObjectKey>,
    pub bucket: Option<BucketName>,
    pub request_payer: Option<RequestPayer>,
    pub version_id: Option<ObjectVersionId>,
    pub legal_hold: Option<ObjectLockLegalHold>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
}

#[derive(Debug, Default, Clone)]
pub struct MetricsConfiguration {
    pub filter: Option<MetricsFilter>,
    pub id: Option<MetricsId>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ReplicationRuleStatus {
    Enabled,
    Disabled,
}
impl AsRef<str> for ReplicationRuleStatus {
    fn as_ref(&self) -> &str {
        match self {
            Self::Enabled => "Enabled",
            Self::Disabled => "Disabled",
        }
    }
}
impl TryFrom<&str> for ReplicationRuleStatus {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "Enabled" => Ok(Self::Enabled),
            "Disabled" => Ok(Self::Disabled),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Ssekms {
    pub key_id: Option<SsekmsKeyId>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum StorageClass {
    STANDARD,
    REDUCED_REDUNDANCY,
    STANDARD_IA,
    ONEZONE_IA,
    INTELLIGENT_TIERING,
    GLACIER,
    DEEP_ARCHIVE,
    OUTPOSTS,
    GLACIER_IR,
}
impl AsRef<str> for StorageClass {
    fn as_ref(&self) -> &str {
        match self {
            Self::STANDARD => "STANDARD",
            Self::REDUCED_REDUNDANCY => "REDUCED_REDUNDANCY",
            Self::STANDARD_IA => "STANDARD_IA",
            Self::ONEZONE_IA => "ONEZONE_IA",
            Self::INTELLIGENT_TIERING => "INTELLIGENT_TIERING",
            Self::GLACIER => "GLACIER",
            Self::DEEP_ARCHIVE => "DEEP_ARCHIVE",
            Self::OUTPOSTS => "OUTPOSTS",
            Self::GLACIER_IR => "GLACIER_IR",
        }
    }
}
impl TryFrom<&str> for StorageClass {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "STANDARD" => Ok(Self::STANDARD),
            "REDUCED_REDUNDANCY" => Ok(Self::REDUCED_REDUNDANCY),
            "STANDARD_IA" => Ok(Self::STANDARD_IA),
            "ONEZONE_IA" => Ok(Self::ONEZONE_IA),
            "INTELLIGENT_TIERING" => Ok(Self::INTELLIGENT_TIERING),
            "GLACIER" => Ok(Self::GLACIER),
            "DEEP_ARCHIVE" => Ok(Self::DEEP_ARCHIVE),
            "OUTPOSTS" => Ok(Self::OUTPOSTS),
            "GLACIER_IR" => Ok(Self::GLACIER_IR),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

pub type VersionCount = i32;

#[derive(Debug, Default, Clone)]
pub struct WriteGetObjectResponse {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for WriteGetObjectResponse {
    const NAME: &'static str = "WriteGetObjectResponse";
    type Input = WriteGetObjectResponseRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketAcl {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for GetBucketAcl {
    const NAME: &'static str = "GetBucketAcl";
    type Input = GetBucketAclRequest;
    type Output = GetBucketAclOutput;
    type Error = ();
}

pub type MaxParts = i32;

#[derive(Debug, Default, Clone)]
pub struct PutBucketVersioning {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for PutBucketVersioning {
    const NAME: &'static str = "PutBucketVersioning";
    type Input = PutBucketVersioningRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct PutObjectRetentionRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub version_id: Option<ObjectVersionId>,
    pub retention: Option<ObjectLockRetention>,
    pub content_md5: Option<ContentMd5>,
    pub request_payer: Option<RequestPayer>,
    pub bypass_governance_retention: Option<BypassGovernanceRetention>,
    pub bucket: Option<BucketName>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub key: Option<ObjectKey>,
}

#[derive(Debug, Default, Clone)]
pub struct PutBucketNotificationConfigurationRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
    pub notification_configuration: Option<NotificationConfiguration>,
    pub skip_destination_validation: Option<SkipValidation>,
}

pub type InventoryConfigurationList = Vec<InventoryConfiguration>;

#[derive(Debug, Default, Clone)]
pub struct DeleteObjectOutput {
    pub delete_marker: Option<DeleteMarker>,
    pub version_id: Option<ObjectVersionId>,
    pub request_charged: Option<RequestCharged>,
}

#[derive(Debug, Default, Clone)]
pub struct WriteGetObjectResponseRequest {
    pub body: Option<StreamingBlob>,
    pub parts_count: Option<PartsCount>,
    pub metadata: Option<Metadata>,
    pub request_token: Option<RequestToken>,
    pub content_language: Option<ContentLanguage>,
    pub replication_status: Option<ReplicationStatus>,
    pub content_range: Option<ContentRange>,
    pub checksum_sha256: Option<ChecksumSha256>,
    pub expires: Option<Expires>,
    pub object_lock_legal_hold_status: Option<ObjectLockLegalHoldStatus>,
    pub content_disposition: Option<ContentDisposition>,
    pub content_encoding: Option<ContentEncoding>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub checksum_sha1: Option<ChecksumSha1>,
    pub last_modified: Option<LastModified>,
    pub e_tag: Option<ETag>,
    pub status_code: Option<GetObjectResponseStatusCode>,
    pub content_type: Option<ContentType>,
    pub request_charged: Option<RequestCharged>,
    pub storage_class: Option<StorageClass>,
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
    pub content_length: Option<ContentLength>,
    pub expiration: Option<Expiration>,
    pub object_lock_mode: Option<ObjectLockMode>,
    pub missing_meta: Option<MissingMeta>,
    pub object_lock_retain_until_date: Option<ObjectLockRetainUntilDate>,
    pub version_id: Option<ObjectVersionId>,
    pub cache_control: Option<CacheControl>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub delete_marker: Option<DeleteMarker>,
    pub accept_ranges: Option<AcceptRanges>,
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    pub error_code: Option<ErrorCode>,
    pub tag_count: Option<TagCount>,
    pub error_message: Option<ErrorMessage>,
    pub checksum_crc32: Option<ChecksumCrc32>,
    pub request_route: Option<RequestRoute>,
    pub server_side_encryption: Option<ServerSideEncryption>,
    pub restore: Option<Restore>,
    pub ssekms_key_id: Option<SsekmsKeyId>,
}

pub type KeyMarker = String;

#[derive(Debug, Default, Clone)]
pub struct StatsEvent {
    pub details: Option<Stats>,
}

#[derive(Debug, Default, Clone)]
pub struct RecordsEvent {
    pub payload: Option<Body>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum MetricsStatus {
    Enabled,
    Disabled,
}
impl AsRef<str> for MetricsStatus {
    fn as_ref(&self) -> &str {
        match self {
            Self::Enabled => "Enabled",
            Self::Disabled => "Disabled",
        }
    }
}
impl TryFrom<&str> for MetricsStatus {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "Enabled" => Ok(Self::Enabled),
            "Disabled" => Ok(Self::Disabled),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ParquetInput {}

#[derive(Debug, Default, Clone)]
pub struct TopicConfiguration {
    pub events: Option<EventList>,
    pub filter: Option<NotificationConfigurationFilter>,
    pub id: Option<NotificationId>,
    pub topic_arn: Option<TopicArn>,
}

#[derive(Debug, Default, Clone)]
pub struct CreateBucketOutput {
    pub location: Option<Location>,
}

#[derive(Debug, Default, Clone)]
pub struct ListBucketMetricsConfigurations {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for ListBucketMetricsConfigurations {
    const NAME: &'static str = "ListBucketMetricsConfigurations";
    type Input = ListBucketMetricsConfigurationsRequest;
    type Output = ListBucketMetricsConfigurationsOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketMetricsConfigurationOutput {
    pub metrics_configuration: Option<MetricsConfiguration>,
}

pub type MetadataValue = String;

#[derive(Debug, Default, Clone)]
pub struct GetObjectLegalHoldRequest {
    pub key: Option<ObjectKey>,
    pub bucket: Option<BucketName>,
    pub request_payer: Option<RequestPayer>,
    pub expected_bucket_owner: Option<AccountId>,
    pub version_id: Option<ObjectVersionId>,
}

pub type Expires = String;

#[derive(Debug, Default, Clone)]
pub struct GetBucketAnalyticsConfigurationOutput {
    pub analytics_configuration: Option<AnalyticsConfiguration>,
}

#[derive(Debug, Default, Clone)]
pub struct RestoreRequest {
    pub output_location: Option<OutputLocation>,
    pub tier: Option<Tier>,
    pub days: Option<Days>,
    pub glacier_job_parameters: Option<GlacierJobParameters>,
    pub select_parameters: Option<SelectParameters>,
    pub description: Option<Description>,
    pub r#type: Option<RestoreRequestType>,
}

pub type Metadata = HashMap<MetadataKey, MetadataValue>;

#[derive(Debug, Default, Clone)]
pub struct HeadObjectOutput {
    pub content_length: Option<ContentLength>,
    pub metadata: Option<Metadata>,
    pub delete_marker: Option<DeleteMarker>,
    pub cache_control: Option<CacheControl>,
    pub last_modified: Option<LastModified>,
    pub website_redirect_location: Option<WebsiteRedirectLocation>,
    pub object_lock_retain_until_date: Option<ObjectLockRetainUntilDate>,
    pub parts_count: Option<PartsCount>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub content_encoding: Option<ContentEncoding>,
    pub checksum_crc32: Option<ChecksumCrc32>,
    pub server_side_encryption: Option<ServerSideEncryption>,
    pub version_id: Option<ObjectVersionId>,
    pub checksum_sha256: Option<ChecksumSha256>,
    pub object_lock_mode: Option<ObjectLockMode>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub restore: Option<Restore>,
    pub archive_status: Option<ArchiveStatus>,
    pub checksum_sha1: Option<ChecksumSha1>,
    pub storage_class: Option<StorageClass>,
    pub content_disposition: Option<ContentDisposition>,
    pub ssekms_key_id: Option<SsekmsKeyId>,
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
    pub e_tag: Option<ETag>,
    pub missing_meta: Option<MissingMeta>,
    pub expires: Option<Expires>,
    pub object_lock_legal_hold_status: Option<ObjectLockLegalHoldStatus>,
    pub request_charged: Option<RequestCharged>,
    pub accept_ranges: Option<AcceptRanges>,
    pub content_language: Option<ContentLanguage>,
    pub content_type: Option<ContentType>,
    pub replication_status: Option<ReplicationStatus>,
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    pub expiration: Option<Expiration>,
}

#[derive(Debug, Default, Clone)]
pub struct PutObjectAclRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub grant_read_acp: Option<GrantReadAcp>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub grant_write_acp: Option<GrantWriteAcp>,
    pub access_control_policy: Option<AccessControlPolicy>,
    pub request_payer: Option<RequestPayer>,
    pub version_id: Option<ObjectVersionId>,
    pub acl: Option<ObjectCannedAcl>,
    pub content_md5: Option<ContentMd5>,
    pub grant_write: Option<GrantWrite>,
    pub bucket: Option<BucketName>,
    pub grant_full_control: Option<GrantFullControl>,
    pub grant_read: Option<GrantRead>,
    pub key: Option<ObjectKey>,
}

pub type AccessPointArn = String;

#[derive(Debug, Default, Clone)]
pub struct LifecycleExpiration {
    pub expired_object_delete_marker: Option<ExpiredObjectDeleteMarker>,
    pub date: Option<Date>,
    pub days: Option<Days>,
}

#[derive(Debug, Default, Clone)]
pub struct Progress {
    pub bytes_returned: Option<BytesReturned>,
    pub bytes_scanned: Option<BytesScanned>,
    pub bytes_processed: Option<BytesProcessed>,
}

#[derive(Debug, Default, Clone)]
pub struct PutObjectRetentionOutput {
    pub request_charged: Option<RequestCharged>,
}

pub type ResponseContentEncoding = String;

pub type CopySourceRange = String;

#[derive(Debug, Default, Clone)]
pub struct GetBucketReplication {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for GetBucketReplication {
    const NAME: &'static str = "GetBucketReplication";
    type Input = GetBucketReplicationRequest;
    type Output = GetBucketReplicationOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct EndEvent {}

pub type ContentType = String;

#[derive(Debug, Default, Clone)]
pub struct DeleteObjects {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for DeleteObjects {
    const NAME: &'static str = "DeleteObjects";
    type Input = DeleteObjectsRequest;
    type Output = DeleteObjectsOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetObjectAttributes {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for GetObjectAttributes {
    const NAME: &'static str = "GetObjectAttributes";
    type Input = GetObjectAttributesRequest;
    type Output = GetObjectAttributesOutput;
    type Error = ();
}

pub type TaggingHeader = String;

pub type ErrorMessage = String;

pub type PartNumber = i32;

pub type MaxAgeSeconds = i32;

#[derive(Debug, Default, Clone)]
pub struct PutBucketInventoryConfigurationRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
    pub inventory_configuration: Option<InventoryConfiguration>,
    pub id: Option<InventoryId>,
}

pub type QueueConfigurationList = Vec<QueueConfiguration>;

#[derive(Debug, Default, Clone)]
pub struct GetBucketAccelerateConfigurationOutput {
    pub status: Option<BucketAccelerateStatus>,
}

#[derive(Debug, Default, Clone)]
pub struct Tagging {
    pub tag_set: Option<TagSet>,
}

#[derive(Debug, Default, Clone)]
pub struct Initiator {
    pub id: Option<Id>,
    pub display_name: Option<DisplayName>,
}

#[derive(Debug, Default, Clone)]
pub struct PutBucketWebsiteRequest {
    pub content_md5: Option<ContentMd5>,
    pub bucket: Option<BucketName>,
    pub website_configuration: Option<WebsiteConfiguration>,
    pub expected_bucket_owner: Option<AccountId>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketPolicyRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}

pub type CompletedPartList = Vec<CompletedPart>;

#[derive(Debug, Default, Clone)]
pub struct GetBucketLifecycleConfiguration {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for GetBucketLifecycleConfiguration {
    const NAME: &'static str = "GetBucketLifecycleConfiguration";
    type Input = GetBucketLifecycleConfigurationRequest;
    type Output = GetBucketLifecycleConfigurationOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetObjectLockConfigurationOutput {
    pub object_lock_configuration: Option<ObjectLockConfiguration>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum InventoryOptionalField {
    Size,
    LastModifiedDate,
    StorageClass,
    ETag,
    IsMultipartUploaded,
    ReplicationStatus,
    EncryptionStatus,
    ObjectLockRetainUntilDate,
    ObjectLockMode,
    ObjectLockLegalHoldStatus,
    IntelligentTieringAccessTier,
    BucketKeyStatus,
    ChecksumAlgorithm,
}
impl AsRef<str> for InventoryOptionalField {
    fn as_ref(&self) -> &str {
        match self {
            Self::Size => "Size",
            Self::LastModifiedDate => "LastModifiedDate",
            Self::StorageClass => "StorageClass",
            Self::ETag => "ETag",
            Self::IsMultipartUploaded => "IsMultipartUploaded",
            Self::ReplicationStatus => "ReplicationStatus",
            Self::EncryptionStatus => "EncryptionStatus",
            Self::ObjectLockRetainUntilDate => "ObjectLockRetainUntilDate",
            Self::ObjectLockMode => "ObjectLockMode",
            Self::ObjectLockLegalHoldStatus => "ObjectLockLegalHoldStatus",
            Self::IntelligentTieringAccessTier => "IntelligentTieringAccessTier",
            Self::BucketKeyStatus => "BucketKeyStatus",
            Self::ChecksumAlgorithm => "ChecksumAlgorithm",
        }
    }
}
impl TryFrom<&str> for InventoryOptionalField {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "Size" => Ok(Self::Size),
            "LastModifiedDate" => Ok(Self::LastModifiedDate),
            "StorageClass" => Ok(Self::StorageClass),
            "ETag" => Ok(Self::ETag),
            "IsMultipartUploaded" => Ok(Self::IsMultipartUploaded),
            "ReplicationStatus" => Ok(Self::ReplicationStatus),
            "EncryptionStatus" => Ok(Self::EncryptionStatus),
            "ObjectLockRetainUntilDate" => Ok(Self::ObjectLockRetainUntilDate),
            "ObjectLockMode" => Ok(Self::ObjectLockMode),
            "ObjectLockLegalHoldStatus" => Ok(Self::ObjectLockLegalHoldStatus),
            "IntelligentTieringAccessTier" => Ok(Self::IntelligentTieringAccessTier),
            "BucketKeyStatus" => Ok(Self::BucketKeyStatus),
            "ChecksumAlgorithm" => Ok(Self::ChecksumAlgorithm),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

pub type IsLatest = bool;

#[derive(Debug, Default, Clone)]
pub struct RestoreObjectOutput {
    pub restore_output_path: Option<RestoreOutputPath>,
    pub request_charged: Option<RequestCharged>,
}

pub type ReplaceKeyWith = String;

#[derive(Debug, Default, Clone)]
pub struct ListObjectsOutput {
    pub prefix: Option<Prefix>,
    pub delimiter: Option<Delimiter>,
    pub name: Option<BucketName>,
    pub marker: Option<Marker>,
    pub next_marker: Option<NextMarker>,
    pub common_prefixes: Option<CommonPrefixList>,
    pub contents: Option<ObjectList>,
    pub encoding_type: Option<EncodingType>,
    pub is_truncated: Option<IsTruncated>,
    pub max_keys: Option<MaxKeys>,
}

#[derive(Debug, Default, Clone)]
pub struct ServerSideEncryptionByDefault {
    pub kms_master_key_id: Option<SsekmsKeyId>,
    pub sse_algorithm: Option<ServerSideEncryption>,
}

#[derive(Debug, Default, Clone)]
pub struct GetObjectRetentionOutput {
    pub retention: Option<ObjectLockRetention>,
}

pub type CorsRules = Vec<CorsRule>;

pub type SseCustomerAlgorithm = String;

#[derive(Debug, Default, Clone)]
pub struct CommonPrefix {
    pub prefix: Option<Prefix>,
}

#[derive(Debug, Default, Clone)]
pub struct InventoryDestination {
    pub s3_bucket_destination: Option<InventoryS3BucketDestination>,
}

pub type MultipartUploadList = Vec<MultipartUpload>;

#[derive(Debug, Default, Clone)]
pub struct PutObjectRequest {
    pub bucket: Option<BucketName>,
    pub checksum_crc32: Option<ChecksumCrc32>,
    pub content_md5: Option<ContentMd5>,
    pub grant_read: Option<GrantRead>,
    pub object_lock_mode: Option<ObjectLockMode>,
    pub ssekms_encryption_context: Option<SsekmsEncryptionContext>,
    pub expires: Option<Expires>,
    pub body: Option<StreamingBlob>,
    pub grant_write_acp: Option<GrantWriteAcp>,
    pub cache_control: Option<CacheControl>,
    pub metadata: Option<Metadata>,
    pub acl: Option<ObjectCannedAcl>,
    pub grant_read_acp: Option<GrantReadAcp>,
    pub object_lock_legal_hold_status: Option<ObjectLockLegalHoldStatus>,
    pub sse_customer_key: Option<SseCustomerKey>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub server_side_encryption: Option<ServerSideEncryption>,
    pub key: Option<ObjectKey>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub content_disposition: Option<ContentDisposition>,
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    pub grant_full_control: Option<GrantFullControl>,
    pub ssekms_key_id: Option<SsekmsKeyId>,
    pub content_type: Option<ContentType>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub checksum_sha256: Option<ChecksumSha256>,
    pub expected_bucket_owner: Option<AccountId>,
    pub storage_class: Option<StorageClass>,
    pub tagging: Option<TaggingHeader>,
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
    pub checksum_sha1: Option<ChecksumSha1>,
    pub object_lock_retain_until_date: Option<ObjectLockRetainUntilDate>,
    pub content_encoding: Option<ContentEncoding>,
    pub content_length: Option<ContentLength>,
    pub website_redirect_location: Option<WebsiteRedirectLocation>,
    pub content_language: Option<ContentLanguage>,
    pub request_payer: Option<RequestPayer>,
}

#[derive(Debug, Default, Clone)]
pub struct DeletePublicAccessBlock {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for DeletePublicAccessBlock {
    const NAME: &'static str = "DeletePublicAccessBlock";
    type Input = DeletePublicAccessBlockRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct StorageClassAnalysis {
    pub data_export: Option<StorageClassAnalysisDataExport>,
}

#[derive(Debug, Default, Clone)]
pub struct AccelerateConfiguration {
    pub status: Option<BucketAccelerateStatus>,
}

pub type HttpErrorCodeReturnedEquals = String;

pub type NextUploadIdMarker = String;

#[derive(Debug, Default, Clone)]
pub struct RequestProgress {
    pub enabled: Option<EnableRequestProgress>,
}

pub type AllowedHeader = String;

pub type TargetPrefix = String;

pub type AllowQuotedRecordDelimiter = bool;

#[derive(Debug, Default, Clone)]
pub struct UploadPart {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for UploadPart {
    const NAME: &'static str = "UploadPart";
    type Input = UploadPartRequest;
    type Output = UploadPartOutput;
    type Error = ();
}

pub type AbortDate = String;

pub type FilterRuleList = Vec<FilterRule>;

pub type MetricsConfigurationList = Vec<MetricsConfiguration>;

#[derive(Debug, Default, Clone)]
pub struct OutputSerialization {
    pub json: Option<JsonOutput>,
    pub csv: Option<CsvOutput>,
}

pub type AllowedOrigin = String;

pub type ResponseContentLanguage = String;

#[derive(Debug, Default, Clone)]
pub struct GetBucketIntelligentTieringConfigurationOutput {
    pub intelligent_tiering_configuration: Option<IntelligentTieringConfiguration>,
}

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketAnalyticsConfigurationRequest {
    pub id: Option<AnalyticsId>,
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketRequestPayment {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for GetBucketRequestPayment {
    const NAME: &'static str = "GetBucketRequestPayment";
    type Input = GetBucketRequestPaymentRequest;
    type Output = GetBucketRequestPaymentOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct NotificationConfigurationFilter {
    pub key: Option<S3KeyFilter>,
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketIntelligentTieringConfiguration {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for GetBucketIntelligentTieringConfiguration {
    const NAME: &'static str = "GetBucketIntelligentTieringConfiguration";
    type Input = GetBucketIntelligentTieringConfigurationRequest;
    type Output = GetBucketIntelligentTieringConfigurationOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct AnalyticsExportDestination {
    pub s3_bucket_destination: Option<AnalyticsS3BucketDestination>,
}

#[derive(Debug, Default, Clone)]
pub struct ObjectAlreadyInActiveTierError {}

pub type EmailAddress = String;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum FileHeaderInfo {
    USE,
    IGNORE,
    NONE,
}
impl AsRef<str> for FileHeaderInfo {
    fn as_ref(&self) -> &str {
        match self {
            Self::USE => "USE",
            Self::IGNORE => "IGNORE",
            Self::NONE => "NONE",
        }
    }
}
impl TryFrom<&str> for FileHeaderInfo {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "USE" => Ok(Self::USE),
            "IGNORE" => Ok(Self::IGNORE),
            "NONE" => Ok(Self::NONE),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ObjectStorageClass {
    STANDARD,
    REDUCED_REDUNDANCY,
    GLACIER,
    STANDARD_IA,
    ONEZONE_IA,
    INTELLIGENT_TIERING,
    DEEP_ARCHIVE,
    OUTPOSTS,
    GLACIER_IR,
}
impl AsRef<str> for ObjectStorageClass {
    fn as_ref(&self) -> &str {
        match self {
            Self::STANDARD => "STANDARD",
            Self::REDUCED_REDUNDANCY => "REDUCED_REDUNDANCY",
            Self::GLACIER => "GLACIER",
            Self::STANDARD_IA => "STANDARD_IA",
            Self::ONEZONE_IA => "ONEZONE_IA",
            Self::INTELLIGENT_TIERING => "INTELLIGENT_TIERING",
            Self::DEEP_ARCHIVE => "DEEP_ARCHIVE",
            Self::OUTPOSTS => "OUTPOSTS",
            Self::GLACIER_IR => "GLACIER_IR",
        }
    }
}
impl TryFrom<&str> for ObjectStorageClass {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "STANDARD" => Ok(Self::STANDARD),
            "REDUCED_REDUNDANCY" => Ok(Self::REDUCED_REDUNDANCY),
            "GLACIER" => Ok(Self::GLACIER),
            "STANDARD_IA" => Ok(Self::STANDARD_IA),
            "ONEZONE_IA" => Ok(Self::ONEZONE_IA),
            "INTELLIGENT_TIERING" => Ok(Self::INTELLIGENT_TIERING),
            "DEEP_ARCHIVE" => Ok(Self::DEEP_ARCHIVE),
            "OUTPOSTS" => Ok(Self::OUTPOSTS),
            "GLACIER_IR" => Ok(Self::GLACIER_IR),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketCorsRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug, Default, Clone)]
pub struct GetObjectTaggingRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub request_payer: Option<RequestPayer>,
    pub key: Option<ObjectKey>,
    pub bucket: Option<BucketName>,
    pub version_id: Option<ObjectVersionId>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BucketCannedAcl {
    private,
    public_read,
    public_read_write,
    authenticated_read,
}
impl AsRef<str> for BucketCannedAcl {
    fn as_ref(&self) -> &str {
        match self {
            Self::private => "private",
            Self::public_read => "public-read",
            Self::public_read_write => "public-read-write",
            Self::authenticated_read => "authenticated-read",
        }
    }
}
impl TryFrom<&str> for BucketCannedAcl {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "private" => Ok(Self::private),
            "public-read" => Ok(Self::public_read),
            "public-read-write" => Ok(Self::public_read_write),
            "authenticated-read" => Ok(Self::authenticated_read),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

pub type MissingMeta = i32;

pub type Parts = Vec<Part>;

#[derive(Debug, Default, Clone)]
pub struct GetObjectLockConfigurationRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug, Default, Clone)]
pub struct S3Location {
    pub access_control_list: Option<Grants>,
    pub prefix: Option<LocationPrefix>,
    pub encryption: Option<Encryption>,
    pub canned_acl: Option<ObjectCannedAcl>,
    pub storage_class: Option<StorageClass>,
    pub tagging: Option<Tagging>,
    pub user_metadata: Option<UserMetadata>,
    pub bucket_name: Option<BucketName>,
}

#[derive(Debug, Default, Clone)]
pub struct CopyObjectResult {
    pub e_tag: Option<ETag>,
    pub checksum_sha1: Option<ChecksumSha1>,
    pub last_modified: Option<LastModified>,
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
    pub checksum_crc32: Option<ChecksumCrc32>,
    pub checksum_sha256: Option<ChecksumSha256>,
}

#[derive(Debug, Default, Clone)]
pub struct PutObjectLockConfigurationRequest {
    pub object_lock_configuration: Option<ObjectLockConfiguration>,
    pub bucket: Option<BucketName>,
    pub request_payer: Option<RequestPayer>,
    pub token: Option<ObjectLockToken>,
    pub expected_bucket_owner: Option<AccountId>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub content_md5: Option<ContentMd5>,
}

#[derive(Debug, Default, Clone)]
pub struct AnalyticsAndOperator {
    pub prefix: Option<Prefix>,
    pub tags: Option<TagSet>,
}

#[derive(Debug, Default, Clone)]
pub struct DefaultRetention {
    pub mode: Option<ObjectLockRetentionMode>,
    pub days: Option<Days>,
    pub years: Option<Years>,
}

pub type Mfa = String;

#[derive(Debug, Default, Clone)]
pub struct ReplicationTime {
    pub time: Option<ReplicationTimeValue>,
    pub status: Option<ReplicationTimeStatus>,
}

pub type ContentLength = i64;

pub type BytesScanned = i64;

#[derive(Debug, Default, Clone)]
pub struct GetBucketPolicyOutput {
    pub policy: Option<Policy>,
}

#[derive(Debug, Default, Clone)]
pub struct NoncurrentVersionExpiration {
    pub newer_noncurrent_versions: Option<VersionCount>,
    pub noncurrent_days: Option<Days>,
}

pub type KeyPrefixEquals = String;

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug, Clone)]
pub enum ReplicationRuleFilter {
    Prefix(Prefix),
    And(ReplicationRuleAndOperator),
    Tag(Tag),
}

#[derive(Debug, Default, Clone)]
pub struct Sses3 {}

#[derive(Debug, Default, Clone)]
pub struct GetPublicAccessBlock {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for GetPublicAccessBlock {
    const NAME: &'static str = "GetPublicAccessBlock";
    type Input = GetPublicAccessBlockRequest;
    type Output = GetPublicAccessBlockOutput;
    type Error = ();
}

pub type ObjectList = Vec<Object>;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ObjectOwnership {
    BucketOwnerPreferred,
    ObjectWriter,
    BucketOwnerEnforced,
}
impl AsRef<str> for ObjectOwnership {
    fn as_ref(&self) -> &str {
        match self {
            Self::BucketOwnerPreferred => "BucketOwnerPreferred",
            Self::ObjectWriter => "ObjectWriter",
            Self::BucketOwnerEnforced => "BucketOwnerEnforced",
        }
    }
}
impl TryFrom<&str> for ObjectOwnership {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "BucketOwnerPreferred" => Ok(Self::BucketOwnerPreferred),
            "ObjectWriter" => Ok(Self::ObjectWriter),
            "BucketOwnerEnforced" => Ok(Self::BucketOwnerEnforced),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

pub type BytesProcessed = i64;

pub type LocationPrefix = String;

pub type MetadataKey = String;

pub type TopicConfigurationList = Vec<TopicConfiguration>;

#[derive(Debug, Default, Clone)]
pub struct CreateBucketRequest {
    pub object_lock_enabled_for_bucket: Option<ObjectLockEnabledForBucket>,
    pub acl: Option<BucketCannedAcl>,
    pub grant_write: Option<GrantWrite>,
    pub object_ownership: Option<ObjectOwnership>,
    pub grant_full_control: Option<GrantFullControl>,
    pub bucket: Option<BucketName>,
    pub grant_read: Option<GrantRead>,
    pub grant_read_acp: Option<GrantReadAcp>,
    pub create_bucket_configuration: Option<CreateBucketConfiguration>,
    pub grant_write_acp: Option<GrantWriteAcp>,
}

#[derive(Debug, Default, Clone)]
pub struct MetadataEntry {
    pub name: Option<MetadataKey>,
    pub value: Option<MetadataValue>,
}

#[derive(Debug, Default, Clone)]
pub struct ListPartsOutput {
    pub request_charged: Option<RequestCharged>,
    pub owner: Option<Owner>,
    pub abort_date: Option<AbortDate>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub upload_id: Option<MultipartUploadId>,
    pub abort_rule_id: Option<AbortRuleId>,
    pub is_truncated: Option<IsTruncated>,
    pub parts: Option<Parts>,
    pub initiator: Option<Initiator>,
    pub next_part_number_marker: Option<NextPartNumberMarker>,
    pub key: Option<ObjectKey>,
    pub max_parts: Option<MaxParts>,
    pub storage_class: Option<StorageClass>,
    pub part_number_marker: Option<PartNumberMarker>,
    pub bucket: Option<BucketName>,
}

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketIntelligentTieringConfigurationRequest {
    pub bucket: Option<BucketName>,
    pub id: Option<IntelligentTieringId>,
}

pub type BytesReturned = i64;

#[derive(Debug, Default, Clone)]
pub struct ContinuationEvent {}

#[derive(Debug, Default, Clone)]
pub struct ListParts {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for ListParts {
    const NAME: &'static str = "ListParts";
    type Input = ListPartsRequest;
    type Output = ListPartsOutput;
    type Error = ();
}

pub type EventList = Vec<Event>;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Tier {
    Standard,
    Bulk,
    Expedited,
}
impl AsRef<str> for Tier {
    fn as_ref(&self) -> &str {
        match self {
            Self::Standard => "Standard",
            Self::Bulk => "Bulk",
            Self::Expedited => "Expedited",
        }
    }
}
impl TryFrom<&str> for Tier {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "Standard" => Ok(Self::Standard),
            "Bulk" => Ok(Self::Bulk),
            "Expedited" => Ok(Self::Expedited),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Destination {
    pub metrics: Option<Metrics>,
    pub storage_class: Option<StorageClass>,
    pub access_control_translation: Option<AccessControlTranslation>,
    pub account: Option<AccountId>,
    pub encryption_configuration: Option<EncryptionConfiguration>,
    pub replication_time: Option<ReplicationTime>,
    pub bucket: Option<BucketName>,
}

#[derive(Debug, Default, Clone)]
pub struct CompletedPart {
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
    pub part_number: Option<PartNumber>,
    pub checksum_sha1: Option<ChecksumSha1>,
    pub checksum_sha256: Option<ChecksumSha256>,
    pub checksum_crc32: Option<ChecksumCrc32>,
    pub e_tag: Option<ETag>,
}

#[derive(Debug, Default, Clone)]
pub struct InventoryS3BucketDestination {
    pub account_id: Option<AccountId>,
    pub encryption: Option<InventoryEncryption>,
    pub prefix: Option<Prefix>,
    pub bucket: Option<BucketName>,
    pub format: Option<InventoryFormat>,
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketAccelerateConfigurationRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}

pub type Size = i64;

#[derive(Debug, Default, Clone)]
pub struct PutBucketLogging {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for PutBucketLogging {
    const NAME: &'static str = "PutBucketLogging";
    type Input = PutBucketLoggingRequest;
    type Output = ();
    type Error = ();
}

pub type DeleteMarkerVersionId = String;

pub type ExposeHeader = String;

pub type IsEnabled = bool;

pub type Errors = Vec<Error>;

pub type MultipartUploadId = String;

#[derive(Debug, Default, Clone)]
pub struct PutBucketWebsite {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for PutBucketWebsite {
    const NAME: &'static str = "PutBucketWebsite";
    type Input = PutBucketWebsiteRequest;
    type Output = ();
    type Error = ();
}

pub type GrantReadAcp = String;

#[derive(Debug, Default, Clone)]
pub struct CopyObjectRequest {
    pub key: Option<ObjectKey>,
    pub expires: Option<Expires>,
    pub metadata_directive: Option<MetadataDirective>,
    pub grant_read: Option<GrantRead>,
    pub object_lock_legal_hold_status: Option<ObjectLockLegalHoldStatus>,
    pub copy_source_sse_customer_algorithm: Option<CopySourceSseCustomerAlgorithm>,
    pub object_lock_mode: Option<ObjectLockMode>,
    pub cache_control: Option<CacheControl>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub tagging: Option<TaggingHeader>,
    pub content_disposition: Option<ContentDisposition>,
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    pub content_type: Option<ContentType>,
    pub grant_full_control: Option<GrantFullControl>,
    pub grant_read_acp: Option<GrantReadAcp>,
    pub copy_source_if_match: Option<CopySourceIfMatch>,
    pub metadata: Option<Metadata>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub grant_write_acp: Option<GrantWriteAcp>,
    pub content_language: Option<ContentLanguage>,
    pub request_payer: Option<RequestPayer>,
    pub ssekms_key_id: Option<SsekmsKeyId>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub copy_source_if_unmodified_since: Option<CopySourceIfUnmodifiedSince>,
    pub expected_bucket_owner: Option<AccountId>,
    pub copy_source_if_modified_since: Option<CopySourceIfModifiedSince>,
    pub sse_customer_key: Option<SseCustomerKey>,
    pub website_redirect_location: Option<WebsiteRedirectLocation>,
    pub server_side_encryption: Option<ServerSideEncryption>,
    pub content_encoding: Option<ContentEncoding>,
    pub copy_source: Option<CopySource>,
    pub acl: Option<ObjectCannedAcl>,
    pub copy_source_if_none_match: Option<CopySourceIfNoneMatch>,
    pub bucket: Option<BucketName>,
    pub copy_source_sse_customer_key: Option<CopySourceSseCustomerKey>,
    pub copy_source_sse_customer_key_md5: Option<CopySourceSseCustomerKeyMd5>,
    pub object_lock_retain_until_date: Option<ObjectLockRetainUntilDate>,
    pub tagging_directive: Option<TaggingDirective>,
    pub ssekms_encryption_context: Option<SsekmsEncryptionContext>,
    pub storage_class: Option<StorageClass>,
    pub expected_source_bucket_owner: Option<AccountId>,
}

pub type CopySourceIfModifiedSince = String;

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketMetricsConfigurationRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
    pub id: Option<MetricsId>,
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketRequestPaymentOutput {
    pub payer: Option<Payer>,
}

#[derive(Debug, Default, Clone)]
pub struct CompletedMultipartUpload {
    pub parts: Option<CompletedPartList>,
}

pub type LambdaFunctionConfigurationList = Vec<LambdaFunctionConfiguration>;

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketTagging {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for DeleteBucketTagging {
    const NAME: &'static str = "DeleteBucketTagging";
    type Input = DeleteBucketTaggingRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetObjectAttributesRequest {
    pub version_id: Option<ObjectVersionId>,
    pub expected_bucket_owner: Option<AccountId>,
    pub request_payer: Option<RequestPayer>,
    pub sse_customer_key: Option<SseCustomerKey>,
    pub key: Option<ObjectKey>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub part_number_marker: Option<PartNumberMarker>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub bucket: Option<BucketName>,
    pub object_attributes: Option<ObjectAttributesList>,
    pub max_parts: Option<MaxParts>,
}

#[derive(Debug, Default, Clone)]
pub struct GetPublicAccessBlockOutput {
    pub public_access_block_configuration: Option<PublicAccessBlockConfiguration>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ReplicationStatus {
    COMPLETE,
    PENDING,
    FAILED,
    REPLICA,
}
impl AsRef<str> for ReplicationStatus {
    fn as_ref(&self) -> &str {
        match self {
            Self::COMPLETE => "COMPLETE",
            Self::PENDING => "PENDING",
            Self::FAILED => "FAILED",
            Self::REPLICA => "REPLICA",
        }
    }
}
impl TryFrom<&str> for ReplicationStatus {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "COMPLETE" => Ok(Self::COMPLETE),
            "PENDING" => Ok(Self::PENDING),
            "FAILED" => Ok(Self::FAILED),
            "REPLICA" => Ok(Self::REPLICA),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

pub type Setting = bool;

pub type ContentLanguage = String;

#[derive(Debug, Default, Clone)]
pub struct Encryption {
    pub kms_key_id: Option<SsekmsKeyId>,
    pub kms_context: Option<KmsContext>,
    pub encryption_type: Option<ServerSideEncryption>,
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketAnalyticsConfigurationRequest {
    pub id: Option<AnalyticsId>,
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug, Default, Clone)]
pub struct ListObjects {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for ListObjects {
    const NAME: &'static str = "ListObjects";
    type Input = ListObjectsRequest;
    type Output = ListObjectsOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct CreateBucket {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for CreateBucket {
    const NAME: &'static str = "CreateBucket";
    type Input = CreateBucketRequest;
    type Output = CreateBucketOutput;
    type Error = ();
}

pub type Location = String;

#[derive(Debug, Default, Clone)]
pub struct PutObjectLockConfigurationOutput {
    pub request_charged: Option<RequestCharged>,
}

#[derive(Debug, Default, Clone)]
pub struct UploadPartRequest {
    pub key: Option<ObjectKey>,
    pub checksum_sha1: Option<ChecksumSha1>,
    pub content_length: Option<ContentLength>,
    pub checksum_sha256: Option<ChecksumSha256>,
    pub part_number: Option<PartNumber>,
    pub checksum_crc32: Option<ChecksumCrc32>,
    pub body: Option<StreamingBlob>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub request_payer: Option<RequestPayer>,
    pub sse_customer_key: Option<SseCustomerKey>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub upload_id: Option<MultipartUploadId>,
    pub bucket: Option<BucketName>,
    pub content_md5: Option<ContentMd5>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub expected_bucket_owner: Option<AccountId>,
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketReplicationOutput {
    pub replication_configuration: Option<ReplicationConfiguration>,
}

#[derive(Debug, Default, Clone)]
pub struct ReplicaModifications {
    pub status: Option<ReplicaModificationsStatus>,
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketLifecycleConfigurationRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}

pub type RecordDelimiter = String;

#[derive(Debug, Default, Clone)]
pub struct ListObjectVersionsRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub delimiter: Option<Delimiter>,
    pub bucket: Option<BucketName>,
    pub encoding_type: Option<EncodingType>,
    pub key_marker: Option<KeyMarker>,
    pub version_id_marker: Option<VersionIdMarker>,
    pub max_keys: Option<MaxKeys>,
    pub prefix: Option<Prefix>,
}

pub type DeletedObjects = Vec<DeletedObject>;

pub type EnableRequestProgress = bool;

#[derive(Debug, Default, Clone)]
pub struct ListMultipartUploads {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for ListMultipartUploads {
    const NAME: &'static str = "ListMultipartUploads";
    type Input = ListMultipartUploadsRequest;
    type Output = ListMultipartUploadsOutput;
    type Error = ();
}

pub type StreamingBlob = Arc<hyper::Body>;

#[derive(Debug, Default, Clone)]
pub struct DeleteObject {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for DeleteObject {
    const NAME: &'static str = "DeleteObject";
    type Input = DeleteObjectRequest;
    type Output = DeleteObjectOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketEncryptionRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}

#[derive(Debug, Default, Clone)]
pub struct ReplicationTimeValue {
    pub minutes: Option<Minutes>,
}

#[derive(Debug, Default, Clone)]
pub struct OutputLocation {
    pub s3: Option<S3Location>,
}

pub type ChecksumCrc32 = String;

#[derive(Debug, Default, Clone)]
pub struct GetBucketReplicationRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}

pub type IntelligentTieringConfigurationList = Vec<IntelligentTieringConfiguration>;

#[derive(Debug, Default, Clone)]
pub struct DeleteObjectTaggingOutput {
    pub version_id: Option<ObjectVersionId>,
}

#[derive(Debug, Default, Clone)]
pub struct ListBucketMetricsConfigurationsOutput {
    pub is_truncated: Option<IsTruncated>,
    pub metrics_configuration_list: Option<MetricsConfigurationList>,
    pub continuation_token: Option<Token>,
    pub next_continuation_token: Option<NextToken>,
}

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketPolicyRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}

pub type InventoryOptionalFields = Vec<InventoryOptionalField>;

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketLifecycleRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug, Default, Clone)]
pub struct ListBucketsOutput {
    pub owner: Option<Owner>,
    pub buckets: Option<Buckets>,
}

#[derive(Debug, Default, Clone)]
pub struct PutObjectTaggingRequest {
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub tagging: Option<Tagging>,
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
    pub version_id: Option<ObjectVersionId>,
    pub content_md5: Option<ContentMd5>,
    pub key: Option<ObjectKey>,
    pub request_payer: Option<RequestPayer>,
}

#[derive(Debug, Default, Clone)]
pub struct PutBucketOwnershipControlsRequest {
    pub bucket: Option<BucketName>,
    pub content_md5: Option<ContentMd5>,
    pub ownership_controls: Option<OwnershipControls>,
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TaggingDirective {
    COPY,
    REPLACE,
}
impl AsRef<str> for TaggingDirective {
    fn as_ref(&self) -> &str {
        match self {
            Self::COPY => "COPY",
            Self::REPLACE => "REPLACE",
        }
    }
}
impl TryFrom<&str> for TaggingDirective {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "COPY" => Ok(Self::COPY),
            "REPLACE" => Ok(Self::REPLACE),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

pub type IfUnmodifiedSince = String;

#[derive(Debug, Default, Clone)]
pub struct QueueConfiguration {
    pub id: Option<NotificationId>,
    pub queue_arn: Option<QueueArn>,
    pub events: Option<EventList>,
    pub filter: Option<NotificationConfigurationFilter>,
}

#[derive(Debug, Default, Clone)]
pub struct ListBucketIntelligentTieringConfigurations {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for ListBucketIntelligentTieringConfigurations {
    const NAME: &'static str = "ListBucketIntelligentTieringConfigurations";
    type Input = ListBucketIntelligentTieringConfigurationsRequest;
    type Output = ListBucketIntelligentTieringConfigurationsOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct InvalidObjectState {
    pub access_tier: Option<IntelligentTieringAccessTier>,
    pub storage_class: Option<StorageClass>,
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketLogging {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for GetBucketLogging {
    const NAME: &'static str = "GetBucketLogging";
    type Input = GetBucketLoggingRequest;
    type Output = GetBucketLoggingOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct NoSuchUpload {}

#[derive(Debug, Default, Clone)]
pub struct WebsiteConfiguration {
    pub routing_rules: Option<RoutingRules>,
    pub error_document: Option<ErrorDocument>,
    pub index_document: Option<IndexDocument>,
    pub redirect_all_requests_to: Option<RedirectAllRequestsTo>,
}

#[derive(Debug, Default, Clone)]
pub struct CompleteMultipartUpload {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for CompleteMultipartUpload {
    const NAME: &'static str = "CompleteMultipartUpload";
    type Input = CompleteMultipartUploadRequest;
    type Output = CompleteMultipartUploadOutput;
    type Error = ();
}

pub type Id = String;

pub type IfMatch = String;

pub type VersionIdMarker = String;

#[derive(Debug, Default, Clone)]
pub struct EncryptionConfiguration {
    pub replica_kms_key_id: Option<ReplicaKmsKeyId>,
}

pub type Uri = String;

pub type Role = String;

pub type Code = String;

#[derive(Debug, Default, Clone)]
pub struct PutBucketReplicationRequest {
    pub replication_configuration: Option<ReplicationConfiguration>,
    pub token: Option<ObjectLockToken>,
    pub content_md5: Option<ContentMd5>,
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketLifecycleConfigurationOutput {
    pub rules: Option<LifecycleRules>,
}

#[derive(Debug, Default, Clone)]
pub struct GetObjectLockConfiguration {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for GetObjectLockConfiguration {
    const NAME: &'static str = "GetObjectLockConfiguration";
    type Input = GetObjectLockConfigurationRequest;
    type Output = GetObjectLockConfigurationOutput;
    type Error = ();
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum FilterRuleName {
    prefix,
    suffix,
}
impl AsRef<str> for FilterRuleName {
    fn as_ref(&self) -> &str {
        match self {
            Self::prefix => "prefix",
            Self::suffix => "suffix",
        }
    }
}
impl TryFrom<&str> for FilterRuleName {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "prefix" => Ok(Self::prefix),
            "suffix" => Ok(Self::suffix),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct CsvInput {
    pub quote_character: Option<QuoteCharacter>,
    pub record_delimiter: Option<RecordDelimiter>,
    pub quote_escape_character: Option<QuoteEscapeCharacter>,
    pub field_delimiter: Option<FieldDelimiter>,
    pub comments: Option<Comments>,
    pub file_header_info: Option<FileHeaderInfo>,
    pub allow_quoted_record_delimiter: Option<AllowQuotedRecordDelimiter>,
}

pub type Grants = Vec<Grant>;

#[derive(Debug, Default, Clone)]
pub struct GetBucketLoggingRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}

pub type QuoteEscapeCharacter = String;

pub type StartAfter = String;

pub type TagCount = i32;

#[derive(Debug, Default, Clone)]
pub struct ErrorDocument {
    pub key: Option<ObjectKey>,
}

#[derive(Debug, Default, Clone)]
pub struct PutBucketEncryption {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for PutBucketEncryption {
    const NAME: &'static str = "PutBucketEncryption";
    type Input = PutBucketEncryptionRequest;
    type Output = ();
    type Error = ();
}

pub type ReplaceKeyPrefixWith = String;

pub type Suffix = String;

#[derive(Debug, Default, Clone)]
pub struct GetObjectTagging {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for GetObjectTagging {
    const NAME: &'static str = "GetObjectTagging";
    type Input = GetObjectTaggingRequest;
    type Output = GetObjectTaggingOutput;
    type Error = ();
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum MfaDeleteStatus {
    Enabled,
    Disabled,
}
impl AsRef<str> for MfaDeleteStatus {
    fn as_ref(&self) -> &str {
        match self {
            Self::Enabled => "Enabled",
            Self::Disabled => "Disabled",
        }
    }
}
impl TryFrom<&str> for MfaDeleteStatus {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "Enabled" => Ok(Self::Enabled),
            "Disabled" => Ok(Self::Disabled),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct CopyObjectOutput {
    pub request_charged: Option<RequestCharged>,
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    pub copy_object_result: Option<CopyObjectResult>,
    pub ssekms_key_id: Option<SsekmsKeyId>,
    pub expiration: Option<Expiration>,
    pub server_side_encryption: Option<ServerSideEncryption>,
    pub version_id: Option<ObjectVersionId>,
    pub ssekms_encryption_context: Option<SsekmsEncryptionContext>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub copy_source_version_id: Option<CopySourceVersionId>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
}

#[derive(Debug, Default, Clone)]
pub struct ObjectLockRule {
    pub default_retention: Option<DefaultRetention>,
}

#[derive(Debug, Default, Clone)]
pub struct PutBucketAccelerateConfiguration {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for PutBucketAccelerateConfiguration {
    const NAME: &'static str = "PutBucketAccelerateConfiguration";
    type Input = PutBucketAccelerateConfigurationRequest;
    type Output = ();
    type Error = ();
}

pub type Policy = String;

pub type ResponseContentType = String;

pub type LambdaFunctionArn = String;

#[derive(Debug, Default, Clone)]
pub struct ListObjectVersionsOutput {
    pub common_prefixes: Option<CommonPrefixList>,
    pub next_version_id_marker: Option<NextVersionIdMarker>,
    pub prefix: Option<Prefix>,
    pub delete_markers: Option<DeleteMarkers>,
    pub encoding_type: Option<EncodingType>,
    pub is_truncated: Option<IsTruncated>,
    pub max_keys: Option<MaxKeys>,
    pub key_marker: Option<KeyMarker>,
    pub next_key_marker: Option<NextKeyMarker>,
    pub version_id_marker: Option<VersionIdMarker>,
    pub versions: Option<ObjectVersionList>,
    pub name: Option<BucketName>,
    pub delimiter: Option<Delimiter>,
}

pub type CommonPrefixList = Vec<CommonPrefix>;

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketOwnershipControls {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for DeleteBucketOwnershipControls {
    const NAME: &'static str = "DeleteBucketOwnershipControls";
    type Input = DeleteBucketOwnershipControlsRequest;
    type Output = ();
    type Error = ();
}

pub type GrantFullControl = String;

pub type ServerSideEncryptionRules = Vec<ServerSideEncryptionRule>;

#[derive(Debug, Default, Clone)]
pub struct ObjectLockRetention {
    pub mode: Option<ObjectLockRetentionMode>,
    pub retain_until_date: Option<Date>,
}

pub type ObjectSizeGreaterThanBytes = i64;

pub type Message = String;

pub type PartsCount = i32;

pub type ContentRange = String;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ExistingObjectReplicationStatus {
    Enabled,
    Disabled,
}
impl AsRef<str> for ExistingObjectReplicationStatus {
    fn as_ref(&self) -> &str {
        match self {
            Self::Enabled => "Enabled",
            Self::Disabled => "Disabled",
        }
    }
}
impl TryFrom<&str> for ExistingObjectReplicationStatus {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "Enabled" => Ok(Self::Enabled),
            "Disabled" => Ok(Self::Disabled),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetObject {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for GetObject {
    const NAME: &'static str = "GetObject";
    type Input = GetObjectRequest;
    type Output = GetObjectOutput;
    type Error = ();
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum InventoryIncludedObjectVersions {
    All,
    Current,
}
impl AsRef<str> for InventoryIncludedObjectVersions {
    fn as_ref(&self) -> &str {
        match self {
            Self::All => "All",
            Self::Current => "Current",
        }
    }
}
impl TryFrom<&str> for InventoryIncludedObjectVersions {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "All" => Ok(Self::All),
            "Current" => Ok(Self::Current),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum IntelligentTieringAccessTier {
    ARCHIVE_ACCESS,
    DEEP_ARCHIVE_ACCESS,
}
impl AsRef<str> for IntelligentTieringAccessTier {
    fn as_ref(&self) -> &str {
        match self {
            Self::ARCHIVE_ACCESS => "ARCHIVE_ACCESS",
            Self::DEEP_ARCHIVE_ACCESS => "DEEP_ARCHIVE_ACCESS",
        }
    }
}
impl TryFrom<&str> for IntelligentTieringAccessTier {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "ARCHIVE_ACCESS" => Ok(Self::ARCHIVE_ACCESS),
            "DEEP_ARCHIVE_ACCESS" => Ok(Self::DEEP_ARCHIVE_ACCESS),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct SelectObjectContentRequest {
    pub input_serialization: Option<InputSerialization>,
    pub expression_type: Option<ExpressionType>,
    pub expected_bucket_owner: Option<AccountId>,
    pub key: Option<ObjectKey>,
    pub expression: Option<Expression>,
    pub request_progress: Option<RequestProgress>,
    pub sse_customer_key: Option<SseCustomerKey>,
    pub bucket: Option<BucketName>,
    pub output_serialization: Option<OutputSerialization>,
    pub scan_range: Option<ScanRange>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
}

pub type CopySource = String;

pub type AllowedHeaders = Vec<AllowedHeader>;

pub type BucketName = String;

pub type ConfirmRemoveSelfBucketAccess = bool;

pub type FilterRuleValue = String;

#[derive(Debug, Default, Clone)]
pub struct JsonInput {
    pub r#type: Option<JsonType>,
}

#[derive(Debug, Default, Clone)]
pub struct PutBucketRequestPaymentRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
    pub request_payment_configuration: Option<RequestPaymentConfiguration>,
    pub content_md5: Option<ContentMd5>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
}

pub type Quiet = bool;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ObjectLockEnabled {
    Enabled,
}
impl AsRef<str> for ObjectLockEnabled {
    fn as_ref(&self) -> &str {
        match self {
            Self::Enabled => "Enabled",
        }
    }
}
impl TryFrom<&str> for ObjectLockEnabled {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "Enabled" => Ok(Self::Enabled),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

pub type Body = Vec<u8>;

pub type ObjectKey = String;

pub type SsekmsKeyId = String;

pub type AccountId = String;

#[derive(Debug, Default, Clone)]
pub struct CreateBucketConfiguration {
    pub location_constraint: Option<BucketLocationConstraint>,
}

#[derive(Debug, Default, Clone)]
pub struct PutBucketRequestPayment {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for PutBucketRequestPayment {
    const NAME: &'static str = "PutBucketRequestPayment";
    type Input = PutBucketRequestPaymentRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketCors {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for GetBucketCors {
    const NAME: &'static str = "GetBucketCors";
    type Input = GetBucketCorsRequest;
    type Output = GetBucketCorsOutput;
    type Error = ();
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ObjectVersionStorageClass {
    STANDARD,
}
impl AsRef<str> for ObjectVersionStorageClass {
    fn as_ref(&self) -> &str {
        match self {
            Self::STANDARD => "STANDARD",
        }
    }
}
impl TryFrom<&str> for ObjectVersionStorageClass {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "STANDARD" => Ok(Self::STANDARD),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

pub type ReplicaKmsKeyId = String;

pub type RequestToken = String;

#[derive(Debug, Default, Clone)]
pub struct GetBucketVersioningRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}

pub type ObjectVersionList = Vec<ObjectVersion>;

#[derive(Debug, Default, Clone)]
pub struct SourceSelectionCriteria {
    pub replica_modifications: Option<ReplicaModifications>,
    pub sse_kms_encrypted_objects: Option<SseKmsEncryptedObjects>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SseKmsEncryptedObjectsStatus {
    Enabled,
    Disabled,
}
impl AsRef<str> for SseKmsEncryptedObjectsStatus {
    fn as_ref(&self) -> &str {
        match self {
            Self::Enabled => "Enabled",
            Self::Disabled => "Disabled",
        }
    }
}
impl TryFrom<&str> for SseKmsEncryptedObjectsStatus {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "Enabled" => Ok(Self::Enabled),
            "Disabled" => Ok(Self::Disabled),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

pub type TieringList = Vec<Tiering>;

#[derive(Debug, Default, Clone)]
pub struct Transition {
    pub date: Option<Date>,
    pub storage_class: Option<TransitionStorageClass>,
    pub days: Option<Days>,
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketMetricsConfiguration {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for GetBucketMetricsConfiguration {
    const NAME: &'static str = "GetBucketMetricsConfiguration";
    type Input = GetBucketMetricsConfigurationRequest;
    type Output = GetBucketMetricsConfigurationOutput;
    type Error = ();
}

pub type AnalyticsId = String;

pub type MaxKeys = i32;

#[derive(Debug, Default, Clone)]
pub struct IntelligentTieringConfiguration {
    pub tierings: Option<TieringList>,
    pub id: Option<IntelligentTieringId>,
    pub filter: Option<IntelligentTieringFilter>,
    pub status: Option<IntelligentTieringStatus>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CompressionType {
    NONE,
    GZIP,
    BZIP2,
}
impl AsRef<str> for CompressionType {
    fn as_ref(&self) -> &str {
        match self {
            Self::NONE => "NONE",
            Self::GZIP => "GZIP",
            Self::BZIP2 => "BZIP2",
        }
    }
}
impl TryFrom<&str> for CompressionType {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "NONE" => Ok(Self::NONE),
            "GZIP" => Ok(Self::GZIP),
            "BZIP2" => Ok(Self::BZIP2),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketAnalyticsConfiguration {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for DeleteBucketAnalyticsConfiguration {
    const NAME: &'static str = "DeleteBucketAnalyticsConfiguration";
    type Input = DeleteBucketAnalyticsConfigurationRequest;
    type Output = ();
    type Error = ();
}

pub type RestoreOutputPath = String;

pub type CacheControl = String;

#[derive(Debug, Default, Clone)]
pub struct EventBridgeConfiguration {}

#[derive(Debug, Default, Clone)]
pub struct IntelligentTieringAndOperator {
    pub prefix: Option<Prefix>,
    pub tags: Option<TagSet>,
}

#[derive(Debug, Default, Clone)]
pub struct JsonOutput {
    pub record_delimiter: Option<RecordDelimiter>,
}

#[derive(Debug, Default, Clone)]
pub struct PolicyStatus {
    pub is_public: Option<IsPublic>,
}

#[derive(Debug, Default, Clone)]
pub struct PutBucketLoggingRequest {
    pub content_md5: Option<ContentMd5>,
    pub bucket: Option<BucketName>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub bucket_logging_status: Option<BucketLoggingStatus>,
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug, Default, Clone)]
pub struct Redirect {
    pub host_name: Option<HostName>,
    pub protocol: Option<Protocol>,
    pub http_redirect_code: Option<HttpRedirectCode>,
    pub replace_key_prefix_with: Option<ReplaceKeyPrefixWith>,
    pub replace_key_with: Option<ReplaceKeyWith>,
}

pub type KmsContext = String;

pub type SseCustomerKey = String;

#[derive(Debug, Default, Clone)]
pub struct StorageClassAnalysisDataExport {
    pub destination: Option<AnalyticsExportDestination>,
    pub output_schema_version: Option<StorageClassAnalysisSchemaVersion>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Protocol {
    http,
    https,
}
impl AsRef<str> for Protocol {
    fn as_ref(&self) -> &str {
        match self {
            Self::http => "http",
            Self::https => "https",
        }
    }
}
impl TryFrom<&str> for Protocol {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "http" => Ok(Self::http),
            "https" => Ok(Self::https),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct PutObjectAcl {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for PutObjectAcl {
    const NAME: &'static str = "PutObjectAcl";
    type Input = PutObjectAclRequest;
    type Output = PutObjectAclOutput;
    type Error = ();
}

pub type GrantWriteAcp = String;

#[derive(Debug, Default, Clone)]
pub struct GetBucketRequestPaymentRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum MfaDelete {
    Enabled,
    Disabled,
}
impl AsRef<str> for MfaDelete {
    fn as_ref(&self) -> &str {
        match self {
            Self::Enabled => "Enabled",
            Self::Disabled => "Disabled",
        }
    }
}
impl TryFrom<&str> for MfaDelete {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "Enabled" => Ok(Self::Enabled),
            "Disabled" => Ok(Self::Disabled),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

pub type DeleteMarkers = Vec<DeleteMarkerEntry>;

#[derive(Debug, Default, Clone)]
pub struct PutBucketIntelligentTieringConfigurationRequest {
    pub bucket: Option<BucketName>,
    pub id: Option<IntelligentTieringId>,
    pub intelligent_tiering_configuration: Option<IntelligentTieringConfiguration>,
}

#[derive(Debug, Default, Clone)]
pub struct ListBucketMetricsConfigurationsRequest {
    pub bucket: Option<BucketName>,
    pub continuation_token: Option<Token>,
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug, Default, Clone)]
pub struct Delete {
    pub quiet: Option<Quiet>,
    pub objects: Option<ObjectIdentifierList>,
}

pub type ReplicationRules = Vec<ReplicationRule>;

pub type ContentDisposition = String;

#[derive(Debug, Default, Clone)]
pub struct SelectParameters {
    pub output_serialization: Option<OutputSerialization>,
    pub expression: Option<Expression>,
    pub expression_type: Option<ExpressionType>,
    pub input_serialization: Option<InputSerialization>,
}

#[derive(Debug, Default, Clone)]
pub struct Object {
    pub storage_class: Option<ObjectStorageClass>,
    pub checksum_algorithm: Option<ChecksumAlgorithmList>,
    pub owner: Option<Owner>,
    pub e_tag: Option<ETag>,
    pub key: Option<ObjectKey>,
    pub last_modified: Option<LastModified>,
    pub size: Option<Size>,
}

#[derive(Debug, Default, Clone)]
pub struct ServerSideEncryptionRule {
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    pub apply_server_side_encryption_by_default: Option<ServerSideEncryptionByDefault>,
}

#[derive(Debug, Default, Clone)]
pub struct ListMultipartUploadsOutput {
    pub encoding_type: Option<EncodingType>,
    pub is_truncated: Option<IsTruncated>,
    pub key_marker: Option<KeyMarker>,
    pub uploads: Option<MultipartUploadList>,
    pub common_prefixes: Option<CommonPrefixList>,
    pub prefix: Option<Prefix>,
    pub upload_id_marker: Option<UploadIdMarker>,
    pub next_upload_id_marker: Option<NextUploadIdMarker>,
    pub bucket: Option<BucketName>,
    pub next_key_marker: Option<NextKeyMarker>,
    pub delimiter: Option<Delimiter>,
    pub max_uploads: Option<MaxUploads>,
}

pub type Value = String;

pub type AllowedMethod = String;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BucketLocationConstraint {
    af_south_1,
    ap_east_1,
    ap_northeast_1,
    ap_northeast_2,
    ap_northeast_3,
    ap_south_1,
    ap_southeast_1,
    ap_southeast_2,
    ca_central_1,
    cn_north_1,
    cn_northwest_1,
    EU,
    eu_central_1,
    eu_north_1,
    eu_south_1,
    eu_west_1,
    eu_west_2,
    eu_west_3,
    me_south_1,
    sa_east_1,
    us_east_2,
    us_gov_east_1,
    us_gov_west_1,
    us_west_1,
    us_west_2,
}
impl AsRef<str> for BucketLocationConstraint {
    fn as_ref(&self) -> &str {
        match self {
            Self::af_south_1 => "af-south-1",
            Self::ap_east_1 => "ap-east-1",
            Self::ap_northeast_1 => "ap-northeast-1",
            Self::ap_northeast_2 => "ap-northeast-2",
            Self::ap_northeast_3 => "ap-northeast-3",
            Self::ap_south_1 => "ap-south-1",
            Self::ap_southeast_1 => "ap-southeast-1",
            Self::ap_southeast_2 => "ap-southeast-2",
            Self::ca_central_1 => "ca-central-1",
            Self::cn_north_1 => "cn-north-1",
            Self::cn_northwest_1 => "cn-northwest-1",
            Self::EU => "EU",
            Self::eu_central_1 => "eu-central-1",
            Self::eu_north_1 => "eu-north-1",
            Self::eu_south_1 => "eu-south-1",
            Self::eu_west_1 => "eu-west-1",
            Self::eu_west_2 => "eu-west-2",
            Self::eu_west_3 => "eu-west-3",
            Self::me_south_1 => "me-south-1",
            Self::sa_east_1 => "sa-east-1",
            Self::us_east_2 => "us-east-2",
            Self::us_gov_east_1 => "us-gov-east-1",
            Self::us_gov_west_1 => "us-gov-west-1",
            Self::us_west_1 => "us-west-1",
            Self::us_west_2 => "us-west-2",
        }
    }
}
impl TryFrom<&str> for BucketLocationConstraint {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "af-south-1" => Ok(Self::af_south_1),
            "ap-east-1" => Ok(Self::ap_east_1),
            "ap-northeast-1" => Ok(Self::ap_northeast_1),
            "ap-northeast-2" => Ok(Self::ap_northeast_2),
            "ap-northeast-3" => Ok(Self::ap_northeast_3),
            "ap-south-1" => Ok(Self::ap_south_1),
            "ap-southeast-1" => Ok(Self::ap_southeast_1),
            "ap-southeast-2" => Ok(Self::ap_southeast_2),
            "ca-central-1" => Ok(Self::ca_central_1),
            "cn-north-1" => Ok(Self::cn_north_1),
            "cn-northwest-1" => Ok(Self::cn_northwest_1),
            "EU" => Ok(Self::EU),
            "eu-central-1" => Ok(Self::eu_central_1),
            "eu-north-1" => Ok(Self::eu_north_1),
            "eu-south-1" => Ok(Self::eu_south_1),
            "eu-west-1" => Ok(Self::eu_west_1),
            "eu-west-2" => Ok(Self::eu_west_2),
            "eu-west-3" => Ok(Self::eu_west_3),
            "me-south-1" => Ok(Self::me_south_1),
            "sa-east-1" => Ok(Self::sa_east_1),
            "us-east-2" => Ok(Self::us_east_2),
            "us-gov-east-1" => Ok(Self::us_gov_east_1),
            "us-gov-west-1" => Ok(Self::us_gov_west_1),
            "us-west-1" => Ok(Self::us_west_1),
            "us-west-2" => Ok(Self::us_west_2),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct CopyObject {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for CopyObject {
    const NAME: &'static str = "CopyObject";
    type Input = CopyObjectRequest;
    type Output = CopyObjectOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketTaggingRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketWebsite {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for GetBucketWebsite {
    const NAME: &'static str = "GetBucketWebsite";
    type Input = GetBucketWebsiteRequest;
    type Output = GetBucketWebsiteOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct PutBucketReplication {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for PutBucketReplication {
    const NAME: &'static str = "PutBucketReplication";
    type Input = PutBucketReplicationRequest;
    type Output = ();
    type Error = ();
}

pub type UserMetadata = Vec<MetadataEntry>;

pub type Initiated = String;

#[derive(Debug, Default, Clone)]
pub struct PutPublicAccessBlock {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for PutPublicAccessBlock {
    const NAME: &'static str = "PutPublicAccessBlock";
    type Input = PutPublicAccessBlockRequest;
    type Output = ();
    type Error = ();
}

pub type CopySourceIfUnmodifiedSince = String;

#[derive(Debug, Default, Clone)]
pub struct NoSuchKey {}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TransitionStorageClass {
    GLACIER,
    STANDARD_IA,
    ONEZONE_IA,
    INTELLIGENT_TIERING,
    DEEP_ARCHIVE,
    GLACIER_IR,
}
impl AsRef<str> for TransitionStorageClass {
    fn as_ref(&self) -> &str {
        match self {
            Self::GLACIER => "GLACIER",
            Self::STANDARD_IA => "STANDARD_IA",
            Self::ONEZONE_IA => "ONEZONE_IA",
            Self::INTELLIGENT_TIERING => "INTELLIGENT_TIERING",
            Self::DEEP_ARCHIVE => "DEEP_ARCHIVE",
            Self::GLACIER_IR => "GLACIER_IR",
        }
    }
}
impl TryFrom<&str> for TransitionStorageClass {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "GLACIER" => Ok(Self::GLACIER),
            "STANDARD_IA" => Ok(Self::STANDARD_IA),
            "ONEZONE_IA" => Ok(Self::ONEZONE_IA),
            "INTELLIGENT_TIERING" => Ok(Self::INTELLIGENT_TIERING),
            "DEEP_ARCHIVE" => Ok(Self::DEEP_ARCHIVE),
            "GLACIER_IR" => Ok(Self::GLACIER_IR),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

pub type Years = i32;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum StorageClassAnalysisSchemaVersion {
    V_1,
}
impl AsRef<str> for StorageClassAnalysisSchemaVersion {
    fn as_ref(&self) -> &str {
        match self {
            Self::V_1 => "V_1",
        }
    }
}
impl TryFrom<&str> for StorageClassAnalysisSchemaVersion {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "V_1" => Ok(Self::V_1),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketLifecycle {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for DeleteBucketLifecycle {
    const NAME: &'static str = "DeleteBucketLifecycle";
    type Input = DeleteBucketLifecycleRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketLocationRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}

pub type Description = String;

pub type ResponseExpires = String;

pub type QuoteCharacter = String;

#[derive(Debug, Default, Clone)]
pub struct GetObjectRetention {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for GetObjectRetention {
    const NAME: &'static str = "GetObjectRetention";
    type Input = GetObjectRetentionRequest;
    type Output = GetObjectRetentionOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketWebsite {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for DeleteBucketWebsite {
    const NAME: &'static str = "DeleteBucketWebsite";
    type Input = DeleteBucketWebsiteRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct ListBuckets {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for ListBuckets {
    const NAME: &'static str = "ListBuckets";
    type Input = ();
    type Output = ListBucketsOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct PutBucketInventoryConfiguration {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for PutBucketInventoryConfiguration {
    const NAME: &'static str = "PutBucketInventoryConfiguration";
    type Input = PutBucketInventoryConfigurationRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct ReplicationRuleAndOperator {
    pub prefix: Option<Prefix>,
    pub tags: Option<TagSet>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum EncodingType {
    url,
}
impl AsRef<str> for EncodingType {
    fn as_ref(&self) -> &str {
        match self {
            Self::url => "url",
        }
    }
}
impl TryFrom<&str> for EncodingType {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "url" => Ok(Self::url),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

pub type ETag = String;

pub type ExpiredObjectDeleteMarker = bool;

#[derive(Debug, Default, Clone)]
pub struct GetBucketAclOutput {
    pub grants: Option<Grants>,
    pub owner: Option<Owner>,
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketVersioningOutput {
    pub mfa_delete: Option<MfaDeleteStatus>,
    pub status: Option<BucketVersioningStatus>,
}

pub type GrantRead = String;

#[derive(Debug, Default, Clone)]
pub struct GetObjectAttributesParts {
    pub next_part_number_marker: Option<NextPartNumberMarker>,
    pub part_number_marker: Option<PartNumberMarker>,
    pub parts: Option<PartsList>,
    pub total_parts_count: Option<PartsCount>,
    pub is_truncated: Option<IsTruncated>,
    pub max_parts: Option<MaxParts>,
}

#[derive(Debug, Default, Clone)]
pub struct ListObjectsRequest {
    pub bucket: Option<BucketName>,
    pub marker: Option<Marker>,
    pub expected_bucket_owner: Option<AccountId>,
    pub max_keys: Option<MaxKeys>,
    pub delimiter: Option<Delimiter>,
    pub encoding_type: Option<EncodingType>,
    pub prefix: Option<Prefix>,
    pub request_payer: Option<RequestPayer>,
}

pub type Priority = i32;

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketReplication {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for DeleteBucketReplication {
    const NAME: &'static str = "DeleteBucketReplication";
    type Input = DeleteBucketReplicationRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetObjectTorrentOutput {
    pub body: Option<StreamingBlob>,
    pub request_charged: Option<RequestCharged>,
}

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketEncryptionRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}

pub type IfNoneMatch = String;

#[derive(Debug, Default, Clone)]
pub struct PutBucketCors {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for PutBucketCors {
    const NAME: &'static str = "PutBucketCors";
    type Input = PutBucketCorsRequest;
    type Output = ();
    type Error = ();
}

pub type BypassGovernanceRetention = bool;

#[derive(Debug, Default, Clone)]
pub struct ListBucketAnalyticsConfigurationsRequest {
    pub bucket: Option<BucketName>,
    pub continuation_token: Option<Token>,
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum QuoteFields {
    ALWAYS,
    ASNEEDED,
}
impl AsRef<str> for QuoteFields {
    fn as_ref(&self) -> &str {
        match self {
            Self::ALWAYS => "ALWAYS",
            Self::ASNEEDED => "ASNEEDED",
        }
    }
}
impl TryFrom<&str> for QuoteFields {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "ALWAYS" => Ok(Self::ALWAYS),
            "ASNEEDED" => Ok(Self::ASNEEDED),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetObjectRetentionRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub version_id: Option<ObjectVersionId>,
    pub key: Option<ObjectKey>,
    pub request_payer: Option<RequestPayer>,
    pub bucket: Option<BucketName>,
}

#[derive(Debug, Default, Clone)]
pub struct PutBucketAnalyticsConfiguration {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for PutBucketAnalyticsConfiguration {
    const NAME: &'static str = "PutBucketAnalyticsConfiguration";
    type Input = PutBucketAnalyticsConfigurationRequest;
    type Output = ();
    type Error = ();
}

pub type TransitionList = Vec<Transition>;

#[derive(Debug, Default, Clone)]
pub struct GetBucketVersioning {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for GetBucketVersioning {
    const NAME: &'static str = "GetBucketVersioning";
    type Input = GetBucketVersioningRequest;
    type Output = GetBucketVersioningOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct LifecycleRule {
    pub id: Option<Id>,
    pub abort_incomplete_multipart_upload: Option<AbortIncompleteMultipartUpload>,
    pub transitions: Option<TransitionList>,
    pub expiration: Option<LifecycleExpiration>,
    pub filter: Option<LifecycleRuleFilter>,
    pub noncurrent_version_expiration: Option<NoncurrentVersionExpiration>,
    pub noncurrent_version_transitions: Option<NoncurrentVersionTransitionList>,
    pub prefix: Option<Prefix>,
    pub status: Option<ExpirationStatus>,
}

#[derive(Debug, Default, Clone)]
pub struct ListObjectVersions {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for ListObjectVersions {
    const NAME: &'static str = "ListObjectVersions";
    type Input = ListObjectVersionsRequest;
    type Output = ListObjectVersionsOutput;
    type Error = ();
}

pub type PartNumberMarker = String;

#[derive(Debug, Default, Clone)]
pub struct UploadPartCopyRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub part_number: Option<PartNumber>,
    pub copy_source_sse_customer_algorithm: Option<CopySourceSseCustomerAlgorithm>,
    pub copy_source_range: Option<CopySourceRange>,
    pub request_payer: Option<RequestPayer>,
    pub bucket: Option<BucketName>,
    pub copy_source_sse_customer_key_md5: Option<CopySourceSseCustomerKeyMd5>,
    pub copy_source_if_match: Option<CopySourceIfMatch>,
    pub key: Option<ObjectKey>,
    pub sse_customer_key: Option<SseCustomerKey>,
    pub upload_id: Option<MultipartUploadId>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub copy_source_if_unmodified_since: Option<CopySourceIfUnmodifiedSince>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub copy_source_if_none_match: Option<CopySourceIfNoneMatch>,
    pub copy_source_sse_customer_key: Option<CopySourceSseCustomerKey>,
    pub copy_source: Option<CopySource>,
    pub expected_source_bucket_owner: Option<AccountId>,
    pub copy_source_if_modified_since: Option<CopySourceIfModifiedSince>,
}

#[derive(Debug, Default, Clone)]
pub struct GetObjectRequest {
    pub if_unmodified_since: Option<IfUnmodifiedSince>,
    pub expected_bucket_owner: Option<AccountId>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub range: Option<Range>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub version_id: Option<ObjectVersionId>,
    pub response_content_type: Option<ResponseContentType>,
    pub if_modified_since: Option<IfModifiedSince>,
    pub response_content_encoding: Option<ResponseContentEncoding>,
    pub part_number: Option<PartNumber>,
    pub response_expires: Option<ResponseExpires>,
    pub bucket: Option<BucketName>,
    pub if_match: Option<IfMatch>,
    pub checksum_mode: Option<ChecksumMode>,
    pub key: Option<ObjectKey>,
    pub response_cache_control: Option<ResponseCacheControl>,
    pub request_payer: Option<RequestPayer>,
    pub response_content_disposition: Option<ResponseContentDisposition>,
    pub if_none_match: Option<IfNoneMatch>,
    pub response_content_language: Option<ResponseContentLanguage>,
    pub sse_customer_key: Option<SseCustomerKey>,
}

#[derive(Debug, Default, Clone)]
pub struct ServerSideEncryptionConfiguration {
    pub rules: Option<ServerSideEncryptionRules>,
}

#[derive(Debug, Default, Clone)]
pub struct PutObjectTagging {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for PutObjectTagging {
    const NAME: &'static str = "PutObjectTagging";
    type Input = PutObjectTaggingRequest;
    type Output = PutObjectTaggingOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetObjectLegalHoldOutput {
    pub legal_hold: Option<ObjectLockLegalHold>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum IntelligentTieringStatus {
    Enabled,
    Disabled,
}
impl AsRef<str> for IntelligentTieringStatus {
    fn as_ref(&self) -> &str {
        match self {
            Self::Enabled => "Enabled",
            Self::Disabled => "Disabled",
        }
    }
}
impl TryFrom<&str> for IntelligentTieringStatus {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "Enabled" => Ok(Self::Enabled),
            "Disabled" => Ok(Self::Disabled),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketTaggingRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}

pub type KeyCount = i32;

pub type LifecycleRules = Vec<LifecycleRule>;

#[derive(Debug, Clone)]
pub enum MetricsFilter {
    Tag(Tag),
    AccessPointArn(AccessPointArn),
    And(MetricsAndOperator),
    Prefix(Prefix),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Type {
    CanonicalUser,
    AmazonCustomerByEmail,
    Group,
}
impl AsRef<str> for Type {
    fn as_ref(&self) -> &str {
        match self {
            Self::CanonicalUser => "CanonicalUser",
            Self::AmazonCustomerByEmail => "AmazonCustomerByEmail",
            Self::Group => "Group",
        }
    }
}
impl TryFrom<&str> for Type {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "CanonicalUser" => Ok(Self::CanonicalUser),
            "AmazonCustomerByEmail" => Ok(Self::AmazonCustomerByEmail),
            "Group" => Ok(Self::Group),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketOwnershipControlsOutput {
    pub ownership_controls: Option<OwnershipControls>,
}

#[derive(Debug, Default, Clone)]
pub struct LoggingEnabled {
    pub target_bucket: Option<TargetBucket>,
    pub target_grants: Option<TargetGrants>,
    pub target_prefix: Option<TargetPrefix>,
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketIntelligentTieringConfigurationRequest {
    pub id: Option<IntelligentTieringId>,
    pub bucket: Option<BucketName>,
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketEncryption {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for GetBucketEncryption {
    const NAME: &'static str = "GetBucketEncryption";
    type Input = GetBucketEncryptionRequest;
    type Output = GetBucketEncryptionOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketInventoryConfigurationRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub id: Option<InventoryId>,
    pub bucket: Option<BucketName>,
}

#[derive(Debug, Default, Clone)]
pub struct GetObjectTaggingOutput {
    pub tag_set: Option<TagSet>,
    pub version_id: Option<ObjectVersionId>,
}

#[derive(Debug, Default, Clone)]
pub struct InventoryEncryption {
    pub sses3: Option<Sses3>,
    pub ssekms: Option<Ssekms>,
}

#[derive(Debug, Default, Clone)]
pub struct ListBucketIntelligentTieringConfigurationsRequest {
    pub bucket: Option<BucketName>,
    pub continuation_token: Option<Token>,
}

#[derive(Debug, Default, Clone)]
pub struct NotFound {}

#[derive(Debug, Default, Clone)]
pub struct ObjectLockConfiguration {
    pub object_lock_enabled: Option<ObjectLockEnabled>,
    pub rule: Option<ObjectLockRule>,
}

#[derive(Debug, Default, Clone)]
pub struct PutBucketAnalyticsConfigurationRequest {
    pub bucket: Option<BucketName>,
    pub analytics_configuration: Option<AnalyticsConfiguration>,
    pub id: Option<AnalyticsId>,
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug, Default, Clone)]
pub struct PutBucketEncryptionRequest {
    pub bucket: Option<BucketName>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub content_md5: Option<ContentMd5>,
    pub expected_bucket_owner: Option<AccountId>,
    pub server_side_encryption_configuration: Option<ServerSideEncryptionConfiguration>,
}

#[derive(Debug, Default, Clone)]
pub struct RestoreObjectRequest {
    pub restore_request: Option<RestoreRequest>,
    pub bucket: Option<BucketName>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub key: Option<ObjectKey>,
    pub expected_bucket_owner: Option<AccountId>,
    pub version_id: Option<ObjectVersionId>,
    pub request_payer: Option<RequestPayer>,
}

pub type HttpRedirectCode = String;

pub type DeleteMarker = bool;

#[derive(Debug, Default, Clone)]
pub struct GetBucketAclRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug, Default, Clone)]
pub struct OwnershipControls {
    pub rules: Option<OwnershipControlsRules>,
}

#[derive(Debug, Default, Clone)]
pub struct PutBucketAcl {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for PutBucketAcl {
    const NAME: &'static str = "PutBucketAcl";
    type Input = PutBucketAclRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct SelectObjectContent {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for SelectObjectContent {
    const NAME: &'static str = "SelectObjectContent";
    type Input = SelectObjectContentRequest;
    type Output = SelectObjectContentOutput;
    type Error = ();
}

pub type TagSet = Vec<Tag>;

#[derive(Debug, Default, Clone)]
pub struct DeletePublicAccessBlockRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}

pub type NotificationId = String;

#[derive(Debug, Default, Clone)]
pub struct S3KeyFilter {
    pub filter_rules: Option<FilterRuleList>,
}

pub type CopySourceVersionId = String;

pub type TargetBucket = String;

#[derive(Debug, Default, Clone)]
pub struct AbortMultipartUploadRequest {
    pub upload_id: Option<MultipartUploadId>,
    pub key: Option<ObjectKey>,
    pub request_payer: Option<RequestPayer>,
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ChecksumMode {
    ENABLED,
}
impl AsRef<str> for ChecksumMode {
    fn as_ref(&self) -> &str {
        match self {
            Self::ENABLED => "ENABLED",
        }
    }
}
impl TryFrom<&str> for ChecksumMode {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "ENABLED" => Ok(Self::ENABLED),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct PutBucketVersioningRequest {
    pub mfa: Option<Mfa>,
    pub versioning_configuration: Option<VersioningConfiguration>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
    pub content_md5: Option<ContentMd5>,
}

pub type FieldDelimiter = String;

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketMetricsConfiguration {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for DeleteBucketMetricsConfiguration {
    const NAME: &'static str = "DeleteBucketMetricsConfiguration";
    type Input = DeleteBucketMetricsConfigurationRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct PutObjectLegalHoldOutput {
    pub request_charged: Option<RequestCharged>,
}

pub type TargetGrants = Vec<TargetGrant>;

pub type LastModified = String;

#[derive(Debug, Default, Clone)]
pub struct ReplicationConfiguration {
    pub rules: Option<ReplicationRules>,
    pub role: Option<Role>,
}

pub type WebsiteRedirectLocation = String;

#[derive(Debug, Default, Clone)]
pub struct DeleteObjectsOutput {
    pub deleted: Option<DeletedObjects>,
    pub errors: Option<Errors>,
    pub request_charged: Option<RequestCharged>,
}

pub type NextPartNumberMarker = String;

#[derive(Debug, Default, Clone)]
pub struct ListBucketAnalyticsConfigurationsOutput {
    pub analytics_configuration_list: Option<AnalyticsConfigurationList>,
    pub continuation_token: Option<Token>,
    pub is_truncated: Option<IsTruncated>,
    pub next_continuation_token: Option<NextToken>,
}

pub type HostName = String;

#[derive(Debug, Default, Clone)]
pub struct GetBucketNotificationConfigurationRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug, Default, Clone)]
pub struct GetPublicAccessBlockRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}

pub type Minutes = i32;

pub type ObjectSize = i64;

#[derive(Debug, Default, Clone)]
pub struct PutObjectRetention {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for PutObjectRetention {
    const NAME: &'static str = "PutObjectRetention";
    type Input = PutObjectRetentionRequest;
    type Output = PutObjectRetentionOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketOwnershipControlsRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}

#[derive(Debug, Default, Clone)]
pub struct ListBucketInventoryConfigurationsOutput {
    pub continuation_token: Option<Token>,
    pub inventory_configuration_list: Option<InventoryConfigurationList>,
    pub is_truncated: Option<IsTruncated>,
    pub next_continuation_token: Option<NextToken>,
}

#[derive(Debug, Default, Clone)]
pub struct Error {
    pub version_id: Option<ObjectVersionId>,
    pub key: Option<ObjectKey>,
    pub message: Option<Message>,
    pub code: Option<Code>,
}

pub type PartsList = Vec<ObjectPart>;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum InventoryFormat {
    CSV,
    ORC,
    Parquet,
}
impl AsRef<str> for InventoryFormat {
    fn as_ref(&self) -> &str {
        match self {
            Self::CSV => "CSV",
            Self::ORC => "ORC",
            Self::Parquet => "Parquet",
        }
    }
}
impl TryFrom<&str> for InventoryFormat {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "CSV" => Ok(Self::CSV),
            "ORC" => Ok(Self::ORC),
            "Parquet" => Ok(Self::Parquet),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

pub type Buckets = Vec<Bucket>;

#[derive(Debug, Default, Clone)]
pub struct PutBucketTaggingRequest {
    pub bucket: Option<BucketName>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub tagging: Option<Tagging>,
    pub expected_bucket_owner: Option<AccountId>,
    pub content_md5: Option<ContentMd5>,
}

#[derive(Debug, Default, Clone)]
pub struct PutBucketPolicyRequest {
    pub bucket: Option<BucketName>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub expected_bucket_owner: Option<AccountId>,
    pub confirm_remove_self_bucket_access: Option<ConfirmRemoveSelfBucketAccess>,
    pub content_md5: Option<ContentMd5>,
    pub policy: Option<Policy>,
}

#[derive(Debug, Default, Clone)]
pub struct LambdaFunctionConfiguration {
    pub events: Option<EventList>,
    pub id: Option<NotificationId>,
    pub lambda_function_arn: Option<LambdaFunctionArn>,
    pub filter: Option<NotificationConfigurationFilter>,
}

#[derive(Debug, Default, Clone)]
pub struct Bucket {
    pub name: Option<BucketName>,
    pub creation_date: Option<CreationDate>,
}

#[derive(Debug, Default, Clone)]
pub struct CreateMultipartUploadOutput {
    pub request_charged: Option<RequestCharged>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub abort_rule_id: Option<AbortRuleId>,
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    pub abort_date: Option<AbortDate>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub ssekms_encryption_context: Option<SsekmsEncryptionContext>,
    pub server_side_encryption: Option<ServerSideEncryption>,
    pub upload_id: Option<MultipartUploadId>,
    pub key: Option<ObjectKey>,
    pub bucket: Option<BucketName>,
    pub ssekms_key_id: Option<SsekmsKeyId>,
}

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketIntelligentTieringConfiguration {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for DeleteBucketIntelligentTieringConfiguration {
    const NAME: &'static str = "DeleteBucketIntelligentTieringConfiguration";
    type Input = DeleteBucketIntelligentTieringConfigurationRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct InventoryConfiguration {
    pub schedule: Option<InventorySchedule>,
    pub included_object_versions: Option<InventoryIncludedObjectVersions>,
    pub destination: Option<InventoryDestination>,
    pub id: Option<InventoryId>,
    pub is_enabled: Option<IsEnabled>,
    pub optional_fields: Option<InventoryOptionalFields>,
    pub filter: Option<InventoryFilter>,
}

pub type Range = String;

#[derive(Debug, Default, Clone)]
pub struct RoutingRule {
    pub condition: Option<Condition>,
    pub redirect: Option<Redirect>,
}

#[derive(Debug, Default, Clone)]
pub struct LifecycleRuleAndOperator {
    pub tags: Option<TagSet>,
    pub object_size_less_than: Option<ObjectSizeLessThanBytes>,
    pub prefix: Option<Prefix>,
    pub object_size_greater_than: Option<ObjectSizeGreaterThanBytes>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ExpressionType {
    SQL,
}
impl AsRef<str> for ExpressionType {
    fn as_ref(&self) -> &str {
        match self {
            Self::SQL => "SQL",
        }
    }
}
impl TryFrom<&str> for ExpressionType {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "SQL" => Ok(Self::SQL),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

pub type End = i64;

#[derive(Debug, Default, Clone)]
pub struct RequestPaymentConfiguration {
    pub payer: Option<Payer>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ObjectLockRetentionMode {
    GOVERNANCE,
    COMPLIANCE,
}
impl AsRef<str> for ObjectLockRetentionMode {
    fn as_ref(&self) -> &str {
        match self {
            Self::GOVERNANCE => "GOVERNANCE",
            Self::COMPLIANCE => "COMPLIANCE",
        }
    }
}
impl TryFrom<&str> for ObjectLockRetentionMode {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "GOVERNANCE" => Ok(Self::GOVERNANCE),
            "COMPLIANCE" => Ok(Self::COMPLIANCE),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

pub type IntelligentTieringId = String;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BucketLogsPermission {
    FULL_CONTROL,
    READ,
    WRITE,
}
impl AsRef<str> for BucketLogsPermission {
    fn as_ref(&self) -> &str {
        match self {
            Self::FULL_CONTROL => "FULL_CONTROL",
            Self::READ => "READ",
            Self::WRITE => "WRITE",
        }
    }
}
impl TryFrom<&str> for BucketLogsPermission {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "FULL_CONTROL" => Ok(Self::FULL_CONTROL),
            "READ" => Ok(Self::READ),
            "WRITE" => Ok(Self::WRITE),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

pub type RoutingRules = Vec<RoutingRule>;

#[derive(Debug, Default, Clone)]
pub struct OwnershipControlsRule {
    pub object_ownership: Option<ObjectOwnership>,
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketLocation {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for GetBucketLocation {
    const NAME: &'static str = "GetBucketLocation";
    type Input = GetBucketLocationRequest;
    type Output = GetBucketLocationOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct ObjectVersion {
    pub owner: Option<Owner>,
    pub e_tag: Option<ETag>,
    pub is_latest: Option<IsLatest>,
    pub key: Option<ObjectKey>,
    pub size: Option<Size>,
    pub version_id: Option<ObjectVersionId>,
    pub checksum_algorithm: Option<ChecksumAlgorithmList>,
    pub last_modified: Option<LastModified>,
    pub storage_class: Option<ObjectVersionStorageClass>,
}

#[derive(Debug, Default, Clone)]
pub struct Part {
    pub checksum_sha256: Option<ChecksumSha256>,
    pub part_number: Option<PartNumber>,
    pub checksum_sha1: Option<ChecksumSha1>,
    pub size: Option<Size>,
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
    pub checksum_crc32: Option<ChecksumCrc32>,
    pub e_tag: Option<ETag>,
    pub last_modified: Option<LastModified>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ReplicaModificationsStatus {
    Enabled,
    Disabled,
}
impl AsRef<str> for ReplicaModificationsStatus {
    fn as_ref(&self) -> &str {
        match self {
            Self::Enabled => "Enabled",
            Self::Disabled => "Disabled",
        }
    }
}
impl TryFrom<&str> for ReplicaModificationsStatus {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "Enabled" => Ok(Self::Enabled),
            "Disabled" => Ok(Self::Disabled),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketInventoryConfigurationOutput {
    pub inventory_configuration: Option<InventoryConfiguration>,
}

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketPolicy {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for DeleteBucketPolicy {
    const NAME: &'static str = "DeleteBucketPolicy";
    type Input = DeleteBucketPolicyRequest;
    type Output = ();
    type Error = ();
}

pub type ObjectAttributesList = Vec<ObjectAttributes>;

#[derive(Debug, Default, Clone)]
pub struct Condition {
    pub http_error_code_returned_equals: Option<HttpErrorCodeReturnedEquals>,
    pub key_prefix_equals: Option<KeyPrefixEquals>,
}

#[derive(Debug, Default, Clone)]
pub struct NoncurrentVersionTransition {
    pub noncurrent_days: Option<Days>,
    pub storage_class: Option<TransitionStorageClass>,
    pub newer_noncurrent_versions: Option<VersionCount>,
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketAnalyticsConfiguration {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for GetBucketAnalyticsConfiguration {
    const NAME: &'static str = "GetBucketAnalyticsConfiguration";
    type Input = GetBucketAnalyticsConfigurationRequest;
    type Output = GetBucketAnalyticsConfigurationOutput;
    type Error = ();
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ObjectCannedAcl {
    private,
    public_read,
    public_read_write,
    authenticated_read,
    aws_exec_read,
    bucket_owner_read,
    bucket_owner_full_control,
}
impl AsRef<str> for ObjectCannedAcl {
    fn as_ref(&self) -> &str {
        match self {
            Self::private => "private",
            Self::public_read => "public-read",
            Self::public_read_write => "public-read-write",
            Self::authenticated_read => "authenticated-read",
            Self::aws_exec_read => "aws-exec-read",
            Self::bucket_owner_read => "bucket-owner-read",
            Self::bucket_owner_full_control => "bucket-owner-full-control",
        }
    }
}
impl TryFrom<&str> for ObjectCannedAcl {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "private" => Ok(Self::private),
            "public-read" => Ok(Self::public_read),
            "public-read-write" => Ok(Self::public_read_write),
            "authenticated-read" => Ok(Self::authenticated_read),
            "aws-exec-read" => Ok(Self::aws_exec_read),
            "bucket-owner-read" => Ok(Self::bucket_owner_read),
            "bucket-owner-full-control" => Ok(Self::bucket_owner_full_control),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct PutBucketNotificationConfiguration {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for PutBucketNotificationConfiguration {
    const NAME: &'static str = "PutBucketNotificationConfiguration";
    type Input = PutBucketNotificationConfigurationRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketLocationOutput {
    pub location_constraint: Option<BucketLocationConstraint>,
}

#[derive(Debug, Default, Clone)]
pub struct CompleteMultipartUploadRequest {
    pub checksum_crc32: Option<ChecksumCrc32>,
    pub expected_bucket_owner: Option<AccountId>,
    pub key: Option<ObjectKey>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub sse_customer_key: Option<SseCustomerKey>,
    pub upload_id: Option<MultipartUploadId>,
    pub bucket: Option<BucketName>,
    pub multipart_upload: Option<CompletedMultipartUpload>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub checksum_sha1: Option<ChecksumSha1>,
    pub request_payer: Option<RequestPayer>,
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
    pub checksum_sha256: Option<ChecksumSha256>,
}

#[derive(Debug, Default, Clone)]
pub struct GetObjectLegalHold {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for GetObjectLegalHold {
    const NAME: &'static str = "GetObjectLegalHold";
    type Input = GetObjectLegalHoldRequest;
    type Output = GetObjectLegalHoldOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct AnalyticsS3BucketDestination {
    pub format: Option<AnalyticsS3ExportFileFormat>,
    pub bucket_account_id: Option<AccountId>,
    pub prefix: Option<Prefix>,
    pub bucket: Option<BucketName>,
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketTaggingOutput {
    pub tag_set: Option<TagSet>,
}

pub type InventoryId = String;

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketInventoryConfiguration {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for DeleteBucketInventoryConfiguration {
    const NAME: &'static str = "DeleteBucketInventoryConfiguration";
    type Input = DeleteBucketInventoryConfigurationRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct ListBucketAnalyticsConfigurations {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for ListBucketAnalyticsConfigurations {
    const NAME: &'static str = "ListBucketAnalyticsConfigurations";
    type Input = ListBucketAnalyticsConfigurationsRequest;
    type Output = ListBucketAnalyticsConfigurationsOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct PutObject {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for PutObject {
    const NAME: &'static str = "PutObject";
    type Input = PutObjectRequest;
    type Output = PutObjectOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketAccelerateConfiguration {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for GetBucketAccelerateConfiguration {
    const NAME: &'static str = "GetBucketAccelerateConfiguration";
    type Input = GetBucketAccelerateConfigurationRequest;
    type Output = GetBucketAccelerateConfigurationOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketMetricsConfigurationRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
    pub id: Option<MetricsId>,
}

#[derive(Debug, Default, Clone)]
pub struct DeletedObject {
    pub delete_marker: Option<DeleteMarker>,
    pub delete_marker_version_id: Option<DeleteMarkerVersionId>,
    pub key: Option<ObjectKey>,
    pub version_id: Option<ObjectVersionId>,
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketEncryptionOutput {
    pub server_side_encryption_configuration: Option<ServerSideEncryptionConfiguration>,
}

pub type NextMarker = String;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ObjectLockMode {
    GOVERNANCE,
    COMPLIANCE,
}
impl AsRef<str> for ObjectLockMode {
    fn as_ref(&self) -> &str {
        match self {
            Self::GOVERNANCE => "GOVERNANCE",
            Self::COMPLIANCE => "COMPLIANCE",
        }
    }
}
impl TryFrom<&str> for ObjectLockMode {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "GOVERNANCE" => Ok(Self::GOVERNANCE),
            "COMPLIANCE" => Ok(Self::COMPLIANCE),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum OwnerOverride {
    Destination,
}
impl AsRef<str> for OwnerOverride {
    fn as_ref(&self) -> &str {
        match self {
            Self::Destination => "Destination",
        }
    }
}
impl TryFrom<&str> for OwnerOverride {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "Destination" => Ok(Self::Destination),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Payer {
    Requester,
    BucketOwner,
}
impl AsRef<str> for Payer {
    fn as_ref(&self) -> &str {
        match self {
            Self::Requester => "Requester",
            Self::BucketOwner => "BucketOwner",
        }
    }
}
impl TryFrom<&str> for Payer {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "Requester" => Ok(Self::Requester),
            "BucketOwner" => Ok(Self::BucketOwner),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

pub type CreationDate = String;

#[derive(Debug, Clone)]
pub enum LifecycleRuleFilter {
    ObjectSizeLessThan(ObjectSizeLessThanBytes),
    ObjectSizeGreaterThan(ObjectSizeGreaterThanBytes),
    And(LifecycleRuleAndOperator),
    Prefix(Prefix),
    Tag(Tag),
}

#[derive(Debug, Default, Clone)]
pub struct PutBucketMetricsConfigurationRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
    pub id: Option<MetricsId>,
    pub metrics_configuration: Option<MetricsConfiguration>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Permission {
    FULL_CONTROL,
    WRITE,
    WRITE_ACP,
    READ,
    READ_ACP,
}
impl AsRef<str> for Permission {
    fn as_ref(&self) -> &str {
        match self {
            Self::FULL_CONTROL => "FULL_CONTROL",
            Self::WRITE => "WRITE",
            Self::WRITE_ACP => "WRITE_ACP",
            Self::READ => "READ",
            Self::READ_ACP => "READ_ACP",
        }
    }
}
impl TryFrom<&str> for Permission {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "FULL_CONTROL" => Ok(Self::FULL_CONTROL),
            "WRITE" => Ok(Self::WRITE),
            "WRITE_ACP" => Ok(Self::WRITE_ACP),
            "READ" => Ok(Self::READ),
            "READ_ACP" => Ok(Self::READ_ACP),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum RequestCharged {
    requester,
}
impl AsRef<str> for RequestCharged {
    fn as_ref(&self) -> &str {
        match self {
            Self::requester => "requester",
        }
    }
}
impl TryFrom<&str> for RequestCharged {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "requester" => Ok(Self::requester),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Owner {
    pub display_name: Option<DisplayName>,
    pub id: Option<Id>,
}

pub type OwnershipControlsRules = Vec<OwnershipControlsRule>;

#[derive(Debug, Default, Clone)]
pub struct PutBucketLifecycleConfigurationRequest {
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
    pub lifecycle_configuration: Option<BucketLifecycleConfiguration>,
}

pub type RequestRoute = String;

#[derive(Debug, Default, Clone)]
pub struct MultipartUpload {
    pub initiator: Option<Initiator>,
    pub owner: Option<Owner>,
    pub storage_class: Option<StorageClass>,
    pub key: Option<ObjectKey>,
    pub upload_id: Option<MultipartUploadId>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub initiated: Option<Initiated>,
}

pub type CopySourceSseCustomerKeyMd5 = String;

pub type Days = i32;

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketEncryption {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for DeleteBucketEncryption {
    const NAME: &'static str = "DeleteBucketEncryption";
    type Input = DeleteBucketEncryptionRequest;
    type Output = ();
    type Error = ();
}

pub type ExposeHeaders = Vec<ExposeHeader>;

#[derive(Debug, Default, Clone)]
pub struct ListObjectsV2Output {
    pub max_keys: Option<MaxKeys>,
    pub contents: Option<ObjectList>,
    pub common_prefixes: Option<CommonPrefixList>,
    pub encoding_type: Option<EncodingType>,
    pub key_count: Option<KeyCount>,
    pub next_continuation_token: Option<NextToken>,
    pub start_after: Option<StartAfter>,
    pub name: Option<BucketName>,
    pub prefix: Option<Prefix>,
    pub is_truncated: Option<IsTruncated>,
    pub continuation_token: Option<Token>,
    pub delimiter: Option<Delimiter>,
}

pub type NoncurrentVersionTransitionList = Vec<NoncurrentVersionTransition>;

#[derive(Debug, Default, Clone)]
pub struct ObjectLockLegalHold {
    pub status: Option<ObjectLockLegalHoldStatus>,
}

#[derive(Debug, Default, Clone)]
pub struct CorsRule {
    pub max_age_seconds: Option<MaxAgeSeconds>,
    pub expose_headers: Option<ExposeHeaders>,
    pub allowed_headers: Option<AllowedHeaders>,
    pub allowed_origins: Option<AllowedOrigins>,
    pub allowed_methods: Option<AllowedMethods>,
    pub id: Option<Id>,
}

#[derive(Debug, Default, Clone)]
pub struct RedirectAllRequestsTo {
    pub host_name: Option<HostName>,
    pub protocol: Option<Protocol>,
}

#[derive(Debug, Default, Clone)]
pub struct BucketAlreadyExists {}

#[derive(Debug, Default, Clone)]
pub struct DeleteMarkerReplication {
    pub status: Option<DeleteMarkerReplicationStatus>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BucketAccelerateStatus {
    Enabled,
    Suspended,
}
impl AsRef<str> for BucketAccelerateStatus {
    fn as_ref(&self) -> &str {
        match self {
            Self::Enabled => "Enabled",
            Self::Suspended => "Suspended",
        }
    }
}
impl TryFrom<&str> for BucketAccelerateStatus {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "Enabled" => Ok(Self::Enabled),
            "Suspended" => Ok(Self::Suspended),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

pub type FetchOwner = bool;

pub type IntelligentTieringDays = i32;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum InventoryFrequency {
    Daily,
    Weekly,
}
impl AsRef<str> for InventoryFrequency {
    fn as_ref(&self) -> &str {
        match self {
            Self::Daily => "Daily",
            Self::Weekly => "Weekly",
        }
    }
}
impl TryFrom<&str> for InventoryFrequency {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "Daily" => Ok(Self::Daily),
            "Weekly" => Ok(Self::Weekly),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct IndexDocument {
    pub suffix: Option<Suffix>,
}

pub type Start = i64;

pub type SsekmsEncryptionContext = String;

#[derive(Debug, Default, Clone)]
pub struct ListObjectsV2Request {
    pub max_keys: Option<MaxKeys>,
    pub continuation_token: Option<Token>,
    pub delimiter: Option<Delimiter>,
    pub bucket: Option<BucketName>,
    pub encoding_type: Option<EncodingType>,
    pub expected_bucket_owner: Option<AccountId>,
    pub prefix: Option<Prefix>,
    pub request_payer: Option<RequestPayer>,
    pub start_after: Option<StartAfter>,
    pub fetch_owner: Option<FetchOwner>,
}

#[derive(Debug, Default, Clone)]
pub struct GetObjectAttributesOutput {
    pub request_charged: Option<RequestCharged>,
    pub delete_marker: Option<DeleteMarker>,
    pub e_tag: Option<ETag>,
    pub object_parts: Option<GetObjectAttributesParts>,
    pub last_modified: Option<LastModified>,
    pub object_size: Option<ObjectSize>,
    pub storage_class: Option<StorageClass>,
    pub checksum: Option<Checksum>,
    pub version_id: Option<ObjectVersionId>,
}

pub type ObjectIdentifierList = Vec<ObjectIdentifier>;

#[derive(Debug, Default, Clone)]
pub struct Stats {
    pub bytes_processed: Option<BytesProcessed>,
    pub bytes_returned: Option<BytesReturned>,
    pub bytes_scanned: Option<BytesScanned>,
}

#[derive(Debug, Default, Clone)]
pub struct GetObjectTorrentRequest {
    pub key: Option<ObjectKey>,
    pub request_payer: Option<RequestPayer>,
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug, Default, Clone)]
pub struct NotificationConfiguration {
    pub event_bridge_configuration: Option<EventBridgeConfiguration>,
    pub lambda_function_configurations: Option<LambdaFunctionConfigurationList>,
    pub topic_configurations: Option<TopicConfigurationList>,
    pub queue_configurations: Option<QueueConfigurationList>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum AnalyticsS3ExportFileFormat {
    CSV,
}
impl AsRef<str> for AnalyticsS3ExportFileFormat {
    fn as_ref(&self) -> &str {
        match self {
            Self::CSV => "CSV",
        }
    }
}
impl TryFrom<&str> for AnalyticsS3ExportFileFormat {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "CSV" => Ok(Self::CSV),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum AnalyticsFilter {
    Tag(Tag),
    And(AnalyticsAndOperator),
    Prefix(Prefix),
}

#[derive(Debug, Default, Clone)]
pub struct HeadObject {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for HeadObject {
    const NAME: &'static str = "HeadObject";
    type Input = HeadObjectRequest;
    type Output = HeadObjectOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketLoggingOutput {
    pub logging_enabled: Option<LoggingEnabled>,
}

pub type ObjectVersionId = String;

pub type CopySourceSseCustomerAlgorithm = String;

#[derive(Debug, Default, Clone)]
pub struct DeleteObjectTaggingRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub key: Option<ObjectKey>,
    pub version_id: Option<ObjectVersionId>,
    pub bucket: Option<BucketName>,
}

pub type ResponseCacheControl = String;

#[derive(Debug, Default, Clone)]
pub struct TargetGrant {
    pub grantee: Option<Grantee>,
    pub permission: Option<BucketLogsPermission>,
}

#[derive(Debug, Default, Clone)]
pub struct PutBucketOwnershipControls {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for PutBucketOwnershipControls {
    const NAME: &'static str = "PutBucketOwnershipControls";
    type Input = PutBucketOwnershipControlsRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct PutBucketPolicy {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for PutBucketPolicy {
    const NAME: &'static str = "PutBucketPolicy";
    type Input = PutBucketPolicyRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct DeleteObjectRequest {
    pub bypass_governance_retention: Option<BypassGovernanceRetention>,
    pub version_id: Option<ObjectVersionId>,
    pub request_payer: Option<RequestPayer>,
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
    pub mfa: Option<Mfa>,
    pub key: Option<ObjectKey>,
}

#[derive(Debug, Default, Clone)]
pub struct ListObjectsV2 {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for ListObjectsV2 {
    const NAME: &'static str = "ListObjectsV2";
    type Input = ListObjectsV2Request;
    type Output = ListObjectsV2Output;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct InventoryFilter {
    pub prefix: Option<Prefix>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ExpirationStatus {
    Enabled,
    Disabled,
}
impl AsRef<str> for ExpirationStatus {
    fn as_ref(&self) -> &str {
        match self {
            Self::Enabled => "Enabled",
            Self::Disabled => "Disabled",
        }
    }
}
impl TryFrom<&str> for ExpirationStatus {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "Enabled" => Ok(Self::Enabled),
            "Disabled" => Ok(Self::Disabled),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

pub type IsTruncated = bool;

pub type NextVersionIdMarker = String;

#[derive(Debug, Default, Clone)]
pub struct AccessControlTranslation {
    pub owner: Option<OwnerOverride>,
}

pub type ChecksumAlgorithmList = Vec<ChecksumAlgorithm>;

pub type IsPublic = bool;

#[derive(Debug, Default, Clone)]
pub struct HeadObjectRequest {
    pub if_none_match: Option<IfNoneMatch>,
    pub checksum_mode: Option<ChecksumMode>,
    pub range: Option<Range>,
    pub if_match: Option<IfMatch>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub bucket: Option<BucketName>,
    pub if_modified_since: Option<IfModifiedSince>,
    pub request_payer: Option<RequestPayer>,
    pub version_id: Option<ObjectVersionId>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub part_number: Option<PartNumber>,
    pub key: Option<ObjectKey>,
    pub sse_customer_key: Option<SseCustomerKey>,
    pub expected_bucket_owner: Option<AccountId>,
    pub if_unmodified_since: Option<IfUnmodifiedSince>,
}

#[derive(Debug, Default, Clone)]
pub struct ObjectPart {
    pub part_number: Option<PartNumber>,
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
    pub size: Option<Size>,
    pub checksum_crc32: Option<ChecksumCrc32>,
    pub checksum_sha256: Option<ChecksumSha256>,
    pub checksum_sha1: Option<ChecksumSha1>,
}

pub type AllowedOrigins = Vec<AllowedOrigin>;

pub type DaysAfterInitiation = i32;

pub type ContentMd5 = String;

#[derive(Debug, Default, Clone)]
pub struct PutBucketLifecycleConfiguration {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for PutBucketLifecycleConfiguration {
    const NAME: &'static str = "PutBucketLifecycleConfiguration";
    type Input = PutBucketLifecycleConfigurationRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketPolicyStatus {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for GetBucketPolicyStatus {
    const NAME: &'static str = "GetBucketPolicyStatus";
    type Input = GetBucketPolicyStatusRequest;
    type Output = GetBucketPolicyStatusOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketCorsRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}

#[derive(Debug, Default, Clone)]
pub struct GetObjectOutput {
    pub parts_count: Option<PartsCount>,
    pub storage_class: Option<StorageClass>,
    pub version_id: Option<ObjectVersionId>,
    pub metadata: Option<Metadata>,
    pub content_encoding: Option<ContentEncoding>,
    pub expiration: Option<Expiration>,
    pub ssekms_key_id: Option<SsekmsKeyId>,
    pub website_redirect_location: Option<WebsiteRedirectLocation>,
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
    pub e_tag: Option<ETag>,
    pub content_range: Option<ContentRange>,
    pub expires: Option<Expires>,
    pub request_charged: Option<RequestCharged>,
    pub object_lock_legal_hold_status: Option<ObjectLockLegalHoldStatus>,
    pub content_disposition: Option<ContentDisposition>,
    pub content_type: Option<ContentType>,
    pub cache_control: Option<CacheControl>,
    pub body: Option<StreamingBlob>,
    pub object_lock_retain_until_date: Option<ObjectLockRetainUntilDate>,
    pub tag_count: Option<TagCount>,
    pub missing_meta: Option<MissingMeta>,
    pub delete_marker: Option<DeleteMarker>,
    pub content_length: Option<ContentLength>,
    pub object_lock_mode: Option<ObjectLockMode>,
    pub checksum_sha256: Option<ChecksumSha256>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub checksum_sha1: Option<ChecksumSha1>,
    pub accept_ranges: Option<AcceptRanges>,
    pub restore: Option<Restore>,
    pub replication_status: Option<ReplicationStatus>,
    pub server_side_encryption: Option<ServerSideEncryption>,
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub checksum_crc32: Option<ChecksumCrc32>,
    pub content_language: Option<ContentLanguage>,
    pub last_modified: Option<LastModified>,
}

#[derive(Debug, Default, Clone)]
pub struct PutObjectOutput {
    pub expiration: Option<Expiration>,
    pub checksum_sha1: Option<ChecksumSha1>,
    pub server_side_encryption: Option<ServerSideEncryption>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub version_id: Option<ObjectVersionId>,
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    pub checksum_crc32: Option<ChecksumCrc32>,
    pub checksum_sha256: Option<ChecksumSha256>,
    pub request_charged: Option<RequestCharged>,
    pub e_tag: Option<ETag>,
    pub ssekms_encryption_context: Option<SsekmsEncryptionContext>,
    pub ssekms_key_id: Option<SsekmsKeyId>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
}

pub type AcceptRanges = String;

#[derive(Debug, Default, Clone)]
pub struct GetBucketTagging {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for GetBucketTagging {
    const NAME: &'static str = "GetBucketTagging";
    type Input = GetBucketTaggingRequest;
    type Output = GetBucketTaggingOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct AbortIncompleteMultipartUpload {
    pub days_after_initiation: Option<DaysAfterInitiation>,
}

pub type Date = String;

pub type NextToken = String;

#[derive(Debug, Default, Clone)]
pub struct GetBucketPolicy {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for GetBucketPolicy {
    const NAME: &'static str = "GetBucketPolicy";
    type Input = GetBucketPolicyRequest;
    type Output = GetBucketPolicyOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct PutPublicAccessBlockRequest {
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub expected_bucket_owner: Option<AccountId>,
    pub public_access_block_configuration: Option<PublicAccessBlockConfiguration>,
    pub bucket: Option<BucketName>,
    pub content_md5: Option<ContentMd5>,
}

#[derive(Debug, Default, Clone)]
pub struct DeleteObjectTagging {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for DeleteObjectTagging {
    const NAME: &'static str = "DeleteObjectTagging";
    type Input = DeleteObjectTaggingRequest;
    type Output = DeleteObjectTaggingOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketCorsOutput {
    pub cors_rules: Option<CorsRules>,
}

pub type GrantWrite = String;

pub type ChecksumSha1 = String;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ServerSideEncryption {
    AES256,
    aws_kms,
}
impl AsRef<str> for ServerSideEncryption {
    fn as_ref(&self) -> &str {
        match self {
            Self::AES256 => "AES256",
            Self::aws_kms => "aws:kms",
        }
    }
}
impl TryFrom<&str> for ServerSideEncryption {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "AES256" => Ok(Self::AES256),
            "aws:kms" => Ok(Self::aws_kms),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct SseKmsEncryptedObjects {
    pub status: Option<SseKmsEncryptedObjectsStatus>,
}

#[derive(Debug, Default, Clone)]
pub struct PutObjectLockConfiguration {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for PutObjectLockConfiguration {
    const NAME: &'static str = "PutObjectLockConfiguration";
    type Input = PutObjectLockConfigurationRequest;
    type Output = PutObjectLockConfigurationOutput;
    type Error = ();
}

pub type CopySourceIfMatch = String;

#[derive(Debug, Default, Clone)]
pub struct PutBucketAclRequest {
    pub grant_write_acp: Option<GrantWriteAcp>,
    pub grant_read_acp: Option<GrantReadAcp>,
    pub acl: Option<BucketCannedAcl>,
    pub grant_full_control: Option<GrantFullControl>,
    pub content_md5: Option<ContentMd5>,
    pub bucket: Option<BucketName>,
    pub grant_read: Option<GrantRead>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub expected_bucket_owner: Option<AccountId>,
    pub grant_write: Option<GrantWrite>,
    pub access_control_policy: Option<AccessControlPolicy>,
}

pub type Restore = String;

#[derive(Debug, Default, Clone)]
pub struct PutBucketTagging {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for PutBucketTagging {
    const NAME: &'static str = "PutBucketTagging";
    type Input = PutBucketTaggingRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum MetadataDirective {
    COPY,
    REPLACE,
}
impl AsRef<str> for MetadataDirective {
    fn as_ref(&self) -> &str {
        match self {
            Self::COPY => "COPY",
            Self::REPLACE => "REPLACE",
        }
    }
}
impl TryFrom<&str> for MetadataDirective {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "COPY" => Ok(Self::COPY),
            "REPLACE" => Ok(Self::REPLACE),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct FilterRule {
    pub name: Option<FilterRuleName>,
    pub value: Option<FilterRuleValue>,
}

pub type IfModifiedSince = String;

#[derive(Debug, Default, Clone)]
pub struct InputSerialization {
    pub csv: Option<CsvInput>,
    pub json: Option<JsonInput>,
    pub compression_type: Option<CompressionType>,
    pub parquet: Option<ParquetInput>,
}

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketReplicationRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}

#[derive(Debug, Default, Clone)]
pub struct Metrics {
    pub status: Option<MetricsStatus>,
    pub event_threshold: Option<ReplicationTimeValue>,
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketOwnershipControls {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for GetBucketOwnershipControls {
    const NAME: &'static str = "GetBucketOwnershipControls";
    type Input = GetBucketOwnershipControlsRequest;
    type Output = GetBucketOwnershipControlsOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketPolicyStatusRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketInventoryConfigurationRequest {
    pub bucket: Option<BucketName>,
    pub id: Option<InventoryId>,
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug, Default, Clone)]
pub struct GlacierJobParameters {
    pub tier: Option<Tier>,
}

#[derive(Debug, Default, Clone)]
pub struct ObjectIdentifier {
    pub version_id: Option<ObjectVersionId>,
    pub key: Option<ObjectKey>,
}

#[derive(Debug, Default, Clone)]
pub struct RestoreObject {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for RestoreObject {
    const NAME: &'static str = "RestoreObject";
    type Input = RestoreObjectRequest;
    type Output = RestoreObjectOutput;
    type Error = ();
}

pub type CopySourceIfNoneMatch = String;

#[derive(Debug, Default, Clone)]
pub struct PutBucketMetricsConfiguration {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for PutBucketMetricsConfiguration {
    const NAME: &'static str = "PutBucketMetricsConfiguration";
    type Input = PutBucketMetricsConfigurationRequest;
    type Output = ();
    type Error = ();
}

pub type MetricsId = String;

#[derive(Debug, Default, Clone)]
pub struct NoSuchBucket {}

#[derive(Debug, Default, Clone)]
pub struct ProgressEvent {
    pub details: Option<Progress>,
}

#[derive(Debug, Default, Clone)]
pub struct BucketLoggingStatus {
    pub logging_enabled: Option<LoggingEnabled>,
}

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketOwnershipControlsRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug, Default, Clone)]
pub struct ObjectNotInActiveTierError {}

pub type Token = String;

#[derive(Debug, Default, Clone)]
pub struct GetBucketPolicyStatusOutput {
    pub policy_status: Option<PolicyStatus>,
}

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketWebsiteRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}

#[derive(Debug, Default, Clone)]
pub struct Grant {
    pub permission: Option<Permission>,
    pub grantee: Option<Grantee>,
}

#[derive(Debug, Default, Clone)]
pub struct ReplicationRule {
    pub prefix: Option<Prefix>,
    pub destination: Option<Destination>,
    pub filter: Option<ReplicationRuleFilter>,
    pub priority: Option<Priority>,
    pub source_selection_criteria: Option<SourceSelectionCriteria>,
    pub existing_object_replication: Option<ExistingObjectReplication>,
    pub status: Option<ReplicationRuleStatus>,
    pub delete_marker_replication: Option<DeleteMarkerReplication>,
    pub id: Option<Id>,
}

#[derive(Debug, Default, Clone)]
pub struct Tag {
    pub key: Option<ObjectKey>,
    pub value: Option<Value>,
}

#[derive(Debug, Default, Clone)]
pub struct CompleteMultipartUploadOutput {
    pub e_tag: Option<ETag>,
    pub bucket: Option<BucketName>,
    pub checksum_crc32: Option<ChecksumCrc32>,
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
    pub checksum_sha1: Option<ChecksumSha1>,
    pub location: Option<Location>,
    pub request_charged: Option<RequestCharged>,
    pub key: Option<ObjectKey>,
    pub ssekms_key_id: Option<SsekmsKeyId>,
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    pub server_side_encryption: Option<ServerSideEncryption>,
    pub version_id: Option<ObjectVersionId>,
    pub expiration: Option<Expiration>,
    pub checksum_sha256: Option<ChecksumSha256>,
}

pub type ContentEncoding = String;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DeleteMarkerReplicationStatus {
    Enabled,
    Disabled,
}
impl AsRef<str> for DeleteMarkerReplicationStatus {
    fn as_ref(&self) -> &str {
        match self {
            Self::Enabled => "Enabled",
            Self::Disabled => "Disabled",
        }
    }
}
impl TryFrom<&str> for DeleteMarkerReplicationStatus {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "Enabled" => Ok(Self::Enabled),
            "Disabled" => Ok(Self::Disabled),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

pub type Expiration = String;

#[derive(Debug, Default, Clone)]
pub struct IntelligentTieringFilter {
    pub and: Option<IntelligentTieringAndOperator>,
    pub prefix: Option<Prefix>,
    pub tag: Option<Tag>,
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketInventoryConfiguration {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for GetBucketInventoryConfiguration {
    const NAME: &'static str = "GetBucketInventoryConfiguration";
    type Input = GetBucketInventoryConfigurationRequest;
    type Output = GetBucketInventoryConfigurationOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct ListBucketInventoryConfigurationsRequest {
    pub bucket: Option<BucketName>,
    pub continuation_token: Option<Token>,
    pub expected_bucket_owner: Option<AccountId>,
}

pub type BucketKeyEnabled = bool;

pub type ChecksumCrc32c = String;

#[derive(Debug, Default, Clone)]
pub struct Checksum {
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
    pub checksum_sha1: Option<ChecksumSha1>,
    pub checksum_crc32: Option<ChecksumCrc32>,
    pub checksum_sha256: Option<ChecksumSha256>,
}

pub type GetObjectResponseStatusCode = i32;

pub type ObjectLockRetainUntilDate = String;

#[derive(Debug, Default, Clone)]
pub struct AbortMultipartUpload {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for AbortMultipartUpload {
    const NAME: &'static str = "AbortMultipartUpload";
    type Input = AbortMultipartUploadRequest;
    type Output = AbortMultipartUploadOutput;
    type Error = ();
}

pub type AbortRuleId = String;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Event {
    _0,
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    _10,
    _11,
    _12,
    _13,
    _14,
    _15,
    _16,
    _17,
    _18,
    _19,
    _20,
    _21,
    _22,
    _23,
    _24,
    _25,
    _26,
}
impl AsRef<str> for Event {
    fn as_ref(&self) -> &str {
        match self {
            Self::_0 => "s3:ReducedRedundancyLostObject",
            Self::_1 => "s3:ObjectCreated:*",
            Self::_2 => "s3:ObjectCreated:Put",
            Self::_3 => "s3:ObjectCreated:Post",
            Self::_4 => "s3:ObjectCreated:Copy",
            Self::_5 => "s3:ObjectCreated:CompleteMultipartUpload",
            Self::_6 => "s3:ObjectRemoved:*",
            Self::_7 => "s3:ObjectRemoved:Delete",
            Self::_8 => "s3:ObjectRemoved:DeleteMarkerCreated",
            Self::_9 => "s3:ObjectRestore:*",
            Self::_10 => "s3:ObjectRestore:Post",
            Self::_11 => "s3:ObjectRestore:Completed",
            Self::_12 => "s3:Replication:*",
            Self::_13 => "s3:Replication:OperationFailedReplication",
            Self::_14 => "s3:Replication:OperationNotTracked",
            Self::_15 => "s3:Replication:OperationMissedThreshold",
            Self::_16 => "s3:Replication:OperationReplicatedAfterThreshold",
            Self::_17 => "s3:ObjectRestore:Delete",
            Self::_18 => "s3:LifecycleTransition",
            Self::_19 => "s3:IntelligentTiering",
            Self::_20 => "s3:ObjectAcl:Put",
            Self::_21 => "s3:LifecycleExpiration:*",
            Self::_22 => "s3:LifecycleExpiration:Delete",
            Self::_23 => "s3:LifecycleExpiration:DeleteMarkerCreated",
            Self::_24 => "s3:ObjectTagging:*",
            Self::_25 => "s3:ObjectTagging:Put",
            Self::_26 => "s3:ObjectTagging:Delete",
        }
    }
}
impl TryFrom<&str> for Event {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "s3:ReducedRedundancyLostObject" => Ok(Self::_0),
            "s3:ObjectCreated:*" => Ok(Self::_1),
            "s3:ObjectCreated:Put" => Ok(Self::_2),
            "s3:ObjectCreated:Post" => Ok(Self::_3),
            "s3:ObjectCreated:Copy" => Ok(Self::_4),
            "s3:ObjectCreated:CompleteMultipartUpload" => Ok(Self::_5),
            "s3:ObjectRemoved:*" => Ok(Self::_6),
            "s3:ObjectRemoved:Delete" => Ok(Self::_7),
            "s3:ObjectRemoved:DeleteMarkerCreated" => Ok(Self::_8),
            "s3:ObjectRestore:*" => Ok(Self::_9),
            "s3:ObjectRestore:Post" => Ok(Self::_10),
            "s3:ObjectRestore:Completed" => Ok(Self::_11),
            "s3:Replication:*" => Ok(Self::_12),
            "s3:Replication:OperationFailedReplication" => Ok(Self::_13),
            "s3:Replication:OperationNotTracked" => Ok(Self::_14),
            "s3:Replication:OperationMissedThreshold" => Ok(Self::_15),
            "s3:Replication:OperationReplicatedAfterThreshold" => Ok(Self::_16),
            "s3:ObjectRestore:Delete" => Ok(Self::_17),
            "s3:LifecycleTransition" => Ok(Self::_18),
            "s3:IntelligentTiering" => Ok(Self::_19),
            "s3:ObjectAcl:Put" => Ok(Self::_20),
            "s3:LifecycleExpiration:*" => Ok(Self::_21),
            "s3:LifecycleExpiration:Delete" => Ok(Self::_22),
            "s3:LifecycleExpiration:DeleteMarkerCreated" => Ok(Self::_23),
            "s3:ObjectTagging:*" => Ok(Self::_24),
            "s3:ObjectTagging:Put" => Ok(Self::_25),
            "s3:ObjectTagging:Delete" => Ok(Self::_26),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketNotificationConfiguration {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for GetBucketNotificationConfiguration {
    const NAME: &'static str = "GetBucketNotificationConfiguration";
    type Input = GetBucketNotificationConfigurationRequest;
    type Output = NotificationConfiguration;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct DeleteMarkerEntry {
    pub key: Option<ObjectKey>,
    pub version_id: Option<ObjectVersionId>,
    pub is_latest: Option<IsLatest>,
    pub last_modified: Option<LastModified>,
    pub owner: Option<Owner>,
}

pub type ObjectLockEnabledForBucket = bool;

pub type ObjectLockToken = String;

#[derive(Debug, Default, Clone)]
pub struct PutObjectAclOutput {
    pub request_charged: Option<RequestCharged>,
}

pub type ObjectSizeLessThanBytes = i64;

#[derive(Debug, Default, Clone)]
pub struct GetBucketWebsiteOutput {
    pub redirect_all_requests_to: Option<RedirectAllRequestsTo>,
    pub error_document: Option<ErrorDocument>,
    pub routing_rules: Option<RoutingRules>,
    pub index_document: Option<IndexDocument>,
}

pub type NextKeyMarker = String;

pub type QueueArn = String;

#[derive(Debug, Default, Clone)]
pub struct ScanRange {
    pub end: Option<End>,
    pub start: Option<Start>,
}

pub type DisplayName = String;

#[derive(Debug, Default, Clone)]
pub struct AbortMultipartUploadOutput {
    pub request_charged: Option<RequestCharged>,
}

#[derive(Debug, Clone)]
pub enum SelectObjectContentEventStream {
    Stats(StatsEvent),
    Cont(ContinuationEvent),
    Progress(ProgressEvent),
    Records(RecordsEvent),
    End(EndEvent),
}

#[derive(Debug, Default, Clone)]
pub struct SelectObjectContentOutput {
    pub payload: Option<SelectObjectContentEventStream>,
}

#[derive(Debug, Default, Clone)]
pub struct AccessControlPolicy {
    pub owner: Option<Owner>,
    pub grants: Option<Grants>,
}

#[derive(Debug, Default, Clone)]
pub struct DeleteBucket {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for DeleteBucket {
    const NAME: &'static str = "DeleteBucket";
    type Input = DeleteBucketRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct PutBucketAccelerateConfigurationRequest {
    pub accelerate_configuration: Option<AccelerateConfiguration>,
    pub bucket: Option<BucketName>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug, Default, Clone)]
pub struct CreateMultipartUploadRequest {
    pub content_disposition: Option<ContentDisposition>,
    pub acl: Option<ObjectCannedAcl>,
    pub grant_read: Option<GrantRead>,
    pub expires: Option<Expires>,
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    pub storage_class: Option<StorageClass>,
    pub key: Option<ObjectKey>,
    pub website_redirect_location: Option<WebsiteRedirectLocation>,
    pub cache_control: Option<CacheControl>,
    pub content_language: Option<ContentLanguage>,
    pub ssekms_encryption_context: Option<SsekmsEncryptionContext>,
    pub sse_customer_key: Option<SseCustomerKey>,
    pub ssekms_key_id: Option<SsekmsKeyId>,
    pub grant_write_acp: Option<GrantWriteAcp>,
    pub server_side_encryption: Option<ServerSideEncryption>,
    pub content_encoding: Option<ContentEncoding>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub bucket: Option<BucketName>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub request_payer: Option<RequestPayer>,
    pub tagging: Option<TaggingHeader>,
    pub object_lock_legal_hold_status: Option<ObjectLockLegalHoldStatus>,
    pub grant_full_control: Option<GrantFullControl>,
    pub expected_bucket_owner: Option<AccountId>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub content_type: Option<ContentType>,
    pub grant_read_acp: Option<GrantReadAcp>,
    pub object_lock_retain_until_date: Option<ObjectLockRetainUntilDate>,
    pub object_lock_mode: Option<ObjectLockMode>,
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Default, Clone)]
pub struct UploadPartCopyOutput {
    pub copy_part_result: Option<CopyPartResult>,
    pub copy_source_version_id: Option<CopySourceVersionId>,
    pub server_side_encryption: Option<ServerSideEncryption>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub request_charged: Option<RequestCharged>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    pub ssekms_key_id: Option<SsekmsKeyId>,
}

#[derive(Debug, Default, Clone)]
pub struct ListMultipartUploadsRequest {
    pub prefix: Option<Prefix>,
    pub encoding_type: Option<EncodingType>,
    pub key_marker: Option<KeyMarker>,
    pub upload_id_marker: Option<UploadIdMarker>,
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
    pub delimiter: Option<Delimiter>,
    pub max_uploads: Option<MaxUploads>,
}

#[derive(Debug, Default, Clone)]
pub struct ListBucketIntelligentTieringConfigurationsOutput {
    pub continuation_token: Option<Token>,
    pub is_truncated: Option<IsTruncated>,
    pub intelligent_tiering_configuration_list: Option<IntelligentTieringConfigurationList>,
    pub next_continuation_token: Option<NextToken>,
}

#[derive(Debug, Default, Clone)]
pub struct InventorySchedule {
    pub frequency: Option<InventoryFrequency>,
}

#[derive(Debug, Default, Clone)]
pub struct ListBucketInventoryConfigurations {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for ListBucketInventoryConfigurations {
    const NAME: &'static str = "ListBucketInventoryConfigurations";
    type Input = ListBucketInventoryConfigurationsRequest;
    type Output = ListBucketInventoryConfigurationsOutput;
    type Error = ();
}

pub type MaxUploads = i32;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ReplicationTimeStatus {
    Enabled,
    Disabled,
}
impl AsRef<str> for ReplicationTimeStatus {
    fn as_ref(&self) -> &str {
        match self {
            Self::Enabled => "Enabled",
            Self::Disabled => "Disabled",
        }
    }
}
impl TryFrom<&str> for ReplicationTimeStatus {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "Enabled" => Ok(Self::Enabled),
            "Disabled" => Ok(Self::Disabled),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

pub type TopicArn = String;

pub type AllowedMethods = Vec<AllowedMethod>;

#[derive(Debug, Default, Clone)]
pub struct VersioningConfiguration {
    pub mfa_delete: Option<MfaDelete>,
    pub status: Option<BucketVersioningStatus>,
}

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketCors {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for DeleteBucketCors {
    const NAME: &'static str = "DeleteBucketCors";
    type Input = DeleteBucketCorsRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct BucketAlreadyOwnedByYou {}

#[derive(Debug, Default, Clone)]
pub struct ListPartsRequest {
    pub request_payer: Option<RequestPayer>,
    pub expected_bucket_owner: Option<AccountId>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub upload_id: Option<MultipartUploadId>,
    pub max_parts: Option<MaxParts>,
    pub key: Option<ObjectKey>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub sse_customer_key: Option<SseCustomerKey>,
    pub bucket: Option<BucketName>,
    pub part_number_marker: Option<PartNumberMarker>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ObjectAttributes {
    ETAG,
    CHECKSUM,
    OBJECT_PARTS,
    STORAGE_CLASS,
    OBJECT_SIZE,
}
impl AsRef<str> for ObjectAttributes {
    fn as_ref(&self) -> &str {
        match self {
            Self::ETAG => "ETag",
            Self::CHECKSUM => "Checksum",
            Self::OBJECT_PARTS => "ObjectParts",
            Self::STORAGE_CLASS => "StorageClass",
            Self::OBJECT_SIZE => "ObjectSize",
        }
    }
}
impl TryFrom<&str> for ObjectAttributes {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "ETag" => Ok(Self::ETAG),
            "Checksum" => Ok(Self::CHECKSUM),
            "ObjectParts" => Ok(Self::OBJECT_PARTS),
            "StorageClass" => Ok(Self::STORAGE_CLASS),
            "ObjectSize" => Ok(Self::OBJECT_SIZE),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ObjectLockLegalHoldStatus {
    ON,
    OFF,
}
impl AsRef<str> for ObjectLockLegalHoldStatus {
    fn as_ref(&self) -> &str {
        match self {
            Self::ON => "ON",
            Self::OFF => "OFF",
        }
    }
}
impl TryFrom<&str> for ObjectLockLegalHoldStatus {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "ON" => Ok(Self::ON),
            "OFF" => Ok(Self::OFF),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

pub type UploadIdMarker = String;

#[derive(Debug, Default, Clone)]
pub struct PublicAccessBlockConfiguration {
    pub restrict_public_buckets: Option<Setting>,
    pub block_public_policy: Option<Setting>,
    pub block_public_acls: Option<Setting>,
    pub ignore_public_acls: Option<Setting>,
}

pub type SseCustomerKeyMd5 = String;

#[derive(Debug, Default, Clone)]
pub struct GetObjectTorrent {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for GetObjectTorrent {
    const NAME: &'static str = "GetObjectTorrent";
    type Input = GetObjectTorrentRequest;
    type Output = GetObjectTorrentOutput;
    type Error = ();
}

pub type ResponseContentDisposition = String;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ArchiveStatus {
    ARCHIVE_ACCESS,
    DEEP_ARCHIVE_ACCESS,
}
impl AsRef<str> for ArchiveStatus {
    fn as_ref(&self) -> &str {
        match self {
            Self::ARCHIVE_ACCESS => "ARCHIVE_ACCESS",
            Self::DEEP_ARCHIVE_ACCESS => "DEEP_ARCHIVE_ACCESS",
        }
    }
}
impl TryFrom<&str> for ArchiveStatus {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "ARCHIVE_ACCESS" => Ok(Self::ARCHIVE_ACCESS),
            "DEEP_ARCHIVE_ACCESS" => Ok(Self::DEEP_ARCHIVE_ACCESS),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

pub type Delimiter = String;

#[derive(Debug, Default, Clone)]
pub struct ExistingObjectReplication {
    pub status: Option<ExistingObjectReplicationStatus>,
}

#[derive(Debug, Default, Clone)]
pub struct UploadPartOutput {
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub ssekms_key_id: Option<SsekmsKeyId>,
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    pub request_charged: Option<RequestCharged>,
    pub server_side_encryption: Option<ServerSideEncryption>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub checksum_crc32: Option<ChecksumCrc32>,
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
    pub checksum_sha1: Option<ChecksumSha1>,
    pub checksum_sha256: Option<ChecksumSha256>,
    pub e_tag: Option<ETag>,
}

#[derive(Debug, Default, Clone)]
pub struct CreateMultipartUpload {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for CreateMultipartUpload {
    const NAME: &'static str = "CreateMultipartUpload";
    type Input = CreateMultipartUploadRequest;
    type Output = CreateMultipartUploadOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct CopyPartResult {
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
    pub e_tag: Option<ETag>,
    pub last_modified: Option<LastModified>,
    pub checksum_crc32: Option<ChecksumCrc32>,
    pub checksum_sha1: Option<ChecksumSha1>,
    pub checksum_sha256: Option<ChecksumSha256>,
}

pub type ChecksumSha256 = String;

#[derive(Debug, Default, Clone)]
pub struct CsvOutput {
    pub quote_escape_character: Option<QuoteEscapeCharacter>,
    pub quote_character: Option<QuoteCharacter>,
    pub quote_fields: Option<QuoteFields>,
    pub record_delimiter: Option<RecordDelimiter>,
    pub field_delimiter: Option<FieldDelimiter>,
}

pub type AnalyticsConfigurationList = Vec<AnalyticsConfiguration>;

pub type Expression = String;

#[derive(Debug, Default, Clone)]
pub struct DeleteObjectsRequest {
    pub bucket: Option<BucketName>,
    pub mfa: Option<Mfa>,
    pub delete: Option<Delete>,
    pub expected_bucket_owner: Option<AccountId>,
    pub bypass_governance_retention: Option<BypassGovernanceRetention>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub request_payer: Option<RequestPayer>,
}

#[derive(Debug, Default, Clone)]
pub struct PutBucketIntelligentTieringConfiguration {
    pub input: <Self as OperationShape>::Input,
    pub output: <Self as OperationShape>::Output,
    pub error: Option<<Self as OperationShape>::Error>,
}
impl OperationShape for PutBucketIntelligentTieringConfiguration {
    const NAME: &'static str = "PutBucketIntelligentTieringConfiguration";
    type Input = PutBucketIntelligentTieringConfigurationRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum S3Ops {
    HeadBucket,
    UploadPartCopy,
    GetObjectAcl,
    PutObjectLegalHold,
    WriteGetObjectResponse,
    GetBucketAcl,
    PutBucketVersioning,
    ListBucketMetricsConfigurations,
    GetBucketReplication,
    DeleteObjects,
    GetObjectAttributes,
    GetBucketLifecycleConfiguration,
    DeletePublicAccessBlock,
    UploadPart,
    GetBucketRequestPayment,
    GetBucketIntelligentTieringConfiguration,
    GetPublicAccessBlock,
    ListParts,
    PutBucketLogging,
    PutBucketWebsite,
    DeleteBucketTagging,
    ListObjects,
    CreateBucket,
    ListMultipartUploads,
    DeleteObject,
    ListBucketIntelligentTieringConfigurations,
    GetBucketLogging,
    CompleteMultipartUpload,
    GetObjectLockConfiguration,
    PutBucketEncryption,
    GetObjectTagging,
    PutBucketAccelerateConfiguration,
    DeleteBucketOwnershipControls,
    GetObject,
    PutBucketRequestPayment,
    GetBucketCors,
    GetBucketMetricsConfiguration,
    DeleteBucketAnalyticsConfiguration,
    PutObjectAcl,
    CopyObject,
    GetBucketWebsite,
    PutBucketReplication,
    PutPublicAccessBlock,
    DeleteBucketLifecycle,
    GetObjectRetention,
    DeleteBucketWebsite,
    ListBuckets,
    PutBucketInventoryConfiguration,
    DeleteBucketReplication,
    PutBucketCors,
    PutBucketAnalyticsConfiguration,
    GetBucketVersioning,
    ListObjectVersions,
    PutObjectTagging,
    GetBucketEncryption,
    PutBucketAcl,
    SelectObjectContent,
    DeleteBucketMetricsConfiguration,
    PutObjectRetention,
    DeleteBucketIntelligentTieringConfiguration,
    GetBucketLocation,
    DeleteBucketPolicy,
    GetBucketAnalyticsConfiguration,
    PutBucketNotificationConfiguration,
    GetObjectLegalHold,
    DeleteBucketInventoryConfiguration,
    ListBucketAnalyticsConfigurations,
    PutObject,
    GetBucketAccelerateConfiguration,
    DeleteBucketEncryption,
    HeadObject,
    PutBucketOwnershipControls,
    PutBucketPolicy,
    ListObjectsV2,
    PutBucketLifecycleConfiguration,
    GetBucketPolicyStatus,
    GetBucketTagging,
    GetBucketPolicy,
    DeleteObjectTagging,
    PutObjectLockConfiguration,
    PutBucketTagging,
    GetBucketOwnershipControls,
    RestoreObject,
    PutBucketMetricsConfiguration,
    GetBucketInventoryConfiguration,
    AbortMultipartUpload,
    GetBucketNotificationConfiguration,
    DeleteBucket,
    ListBucketInventoryConfigurations,
    DeleteBucketCors,
    GetObjectTorrent,
    CreateMultipartUpload,
    PutBucketIntelligentTieringConfiguration,
}
#[doc = r" This macro calls a provided $macro for each S3 operation to generate code per op."]
macro_rules! generate_ops_code {
    ($ macro : ident) => {
        $macro!(HeadBucket);
        $macro!(UploadPartCopy);
        $macro!(GetObjectAcl);
        $macro!(PutObjectLegalHold);
        $macro!(WriteGetObjectResponse);
        $macro!(GetBucketAcl);
        $macro!(PutBucketVersioning);
        $macro!(ListBucketMetricsConfigurations);
        $macro!(GetBucketReplication);
        $macro!(DeleteObjects);
        $macro!(GetObjectAttributes);
        $macro!(GetBucketLifecycleConfiguration);
        $macro!(DeletePublicAccessBlock);
        $macro!(UploadPart);
        $macro!(GetBucketRequestPayment);
        $macro!(GetBucketIntelligentTieringConfiguration);
        $macro!(GetPublicAccessBlock);
        $macro!(ListParts);
        $macro!(PutBucketLogging);
        $macro!(PutBucketWebsite);
        $macro!(DeleteBucketTagging);
        $macro!(ListObjects);
        $macro!(CreateBucket);
        $macro!(ListMultipartUploads);
        $macro!(DeleteObject);
        $macro!(ListBucketIntelligentTieringConfigurations);
        $macro!(GetBucketLogging);
        $macro!(CompleteMultipartUpload);
        $macro!(GetObjectLockConfiguration);
        $macro!(PutBucketEncryption);
        $macro!(GetObjectTagging);
        $macro!(PutBucketAccelerateConfiguration);
        $macro!(DeleteBucketOwnershipControls);
        $macro!(GetObject);
        $macro!(PutBucketRequestPayment);
        $macro!(GetBucketCors);
        $macro!(GetBucketMetricsConfiguration);
        $macro!(DeleteBucketAnalyticsConfiguration);
        $macro!(PutObjectAcl);
        $macro!(CopyObject);
        $macro!(GetBucketWebsite);
        $macro!(PutBucketReplication);
        $macro!(PutPublicAccessBlock);
        $macro!(DeleteBucketLifecycle);
        $macro!(GetObjectRetention);
        $macro!(DeleteBucketWebsite);
        $macro!(ListBuckets);
        $macro!(PutBucketInventoryConfiguration);
        $macro!(DeleteBucketReplication);
        $macro!(PutBucketCors);
        $macro!(PutBucketAnalyticsConfiguration);
        $macro!(GetBucketVersioning);
        $macro!(ListObjectVersions);
        $macro!(PutObjectTagging);
        $macro!(GetBucketEncryption);
        $macro!(PutBucketAcl);
        $macro!(SelectObjectContent);
        $macro!(DeleteBucketMetricsConfiguration);
        $macro!(PutObjectRetention);
        $macro!(DeleteBucketIntelligentTieringConfiguration);
        $macro!(GetBucketLocation);
        $macro!(DeleteBucketPolicy);
        $macro!(GetBucketAnalyticsConfiguration);
        $macro!(PutBucketNotificationConfiguration);
        $macro!(GetObjectLegalHold);
        $macro!(DeleteBucketInventoryConfiguration);
        $macro!(ListBucketAnalyticsConfigurations);
        $macro!(PutObject);
        $macro!(GetBucketAccelerateConfiguration);
        $macro!(DeleteBucketEncryption);
        $macro!(HeadObject);
        $macro!(PutBucketOwnershipControls);
        $macro!(PutBucketPolicy);
        $macro!(ListObjectsV2);
        $macro!(PutBucketLifecycleConfiguration);
        $macro!(GetBucketPolicyStatus);
        $macro!(GetBucketTagging);
        $macro!(GetBucketPolicy);
        $macro!(DeleteObjectTagging);
        $macro!(PutObjectLockConfiguration);
        $macro!(PutBucketTagging);
        $macro!(GetBucketOwnershipControls);
        $macro!(RestoreObject);
        $macro!(PutBucketMetricsConfiguration);
        $macro!(GetBucketInventoryConfiguration);
        $macro!(AbortMultipartUpload);
        $macro!(GetBucketNotificationConfiguration);
        $macro!(DeleteBucket);
        $macro!(ListBucketInventoryConfigurations);
        $macro!(DeleteBucketCors);
        $macro!(GetObjectTorrent);
        $macro!(CreateMultipartUpload);
        $macro!(PutBucketIntelligentTieringConfiguration);
    };
}
#[doc = r" This macro calls a provided $macro for each S3 operation to generate item per op."]
macro_rules! generate_ops_items {
    ($ macro : ident) => {
        $macro!(HeadBucket);
        $macro!(UploadPartCopy);
        $macro!(GetObjectAcl);
        $macro!(PutObjectLegalHold);
        $macro!(WriteGetObjectResponse);
        $macro!(GetBucketAcl);
        $macro!(PutBucketVersioning);
        $macro!(ListBucketMetricsConfigurations);
        $macro!(GetBucketReplication);
        $macro!(DeleteObjects);
        $macro!(GetObjectAttributes);
        $macro!(GetBucketLifecycleConfiguration);
        $macro!(DeletePublicAccessBlock);
        $macro!(UploadPart);
        $macro!(GetBucketRequestPayment);
        $macro!(GetBucketIntelligentTieringConfiguration);
        $macro!(GetPublicAccessBlock);
        $macro!(ListParts);
        $macro!(PutBucketLogging);
        $macro!(PutBucketWebsite);
        $macro!(DeleteBucketTagging);
        $macro!(ListObjects);
        $macro!(CreateBucket);
        $macro!(ListMultipartUploads);
        $macro!(DeleteObject);
        $macro!(ListBucketIntelligentTieringConfigurations);
        $macro!(GetBucketLogging);
        $macro!(CompleteMultipartUpload);
        $macro!(GetObjectLockConfiguration);
        $macro!(PutBucketEncryption);
        $macro!(GetObjectTagging);
        $macro!(PutBucketAccelerateConfiguration);
        $macro!(DeleteBucketOwnershipControls);
        $macro!(GetObject);
        $macro!(PutBucketRequestPayment);
        $macro!(GetBucketCors);
        $macro!(GetBucketMetricsConfiguration);
        $macro!(DeleteBucketAnalyticsConfiguration);
        $macro!(PutObjectAcl);
        $macro!(CopyObject);
        $macro!(GetBucketWebsite);
        $macro!(PutBucketReplication);
        $macro!(PutPublicAccessBlock);
        $macro!(DeleteBucketLifecycle);
        $macro!(GetObjectRetention);
        $macro!(DeleteBucketWebsite);
        $macro!(ListBuckets);
        $macro!(PutBucketInventoryConfiguration);
        $macro!(DeleteBucketReplication);
        $macro!(PutBucketCors);
        $macro!(PutBucketAnalyticsConfiguration);
        $macro!(GetBucketVersioning);
        $macro!(ListObjectVersions);
        $macro!(PutObjectTagging);
        $macro!(GetBucketEncryption);
        $macro!(PutBucketAcl);
        $macro!(SelectObjectContent);
        $macro!(DeleteBucketMetricsConfiguration);
        $macro!(PutObjectRetention);
        $macro!(DeleteBucketIntelligentTieringConfiguration);
        $macro!(GetBucketLocation);
        $macro!(DeleteBucketPolicy);
        $macro!(GetBucketAnalyticsConfiguration);
        $macro!(PutBucketNotificationConfiguration);
        $macro!(GetObjectLegalHold);
        $macro!(DeleteBucketInventoryConfiguration);
        $macro!(ListBucketAnalyticsConfigurations);
        $macro!(PutObject);
        $macro!(GetBucketAccelerateConfiguration);
        $macro!(DeleteBucketEncryption);
        $macro!(HeadObject);
        $macro!(PutBucketOwnershipControls);
        $macro!(PutBucketPolicy);
        $macro!(ListObjectsV2);
        $macro!(PutBucketLifecycleConfiguration);
        $macro!(GetBucketPolicyStatus);
        $macro!(GetBucketTagging);
        $macro!(GetBucketPolicy);
        $macro!(DeleteObjectTagging);
        $macro!(PutObjectLockConfiguration);
        $macro!(PutBucketTagging);
        $macro!(GetBucketOwnershipControls);
        $macro!(RestoreObject);
        $macro!(PutBucketMetricsConfiguration);
        $macro!(GetBucketInventoryConfiguration);
        $macro!(AbortMultipartUpload);
        $macro!(GetBucketNotificationConfiguration);
        $macro!(DeleteBucket);
        $macro!(ListBucketInventoryConfigurations);
        $macro!(DeleteBucketCors);
        $macro!(GetObjectTorrent);
        $macro!(CreateMultipartUpload);
        $macro!(PutBucketIntelligentTieringConfiguration);
    };
}
#[doc = r" This macro matches a variable of S3Ops type and expands the provided $macro"]
#[doc = r" for each S3 operation to generate code handler per op."]
macro_rules! generate_ops_match {
    ($ macro : ident , $ op : expr) => {
        match ($op) {
            S3Ops::HeadBucket => {
                $macro!(HeadBucket)
            }
            S3Ops::UploadPartCopy => {
                $macro!(UploadPartCopy)
            }
            S3Ops::GetObjectAcl => {
                $macro!(GetObjectAcl)
            }
            S3Ops::PutObjectLegalHold => {
                $macro!(PutObjectLegalHold)
            }
            S3Ops::WriteGetObjectResponse => {
                $macro!(WriteGetObjectResponse)
            }
            S3Ops::GetBucketAcl => {
                $macro!(GetBucketAcl)
            }
            S3Ops::PutBucketVersioning => {
                $macro!(PutBucketVersioning)
            }
            S3Ops::ListBucketMetricsConfigurations => {
                $macro!(ListBucketMetricsConfigurations)
            }
            S3Ops::GetBucketReplication => {
                $macro!(GetBucketReplication)
            }
            S3Ops::DeleteObjects => {
                $macro!(DeleteObjects)
            }
            S3Ops::GetObjectAttributes => {
                $macro!(GetObjectAttributes)
            }
            S3Ops::GetBucketLifecycleConfiguration => {
                $macro!(GetBucketLifecycleConfiguration)
            }
            S3Ops::DeletePublicAccessBlock => {
                $macro!(DeletePublicAccessBlock)
            }
            S3Ops::UploadPart => {
                $macro!(UploadPart)
            }
            S3Ops::GetBucketRequestPayment => {
                $macro!(GetBucketRequestPayment)
            }
            S3Ops::GetBucketIntelligentTieringConfiguration => {
                $macro!(GetBucketIntelligentTieringConfiguration)
            }
            S3Ops::GetPublicAccessBlock => {
                $macro!(GetPublicAccessBlock)
            }
            S3Ops::ListParts => {
                $macro!(ListParts)
            }
            S3Ops::PutBucketLogging => {
                $macro!(PutBucketLogging)
            }
            S3Ops::PutBucketWebsite => {
                $macro!(PutBucketWebsite)
            }
            S3Ops::DeleteBucketTagging => {
                $macro!(DeleteBucketTagging)
            }
            S3Ops::ListObjects => {
                $macro!(ListObjects)
            }
            S3Ops::CreateBucket => {
                $macro!(CreateBucket)
            }
            S3Ops::ListMultipartUploads => {
                $macro!(ListMultipartUploads)
            }
            S3Ops::DeleteObject => {
                $macro!(DeleteObject)
            }
            S3Ops::ListBucketIntelligentTieringConfigurations => {
                $macro!(ListBucketIntelligentTieringConfigurations)
            }
            S3Ops::GetBucketLogging => {
                $macro!(GetBucketLogging)
            }
            S3Ops::CompleteMultipartUpload => {
                $macro!(CompleteMultipartUpload)
            }
            S3Ops::GetObjectLockConfiguration => {
                $macro!(GetObjectLockConfiguration)
            }
            S3Ops::PutBucketEncryption => {
                $macro!(PutBucketEncryption)
            }
            S3Ops::GetObjectTagging => {
                $macro!(GetObjectTagging)
            }
            S3Ops::PutBucketAccelerateConfiguration => {
                $macro!(PutBucketAccelerateConfiguration)
            }
            S3Ops::DeleteBucketOwnershipControls => {
                $macro!(DeleteBucketOwnershipControls)
            }
            S3Ops::GetObject => {
                $macro!(GetObject)
            }
            S3Ops::PutBucketRequestPayment => {
                $macro!(PutBucketRequestPayment)
            }
            S3Ops::GetBucketCors => {
                $macro!(GetBucketCors)
            }
            S3Ops::GetBucketMetricsConfiguration => {
                $macro!(GetBucketMetricsConfiguration)
            }
            S3Ops::DeleteBucketAnalyticsConfiguration => {
                $macro!(DeleteBucketAnalyticsConfiguration)
            }
            S3Ops::PutObjectAcl => {
                $macro!(PutObjectAcl)
            }
            S3Ops::CopyObject => {
                $macro!(CopyObject)
            }
            S3Ops::GetBucketWebsite => {
                $macro!(GetBucketWebsite)
            }
            S3Ops::PutBucketReplication => {
                $macro!(PutBucketReplication)
            }
            S3Ops::PutPublicAccessBlock => {
                $macro!(PutPublicAccessBlock)
            }
            S3Ops::DeleteBucketLifecycle => {
                $macro!(DeleteBucketLifecycle)
            }
            S3Ops::GetObjectRetention => {
                $macro!(GetObjectRetention)
            }
            S3Ops::DeleteBucketWebsite => {
                $macro!(DeleteBucketWebsite)
            }
            S3Ops::ListBuckets => {
                $macro!(ListBuckets)
            }
            S3Ops::PutBucketInventoryConfiguration => {
                $macro!(PutBucketInventoryConfiguration)
            }
            S3Ops::DeleteBucketReplication => {
                $macro!(DeleteBucketReplication)
            }
            S3Ops::PutBucketCors => {
                $macro!(PutBucketCors)
            }
            S3Ops::PutBucketAnalyticsConfiguration => {
                $macro!(PutBucketAnalyticsConfiguration)
            }
            S3Ops::GetBucketVersioning => {
                $macro!(GetBucketVersioning)
            }
            S3Ops::ListObjectVersions => {
                $macro!(ListObjectVersions)
            }
            S3Ops::PutObjectTagging => {
                $macro!(PutObjectTagging)
            }
            S3Ops::GetBucketEncryption => {
                $macro!(GetBucketEncryption)
            }
            S3Ops::PutBucketAcl => {
                $macro!(PutBucketAcl)
            }
            S3Ops::SelectObjectContent => {
                $macro!(SelectObjectContent)
            }
            S3Ops::DeleteBucketMetricsConfiguration => {
                $macro!(DeleteBucketMetricsConfiguration)
            }
            S3Ops::PutObjectRetention => {
                $macro!(PutObjectRetention)
            }
            S3Ops::DeleteBucketIntelligentTieringConfiguration => {
                $macro!(DeleteBucketIntelligentTieringConfiguration)
            }
            S3Ops::GetBucketLocation => {
                $macro!(GetBucketLocation)
            }
            S3Ops::DeleteBucketPolicy => {
                $macro!(DeleteBucketPolicy)
            }
            S3Ops::GetBucketAnalyticsConfiguration => {
                $macro!(GetBucketAnalyticsConfiguration)
            }
            S3Ops::PutBucketNotificationConfiguration => {
                $macro!(PutBucketNotificationConfiguration)
            }
            S3Ops::GetObjectLegalHold => {
                $macro!(GetObjectLegalHold)
            }
            S3Ops::DeleteBucketInventoryConfiguration => {
                $macro!(DeleteBucketInventoryConfiguration)
            }
            S3Ops::ListBucketAnalyticsConfigurations => {
                $macro!(ListBucketAnalyticsConfigurations)
            }
            S3Ops::PutObject => {
                $macro!(PutObject)
            }
            S3Ops::GetBucketAccelerateConfiguration => {
                $macro!(GetBucketAccelerateConfiguration)
            }
            S3Ops::DeleteBucketEncryption => {
                $macro!(DeleteBucketEncryption)
            }
            S3Ops::HeadObject => {
                $macro!(HeadObject)
            }
            S3Ops::PutBucketOwnershipControls => {
                $macro!(PutBucketOwnershipControls)
            }
            S3Ops::PutBucketPolicy => {
                $macro!(PutBucketPolicy)
            }
            S3Ops::ListObjectsV2 => {
                $macro!(ListObjectsV2)
            }
            S3Ops::PutBucketLifecycleConfiguration => {
                $macro!(PutBucketLifecycleConfiguration)
            }
            S3Ops::GetBucketPolicyStatus => {
                $macro!(GetBucketPolicyStatus)
            }
            S3Ops::GetBucketTagging => {
                $macro!(GetBucketTagging)
            }
            S3Ops::GetBucketPolicy => {
                $macro!(GetBucketPolicy)
            }
            S3Ops::DeleteObjectTagging => {
                $macro!(DeleteObjectTagging)
            }
            S3Ops::PutObjectLockConfiguration => {
                $macro!(PutObjectLockConfiguration)
            }
            S3Ops::PutBucketTagging => {
                $macro!(PutBucketTagging)
            }
            S3Ops::GetBucketOwnershipControls => {
                $macro!(GetBucketOwnershipControls)
            }
            S3Ops::RestoreObject => {
                $macro!(RestoreObject)
            }
            S3Ops::PutBucketMetricsConfiguration => {
                $macro!(PutBucketMetricsConfiguration)
            }
            S3Ops::GetBucketInventoryConfiguration => {
                $macro!(GetBucketInventoryConfiguration)
            }
            S3Ops::AbortMultipartUpload => {
                $macro!(AbortMultipartUpload)
            }
            S3Ops::GetBucketNotificationConfiguration => {
                $macro!(GetBucketNotificationConfiguration)
            }
            S3Ops::DeleteBucket => {
                $macro!(DeleteBucket)
            }
            S3Ops::ListBucketInventoryConfigurations => {
                $macro!(ListBucketInventoryConfigurations)
            }
            S3Ops::DeleteBucketCors => {
                $macro!(DeleteBucketCors)
            }
            S3Ops::GetObjectTorrent => {
                $macro!(GetObjectTorrent)
            }
            S3Ops::CreateMultipartUpload => {
                $macro!(CreateMultipartUpload)
            }
            S3Ops::PutBucketIntelligentTieringConfiguration => {
                $macro!(PutBucketIntelligentTieringConfiguration)
            }
        }
    };
}
pub(crate) use generate_ops_code;
pub(crate) use generate_ops_items;
pub(crate) use generate_ops_match;
