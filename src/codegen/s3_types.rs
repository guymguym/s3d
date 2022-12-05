#![allow(unused)]
#![allow(non_camel_case_types)]
use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;
use std::sync::Arc;
trait Op {
    type Input;
    type Output;
}

pub type AllowedHeader = String;

pub type CopySource = String;

pub type TaggingHeader = String;

pub type IfModifiedSince = String;

#[derive(Debug, Clone)]
pub struct IntelligentTieringFilter {
    pub prefix: Option<Prefix>,
    pub tag: Option<Tag>,
    pub and: Option<IntelligentTieringAndOperator>,
}

#[derive(Debug, Clone)]
pub struct DeleteBucketAnalyticsConfigurationRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
    pub id: Option<AnalyticsId>,
}

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

#[derive(Debug, Clone)]
pub struct ObjectLockLegalHold {
    pub status: Option<ObjectLockLegalHoldStatus>,
}

pub type QuoteCharacter = String;

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

pub type QueueArn = String;

pub type OwnershipControlsRules = Vec<OwnershipControlsRule>;

#[derive(Debug, Clone)]
pub struct AnalyticsAndOperator {
    pub tags: Option<TagSet>,
    pub prefix: Option<Prefix>,
}

#[derive(Debug, Clone)]
pub struct InventoryS3BucketDestination {
    pub account_id: Option<AccountId>,
    pub prefix: Option<Prefix>,
    pub encryption: Option<InventoryEncryption>,
    pub bucket: Option<BucketName>,
    pub format: Option<InventoryFormat>,
}

#[derive(Debug, Clone)]
pub struct ObjectNotInActiveTierError {}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PutBucketVersioning;
impl Op for PutBucketVersioning {
    type Input = PutBucketVersioningRequest;
    type Output = ();
}

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

#[derive(Debug, Clone)]
pub struct AnalyticsConfiguration {
    pub id: Option<AnalyticsId>,
    pub filter: Option<AnalyticsFilter>,
    pub storage_class_analysis: Option<StorageClassAnalysis>,
}

pub type IntelligentTieringConfigurationList = Vec<IntelligentTieringConfiguration>;

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

#[derive(Debug, Clone)]
pub struct SelectParameters {
    pub input_serialization: Option<InputSerialization>,
    pub output_serialization: Option<OutputSerialization>,
    pub expression_type: Option<ExpressionType>,
    pub expression: Option<Expression>,
}

pub type AcceptRanges = String;

pub type Expiration = String;

pub type LambdaFunctionConfigurationList = Vec<LambdaFunctionConfiguration>;

pub type Value = String;

#[derive(Debug, Clone)]
pub struct GetBucketEncryptionRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}

pub type InventoryOptionalFields = Vec<InventoryOptionalField>;

pub type GrantFullControl = String;

