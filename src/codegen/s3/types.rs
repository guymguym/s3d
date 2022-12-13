#![allow(unused)]
#![allow(non_camel_case_types)]
use crate::common::OpService;
use aws_smithy_http_server::body::BoxBody;
use aws_smithy_http_server::operation::{
    Handler, IntoService, Operation, OperationService, OperationShape,
};
use aws_smithy_http_server::proto::rest_xml::RestXml;
use aws_smithy_http_server::response::IntoResponse;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;
use std::sync::Arc;

#[derive(Debug, Default, Clone)]
pub struct GetObjectAttributesParts {
    pub max_parts: Option<MaxParts>,
    pub parts: Option<PartsList>,
    pub total_parts_count: Option<PartsCount>,
    pub is_truncated: Option<IsTruncated>,
    pub part_number_marker: Option<PartNumberMarker>,
    pub next_part_number_marker: Option<NextPartNumberMarker>,
}
impl GetObjectAttributesParts {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct PutBucketLifecycleConfiguration;
impl OperationShape for PutBucketLifecycleConfiguration {
    const NAME: &'static str = "PutBucketLifecycleConfiguration";
    type Input = PutBucketLifecycleConfigurationRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct DeleteObjectsRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
    pub bypass_governance_retention: Option<BypassGovernanceRetention>,
    pub delete: Option<Delete>,
    pub mfa: Option<Mfa>,
    pub request_payer: Option<RequestPayer>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
}
impl DeleteObjectsRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type ObjectLockToken = String;

pub type ChecksumCrc32 = String;

#[derive(Debug, Default, Clone)]
pub struct BucketLifecycleConfiguration {
    pub rules: Option<LifecycleRules>,
}
impl BucketLifecycleConfiguration {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketReplicationRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}
impl DeleteBucketReplicationRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type TransitionList = Vec<Transition>;

pub type CreationDate = String;

#[derive(Debug, Default, Clone)]
pub struct CompletedMultipartUpload {
    pub parts: Option<CompletedPartList>,
}
impl CompletedMultipartUpload {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type CompletedPartList = Vec<CompletedPart>;

#[derive(Debug, Default, Clone)]
pub struct GetBucketAnalyticsConfiguration;
impl OperationShape for GetBucketAnalyticsConfiguration {
    const NAME: &'static str = "GetBucketAnalyticsConfiguration";
    type Input = GetBucketAnalyticsConfigurationRequest;
    type Output = GetBucketAnalyticsConfigurationOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetObjectTaggingOutput {
    pub tag_set: Option<TagSet>,
    pub version_id: Option<ObjectVersionId>,
}
impl GetObjectTaggingOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct IntelligentTieringAndOperator {
    pub tags: Option<TagSet>,
    pub prefix: Option<Prefix>,
}
impl IntelligentTieringAndOperator {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct IntelligentTieringFilter {
    pub and: Option<IntelligentTieringAndOperator>,
    pub prefix: Option<Prefix>,
    pub tag: Option<Tag>,
}
impl IntelligentTieringFilter {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type EventList = Vec<Event>;

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketAnalyticsConfigurationRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub id: Option<AnalyticsId>,
    pub bucket: Option<BucketName>,
}
impl DeleteBucketAnalyticsConfigurationRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type InventoryId = String;

#[derive(Debug, Default, Clone)]
pub struct InventoryS3BucketDestination {
    pub format: Option<InventoryFormat>,
    pub bucket: Option<BucketName>,
    pub account_id: Option<AccountId>,
    pub encryption: Option<InventoryEncryption>,
    pub prefix: Option<Prefix>,
}
impl InventoryS3BucketDestination {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct CommonPrefix {
    pub prefix: Option<Prefix>,
}
impl CommonPrefix {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct AbortMultipartUploadOutput {
    pub request_charged: Option<RequestCharged>,
}
impl AbortMultipartUploadOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum ObjectLockMode {
    COMPLIANCE,
    GOVERNANCE,
}
impl AsRef<str> for ObjectLockMode {
    fn as_ref(&self) -> &str {
        match self {
            Self::COMPLIANCE => "COMPLIANCE",
            Self::GOVERNANCE => "GOVERNANCE",
        }
    }
}
impl TryFrom<&str> for ObjectLockMode {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "COMPLIANCE" => Ok(Self::COMPLIANCE),
            "GOVERNANCE" => Ok(Self::GOVERNANCE),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListObjectsRequest {
    pub prefix: Option<Prefix>,
    pub encoding_type: Option<EncodingType>,
    pub delimiter: Option<Delimiter>,
    pub max_keys: Option<MaxKeys>,
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
    pub marker: Option<Marker>,
    pub request_payer: Option<RequestPayer>,
}
impl ListObjectsRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type ObjectSize = i64;

#[derive(Debug, Default, Clone)]
pub struct GetBucketTaggingOutput {
    pub tag_set: Option<TagSet>,
}
impl GetBucketTaggingOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetObjectTorrentOutput {
    pub body: Option<StreamingBlob>,
    pub request_charged: Option<RequestCharged>,
}
impl GetObjectTorrentOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct PutBucketLifecycleConfigurationRequest {
    pub bucket: Option<BucketName>,
    pub lifecycle_configuration: Option<BucketLifecycleConfiguration>,
    pub expected_bucket_owner: Option<AccountId>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
}
impl PutBucketLifecycleConfigurationRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type ReplaceKeyPrefixWith = String;

#[derive(Debug, Default, Clone)]
pub struct ReplicaModifications {
    pub status: Option<ReplicaModificationsStatus>,
}
impl ReplicaModifications {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ReplicationRule {
    pub prefix: Option<Prefix>,
    pub destination: Option<Destination>,
    pub existing_object_replication: Option<ExistingObjectReplication>,
    pub status: Option<ReplicationRuleStatus>,
    pub delete_marker_replication: Option<DeleteMarkerReplication>,
    pub filter: Option<ReplicationRuleFilter>,
    pub priority: Option<Priority>,
    pub source_selection_criteria: Option<SourceSelectionCriteria>,
    pub id: Option<Id>,
}
impl ReplicationRule {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type Description = String;

pub type KeyCount = i32;

pub type Location = String;

pub type Message = String;

#[derive(Debug, Default, Clone)]
pub struct PublicAccessBlockConfiguration {
    pub ignore_public_acls: Option<Setting>,
    pub block_public_policy: Option<Setting>,
    pub block_public_acls: Option<Setting>,
    pub restrict_public_buckets: Option<Setting>,
}
impl PublicAccessBlockConfiguration {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type ContentMd5 = String;

pub type Id = String;

pub type Metadata = HashMap<MetadataKey, MetadataValue>;

#[derive(Debug, Default, Clone)]
pub struct ReplicationTimeValue {
    pub minutes: Option<Minutes>,
}
impl ReplicationTimeValue {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ScanRange {
    pub end: Option<End>,
    pub start: Option<Start>,
}
impl ScanRange {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type TieringList = Vec<Tiering>;

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum BucketLogsPermission {
    READ,
    WRITE,
    FULL_CONTROL,
}
impl AsRef<str> for BucketLogsPermission {
    fn as_ref(&self) -> &str {
        match self {
            Self::READ => "READ",
            Self::WRITE => "WRITE",
            Self::FULL_CONTROL => "FULL_CONTROL",
        }
    }
}
impl TryFrom<&str> for BucketLogsPermission {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "READ" => Ok(Self::READ),
            "WRITE" => Ok(Self::WRITE),
            "FULL_CONTROL" => Ok(Self::FULL_CONTROL),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct PutBucketWebsiteRequest {
    pub website_configuration: Option<WebsiteConfiguration>,
    pub bucket: Option<BucketName>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub content_md5: Option<ContentMd5>,
    pub expected_bucket_owner: Option<AccountId>,
}
impl PutBucketWebsiteRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type ServerSideEncryptionRules = Vec<ServerSideEncryptionRule>;

pub type Expires = String;

#[derive(Debug, Default, Clone)]
pub struct PutBucketMetricsConfiguration;
impl OperationShape for PutBucketMetricsConfiguration {
    const NAME: &'static str = "PutBucketMetricsConfiguration";
    type Input = PutBucketMetricsConfigurationRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct Part {
    pub part_number: Option<PartNumber>,
    pub last_modified: Option<LastModified>,
    pub checksum_sha256: Option<ChecksumSha256>,
    pub checksum_crc32: Option<ChecksumCrc32>,
    pub checksum_sha1: Option<ChecksumSha1>,
    pub size: Option<Size>,
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
    pub e_tag: Option<ETag>,
}
impl Part {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum BucketCannedAcl {
    private,
    authenticated_read,
    public_read,
    public_read_write,
}
impl AsRef<str> for BucketCannedAcl {
    fn as_ref(&self) -> &str {
        match self {
            Self::private => "private",
            Self::authenticated_read => "authenticated-read",
            Self::public_read => "public-read",
            Self::public_read_write => "public-read-write",
        }
    }
}
impl TryFrom<&str> for BucketCannedAcl {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "private" => Ok(Self::private),
            "authenticated-read" => Ok(Self::authenticated_read),
            "public-read" => Ok(Self::public_read),
            "public-read-write" => Ok(Self::public_read_write),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetObjectRetention;
impl OperationShape for GetObjectRetention {
    const NAME: &'static str = "GetObjectRetention";
    type Input = GetObjectRetentionRequest;
    type Output = GetObjectRetentionOutput;
    type Error = ();
}

pub type ExposeHeader = String;

pub type UploadIdMarker = String;

#[derive(Debug, Default, Clone)]
pub struct PutObjectOutput {
    pub expiration: Option<Expiration>,
    pub checksum_crc32: Option<ChecksumCrc32>,
    pub checksum_sha256: Option<ChecksumSha256>,
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
    pub e_tag: Option<ETag>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub checksum_sha1: Option<ChecksumSha1>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    pub request_charged: Option<RequestCharged>,
    pub ssekms_encryption_context: Option<SsekmsEncryptionContext>,
    pub ssekms_key_id: Option<SsekmsKeyId>,
    pub server_side_encryption: Option<ServerSideEncryption>,
    pub version_id: Option<ObjectVersionId>,
}
impl PutObjectOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct PutBucketAnalyticsConfiguration;
impl OperationShape for PutBucketAnalyticsConfiguration {
    const NAME: &'static str = "PutBucketAnalyticsConfiguration";
    type Input = PutBucketAnalyticsConfigurationRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct ObjectLockConfiguration {
    pub object_lock_enabled: Option<ObjectLockEnabled>,
    pub rule: Option<ObjectLockRule>,
}
impl ObjectLockConfiguration {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct PutPublicAccessBlock;
impl OperationShape for PutPublicAccessBlock {
    const NAME: &'static str = "PutPublicAccessBlock";
    type Input = PutPublicAccessBlockRequest;
    type Output = ();
    type Error = ();
}

pub type SseCustomerKeyMd5 = String;

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
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

#[derive(Debug, Default, Clone)]
pub struct GetObjectLockConfigurationRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}
impl GetObjectLockConfigurationRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct PutBucketPolicyRequest {
    pub bucket: Option<BucketName>,
    pub confirm_remove_self_bucket_access: Option<ConfirmRemoveSelfBucketAccess>,
    pub content_md5: Option<ContentMd5>,
    pub policy: Option<Policy>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub expected_bucket_owner: Option<AccountId>,
}
impl PutBucketPolicyRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct PutObject;
impl OperationShape for PutObject {
    const NAME: &'static str = "PutObject";
    type Input = PutObjectRequest;
    type Output = PutObjectOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetPublicAccessBlockRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}
impl GetPublicAccessBlockRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct CreateMultipartUpload;
impl OperationShape for CreateMultipartUpload {
    const NAME: &'static str = "CreateMultipartUpload";
    type Input = CreateMultipartUploadRequest;
    type Output = CreateMultipartUploadOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct Object {
    pub owner: Option<Owner>,
    pub last_modified: Option<LastModified>,
    pub key: Option<ObjectKey>,
    pub storage_class: Option<ObjectStorageClass>,
    pub size: Option<Size>,
    pub e_tag: Option<ETag>,
    pub checksum_algorithm: Option<ChecksumAlgorithmList>,
}
impl Object {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct Owner {
    pub display_name: Option<DisplayName>,
    pub id: Option<Id>,
}
impl Owner {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct DeleteObjectRequest {
    pub bucket: Option<BucketName>,
    pub bypass_governance_retention: Option<BypassGovernanceRetention>,
    pub key: Option<ObjectKey>,
    pub mfa: Option<Mfa>,
    pub version_id: Option<ObjectVersionId>,
    pub expected_bucket_owner: Option<AccountId>,
    pub request_payer: Option<RequestPayer>,
}
impl DeleteObjectRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type IfMatch = String;

#[derive(Debug, Default, Clone)]
pub struct IndexDocument {
    pub suffix: Option<Suffix>,
}
impl IndexDocument {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type SseCustomerKey = String;

#[derive(Debug, Default, Clone)]
pub struct SelectObjectContentOutput {
    pub payload: Option<SelectObjectContentEventStream>,
}
impl SelectObjectContentOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListBucketInventoryConfigurationsRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
    pub continuation_token: Option<Token>,
}
impl ListBucketInventoryConfigurationsRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketLifecycleConfigurationRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}
impl GetBucketLifecycleConfigurationRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct CorsRule {
    pub allowed_headers: Option<AllowedHeaders>,
    pub allowed_origins: Option<AllowedOrigins>,
    pub allowed_methods: Option<AllowedMethods>,
    pub id: Option<Id>,
    pub max_age_seconds: Option<MaxAgeSeconds>,
    pub expose_headers: Option<ExposeHeaders>,
}
impl CorsRule {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct WriteGetObjectResponse;
impl OperationShape for WriteGetObjectResponse {
    const NAME: &'static str = "WriteGetObjectResponse";
    type Input = WriteGetObjectResponseRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct MetadataEntry {
    pub name: Option<MetadataKey>,
    pub value: Option<MetadataValue>,
}
impl MetadataEntry {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct PutBucketVersioning;
impl OperationShape for PutBucketVersioning {
    const NAME: &'static str = "PutBucketVersioning";
    type Input = PutBucketVersioningRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct ReplicationRuleAndOperator {
    pub prefix: Option<Prefix>,
    pub tags: Option<TagSet>,
}
impl ReplicationRuleAndOperator {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketWebsite;
impl OperationShape for GetBucketWebsite {
    const NAME: &'static str = "GetBucketWebsite";
    type Input = GetBucketWebsiteRequest;
    type Output = GetBucketWebsiteOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketOwnershipControls;
impl OperationShape for GetBucketOwnershipControls {
    const NAME: &'static str = "GetBucketOwnershipControls";
    type Input = GetBucketOwnershipControlsRequest;
    type Output = GetBucketOwnershipControlsOutput;
    type Error = ();
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
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

pub type TagSet = Vec<Tag>;

#[derive(Debug, Default, Clone)]
pub struct ListBucketIntelligentTieringConfigurationsRequest {
    pub bucket: Option<BucketName>,
    pub continuation_token: Option<Token>,
}
impl ListBucketIntelligentTieringConfigurationsRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ReplicationTime {
    pub status: Option<ReplicationTimeStatus>,
    pub time: Option<ReplicationTimeValue>,
}
impl ReplicationTime {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum Permission {
    READ_ACP,
    WRITE_ACP,
    FULL_CONTROL,
    READ,
    WRITE,
}
impl AsRef<str> for Permission {
    fn as_ref(&self) -> &str {
        match self {
            Self::READ_ACP => "READ_ACP",
            Self::WRITE_ACP => "WRITE_ACP",
            Self::FULL_CONTROL => "FULL_CONTROL",
            Self::READ => "READ",
            Self::WRITE => "WRITE",
        }
    }
}
impl TryFrom<&str> for Permission {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "READ_ACP" => Ok(Self::READ_ACP),
            "WRITE_ACP" => Ok(Self::WRITE_ACP),
            "FULL_CONTROL" => Ok(Self::FULL_CONTROL),
            "READ" => Ok(Self::READ),
            "WRITE" => Ok(Self::WRITE),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

pub type AllowedHeaders = Vec<AllowedHeader>;

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum ReplicationTimeStatus {
    Disabled,
    Enabled,
}
impl AsRef<str> for ReplicationTimeStatus {
    fn as_ref(&self) -> &str {
        match self {
            Self::Disabled => "Disabled",
            Self::Enabled => "Enabled",
        }
    }
}
impl TryFrom<&str> for ReplicationTimeStatus {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "Disabled" => Ok(Self::Disabled),
            "Enabled" => Ok(Self::Enabled),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetObjectLockConfiguration;
impl OperationShape for GetObjectLockConfiguration {
    const NAME: &'static str = "GetObjectLockConfiguration";
    type Input = GetObjectLockConfigurationRequest;
    type Output = GetObjectLockConfigurationOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct ListBucketIntelligentTieringConfigurationsOutput {
    pub continuation_token: Option<Token>,
    pub next_continuation_token: Option<NextToken>,
    pub is_truncated: Option<IsTruncated>,
    pub intelligent_tiering_configuration_list: Option<IntelligentTieringConfigurationList>,
}
impl ListBucketIntelligentTieringConfigurationsOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListObjectsV2;
impl OperationShape for ListObjectsV2 {
    const NAME: &'static str = "ListObjectsV2";
    type Input = ListObjectsV2Request;
    type Output = ListObjectsV2Output;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct CopyPartResult {
    pub e_tag: Option<ETag>,
    pub checksum_crc32: Option<ChecksumCrc32>,
    pub checksum_sha1: Option<ChecksumSha1>,
    pub checksum_sha256: Option<ChecksumSha256>,
    pub last_modified: Option<LastModified>,
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
}
impl CopyPartResult {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type CorsRules = Vec<CorsRule>;

pub type LocationPrefix = String;

#[derive(Debug, Default, Clone)]
pub struct NoSuchBucket {}
impl NoSuchBucket {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
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

pub type HostName = String;

#[derive(Debug, Default, Clone)]
pub struct GetBucketNotificationConfiguration;
impl OperationShape for GetBucketNotificationConfiguration {
    const NAME: &'static str = "GetBucketNotificationConfiguration";
    type Input = GetBucketNotificationConfigurationRequest;
    type Output = NotificationConfiguration;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct Initiator {
    pub id: Option<Id>,
    pub display_name: Option<DisplayName>,
}
impl Initiator {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type Initiated = String;

#[derive(Debug, Default, Clone)]
pub struct InventoryFilter {
    pub prefix: Option<Prefix>,
}
impl InventoryFilter {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct SelectParameters {
    pub expression_type: Option<ExpressionType>,
    pub output_serialization: Option<OutputSerialization>,
    pub expression: Option<Expression>,
    pub input_serialization: Option<InputSerialization>,
}
impl SelectParameters {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListParts;
impl OperationShape for ListParts {
    const NAME: &'static str = "ListParts";
    type Input = ListPartsRequest;
    type Output = ListPartsOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct Delete {
    pub quiet: Option<Quiet>,
    pub objects: Option<ObjectIdentifierList>,
}
impl Delete {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type CopySourceSseCustomerAlgorithm = String;

pub type IsTruncated = bool;

#[derive(Debug, Default, Clone)]
pub struct DeleteBucket;
impl OperationShape for DeleteBucket {
    const NAME: &'static str = "DeleteBucket";
    type Input = DeleteBucketRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct PutObjectRequest {
    pub cache_control: Option<CacheControl>,
    pub grant_read_acp: Option<GrantReadAcp>,
    pub object_lock_retain_until_date: Option<ObjectLockRetainUntilDate>,
    pub expected_bucket_owner: Option<AccountId>,
    pub body: Option<StreamingBlob>,
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
    pub bucket: Option<BucketName>,
    pub content_language: Option<ContentLanguage>,
    pub storage_class: Option<StorageClass>,
    pub content_encoding: Option<ContentEncoding>,
    pub content_length: Option<ContentLength>,
    pub expires: Option<Expires>,
    pub object_lock_mode: Option<ObjectLockMode>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub checksum_sha256: Option<ChecksumSha256>,
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    pub content_md5: Option<ContentMd5>,
    pub request_payer: Option<RequestPayer>,
    pub checksum_sha1: Option<ChecksumSha1>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub ssekms_encryption_context: Option<SsekmsEncryptionContext>,
    pub object_lock_legal_hold_status: Option<ObjectLockLegalHoldStatus>,
    pub sse_customer_key: Option<SseCustomerKey>,
    pub content_disposition: Option<ContentDisposition>,
    pub tagging: Option<TaggingHeader>,
    pub checksum_crc32: Option<ChecksumCrc32>,
    pub ssekms_key_id: Option<SsekmsKeyId>,
    pub server_side_encryption: Option<ServerSideEncryption>,
    pub grant_full_control: Option<GrantFullControl>,
    pub acl: Option<ObjectCannedAcl>,
    pub grant_read: Option<GrantRead>,
    pub key: Option<ObjectKey>,
    pub content_type: Option<ContentType>,
    pub grant_write_acp: Option<GrantWriteAcp>,
    pub metadata: Option<Metadata>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub website_redirect_location: Option<WebsiteRedirectLocation>,
}
impl PutObjectRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type StartAfter = String;

#[derive(Debug, Default, Clone)]
pub struct ListObjectsV2Output {
    pub delimiter: Option<Delimiter>,
    pub prefix: Option<Prefix>,
    pub start_after: Option<StartAfter>,
    pub contents: Option<ObjectList>,
    pub is_truncated: Option<IsTruncated>,
    pub max_keys: Option<MaxKeys>,
    pub encoding_type: Option<EncodingType>,
    pub next_continuation_token: Option<NextToken>,
    pub continuation_token: Option<Token>,
    pub common_prefixes: Option<CommonPrefixList>,
    pub name: Option<BucketName>,
    pub key_count: Option<KeyCount>,
}
impl ListObjectsV2Output {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type InventoryConfigurationList = Vec<InventoryConfiguration>;

#[derive(Debug, Default, Clone)]
pub struct DeleteMarkerEntry {
    pub key: Option<ObjectKey>,
    pub owner: Option<Owner>,
    pub is_latest: Option<IsLatest>,
    pub last_modified: Option<LastModified>,
    pub version_id: Option<ObjectVersionId>,
}
impl DeleteMarkerEntry {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct WriteGetObjectResponseRequest {
    pub content_range: Option<ContentRange>,
    pub storage_class: Option<StorageClass>,
    pub checksum_sha1: Option<ChecksumSha1>,
    pub object_lock_legal_hold_status: Option<ObjectLockLegalHoldStatus>,
    pub ssekms_key_id: Option<SsekmsKeyId>,
    pub tag_count: Option<TagCount>,
    pub status_code: Option<GetObjectResponseStatusCode>,
    pub expiration: Option<Expiration>,
    pub error_message: Option<ErrorMessage>,
    pub error_code: Option<ErrorCode>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub server_side_encryption: Option<ServerSideEncryption>,
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
    pub metadata: Option<Metadata>,
    pub parts_count: Option<PartsCount>,
    pub e_tag: Option<ETag>,
    pub restore: Option<Restore>,
    pub request_route: Option<RequestRoute>,
    pub missing_meta: Option<MissingMeta>,
    pub content_encoding: Option<ContentEncoding>,
    pub object_lock_mode: Option<ObjectLockMode>,
    pub accept_ranges: Option<AcceptRanges>,
    pub checksum_crc32: Option<ChecksumCrc32>,
    pub checksum_sha256: Option<ChecksumSha256>,
    pub content_type: Option<ContentType>,
    pub delete_marker: Option<DeleteMarker>,
    pub version_id: Option<ObjectVersionId>,
    pub request_token: Option<RequestToken>,
    pub content_length: Option<ContentLength>,
    pub body: Option<StreamingBlob>,
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    pub expires: Option<Expires>,
    pub content_language: Option<ContentLanguage>,
    pub object_lock_retain_until_date: Option<ObjectLockRetainUntilDate>,
    pub replication_status: Option<ReplicationStatus>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub content_disposition: Option<ContentDisposition>,
    pub last_modified: Option<LastModified>,
    pub request_charged: Option<RequestCharged>,
    pub cache_control: Option<CacheControl>,
}
impl WriteGetObjectResponseRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct UploadPartRequest {
    pub part_number: Option<PartNumber>,
    pub expected_bucket_owner: Option<AccountId>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub key: Option<ObjectKey>,
    pub checksum_crc32: Option<ChecksumCrc32>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub upload_id: Option<MultipartUploadId>,
    pub checksum_sha1: Option<ChecksumSha1>,
    pub request_payer: Option<RequestPayer>,
    pub sse_customer_key: Option<SseCustomerKey>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub checksum_sha256: Option<ChecksumSha256>,
    pub bucket: Option<BucketName>,
    pub content_length: Option<ContentLength>,
    pub body: Option<StreamingBlob>,
    pub content_md5: Option<ContentMd5>,
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
}
impl UploadPartRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct Stats {
    pub bytes_returned: Option<BytesReturned>,
    pub bytes_processed: Option<BytesProcessed>,
    pub bytes_scanned: Option<BytesScanned>,
}
impl Stats {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct NotificationConfiguration {
    pub lambda_function_configurations: Option<LambdaFunctionConfigurationList>,
    pub event_bridge_configuration: Option<EventBridgeConfiguration>,
    pub queue_configurations: Option<QueueConfigurationList>,
    pub topic_configurations: Option<TopicConfigurationList>,
}
impl NotificationConfiguration {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type AccessPointArn = String;

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum MetricsStatus {
    Disabled,
    Enabled,
}
impl AsRef<str> for MetricsStatus {
    fn as_ref(&self) -> &str {
        match self {
            Self::Disabled => "Disabled",
            Self::Enabled => "Enabled",
        }
    }
}
impl TryFrom<&str> for MetricsStatus {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "Disabled" => Ok(Self::Disabled),
            "Enabled" => Ok(Self::Enabled),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketVersioningRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}
impl GetBucketVersioningRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct Tiering {
    pub days: Option<IntelligentTieringDays>,
    pub access_tier: Option<IntelligentTieringAccessTier>,
}
impl Tiering {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListPartsOutput {
    pub next_part_number_marker: Option<NextPartNumberMarker>,
    pub is_truncated: Option<IsTruncated>,
    pub abort_date: Option<AbortDate>,
    pub parts: Option<Parts>,
    pub upload_id: Option<MultipartUploadId>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub abort_rule_id: Option<AbortRuleId>,
    pub storage_class: Option<StorageClass>,
    pub initiator: Option<Initiator>,
    pub part_number_marker: Option<PartNumberMarker>,
    pub max_parts: Option<MaxParts>,
    pub key: Option<ObjectKey>,
    pub request_charged: Option<RequestCharged>,
    pub owner: Option<Owner>,
    pub bucket: Option<BucketName>,
}
impl ListPartsOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetObjectRetentionRequest {
    pub request_payer: Option<RequestPayer>,
    pub version_id: Option<ObjectVersionId>,
    pub key: Option<ObjectKey>,
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}
impl GetObjectRetentionRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct AccessControlPolicy {
    pub grants: Option<Grants>,
    pub owner: Option<Owner>,
}
impl AccessControlPolicy {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketMetricsConfiguration;
impl OperationShape for DeleteBucketMetricsConfiguration {
    const NAME: &'static str = "DeleteBucketMetricsConfiguration";
    type Input = DeleteBucketMetricsConfigurationRequest;
    type Output = ();
    type Error = ();
}

pub type ErrorCode = String;

#[derive(Debug, Default, Clone)]
pub struct GetObjectLockConfigurationOutput {
    pub object_lock_configuration: Option<ObjectLockConfiguration>,
}
impl GetObjectLockConfigurationOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct RestoreObjectRequest {
    pub restore_request: Option<RestoreRequest>,
    pub version_id: Option<ObjectVersionId>,
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub key: Option<ObjectKey>,
    pub request_payer: Option<RequestPayer>,
}
impl RestoreObjectRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct CopyObjectResult {
    pub last_modified: Option<LastModified>,
    pub checksum_sha1: Option<ChecksumSha1>,
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
    pub checksum_crc32: Option<ChecksumCrc32>,
    pub checksum_sha256: Option<ChecksumSha256>,
    pub e_tag: Option<ETag>,
}
impl CopyObjectResult {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketPolicy;
impl OperationShape for DeleteBucketPolicy {
    const NAME: &'static str = "DeleteBucketPolicy";
    type Input = DeleteBucketPolicyRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketPolicyRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}
impl GetBucketPolicyRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type RecordDelimiter = String;

#[derive(Debug, Default, Clone)]
pub struct Redirect {
    pub replace_key_with: Option<ReplaceKeyWith>,
    pub host_name: Option<HostName>,
    pub protocol: Option<Protocol>,
    pub http_redirect_code: Option<HttpRedirectCode>,
    pub replace_key_prefix_with: Option<ReplaceKeyPrefixWith>,
}
impl Redirect {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type AllowedMethod = String;

#[derive(Debug, Default, Clone)]
pub struct GetBucketInventoryConfigurationRequest {
    pub bucket: Option<BucketName>,
    pub id: Option<InventoryId>,
    pub expected_bucket_owner: Option<AccountId>,
}
impl GetBucketInventoryConfigurationRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct PutBucketInventoryConfigurationRequest {
    pub inventory_configuration: Option<InventoryConfiguration>,
    pub id: Option<InventoryId>,
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}
impl PutBucketInventoryConfigurationRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct PutObjectLockConfigurationRequest {
    pub content_md5: Option<ContentMd5>,
    pub object_lock_configuration: Option<ObjectLockConfiguration>,
    pub request_payer: Option<RequestPayer>,
    pub token: Option<ObjectLockToken>,
    pub bucket: Option<BucketName>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub expected_bucket_owner: Option<AccountId>,
}
impl PutObjectLockConfigurationRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketPolicyRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}
impl DeleteBucketPolicyRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
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

#[derive(Debug, Default, Clone)]
pub struct GetObject;
impl OperationShape for GetObject {
    const NAME: &'static str = "GetObject";
    type Input = GetObjectRequest;
    type Output = GetObjectOutput;
    type Error = ();
}

pub type ErrorMessage = String;

pub type AnalyticsId = String;

#[derive(Debug, Default, Clone)]
pub struct GetBucketAccelerateConfiguration;
impl OperationShape for GetBucketAccelerateConfiguration {
    const NAME: &'static str = "GetBucketAccelerateConfiguration";
    type Input = GetBucketAccelerateConfigurationRequest;
    type Output = GetBucketAccelerateConfigurationOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketTaggingRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}
impl DeleteBucketTaggingRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type ReplicaKmsKeyId = String;

#[derive(Debug, Default, Clone)]
pub struct SelectObjectContent;
impl OperationShape for SelectObjectContent {
    const NAME: &'static str = "SelectObjectContent";
    type Input = SelectObjectContentRequest;
    type Output = SelectObjectContentOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct NoSuchKey {}
impl NoSuchKey {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type KmsContext = String;

#[derive(Debug, Default, Clone)]
pub struct LifecycleRuleAndOperator {
    pub prefix: Option<Prefix>,
    pub object_size_less_than: Option<ObjectSizeLessThanBytes>,
    pub object_size_greater_than: Option<ObjectSizeGreaterThanBytes>,
    pub tags: Option<TagSet>,
}
impl LifecycleRuleAndOperator {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type Uri = String;

#[derive(Debug, Default, Clone)]
pub struct AnalyticsS3BucketDestination {
    pub bucket: Option<BucketName>,
    pub bucket_account_id: Option<AccountId>,
    pub format: Option<AnalyticsS3ExportFileFormat>,
    pub prefix: Option<Prefix>,
}
impl AnalyticsS3BucketDestination {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type BucketName = String;

pub type Start = i64;

#[derive(Debug, Default, Clone)]
pub struct DefaultRetention {
    pub mode: Option<ObjectLockRetentionMode>,
    pub years: Option<Years>,
    pub days: Option<Days>,
}
impl DefaultRetention {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type RequestRoute = String;

pub type Date = String;

pub type LambdaFunctionArn = String;

#[derive(Debug, Default, Clone)]
pub struct NotFound {}
impl NotFound {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
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

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum QuoteFields {
    ASNEEDED,
    ALWAYS,
}
impl AsRef<str> for QuoteFields {
    fn as_ref(&self) -> &str {
        match self {
            Self::ASNEEDED => "ASNEEDED",
            Self::ALWAYS => "ALWAYS",
        }
    }
}
impl TryFrom<&str> for QuoteFields {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "ASNEEDED" => Ok(Self::ASNEEDED),
            "ALWAYS" => Ok(Self::ALWAYS),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
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

pub type Mfa = String;

#[derive(Debug, Clone)]
pub enum ReplicationRuleFilter {
    Prefix(Prefix),
    And(ReplicationRuleAndOperator),
    Tag(Tag),
}

pub type BucketKeyEnabled = bool;

pub type Delimiter = String;

#[derive(Debug, Default, Clone)]
pub struct NoncurrentVersionTransition {
    pub noncurrent_days: Option<Days>,
    pub storage_class: Option<TransitionStorageClass>,
    pub newer_noncurrent_versions: Option<VersionCount>,
}
impl NoncurrentVersionTransition {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct PutPublicAccessBlockRequest {
    pub bucket: Option<BucketName>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub expected_bucket_owner: Option<AccountId>,
    pub content_md5: Option<ContentMd5>,
    pub public_access_block_configuration: Option<PublicAccessBlockConfiguration>,
}
impl PutPublicAccessBlockRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct UploadPartCopy;
impl OperationShape for UploadPartCopy {
    const NAME: &'static str = "UploadPartCopy";
    type Input = UploadPartCopyRequest;
    type Output = UploadPartCopyOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct ListObjectsV2Request {
    pub continuation_token: Option<Token>,
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
    pub start_after: Option<StartAfter>,
    pub encoding_type: Option<EncodingType>,
    pub max_keys: Option<MaxKeys>,
    pub request_payer: Option<RequestPayer>,
    pub delimiter: Option<Delimiter>,
    pub fetch_owner: Option<FetchOwner>,
    pub prefix: Option<Prefix>,
}
impl ListObjectsV2Request {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type NextUploadIdMarker = String;

#[derive(Debug, Default, Clone)]
pub struct ObjectPart {
    pub checksum_sha256: Option<ChecksumSha256>,
    pub size: Option<Size>,
    pub part_number: Option<PartNumber>,
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
    pub checksum_sha1: Option<ChecksumSha1>,
    pub checksum_crc32: Option<ChecksumCrc32>,
}
impl ObjectPart {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type Suffix = String;

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketEncryptionRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}
impl DeleteBucketEncryptionRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct AbortMultipartUploadRequest {
    pub bucket: Option<BucketName>,
    pub request_payer: Option<RequestPayer>,
    pub upload_id: Option<MultipartUploadId>,
    pub expected_bucket_owner: Option<AccountId>,
    pub key: Option<ObjectKey>,
}
impl AbortMultipartUploadRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketInventoryConfigurationRequest {
    pub id: Option<InventoryId>,
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}
impl DeleteBucketInventoryConfigurationRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct AbortIncompleteMultipartUpload {
    pub days_after_initiation: Option<DaysAfterInitiation>,
}
impl AbortIncompleteMultipartUpload {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type ResponseExpires = String;

#[derive(Debug, Default, Clone)]
pub struct GetBucketCorsRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}
impl GetBucketCorsRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ErrorDocument {
    pub key: Option<ObjectKey>,
}
impl ErrorDocument {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketNotificationConfigurationRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}
impl GetBucketNotificationConfigurationRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
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

pub type Policy = String;

pub type ETag = String;

#[derive(Debug, Default, Clone)]
pub struct PutObjectLegalHold;
impl OperationShape for PutObjectLegalHold {
    const NAME: &'static str = "PutObjectLegalHold";
    type Input = PutObjectLegalHoldRequest;
    type Output = PutObjectLegalHoldOutput;
    type Error = ();
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
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

#[derive(Debug, Default, Clone)]
pub struct PutBucketMetricsConfigurationRequest {
    pub metrics_configuration: Option<MetricsConfiguration>,
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
    pub id: Option<MetricsId>,
}
impl PutBucketMetricsConfigurationRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct PutObjectRetention;
impl OperationShape for PutObjectRetention {
    const NAME: &'static str = "PutObjectRetention";
    type Input = PutObjectRetentionRequest;
    type Output = PutObjectRetentionOutput;
    type Error = ();
}

pub type FilterRuleValue = String;

#[derive(Debug, Default, Clone)]
pub struct ServerSideEncryptionByDefault {
    pub sse_algorithm: Option<ServerSideEncryption>,
    pub kms_master_key_id: Option<SsekmsKeyId>,
}
impl ServerSideEncryptionByDefault {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type VersionCount = i32;

#[derive(Debug, Default, Clone)]
pub struct GetBucketReplication;
impl OperationShape for GetBucketReplication {
    const NAME: &'static str = "GetBucketReplication";
    type Input = GetBucketReplicationRequest;
    type Output = GetBucketReplicationOutput;
    type Error = ();
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
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

pub type ChecksumSha1 = String;

#[derive(Debug, Default, Clone)]
pub struct GetBucketTagging;
impl OperationShape for GetBucketTagging {
    const NAME: &'static str = "GetBucketTagging";
    type Input = GetBucketTaggingRequest;
    type Output = GetBucketTaggingOutput;
    type Error = ();
}

pub type Expiration = String;

#[derive(Debug, Default, Clone)]
pub struct GetObjectAttributesOutput {
    pub version_id: Option<ObjectVersionId>,
    pub e_tag: Option<ETag>,
    pub checksum: Option<Checksum>,
    pub object_parts: Option<GetObjectAttributesParts>,
    pub object_size: Option<ObjectSize>,
    pub request_charged: Option<RequestCharged>,
    pub last_modified: Option<LastModified>,
    pub delete_marker: Option<DeleteMarker>,
    pub storage_class: Option<StorageClass>,
}
impl GetObjectAttributesOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum BucketLocationConstraint {
    sa_east_1,
    ap_northeast_2,
    ca_central_1,
    cn_north_1,
    eu_north_1,
    ap_south_1,
    ap_southeast_1,
    eu_west_1,
    eu_west_2,
    us_west_2,
    af_south_1,
    ap_southeast_3,
    ap_east_1,
    EU,
    ap_northeast_3,
    ap_southeast_2,
    me_south_1,
    eu_west_3,
    us_east_2,
    us_gov_east_1,
    us_gov_west_1,
    eu_central_1,
    cn_northwest_1,
    us_west_1,
    ap_northeast_1,
    eu_south_1,
}
impl AsRef<str> for BucketLocationConstraint {
    fn as_ref(&self) -> &str {
        match self {
            Self::sa_east_1 => "sa-east-1",
            Self::ap_northeast_2 => "ap-northeast-2",
            Self::ca_central_1 => "ca-central-1",
            Self::cn_north_1 => "cn-north-1",
            Self::eu_north_1 => "eu-north-1",
            Self::ap_south_1 => "ap-south-1",
            Self::ap_southeast_1 => "ap-southeast-1",
            Self::eu_west_1 => "eu-west-1",
            Self::eu_west_2 => "eu-west-2",
            Self::us_west_2 => "us-west-2",
            Self::af_south_1 => "af-south-1",
            Self::ap_southeast_3 => "ap-southeast-3",
            Self::ap_east_1 => "ap-east-1",
            Self::EU => "EU",
            Self::ap_northeast_3 => "ap-northeast-3",
            Self::ap_southeast_2 => "ap-southeast-2",
            Self::me_south_1 => "me-south-1",
            Self::eu_west_3 => "eu-west-3",
            Self::us_east_2 => "us-east-2",
            Self::us_gov_east_1 => "us-gov-east-1",
            Self::us_gov_west_1 => "us-gov-west-1",
            Self::eu_central_1 => "eu-central-1",
            Self::cn_northwest_1 => "cn-northwest-1",
            Self::us_west_1 => "us-west-1",
            Self::ap_northeast_1 => "ap-northeast-1",
            Self::eu_south_1 => "eu-south-1",
        }
    }
}
impl TryFrom<&str> for BucketLocationConstraint {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "sa-east-1" => Ok(Self::sa_east_1),
            "ap-northeast-2" => Ok(Self::ap_northeast_2),
            "ca-central-1" => Ok(Self::ca_central_1),
            "cn-north-1" => Ok(Self::cn_north_1),
            "eu-north-1" => Ok(Self::eu_north_1),
            "ap-south-1" => Ok(Self::ap_south_1),
            "ap-southeast-1" => Ok(Self::ap_southeast_1),
            "eu-west-1" => Ok(Self::eu_west_1),
            "eu-west-2" => Ok(Self::eu_west_2),
            "us-west-2" => Ok(Self::us_west_2),
            "af-south-1" => Ok(Self::af_south_1),
            "ap-southeast-3" => Ok(Self::ap_southeast_3),
            "ap-east-1" => Ok(Self::ap_east_1),
            "EU" => Ok(Self::EU),
            "ap-northeast-3" => Ok(Self::ap_northeast_3),
            "ap-southeast-2" => Ok(Self::ap_southeast_2),
            "me-south-1" => Ok(Self::me_south_1),
            "eu-west-3" => Ok(Self::eu_west_3),
            "us-east-2" => Ok(Self::us_east_2),
            "us-gov-east-1" => Ok(Self::us_gov_east_1),
            "us-gov-west-1" => Ok(Self::us_gov_west_1),
            "eu-central-1" => Ok(Self::eu_central_1),
            "cn-northwest-1" => Ok(Self::cn_northwest_1),
            "us-west-1" => Ok(Self::us_west_1),
            "ap-northeast-1" => Ok(Self::ap_northeast_1),
            "eu-south-1" => Ok(Self::eu_south_1),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

pub type Priority = i32;

#[derive(Debug, Default, Clone)]
pub struct PutObjectLockConfiguration;
impl OperationShape for PutObjectLockConfiguration {
    const NAME: &'static str = "PutObjectLockConfiguration";
    type Input = PutObjectLockConfigurationRequest;
    type Output = PutObjectLockConfigurationOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct PutObjectTaggingOutput {
    pub version_id: Option<ObjectVersionId>,
}
impl PutObjectTaggingOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketIntelligentTieringConfigurationOutput {
    pub intelligent_tiering_configuration: Option<IntelligentTieringConfiguration>,
}
impl GetBucketIntelligentTieringConfigurationOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketPolicyStatus;
impl OperationShape for GetBucketPolicyStatus {
    const NAME: &'static str = "GetBucketPolicyStatus";
    type Input = GetBucketPolicyStatusRequest;
    type Output = GetBucketPolicyStatusOutput;
    type Error = ();
}

pub type QueueConfigurationList = Vec<QueueConfiguration>;

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketWebsiteRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}
impl DeleteBucketWebsiteRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct TopicConfiguration {
    pub id: Option<NotificationId>,
    pub events: Option<EventList>,
    pub topic_arn: Option<TopicArn>,
    pub filter: Option<NotificationConfigurationFilter>,
}
impl TopicConfiguration {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetObjectAclRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
    pub key: Option<ObjectKey>,
    pub request_payer: Option<RequestPayer>,
    pub version_id: Option<ObjectVersionId>,
}
impl GetObjectAclRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketEncryptionOutput {
    pub server_side_encryption_configuration: Option<ServerSideEncryptionConfiguration>,
}
impl GetBucketEncryptionOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct PutBucketReplicationRequest {
    pub content_md5: Option<ContentMd5>,
    pub token: Option<ObjectLockToken>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub expected_bucket_owner: Option<AccountId>,
    pub replication_configuration: Option<ReplicationConfiguration>,
    pub bucket: Option<BucketName>,
}
impl PutBucketReplicationRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketPolicy;
impl OperationShape for GetBucketPolicy {
    const NAME: &'static str = "GetBucketPolicy";
    type Input = GetBucketPolicyRequest;
    type Output = GetBucketPolicyOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetObjectAcl;
impl OperationShape for GetObjectAcl {
    const NAME: &'static str = "GetObjectAcl";
    type Input = GetObjectAclRequest;
    type Output = GetObjectAclOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetObjectAttributes;
impl OperationShape for GetObjectAttributes {
    const NAME: &'static str = "GetObjectAttributes";
    type Input = GetObjectAttributesRequest;
    type Output = GetObjectAttributesOutput;
    type Error = ();
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
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
pub struct PutBucketTagging;
impl OperationShape for PutBucketTagging {
    const NAME: &'static str = "PutBucketTagging";
    type Input = PutBucketTaggingRequest;
    type Output = ();
    type Error = ();
}

pub type VersionIdMarker = String;

#[derive(Debug, Default, Clone)]
pub struct CopyObject;
impl OperationShape for CopyObject {
    const NAME: &'static str = "CopyObject";
    type Input = CopyObjectRequest;
    type Output = CopyObjectOutput;
    type Error = ();
}

pub type MaxAgeSeconds = i32;

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketLifecycleRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}
impl DeleteBucketLifecycleRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetPublicAccessBlockOutput {
    pub public_access_block_configuration: Option<PublicAccessBlockConfiguration>,
}
impl GetPublicAccessBlockOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type ResponseContentEncoding = String;

#[derive(Debug, Default, Clone)]
pub struct ParquetInput {}
impl ParquetInput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type ResponseCacheControl = String;

#[derive(Debug, Default, Clone)]
pub struct GetObjectRetentionOutput {
    pub retention: Option<ObjectLockRetention>,
}
impl GetObjectRetentionOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type UserMetadata = Vec<MetadataEntry>;

#[derive(Debug, Default, Clone)]
pub struct InventoryDestination {
    pub s3_bucket_destination: Option<InventoryS3BucketDestination>,
}
impl InventoryDestination {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketMetricsConfigurationRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
    pub id: Option<MetricsId>,
}
impl DeleteBucketMetricsConfigurationRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type RestoreOutputPath = String;

pub type CopySourceRange = String;

#[derive(Debug, Default, Clone)]
pub struct GetBucketOwnershipControlsRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}
impl GetBucketOwnershipControlsRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type IntelligentTieringId = String;

pub type ResponseContentType = String;

#[derive(Debug, Default, Clone)]
pub struct ListObjects;
impl OperationShape for ListObjects {
    const NAME: &'static str = "ListObjects";
    type Input = ListObjectsRequest;
    type Output = ListObjectsOutput;
    type Error = ();
}

pub type ExpiredObjectDeleteMarker = bool;

#[derive(Debug, Default, Clone)]
pub struct OutputLocation {
    pub s3: Option<S3Location>,
}
impl OutputLocation {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketAnalyticsConfiguration;
impl OperationShape for DeleteBucketAnalyticsConfiguration {
    const NAME: &'static str = "DeleteBucketAnalyticsConfiguration";
    type Input = DeleteBucketAnalyticsConfigurationRequest;
    type Output = ();
    type Error = ();
}

pub type MaxParts = i32;

pub type TargetPrefix = String;

#[derive(Debug, Default, Clone)]
pub struct DeleteObjects;
impl OperationShape for DeleteObjects {
    const NAME: &'static str = "DeleteObjects";
    type Input = DeleteObjectsRequest;
    type Output = DeleteObjectsOutput;
    type Error = ();
}

pub type ObjectSizeGreaterThanBytes = i64;

pub type End = i64;

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
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

#[derive(Debug, Default, Clone)]
pub struct UploadPartOutput {
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub checksum_sha256: Option<ChecksumSha256>,
    pub e_tag: Option<ETag>,
    pub checksum_sha1: Option<ChecksumSha1>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub checksum_crc32: Option<ChecksumCrc32>,
    pub request_charged: Option<RequestCharged>,
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
    pub ssekms_key_id: Option<SsekmsKeyId>,
    pub server_side_encryption: Option<ServerSideEncryption>,
}
impl UploadPartOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
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

#[derive(Debug, Default, Clone)]
pub struct GetBucketWebsiteRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}
impl GetBucketWebsiteRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type MissingMeta = i32;

pub type AnalyticsConfigurationList = Vec<AnalyticsConfiguration>;

#[derive(Debug, Default, Clone)]
pub struct CompleteMultipartUploadOutput {
    pub bucket: Option<BucketName>,
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
    pub checksum_crc32: Option<ChecksumCrc32>,
    pub location: Option<Location>,
    pub request_charged: Option<RequestCharged>,
    pub version_id: Option<ObjectVersionId>,
    pub checksum_sha1: Option<ChecksumSha1>,
    pub checksum_sha256: Option<ChecksumSha256>,
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    pub e_tag: Option<ETag>,
    pub key: Option<ObjectKey>,
    pub expiration: Option<Expiration>,
    pub ssekms_key_id: Option<SsekmsKeyId>,
    pub server_side_encryption: Option<ServerSideEncryption>,
}
impl CompleteMultipartUploadOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketAclRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}
impl GetBucketAclRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketLifecycleConfigurationOutput {
    pub rules: Option<LifecycleRules>,
}
impl GetBucketLifecycleConfigurationOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
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

pub type EnableRequestProgress = bool;

pub type Years = i32;

#[derive(Debug, Default, Clone)]
pub struct LoggingEnabled {
    pub target_grants: Option<TargetGrants>,
    pub target_prefix: Option<TargetPrefix>,
    pub target_bucket: Option<TargetBucket>,
}
impl LoggingEnabled {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type Range = String;

#[derive(Debug, Default, Clone)]
pub struct AccessControlTranslation {
    pub owner: Option<OwnerOverride>,
}
impl AccessControlTranslation {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListBucketMetricsConfigurations;
impl OperationShape for ListBucketMetricsConfigurations {
    const NAME: &'static str = "ListBucketMetricsConfigurations";
    type Input = ListBucketMetricsConfigurationsRequest;
    type Output = ListBucketMetricsConfigurationsOutput;
    type Error = ();
}

pub type MultipartUploadId = String;

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketEncryption;
impl OperationShape for DeleteBucketEncryption {
    const NAME: &'static str = "DeleteBucketEncryption";
    type Input = DeleteBucketEncryptionRequest;
    type Output = ();
    type Error = ();
}

pub type IntelligentTieringDays = i32;

#[derive(Debug, Default, Clone)]
pub struct FilterRule {
    pub name: Option<FilterRuleName>,
    pub value: Option<FilterRuleValue>,
}
impl FilterRule {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct PutBucketLogging;
impl OperationShape for PutBucketLogging {
    const NAME: &'static str = "PutBucketLogging";
    type Input = PutBucketLoggingRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketRequestPaymentRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}
impl GetBucketRequestPaymentRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type SsekmsKeyId = String;

#[derive(Debug, Default, Clone)]
pub struct Condition {
    pub http_error_code_returned_equals: Option<HttpErrorCodeReturnedEquals>,
    pub key_prefix_equals: Option<KeyPrefixEquals>,
}
impl Condition {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ExistingObjectReplication {
    pub status: Option<ExistingObjectReplicationStatus>,
}
impl ExistingObjectReplication {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type HttpErrorCodeReturnedEquals = String;

pub type MetadataValue = String;

#[derive(Debug, Default, Clone)]
pub struct UploadPartCopyRequest {
    pub copy_source_range: Option<CopySourceRange>,
    pub copy_source_if_match: Option<CopySourceIfMatch>,
    pub copy_source_if_none_match: Option<CopySourceIfNoneMatch>,
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
    pub copy_source_sse_customer_key_md5: Option<CopySourceSseCustomerKeyMd5>,
    pub key: Option<ObjectKey>,
    pub upload_id: Option<MultipartUploadId>,
    pub copy_source_sse_customer_key: Option<CopySourceSseCustomerKey>,
    pub expected_source_bucket_owner: Option<AccountId>,
    pub part_number: Option<PartNumber>,
    pub sse_customer_key: Option<SseCustomerKey>,
    pub request_payer: Option<RequestPayer>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub copy_source_if_unmodified_since: Option<CopySourceIfUnmodifiedSince>,
    pub copy_source_sse_customer_algorithm: Option<CopySourceSseCustomerAlgorithm>,
    pub copy_source_if_modified_since: Option<CopySourceIfModifiedSince>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub copy_source: Option<CopySource>,
}
impl UploadPartCopyRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type GrantWriteAcp = String;

pub type ObjectLockEnabledForBucket = bool;

pub type BypassGovernanceRetention = bool;

#[derive(Debug, Default, Clone)]
pub struct LifecycleRule {
    pub abort_incomplete_multipart_upload: Option<AbortIncompleteMultipartUpload>,
    pub expiration: Option<LifecycleExpiration>,
    pub prefix: Option<Prefix>,
    pub noncurrent_version_expiration: Option<NoncurrentVersionExpiration>,
    pub status: Option<ExpirationStatus>,
    pub transitions: Option<TransitionList>,
    pub id: Option<Id>,
    pub noncurrent_version_transitions: Option<NoncurrentVersionTransitionList>,
    pub filter: Option<LifecycleRuleFilter>,
}
impl LifecycleRule {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum FileHeaderInfo {
    IGNORE,
    USE,
    NONE,
}
impl AsRef<str> for FileHeaderInfo {
    fn as_ref(&self) -> &str {
        match self {
            Self::IGNORE => "IGNORE",
            Self::USE => "USE",
            Self::NONE => "NONE",
        }
    }
}
impl TryFrom<&str> for FileHeaderInfo {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "IGNORE" => Ok(Self::IGNORE),
            "USE" => Ok(Self::USE),
            "NONE" => Ok(Self::NONE),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

pub type FilterRuleList = Vec<FilterRule>;

pub type NoncurrentVersionTransitionList = Vec<NoncurrentVersionTransition>;

#[derive(Debug, Default, Clone)]
pub struct ListBucketAnalyticsConfigurations;
impl OperationShape for ListBucketAnalyticsConfigurations {
    const NAME: &'static str = "ListBucketAnalyticsConfigurations";
    type Input = ListBucketAnalyticsConfigurationsRequest;
    type Output = ListBucketAnalyticsConfigurationsOutput;
    type Error = ();
}

pub type Marker = String;

#[derive(Debug, Default, Clone)]
pub struct PutBucketWebsite;
impl OperationShape for PutBucketWebsite {
    const NAME: &'static str = "PutBucketWebsite";
    type Input = PutBucketWebsiteRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct BucketAlreadyOwnedByYou {}
impl BucketAlreadyOwnedByYou {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum MetadataDirective {
    REPLACE,
    COPY,
}
impl AsRef<str> for MetadataDirective {
    fn as_ref(&self) -> &str {
        match self {
            Self::REPLACE => "REPLACE",
            Self::COPY => "COPY",
        }
    }
}
impl TryFrom<&str> for MetadataDirective {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "REPLACE" => Ok(Self::REPLACE),
            "COPY" => Ok(Self::COPY),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Sses3 {}
impl Sses3 {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
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

#[derive(Debug, Default, Clone)]
pub struct DeleteObjectsOutput {
    pub request_charged: Option<RequestCharged>,
    pub errors: Option<Errors>,
    pub deleted: Option<DeletedObjects>,
}
impl DeleteObjectsOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketLoggingRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}
impl GetBucketLoggingRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketWebsiteOutput {
    pub index_document: Option<IndexDocument>,
    pub redirect_all_requests_to: Option<RedirectAllRequestsTo>,
    pub routing_rules: Option<RoutingRules>,
    pub error_document: Option<ErrorDocument>,
}
impl GetBucketWebsiteOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct OwnershipControls {
    pub rules: Option<OwnershipControlsRules>,
}
impl OwnershipControls {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetObjectAttributesRequest {
    pub key: Option<ObjectKey>,
    pub sse_customer_key: Option<SseCustomerKey>,
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
    pub max_parts: Option<MaxParts>,
    pub request_payer: Option<RequestPayer>,
    pub version_id: Option<ObjectVersionId>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub part_number_marker: Option<PartNumberMarker>,
    pub object_attributes: Option<ObjectAttributesList>,
}
impl GetObjectAttributesRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct PutBucketNotificationConfiguration;
impl OperationShape for PutBucketNotificationConfiguration {
    const NAME: &'static str = "PutBucketNotificationConfiguration";
    type Input = PutBucketNotificationConfigurationRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketRequestPayment;
impl OperationShape for GetBucketRequestPayment {
    const NAME: &'static str = "GetBucketRequestPayment";
    type Input = GetBucketRequestPaymentRequest;
    type Output = GetBucketRequestPaymentOutput;
    type Error = ();
}

pub type QuoteEscapeCharacter = String;

pub type SkipValidation = bool;

pub type CopySourceVersionId = String;

#[derive(Debug, Default, Clone)]
pub struct ListObjectsOutput {
    pub contents: Option<ObjectList>,
    pub delimiter: Option<Delimiter>,
    pub common_prefixes: Option<CommonPrefixList>,
    pub prefix: Option<Prefix>,
    pub max_keys: Option<MaxKeys>,
    pub next_marker: Option<NextMarker>,
    pub is_truncated: Option<IsTruncated>,
    pub encoding_type: Option<EncodingType>,
    pub marker: Option<Marker>,
    pub name: Option<BucketName>,
}
impl ListObjectsOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ServerSideEncryptionRule {
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    pub apply_server_side_encryption_by_default: Option<ServerSideEncryptionByDefault>,
}
impl ServerSideEncryptionRule {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type Role = String;

pub type WebsiteRedirectLocation = String;

#[derive(Debug, Default, Clone)]
pub struct PutBucketIntelligentTieringConfiguration;
impl OperationShape for PutBucketIntelligentTieringConfiguration {
    const NAME: &'static str = "PutBucketIntelligentTieringConfiguration";
    type Input = PutBucketIntelligentTieringConfigurationRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct EventBridgeConfiguration {}
impl EventBridgeConfiguration {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct SourceSelectionCriteria {
    pub replica_modifications: Option<ReplicaModifications>,
    pub sse_kms_encrypted_objects: Option<SseKmsEncryptedObjects>,
}
impl SourceSelectionCriteria {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type AllowedOrigin = String;

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
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

#[derive(Debug, Default, Clone)]
pub struct CompleteMultipartUploadRequest {
    pub sse_customer_key: Option<SseCustomerKey>,
    pub expected_bucket_owner: Option<AccountId>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub checksum_crc32: Option<ChecksumCrc32>,
    pub checksum_sha256: Option<ChecksumSha256>,
    pub multipart_upload: Option<CompletedMultipartUpload>,
    pub bucket: Option<BucketName>,
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
    pub checksum_sha1: Option<ChecksumSha1>,
    pub key: Option<ObjectKey>,
    pub request_payer: Option<RequestPayer>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub upload_id: Option<MultipartUploadId>,
}
impl CompleteMultipartUploadRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type InventoryOptionalFields = Vec<InventoryOptionalField>;

pub type LastModified = String;

#[derive(Debug, Default, Clone)]
pub struct Checksum {
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
    pub checksum_crc32: Option<ChecksumCrc32>,
    pub checksum_sha1: Option<ChecksumSha1>,
    pub checksum_sha256: Option<ChecksumSha256>,
}
impl Checksum {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum FilterRuleName {
    suffix,
    prefix,
}
impl AsRef<str> for FilterRuleName {
    fn as_ref(&self) -> &str {
        match self {
            Self::suffix => "suffix",
            Self::prefix => "prefix",
        }
    }
}
impl TryFrom<&str> for FilterRuleName {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "suffix" => Ok(Self::suffix),
            "prefix" => Ok(Self::prefix),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

pub type ContentType = String;

#[derive(Debug, Default, Clone)]
pub struct Ssekms {
    pub key_id: Option<SsekmsKeyId>,
}
impl Ssekms {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct CreateMultipartUploadRequest {
    pub object_lock_legal_hold_status: Option<ObjectLockLegalHoldStatus>,
    pub storage_class: Option<StorageClass>,
    pub request_payer: Option<RequestPayer>,
    pub content_type: Option<ContentType>,
    pub expires: Option<Expires>,
    pub object_lock_mode: Option<ObjectLockMode>,
    pub cache_control: Option<CacheControl>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub server_side_encryption: Option<ServerSideEncryption>,
    pub grant_read: Option<GrantRead>,
    pub object_lock_retain_until_date: Option<ObjectLockRetainUntilDate>,
    pub ssekms_encryption_context: Option<SsekmsEncryptionContext>,
    pub expected_bucket_owner: Option<AccountId>,
    pub grant_read_acp: Option<GrantReadAcp>,
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    pub ssekms_key_id: Option<SsekmsKeyId>,
    pub acl: Option<ObjectCannedAcl>,
    pub tagging: Option<TaggingHeader>,
    pub website_redirect_location: Option<WebsiteRedirectLocation>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub content_language: Option<ContentLanguage>,
    pub content_disposition: Option<ContentDisposition>,
    pub sse_customer_key: Option<SseCustomerKey>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub key: Option<ObjectKey>,
    pub content_encoding: Option<ContentEncoding>,
    pub grant_write_acp: Option<GrantWriteAcp>,
    pub metadata: Option<Metadata>,
    pub bucket: Option<BucketName>,
    pub grant_full_control: Option<GrantFullControl>,
}
impl CreateMultipartUploadRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketRequestPaymentOutput {
    pub payer: Option<Payer>,
}
impl GetBucketRequestPaymentOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Clone)]
pub enum LifecycleRuleFilter {
    And(LifecycleRuleAndOperator),
    Tag(Tag),
    ObjectSizeGreaterThan(ObjectSizeGreaterThanBytes),
    ObjectSizeLessThan(ObjectSizeLessThanBytes),
    Prefix(Prefix),
}

pub type PartsList = Vec<ObjectPart>;

pub type ObjectVersionList = Vec<ObjectVersion>;

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketIntelligentTieringConfigurationRequest {
    pub bucket: Option<BucketName>,
    pub id: Option<IntelligentTieringId>,
}
impl DeleteBucketIntelligentTieringConfigurationRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct PutObjectAclOutput {
    pub request_charged: Option<RequestCharged>,
}
impl PutObjectAclOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketIntelligentTieringConfiguration;
impl OperationShape for GetBucketIntelligentTieringConfiguration {
    const NAME: &'static str = "GetBucketIntelligentTieringConfiguration";
    type Input = GetBucketIntelligentTieringConfigurationRequest;
    type Output = GetBucketIntelligentTieringConfigurationOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketTaggingRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}
impl GetBucketTaggingRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListPartsRequest {
    pub request_payer: Option<RequestPayer>,
    pub expected_bucket_owner: Option<AccountId>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub sse_customer_key: Option<SseCustomerKey>,
    pub part_number_marker: Option<PartNumberMarker>,
    pub bucket: Option<BucketName>,
    pub upload_id: Option<MultipartUploadId>,
    pub max_parts: Option<MaxParts>,
    pub key: Option<ObjectKey>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
}
impl ListPartsRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct AnalyticsExportDestination {
    pub s3_bucket_destination: Option<AnalyticsS3BucketDestination>,
}
impl AnalyticsExportDestination {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct MetricsAndOperator {
    pub access_point_arn: Option<AccessPointArn>,
    pub prefix: Option<Prefix>,
    pub tags: Option<TagSet>,
}
impl MetricsAndOperator {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListMultipartUploadsOutput {
    pub upload_id_marker: Option<UploadIdMarker>,
    pub is_truncated: Option<IsTruncated>,
    pub key_marker: Option<KeyMarker>,
    pub bucket: Option<BucketName>,
    pub max_uploads: Option<MaxUploads>,
    pub next_upload_id_marker: Option<NextUploadIdMarker>,
    pub uploads: Option<MultipartUploadList>,
    pub encoding_type: Option<EncodingType>,
    pub common_prefixes: Option<CommonPrefixList>,
    pub prefix: Option<Prefix>,
    pub delimiter: Option<Delimiter>,
    pub next_key_marker: Option<NextKeyMarker>,
}
impl ListMultipartUploadsOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoSuchUpload {}
impl NoSuchUpload {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type OwnershipControlsRules = Vec<OwnershipControlsRule>;

pub type TargetGrants = Vec<TargetGrant>;

pub type DeletedObjects = Vec<DeletedObject>;

#[derive(Debug, Default, Clone)]
pub struct AnalyticsAndOperator {
    pub prefix: Option<Prefix>,
    pub tags: Option<TagSet>,
}
impl AnalyticsAndOperator {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type IfNoneMatch = String;

pub type Days = i32;

#[derive(Debug, Default, Clone)]
pub struct GetObjectTaggingRequest {
    pub request_payer: Option<RequestPayer>,
    pub expected_bucket_owner: Option<AccountId>,
    pub version_id: Option<ObjectVersionId>,
    pub key: Option<ObjectKey>,
    pub bucket: Option<BucketName>,
}
impl GetObjectTaggingRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
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

pub type GrantReadAcp = String;

#[derive(Debug, Default, Clone)]
pub struct GetObjectOutput {
    pub replication_status: Option<ReplicationStatus>,
    pub tag_count: Option<TagCount>,
    pub body: Option<StreamingBlob>,
    pub parts_count: Option<PartsCount>,
    pub delete_marker: Option<DeleteMarker>,
    pub e_tag: Option<ETag>,
    pub expires: Option<Expires>,
    pub object_lock_retain_until_date: Option<ObjectLockRetainUntilDate>,
    pub content_disposition: Option<ContentDisposition>,
    pub server_side_encryption: Option<ServerSideEncryption>,
    pub expiration: Option<Expiration>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub storage_class: Option<StorageClass>,
    pub content_language: Option<ContentLanguage>,
    pub object_lock_legal_hold_status: Option<ObjectLockLegalHoldStatus>,
    pub version_id: Option<ObjectVersionId>,
    pub ssekms_key_id: Option<SsekmsKeyId>,
    pub request_charged: Option<RequestCharged>,
    pub cache_control: Option<CacheControl>,
    pub last_modified: Option<LastModified>,
    pub checksum_sha1: Option<ChecksumSha1>,
    pub metadata: Option<Metadata>,
    pub missing_meta: Option<MissingMeta>,
    pub restore: Option<Restore>,
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
    pub content_length: Option<ContentLength>,
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    pub content_encoding: Option<ContentEncoding>,
    pub accept_ranges: Option<AcceptRanges>,
    pub content_type: Option<ContentType>,
    pub object_lock_mode: Option<ObjectLockMode>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub checksum_crc32: Option<ChecksumCrc32>,
    pub content_range: Option<ContentRange>,
    pub website_redirect_location: Option<WebsiteRedirectLocation>,
    pub checksum_sha256: Option<ChecksumSha256>,
}
impl GetObjectOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type DeleteMarkers = Vec<DeleteMarkerEntry>;

#[derive(Debug, Clone)]
pub enum AnalyticsFilter {
    Tag(Tag),
    And(AnalyticsAndOperator),
    Prefix(Prefix),
}

#[derive(Debug, Default, Clone)]
pub struct RestoreRequest {
    pub days: Option<Days>,
    pub description: Option<Description>,
    pub glacier_job_parameters: Option<GlacierJobParameters>,
    pub output_location: Option<OutputLocation>,
    pub tier: Option<Tier>,
    pub select_parameters: Option<SelectParameters>,
    pub r#type: Option<RestoreRequestType>,
}
impl RestoreRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct Grant {
    pub grantee: Option<Grantee>,
    pub permission: Option<Permission>,
}
impl Grant {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct DeleteObjectTaggingOutput {
    pub version_id: Option<ObjectVersionId>,
}
impl DeleteObjectTaggingOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type MetricsId = String;

pub type Code = String;

#[derive(Debug, Default, Clone)]
pub struct PutObjectLegalHoldOutput {
    pub request_charged: Option<RequestCharged>,
}
impl PutObjectLegalHoldOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type AllowedOrigins = Vec<AllowedOrigin>;

#[derive(Debug, Default, Clone)]
pub struct GetBucketAnalyticsConfigurationRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
    pub id: Option<AnalyticsId>,
}
impl GetBucketAnalyticsConfigurationRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct PutBucketLoggingRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
    pub bucket_logging_status: Option<BucketLoggingStatus>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub content_md5: Option<ContentMd5>,
}
impl PutBucketLoggingRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type AllowedHeader = String;

#[derive(Debug, Default, Clone)]
pub struct PutObjectRetentionRequest {
    pub version_id: Option<ObjectVersionId>,
    pub bypass_governance_retention: Option<BypassGovernanceRetention>,
    pub request_payer: Option<RequestPayer>,
    pub content_md5: Option<ContentMd5>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub expected_bucket_owner: Option<AccountId>,
    pub retention: Option<ObjectLockRetention>,
    pub key: Option<ObjectKey>,
    pub bucket: Option<BucketName>,
}
impl PutObjectRetentionRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct BucketLoggingStatus {
    pub logging_enabled: Option<LoggingEnabled>,
}
impl BucketLoggingStatus {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketWebsite;
impl OperationShape for DeleteBucketWebsite {
    const NAME: &'static str = "DeleteBucketWebsite";
    type Input = DeleteBucketWebsiteRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct DeleteObject;
impl OperationShape for DeleteObject {
    const NAME: &'static str = "DeleteObject";
    type Input = DeleteObjectRequest;
    type Output = DeleteObjectOutput;
    type Error = ();
}

pub type MetricsConfigurationList = Vec<MetricsConfiguration>;

#[derive(Debug, Default, Clone)]
pub struct ListBucketIntelligentTieringConfigurations;
impl OperationShape for ListBucketIntelligentTieringConfigurations {
    const NAME: &'static str = "ListBucketIntelligentTieringConfigurations";
    type Input = ListBucketIntelligentTieringConfigurationsRequest;
    type Output = ListBucketIntelligentTieringConfigurationsOutput;
    type Error = ();
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum TransitionStorageClass {
    DEEP_ARCHIVE,
    GLACIER_IR,
    INTELLIGENT_TIERING,
    ONEZONE_IA,
    GLACIER,
    STANDARD_IA,
}
impl AsRef<str> for TransitionStorageClass {
    fn as_ref(&self) -> &str {
        match self {
            Self::DEEP_ARCHIVE => "DEEP_ARCHIVE",
            Self::GLACIER_IR => "GLACIER_IR",
            Self::INTELLIGENT_TIERING => "INTELLIGENT_TIERING",
            Self::ONEZONE_IA => "ONEZONE_IA",
            Self::GLACIER => "GLACIER",
            Self::STANDARD_IA => "STANDARD_IA",
        }
    }
}
impl TryFrom<&str> for TransitionStorageClass {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "DEEP_ARCHIVE" => Ok(Self::DEEP_ARCHIVE),
            "GLACIER_IR" => Ok(Self::GLACIER_IR),
            "INTELLIGENT_TIERING" => Ok(Self::INTELLIGENT_TIERING),
            "ONEZONE_IA" => Ok(Self::ONEZONE_IA),
            "GLACIER" => Ok(Self::GLACIER),
            "STANDARD_IA" => Ok(Self::STANDARD_IA),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct NotificationConfigurationFilter {
    pub key: Option<S3KeyFilter>,
}
impl NotificationConfigurationFilter {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type AllowedMethods = Vec<AllowedMethod>;

#[derive(Debug, Default, Clone)]
pub struct PutBucketNotificationConfigurationRequest {
    pub skip_destination_validation: Option<SkipValidation>,
    pub expected_bucket_owner: Option<AccountId>,
    pub notification_configuration: Option<NotificationConfiguration>,
    pub bucket: Option<BucketName>,
}
impl PutBucketNotificationConfigurationRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type ContentDisposition = String;

pub type Restore = String;

pub type AllowQuotedRecordDelimiter = bool;

#[derive(Debug, Default, Clone)]
pub struct Bucket {
    pub creation_date: Option<CreationDate>,
    pub name: Option<BucketName>,
}
impl Bucket {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type ConfirmRemoveSelfBucketAccess = bool;

pub type ExposeHeaders = Vec<ExposeHeader>;

#[derive(Debug, Default, Clone)]
pub struct GlacierJobParameters {
    pub tier: Option<Tier>,
}
impl GlacierJobParameters {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ObjectIdentifier {
    pub version_id: Option<ObjectVersionId>,
    pub key: Option<ObjectKey>,
}
impl ObjectIdentifier {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetObjectLegalHoldOutput {
    pub legal_hold: Option<ObjectLockLegalHold>,
}
impl GetObjectLegalHoldOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct Grantee {
    pub id: Option<Id>,
    pub uri: Option<Uri>,
    pub display_name: Option<DisplayName>,
    pub r#type: Option<Type>,
    pub email_address: Option<EmailAddress>,
}
impl Grantee {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type MultipartUploadList = Vec<MultipartUpload>;

pub type QuoteCharacter = String;

#[derive(Debug, Default, Clone)]
pub struct PutBucketReplication;
impl OperationShape for PutBucketReplication {
    const NAME: &'static str = "PutBucketReplication";
    type Input = PutBucketReplicationRequest;
    type Output = ();
    type Error = ();
}

pub type ReplicationRules = Vec<ReplicationRule>;

pub type AccountId = String;

#[derive(Debug, Default, Clone)]
pub struct GetBucketAccelerateConfigurationRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}
impl GetBucketAccelerateConfigurationRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type ObjectAttributesList = Vec<ObjectAttributes>;

#[derive(Debug, Default, Clone)]
pub struct GetObjectRequest {
    pub response_content_language: Option<ResponseContentLanguage>,
    pub if_none_match: Option<IfNoneMatch>,
    pub part_number: Option<PartNumber>,
    pub sse_customer_key: Option<SseCustomerKey>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub checksum_mode: Option<ChecksumMode>,
    pub request_payer: Option<RequestPayer>,
    pub bucket: Option<BucketName>,
    pub range: Option<Range>,
    pub key: Option<ObjectKey>,
    pub response_expires: Option<ResponseExpires>,
    pub if_match: Option<IfMatch>,
    pub response_content_disposition: Option<ResponseContentDisposition>,
    pub response_content_encoding: Option<ResponseContentEncoding>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub version_id: Option<ObjectVersionId>,
    pub expected_bucket_owner: Option<AccountId>,
    pub if_modified_since: Option<IfModifiedSince>,
    pub response_cache_control: Option<ResponseCacheControl>,
    pub response_content_type: Option<ResponseContentType>,
    pub if_unmodified_since: Option<IfUnmodifiedSince>,
}
impl GetObjectRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetObjectTorrentRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
    pub key: Option<ObjectKey>,
    pub request_payer: Option<RequestPayer>,
}
impl GetObjectTorrentRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type ObjectSizeLessThanBytes = i64;

#[derive(Debug, Default, Clone)]
pub struct Metrics {
    pub event_threshold: Option<ReplicationTimeValue>,
    pub status: Option<MetricsStatus>,
}
impl Metrics {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type ContentLength = i64;

#[derive(Debug, Default, Clone)]
pub struct CreateMultipartUploadOutput {
    pub ssekms_key_id: Option<SsekmsKeyId>,
    pub abort_date: Option<AbortDate>,
    pub request_charged: Option<RequestCharged>,
    pub abort_rule_id: Option<AbortRuleId>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub bucket: Option<BucketName>,
    pub key: Option<ObjectKey>,
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub ssekms_encryption_context: Option<SsekmsEncryptionContext>,
    pub server_side_encryption: Option<ServerSideEncryption>,
    pub upload_id: Option<MultipartUploadId>,
}
impl CreateMultipartUploadOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type PartNumberMarker = String;

#[derive(Debug, Default, Clone)]
pub struct PutBucketTaggingRequest {
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub content_md5: Option<ContentMd5>,
    pub tagging: Option<Tagging>,
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}
impl PutBucketTaggingRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type BytesReturned = i64;

pub type Expression = String;

#[derive(Debug, Default, Clone)]
pub struct GetPublicAccessBlock;
impl OperationShape for GetPublicAccessBlock {
    const NAME: &'static str = "GetPublicAccessBlock";
    type Input = GetPublicAccessBlockRequest;
    type Output = GetPublicAccessBlockOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct InvalidObjectState {
    pub storage_class: Option<StorageClass>,
    pub access_tier: Option<IntelligentTieringAccessTier>,
}
impl InvalidObjectState {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
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
pub struct MetricsConfiguration {
    pub filter: Option<MetricsFilter>,
    pub id: Option<MetricsId>,
}
impl MetricsConfiguration {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct SseKmsEncryptedObjects {
    pub status: Option<SseKmsEncryptedObjectsStatus>,
}
impl SseKmsEncryptedObjects {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct IntelligentTieringConfiguration {
    pub tierings: Option<TieringList>,
    pub filter: Option<IntelligentTieringFilter>,
    pub id: Option<IntelligentTieringId>,
    pub status: Option<IntelligentTieringStatus>,
}
impl IntelligentTieringConfiguration {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type CopySource = String;

#[derive(Debug, Default, Clone)]
pub struct Destination {
    pub metrics: Option<Metrics>,
    pub replication_time: Option<ReplicationTime>,
    pub access_control_translation: Option<AccessControlTranslation>,
    pub bucket: Option<BucketName>,
    pub encryption_configuration: Option<EncryptionConfiguration>,
    pub storage_class: Option<StorageClass>,
    pub account: Option<AccountId>,
}
impl Destination {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum InventoryFrequency {
    Weekly,
    Daily,
}
impl AsRef<str> for InventoryFrequency {
    fn as_ref(&self) -> &str {
        match self {
            Self::Weekly => "Weekly",
            Self::Daily => "Daily",
        }
    }
}
impl TryFrom<&str> for InventoryFrequency {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "Weekly" => Ok(Self::Weekly),
            "Daily" => Ok(Self::Daily),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum InventoryOptionalField {
    EncryptionStatus,
    IsMultipartUploaded,
    ObjectLockLegalHoldStatus,
    ChecksumAlgorithm,
    Size,
    ETag,
    BucketKeyStatus,
    ReplicationStatus,
    IntelligentTieringAccessTier,
    LastModifiedDate,
    ObjectLockRetainUntilDate,
    ObjectLockMode,
    StorageClass,
}
impl AsRef<str> for InventoryOptionalField {
    fn as_ref(&self) -> &str {
        match self {
            Self::EncryptionStatus => "EncryptionStatus",
            Self::IsMultipartUploaded => "IsMultipartUploaded",
            Self::ObjectLockLegalHoldStatus => "ObjectLockLegalHoldStatus",
            Self::ChecksumAlgorithm => "ChecksumAlgorithm",
            Self::Size => "Size",
            Self::ETag => "ETag",
            Self::BucketKeyStatus => "BucketKeyStatus",
            Self::ReplicationStatus => "ReplicationStatus",
            Self::IntelligentTieringAccessTier => "IntelligentTieringAccessTier",
            Self::LastModifiedDate => "LastModifiedDate",
            Self::ObjectLockRetainUntilDate => "ObjectLockRetainUntilDate",
            Self::ObjectLockMode => "ObjectLockMode",
            Self::StorageClass => "StorageClass",
        }
    }
}
impl TryFrom<&str> for InventoryOptionalField {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "EncryptionStatus" => Ok(Self::EncryptionStatus),
            "IsMultipartUploaded" => Ok(Self::IsMultipartUploaded),
            "ObjectLockLegalHoldStatus" => Ok(Self::ObjectLockLegalHoldStatus),
            "ChecksumAlgorithm" => Ok(Self::ChecksumAlgorithm),
            "Size" => Ok(Self::Size),
            "ETag" => Ok(Self::ETag),
            "BucketKeyStatus" => Ok(Self::BucketKeyStatus),
            "ReplicationStatus" => Ok(Self::ReplicationStatus),
            "IntelligentTieringAccessTier" => Ok(Self::IntelligentTieringAccessTier),
            "LastModifiedDate" => Ok(Self::LastModifiedDate),
            "ObjectLockRetainUntilDate" => Ok(Self::ObjectLockRetainUntilDate),
            "ObjectLockMode" => Ok(Self::ObjectLockMode),
            "StorageClass" => Ok(Self::StorageClass),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct JsonInput {
    pub r#type: Option<JsonType>,
}
impl JsonInput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct Tagging {
    pub tag_set: Option<TagSet>,
}
impl Tagging {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type Quiet = bool;

pub type DaysAfterInitiation = i32;

#[derive(Debug, Default, Clone)]
pub struct PutBucketOwnershipControlsRequest {
    pub ownership_controls: Option<OwnershipControls>,
    pub content_md5: Option<ContentMd5>,
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}
impl PutBucketOwnershipControlsRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketAclOutput {
    pub grants: Option<Grants>,
    pub owner: Option<Owner>,
}
impl GetBucketAclOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type CopySourceIfMatch = String;

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
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

#[derive(Debug, Default, Clone)]
pub struct StorageClassAnalysisDataExport {
    pub output_schema_version: Option<StorageClassAnalysisSchemaVersion>,
    pub destination: Option<AnalyticsExportDestination>,
}
impl StorageClassAnalysisDataExport {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type LifecycleRules = Vec<LifecycleRule>;

pub type Size = i64;

#[derive(Debug, Default, Clone)]
pub struct GetBucketPolicyOutput {
    pub policy: Option<Policy>,
}
impl GetBucketPolicyOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type IntelligentTieringConfigurationList = Vec<IntelligentTieringConfiguration>;

pub type NextKeyMarker = String;

pub type ChecksumAlgorithmList = Vec<ChecksumAlgorithm>;

pub type NextVersionIdMarker = String;

#[derive(Debug, Default, Clone)]
pub struct PutBucketAccelerateConfigurationRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub bucket: Option<BucketName>,
    pub accelerate_configuration: Option<AccelerateConfiguration>,
}
impl PutBucketAccelerateConfigurationRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketLifecycle;
impl OperationShape for DeleteBucketLifecycle {
    const NAME: &'static str = "DeleteBucketLifecycle";
    type Input = DeleteBucketLifecycleRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct Error {
    pub message: Option<Message>,
    pub code: Option<Code>,
    pub version_id: Option<ObjectVersionId>,
    pub key: Option<ObjectKey>,
}
impl Error {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type LambdaFunctionConfigurationList = Vec<LambdaFunctionConfiguration>;

#[derive(Debug, Clone)]
pub enum MetricsFilter {
    AccessPointArn(AccessPointArn),
    Tag(Tag),
    And(MetricsAndOperator),
    Prefix(Prefix),
}

#[derive(Debug, Default, Clone)]
pub struct PutObjectTaggingRequest {
    pub bucket: Option<BucketName>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub request_payer: Option<RequestPayer>,
    pub expected_bucket_owner: Option<AccountId>,
    pub tagging: Option<Tagging>,
    pub content_md5: Option<ContentMd5>,
    pub key: Option<ObjectKey>,
    pub version_id: Option<ObjectVersionId>,
}
impl PutObjectTaggingRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct AmazonS3 {
    pub delete_bucket_policy: Option<OpService<DeleteBucketPolicy>>,
    pub get_bucket_intelligent_tiering_configuration:
        Option<OpService<GetBucketIntelligentTieringConfiguration>>,
    pub put_bucket_replication: Option<OpService<PutBucketReplication>>,
    pub get_bucket_accelerate_configuration: Option<OpService<GetBucketAccelerateConfiguration>>,
    pub delete_bucket_ownership_controls: Option<OpService<DeleteBucketOwnershipControls>>,
    pub delete_bucket_tagging: Option<OpService<DeleteBucketTagging>>,
    pub delete_objects: Option<OpService<DeleteObjects>>,
    pub get_bucket_encryption: Option<OpService<GetBucketEncryption>>,
    pub get_object_retention: Option<OpService<GetObjectRetention>>,
    pub get_bucket_policy_status: Option<OpService<GetBucketPolicyStatus>>,
    pub list_bucket_intelligent_tiering_configurations:
        Option<OpService<ListBucketIntelligentTieringConfigurations>>,
    pub copy_object: Option<OpService<CopyObject>>,
    pub delete_bucket_analytics_configuration:
        Option<OpService<DeleteBucketAnalyticsConfiguration>>,
    pub get_bucket_metrics_configuration: Option<OpService<GetBucketMetricsConfiguration>>,
    pub delete_bucket_encryption: Option<OpService<DeleteBucketEncryption>>,
    pub get_bucket_notification_configuration:
        Option<OpService<GetBucketNotificationConfiguration>>,
    pub get_object_tagging: Option<OpService<GetObjectTagging>>,
    pub get_bucket_ownership_controls: Option<OpService<GetBucketOwnershipControls>>,
    pub put_bucket_cors: Option<OpService<PutBucketCors>>,
    pub put_bucket_request_payment: Option<OpService<PutBucketRequestPayment>>,
    pub head_bucket: Option<OpService<HeadBucket>>,
    pub delete_bucket_replication: Option<OpService<DeleteBucketReplication>>,
    pub delete_object: Option<OpService<DeleteObject>>,
    pub put_bucket_tagging: Option<OpService<PutBucketTagging>>,
    pub put_object: Option<OpService<PutObject>>,
    pub put_object_retention: Option<OpService<PutObjectRetention>>,
    pub head_object: Option<OpService<HeadObject>>,
    pub get_object_lock_configuration: Option<OpService<GetObjectLockConfiguration>>,
    pub list_parts: Option<OpService<ListParts>>,
    pub delete_bucket_lifecycle: Option<OpService<DeleteBucketLifecycle>>,
    pub delete_bucket_metrics_configuration: Option<OpService<DeleteBucketMetricsConfiguration>>,
    pub get_bucket_location: Option<OpService<GetBucketLocation>>,
    pub list_bucket_inventory_configurations: Option<OpService<ListBucketInventoryConfigurations>>,
    pub delete_public_access_block: Option<OpService<DeletePublicAccessBlock>>,
    pub get_bucket_inventory_configuration: Option<OpService<GetBucketInventoryConfiguration>>,
    pub get_bucket_versioning: Option<OpService<GetBucketVersioning>>,
    pub get_bucket_website: Option<OpService<GetBucketWebsite>>,
    pub get_bucket_policy: Option<OpService<GetBucketPolicy>>,
    pub put_bucket_policy: Option<OpService<PutBucketPolicy>>,
    pub delete_bucket_website: Option<OpService<DeleteBucketWebsite>>,
    pub delete_bucket_intelligent_tiering_configuration:
        Option<OpService<DeleteBucketIntelligentTieringConfiguration>>,
    pub put_object_lock_configuration: Option<OpService<PutObjectLockConfiguration>>,
    pub delete_object_tagging: Option<OpService<DeleteObjectTagging>>,
    pub put_bucket_ownership_controls: Option<OpService<PutBucketOwnershipControls>>,
    pub get_public_access_block: Option<OpService<GetPublicAccessBlock>>,
    pub upload_part: Option<OpService<UploadPart>>,
    pub list_bucket_analytics_configurations: Option<OpService<ListBucketAnalyticsConfigurations>>,
    pub list_multipart_uploads: Option<OpService<ListMultipartUploads>>,
    pub upload_part_copy: Option<OpService<UploadPartCopy>>,
    pub write_get_object_response: Option<OpService<WriteGetObjectResponse>>,
    pub put_bucket_analytics_configuration: Option<OpService<PutBucketAnalyticsConfiguration>>,
    pub get_bucket_analytics_configuration: Option<OpService<GetBucketAnalyticsConfiguration>>,
    pub put_object_legal_hold: Option<OpService<PutObjectLegalHold>>,
    pub list_bucket_metrics_configurations: Option<OpService<ListBucketMetricsConfigurations>>,
    pub put_bucket_acl: Option<OpService<PutBucketAcl>>,
    pub put_bucket_encryption: Option<OpService<PutBucketEncryption>>,
    pub list_objects: Option<OpService<ListObjects>>,
    pub put_bucket_metrics_configuration: Option<OpService<PutBucketMetricsConfiguration>>,
    pub delete_bucket_cors: Option<OpService<DeleteBucketCors>>,
    pub put_bucket_versioning: Option<OpService<PutBucketVersioning>>,
    pub put_public_access_block: Option<OpService<PutPublicAccessBlock>>,
    pub put_bucket_website: Option<OpService<PutBucketWebsite>>,
    pub put_bucket_lifecycle_configuration: Option<OpService<PutBucketLifecycleConfiguration>>,
    pub get_bucket_tagging: Option<OpService<GetBucketTagging>>,
    pub get_bucket_lifecycle_configuration: Option<OpService<GetBucketLifecycleConfiguration>>,
    pub delete_bucket: Option<OpService<DeleteBucket>>,
    pub get_bucket_replication: Option<OpService<GetBucketReplication>>,
    pub put_bucket_notification_configuration:
        Option<OpService<PutBucketNotificationConfiguration>>,
    pub put_bucket_intelligent_tiering_configuration:
        Option<OpService<PutBucketIntelligentTieringConfiguration>>,
    pub create_multipart_upload: Option<OpService<CreateMultipartUpload>>,
    pub put_bucket_inventory_configuration: Option<OpService<PutBucketInventoryConfiguration>>,
    pub get_bucket_request_payment: Option<OpService<GetBucketRequestPayment>>,
    pub get_object: Option<OpService<GetObject>>,
    pub get_object_acl: Option<OpService<GetObjectAcl>>,
    pub get_object_legal_hold: Option<OpService<GetObjectLegalHold>>,
    pub complete_multipart_upload: Option<OpService<CompleteMultipartUpload>>,
    pub get_object_torrent: Option<OpService<GetObjectTorrent>>,
    pub get_bucket_cors: Option<OpService<GetBucketCors>>,
    pub create_bucket: Option<OpService<CreateBucket>>,
    pub list_object_versions: Option<OpService<ListObjectVersions>>,
    pub put_bucket_logging: Option<OpService<PutBucketLogging>>,
    pub put_object_tagging: Option<OpService<PutObjectTagging>>,
    pub get_object_attributes: Option<OpService<GetObjectAttributes>>,
    pub list_buckets: Option<OpService<ListBuckets>>,
    pub put_object_acl: Option<OpService<PutObjectAcl>>,
    pub get_bucket_logging: Option<OpService<GetBucketLogging>>,
    pub get_bucket_acl: Option<OpService<GetBucketAcl>>,
    pub put_bucket_accelerate_configuration: Option<OpService<PutBucketAccelerateConfiguration>>,
    pub restore_object: Option<OpService<RestoreObject>>,
    pub select_object_content: Option<OpService<SelectObjectContent>>,
    pub delete_bucket_inventory_configuration:
        Option<OpService<DeleteBucketInventoryConfiguration>>,
    pub list_objects_v2: Option<OpService<ListObjectsV2>>,
    pub abort_multipart_upload: Option<OpService<AbortMultipartUpload>>,
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum ReplicaModificationsStatus {
    Disabled,
    Enabled,
}
impl AsRef<str> for ReplicaModificationsStatus {
    fn as_ref(&self) -> &str {
        match self {
            Self::Disabled => "Disabled",
            Self::Enabled => "Enabled",
        }
    }
}
impl TryFrom<&str> for ReplicaModificationsStatus {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "Disabled" => Ok(Self::Disabled),
            "Enabled" => Ok(Self::Enabled),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

pub type GetObjectResponseStatusCode = i32;

pub type NextPartNumberMarker = String;

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum StorageClass {
    ONEZONE_IA,
    INTELLIGENT_TIERING,
    REDUCED_REDUNDANCY,
    DEEP_ARCHIVE,
    STANDARD,
    OUTPOSTS,
    GLACIER_IR,
    STANDARD_IA,
    GLACIER,
}
impl AsRef<str> for StorageClass {
    fn as_ref(&self) -> &str {
        match self {
            Self::ONEZONE_IA => "ONEZONE_IA",
            Self::INTELLIGENT_TIERING => "INTELLIGENT_TIERING",
            Self::REDUCED_REDUNDANCY => "REDUCED_REDUNDANCY",
            Self::DEEP_ARCHIVE => "DEEP_ARCHIVE",
            Self::STANDARD => "STANDARD",
            Self::OUTPOSTS => "OUTPOSTS",
            Self::GLACIER_IR => "GLACIER_IR",
            Self::STANDARD_IA => "STANDARD_IA",
            Self::GLACIER => "GLACIER",
        }
    }
}
impl TryFrom<&str> for StorageClass {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "ONEZONE_IA" => Ok(Self::ONEZONE_IA),
            "INTELLIGENT_TIERING" => Ok(Self::INTELLIGENT_TIERING),
            "REDUCED_REDUNDANCY" => Ok(Self::REDUCED_REDUNDANCY),
            "DEEP_ARCHIVE" => Ok(Self::DEEP_ARCHIVE),
            "STANDARD" => Ok(Self::STANDARD),
            "OUTPOSTS" => Ok(Self::OUTPOSTS),
            "GLACIER_IR" => Ok(Self::GLACIER_IR),
            "STANDARD_IA" => Ok(Self::STANDARD_IA),
            "GLACIER" => Ok(Self::GLACIER),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct BucketAlreadyExists {}
impl BucketAlreadyExists {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct PutObjectAcl;
impl OperationShape for PutObjectAcl {
    const NAME: &'static str = "PutObjectAcl";
    type Input = PutObjectAclRequest;
    type Output = PutObjectAclOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketInventoryConfigurationOutput {
    pub inventory_configuration: Option<InventoryConfiguration>,
}
impl GetBucketInventoryConfigurationOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type Minutes = i32;

#[derive(Debug, Default, Clone)]
pub struct GetBucketLoggingOutput {
    pub logging_enabled: Option<LoggingEnabled>,
}
impl GetBucketLoggingOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct DeleteObjectTagging;
impl OperationShape for DeleteObjectTagging {
    const NAME: &'static str = "DeleteObjectTagging";
    type Input = DeleteObjectTaggingRequest;
    type Output = DeleteObjectTaggingOutput;
    type Error = ();
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
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

pub type AbortRuleId = String;

#[derive(Debug, Default, Clone)]
pub struct ListBucketInventoryConfigurations;
impl OperationShape for ListBucketInventoryConfigurations {
    const NAME: &'static str = "ListBucketInventoryConfigurations";
    type Input = ListBucketInventoryConfigurationsRequest;
    type Output = ListBucketInventoryConfigurationsOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct ObjectNotInActiveTierError {}
impl ObjectNotInActiveTierError {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum ReplicationStatus {
    PENDING,
    REPLICA,
    COMPLETE,
    FAILED,
}
impl AsRef<str> for ReplicationStatus {
    fn as_ref(&self) -> &str {
        match self {
            Self::PENDING => "PENDING",
            Self::REPLICA => "REPLICA",
            Self::COMPLETE => "COMPLETE",
            Self::FAILED => "FAILED",
        }
    }
}
impl TryFrom<&str> for ReplicationStatus {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "PENDING" => Ok(Self::PENDING),
            "REPLICA" => Ok(Self::REPLICA),
            "COMPLETE" => Ok(Self::COMPLETE),
            "FAILED" => Ok(Self::FAILED),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct TargetGrant {
    pub grantee: Option<Grantee>,
    pub permission: Option<BucketLogsPermission>,
}
impl TargetGrant {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetObjectTagging;
impl OperationShape for GetObjectTagging {
    const NAME: &'static str = "GetObjectTagging";
    type Input = GetObjectTaggingRequest;
    type Output = GetObjectTaggingOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct JsonOutput {
    pub record_delimiter: Option<RecordDelimiter>,
}
impl JsonOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketReplicationRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}
impl GetBucketReplicationRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type AbortDate = String;

pub type Comments = String;

pub type ResponseContentLanguage = String;

#[derive(Debug, Default, Clone)]
pub struct GetBucketPolicyStatusRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}
impl GetBucketPolicyStatusRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct StorageClassAnalysis {
    pub data_export: Option<StorageClassAnalysisDataExport>,
}
impl StorageClassAnalysis {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct PutBucketAclRequest {
    pub content_md5: Option<ContentMd5>,
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
    pub grant_read_acp: Option<GrantReadAcp>,
    pub grant_read: Option<GrantRead>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub grant_write: Option<GrantWrite>,
    pub access_control_policy: Option<AccessControlPolicy>,
    pub acl: Option<BucketCannedAcl>,
    pub grant_write_acp: Option<GrantWriteAcp>,
    pub grant_full_control: Option<GrantFullControl>,
}
impl PutBucketAclRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type DisplayName = String;

pub type KeyMarker = String;

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
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

#[derive(Debug, Default, Clone)]
pub struct HeadBucketRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}
impl HeadBucketRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct HeadObjectRequest {
    pub checksum_mode: Option<ChecksumMode>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub if_none_match: Option<IfNoneMatch>,
    pub sse_customer_key: Option<SseCustomerKey>,
    pub request_payer: Option<RequestPayer>,
    pub expected_bucket_owner: Option<AccountId>,
    pub if_match: Option<IfMatch>,
    pub key: Option<ObjectKey>,
    pub part_number: Option<PartNumber>,
    pub if_modified_since: Option<IfModifiedSince>,
    pub version_id: Option<ObjectVersionId>,
    pub if_unmodified_since: Option<IfUnmodifiedSince>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub bucket: Option<BucketName>,
    pub range: Option<Range>,
}
impl HeadObjectRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type NextMarker = String;

#[derive(Debug, Default, Clone)]
pub struct S3Location {
    pub tagging: Option<Tagging>,
    pub bucket_name: Option<BucketName>,
    pub user_metadata: Option<UserMetadata>,
    pub canned_acl: Option<ObjectCannedAcl>,
    pub prefix: Option<LocationPrefix>,
    pub access_control_list: Option<Grants>,
    pub encryption: Option<Encryption>,
    pub storage_class: Option<StorageClass>,
}
impl S3Location {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ObjectLockRule {
    pub default_retention: Option<DefaultRetention>,
}
impl ObjectLockRule {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type ChecksumCrc32c = String;

#[derive(Debug, Default, Clone)]
pub struct ReplicationConfiguration {
    pub rules: Option<ReplicationRules>,
    pub role: Option<Role>,
}
impl ReplicationConfiguration {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum BucketVersioningStatus {
    Suspended,
    Enabled,
}
impl AsRef<str> for BucketVersioningStatus {
    fn as_ref(&self) -> &str {
        match self {
            Self::Suspended => "Suspended",
            Self::Enabled => "Enabled",
        }
    }
}
impl TryFrom<&str> for BucketVersioningStatus {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "Suspended" => Ok(Self::Suspended),
            "Enabled" => Ok(Self::Enabled),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

pub type FetchOwner = bool;

#[derive(Debug, Default, Clone)]
pub struct CompletedPart {
    pub checksum_sha1: Option<ChecksumSha1>,
    pub checksum_crc32: Option<ChecksumCrc32>,
    pub checksum_sha256: Option<ChecksumSha256>,
    pub e_tag: Option<ETag>,
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
    pub part_number: Option<PartNumber>,
}
impl CompletedPart {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketVersioningOutput {
    pub status: Option<BucketVersioningStatus>,
    pub mfa_delete: Option<MfaDeleteStatus>,
}
impl GetBucketVersioningOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketCors;
impl OperationShape for DeleteBucketCors {
    const NAME: &'static str = "DeleteBucketCors";
    type Input = DeleteBucketCorsRequest;
    type Output = ();
    type Error = ();
}

pub type KeyPrefixEquals = String;

pub type ObjectVersionId = String;

#[derive(Debug, Default, Clone)]
pub struct PutBucketRequestPayment;
impl OperationShape for PutBucketRequestPayment {
    const NAME: &'static str = "PutBucketRequestPayment";
    type Input = PutBucketRequestPaymentRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct ListBucketMetricsConfigurationsOutput {
    pub is_truncated: Option<IsTruncated>,
    pub continuation_token: Option<Token>,
    pub metrics_configuration_list: Option<MetricsConfigurationList>,
    pub next_continuation_token: Option<NextToken>,
}
impl ListBucketMetricsConfigurationsOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Clone)]
pub enum SelectObjectContentEventStream {
    Cont(ContinuationEvent),
    Records(RecordsEvent),
    Progress(ProgressEvent),
    Stats(StatsEvent),
    End(EndEvent),
}

#[derive(Debug, Default, Clone)]
pub struct PutObjectRetentionOutput {
    pub request_charged: Option<RequestCharged>,
}
impl PutObjectRetentionOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type ReplaceKeyWith = String;

#[derive(Debug, Default, Clone)]
pub struct InventoryEncryption {
    pub ssekms: Option<Ssekms>,
    pub sses3: Option<Sses3>,
}
impl InventoryEncryption {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct CreateBucketRequest {
    pub acl: Option<BucketCannedAcl>,
    pub grant_read: Option<GrantRead>,
    pub bucket: Option<BucketName>,
    pub grant_full_control: Option<GrantFullControl>,
    pub grant_write_acp: Option<GrantWriteAcp>,
    pub create_bucket_configuration: Option<CreateBucketConfiguration>,
    pub object_ownership: Option<ObjectOwnership>,
    pub object_lock_enabled_for_bucket: Option<ObjectLockEnabledForBucket>,
    pub grant_write: Option<GrantWrite>,
    pub grant_read_acp: Option<GrantReadAcp>,
}
impl CreateBucketRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct PutBucketVersioningRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub versioning_configuration: Option<VersioningConfiguration>,
    pub mfa: Option<Mfa>,
    pub bucket: Option<BucketName>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub content_md5: Option<ContentMd5>,
}
impl PutBucketVersioningRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type Token = String;

#[derive(Debug, Default, Clone)]
pub struct GetBucketPolicyStatusOutput {
    pub policy_status: Option<PolicyStatus>,
}
impl GetBucketPolicyStatusOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type CopySourceSseCustomerKey = String;

#[derive(Debug, Default, Clone)]
pub struct WebsiteConfiguration {
    pub redirect_all_requests_to: Option<RedirectAllRequestsTo>,
    pub error_document: Option<ErrorDocument>,
    pub routing_rules: Option<RoutingRules>,
    pub index_document: Option<IndexDocument>,
}
impl WebsiteConfiguration {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ObjectVersion {
    pub storage_class: Option<ObjectVersionStorageClass>,
    pub version_id: Option<ObjectVersionId>,
    pub last_modified: Option<LastModified>,
    pub owner: Option<Owner>,
    pub is_latest: Option<IsLatest>,
    pub e_tag: Option<ETag>,
    pub checksum_algorithm: Option<ChecksumAlgorithmList>,
    pub key: Option<ObjectKey>,
    pub size: Option<Size>,
}
impl ObjectVersion {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct OutputSerialization {
    pub csv: Option<CsvOutput>,
    pub json: Option<JsonOutput>,
}
impl OutputSerialization {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketLocation;
impl OperationShape for GetBucketLocation {
    const NAME: &'static str = "GetBucketLocation";
    type Input = GetBucketLocationRequest;
    type Output = GetBucketLocationOutput;
    type Error = ();
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum ArchiveStatus {
    DEEP_ARCHIVE_ACCESS,
    ARCHIVE_ACCESS,
}
impl AsRef<str> for ArchiveStatus {
    fn as_ref(&self) -> &str {
        match self {
            Self::DEEP_ARCHIVE_ACCESS => "DEEP_ARCHIVE_ACCESS",
            Self::ARCHIVE_ACCESS => "ARCHIVE_ACCESS",
        }
    }
}
impl TryFrom<&str> for ArchiveStatus {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "DEEP_ARCHIVE_ACCESS" => Ok(Self::DEEP_ARCHIVE_ACCESS),
            "ARCHIVE_ACCESS" => Ok(Self::ARCHIVE_ACCESS),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

pub type CopySourceIfNoneMatch = String;

pub type HttpRedirectCode = String;

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum Type {
    AmazonCustomerByEmail,
    CanonicalUser,
    Group,
}
impl AsRef<str> for Type {
    fn as_ref(&self) -> &str {
        match self {
            Self::AmazonCustomerByEmail => "AmazonCustomerByEmail",
            Self::CanonicalUser => "CanonicalUser",
            Self::Group => "Group",
        }
    }
}
impl TryFrom<&str> for Type {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "AmazonCustomerByEmail" => Ok(Self::AmazonCustomerByEmail),
            "CanonicalUser" => Ok(Self::CanonicalUser),
            "Group" => Ok(Self::Group),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

pub type MetadataKey = String;

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketCorsRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}
impl DeleteBucketCorsRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
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

pub type NotificationId = String;

pub type Errors = Vec<Error>;

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketOwnershipControlsRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}
impl DeleteBucketOwnershipControlsRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketCorsOutput {
    pub cors_rules: Option<CorsRules>,
}
impl GetBucketCorsOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListBuckets;
impl OperationShape for ListBuckets {
    const NAME: &'static str = "ListBuckets";
    type Input = ();
    type Output = ListBucketsOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct Progress {
    pub bytes_processed: Option<BytesProcessed>,
    pub bytes_scanned: Option<BytesScanned>,
    pub bytes_returned: Option<BytesReturned>,
}
impl Progress {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type RequestToken = String;

#[derive(Debug, Default, Clone)]
pub struct GetBucketAccelerateConfigurationOutput {
    pub status: Option<BucketAccelerateStatus>,
}
impl GetBucketAccelerateConfigurationOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct SelectObjectContentRequest {
    pub expression: Option<Expression>,
    pub expected_bucket_owner: Option<AccountId>,
    pub output_serialization: Option<OutputSerialization>,
    pub sse_customer_key: Option<SseCustomerKey>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub scan_range: Option<ScanRange>,
    pub bucket: Option<BucketName>,
    pub expression_type: Option<ExpressionType>,
    pub key: Option<ObjectKey>,
    pub input_serialization: Option<InputSerialization>,
    pub request_progress: Option<RequestProgress>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
}
impl SelectObjectContentRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum ObjectAttributes {
    CHECKSUM,
    OBJECT_PARTS,
    OBJECT_SIZE,
    ETAG,
    STORAGE_CLASS,
}
impl AsRef<str> for ObjectAttributes {
    fn as_ref(&self) -> &str {
        match self {
            Self::CHECKSUM => "Checksum",
            Self::OBJECT_PARTS => "ObjectParts",
            Self::OBJECT_SIZE => "ObjectSize",
            Self::ETAG => "ETag",
            Self::STORAGE_CLASS => "StorageClass",
        }
    }
}
impl TryFrom<&str> for ObjectAttributes {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "Checksum" => Ok(Self::CHECKSUM),
            "ObjectParts" => Ok(Self::OBJECT_PARTS),
            "ObjectSize" => Ok(Self::OBJECT_SIZE),
            "ETag" => Ok(Self::ETAG),
            "StorageClass" => Ok(Self::STORAGE_CLASS),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

pub type ContentEncoding = String;

pub type Body = Vec<u8>;

#[derive(Debug, Default, Clone)]
pub struct GetObjectAclOutput {
    pub grants: Option<Grants>,
    pub owner: Option<Owner>,
    pub request_charged: Option<RequestCharged>,
}
impl GetObjectAclOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct DeletedObject {
    pub delete_marker_version_id: Option<DeleteMarkerVersionId>,
    pub version_id: Option<ObjectVersionId>,
    pub key: Option<ObjectKey>,
    pub delete_marker: Option<DeleteMarker>,
}
impl DeletedObject {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct HeadObjectOutput {
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
    pub content_disposition: Option<ContentDisposition>,
    pub metadata: Option<Metadata>,
    pub storage_class: Option<StorageClass>,
    pub object_lock_retain_until_date: Option<ObjectLockRetainUntilDate>,
    pub content_encoding: Option<ContentEncoding>,
    pub expires: Option<Expires>,
    pub object_lock_mode: Option<ObjectLockMode>,
    pub parts_count: Option<PartsCount>,
    pub last_modified: Option<LastModified>,
    pub content_length: Option<ContentLength>,
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    pub checksum_sha1: Option<ChecksumSha1>,
    pub request_charged: Option<RequestCharged>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub replication_status: Option<ReplicationStatus>,
    pub archive_status: Option<ArchiveStatus>,
    pub delete_marker: Option<DeleteMarker>,
    pub cache_control: Option<CacheControl>,
    pub e_tag: Option<ETag>,
    pub website_redirect_location: Option<WebsiteRedirectLocation>,
    pub content_type: Option<ContentType>,
    pub missing_meta: Option<MissingMeta>,
    pub version_id: Option<ObjectVersionId>,
    pub checksum_sha256: Option<ChecksumSha256>,
    pub server_side_encryption: Option<ServerSideEncryption>,
    pub checksum_crc32: Option<ChecksumCrc32>,
    pub expiration: Option<Expiration>,
    pub accept_ranges: Option<AcceptRanges>,
    pub content_language: Option<ContentLanguage>,
    pub restore: Option<Restore>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub ssekms_key_id: Option<SsekmsKeyId>,
    pub object_lock_legal_hold_status: Option<ObjectLockLegalHoldStatus>,
}
impl HeadObjectOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketEncryptionRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}
impl GetBucketEncryptionRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketIntelligentTieringConfiguration;
impl OperationShape for DeleteBucketIntelligentTieringConfiguration {
    const NAME: &'static str = "DeleteBucketIntelligentTieringConfiguration";
    type Input = DeleteBucketIntelligentTieringConfigurationRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketOwnershipControlsOutput {
    pub ownership_controls: Option<OwnershipControls>,
}
impl GetBucketOwnershipControlsOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListObjectVersionsRequest {
    pub prefix: Option<Prefix>,
    pub bucket: Option<BucketName>,
    pub delimiter: Option<Delimiter>,
    pub encoding_type: Option<EncodingType>,
    pub key_marker: Option<KeyMarker>,
    pub version_id_marker: Option<VersionIdMarker>,
    pub max_keys: Option<MaxKeys>,
    pub expected_bucket_owner: Option<AccountId>,
}
impl ListObjectVersionsRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct Encryption {
    pub kms_key_id: Option<SsekmsKeyId>,
    pub encryption_type: Option<ServerSideEncryption>,
    pub kms_context: Option<KmsContext>,
}
impl Encryption {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum ObjectCannedAcl {
    aws_exec_read,
    public_read,
    bucket_owner_full_control,
    private,
    bucket_owner_read,
    public_read_write,
    authenticated_read,
}
impl AsRef<str> for ObjectCannedAcl {
    fn as_ref(&self) -> &str {
        match self {
            Self::aws_exec_read => "aws-exec-read",
            Self::public_read => "public-read",
            Self::bucket_owner_full_control => "bucket-owner-full-control",
            Self::private => "private",
            Self::bucket_owner_read => "bucket-owner-read",
            Self::public_read_write => "public-read-write",
            Self::authenticated_read => "authenticated-read",
        }
    }
}
impl TryFrom<&str> for ObjectCannedAcl {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "aws-exec-read" => Ok(Self::aws_exec_read),
            "public-read" => Ok(Self::public_read),
            "bucket-owner-full-control" => Ok(Self::bucket_owner_full_control),
            "private" => Ok(Self::private),
            "bucket-owner-read" => Ok(Self::bucket_owner_read),
            "public-read-write" => Ok(Self::public_read_write),
            "authenticated-read" => Ok(Self::authenticated_read),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct DeletePublicAccessBlockRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}
impl DeletePublicAccessBlockRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type BytesProcessed = i64;

pub type PartsCount = i32;

#[derive(Debug, Default, Clone)]
pub struct AccelerateConfiguration {
    pub status: Option<BucketAccelerateStatus>,
}
impl AccelerateConfiguration {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ContinuationEvent {}
impl ContinuationEvent {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct CopyObjectOutput {
    pub copy_object_result: Option<CopyObjectResult>,
    pub ssekms_encryption_context: Option<SsekmsEncryptionContext>,
    pub server_side_encryption: Option<ServerSideEncryption>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub expiration: Option<Expiration>,
    pub request_charged: Option<RequestCharged>,
    pub ssekms_key_id: Option<SsekmsKeyId>,
    pub copy_source_version_id: Option<CopySourceVersionId>,
    pub version_id: Option<ObjectVersionId>,
}
impl CopyObjectOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type Prefix = String;

#[derive(Debug, Default, Clone)]
pub struct CreateBucketConfiguration {
    pub location_constraint: Option<BucketLocationConstraint>,
}
impl CreateBucketConfiguration {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct DeletePublicAccessBlock;
impl OperationShape for DeletePublicAccessBlock {
    const NAME: &'static str = "DeletePublicAccessBlock";
    type Input = DeletePublicAccessBlockRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct RequestProgress {
    pub enabled: Option<EnableRequestProgress>,
}
impl RequestProgress {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct UploadPartCopyOutput {
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub ssekms_key_id: Option<SsekmsKeyId>,
    pub copy_part_result: Option<CopyPartResult>,
    pub server_side_encryption: Option<ServerSideEncryption>,
    pub copy_source_version_id: Option<CopySourceVersionId>,
    pub request_charged: Option<RequestCharged>,
}
impl UploadPartCopyOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type Value = String;

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum Protocol {
    https,
    http,
}
impl AsRef<str> for Protocol {
    fn as_ref(&self) -> &str {
        match self {
            Self::https => "https",
            Self::http => "http",
        }
    }
}
impl TryFrom<&str> for Protocol {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "https" => Ok(Self::https),
            "http" => Ok(Self::http),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum DeleteMarkerReplicationStatus {
    Disabled,
    Enabled,
}
impl AsRef<str> for DeleteMarkerReplicationStatus {
    fn as_ref(&self) -> &str {
        match self {
            Self::Disabled => "Disabled",
            Self::Enabled => "Enabled",
        }
    }
}
impl TryFrom<&str> for DeleteMarkerReplicationStatus {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "Disabled" => Ok(Self::Disabled),
            "Enabled" => Ok(Self::Enabled),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct CopyObjectRequest {
    pub ssekms_key_id: Option<SsekmsKeyId>,
    pub grant_full_control: Option<GrantFullControl>,
    pub grant_read_acp: Option<GrantReadAcp>,
    pub copy_source_sse_customer_key: Option<CopySourceSseCustomerKey>,
    pub object_lock_mode: Option<ObjectLockMode>,
    pub content_language: Option<ContentLanguage>,
    pub content_type: Option<ContentType>,
    pub grant_read: Option<GrantRead>,
    pub sse_customer_key: Option<SseCustomerKey>,
    pub object_lock_retain_until_date: Option<ObjectLockRetainUntilDate>,
    pub server_side_encryption: Option<ServerSideEncryption>,
    pub cache_control: Option<CacheControl>,
    pub expected_bucket_owner: Option<AccountId>,
    pub key: Option<ObjectKey>,
    pub copy_source_sse_customer_algorithm: Option<CopySourceSseCustomerAlgorithm>,
    pub copy_source: Option<CopySource>,
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    pub expires: Option<Expires>,
    pub metadata_directive: Option<MetadataDirective>,
    pub ssekms_encryption_context: Option<SsekmsEncryptionContext>,
    pub acl: Option<ObjectCannedAcl>,
    pub request_payer: Option<RequestPayer>,
    pub copy_source_if_match: Option<CopySourceIfMatch>,
    pub content_encoding: Option<ContentEncoding>,
    pub copy_source_sse_customer_key_md5: Option<CopySourceSseCustomerKeyMd5>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub copy_source_if_none_match: Option<CopySourceIfNoneMatch>,
    pub metadata: Option<Metadata>,
    pub website_redirect_location: Option<WebsiteRedirectLocation>,
    pub bucket: Option<BucketName>,
    pub object_lock_legal_hold_status: Option<ObjectLockLegalHoldStatus>,
    pub copy_source_if_unmodified_since: Option<CopySourceIfUnmodifiedSince>,
    pub copy_source_if_modified_since: Option<CopySourceIfModifiedSince>,
    pub expected_source_bucket_owner: Option<AccountId>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub tagging: Option<TaggingHeader>,
    pub grant_write_acp: Option<GrantWriteAcp>,
    pub content_disposition: Option<ContentDisposition>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub tagging_directive: Option<TaggingDirective>,
    pub storage_class: Option<StorageClass>,
}
impl CopyObjectRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type ObjectKey = String;

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
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

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
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

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}
impl DeleteBucketRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct PolicyStatus {
    pub is_public: Option<IsPublic>,
}
impl PolicyStatus {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type ContentLanguage = String;

pub type CopySourceIfModifiedSince = String;

pub type CopySourceSseCustomerKeyMd5 = String;

#[derive(Debug, Default, Clone)]
pub struct GetBucketCors;
impl OperationShape for GetBucketCors {
    const NAME: &'static str = "GetBucketCors";
    type Input = GetBucketCorsRequest;
    type Output = GetBucketCorsOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct RedirectAllRequestsTo {
    pub protocol: Option<Protocol>,
    pub host_name: Option<HostName>,
}
impl RedirectAllRequestsTo {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketAcl;
impl OperationShape for GetBucketAcl {
    const NAME: &'static str = "GetBucketAcl";
    type Input = GetBucketAclRequest;
    type Output = GetBucketAclOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct RestoreObjectOutput {
    pub restore_output_path: Option<RestoreOutputPath>,
    pub request_charged: Option<RequestCharged>,
}
impl RestoreObjectOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct RecordsEvent {
    pub payload: Option<Body>,
}
impl RecordsEvent {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct RestoreObject;
impl OperationShape for RestoreObject {
    const NAME: &'static str = "RestoreObject";
    type Input = RestoreObjectRequest;
    type Output = RestoreObjectOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct ListBucketInventoryConfigurationsOutput {
    pub continuation_token: Option<Token>,
    pub inventory_configuration_list: Option<InventoryConfigurationList>,
    pub next_continuation_token: Option<NextToken>,
    pub is_truncated: Option<IsTruncated>,
}
impl ListBucketInventoryConfigurationsOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct PutObjectTagging;
impl OperationShape for PutObjectTagging {
    const NAME: &'static str = "PutObjectTagging";
    type Input = PutObjectTaggingRequest;
    type Output = PutObjectTaggingOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketInventoryConfiguration;
impl OperationShape for GetBucketInventoryConfiguration {
    const NAME: &'static str = "GetBucketInventoryConfiguration";
    type Input = GetBucketInventoryConfigurationRequest;
    type Output = GetBucketInventoryConfigurationOutput;
    type Error = ();
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum ObjectStorageClass {
    STANDARD,
    GLACIER,
    REDUCED_REDUNDANCY,
    ONEZONE_IA,
    GLACIER_IR,
    STANDARD_IA,
    DEEP_ARCHIVE,
    INTELLIGENT_TIERING,
    OUTPOSTS,
}
impl AsRef<str> for ObjectStorageClass {
    fn as_ref(&self) -> &str {
        match self {
            Self::STANDARD => "STANDARD",
            Self::GLACIER => "GLACIER",
            Self::REDUCED_REDUNDANCY => "REDUCED_REDUNDANCY",
            Self::ONEZONE_IA => "ONEZONE_IA",
            Self::GLACIER_IR => "GLACIER_IR",
            Self::STANDARD_IA => "STANDARD_IA",
            Self::DEEP_ARCHIVE => "DEEP_ARCHIVE",
            Self::INTELLIGENT_TIERING => "INTELLIGENT_TIERING",
            Self::OUTPOSTS => "OUTPOSTS",
        }
    }
}
impl TryFrom<&str> for ObjectStorageClass {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "STANDARD" => Ok(Self::STANDARD),
            "GLACIER" => Ok(Self::GLACIER),
            "REDUCED_REDUNDANCY" => Ok(Self::REDUCED_REDUNDANCY),
            "ONEZONE_IA" => Ok(Self::ONEZONE_IA),
            "GLACIER_IR" => Ok(Self::GLACIER_IR),
            "STANDARD_IA" => Ok(Self::STANDARD_IA),
            "DEEP_ARCHIVE" => Ok(Self::DEEP_ARCHIVE),
            "INTELLIGENT_TIERING" => Ok(Self::INTELLIGENT_TIERING),
            "OUTPOSTS" => Ok(Self::OUTPOSTS),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

pub type Grants = Vec<Grant>;

#[derive(Debug, Default, Clone)]
pub struct GetBucketLocationOutput {
    pub location_constraint: Option<BucketLocationConstraint>,
}
impl GetBucketLocationOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct PutObjectLegalHoldRequest {
    pub request_payer: Option<RequestPayer>,
    pub version_id: Option<ObjectVersionId>,
    pub bucket: Option<BucketName>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub key: Option<ObjectKey>,
    pub content_md5: Option<ContentMd5>,
    pub expected_bucket_owner: Option<AccountId>,
    pub legal_hold: Option<ObjectLockLegalHold>,
}
impl PutObjectLegalHoldRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type IsPublic = bool;

#[derive(Debug, Default, Clone)]
pub struct ObjectLockRetention {
    pub mode: Option<ObjectLockRetentionMode>,
    pub retain_until_date: Option<Date>,
}
impl ObjectLockRetention {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct CreateBucketOutput {
    pub location: Option<Location>,
}
impl CreateBucketOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketEncryption;
impl OperationShape for GetBucketEncryption {
    const NAME: &'static str = "GetBucketEncryption";
    type Input = GetBucketEncryptionRequest;
    type Output = GetBucketEncryptionOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct ObjectLockLegalHold {
    pub status: Option<ObjectLockLegalHoldStatus>,
}
impl ObjectLockLegalHold {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct QueueConfiguration {
    pub queue_arn: Option<QueueArn>,
    pub filter: Option<NotificationConfigurationFilter>,
    pub id: Option<NotificationId>,
    pub events: Option<EventList>,
}
impl QueueConfiguration {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ServerSideEncryptionConfiguration {
    pub rules: Option<ServerSideEncryptionRules>,
}
impl ServerSideEncryptionConfiguration {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct AbortMultipartUpload;
impl OperationShape for AbortMultipartUpload {
    const NAME: &'static str = "AbortMultipartUpload";
    type Input = AbortMultipartUploadRequest;
    type Output = AbortMultipartUploadOutput;
    type Error = ();
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
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
pub struct InputSerialization {
    pub csv: Option<CsvInput>,
    pub compression_type: Option<CompressionType>,
    pub parquet: Option<ParquetInput>,
    pub json: Option<JsonInput>,
}
impl InputSerialization {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type Parts = Vec<Part>;

pub type ContentRange = String;

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketTagging;
impl OperationShape for DeleteBucketTagging {
    const NAME: &'static str = "DeleteBucketTagging";
    type Input = DeleteBucketTaggingRequest;
    type Output = ();
    type Error = ();
}

pub type DeleteMarker = bool;

#[derive(Debug, Default, Clone)]
pub struct GetBucketLocationRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}
impl GetBucketLocationRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct PutBucketOwnershipControls;
impl OperationShape for PutBucketOwnershipControls {
    const NAME: &'static str = "PutBucketOwnershipControls";
    type Input = PutBucketOwnershipControlsRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct DeleteObjectTaggingRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub key: Option<ObjectKey>,
    pub version_id: Option<ObjectVersionId>,
    pub bucket: Option<BucketName>,
}
impl DeleteObjectTaggingRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct EncryptionConfiguration {
    pub replica_kms_key_id: Option<ReplicaKmsKeyId>,
}
impl EncryptionConfiguration {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum ExpirationStatus {
    Disabled,
    Enabled,
}
impl AsRef<str> for ExpirationStatus {
    fn as_ref(&self) -> &str {
        match self {
            Self::Disabled => "Disabled",
            Self::Enabled => "Enabled",
        }
    }
}
impl TryFrom<&str> for ExpirationStatus {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "Disabled" => Ok(Self::Disabled),
            "Enabled" => Ok(Self::Enabled),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketOwnershipControls;
impl OperationShape for DeleteBucketOwnershipControls {
    const NAME: &'static str = "DeleteBucketOwnershipControls";
    type Input = DeleteBucketOwnershipControlsRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum ObjectOwnership {
    ObjectWriter,
    BucketOwnerEnforced,
    BucketOwnerPreferred,
}
impl AsRef<str> for ObjectOwnership {
    fn as_ref(&self) -> &str {
        match self {
            Self::ObjectWriter => "ObjectWriter",
            Self::BucketOwnerEnforced => "BucketOwnerEnforced",
            Self::BucketOwnerPreferred => "BucketOwnerPreferred",
        }
    }
}
impl TryFrom<&str> for ObjectOwnership {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "ObjectWriter" => Ok(Self::ObjectWriter),
            "BucketOwnerEnforced" => Ok(Self::BucketOwnerEnforced),
            "BucketOwnerPreferred" => Ok(Self::BucketOwnerPreferred),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

pub type Setting = bool;

pub type AcceptRanges = String;

#[derive(Debug, Default, Clone)]
pub struct OwnershipControlsRule {
    pub object_ownership: Option<ObjectOwnership>,
}
impl OwnershipControlsRule {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type TaggingHeader = String;

#[derive(Debug, Default, Clone)]
pub struct DeleteMarkerReplication {
    pub status: Option<DeleteMarkerReplicationStatus>,
}
impl DeleteMarkerReplication {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketMetricsConfiguration;
impl OperationShape for GetBucketMetricsConfiguration {
    const NAME: &'static str = "GetBucketMetricsConfiguration";
    type Input = GetBucketMetricsConfigurationRequest;
    type Output = GetBucketMetricsConfigurationOutput;
    type Error = ();
}

pub type GrantWrite = String;

#[derive(Debug, Default, Clone)]
pub struct PutBucketAnalyticsConfigurationRequest {
    pub bucket: Option<BucketName>,
    pub analytics_configuration: Option<AnalyticsConfiguration>,
    pub expected_bucket_owner: Option<AccountId>,
    pub id: Option<AnalyticsId>,
}
impl PutBucketAnalyticsConfigurationRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct UploadPart;
impl OperationShape for UploadPart {
    const NAME: &'static str = "UploadPart";
    type Input = UploadPartRequest;
    type Output = UploadPartOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct CsvInput {
    pub allow_quoted_record_delimiter: Option<AllowQuotedRecordDelimiter>,
    pub comments: Option<Comments>,
    pub quote_character: Option<QuoteCharacter>,
    pub record_delimiter: Option<RecordDelimiter>,
    pub file_header_info: Option<FileHeaderInfo>,
    pub field_delimiter: Option<FieldDelimiter>,
    pub quote_escape_character: Option<QuoteEscapeCharacter>,
}
impl CsvInput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketVersioning;
impl OperationShape for GetBucketVersioning {
    const NAME: &'static str = "GetBucketVersioning";
    type Input = GetBucketVersioningRequest;
    type Output = GetBucketVersioningOutput;
    type Error = ();
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum IntelligentTieringStatus {
    Disabled,
    Enabled,
}
impl AsRef<str> for IntelligentTieringStatus {
    fn as_ref(&self) -> &str {
        match self {
            Self::Disabled => "Disabled",
            Self::Enabled => "Enabled",
        }
    }
}
impl TryFrom<&str> for IntelligentTieringStatus {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "Disabled" => Ok(Self::Disabled),
            "Enabled" => Ok(Self::Enabled),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListBucketAnalyticsConfigurationsOutput {
    pub analytics_configuration_list: Option<AnalyticsConfigurationList>,
    pub is_truncated: Option<IsTruncated>,
    pub next_continuation_token: Option<NextToken>,
    pub continuation_token: Option<Token>,
}
impl ListBucketAnalyticsConfigurationsOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type ChecksumSha256 = String;

#[derive(Debug, Default, Clone)]
pub struct GetObjectTorrent;
impl OperationShape for GetObjectTorrent {
    const NAME: &'static str = "GetObjectTorrent";
    type Input = GetObjectTorrentRequest;
    type Output = GetObjectTorrentOutput;
    type Error = ();
}

pub type MaxKeys = i32;

#[derive(Debug, Default, Clone)]
pub struct PutBucketCorsRequest {
    pub cors_configuration: Option<CorsConfiguration>,
    pub expected_bucket_owner: Option<AccountId>,
    pub content_md5: Option<ContentMd5>,
    pub bucket: Option<BucketName>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
}
impl PutBucketCorsRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type TargetBucket = String;

#[derive(Debug, Default, Clone)]
pub struct InventoryConfiguration {
    pub id: Option<InventoryId>,
    pub destination: Option<InventoryDestination>,
    pub included_object_versions: Option<InventoryIncludedObjectVersions>,
    pub filter: Option<InventoryFilter>,
    pub optional_fields: Option<InventoryOptionalFields>,
    pub is_enabled: Option<IsEnabled>,
    pub schedule: Option<InventorySchedule>,
}
impl InventoryConfiguration {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type NextToken = String;

pub type TopicArn = String;

#[derive(Debug, Default, Clone)]
pub struct GetBucketAnalyticsConfigurationOutput {
    pub analytics_configuration: Option<AnalyticsConfiguration>,
}
impl GetBucketAnalyticsConfigurationOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListBucketAnalyticsConfigurationsRequest {
    pub continuation_token: Option<Token>,
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}
impl ListBucketAnalyticsConfigurationsRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListBucketMetricsConfigurationsRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub continuation_token: Option<Token>,
    pub bucket: Option<BucketName>,
}
impl ListBucketMetricsConfigurationsRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ObjectAlreadyInActiveTierError {}
impl ObjectAlreadyInActiveTierError {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct S3KeyFilter {
    pub filter_rules: Option<FilterRuleList>,
}
impl S3KeyFilter {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct Tag {
    pub key: Option<ObjectKey>,
    pub value: Option<Value>,
}
impl Tag {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct AnalyticsConfiguration {
    pub storage_class_analysis: Option<StorageClassAnalysis>,
    pub filter: Option<AnalyticsFilter>,
    pub id: Option<AnalyticsId>,
}
impl AnalyticsConfiguration {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct PutBucketEncryptionRequest {
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
    pub server_side_encryption_configuration: Option<ServerSideEncryptionConfiguration>,
    pub content_md5: Option<ContentMd5>,
}
impl PutBucketEncryptionRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketMetricsConfigurationRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
    pub id: Option<MetricsId>,
}
impl GetBucketMetricsConfigurationRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct PutBucketEncryption;
impl OperationShape for PutBucketEncryption {
    const NAME: &'static str = "PutBucketEncryption";
    type Input = PutBucketEncryptionRequest;
    type Output = ();
    type Error = ();
}

pub type CacheControl = String;

pub type EmailAddress = String;

#[derive(Debug, Default, Clone)]
pub struct LambdaFunctionConfiguration {
    pub filter: Option<NotificationConfigurationFilter>,
    pub id: Option<NotificationId>,
    pub lambda_function_arn: Option<LambdaFunctionArn>,
    pub events: Option<EventList>,
}
impl LambdaFunctionConfiguration {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct CreateBucket;
impl OperationShape for CreateBucket {
    const NAME: &'static str = "CreateBucket";
    type Input = CreateBucketRequest;
    type Output = CreateBucketOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketInventoryConfiguration;
impl OperationShape for DeleteBucketInventoryConfiguration {
    const NAME: &'static str = "DeleteBucketInventoryConfiguration";
    type Input = DeleteBucketInventoryConfigurationRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketLifecycleConfiguration;
impl OperationShape for GetBucketLifecycleConfiguration {
    const NAME: &'static str = "GetBucketLifecycleConfiguration";
    type Input = GetBucketLifecycleConfigurationRequest;
    type Output = GetBucketLifecycleConfigurationOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetObjectLegalHoldRequest {
    pub request_payer: Option<RequestPayer>,
    pub bucket: Option<BucketName>,
    pub version_id: Option<ObjectVersionId>,
    pub expected_bucket_owner: Option<AccountId>,
    pub key: Option<ObjectKey>,
}
impl GetObjectLegalHoldRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct PutBucketAcl;
impl OperationShape for PutBucketAcl {
    const NAME: &'static str = "PutBucketAcl";
    type Input = PutBucketAclRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct CorsConfiguration {
    pub cors_rules: Option<CorsRules>,
}
impl CorsConfiguration {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type QueueArn = String;

#[derive(Debug, Default, Clone)]
pub struct ListBucketsOutput {
    pub buckets: Option<Buckets>,
    pub owner: Option<Owner>,
}
impl ListBucketsOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct LifecycleExpiration {
    pub date: Option<Date>,
    pub expired_object_delete_marker: Option<ExpiredObjectDeleteMarker>,
    pub days: Option<Days>,
}
impl LifecycleExpiration {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type ObjectLockRetainUntilDate = String;

#[derive(Debug, Default, Clone)]
pub struct PutObjectAclRequest {
    pub grant_read: Option<GrantRead>,
    pub acl: Option<ObjectCannedAcl>,
    pub expected_bucket_owner: Option<AccountId>,
    pub grant_full_control: Option<GrantFullControl>,
    pub access_control_policy: Option<AccessControlPolicy>,
    pub request_payer: Option<RequestPayer>,
    pub version_id: Option<ObjectVersionId>,
    pub grant_write_acp: Option<GrantWriteAcp>,
    pub key: Option<ObjectKey>,
    pub grant_read_acp: Option<GrantReadAcp>,
    pub grant_write: Option<GrantWrite>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub bucket: Option<BucketName>,
    pub content_md5: Option<ContentMd5>,
}
impl PutObjectAclRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct RoutingRule {
    pub condition: Option<Condition>,
    pub redirect: Option<Redirect>,
}
impl RoutingRule {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct PutBucketPolicy;
impl OperationShape for PutBucketPolicy {
    const NAME: &'static str = "PutBucketPolicy";
    type Input = PutBucketPolicyRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct DeleteObjectOutput {
    pub version_id: Option<ObjectVersionId>,
    pub delete_marker: Option<DeleteMarker>,
    pub request_charged: Option<RequestCharged>,
}
impl DeleteObjectOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct PutObjectLockConfigurationOutput {
    pub request_charged: Option<RequestCharged>,
}
impl PutObjectLockConfigurationOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct VersioningConfiguration {
    pub status: Option<BucketVersioningStatus>,
    pub mfa_delete: Option<MfaDelete>,
}
impl VersioningConfiguration {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type StreamingBlob = Arc<hyper::Body>;

pub type GrantFullControl = String;

#[derive(Debug, Default, Clone)]
pub struct InventorySchedule {
    pub frequency: Option<InventoryFrequency>,
}
impl InventorySchedule {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type IsLatest = bool;

pub type GrantRead = String;

#[derive(Debug, Default, Clone)]
pub struct ListMultipartUploads;
impl OperationShape for ListMultipartUploads {
    const NAME: &'static str = "ListMultipartUploads";
    type Input = ListMultipartUploadsRequest;
    type Output = ListMultipartUploadsOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct PutBucketIntelligentTieringConfigurationRequest {
    pub id: Option<IntelligentTieringId>,
    pub intelligent_tiering_configuration: Option<IntelligentTieringConfiguration>,
    pub bucket: Option<BucketName>,
}
impl PutBucketIntelligentTieringConfigurationRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type RoutingRules = Vec<RoutingRule>;

pub type TopicConfigurationList = Vec<TopicConfiguration>;

#[derive(Debug, Default, Clone)]
pub struct ProgressEvent {
    pub details: Option<Progress>,
}
impl ProgressEvent {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct CompleteMultipartUpload;
impl OperationShape for CompleteMultipartUpload {
    const NAME: &'static str = "CompleteMultipartUpload";
    type Input = CompleteMultipartUploadRequest;
    type Output = CompleteMultipartUploadOutput;
    type Error = ();
}

pub type DeleteMarkerVersionId = String;

#[derive(Debug, Default, Clone)]
pub struct HeadBucket;
impl OperationShape for HeadBucket {
    const NAME: &'static str = "HeadBucket";
    type Input = HeadBucketRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketIntelligentTieringConfigurationRequest {
    pub id: Option<IntelligentTieringId>,
    pub bucket: Option<BucketName>,
}
impl GetBucketIntelligentTieringConfigurationRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
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
pub struct EndEvent {}
impl EndEvent {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type BytesScanned = i64;

#[derive(Debug, Default, Clone)]
pub struct ListObjectVersionsOutput {
    pub key_marker: Option<KeyMarker>,
    pub delete_markers: Option<DeleteMarkers>,
    pub versions: Option<ObjectVersionList>,
    pub name: Option<BucketName>,
    pub next_key_marker: Option<NextKeyMarker>,
    pub delimiter: Option<Delimiter>,
    pub encoding_type: Option<EncodingType>,
    pub prefix: Option<Prefix>,
    pub is_truncated: Option<IsTruncated>,
    pub max_keys: Option<MaxKeys>,
    pub common_prefixes: Option<CommonPrefixList>,
    pub version_id_marker: Option<VersionIdMarker>,
    pub next_version_id_marker: Option<NextVersionIdMarker>,
}
impl ListObjectVersionsOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
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

#[derive(Debug, Default, Clone)]
pub struct NoncurrentVersionExpiration {
    pub newer_noncurrent_versions: Option<VersionCount>,
    pub noncurrent_days: Option<Days>,
}
impl NoncurrentVersionExpiration {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct PutBucketAccelerateConfiguration;
impl OperationShape for PutBucketAccelerateConfiguration {
    const NAME: &'static str = "PutBucketAccelerateConfiguration";
    type Input = PutBucketAccelerateConfigurationRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum ChecksumAlgorithm {
    SHA256,
    SHA1,
    CRC32C,
    CRC32,
}
impl AsRef<str> for ChecksumAlgorithm {
    fn as_ref(&self) -> &str {
        match self {
            Self::SHA256 => "SHA256",
            Self::SHA1 => "SHA1",
            Self::CRC32C => "CRC32C",
            Self::CRC32 => "CRC32",
        }
    }
}
impl TryFrom<&str> for ChecksumAlgorithm {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "SHA256" => Ok(Self::SHA256),
            "SHA1" => Ok(Self::SHA1),
            "CRC32C" => Ok(Self::CRC32C),
            "CRC32" => Ok(Self::CRC32),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum CompressionType {
    BZIP2,
    NONE,
    GZIP,
}
impl AsRef<str> for CompressionType {
    fn as_ref(&self) -> &str {
        match self {
            Self::BZIP2 => "BZIP2",
            Self::NONE => "NONE",
            Self::GZIP => "GZIP",
        }
    }
}
impl TryFrom<&str> for CompressionType {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "BZIP2" => Ok(Self::BZIP2),
            "NONE" => Ok(Self::NONE),
            "GZIP" => Ok(Self::GZIP),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct HeadObject;
impl OperationShape for HeadObject {
    const NAME: &'static str = "HeadObject";
    type Input = HeadObjectRequest;
    type Output = HeadObjectOutput;
    type Error = ();
}

pub type IfModifiedSince = String;

#[derive(Debug, Default, Clone)]
pub struct PutBucketCors;
impl OperationShape for PutBucketCors {
    const NAME: &'static str = "PutBucketCors";
    type Input = PutBucketCorsRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
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
pub struct GetBucketMetricsConfigurationOutput {
    pub metrics_configuration: Option<MetricsConfiguration>,
}
impl GetBucketMetricsConfigurationOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct StatsEvent {
    pub details: Option<Stats>,
}
impl StatsEvent {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type CopySourceIfUnmodifiedSince = String;

#[derive(Debug, Default, Clone)]
pub struct ListObjectVersions;
impl OperationShape for ListObjectVersions {
    const NAME: &'static str = "ListObjectVersions";
    type Input = ListObjectVersionsRequest;
    type Output = ListObjectVersionsOutput;
    type Error = ();
}

pub type MaxUploads = i32;

pub type CommonPrefixList = Vec<CommonPrefix>;

pub type ObjectList = Vec<Object>;

#[derive(Debug, Default, Clone)]
pub struct GetObjectLegalHold;
impl OperationShape for GetObjectLegalHold {
    const NAME: &'static str = "GetObjectLegalHold";
    type Input = GetObjectLegalHoldRequest;
    type Output = GetObjectLegalHoldOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketReplication;
impl OperationShape for DeleteBucketReplication {
    const NAME: &'static str = "DeleteBucketReplication";
    type Input = DeleteBucketReplicationRequest;
    type Output = ();
    type Error = ();
}

pub type Buckets = Vec<Bucket>;

pub type FieldDelimiter = String;

pub type IsEnabled = bool;

pub type ObjectIdentifierList = Vec<ObjectIdentifier>;

#[derive(Debug, Default, Clone)]
pub struct GetBucketLogging;
impl OperationShape for GetBucketLogging {
    const NAME: &'static str = "GetBucketLogging";
    type Input = GetBucketLoggingRequest;
    type Output = GetBucketLoggingOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketReplicationOutput {
    pub replication_configuration: Option<ReplicationConfiguration>,
}
impl GetBucketReplicationOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListMultipartUploadsRequest {
    pub max_uploads: Option<MaxUploads>,
    pub expected_bucket_owner: Option<AccountId>,
    pub key_marker: Option<KeyMarker>,
    pub encoding_type: Option<EncodingType>,
    pub bucket: Option<BucketName>,
    pub upload_id_marker: Option<UploadIdMarker>,
    pub prefix: Option<Prefix>,
    pub delimiter: Option<Delimiter>,
}
impl ListMultipartUploadsRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type PartNumber = i32;

#[derive(Debug, Default, Clone)]
pub struct PutBucketInventoryConfiguration;
impl OperationShape for PutBucketInventoryConfiguration {
    const NAME: &'static str = "PutBucketInventoryConfiguration";
    type Input = PutBucketInventoryConfigurationRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
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

pub type SsekmsEncryptionContext = String;

#[derive(Debug, Default, Clone)]
pub struct RequestPaymentConfiguration {
    pub payer: Option<Payer>,
}
impl RequestPaymentConfiguration {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct PutBucketRequestPaymentRequest {
    pub bucket: Option<BucketName>,
    pub request_payment_configuration: Option<RequestPaymentConfiguration>,
    pub content_md5: Option<ContentMd5>,
    pub expected_bucket_owner: Option<AccountId>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
}
impl PutBucketRequestPaymentRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct Transition {
    pub date: Option<Date>,
    pub days: Option<Days>,
    pub storage_class: Option<TransitionStorageClass>,
}
impl Transition {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type ResponseContentDisposition = String;

pub type SseCustomerAlgorithm = String;

pub type IfUnmodifiedSince = String;

#[derive(Debug, Default, Clone)]
pub struct MultipartUpload {
    pub key: Option<ObjectKey>,
    pub storage_class: Option<StorageClass>,
    pub owner: Option<Owner>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub initiator: Option<Initiator>,
    pub initiated: Option<Initiated>,
    pub upload_id: Option<MultipartUploadId>,
}
impl MultipartUpload {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type TagCount = i32;

#[derive(Debug, Default, Clone)]
pub struct CsvOutput {
    pub quote_character: Option<QuoteCharacter>,
    pub quote_fields: Option<QuoteFields>,
    pub record_delimiter: Option<RecordDelimiter>,
    pub field_delimiter: Option<FieldDelimiter>,
    pub quote_escape_character: Option<QuoteEscapeCharacter>,
}
impl CsvOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}