#[derive(Debug, Clone)]
pub struct CsvInput {
    pub allow_quoted_record_delimiter: Option<AllowQuotedRecordDelimiter>,
    pub file_header_info: Option<FileHeaderInfo>,
    pub quote_character: Option<QuoteCharacter>,
    pub quote_escape_character: Option<QuoteEscapeCharacter>,
    pub field_delimiter: Option<FieldDelimiter>,
    pub comments: Option<Comments>,
    pub record_delimiter: Option<RecordDelimiter>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ListBucketInventoryConfigurations;
impl Op for ListBucketInventoryConfigurations {
    type Input = ListBucketInventoryConfigurationsRequest;
    type Output = ListBucketInventoryConfigurationsOutput;
}

#[derive(Debug, Clone)]
pub struct CsvOutput {
    pub record_delimiter: Option<RecordDelimiter>,
    pub quote_fields: Option<QuoteFields>,
    pub field_delimiter: Option<FieldDelimiter>,
    pub quote_escape_character: Option<QuoteEscapeCharacter>,
    pub quote_character: Option<QuoteCharacter>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct GetObject;
impl Op for GetObject {
    type Input = GetObjectRequest;
    type Output = GetObjectOutput;
}

pub type Setting = bool;

#[derive(Debug, Clone)]
pub struct RoutingRule {
    pub redirect: Option<Redirect>,
    pub condition: Option<Condition>,
}

pub type FilterRuleList = Vec<FilterRule>;

#[derive(Debug, Clone)]
pub struct ListBucketAnalyticsConfigurationsRequest {
    pub continuation_token: Option<Token>,
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}

#[derive(Debug, Clone)]
pub struct LambdaFunctionConfiguration {
    pub filter: Option<NotificationConfigurationFilter>,
    pub id: Option<NotificationId>,
    pub events: Option<EventList>,
    pub lambda_function_arn: Option<LambdaFunctionArn>,
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

#[derive(Debug, Clone)]
pub struct PutBucketVersioningRequest {
    pub bucket: Option<BucketName>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub expected_bucket_owner: Option<AccountId>,
    pub versioning_configuration: Option<VersioningConfiguration>,
    pub mfa: Option<Mfa>,
    pub content_md5: Option<ContentMd5>,
}

pub type UserMetadata = Vec<MetadataEntry>;

#[derive(Debug, Clone)]
pub struct MultipartUpload {
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub storage_class: Option<StorageClass>,
    pub initiator: Option<Initiator>,
    pub initiated: Option<Initiated>,
    pub key: Option<ObjectKey>,
    pub owner: Option<Owner>,
    pub upload_id: Option<MultipartUploadId>,
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

#[derive(Debug, Clone)]
pub struct DeleteBucketEncryptionRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct GetBucketLifecycleConfiguration;
impl Op for GetBucketLifecycleConfiguration {
    type Input = GetBucketLifecycleConfigurationRequest;
    type Output = GetBucketLifecycleConfigurationOutput;
}

#[derive(Debug, Clone)]
pub struct ReplicaModifications {
    pub status: Option<ReplicaModificationsStatus>,
}

#[derive(Debug, Clone)]
pub struct CopyObjectResult {
    pub last_modified: Option<LastModified>,
    pub checksum_sha1: Option<ChecksumSha1>,
    pub checksum_crc32: Option<ChecksumCrc32>,
    pub checksum_sha256: Option<ChecksumSha256>,
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
    pub e_tag: Option<ETag>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PutObjectTagging;
impl Op for PutObjectTagging {
    type Input = PutObjectTaggingRequest;
    type Output = PutObjectTaggingOutput;
}

pub type ContentMd5 = String;

#[derive(Debug, Clone)]
pub struct CreateMultipartUploadOutput {
    pub abort_date: Option<AbortDate>,
    pub abort_rule_id: Option<AbortRuleId>,
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    pub key: Option<ObjectKey>,
    pub server_side_encryption: Option<ServerSideEncryption>,
    pub upload_id: Option<MultipartUploadId>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub ssekms_key_id: Option<SsekmsKeyId>,
    pub bucket: Option<BucketName>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub request_charged: Option<RequestCharged>,
    pub ssekms_encryption_context: Option<SsekmsEncryptionContext>,
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

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct GetBucketReplication;
impl Op for GetBucketReplication {
    type Input = GetBucketReplicationRequest;
    type Output = GetBucketReplicationOutput;
}

#[derive(Debug, Clone)]
pub struct ListBucketAnalyticsConfigurationsOutput {
    pub continuation_token: Option<Token>,
    pub next_continuation_token: Option<NextToken>,
    pub analytics_configuration_list: Option<AnalyticsConfigurationList>,
    pub is_truncated: Option<IsTruncated>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PutObject;
impl Op for PutObject {
    type Input = PutObjectRequest;
    type Output = PutObjectOutput;
}

pub type NextKeyMarker = String;

pub type AnalyticsConfigurationList = Vec<AnalyticsConfiguration>;

pub type BytesScanned = i64;

#[derive(Debug, Clone)]
pub struct ListObjectVersionsRequest {
    pub prefix: Option<Prefix>,
    pub bucket: Option<BucketName>,
    pub key_marker: Option<KeyMarker>,
    pub expected_bucket_owner: Option<AccountId>,
    pub delimiter: Option<Delimiter>,
    pub encoding_type: Option<EncodingType>,
    pub max_keys: Option<MaxKeys>,
    pub version_id_marker: Option<VersionIdMarker>,
}

#[derive(Debug, Clone)]
pub struct ReplicationTime {
    pub status: Option<ReplicationTimeStatus>,
    pub time: Option<ReplicationTimeValue>,
}

#[derive(Debug, Clone)]
pub struct UploadPartCopyOutput {
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub copy_source_version_id: Option<CopySourceVersionId>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub request_charged: Option<RequestCharged>,
    pub server_side_encryption: Option<ServerSideEncryption>,
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    pub copy_part_result: Option<CopyPartResult>,
    pub ssekms_key_id: Option<SsekmsKeyId>,
}

#[derive(Debug, Clone)]
pub struct GetObjectAclOutput {
    pub grants: Option<Grants>,
    pub owner: Option<Owner>,
    pub request_charged: Option<RequestCharged>,
}

#[derive(Debug, Clone)]
pub struct LifecycleRule {
    pub transitions: Option<TransitionList>,
    pub status: Option<ExpirationStatus>,
    pub noncurrent_version_expiration: Option<NoncurrentVersionExpiration>,
    pub abort_incomplete_multipart_upload: Option<AbortIncompleteMultipartUpload>,
    pub expiration: Option<LifecycleExpiration>,
    pub noncurrent_version_transitions: Option<NoncurrentVersionTransitionList>,
    pub prefix: Option<Prefix>,
    pub filter: Option<LifecycleRuleFilter>,
    pub id: Option<Id>,
}

#[derive(Debug, Clone)]
pub struct PutObjectLockConfigurationRequest {
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub content_md5: Option<ContentMd5>,
    pub object_lock_configuration: Option<ObjectLockConfiguration>,
    pub token: Option<ObjectLockToken>,
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
    pub request_payer: Option<RequestPayer>,
}

#[derive(Debug, Clone)]
pub struct PutObjectLockConfigurationOutput {
    pub request_charged: Option<RequestCharged>,
}

pub type Description = String;

#[derive(Debug, Clone)]
pub struct Part {
    pub part_number: Option<PartNumber>,
    pub checksum_crc32: Option<ChecksumCrc32>,
    pub last_modified: Option<LastModified>,
    pub size: Option<Size>,
    pub checksum_sha1: Option<ChecksumSha1>,
    pub e_tag: Option<ETag>,
    pub checksum_sha256: Option<ChecksumSha256>,
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct GetObjectTorrent;
impl Op for GetObjectTorrent {
    type Input = GetObjectTorrentRequest;
    type Output = GetObjectTorrentOutput;
}

#[derive(Debug, Clone)]
pub struct PutObjectAclOutput {
    pub request_charged: Option<RequestCharged>,
}

#[derive(Debug, Clone)]
pub struct ListBucketMetricsConfigurationsRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub continuation_token: Option<Token>,
    pub bucket: Option<BucketName>,
}

pub type ErrorMessage = String;

#[derive(Debug, Clone)]
pub struct PutBucketReplicationRequest {
    pub replication_configuration: Option<ReplicationConfiguration>,
    pub bucket: Option<BucketName>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub token: Option<ObjectLockToken>,
    pub expected_bucket_owner: Option<AccountId>,
    pub content_md5: Option<ContentMd5>,
}

#[derive(Debug, Clone)]
pub struct DefaultRetention {
    pub mode: Option<ObjectLockRetentionMode>,
    pub years: Option<Years>,
    pub days: Option<Days>,
}

pub type AllowQuotedRecordDelimiter = bool;

#[derive(Debug, Clone)]
pub struct PutBucketLifecycleConfigurationRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
    pub lifecycle_configuration: Option<BucketLifecycleConfiguration>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
}

#[derive(Debug, Clone)]
pub struct ListBucketInventoryConfigurationsOutput {
    pub inventory_configuration_list: Option<InventoryConfigurationList>,
    pub is_truncated: Option<IsTruncated>,
    pub next_continuation_token: Option<NextToken>,
    pub continuation_token: Option<Token>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PutBucketMetricsConfiguration;
impl Op for PutBucketMetricsConfiguration {
    type Input = PutBucketMetricsConfigurationRequest;
    type Output = ();
}

#[derive(Debug, Clone)]
pub struct CorsRule {
    pub allowed_methods: Option<AllowedMethods>,
    pub allowed_origins: Option<AllowedOrigins>,
    pub allowed_headers: Option<AllowedHeaders>,
    pub max_age_seconds: Option<MaxAgeSeconds>,
    pub id: Option<Id>,
    pub expose_headers: Option<ExposeHeaders>,
}

#[derive(Debug, Clone)]
pub struct ReplicationRule {
    pub delete_marker_replication: Option<DeleteMarkerReplication>,
    pub existing_object_replication: Option<ExistingObjectReplication>,
    pub prefix: Option<Prefix>,
    pub source_selection_criteria: Option<SourceSelectionCriteria>,
    pub filter: Option<ReplicationRuleFilter>,
    pub id: Option<Id>,
    pub destination: Option<Destination>,
    pub priority: Option<Priority>,
    pub status: Option<ReplicationRuleStatus>,
}

pub type RestoreOutputPath = String;

pub type SsekmsEncryptionContext = String;

#[derive(Debug, Clone)]
pub struct AccessControlTranslation {
    pub owner: Option<OwnerOverride>,
}

#[derive(Debug, Clone)]
pub struct PutObjectOutput {
    pub ssekms_key_id: Option<SsekmsKeyId>,
    pub server_side_encryption: Option<ServerSideEncryption>,
    pub version_id: Option<ObjectVersionId>,
    pub checksum_crc32: Option<ChecksumCrc32>,
    pub checksum_sha256: Option<ChecksumSha256>,
    pub e_tag: Option<ETag>,
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
    pub request_charged: Option<RequestCharged>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub checksum_sha1: Option<ChecksumSha1>,
    pub expiration: Option<Expiration>,
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    pub ssekms_encryption_context: Option<SsekmsEncryptionContext>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
}

#[derive(Debug, Clone)]
pub struct CompleteMultipartUploadRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub multipart_upload: Option<CompletedMultipartUpload>,
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
    pub key: Option<ObjectKey>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub sse_customer_key: Option<SseCustomerKey>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub checksum_sha1: Option<ChecksumSha1>,
    pub checksum_crc32: Option<ChecksumCrc32>,
    pub checksum_sha256: Option<ChecksumSha256>,
    pub request_payer: Option<RequestPayer>,
    pub upload_id: Option<MultipartUploadId>,
    pub bucket: Option<BucketName>,
}

pub type ConfirmRemoveSelfBucketAccess = bool;

#[derive(Debug, Clone)]
pub struct OwnershipControlsRule {
    pub object_ownership: Option<ObjectOwnership>,
}

#[derive(Debug, Clone)]
pub struct PutBucketIntelligentTieringConfigurationRequest {
    pub bucket: Option<BucketName>,
    pub id: Option<IntelligentTieringId>,
    pub intelligent_tiering_configuration: Option<IntelligentTieringConfiguration>,
}

pub type ResponseContentLanguage = String;

pub type BucketKeyEnabled = bool;

#[derive(Debug, Clone)]
pub struct CopyObjectRequest {
    pub bucket: Option<BucketName>,
    pub expires: Option<Expires>,
    pub content_language: Option<ContentLanguage>,
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    pub content_type: Option<ContentType>,
    pub copy_source_sse_customer_algorithm: Option<CopySourceSseCustomerAlgorithm>,
    pub object_lock_retain_until_date: Option<ObjectLockRetainUntilDate>,
    pub expected_source_bucket_owner: Option<AccountId>,
    pub grant_write_acp: Option<GrantWriteAcp>,
    pub copy_source_if_unmodified_since: Option<CopySourceIfUnmodifiedSince>,
    pub storage_class: Option<StorageClass>,
    pub grant_read: Option<GrantRead>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub expected_bucket_owner: Option<AccountId>,
    pub ssekms_key_id: Option<SsekmsKeyId>,
    pub copy_source: Option<CopySource>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub sse_customer_key: Option<SseCustomerKey>,
    pub content_encoding: Option<ContentEncoding>,
    pub copy_source_if_match: Option<CopySourceIfMatch>,
    pub copy_source_if_modified_since: Option<CopySourceIfModifiedSince>,
    pub ssekms_encryption_context: Option<SsekmsEncryptionContext>,
    pub copy_source_if_none_match: Option<CopySourceIfNoneMatch>,
    pub tagging: Option<TaggingHeader>,
    pub tagging_directive: Option<TaggingDirective>,
    pub website_redirect_location: Option<WebsiteRedirectLocation>,
    pub grant_read_acp: Option<GrantReadAcp>,
    pub metadata: Option<Metadata>,
    pub object_lock_mode: Option<ObjectLockMode>,
    pub request_payer: Option<RequestPayer>,
    pub acl: Option<ObjectCannedAcl>,
    pub metadata_directive: Option<MetadataDirective>,
    pub cache_control: Option<CacheControl>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub server_side_encryption: Option<ServerSideEncryption>,
    pub copy_source_sse_customer_key: Option<CopySourceSseCustomerKey>,
    pub content_disposition: Option<ContentDisposition>,
    pub grant_full_control: Option<GrantFullControl>,
    pub copy_source_sse_customer_key_md5: Option<CopySourceSseCustomerKeyMd5>,
    pub key: Option<ObjectKey>,
    pub object_lock_legal_hold_status: Option<ObjectLockLegalHoldStatus>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct DeleteBucketOwnershipControls;
impl Op for DeleteBucketOwnershipControls {
    type Input = DeleteBucketOwnershipControlsRequest;
    type Output = ();
}

#[derive(Debug, Clone)]
pub struct GetBucketLoggingOutput {
    pub logging_enabled: Option<LoggingEnabled>,
}

#[derive(Debug, Clone)]
pub struct GetBucketMetricsConfigurationOutput {
    pub metrics_configuration: Option<MetricsConfiguration>,
}

pub type Restore = String;

pub type TagCount = i32;

pub type TopicConfigurationList = Vec<TopicConfiguration>;

pub type Expression = String;

pub type FetchOwner = bool;

#[derive(Debug, Clone)]
pub struct GetBucketLoggingRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}

#[derive(Debug, Clone)]
pub struct CreateBucketOutput {
    pub location: Option<Location>,
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

pub type KeyCount = i32;

#[derive(Debug, Clone)]
pub struct ExistingObjectReplication {
    pub status: Option<ExistingObjectReplicationStatus>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct AbortMultipartUpload;
impl Op for AbortMultipartUpload {
    type Input = AbortMultipartUploadRequest;
    type Output = AbortMultipartUploadOutput;
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

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct GetBucketTagging;
impl Op for GetBucketTagging {
    type Input = GetBucketTaggingRequest;
    type Output = GetBucketTaggingOutput;
}

#[derive(Debug, Clone)]
pub struct ProgressEvent {
    pub details: Option<Progress>,
}

pub type Minutes = i32;

pub type IfNoneMatch = String;

pub type MaxUploads = i32;

pub type QuoteEscapeCharacter = String;

pub type DeleteMarkerVersionId = String;

#[derive(Debug, Clone)]
pub struct ParquetInput {}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ListBucketIntelligentTieringConfigurations;
impl Op for ListBucketIntelligentTieringConfigurations {
    type Input = ListBucketIntelligentTieringConfigurationsRequest;
    type Output = ListBucketIntelligentTieringConfigurationsOutput;
}

pub type NextToken = String;

#[derive(Debug, Clone)]
pub struct ListBucketsOutput {
    pub owner: Option<Owner>,
    pub buckets: Option<Buckets>,
}

pub type GetObjectResponseStatusCode = i32;

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

#[derive(Debug, Clone)]
pub struct InventoryEncryption {
    pub ssekms: Option<Ssekms>,
    pub sses3: Option<Sses3>,
}

pub type ObjectVersionList = Vec<ObjectVersion>;

#[derive(Debug, Clone)]
pub struct GetBucketAclRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}

#[derive(Debug, Clone)]
pub struct BucketLifecycleConfiguration {
    pub rules: Option<LifecycleRules>,
}

#[derive(Debug, Clone)]
pub struct DeleteBucketWebsiteRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}

pub type IsEnabled = bool;

#[derive(Debug, Clone)]
pub struct BucketAlreadyOwnedByYou {}

pub type IsPublic = bool;

#[derive(Debug, Clone)]
pub struct NoSuchKey {}

#[derive(Debug, Clone)]
pub struct Tiering {
    pub days: Option<IntelligentTieringDays>,
    pub access_tier: Option<IntelligentTieringAccessTier>,
}

pub type LifecycleRules = Vec<LifecycleRule>;

#[derive(Debug, Clone)]
pub struct JsonOutput {
    pub record_delimiter: Option<RecordDelimiter>,
}

pub type ErrorCode = String;

#[derive(Debug, Clone)]
pub struct DeleteBucketIntelligentTieringConfigurationRequest {
    pub bucket: Option<BucketName>,
    pub id: Option<IntelligentTieringId>,
}

#[derive(Debug, Clone)]
pub struct ListObjectsV2Output {
    pub start_after: Option<StartAfter>,
    pub is_truncated: Option<IsTruncated>,
    pub common_prefixes: Option<CommonPrefixList>,
    pub max_keys: Option<MaxKeys>,
    pub prefix: Option<Prefix>,
    pub continuation_token: Option<Token>,
    pub encoding_type: Option<EncodingType>,
    pub key_count: Option<KeyCount>,
    pub name: Option<BucketName>,
    pub next_continuation_token: Option<NextToken>,
    pub contents: Option<ObjectList>,
    pub delimiter: Option<Delimiter>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PutBucketWebsite;
impl Op for PutBucketWebsite {
    type Input = PutBucketWebsiteRequest;
    type Output = ();
}

pub type ObjectSizeLessThanBytes = i64;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PutBucketNotificationConfiguration;
impl Op for PutBucketNotificationConfiguration {
    type Input = PutBucketNotificationConfigurationRequest;
    type Output = ();
}

#[derive(Debug, Clone)]
pub struct ListPartsRequest {
    pub sse_customer_key: Option<SseCustomerKey>,
    pub bucket: Option<BucketName>,
    pub upload_id: Option<MultipartUploadId>,
    pub part_number_marker: Option<PartNumberMarker>,
    pub key: Option<ObjectKey>,
    pub max_parts: Option<MaxParts>,
    pub expected_bucket_owner: Option<AccountId>,
    pub request_payer: Option<RequestPayer>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
}

#[derive(Debug, Clone)]
pub struct GetBucketPolicyStatusOutput {
    pub policy_status: Option<PolicyStatus>,
}

pub type VersionIdMarker = String;

#[derive(Debug, Clone)]
pub struct OutputLocation {
    pub s3: Option<S3Location>,
}

#[derive(Debug, Clone)]
pub struct ListMultipartUploadsRequest {
    pub delimiter: Option<Delimiter>,
    pub key_marker: Option<KeyMarker>,
    pub max_uploads: Option<MaxUploads>,
    pub prefix: Option<Prefix>,
    pub upload_id_marker: Option<UploadIdMarker>,
    pub bucket: Option<BucketName>,
    pub encoding_type: Option<EncodingType>,
    pub expected_bucket_owner: Option<AccountId>,
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

pub type AllowedMethod = String;

#[derive(Debug, Clone)]
pub struct GetBucketLocationRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct DeleteBucketMetricsConfiguration;
impl Op for DeleteBucketMetricsConfiguration {
    type Input = DeleteBucketMetricsConfigurationRequest;
    type Output = ();
}

pub type AllowedHeaders = Vec<AllowedHeader>;

#[derive(Debug, Clone)]
pub struct GetObjectLegalHoldOutput {
    pub legal_hold: Option<ObjectLockLegalHold>,
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

pub type ResponseContentDisposition = String;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct UploadPart;
impl Op for UploadPart {
    type Input = UploadPartRequest;
    type Output = UploadPartOutput;
}

#[derive(Debug, Clone)]
pub struct Owner {
    pub display_name: Option<DisplayName>,
    pub id: Option<Id>,
}

#[derive(Debug, Clone)]
pub struct WebsiteConfiguration {
    pub error_document: Option<ErrorDocument>,
    pub routing_rules: Option<RoutingRules>,
    pub redirect_all_requests_to: Option<RedirectAllRequestsTo>,
    pub index_document: Option<IndexDocument>,
}

pub type CopySourceIfUnmodifiedSince = String;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PutBucketInventoryConfiguration;
impl Op for PutBucketInventoryConfiguration {
    type Input = PutBucketInventoryConfigurationRequest;
    type Output = ();
}

#[derive(Debug, Clone)]
pub struct OwnershipControls {
    pub rules: Option<OwnershipControlsRules>,
}

#[derive(Debug, Clone)]
pub struct TopicConfiguration {
    pub id: Option<NotificationId>,
    pub events: Option<EventList>,
    pub filter: Option<NotificationConfigurationFilter>,
    pub topic_arn: Option<TopicArn>,
}

#[derive(Debug, Clone)]
pub struct GetBucketIntelligentTieringConfigurationOutput {
    pub intelligent_tiering_configuration: Option<IntelligentTieringConfiguration>,
}

#[derive(Debug, Clone)]
pub struct GetBucketRequestPaymentRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}

#[derive(Debug, Clone)]
pub struct HeadObjectRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub sse_customer_key: Option<SseCustomerKey>,
    pub checksum_mode: Option<ChecksumMode>,
    pub if_unmodified_since: Option<IfUnmodifiedSince>,
    pub version_id: Option<ObjectVersionId>,
    pub part_number: Option<PartNumber>,
    pub if_none_match: Option<IfNoneMatch>,
    pub range: Option<Range>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub bucket: Option<BucketName>,
    pub if_match: Option<IfMatch>,
    pub key: Option<ObjectKey>,
    pub request_payer: Option<RequestPayer>,
    pub if_modified_since: Option<IfModifiedSince>,
}

#[derive(Debug, Clone)]
pub struct BucketLoggingStatus {
    pub logging_enabled: Option<LoggingEnabled>,
}

pub type MetadataValue = String;

#[derive(Debug, Clone)]
pub struct ScanRange {
    pub end: Option<End>,
    pub start: Option<Start>,
}

#[derive(Debug, Clone)]
pub struct WriteGetObjectResponseRequest {
    pub accept_ranges: Option<AcceptRanges>,
    pub metadata: Option<Metadata>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub ssekms_key_id: Option<SsekmsKeyId>,
    pub version_id: Option<ObjectVersionId>,
    pub checksum_crc32: Option<ChecksumCrc32>,
    pub content_disposition: Option<ContentDisposition>,
    pub tag_count: Option<TagCount>,
    pub expiration: Option<Expiration>,
    pub content_type: Option<ContentType>,
    pub expires: Option<Expires>,
    pub last_modified: Option<LastModified>,
    pub e_tag: Option<ETag>,
    pub object_lock_mode: Option<ObjectLockMode>,
    pub parts_count: Option<PartsCount>,
    pub request_route: Option<RequestRoute>,
    pub checksum_sha1: Option<ChecksumSha1>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub request_charged: Option<RequestCharged>,
    pub server_side_encryption: Option<ServerSideEncryption>,
    pub cache_control: Option<CacheControl>,
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    pub content_language: Option<ContentLanguage>,
    pub content_encoding: Option<ContentEncoding>,
    pub error_message: Option<ErrorMessage>,
    pub content_range: Option<ContentRange>,
    pub body: Option<StreamingBlob>,
    pub content_length: Option<ContentLength>,
    pub delete_marker: Option<DeleteMarker>,
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
    pub missing_meta: Option<MissingMeta>,
    pub object_lock_legal_hold_status: Option<ObjectLockLegalHoldStatus>,
    pub replication_status: Option<ReplicationStatus>,
    pub request_token: Option<RequestToken>,
    pub restore: Option<Restore>,
    pub storage_class: Option<StorageClass>,
    pub checksum_sha256: Option<ChecksumSha256>,
    pub status_code: Option<GetObjectResponseStatusCode>,
    pub error_code: Option<ErrorCode>,
    pub object_lock_retain_until_date: Option<ObjectLockRetainUntilDate>,
}

#[derive(Debug, Clone)]
pub struct SseKmsEncryptedObjects {
    pub status: Option<SseKmsEncryptedObjectsStatus>,
}

#[derive(Debug, Clone)]
pub struct PutBucketAnalyticsConfigurationRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub id: Option<AnalyticsId>,
    pub analytics_configuration: Option<AnalyticsConfiguration>,
    pub bucket: Option<BucketName>,
}

pub type SkipValidation = bool;

pub type Date = String;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ListBucketAnalyticsConfigurations;
impl Op for ListBucketAnalyticsConfigurations {
    type Input = ListBucketAnalyticsConfigurationsRequest;
    type Output = ListBucketAnalyticsConfigurationsOutput;
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

#[derive(Debug, Clone)]
pub struct PutBucketEncryptionRequest {
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub bucket: Option<BucketName>,
    pub content_md5: Option<ContentMd5>,
    pub server_side_encryption_configuration: Option<ServerSideEncryptionConfiguration>,
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct GetBucketLocation;
impl Op for GetBucketLocation {
    type Input = GetBucketLocationRequest;
    type Output = GetBucketLocationOutput;
}

#[derive(Debug, Clone)]
pub struct ServerSideEncryptionRule {
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    pub apply_server_side_encryption_by_default: Option<ServerSideEncryptionByDefault>,
}

#[derive(Debug, Clone)]
pub struct S3KeyFilter {
    pub filter_rules: Option<FilterRuleList>,
}

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

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct UploadPartCopy;
impl Op for UploadPartCopy {
    type Input = UploadPartCopyRequest;
    type Output = UploadPartCopyOutput;
}

pub type Quiet = bool;

#[derive(Debug, Clone)]
pub struct CreateBucketConfiguration {
    pub location_constraint: Option<BucketLocationConstraint>,
}

#[derive(Debug, Clone)]
pub struct DeletedObject {
    pub delete_marker: Option<DeleteMarker>,
    pub delete_marker_version_id: Option<DeleteMarkerVersionId>,
    pub version_id: Option<ObjectVersionId>,
    pub key: Option<ObjectKey>,
}

pub type ExposeHeader = String;

#[derive(Debug, Clone)]
pub struct InventoryFilter {
    pub prefix: Option<Prefix>,
}

#[derive(Debug, Clone)]
pub struct GetBucketAccelerateConfigurationRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug, Clone)]
pub struct GetObjectTorrentRequest {
    pub key: Option<ObjectKey>,
    pub request_payer: Option<RequestPayer>,
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct DeleteBucket;
impl Op for DeleteBucket {
    type Input = DeleteBucketRequest;
    type Output = ();
}

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

pub type MultipartUploadId = String;

pub type Errors = Vec<Error>;

pub type ContentDisposition = String;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PutBucketAcl;
impl Op for PutBucketAcl {
    type Input = PutBucketAclRequest;
    type Output = ();
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PutBucketIntelligentTieringConfiguration;
impl Op for PutBucketIntelligentTieringConfiguration {
    type Input = PutBucketIntelligentTieringConfigurationRequest;
    type Output = ();
}

#[derive(Debug, Clone)]
pub struct GetBucketAclOutput {
    pub grants: Option<Grants>,
    pub owner: Option<Owner>,
}

#[derive(Debug, Clone)]
pub struct PutBucketAclRequest {
    pub grant_write: Option<GrantWrite>,
    pub content_md5: Option<ContentMd5>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub expected_bucket_owner: Option<AccountId>,
    pub acl: Option<BucketCannedAcl>,
    pub grant_write_acp: Option<GrantWriteAcp>,
    pub bucket: Option<BucketName>,
    pub grant_read: Option<GrantRead>,
    pub grant_read_acp: Option<GrantReadAcp>,
    pub grant_full_control: Option<GrantFullControl>,
    pub access_control_policy: Option<AccessControlPolicy>,
}

#[derive(Debug, Clone)]
pub struct PutObjectRetentionRequest {
    pub bypass_governance_retention: Option<BypassGovernanceRetention>,
    pub content_md5: Option<ContentMd5>,
    pub key: Option<ObjectKey>,
    pub expected_bucket_owner: Option<AccountId>,
    pub version_id: Option<ObjectVersionId>,
    pub retention: Option<ObjectLockRetention>,
    pub request_payer: Option<RequestPayer>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub bucket: Option<BucketName>,
}

pub type MultipartUploadList = Vec<MultipartUpload>;

pub type RecordDelimiter = String;

pub type ExposeHeaders = Vec<ExposeHeader>;

#[derive(Debug, Clone)]
pub struct GetBucketVersioningOutput {
    pub mfa_delete: Option<MfaDeleteStatus>,
    pub status: Option<BucketVersioningStatus>,
}

pub type ResponseExpires = String;

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

pub type TargetGrants = Vec<TargetGrant>;

#[derive(Debug, Clone)]
pub struct PutBucketPolicyRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub policy: Option<Policy>,
    pub confirm_remove_self_bucket_access: Option<ConfirmRemoveSelfBucketAccess>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub bucket: Option<BucketName>,
    pub content_md5: Option<ContentMd5>,
}

pub type AccessPointArn = String;

#[derive(Debug, Clone)]
pub struct NotFound {}

#[derive(Debug, Clone)]
pub struct Sses3 {}

pub type CopySourceSseCustomerKeyMd5 = String;

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

#[derive(Debug, Clone)]
pub struct CreateBucketRequest {
    pub acl: Option<BucketCannedAcl>,
    pub object_ownership: Option<ObjectOwnership>,
    pub grant_full_control: Option<GrantFullControl>,
    pub object_lock_enabled_for_bucket: Option<ObjectLockEnabledForBucket>,
    pub grant_write: Option<GrantWrite>,
    pub bucket: Option<BucketName>,
    pub grant_read_acp: Option<GrantReadAcp>,
    pub create_bucket_configuration: Option<CreateBucketConfiguration>,
    pub grant_read: Option<GrantRead>,
    pub grant_write_acp: Option<GrantWriteAcp>,
}

#[derive(Debug, Clone)]
pub struct GetObjectAclRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub request_payer: Option<RequestPayer>,
    pub key: Option<ObjectKey>,
    pub version_id: Option<ObjectVersionId>,
    pub bucket: Option<BucketName>,
}

#[derive(Debug, Clone)]
pub struct OutputSerialization {
    pub csv: Option<CsvOutput>,
    pub json: Option<JsonOutput>,
}

#[derive(Debug, Clone)]
pub struct GetBucketOwnershipControlsRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}

pub type RequestRoute = String;

pub type ReplaceKeyPrefixWith = String;

#[derive(Debug, Clone)]
pub struct LifecycleRuleAndOperator {
    pub object_size_less_than: Option<ObjectSizeLessThanBytes>,
    pub prefix: Option<Prefix>,
    pub tags: Option<TagSet>,
    pub object_size_greater_than: Option<ObjectSizeGreaterThanBytes>,
}

#[derive(Debug, Clone)]
pub struct StorageClassAnalysisDataExport {
    pub destination: Option<AnalyticsExportDestination>,
    pub output_schema_version: Option<StorageClassAnalysisSchemaVersion>,
}

#[derive(Debug, Clone)]
pub struct S3Location {
    pub encryption: Option<Encryption>,
    pub canned_acl: Option<ObjectCannedAcl>,
    pub bucket_name: Option<BucketName>,
    pub prefix: Option<LocationPrefix>,
    pub access_control_list: Option<Grants>,
    pub storage_class: Option<StorageClass>,
    pub user_metadata: Option<UserMetadata>,
    pub tagging: Option<Tagging>,
}

#[derive(Debug, Clone)]
pub struct DeleteBucketRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug, Clone)]
pub struct CopyObjectOutput {
    pub copy_source_version_id: Option<CopySourceVersionId>,
    pub expiration: Option<Expiration>,
    pub request_charged: Option<RequestCharged>,
    pub server_side_encryption: Option<ServerSideEncryption>,
    pub copy_object_result: Option<CopyObjectResult>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub ssekms_encryption_context: Option<SsekmsEncryptionContext>,
    pub ssekms_key_id: Option<SsekmsKeyId>,
    pub version_id: Option<ObjectVersionId>,
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
}

#[derive(Debug, Clone)]
pub struct GetBucketIntelligentTieringConfigurationRequest {
    pub bucket: Option<BucketName>,
    pub id: Option<IntelligentTieringId>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct CreateMultipartUpload;
impl Op for CreateMultipartUpload {
    type Input = CreateMultipartUploadRequest;
    type Output = CreateMultipartUploadOutput;
}

#[derive(Debug, Clone)]
pub struct NoSuchUpload {}

pub type PartNumberMarker = String;

#[derive(Debug, Clone)]
pub struct DeleteBucketOwnershipControlsRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}

pub type IntelligentTieringId = String;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct GetBucketAcl;
impl Op for GetBucketAcl {
    type Input = GetBucketAclRequest;
    type Output = GetBucketAclOutput;
}

pub type MetricsConfigurationList = Vec<MetricsConfiguration>;

pub type InventoryId = String;

#[derive(Debug, Clone)]
pub struct MetadataEntry {
    pub name: Option<MetadataKey>,
    pub value: Option<MetadataValue>,
}

pub type ReplicaKmsKeyId = String;

pub type GrantReadAcp = String;

pub type Mfa = String;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct DeleteBucketWebsite;
impl Op for DeleteBucketWebsite {
    type Input = DeleteBucketWebsiteRequest;
    type Output = ();
}

#[derive(Debug, Clone)]
pub struct ReplicationRuleAndOperator {
    pub tags: Option<TagSet>,
    pub prefix: Option<Prefix>,
}

pub type Buckets = Vec<Bucket>;

#[derive(Debug, Clone)]
pub struct RestoreRequest {
    pub glacier_job_parameters: Option<GlacierJobParameters>,
    pub r#type: Option<RestoreRequestType>,
    pub days: Option<Days>,
    pub description: Option<Description>,
    pub output_location: Option<OutputLocation>,
    pub tier: Option<Tier>,
    pub select_parameters: Option<SelectParameters>,
}

#[derive(Debug, Clone)]
pub struct GetBucketNotificationConfigurationRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}

pub type SsekmsKeyId = String;

#[derive(Debug, Clone)]
pub struct IntelligentTieringAndOperator {
    pub prefix: Option<Prefix>,
    pub tags: Option<TagSet>,
}

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

pub type SseCustomerKey = String;

#[derive(Debug, Clone)]
pub struct ObjectIdentifier {
    pub key: Option<ObjectKey>,
    pub version_id: Option<ObjectVersionId>,
}

pub type CopySourceSseCustomerAlgorithm = String;

pub type FilterRuleValue = String;

#[derive(Debug, Clone)]
pub struct GetObjectLegalHoldRequest {
    pub key: Option<ObjectKey>,
    pub request_payer: Option<RequestPayer>,
    pub version_id: Option<ObjectVersionId>,
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}

#[derive(Debug, Clone)]
pub struct PutBucketCorsRequest {
    pub cors_configuration: Option<CorsConfiguration>,
    pub bucket: Option<BucketName>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub expected_bucket_owner: Option<AccountId>,
    pub content_md5: Option<ContentMd5>,
}

pub type EmailAddress = String;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct DeletePublicAccessBlock;
impl Op for DeletePublicAccessBlock {
    type Input = DeletePublicAccessBlockRequest;
    type Output = ();
}

#[derive(Debug, Clone)]
pub struct HeadObjectOutput {
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
    pub metadata: Option<Metadata>,
    pub cache_control: Option<CacheControl>,
    pub storage_class: Option<StorageClass>,
    pub content_disposition: Option<ContentDisposition>,
    pub object_lock_retain_until_date: Option<ObjectLockRetainUntilDate>,
    pub accept_ranges: Option<AcceptRanges>,
    pub restore: Option<Restore>,
    pub content_language: Option<ContentLanguage>,
    pub website_redirect_location: Option<WebsiteRedirectLocation>,
    pub version_id: Option<ObjectVersionId>,
    pub delete_marker: Option<DeleteMarker>,
    pub object_lock_legal_hold_status: Option<ObjectLockLegalHoldStatus>,
    pub checksum_crc32: Option<ChecksumCrc32>,
    pub expires: Option<Expires>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub content_type: Option<ContentType>,
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    pub replication_status: Option<ReplicationStatus>,
    pub object_lock_mode: Option<ObjectLockMode>,
    pub request_charged: Option<RequestCharged>,
    pub archive_status: Option<ArchiveStatus>,
    pub content_encoding: Option<ContentEncoding>,
    pub parts_count: Option<PartsCount>,
    pub ssekms_key_id: Option<SsekmsKeyId>,
    pub checksum_sha256: Option<ChecksumSha256>,
    pub last_modified: Option<LastModified>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub server_side_encryption: Option<ServerSideEncryption>,
    pub missing_meta: Option<MissingMeta>,
    pub expiration: Option<Expiration>,
    pub e_tag: Option<ETag>,
    pub checksum_sha1: Option<ChecksumSha1>,
    pub content_length: Option<ContentLength>,
}

#[derive(Debug, Clone)]
pub struct DeleteBucketTaggingRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}

pub type NextMarker = String;

pub type CacheControl = String;

#[derive(Debug, Clone)]
pub struct GetBucketLocationOutput {
    pub location_constraint: Option<BucketLocationConstraint>,
}

pub type LocationPrefix = String;

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

#[derive(Debug, Clone)]
pub struct ObjectLockRetention {
    pub mode: Option<ObjectLockRetentionMode>,
    pub retain_until_date: Option<Date>,
}

#[derive(Debug, Clone)]
pub struct ObjectLockRule {
    pub default_retention: Option<DefaultRetention>,
}

#[derive(Debug, Clone)]
pub struct AbortMultipartUploadRequest {
    pub upload_id: Option<MultipartUploadId>,
    pub bucket: Option<BucketName>,
    pub key: Option<ObjectKey>,
    pub request_payer: Option<RequestPayer>,
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug, Clone)]
pub struct ListBucketIntelligentTieringConfigurationsRequest {
    pub continuation_token: Option<Token>,
    pub bucket: Option<BucketName>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct GetBucketPolicyStatus;
impl Op for GetBucketPolicyStatus {
    type Input = GetBucketPolicyStatusRequest;
    type Output = GetBucketPolicyStatusOutput;
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ListObjectVersions;
impl Op for ListObjectVersions {
    type Input = ListObjectVersionsRequest;
    type Output = ListObjectVersionsOutput;
}

pub type Priority = i32;

#[derive(Debug, Clone)]
pub struct PutBucketOwnershipControlsRequest {
    pub ownership_controls: Option<OwnershipControls>,
    pub bucket: Option<BucketName>,
    pub content_md5: Option<ContentMd5>,
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug, Clone)]
pub struct RecordsEvent {
    pub payload: Option<Body>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct SelectObjectContent;
impl Op for SelectObjectContent {
    type Input = SelectObjectContentRequest;
    type Output = SelectObjectContentOutput;
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PutBucketAnalyticsConfiguration;
impl Op for PutBucketAnalyticsConfiguration {
    type Input = PutBucketAnalyticsConfigurationRequest;
    type Output = ();
}

pub type ObjectIdentifierList = Vec<ObjectIdentifier>;

pub type Range = String;

pub type CopySourceIfNoneMatch = String;

#[derive(Debug, Clone)]
pub struct Error {
    pub version_id: Option<ObjectVersionId>,
    pub code: Option<Code>,
    pub key: Option<ObjectKey>,
    pub message: Option<Message>,
}

#[derive(Debug, Clone)]
pub struct GetBucketPolicyOutput {
    pub policy: Option<Policy>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct GetBucketIntelligentTieringConfiguration;
impl Op for GetBucketIntelligentTieringConfiguration {
    type Input = GetBucketIntelligentTieringConfigurationRequest;
    type Output = GetBucketIntelligentTieringConfigurationOutput;
}

#[derive(Debug, Clone)]
pub struct PublicAccessBlockConfiguration {
    pub ignore_public_acls: Option<Setting>,
    pub block_public_policy: Option<Setting>,
    pub restrict_public_buckets: Option<Setting>,
    pub block_public_acls: Option<Setting>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PutBucketAccelerateConfiguration;
impl Op for PutBucketAccelerateConfiguration {
    type Input = PutBucketAccelerateConfigurationRequest;
    type Output = ();
}

pub type IsTruncated = bool;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct GetBucketEncryption;
impl Op for GetBucketEncryption {
    type Input = GetBucketEncryptionRequest;
    type Output = GetBucketEncryptionOutput;
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

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct DeleteBucketCors;
impl Op for DeleteBucketCors {
    type Input = DeleteBucketCorsRequest;
    type Output = ();
}

pub type IntelligentTieringDays = i32;

#[derive(Debug, Clone)]
pub struct NoSuchBucket {}

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

pub type PartsList = Vec<ObjectPart>;

pub type ResponseContentEncoding = String;

#[derive(Debug, Clone)]
pub struct VersioningConfiguration {
    pub status: Option<BucketVersioningStatus>,
    pub mfa_delete: Option<MfaDelete>,
}

#[derive(Debug, Clone)]
pub struct PutObjectTaggingRequest {
    pub bucket: Option<BucketName>,
    pub key: Option<ObjectKey>,
    pub content_md5: Option<ContentMd5>,
    pub tagging: Option<Tagging>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub version_id: Option<ObjectVersionId>,
    pub expected_bucket_owner: Option<AccountId>,
    pub request_payer: Option<RequestPayer>,
}

pub type ObjectSize = i64;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct DeleteBucketInventoryConfiguration;
impl Op for DeleteBucketInventoryConfiguration {
    type Input = DeleteBucketInventoryConfigurationRequest;
    type Output = ();
}

#[derive(Debug, Clone)]
pub struct GetBucketReplicationRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}

#[derive(Debug, Clone)]
pub struct CreateMultipartUploadRequest {
    pub server_side_encryption: Option<ServerSideEncryption>,
    pub tagging: Option<TaggingHeader>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub sse_customer_key: Option<SseCustomerKey>,
    pub acl: Option<ObjectCannedAcl>,
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    pub content_encoding: Option<ContentEncoding>,
    pub content_language: Option<ContentLanguage>,
    pub grant_read: Option<GrantRead>,
    pub object_lock_retain_until_date: Option<ObjectLockRetainUntilDate>,
    pub metadata: Option<Metadata>,
    pub storage_class: Option<StorageClass>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub bucket: Option<BucketName>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub cache_control: Option<CacheControl>,
    pub ssekms_encryption_context: Option<SsekmsEncryptionContext>,
    pub ssekms_key_id: Option<SsekmsKeyId>,
    pub key: Option<ObjectKey>,
    pub content_disposition: Option<ContentDisposition>,
    pub object_lock_legal_hold_status: Option<ObjectLockLegalHoldStatus>,
    pub request_payer: Option<RequestPayer>,
    pub content_type: Option<ContentType>,
    pub grant_read_acp: Option<GrantReadAcp>,
    pub expected_bucket_owner: Option<AccountId>,
    pub grant_write_acp: Option<GrantWriteAcp>,
    pub grant_full_control: Option<GrantFullControl>,
    pub expires: Option<Expires>,
    pub object_lock_mode: Option<ObjectLockMode>,
    pub website_redirect_location: Option<WebsiteRedirectLocation>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct HeadObject;
impl Op for HeadObject {
    type Input = HeadObjectRequest;
    type Output = HeadObjectOutput;
}

pub type PartsCount = i32;

#[derive(Debug, Clone)]
pub struct Delete {
    pub quiet: Option<Quiet>,
    pub objects: Option<ObjectIdentifierList>,
}

#[derive(Debug, Clone)]
pub struct UploadPartCopyRequest {
    pub bucket: Option<BucketName>,
    pub copy_source_if_unmodified_since: Option<CopySourceIfUnmodifiedSince>,
    pub expected_bucket_owner: Option<AccountId>,
    pub key: Option<ObjectKey>,
    pub copy_source_if_match: Option<CopySourceIfMatch>,
    pub sse_customer_key: Option<SseCustomerKey>,
    pub copy_source_sse_customer_algorithm: Option<CopySourceSseCustomerAlgorithm>,
    pub copy_source: Option<CopySource>,
    pub copy_source_sse_customer_key: Option<CopySourceSseCustomerKey>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub copy_source_range: Option<CopySourceRange>,
    pub part_number: Option<PartNumber>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub upload_id: Option<MultipartUploadId>,
    pub expected_source_bucket_owner: Option<AccountId>,
    pub request_payer: Option<RequestPayer>,
    pub copy_source_if_none_match: Option<CopySourceIfNoneMatch>,
    pub copy_source_if_modified_since: Option<CopySourceIfModifiedSince>,
    pub copy_source_sse_customer_key_md5: Option<CopySourceSseCustomerKeyMd5>,
}

#[derive(Debug, Clone)]
pub struct PutObjectLegalHoldOutput {
    pub request_charged: Option<RequestCharged>,
}

pub type GrantRead = String;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct GetObjectRetention;
impl Op for GetObjectRetention {
    type Input = GetObjectRetentionRequest;
    type Output = GetObjectRetentionOutput;
}

#[derive(Debug, Clone)]
pub struct GetObjectRetentionOutput {
    pub retention: Option<ObjectLockRetention>,
}

#[derive(Debug, Clone)]
pub struct CompletedMultipartUpload {
    pub parts: Option<CompletedPartList>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct DeleteBucketLifecycle;
impl Op for DeleteBucketLifecycle {
    type Input = DeleteBucketLifecycleRequest;
    type Output = ();
}

#[derive(Debug, Clone)]
pub struct ServerSideEncryptionConfiguration {
    pub rules: Option<ServerSideEncryptionRules>,
}

#[derive(Debug, Clone)]
pub struct Tagging {
    pub tag_set: Option<TagSet>,
}

pub type ExpiredObjectDeleteMarker = bool;

#[derive(Debug, Clone)]
pub struct CommonPrefix {
    pub prefix: Option<Prefix>,
}

pub type KeyMarker = String;

#[derive(Debug, Clone)]
pub struct PutObjectRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub ssekms_key_id: Option<SsekmsKeyId>,
    pub bucket: Option<BucketName>,
    pub sse_customer_key: Option<SseCustomerKey>,
    pub grant_write_acp: Option<GrantWriteAcp>,
    pub checksum_sha1: Option<ChecksumSha1>,
    pub content_md5: Option<ContentMd5>,
    pub server_side_encryption: Option<ServerSideEncryption>,
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    pub object_lock_retain_until_date: Option<ObjectLockRetainUntilDate>,
    pub request_payer: Option<RequestPayer>,
    pub ssekms_encryption_context: Option<SsekmsEncryptionContext>,
    pub metadata: Option<Metadata>,
    pub checksum_sha256: Option<ChecksumSha256>,
    pub object_lock_legal_hold_status: Option<ObjectLockLegalHoldStatus>,
    pub content_encoding: Option<ContentEncoding>,
    pub grant_read_acp: Option<GrantReadAcp>,
    pub grant_full_control: Option<GrantFullControl>,
    pub storage_class: Option<StorageClass>,
    pub tagging: Option<TaggingHeader>,
    pub website_redirect_location: Option<WebsiteRedirectLocation>,
    pub content_disposition: Option<ContentDisposition>,
    pub acl: Option<ObjectCannedAcl>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub content_length: Option<ContentLength>,
    pub checksum_crc32: Option<ChecksumCrc32>,
    pub body: Option<StreamingBlob>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub cache_control: Option<CacheControl>,
    pub expires: Option<Expires>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub content_type: Option<ContentType>,
    pub key: Option<ObjectKey>,
    pub content_language: Option<ContentLanguage>,
    pub grant_read: Option<GrantRead>,
    pub object_lock_mode: Option<ObjectLockMode>,
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
}

pub type Marker = String;

#[derive(Debug, Clone)]
pub struct AccelerateConfiguration {
    pub status: Option<BucketAccelerateStatus>,
}

#[derive(Debug, Clone)]
pub struct GetBucketWebsiteRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}

pub type Grants = Vec<Grant>;

#[derive(Debug, Clone)]
pub struct DeleteObjectsRequest {
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub mfa: Option<Mfa>,
    pub request_payer: Option<RequestPayer>,
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
    pub bypass_governance_retention: Option<BypassGovernanceRetention>,
    pub delete: Option<Delete>,
}

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

#[derive(Debug, Clone)]
pub struct GetBucketReplicationOutput {
    pub replication_configuration: Option<ReplicationConfiguration>,
}

pub type BytesProcessed = i64;

#[derive(Debug, Clone)]
pub struct GetObjectLockConfigurationRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug, Clone)]
pub struct GetBucketAnalyticsConfigurationRequest {
    pub id: Option<AnalyticsId>,
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct GetBucketRequestPayment;
impl Op for GetBucketRequestPayment {
    type Input = GetBucketRequestPaymentRequest;
    type Output = GetBucketRequestPaymentOutput;
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

pub type CommonPrefixList = Vec<CommonPrefix>;

pub type BucketName = String;

#[derive(Debug, Clone)]
pub struct RequestPaymentConfiguration {
    pub payer: Option<Payer>,
}

#[derive(Debug, Clone)]
pub struct PutObjectAclRequest {
    pub version_id: Option<ObjectVersionId>,
    pub access_control_policy: Option<AccessControlPolicy>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub expected_bucket_owner: Option<AccountId>,
    pub grant_read_acp: Option<GrantReadAcp>,
    pub acl: Option<ObjectCannedAcl>,
    pub grant_write: Option<GrantWrite>,
    pub grant_write_acp: Option<GrantWriteAcp>,
    pub bucket: Option<BucketName>,
    pub content_md5: Option<ContentMd5>,
    pub key: Option<ObjectKey>,
    pub request_payer: Option<RequestPayer>,
    pub grant_full_control: Option<GrantFullControl>,
    pub grant_read: Option<GrantRead>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct DeleteBucketTagging;
impl Op for DeleteBucketTagging {
    type Input = DeleteBucketTaggingRequest;
    type Output = ();
}

pub type Token = String;

pub type CopySourceIfMatch = String;

pub type AbortDate = String;

pub type ContentRange = String;

pub type Metadata = HashMap<MetadataKey, MetadataValue>;

#[derive(Debug, Clone)]
pub struct PutBucketLoggingRequest {
    pub bucket_logging_status: Option<BucketLoggingStatus>,
    pub bucket: Option<BucketName>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub content_md5: Option<ContentMd5>,
    pub expected_bucket_owner: Option<AccountId>,
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

#[derive(Debug, Clone)]
pub struct CompletedPart {
    pub checksum_sha1: Option<ChecksumSha1>,
    pub checksum_sha256: Option<ChecksumSha256>,
    pub e_tag: Option<ETag>,
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
    pub part_number: Option<PartNumber>,
    pub checksum_crc32: Option<ChecksumCrc32>,
}

pub type AllowedMethods = Vec<AllowedMethod>;

#[derive(Debug, Clone)]
pub struct GetBucketInventoryConfigurationRequest {
    pub id: Option<InventoryId>,
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}

pub type DaysAfterInitiation = i32;

#[derive(Debug, Clone)]
pub struct GetBucketTaggingOutput {
    pub tag_set: Option<TagSet>,
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

#[derive(Debug, Clone)]
pub struct DeleteBucketMetricsConfigurationRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
    pub id: Option<MetricsId>,
}

#[derive(Debug, Clone)]
pub struct DeleteObjectTaggingRequest {
    pub bucket: Option<BucketName>,
    pub key: Option<ObjectKey>,
    pub expected_bucket_owner: Option<AccountId>,
    pub version_id: Option<ObjectVersionId>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct GetBucketAnalyticsConfiguration;
impl Op for GetBucketAnalyticsConfiguration {
    type Input = GetBucketAnalyticsConfigurationRequest;
    type Output = GetBucketAnalyticsConfigurationOutput;
}

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

#[derive(Debug, Clone)]
pub struct Checksum {
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
    pub checksum_sha1: Option<ChecksumSha1>,
    pub checksum_sha256: Option<ChecksumSha256>,
    pub checksum_crc32: Option<ChecksumCrc32>,
}

pub type ChecksumAlgorithmList = Vec<ChecksumAlgorithm>;

pub type ObjectKey = String;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PutObjectLegalHold;
impl Op for PutObjectLegalHold {
    type Input = PutObjectLegalHoldRequest;
    type Output = PutObjectLegalHoldOutput;
}

#[derive(Debug, Clone)]
pub struct ListPartsOutput {
    pub initiator: Option<Initiator>,
    pub request_charged: Option<RequestCharged>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub owner: Option<Owner>,
    pub bucket: Option<BucketName>,
    pub is_truncated: Option<IsTruncated>,
    pub upload_id: Option<MultipartUploadId>,
    pub part_number_marker: Option<PartNumberMarker>,
    pub abort_rule_id: Option<AbortRuleId>,
    pub abort_date: Option<AbortDate>,
    pub max_parts: Option<MaxParts>,
    pub parts: Option<Parts>,
    pub storage_class: Option<StorageClass>,
    pub next_part_number_marker: Option<NextPartNumberMarker>,
    pub key: Option<ObjectKey>,
}

pub type RequestToken = String;

pub type TieringList = Vec<Tiering>;

#[derive(Debug, Clone)]
pub struct GetPublicAccessBlockOutput {
    pub public_access_block_configuration: Option<PublicAccessBlockConfiguration>,
}

#[derive(Debug, Clone)]
pub struct DeleteObjectOutput {
    pub version_id: Option<ObjectVersionId>,
    pub delete_marker: Option<DeleteMarker>,
    pub request_charged: Option<RequestCharged>,
}

#[derive(Debug, Clone)]
pub struct PolicyStatus {
    pub is_public: Option<IsPublic>,
}

pub type PartNumber = i32;

pub type LastModified = String;

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

#[derive(Debug, Clone)]
pub struct NoncurrentVersionTransition {
    pub newer_noncurrent_versions: Option<VersionCount>,
    pub storage_class: Option<TransitionStorageClass>,
    pub noncurrent_days: Option<Days>,
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

pub type SseCustomerAlgorithm = String;

#[derive(Debug, Clone)]
pub struct GetBucketPolicyRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}

pub type AllowedOrigins = Vec<AllowedOrigin>;

#[derive(Debug, Clone)]
pub struct LifecycleExpiration {
    pub expired_object_delete_marker: Option<ExpiredObjectDeleteMarker>,
    pub days: Option<Days>,
    pub date: Option<Date>,
}

pub type TopicArn = String;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct GetBucketOwnershipControls;
impl Op for GetBucketOwnershipControls {
    type Input = GetBucketOwnershipControlsRequest;
    type Output = GetBucketOwnershipControlsOutput;
}

#[derive(Debug, Clone)]
pub struct DeleteBucketInventoryConfigurationRequest {
    pub bucket: Option<BucketName>,
    pub id: Option<InventoryId>,
    pub expected_bucket_owner: Option<AccountId>,
}

pub type CopySourceIfModifiedSince = String;

#[derive(Debug, Clone)]
pub struct EndEvent {}

pub type ContentEncoding = String;

pub type Prefix = String;

pub type ObjectLockRetainUntilDate = String;

#[derive(Debug, Clone)]
pub enum SelectObjectContentEventStream {
    Cont(ContinuationEvent),
    Progress(ProgressEvent),
    End(EndEvent),
    Records(RecordsEvent),
    Stats(StatsEvent),
}

#[derive(Debug, Clone)]
pub struct FilterRule {
    pub value: Option<FilterRuleValue>,
    pub name: Option<FilterRuleName>,
}

pub type IfUnmodifiedSince = String;

#[derive(Debug, Clone)]
pub struct GetObjectRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub range: Option<Range>,
    pub checksum_mode: Option<ChecksumMode>,
    pub request_payer: Option<RequestPayer>,
    pub response_content_type: Option<ResponseContentType>,
    pub bucket: Option<BucketName>,
    pub if_modified_since: Option<IfModifiedSince>,
    pub if_none_match: Option<IfNoneMatch>,
    pub key: Option<ObjectKey>,
    pub part_number: Option<PartNumber>,
    pub if_match: Option<IfMatch>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub response_cache_control: Option<ResponseCacheControl>,
    pub response_content_language: Option<ResponseContentLanguage>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub version_id: Option<ObjectVersionId>,
    pub if_unmodified_since: Option<IfUnmodifiedSince>,
    pub sse_customer_key: Option<SseCustomerKey>,
    pub response_expires: Option<ResponseExpires>,
    pub response_content_encoding: Option<ResponseContentEncoding>,
    pub response_content_disposition: Option<ResponseContentDisposition>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct GetObjectLockConfiguration;
impl Op for GetObjectLockConfiguration {
    type Input = GetObjectLockConfigurationRequest;
    type Output = GetObjectLockConfigurationOutput;
}

#[derive(Debug, Clone)]
pub struct JsonInput {
    pub r#type: Option<JsonType>,
}

pub type KmsContext = String;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct GetBucketWebsite;
impl Op for GetBucketWebsite {
    type Input = GetBucketWebsiteRequest;
    type Output = GetBucketWebsiteOutput;
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct GetObjectAcl;
impl Op for GetObjectAcl {
    type Input = GetObjectAclRequest;
    type Output = GetObjectAclOutput;
}

pub type AllowedOrigin = String;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ListObjectsV2;
impl Op for ListObjectsV2 {
    type Input = ListObjectsV2Request;
    type Output = ListObjectsV2Output;
}

pub type HttpErrorCodeReturnedEquals = String;

#[derive(Debug, Clone)]
pub struct Redirect {
    pub host_name: Option<HostName>,
    pub protocol: Option<Protocol>,
    pub http_redirect_code: Option<HttpRedirectCode>,
    pub replace_key_with: Option<ReplaceKeyWith>,
    pub replace_key_prefix_with: Option<ReplaceKeyPrefixWith>,
}

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

#[derive(Debug, Clone)]
pub struct PutBucketWebsiteRequest {
    pub content_md5: Option<ContentMd5>,
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub website_configuration: Option<WebsiteConfiguration>,
}

#[derive(Debug, Clone)]
pub struct StatsEvent {
    pub details: Option<Stats>,
}

#[derive(Debug, Clone)]
pub struct DeleteBucketReplicationRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}

#[derive(Debug, Clone)]
pub struct SelectObjectContentOutput {
    pub payload: Option<SelectObjectContentEventStream>,
}

pub type TransitionList = Vec<Transition>;

#[derive(Debug, Clone)]
pub struct InventoryDestination {
    pub s3_bucket_destination: Option<InventoryS3BucketDestination>,
}

#[derive(Debug, Clone)]
pub struct Progress {
    pub bytes_processed: Option<BytesProcessed>,
    pub bytes_scanned: Option<BytesScanned>,
    pub bytes_returned: Option<BytesReturned>,
}

#[derive(Debug, Clone)]
pub struct GetObjectTaggingOutput {
    pub version_id: Option<ObjectVersionId>,
    pub tag_set: Option<TagSet>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct DeleteObjectTagging;
impl Op for DeleteObjectTagging {
    type Input = DeleteObjectTaggingRequest;
    type Output = DeleteObjectTaggingOutput;
}

pub type Uri = String;

#[derive(Debug, Clone)]
pub struct Object {
    pub owner: Option<Owner>,
    pub size: Option<Size>,
    pub storage_class: Option<ObjectStorageClass>,
    pub last_modified: Option<LastModified>,
    pub e_tag: Option<ETag>,
    pub checksum_algorithm: Option<ChecksumAlgorithmList>,
    pub key: Option<ObjectKey>,
}

#[derive(Debug, Clone)]
pub struct TargetGrant {
    pub grantee: Option<Grantee>,
    pub permission: Option<BucketLogsPermission>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct DeleteBucketAnalyticsConfiguration;
impl Op for DeleteBucketAnalyticsConfiguration {
    type Input = DeleteBucketAnalyticsConfigurationRequest;
    type Output = ();
}

pub type TargetPrefix = String;

pub type EnableRequestProgress = bool;

pub type HttpRedirectCode = String;

pub type DisplayName = String;

#[derive(Debug, Clone)]
pub struct GetBucketInventoryConfigurationOutput {
    pub inventory_configuration: Option<InventoryConfiguration>,
}

pub type FieldDelimiter = String;

#[derive(Debug, Clone)]
pub struct GetBucketMetricsConfigurationRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
    pub id: Option<MetricsId>,
}

pub type Days = i32;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PutBucketPolicy;
impl Op for PutBucketPolicy {
    type Input = PutBucketPolicyRequest;
    type Output = ();
}

#[derive(Debug, Clone)]
pub struct UploadPartOutput {
    pub server_side_encryption: Option<ServerSideEncryption>,
    pub checksum_sha1: Option<ChecksumSha1>,
    pub ssekms_key_id: Option<SsekmsKeyId>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub request_charged: Option<RequestCharged>,
    pub e_tag: Option<ETag>,
    pub checksum_crc32: Option<ChecksumCrc32>,
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
    pub checksum_sha256: Option<ChecksumSha256>,
}

#[derive(Debug, Clone)]
pub struct ListObjectsOutput {
    pub encoding_type: Option<EncodingType>,
    pub common_prefixes: Option<CommonPrefixList>,
    pub prefix: Option<Prefix>,
    pub marker: Option<Marker>,
    pub name: Option<BucketName>,
    pub contents: Option<ObjectList>,
    pub is_truncated: Option<IsTruncated>,
    pub max_keys: Option<MaxKeys>,
    pub delimiter: Option<Delimiter>,
    pub next_marker: Option<NextMarker>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PutBucketLogging;
impl Op for PutBucketLogging {
    type Input = PutBucketLoggingRequest;
    type Output = ();
}

pub type ReplaceKeyWith = String;

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

#[derive(Debug, Clone)]
pub struct LoggingEnabled {
    pub target_prefix: Option<TargetPrefix>,
    pub target_bucket: Option<TargetBucket>,
    pub target_grants: Option<TargetGrants>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PutBucketReplication;
impl Op for PutBucketReplication {
    type Input = PutBucketReplicationRequest;
    type Output = ();
}

#[derive(Debug, Clone)]
pub struct ErrorDocument {
    pub key: Option<ObjectKey>,
}

pub type MetricsId = String;

#[derive(Debug, Clone)]
pub struct PutBucketRequestPaymentRequest {
    pub bucket: Option<BucketName>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub request_payment_configuration: Option<RequestPaymentConfiguration>,
    pub expected_bucket_owner: Option<AccountId>,
    pub content_md5: Option<ContentMd5>,
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

#[derive(Debug, Clone)]
pub struct RedirectAllRequestsTo {
    pub host_name: Option<HostName>,
    pub protocol: Option<Protocol>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct GetBucketLogging;
impl Op for GetBucketLogging {
    type Input = GetBucketLoggingRequest;
    type Output = GetBucketLoggingOutput;
}

#[derive(Debug, Clone)]
pub struct SourceSelectionCriteria {
    pub replica_modifications: Option<ReplicaModifications>,
    pub sse_kms_encrypted_objects: Option<SseKmsEncryptedObjects>,
}

#[derive(Debug, Clone)]
pub struct GetObjectAttributesRequest {
    pub request_payer: Option<RequestPayer>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub expected_bucket_owner: Option<AccountId>,
    pub max_parts: Option<MaxParts>,
    pub sse_customer_key: Option<SseCustomerKey>,
    pub object_attributes: Option<ObjectAttributesList>,
    pub bucket: Option<BucketName>,
    pub key: Option<ObjectKey>,
    pub part_number_marker: Option<PartNumberMarker>,
    pub version_id: Option<ObjectVersionId>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
}

#[derive(Debug, Clone)]
pub struct GetObjectTaggingRequest {
    pub version_id: Option<ObjectVersionId>,
    pub key: Option<ObjectKey>,
    pub bucket: Option<BucketName>,
    pub request_payer: Option<RequestPayer>,
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug, Clone)]
pub struct AnalyticsS3BucketDestination {
    pub bucket_account_id: Option<AccountId>,
    pub prefix: Option<Prefix>,
    pub bucket: Option<BucketName>,
    pub format: Option<AnalyticsS3ExportFileFormat>,
}

pub type ContentType = String;

#[derive(Debug, Clone)]
pub struct GetPublicAccessBlockRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}

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

#[derive(Debug, Clone)]
pub struct PutBucketMetricsConfigurationRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub id: Option<MetricsId>,
    pub metrics_configuration: Option<MetricsConfiguration>,
    pub bucket: Option<BucketName>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PutBucketOwnershipControls;
impl Op for PutBucketOwnershipControls {
    type Input = PutBucketOwnershipControlsRequest;
    type Output = ();
}

#[derive(Debug, Clone)]
pub struct GetObjectOutput {
    pub metadata: Option<Metadata>,
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
    pub ssekms_key_id: Option<SsekmsKeyId>,
    pub accept_ranges: Option<AcceptRanges>,
    pub website_redirect_location: Option<WebsiteRedirectLocation>,
    pub cache_control: Option<CacheControl>,
    pub object_lock_mode: Option<ObjectLockMode>,
    pub expires: Option<Expires>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub request_charged: Option<RequestCharged>,
    pub replication_status: Option<ReplicationStatus>,
    pub delete_marker: Option<DeleteMarker>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub server_side_encryption: Option<ServerSideEncryption>,
    pub object_lock_retain_until_date: Option<ObjectLockRetainUntilDate>,
    pub storage_class: Option<StorageClass>,
    pub checksum_crc32: Option<ChecksumCrc32>,
    pub checksum_sha1: Option<ChecksumSha1>,
    pub content_range: Option<ContentRange>,
    pub expiration: Option<Expiration>,
    pub last_modified: Option<LastModified>,
    pub checksum_sha256: Option<ChecksumSha256>,
    pub e_tag: Option<ETag>,
    pub missing_meta: Option<MissingMeta>,
    pub tag_count: Option<TagCount>,
    pub version_id: Option<ObjectVersionId>,
    pub parts_count: Option<PartsCount>,
    pub content_type: Option<ContentType>,
    pub content_disposition: Option<ContentDisposition>,
    pub object_lock_legal_hold_status: Option<ObjectLockLegalHoldStatus>,
    pub content_encoding: Option<ContentEncoding>,
    pub body: Option<StreamingBlob>,
    pub restore: Option<Restore>,
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    pub content_language: Option<ContentLanguage>,
    pub content_length: Option<ContentLength>,
}

pub type ReplicationRules = Vec<ReplicationRule>;

pub type UploadIdMarker = String;

pub type NextVersionIdMarker = String;

pub type ChecksumCrc32 = String;

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

pub type Years = i32;

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

pub type MetadataKey = String;

pub type ETag = String;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ListParts;
impl Op for ListParts {
    type Input = ListPartsRequest;
    type Output = ListPartsOutput;
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

pub type ResponseCacheControl = String;

#[derive(Debug, Clone)]
pub struct AccessControlPolicy {
    pub owner: Option<Owner>,
    pub grants: Option<Grants>,
}

pub type VersionCount = i32;

pub type End = i64;

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

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ListBuckets;
impl Op for ListBuckets {
    type Input = ();
    type Output = ListBucketsOutput;
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct DeleteBucketPolicy;
impl Op for DeleteBucketPolicy {
    type Input = DeleteBucketPolicyRequest;
    type Output = ();
}

#[derive(Debug, Clone)]
pub struct ListBucketMetricsConfigurationsOutput {
    pub continuation_token: Option<Token>,
    pub metrics_configuration_list: Option<MetricsConfigurationList>,
    pub is_truncated: Option<IsTruncated>,
    pub next_continuation_token: Option<NextToken>,
}

pub type CopySourceRange = String;

#[derive(Debug, Clone)]
pub struct PutBucketTaggingRequest {
    pub bucket: Option<BucketName>,
    pub tagging: Option<Tagging>,
    pub content_md5: Option<ContentMd5>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug, Clone)]
pub struct QueueConfiguration {
    pub events: Option<EventList>,
    pub id: Option<NotificationId>,
    pub queue_arn: Option<QueueArn>,
    pub filter: Option<NotificationConfigurationFilter>,
}

#[derive(Debug, Clone)]
pub struct ContinuationEvent {}

#[derive(Debug, Clone)]
pub struct RequestProgress {
    pub enabled: Option<EnableRequestProgress>,
}

#[derive(Debug, Clone)]
pub struct GetBucketAnalyticsConfigurationOutput {
    pub analytics_configuration: Option<AnalyticsConfiguration>,
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

pub type AnalyticsId = String;

#[derive(Debug, Clone)]
pub struct ReplicationTimeValue {
    pub minutes: Option<Minutes>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct RestoreObject;
impl Op for RestoreObject {
    type Input = RestoreObjectRequest;
    type Output = RestoreObjectOutput;
}

#[derive(Debug, Clone)]
pub struct Destination {
    pub account: Option<AccountId>,
    pub replication_time: Option<ReplicationTime>,
    pub storage_class: Option<StorageClass>,
    pub encryption_configuration: Option<EncryptionConfiguration>,
    pub access_control_translation: Option<AccessControlTranslation>,
    pub bucket: Option<BucketName>,
    pub metrics: Option<Metrics>,
}

#[derive(Debug, Clone)]
pub struct Tag {
    pub key: Option<ObjectKey>,
    pub value: Option<Value>,
}

pub type TagSet = Vec<Tag>;

#[derive(Debug, Clone)]
pub struct ReplicationConfiguration {
    pub role: Option<Role>,
    pub rules: Option<ReplicationRules>,
}

pub type Id = String;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ListObjects;
impl Op for ListObjects {
    type Input = ListObjectsRequest;
    type Output = ListObjectsOutput;
}

pub type EventList = Vec<Event>;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct CompleteMultipartUpload;
impl Op for CompleteMultipartUpload {
    type Input = CompleteMultipartUploadRequest;
    type Output = CompleteMultipartUploadOutput;
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct DeleteBucketReplication;
impl Op for DeleteBucketReplication {
    type Input = DeleteBucketReplicationRequest;
    type Output = ();
}

#[derive(Debug, Clone)]
pub struct EventBridgeConfiguration {}

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
pub struct WriteGetObjectResponse;
impl Op for WriteGetObjectResponse {
    type Input = WriteGetObjectResponseRequest;
    type Output = ();
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct GetBucketPolicy;
impl Op for GetBucketPolicy {
    type Input = GetBucketPolicyRequest;
    type Output = GetBucketPolicyOutput;
}

pub type Expires = String;

#[derive(Debug, Clone)]
pub struct ListMultipartUploadsOutput {
    pub encoding_type: Option<EncodingType>,
    pub bucket: Option<BucketName>,
    pub delimiter: Option<Delimiter>,
    pub key_marker: Option<KeyMarker>,
    pub is_truncated: Option<IsTruncated>,
    pub next_key_marker: Option<NextKeyMarker>,
    pub upload_id_marker: Option<UploadIdMarker>,
    pub next_upload_id_marker: Option<NextUploadIdMarker>,
    pub uploads: Option<MultipartUploadList>,
    pub prefix: Option<Prefix>,
    pub max_uploads: Option<MaxUploads>,
    pub common_prefixes: Option<CommonPrefixList>,
}

#[derive(Debug, Clone)]
pub struct DeleteMarkerEntry {
    pub key: Option<ObjectKey>,
    pub owner: Option<Owner>,
    pub is_latest: Option<IsLatest>,
    pub last_modified: Option<LastModified>,
    pub version_id: Option<ObjectVersionId>,
}

#[derive(Debug, Clone)]
pub struct GetObjectRetentionRequest {
    pub key: Option<ObjectKey>,
    pub version_id: Option<ObjectVersionId>,
    pub request_payer: Option<RequestPayer>,
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}

#[derive(Debug, Clone)]
pub struct RestoreObjectRequest {
    pub request_payer: Option<RequestPayer>,
    pub bucket: Option<BucketName>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub version_id: Option<ObjectVersionId>,
    pub restore_request: Option<RestoreRequest>,
    pub key: Option<ObjectKey>,
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug, Clone)]
pub struct InputSerialization {
    pub json: Option<JsonInput>,
    pub compression_type: Option<CompressionType>,
    pub parquet: Option<ParquetInput>,
    pub csv: Option<CsvInput>,
}

#[derive(Debug, Clone)]
pub struct Metrics {
    pub event_threshold: Option<ReplicationTimeValue>,
    pub status: Option<MetricsStatus>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct GetBucketAccelerateConfiguration;
impl Op for GetBucketAccelerateConfiguration {
    type Input = GetBucketAccelerateConfigurationRequest;
    type Output = GetBucketAccelerateConfigurationOutput;
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

#[derive(Debug, Clone)]
pub struct AbortMultipartUploadOutput {
    pub request_charged: Option<RequestCharged>,
}

#[derive(Debug, Clone)]
pub struct GetObjectAttributesParts {
    pub max_parts: Option<MaxParts>,
    pub total_parts_count: Option<PartsCount>,
    pub next_part_number_marker: Option<NextPartNumberMarker>,
    pub part_number_marker: Option<PartNumberMarker>,
    pub is_truncated: Option<IsTruncated>,
    pub parts: Option<PartsList>,
}

#[derive(Debug, Clone)]
pub struct DeleteObjectRequest {
    pub bypass_governance_retention: Option<BypassGovernanceRetention>,
    pub request_payer: Option<RequestPayer>,
    pub version_id: Option<ObjectVersionId>,
    pub key: Option<ObjectKey>,
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
    pub mfa: Option<Mfa>,
}

#[derive(Debug, Clone)]
pub struct ListObjectsRequest {
    pub request_payer: Option<RequestPayer>,
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
    pub marker: Option<Marker>,
    pub max_keys: Option<MaxKeys>,
    pub encoding_type: Option<EncodingType>,
    pub prefix: Option<Prefix>,
    pub delimiter: Option<Delimiter>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ListMultipartUploads;
impl Op for ListMultipartUploads {
    type Input = ListMultipartUploadsRequest;
    type Output = ListMultipartUploadsOutput;
}

pub type ObjectAttributesList = Vec<ObjectAttributes>;

#[derive(Debug, Clone)]
pub struct GetBucketLifecycleConfigurationRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}

#[derive(Debug, Clone)]
pub struct Encryption {
    pub encryption_type: Option<ServerSideEncryption>,
    pub kms_key_id: Option<SsekmsKeyId>,
    pub kms_context: Option<KmsContext>,
}

#[derive(Debug, Clone)]
pub struct ListObjectVersionsOutput {
    pub versions: Option<ObjectVersionList>,
    pub max_keys: Option<MaxKeys>,
    pub next_key_marker: Option<NextKeyMarker>,
    pub next_version_id_marker: Option<NextVersionIdMarker>,
    pub key_marker: Option<KeyMarker>,
    pub is_truncated: Option<IsTruncated>,
    pub name: Option<BucketName>,
    pub version_id_marker: Option<VersionIdMarker>,
    pub prefix: Option<Prefix>,
    pub delimiter: Option<Delimiter>,
    pub delete_markers: Option<DeleteMarkers>,
    pub common_prefixes: Option<CommonPrefixList>,
    pub encoding_type: Option<EncodingType>,
}

#[derive(Debug, Clone)]
pub struct PutBucketAccelerateConfigurationRequest {
    pub bucket: Option<BucketName>,
    pub accelerate_configuration: Option<AccelerateConfiguration>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct GetObjectTagging;
impl Op for GetObjectTagging {
    type Input = GetObjectTaggingRequest;
    type Output = GetObjectTaggingOutput;
}

pub type MissingMeta = i32;

pub type StreamingBlob = Arc<hyper::Body>;

pub type CopySourceVersionId = String;

#[derive(Debug, Clone)]
pub struct InventorySchedule {
    pub frequency: Option<InventoryFrequency>,
}

#[derive(Debug, Clone)]
pub struct ListBucketIntelligentTieringConfigurationsOutput {
    pub is_truncated: Option<IsTruncated>,
    pub next_continuation_token: Option<NextToken>,
    pub intelligent_tiering_configuration_list: Option<IntelligentTieringConfigurationList>,
    pub continuation_token: Option<Token>,
}

#[derive(Debug, Clone)]
pub struct EncryptionConfiguration {
    pub replica_kms_key_id: Option<ReplicaKmsKeyId>,
}

#[derive(Debug, Clone)]
pub struct StorageClassAnalysis {
    pub data_export: Option<StorageClassAnalysisDataExport>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PutBucketTagging;
impl Op for PutBucketTagging {
    type Input = PutBucketTaggingRequest;
    type Output = ();
}

#[derive(Debug, Clone)]
pub struct GetBucketOwnershipControlsOutput {
    pub ownership_controls: Option<OwnershipControls>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct GetBucketInventoryConfiguration;
impl Op for GetBucketInventoryConfiguration {
    type Input = GetBucketInventoryConfigurationRequest;
    type Output = GetBucketInventoryConfigurationOutput;
}

#[derive(Debug, Clone)]
pub enum MetricsFilter {
    Prefix(Prefix),
    AccessPointArn(AccessPointArn),
    And(MetricsAndOperator),
    Tag(Tag),
}

pub type NextPartNumberMarker = String;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct GetBucketCors;
impl Op for GetBucketCors {
    type Input = GetBucketCorsRequest;
    type Output = GetBucketCorsOutput;
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PutObjectAcl;
impl Op for PutObjectAcl {
    type Input = PutObjectAclRequest;
    type Output = PutObjectAclOutput;
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

pub type DeletedObjects = Vec<DeletedObject>;

#[derive(Debug, Clone)]
pub struct GetObjectTorrentOutput {
    pub request_charged: Option<RequestCharged>,
    pub body: Option<StreamingBlob>,
}

pub type IfMatch = String;

pub type DeleteMarker = bool;

pub type Body = Vec<u8>;

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

#[derive(Debug, Clone)]
pub enum LifecycleRuleFilter {
    And(LifecycleRuleAndOperator),
    ObjectSizeGreaterThan(ObjectSizeGreaterThanBytes),
    Prefix(Prefix),
    Tag(Tag),
    ObjectSizeLessThan(ObjectSizeLessThanBytes),
}

#[derive(Debug, Clone)]
pub struct ListObjectsV2Request {
    pub continuation_token: Option<Token>,
    pub encoding_type: Option<EncodingType>,
    pub fetch_owner: Option<FetchOwner>,
    pub start_after: Option<StartAfter>,
    pub max_keys: Option<MaxKeys>,
    pub expected_bucket_owner: Option<AccountId>,
    pub delimiter: Option<Delimiter>,
    pub request_payer: Option<RequestPayer>,
    pub prefix: Option<Prefix>,
    pub bucket: Option<BucketName>,
}

#[derive(Debug, Clone)]
pub struct HeadBucketRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug, Clone)]
pub struct NoncurrentVersionExpiration {
    pub noncurrent_days: Option<Days>,
    pub newer_noncurrent_versions: Option<VersionCount>,
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

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PutBucketCors;
impl Op for PutBucketCors {
    type Input = PutBucketCorsRequest;
    type Output = ();
}

#[derive(Debug, Clone)]
pub struct DeleteBucketLifecycleRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PutBucketRequestPayment;
impl Op for PutBucketRequestPayment {
    type Input = PutBucketRequestPaymentRequest;
    type Output = ();
}

#[derive(Debug, Clone)]
pub struct BucketAlreadyExists {}

pub type CopySourceSseCustomerKey = String;

#[derive(Debug, Clone)]
pub struct PutPublicAccessBlockRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub public_access_block_configuration: Option<PublicAccessBlockConfiguration>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub bucket: Option<BucketName>,
    pub content_md5: Option<ContentMd5>,
}

#[derive(Debug, Clone)]
pub struct PutObjectLegalHoldRequest {
    pub bucket: Option<BucketName>,
    pub legal_hold: Option<ObjectLockLegalHold>,
    pub key: Option<ObjectKey>,
    pub content_md5: Option<ContentMd5>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub expected_bucket_owner: Option<AccountId>,
    pub version_id: Option<ObjectVersionId>,
    pub request_payer: Option<RequestPayer>,
}

pub type LambdaFunctionArn = String;

pub type Delimiter = String;

pub type QueueConfigurationList = Vec<QueueConfiguration>;

#[derive(Debug, Clone)]
pub struct ObjectVersion {
    pub e_tag: Option<ETag>,
    pub version_id: Option<ObjectVersionId>,
    pub checksum_algorithm: Option<ChecksumAlgorithmList>,
    pub is_latest: Option<IsLatest>,
    pub owner: Option<Owner>,
    pub last_modified: Option<LastModified>,
    pub size: Option<Size>,
    pub key: Option<ObjectKey>,
    pub storage_class: Option<ObjectVersionStorageClass>,
}

pub type Initiated = String;

pub type Location = String;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct DeleteBucketEncryption;
impl Op for DeleteBucketEncryption {
    type Input = DeleteBucketEncryptionRequest;
    type Output = ();
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

#[derive(Debug, Clone)]
pub struct IntelligentTieringConfiguration {
    pub id: Option<IntelligentTieringId>,
    pub filter: Option<IntelligentTieringFilter>,
    pub status: Option<IntelligentTieringStatus>,
    pub tierings: Option<TieringList>,
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

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PutBucketEncryption;
impl Op for PutBucketEncryption {
    type Input = PutBucketEncryptionRequest;
    type Output = ();
}

#[derive(Debug, Clone)]
pub struct GetBucketRequestPaymentOutput {
    pub payer: Option<Payer>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct DeleteObjects;
impl Op for DeleteObjects {
    type Input = DeleteObjectsRequest;
    type Output = DeleteObjectsOutput;
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct CreateBucket;
impl Op for CreateBucket {
    type Input = CreateBucketRequest;
    type Output = CreateBucketOutput;
}

#[derive(Debug, Clone)]
pub struct MetricsConfiguration {
    pub filter: Option<MetricsFilter>,
    pub id: Option<MetricsId>,
}

pub type NotificationId = String;

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

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PutBucketLifecycleConfiguration;
impl Op for PutBucketLifecycleConfiguration {
    type Input = PutBucketLifecycleConfigurationRequest;
    type Output = ();
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

pub type BytesReturned = i64;

pub type ObjectLockEnabledForBucket = bool;

#[derive(Debug, Clone)]
pub struct CopyPartResult {
    pub last_modified: Option<LastModified>,
    pub checksum_sha256: Option<ChecksumSha256>,
    pub e_tag: Option<ETag>,
    pub checksum_crc32: Option<ChecksumCrc32>,
    pub checksum_sha1: Option<ChecksumSha1>,
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
}

#[derive(Debug, Clone)]
pub struct Initiator {
    pub display_name: Option<DisplayName>,
    pub id: Option<Id>,
}

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

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct GetBucketNotificationConfiguration;
impl Op for GetBucketNotificationConfiguration {
    type Input = GetBucketNotificationConfigurationRequest;
    type Output = NotificationConfiguration;
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PutObjectLockConfiguration;
impl Op for PutObjectLockConfiguration {
    type Input = PutObjectLockConfigurationRequest;
    type Output = PutObjectLockConfigurationOutput;
}

pub type ContentLength = i64;

pub type Code = String;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct GetBucketVersioning;
impl Op for GetBucketVersioning {
    type Input = GetBucketVersioningRequest;
    type Output = GetBucketVersioningOutput;
}

#[derive(Debug, Clone)]
pub struct GetObjectLockConfigurationOutput {
    pub object_lock_configuration: Option<ObjectLockConfiguration>,
}

pub type AccountId = String;

#[derive(Debug, Clone)]
pub struct DeletePublicAccessBlockRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug, Clone)]
pub struct Stats {
    pub bytes_processed: Option<BytesProcessed>,
    pub bytes_returned: Option<BytesReturned>,
    pub bytes_scanned: Option<BytesScanned>,
}

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

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct GetPublicAccessBlock;
impl Op for GetPublicAccessBlock {
    type Input = GetPublicAccessBlockRequest;
    type Output = GetPublicAccessBlockOutput;
}

pub type ObjectList = Vec<Object>;

pub type ObjectSizeGreaterThanBytes = i64;

pub type AbortRuleId = String;

pub type ServerSideEncryptionRules = Vec<ServerSideEncryptionRule>;

#[derive(Debug, Clone)]
pub struct InvalidObjectState {
    pub access_tier: Option<IntelligentTieringAccessTier>,
    pub storage_class: Option<StorageClass>,
}

pub type WebsiteRedirectLocation = String;

pub type InventoryConfigurationList = Vec<InventoryConfiguration>;

#[derive(Debug, Clone)]
pub struct ObjectPart {
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
    pub checksum_sha1: Option<ChecksumSha1>,
    pub checksum_sha256: Option<ChecksumSha256>,
    pub checksum_crc32: Option<ChecksumCrc32>,
    pub part_number: Option<PartNumber>,
    pub size: Option<Size>,
}

#[derive(Debug, Clone)]
pub struct UploadPartRequest {
    pub key: Option<ObjectKey>,
    pub content_length: Option<ContentLength>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub content_md5: Option<ContentMd5>,
    pub checksum_sha256: Option<ChecksumSha256>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub part_number: Option<PartNumber>,
    pub checksum_crc32: Option<ChecksumCrc32>,
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
    pub checksum_sha1: Option<ChecksumSha1>,
    pub body: Option<StreamingBlob>,
    pub sse_customer_key: Option<SseCustomerKey>,
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub request_payer: Option<RequestPayer>,
    pub upload_id: Option<MultipartUploadId>,
}

#[derive(Debug, Clone)]
pub struct GetBucketCorsRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
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

#[derive(Debug, Clone)]
pub struct GetBucketAccelerateConfigurationOutput {
    pub status: Option<BucketAccelerateStatus>,
}

#[derive(Debug, Clone)]
pub struct Grant {
    pub permission: Option<Permission>,
    pub grantee: Option<Grantee>,
}

pub type ChecksumSha256 = String;

#[derive(Debug, Clone)]
pub struct ObjectLockConfiguration {
    pub object_lock_enabled: Option<ObjectLockEnabled>,
    pub rule: Option<ObjectLockRule>,
}

#[derive(Debug, Clone)]
pub struct Transition {
    pub date: Option<Date>,
    pub storage_class: Option<TransitionStorageClass>,
    pub days: Option<Days>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct DeleteBucketIntelligentTieringConfiguration;
impl Op for DeleteBucketIntelligentTieringConfiguration {
    type Input = DeleteBucketIntelligentTieringConfigurationRequest;
    type Output = ();
}

#[derive(Debug, Clone)]
pub enum AnalyticsFilter {
    Tag(Tag),
    And(AnalyticsAndOperator),
    Prefix(Prefix),
}

#[derive(Debug, Clone)]
pub struct DeleteMarkerReplication {
    pub status: Option<DeleteMarkerReplicationStatus>,
}

#[derive(Debug, Clone)]
pub struct DeleteObjectTaggingOutput {
    pub version_id: Option<ObjectVersionId>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct DeleteObject;
impl Op for DeleteObject {
    type Input = DeleteObjectRequest;
    type Output = DeleteObjectOutput;
}

#[derive(Debug, Clone)]
pub struct GetBucketVersioningRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}

pub type StartAfter = String;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ListBucketMetricsConfigurations;
impl Op for ListBucketMetricsConfigurations {
    type Input = ListBucketMetricsConfigurationsRequest;
    type Output = ListBucketMetricsConfigurationsOutput;
}

pub type IsLatest = bool;

pub type NextUploadIdMarker = String;

pub type TargetBucket = String;

#[derive(Debug, Clone)]
pub struct NotificationConfigurationFilter {
    pub key: Option<S3KeyFilter>,
}

#[derive(Debug, Clone)]
pub struct CorsConfiguration {
    pub cors_rules: Option<CorsRules>,
}

#[derive(Debug, Clone)]
pub struct MetricsAndOperator {
    pub tags: Option<TagSet>,
    pub access_point_arn: Option<AccessPointArn>,
    pub prefix: Option<Prefix>,
}

pub type Message = String;

pub type NoncurrentVersionTransitionList = Vec<NoncurrentVersionTransition>;

#[derive(Debug, Clone)]
pub struct GetBucketCorsOutput {
    pub cors_rules: Option<CorsRules>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct HeadBucket;
impl Op for HeadBucket {
    type Input = HeadBucketRequest;
    type Output = ();
}

#[derive(Debug, Clone)]
pub struct AbortIncompleteMultipartUpload {
    pub days_after_initiation: Option<DaysAfterInitiation>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PutObjectRetention;
impl Op for PutObjectRetention {
    type Input = PutObjectRetentionRequest;
    type Output = PutObjectRetentionOutput;
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

pub type CreationDate = String;

pub type BypassGovernanceRetention = bool;

#[derive(Debug, Clone)]
pub struct Grantee {
    pub email_address: Option<EmailAddress>,
    pub id: Option<Id>,
    pub display_name: Option<DisplayName>,
    pub uri: Option<Uri>,
    pub r#type: Option<Type>,
}

pub type MaxParts = i32;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PutPublicAccessBlock;
impl Op for PutPublicAccessBlock {
    type Input = PutPublicAccessBlockRequest;
    type Output = ();
}

#[derive(Debug, Clone)]
pub enum ReplicationRuleFilter {
    And(ReplicationRuleAndOperator),
    Prefix(Prefix),
    Tag(Tag),
}

#[derive(Debug, Clone)]
pub struct SelectObjectContentRequest {
    pub sse_customer_key: Option<SseCustomerKey>,
    pub bucket: Option<BucketName>,
    pub expression_type: Option<ExpressionType>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub expression: Option<Expression>,
    pub key: Option<ObjectKey>,
    pub input_serialization: Option<InputSerialization>,
    pub expected_bucket_owner: Option<AccountId>,
    pub scan_range: Option<ScanRange>,
    pub output_serialization: Option<OutputSerialization>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub request_progress: Option<RequestProgress>,
}

pub type KeyPrefixEquals = String;

#[derive(Debug, Clone)]
pub struct Condition {
    pub key_prefix_equals: Option<KeyPrefixEquals>,
    pub http_error_code_returned_equals: Option<HttpErrorCodeReturnedEquals>,
}

pub type CorsRules = Vec<CorsRule>;

pub type DeleteMarkers = Vec<DeleteMarkerEntry>;

#[derive(Debug, Clone)]
pub struct PutObjectRetentionOutput {
    pub request_charged: Option<RequestCharged>,
}

pub type Role = String;

pub type Start = i64;

pub type Suffix = String;

pub type ContentLanguage = String;

pub type SseCustomerKeyMd5 = String;

pub type ObjectVersionId = String;

pub type Parts = Vec<Part>;

#[derive(Debug, Clone)]
pub struct Bucket {
    pub creation_date: Option<CreationDate>,
    pub name: Option<BucketName>,
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

pub type Comments = String;

#[derive(Debug, Clone)]
pub struct CompleteMultipartUploadOutput {
    pub ssekms_key_id: Option<SsekmsKeyId>,
    pub key: Option<ObjectKey>,
    pub bucket: Option<BucketName>,
    pub request_charged: Option<RequestCharged>,
    pub server_side_encryption: Option<ServerSideEncryption>,
    pub checksum_crc32: Option<ChecksumCrc32>,
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
    pub checksum_sha1: Option<ChecksumSha1>,
    pub version_id: Option<ObjectVersionId>,
    pub e_tag: Option<ETag>,
    pub expiration: Option<Expiration>,
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    pub checksum_sha256: Option<ChecksumSha256>,
    pub location: Option<Location>,
}

#[derive(Debug, Clone)]
pub struct AnalyticsExportDestination {
    pub s3_bucket_destination: Option<AnalyticsS3BucketDestination>,
}

#[derive(Debug, Clone)]
pub struct GetBucketWebsiteOutput {
    pub redirect_all_requests_to: Option<RedirectAllRequestsTo>,
    pub error_document: Option<ErrorDocument>,
    pub routing_rules: Option<RoutingRules>,
    pub index_document: Option<IndexDocument>,
}

#[derive(Debug, Clone)]
pub struct GlacierJobParameters {
    pub tier: Option<Tier>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct GetObjectLegalHold;
impl Op for GetObjectLegalHold {
    type Input = GetObjectLegalHoldRequest;
    type Output = GetObjectLegalHoldOutput;
}

pub type GrantWrite = String;

pub type ChecksumSha1 = String;

#[derive(Debug, Clone)]
pub struct InventoryConfiguration {
    pub optional_fields: Option<InventoryOptionalFields>,
    pub schedule: Option<InventorySchedule>,
    pub filter: Option<InventoryFilter>,
    pub included_object_versions: Option<InventoryIncludedObjectVersions>,
    pub is_enabled: Option<IsEnabled>,
    pub destination: Option<InventoryDestination>,
    pub id: Option<InventoryId>,
}

#[derive(Debug, Clone)]
pub struct DeleteBucketPolicyRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}

#[derive(Debug, Clone)]
pub struct GetObjectAttributesOutput {
    pub request_charged: Option<RequestCharged>,
    pub object_parts: Option<GetObjectAttributesParts>,
    pub storage_class: Option<StorageClass>,
    pub e_tag: Option<ETag>,
    pub delete_marker: Option<DeleteMarker>,
    pub object_size: Option<ObjectSize>,
    pub version_id: Option<ObjectVersionId>,
    pub last_modified: Option<LastModified>,
    pub checksum: Option<Checksum>,
}

#[derive(Debug, Clone)]
pub struct NotificationConfiguration {
    pub topic_configurations: Option<TopicConfigurationList>,
    pub lambda_function_configurations: Option<LambdaFunctionConfigurationList>,
    pub event_bridge_configuration: Option<EventBridgeConfiguration>,
    pub queue_configurations: Option<QueueConfigurationList>,
}

pub type ObjectLockToken = String;

pub type RoutingRules = Vec<RoutingRule>;

#[derive(Debug, Clone)]
pub struct GetBucketLifecycleConfigurationOutput {
    pub rules: Option<LifecycleRules>,
}

#[derive(Debug, Clone)]
pub struct ServerSideEncryptionByDefault {
    pub kms_master_key_id: Option<SsekmsKeyId>,
    pub sse_algorithm: Option<ServerSideEncryption>,
}

#[derive(Debug, Clone)]
pub struct PutBucketInventoryConfigurationRequest {
    pub id: Option<InventoryId>,
    pub bucket: Option<BucketName>,
    pub inventory_configuration: Option<InventoryConfiguration>,
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct GetBucketMetricsConfiguration;
impl Op for GetBucketMetricsConfiguration {
    type Input = GetBucketMetricsConfigurationRequest;
    type Output = GetBucketMetricsConfigurationOutput;
}

pub type HostName = String;

pub type MaxKeys = i32;

#[derive(Debug, Clone)]
pub struct DeleteObjectsOutput {
    pub request_charged: Option<RequestCharged>,
    pub deleted: Option<DeletedObjects>,
    pub errors: Option<Errors>,
}

pub type Size = i64;

pub type MaxAgeSeconds = i32;

#[derive(Debug, Clone)]
pub struct PutObjectTaggingOutput {
    pub version_id: Option<ObjectVersionId>,
}

pub type Policy = String;

#[derive(Debug, Clone)]
pub struct DeleteBucketCorsRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug, Clone)]
pub struct PutBucketNotificationConfigurationRequest {
    pub notification_configuration: Option<NotificationConfiguration>,
    pub skip_destination_validation: Option<SkipValidation>,
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug, Clone)]
pub struct Ssekms {
    pub key_id: Option<SsekmsKeyId>,
}

pub type CompletedPartList = Vec<CompletedPart>;

#[derive(Debug, Clone)]
pub struct GetBucketEncryptionOutput {
    pub server_side_encryption_configuration: Option<ServerSideEncryptionConfiguration>,
}

#[derive(Debug, Clone)]
pub struct GetBucketTaggingRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}

pub type GrantWriteAcp = String;

#[derive(Debug, Clone)]
pub struct ListBucketInventoryConfigurationsRequest {
    pub continuation_token: Option<Token>,
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}

#[derive(Debug, Clone)]
pub struct IndexDocument {
    pub suffix: Option<Suffix>,
}

#[derive(Debug, Clone)]
pub struct ObjectAlreadyInActiveTierError {}

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

#[derive(Debug, Clone)]
pub struct GetBucketPolicyStatusRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct GetObjectAttributes;
impl Op for GetObjectAttributes {
    type Input = GetObjectAttributesRequest;
    type Output = GetObjectAttributesOutput;
}

pub type ResponseContentType = String;

#[derive(Debug, Clone)]
pub struct RestoreObjectOutput {
    pub request_charged: Option<RequestCharged>,
    pub restore_output_path: Option<RestoreOutputPath>,
}

pub type ChecksumCrc32c = String;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct CopyObject;
impl Op for CopyObject {
    type Input = CopyObjectRequest;
    type Output = CopyObjectOutput;
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
pub enum S3Ops {
    PutBucketVersioning,
    ListBucketInventoryConfigurations,
    GetObject,
    GetBucketLifecycleConfiguration,
    PutObjectTagging,
    GetBucketReplication,
    PutObject,
    GetObjectTorrent,
    PutBucketMetricsConfiguration,
    DeleteBucketOwnershipControls,
    AbortMultipartUpload,
    GetBucketTagging,
    ListBucketIntelligentTieringConfigurations,
    PutBucketWebsite,
    PutBucketNotificationConfiguration,
    DeleteBucketMetricsConfiguration,
    UploadPart,
    PutBucketInventoryConfiguration,
    ListBucketAnalyticsConfigurations,
    GetBucketLocation,
    UploadPartCopy,
    DeleteBucket,
    PutBucketAcl,
    PutBucketIntelligentTieringConfiguration,
    CreateMultipartUpload,
    GetBucketAcl,
    DeleteBucketWebsite,
    DeletePublicAccessBlock,
    GetBucketPolicyStatus,
    ListObjectVersions,
    PutBucketAnalyticsConfiguration,
    GetBucketIntelligentTieringConfiguration,
    PutBucketAccelerateConfiguration,
    GetBucketEncryption,
    DeleteBucketCors,
    DeleteBucketInventoryConfiguration,
    HeadObject,
    GetObjectRetention,
    DeleteBucketLifecycle,
    GetBucketRequestPayment,
    DeleteBucketTagging,
    GetBucketAnalyticsConfiguration,
    PutObjectLegalHold,
    GetBucketOwnershipControls,
    GetObjectLockConfiguration,
    GetBucketWebsite,
    GetObjectAcl,
    ListObjectsV2,
    DeleteObjectTagging,
    DeleteBucketAnalyticsConfiguration,
    PutBucketPolicy,
    PutBucketLogging,
    PutBucketReplication,
    GetBucketLogging,
    PutBucketOwnershipControls,
    ListParts,
    ListBuckets,
    DeleteBucketPolicy,
    RestoreObject,
    ListObjects,
    CompleteMultipartUpload,
    DeleteBucketReplication,
    WriteGetObjectResponse,
    GetBucketPolicy,
    GetBucketAccelerateConfiguration,
    ListMultipartUploads,
    GetObjectTagging,
    PutBucketTagging,
    GetBucketInventoryConfiguration,
    GetBucketCors,
    PutObjectAcl,
    PutBucketCors,
    PutBucketRequestPayment,
    DeleteBucketEncryption,
    PutBucketEncryption,
    DeleteObjects,
    CreateBucket,
    PutBucketLifecycleConfiguration,
    GetBucketNotificationConfiguration,
    PutObjectLockConfiguration,
    GetBucketVersioning,
    GetPublicAccessBlock,
    DeleteBucketIntelligentTieringConfiguration,
    DeleteObject,
    ListBucketMetricsConfigurations,
    HeadBucket,
    PutObjectRetention,
    PutPublicAccessBlock,
    GetObjectLegalHold,
    GetBucketMetricsConfiguration,
    GetObjectAttributes,
    CopyObject,
}
#[doc = r" This macro calls a provided $macro for each S3 operation to generate code per op."]
macro_rules! generate_ops_code {
    ($ macro : ident) => {
        $macro!(PutBucketVersioning);
        $macro!(ListBucketInventoryConfigurations);
        $macro!(GetObject);
        $macro!(GetBucketLifecycleConfiguration);
        $macro!(PutObjectTagging);
        $macro!(GetBucketReplication);
        $macro!(PutObject);
        $macro!(GetObjectTorrent);
        $macro!(PutBucketMetricsConfiguration);
        $macro!(DeleteBucketOwnershipControls);
        $macro!(AbortMultipartUpload);
        $macro!(GetBucketTagging);
        $macro!(ListBucketIntelligentTieringConfigurations);
        $macro!(PutBucketWebsite);
        $macro!(PutBucketNotificationConfiguration);
        $macro!(DeleteBucketMetricsConfiguration);
        $macro!(UploadPart);
        $macro!(PutBucketInventoryConfiguration);
        $macro!(ListBucketAnalyticsConfigurations);
        $macro!(GetBucketLocation);
        $macro!(UploadPartCopy);
        $macro!(DeleteBucket);
        $macro!(PutBucketAcl);
        $macro!(PutBucketIntelligentTieringConfiguration);
        $macro!(CreateMultipartUpload);
        $macro!(GetBucketAcl);
        $macro!(DeleteBucketWebsite);
        $macro!(DeletePublicAccessBlock);
        $macro!(GetBucketPolicyStatus);
        $macro!(ListObjectVersions);
        $macro!(PutBucketAnalyticsConfiguration);
        $macro!(GetBucketIntelligentTieringConfiguration);
        $macro!(PutBucketAccelerateConfiguration);
        $macro!(GetBucketEncryption);
        $macro!(DeleteBucketCors);
        $macro!(DeleteBucketInventoryConfiguration);
        $macro!(HeadObject);
        $macro!(GetObjectRetention);
        $macro!(DeleteBucketLifecycle);
        $macro!(GetBucketRequestPayment);
        $macro!(DeleteBucketTagging);
        $macro!(GetBucketAnalyticsConfiguration);
        $macro!(PutObjectLegalHold);
        $macro!(GetBucketOwnershipControls);
        $macro!(GetObjectLockConfiguration);
        $macro!(GetBucketWebsite);
        $macro!(GetObjectAcl);
        $macro!(ListObjectsV2);
        $macro!(DeleteObjectTagging);
        $macro!(DeleteBucketAnalyticsConfiguration);
        $macro!(PutBucketPolicy);
        $macro!(PutBucketLogging);
        $macro!(PutBucketReplication);
        $macro!(GetBucketLogging);
        $macro!(PutBucketOwnershipControls);
        $macro!(ListParts);
        $macro!(ListBuckets);
        $macro!(DeleteBucketPolicy);
        $macro!(RestoreObject);
        $macro!(ListObjects);
        $macro!(CompleteMultipartUpload);
        $macro!(DeleteBucketReplication);
        $macro!(WriteGetObjectResponse);
        $macro!(GetBucketPolicy);
        $macro!(GetBucketAccelerateConfiguration);
        $macro!(ListMultipartUploads);
        $macro!(GetObjectTagging);
        $macro!(PutBucketTagging);
        $macro!(GetBucketInventoryConfiguration);
        $macro!(GetBucketCors);
        $macro!(PutObjectAcl);
        $macro!(PutBucketCors);
        $macro!(PutBucketRequestPayment);
        $macro!(DeleteBucketEncryption);
        $macro!(PutBucketEncryption);
        $macro!(DeleteObjects);
        $macro!(CreateBucket);
        $macro!(PutBucketLifecycleConfiguration);
        $macro!(GetBucketNotificationConfiguration);
        $macro!(PutObjectLockConfiguration);
        $macro!(GetBucketVersioning);
        $macro!(GetPublicAccessBlock);
        $macro!(DeleteBucketIntelligentTieringConfiguration);
        $macro!(DeleteObject);
        $macro!(ListBucketMetricsConfigurations);
        $macro!(HeadBucket);
        $macro!(PutObjectRetention);
        $macro!(PutPublicAccessBlock);
        $macro!(GetObjectLegalHold);
        $macro!(GetBucketMetricsConfiguration);
        $macro!(GetObjectAttributes);
        $macro!(CopyObject);
    };
}
#[doc = r" This macro calls a provided $macro for each S3 operation to generate item per op."]
macro_rules ! generate_ops_items { ($ macro : ident) => { $ macro ! (PutBucketVersioning) , $ macro ! (ListBucketInventoryConfigurations) , $ macro ! (GetObject) , $ macro ! (GetBucketLifecycleConfiguration) , $ macro ! (PutObjectTagging) , $ macro ! (GetBucketReplication) , $ macro ! (PutObject) , $ macro ! (GetObjectTorrent) , $ macro ! (PutBucketMetricsConfiguration) , $ macro ! (DeleteBucketOwnershipControls) , $ macro ! (AbortMultipartUpload) , $ macro ! (GetBucketTagging) , $ macro ! (ListBucketIntelligentTieringConfigurations) , $ macro ! (PutBucketWebsite) , $ macro ! (PutBucketNotificationConfiguration) , $ macro ! (DeleteBucketMetricsConfiguration) , $ macro ! (UploadPart) , $ macro ! (PutBucketInventoryConfiguration) , $ macro ! (ListBucketAnalyticsConfigurations) , $ macro ! (GetBucketLocation) , $ macro ! (UploadPartCopy) , $ macro ! (DeleteBucket) , $ macro ! (PutBucketAcl) , $ macro ! (PutBucketIntelligentTieringConfiguration) , $ macro ! (CreateMultipartUpload) , $ macro ! (GetBucketAcl) , $ macro ! (DeleteBucketWebsite) , $ macro ! (DeletePublicAccessBlock) , $ macro ! (GetBucketPolicyStatus) , $ macro ! (ListObjectVersions) , $ macro ! (PutBucketAnalyticsConfiguration) , $ macro ! (GetBucketIntelligentTieringConfiguration) , $ macro ! (PutBucketAccelerateConfiguration) , $ macro ! (GetBucketEncryption) , $ macro ! (DeleteBucketCors) , $ macro ! (DeleteBucketInventoryConfiguration) , $ macro ! (HeadObject) , $ macro ! (GetObjectRetention) , $ macro ! (DeleteBucketLifecycle) , $ macro ! (GetBucketRequestPayment) , $ macro ! (DeleteBucketTagging) , $ macro ! (GetBucketAnalyticsConfiguration) , $ macro ! (PutObjectLegalHold) , $ macro ! (GetBucketOwnershipControls) , $ macro ! (GetObjectLockConfiguration) , $ macro ! (GetBucketWebsite) , $ macro ! (GetObjectAcl) , $ macro ! (ListObjectsV2) , $ macro ! (DeleteObjectTagging) , $ macro ! (DeleteBucketAnalyticsConfiguration) , $ macro ! (PutBucketPolicy) , $ macro ! (PutBucketLogging) , $ macro ! (PutBucketReplication) , $ macro ! (GetBucketLogging) , $ macro ! (PutBucketOwnershipControls) , $ macro ! (ListParts) , $ macro ! (ListBuckets) , $ macro ! (DeleteBucketPolicy) , $ macro ! (RestoreObject) , $ macro ! (ListObjects) , $ macro ! (CompleteMultipartUpload) , $ macro ! (DeleteBucketReplication) , $ macro ! (WriteGetObjectResponse) , $ macro ! (GetBucketPolicy) , $ macro ! (GetBucketAccelerateConfiguration) , $ macro ! (ListMultipartUploads) , $ macro ! (GetObjectTagging) , $ macro ! (PutBucketTagging) , $ macro ! (GetBucketInventoryConfiguration) , $ macro ! (GetBucketCors) , $ macro ! (PutObjectAcl) , $ macro ! (PutBucketCors) , $ macro ! (PutBucketRequestPayment) , $ macro ! (DeleteBucketEncryption) , $ macro ! (PutBucketEncryption) , $ macro ! (DeleteObjects) , $ macro ! (CreateBucket) , $ macro ! (PutBucketLifecycleConfiguration) , $ macro ! (GetBucketNotificationConfiguration) , $ macro ! (PutObjectLockConfiguration) , $ macro ! (GetBucketVersioning) , $ macro ! (GetPublicAccessBlock) , $ macro ! (DeleteBucketIntelligentTieringConfiguration) , $ macro ! (DeleteObject) , $ macro ! (ListBucketMetricsConfigurations) , $ macro ! (HeadBucket) , $ macro ! (PutObjectRetention) , $ macro ! (PutPublicAccessBlock) , $ macro ! (GetObjectLegalHold) , $ macro ! (GetBucketMetricsConfiguration) , $ macro ! (GetObjectAttributes) , $ macro ! (CopyObject) , } }
#[doc = r" This macro matches a variable of S3Ops type and expands the provided $macro"]
#[doc = r" for each S3 operation to generate code handler per op."]
macro_rules! generate_ops_match {
    ($ macro : ident , $ op : expr) => {
        match ($op) {
            S3Ops::PutBucketVersioning => $macro!(PutBucketVersioning),
            S3Ops::ListBucketInventoryConfigurations => $macro!(ListBucketInventoryConfigurations),
            S3Ops::GetObject => $macro!(GetObject),
            S3Ops::GetBucketLifecycleConfiguration => $macro!(GetBucketLifecycleConfiguration),
            S3Ops::PutObjectTagging => $macro!(PutObjectTagging),
            S3Ops::GetBucketReplication => $macro!(GetBucketReplication),
            S3Ops::PutObject => $macro!(PutObject),
            S3Ops::GetObjectTorrent => $macro!(GetObjectTorrent),
            S3Ops::PutBucketMetricsConfiguration => $macro!(PutBucketMetricsConfiguration),
            S3Ops::DeleteBucketOwnershipControls => $macro!(DeleteBucketOwnershipControls),
            S3Ops::AbortMultipartUpload => $macro!(AbortMultipartUpload),
            S3Ops::GetBucketTagging => $macro!(GetBucketTagging),
            S3Ops::ListBucketIntelligentTieringConfigurations => {
                $macro!(ListBucketIntelligentTieringConfigurations)
            }
            S3Ops::PutBucketWebsite => $macro!(PutBucketWebsite),
            S3Ops::PutBucketNotificationConfiguration => {
                $macro!(PutBucketNotificationConfiguration)
            }
            S3Ops::DeleteBucketMetricsConfiguration => $macro!(DeleteBucketMetricsConfiguration),
            S3Ops::UploadPart => $macro!(UploadPart),
            S3Ops::PutBucketInventoryConfiguration => $macro!(PutBucketInventoryConfiguration),
            S3Ops::ListBucketAnalyticsConfigurations => $macro!(ListBucketAnalyticsConfigurations),
            S3Ops::GetBucketLocation => $macro!(GetBucketLocation),
            S3Ops::UploadPartCopy => $macro!(UploadPartCopy),
            S3Ops::DeleteBucket => $macro!(DeleteBucket),
            S3Ops::PutBucketAcl => $macro!(PutBucketAcl),
            S3Ops::PutBucketIntelligentTieringConfiguration => {
                $macro!(PutBucketIntelligentTieringConfiguration)
            }
            S3Ops::CreateMultipartUpload => $macro!(CreateMultipartUpload),
            S3Ops::GetBucketAcl => $macro!(GetBucketAcl),
            S3Ops::DeleteBucketWebsite => $macro!(DeleteBucketWebsite),
            S3Ops::DeletePublicAccessBlock => $macro!(DeletePublicAccessBlock),
            S3Ops::GetBucketPolicyStatus => $macro!(GetBucketPolicyStatus),
            S3Ops::ListObjectVersions => $macro!(ListObjectVersions),
            S3Ops::PutBucketAnalyticsConfiguration => $macro!(PutBucketAnalyticsConfiguration),
            S3Ops::GetBucketIntelligentTieringConfiguration => {
                $macro!(GetBucketIntelligentTieringConfiguration)
            }
            S3Ops::PutBucketAccelerateConfiguration => $macro!(PutBucketAccelerateConfiguration),
            S3Ops::GetBucketEncryption => $macro!(GetBucketEncryption),
            S3Ops::DeleteBucketCors => $macro!(DeleteBucketCors),
            S3Ops::DeleteBucketInventoryConfiguration => {
                $macro!(DeleteBucketInventoryConfiguration)
            }
            S3Ops::HeadObject => $macro!(HeadObject),
            S3Ops::GetObjectRetention => $macro!(GetObjectRetention),
            S3Ops::DeleteBucketLifecycle => $macro!(DeleteBucketLifecycle),
            S3Ops::GetBucketRequestPayment => $macro!(GetBucketRequestPayment),
            S3Ops::DeleteBucketTagging => $macro!(DeleteBucketTagging),
            S3Ops::GetBucketAnalyticsConfiguration => $macro!(GetBucketAnalyticsConfiguration),
            S3Ops::PutObjectLegalHold => $macro!(PutObjectLegalHold),
            S3Ops::GetBucketOwnershipControls => $macro!(GetBucketOwnershipControls),
            S3Ops::GetObjectLockConfiguration => $macro!(GetObjectLockConfiguration),
            S3Ops::GetBucketWebsite => $macro!(GetBucketWebsite),
            S3Ops::GetObjectAcl => $macro!(GetObjectAcl),
            S3Ops::ListObjectsV2 => $macro!(ListObjectsV2),
            S3Ops::DeleteObjectTagging => $macro!(DeleteObjectTagging),
            S3Ops::DeleteBucketAnalyticsConfiguration => {
                $macro!(DeleteBucketAnalyticsConfiguration)
            }
            S3Ops::PutBucketPolicy => $macro!(PutBucketPolicy),
            S3Ops::PutBucketLogging => $macro!(PutBucketLogging),
            S3Ops::PutBucketReplication => $macro!(PutBucketReplication),
            S3Ops::GetBucketLogging => $macro!(GetBucketLogging),
            S3Ops::PutBucketOwnershipControls => $macro!(PutBucketOwnershipControls),
            S3Ops::ListParts => $macro!(ListParts),
            S3Ops::ListBuckets => $macro!(ListBuckets),
            S3Ops::DeleteBucketPolicy => $macro!(DeleteBucketPolicy),
            S3Ops::RestoreObject => $macro!(RestoreObject),
            S3Ops::ListObjects => $macro!(ListObjects),
            S3Ops::CompleteMultipartUpload => $macro!(CompleteMultipartUpload),
            S3Ops::DeleteBucketReplication => $macro!(DeleteBucketReplication),
            S3Ops::WriteGetObjectResponse => $macro!(WriteGetObjectResponse),
            S3Ops::GetBucketPolicy => $macro!(GetBucketPolicy),
            S3Ops::GetBucketAccelerateConfiguration => $macro!(GetBucketAccelerateConfiguration),
            S3Ops::ListMultipartUploads => $macro!(ListMultipartUploads),
            S3Ops::GetObjectTagging => $macro!(GetObjectTagging),
            S3Ops::PutBucketTagging => $macro!(PutBucketTagging),
            S3Ops::GetBucketInventoryConfiguration => $macro!(GetBucketInventoryConfiguration),
            S3Ops::GetBucketCors => $macro!(GetBucketCors),
            S3Ops::PutObjectAcl => $macro!(PutObjectAcl),
            S3Ops::PutBucketCors => $macro!(PutBucketCors),
            S3Ops::PutBucketRequestPayment => $macro!(PutBucketRequestPayment),
            S3Ops::DeleteBucketEncryption => $macro!(DeleteBucketEncryption),
            S3Ops::PutBucketEncryption => $macro!(PutBucketEncryption),
            S3Ops::DeleteObjects => $macro!(DeleteObjects),
            S3Ops::CreateBucket => $macro!(CreateBucket),
            S3Ops::PutBucketLifecycleConfiguration => $macro!(PutBucketLifecycleConfiguration),
            S3Ops::GetBucketNotificationConfiguration => {
                $macro!(GetBucketNotificationConfiguration)
            }
            S3Ops::PutObjectLockConfiguration => $macro!(PutObjectLockConfiguration),
            S3Ops::GetBucketVersioning => $macro!(GetBucketVersioning),
            S3Ops::GetPublicAccessBlock => $macro!(GetPublicAccessBlock),
            S3Ops::DeleteBucketIntelligentTieringConfiguration => {
                $macro!(DeleteBucketIntelligentTieringConfiguration)
            }
            S3Ops::DeleteObject => $macro!(DeleteObject),
            S3Ops::ListBucketMetricsConfigurations => $macro!(ListBucketMetricsConfigurations),
            S3Ops::HeadBucket => $macro!(HeadBucket),
            S3Ops::PutObjectRetention => $macro!(PutObjectRetention),
            S3Ops::PutPublicAccessBlock => $macro!(PutPublicAccessBlock),
            S3Ops::GetObjectLegalHold => $macro!(GetObjectLegalHold),
            S3Ops::GetBucketMetricsConfiguration => $macro!(GetBucketMetricsConfiguration),
            S3Ops::GetObjectAttributes => $macro!(GetObjectAttributes),
            S3Ops::CopyObject => $macro!(CopyObject),
        }
    };
}
pub(crate) use generate_ops_code;
pub(crate) use generate_ops_items;
pub(crate) use generate_ops_match;
