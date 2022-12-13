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

pub type DeleteMarkerVersionId = String;

pub type ChecksumCrc32 = String;

#[derive(Debug, Default, Clone)]
pub struct DeleteObject;
impl OperationShape for DeleteObject {
    const NAME: &'static str = "DeleteObject";
    type Input = DeleteObjectRequest;
    type Output = DeleteObjectOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketLoggingRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}
impl GetBucketLoggingRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct NotFound {}
impl NotFound {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketAccelerateConfiguration;
impl OperationShape for GetBucketAccelerateConfiguration {
    const NAME: &'static str = "GetBucketAccelerateConfiguration";
    type Input = GetBucketAccelerateConfigurationRequest;
    type Output = GetBucketAccelerateConfigurationOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct LifecycleRuleAndOperator {
    pub tags: Option<TagSet>,
    pub object_size_greater_than: Option<ObjectSizeGreaterThanBytes>,
    pub object_size_less_than: Option<ObjectSizeLessThanBytes>,
    pub prefix: Option<Prefix>,
}
impl LifecycleRuleAndOperator {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct InvalidObjectState {
    pub access_tier: Option<IntelligentTieringAccessTier>,
    pub storage_class: Option<StorageClass>,
}
impl InvalidObjectState {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type AllowedMethod = String;

#[derive(Debug, Default, Clone)]
pub struct GetObjectRequest {
    pub if_none_match: Option<IfNoneMatch>,
    pub if_unmodified_since: Option<IfUnmodifiedSince>,
    pub version_id: Option<ObjectVersionId>,
    pub response_content_type: Option<ResponseContentType>,
    pub sse_customer_key: Option<SseCustomerKey>,
    pub expected_bucket_owner: Option<AccountId>,
    pub if_match: Option<IfMatch>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub response_content_language: Option<ResponseContentLanguage>,
    pub response_cache_control: Option<ResponseCacheControl>,
    pub part_number: Option<PartNumber>,
    pub request_payer: Option<RequestPayer>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub range: Option<Range>,
    pub response_content_disposition: Option<ResponseContentDisposition>,
    pub response_content_encoding: Option<ResponseContentEncoding>,
    pub key: Option<ObjectKey>,
    pub response_expires: Option<ResponseExpires>,
    pub bucket: Option<BucketName>,
    pub if_modified_since: Option<IfModifiedSince>,
    pub checksum_mode: Option<ChecksumMode>,
}
impl GetObjectRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ProgressEvent {
    pub details: Option<Progress>,
}
impl ProgressEvent {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type Mfa = String;

pub type AnalyticsId = String;

#[derive(Debug, Default, Clone)]
pub struct CompletedPart {
    pub checksum_sha1: Option<ChecksumSha1>,
    pub checksum_sha256: Option<ChecksumSha256>,
    pub checksum_crc32: Option<ChecksumCrc32>,
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
    pub e_tag: Option<ETag>,
    pub part_number: Option<PartNumber>,
}
impl CompletedPart {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ReplicationRule {
    pub source_selection_criteria: Option<SourceSelectionCriteria>,
    pub existing_object_replication: Option<ExistingObjectReplication>,
    pub destination: Option<Destination>,
    pub filter: Option<ReplicationRuleFilter>,
    pub id: Option<Id>,
    pub prefix: Option<Prefix>,
    pub status: Option<ReplicationRuleStatus>,
    pub delete_marker_replication: Option<DeleteMarkerReplication>,
    pub priority: Option<Priority>,
}
impl ReplicationRule {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type SseCustomerKeyMd5 = String;

#[derive(Debug, Default, Clone)]
pub struct Part {
    pub size: Option<Size>,
    pub checksum_sha256: Option<ChecksumSha256>,
    pub last_modified: Option<LastModified>,
    pub checksum_sha1: Option<ChecksumSha1>,
    pub e_tag: Option<ETag>,
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
    pub part_number: Option<PartNumber>,
    pub checksum_crc32: Option<ChecksumCrc32>,
}
impl Part {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct RestoreRequest {
    pub output_location: Option<OutputLocation>,
    pub select_parameters: Option<SelectParameters>,
    pub tier: Option<Tier>,
    pub r#type: Option<RestoreRequestType>,
    pub days: Option<Days>,
    pub glacier_job_parameters: Option<GlacierJobParameters>,
    pub description: Option<Description>,
}
impl RestoreRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type SkipValidation = bool;

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketInventoryConfiguration;
impl OperationShape for DeleteBucketInventoryConfiguration {
    const NAME: &'static str = "DeleteBucketInventoryConfiguration";
    type Input = DeleteBucketInventoryConfigurationRequest;
    type Output = ();
    type Error = ();
}

pub type CopySourceSseCustomerKey = String;

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

pub type ErrorCode = String;

pub type EmailAddress = String;

pub type ErrorMessage = String;

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum BucketLocationConstraint {
    me_south_1,
    ap_northeast_3,
    ap_east_1,
    us_east_2,
    ap_northeast_2,
    af_south_1,
    eu_central_1,
    eu_west_2,
    us_gov_east_1,
    us_west_1,
    us_west_2,
    eu_north_1,
    ap_southeast_2,
    EU,
    ap_south_1,
    eu_south_1,
    cn_northwest_1,
    eu_west_1,
    sa_east_1,
    ap_southeast_3,
    eu_west_3,
    us_gov_west_1,
    ap_southeast_1,
    cn_north_1,
    ap_northeast_1,
    ca_central_1,
}
impl AsRef<str> for BucketLocationConstraint {
    fn as_ref(&self) -> &str {
        match self {
            Self::me_south_1 => "me-south-1",
            Self::ap_northeast_3 => "ap-northeast-3",
            Self::ap_east_1 => "ap-east-1",
            Self::us_east_2 => "us-east-2",
            Self::ap_northeast_2 => "ap-northeast-2",
            Self::af_south_1 => "af-south-1",
            Self::eu_central_1 => "eu-central-1",
            Self::eu_west_2 => "eu-west-2",
            Self::us_gov_east_1 => "us-gov-east-1",
            Self::us_west_1 => "us-west-1",
            Self::us_west_2 => "us-west-2",
            Self::eu_north_1 => "eu-north-1",
            Self::ap_southeast_2 => "ap-southeast-2",
            Self::EU => "EU",
            Self::ap_south_1 => "ap-south-1",
            Self::eu_south_1 => "eu-south-1",
            Self::cn_northwest_1 => "cn-northwest-1",
            Self::eu_west_1 => "eu-west-1",
            Self::sa_east_1 => "sa-east-1",
            Self::ap_southeast_3 => "ap-southeast-3",
            Self::eu_west_3 => "eu-west-3",
            Self::us_gov_west_1 => "us-gov-west-1",
            Self::ap_southeast_1 => "ap-southeast-1",
            Self::cn_north_1 => "cn-north-1",
            Self::ap_northeast_1 => "ap-northeast-1",
            Self::ca_central_1 => "ca-central-1",
        }
    }
}
impl TryFrom<&str> for BucketLocationConstraint {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "me-south-1" => Ok(Self::me_south_1),
            "ap-northeast-3" => Ok(Self::ap_northeast_3),
            "ap-east-1" => Ok(Self::ap_east_1),
            "us-east-2" => Ok(Self::us_east_2),
            "ap-northeast-2" => Ok(Self::ap_northeast_2),
            "af-south-1" => Ok(Self::af_south_1),
            "eu-central-1" => Ok(Self::eu_central_1),
            "eu-west-2" => Ok(Self::eu_west_2),
            "us-gov-east-1" => Ok(Self::us_gov_east_1),
            "us-west-1" => Ok(Self::us_west_1),
            "us-west-2" => Ok(Self::us_west_2),
            "eu-north-1" => Ok(Self::eu_north_1),
            "ap-southeast-2" => Ok(Self::ap_southeast_2),
            "EU" => Ok(Self::EU),
            "ap-south-1" => Ok(Self::ap_south_1),
            "eu-south-1" => Ok(Self::eu_south_1),
            "cn-northwest-1" => Ok(Self::cn_northwest_1),
            "eu-west-1" => Ok(Self::eu_west_1),
            "sa-east-1" => Ok(Self::sa_east_1),
            "ap-southeast-3" => Ok(Self::ap_southeast_3),
            "eu-west-3" => Ok(Self::eu_west_3),
            "us-gov-west-1" => Ok(Self::us_gov_west_1),
            "ap-southeast-1" => Ok(Self::ap_southeast_1),
            "cn-north-1" => Ok(Self::cn_north_1),
            "ap-northeast-1" => Ok(Self::ap_northeast_1),
            "ca-central-1" => Ok(Self::ca_central_1),
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
pub struct NotificationConfiguration {
    pub event_bridge_configuration: Option<EventBridgeConfiguration>,
    pub queue_configurations: Option<QueueConfigurationList>,
    pub topic_configurations: Option<TopicConfigurationList>,
    pub lambda_function_configurations: Option<LambdaFunctionConfigurationList>,
}
impl NotificationConfiguration {
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

pub type PartsList = Vec<ObjectPart>;

#[derive(Debug, Default, Clone)]
pub struct GetObjectLegalHoldRequest {
    pub bucket: Option<BucketName>,
    pub key: Option<ObjectKey>,
    pub expected_bucket_owner: Option<AccountId>,
    pub request_payer: Option<RequestPayer>,
    pub version_id: Option<ObjectVersionId>,
}
impl GetObjectLegalHoldRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum ObjectLockLegalHoldStatus {
    OFF,
    ON,
}
impl AsRef<str> for ObjectLockLegalHoldStatus {
    fn as_ref(&self) -> &str {
        match self {
            Self::OFF => "OFF",
            Self::ON => "ON",
        }
    }
}
impl TryFrom<&str> for ObjectLockLegalHoldStatus {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "OFF" => Ok(Self::OFF),
            "ON" => Ok(Self::ON),
            _ => Err(format!("unknown enum value {}", s)),
        }
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

pub type Parts = Vec<Part>;

#[derive(Debug, Default, Clone)]
pub struct Tagging {
    pub tag_set: Option<TagSet>,
}
impl Tagging {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type TargetGrants = Vec<TargetGrant>;

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum Type {
    Group,
    CanonicalUser,
    AmazonCustomerByEmail,
}
impl AsRef<str> for Type {
    fn as_ref(&self) -> &str {
        match self {
            Self::Group => "Group",
            Self::CanonicalUser => "CanonicalUser",
            Self::AmazonCustomerByEmail => "AmazonCustomerByEmail",
        }
    }
}
impl TryFrom<&str> for Type {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "Group" => Ok(Self::Group),
            "CanonicalUser" => Ok(Self::CanonicalUser),
            "AmazonCustomerByEmail" => Ok(Self::AmazonCustomerByEmail),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct WebsiteConfiguration {
    pub routing_rules: Option<RoutingRules>,
    pub redirect_all_requests_to: Option<RedirectAllRequestsTo>,
    pub index_document: Option<IndexDocument>,
    pub error_document: Option<ErrorDocument>,
}
impl WebsiteConfiguration {
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

pub type RecordDelimiter = String;

pub type CacheControl = String;

#[derive(Debug, Default, Clone)]
pub struct RestoreObjectRequest {
    pub key: Option<ObjectKey>,
    pub bucket: Option<BucketName>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub restore_request: Option<RestoreRequest>,
    pub expected_bucket_owner: Option<AccountId>,
    pub version_id: Option<ObjectVersionId>,
    pub request_payer: Option<RequestPayer>,
}
impl RestoreObjectRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketIntelligentTieringConfigurationRequest {
    pub bucket: Option<BucketName>,
    pub id: Option<IntelligentTieringId>,
}
impl GetBucketIntelligentTieringConfigurationRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type LocationPrefix = String;

pub type Prefix = String;

pub type Buckets = Vec<Bucket>;

#[derive(Debug, Default, Clone)]
pub struct GetBucketCors;
impl OperationShape for GetBucketCors {
    const NAME: &'static str = "GetBucketCors";
    type Input = GetBucketCorsRequest;
    type Output = GetBucketCorsOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct ListObjectsV2Output {
    pub name: Option<BucketName>,
    pub start_after: Option<StartAfter>,
    pub key_count: Option<KeyCount>,
    pub contents: Option<ObjectList>,
    pub common_prefixes: Option<CommonPrefixList>,
    pub encoding_type: Option<EncodingType>,
    pub delimiter: Option<Delimiter>,
    pub continuation_token: Option<Token>,
    pub is_truncated: Option<IsTruncated>,
    pub next_continuation_token: Option<NextToken>,
    pub max_keys: Option<MaxKeys>,
    pub prefix: Option<Prefix>,
}
impl ListObjectsV2Output {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct AccessControlPolicy {
    pub owner: Option<Owner>,
    pub grants: Option<Grants>,
}
impl AccessControlPolicy {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum ObjectOwnership {
    ObjectWriter,
    BucketOwnerPreferred,
    BucketOwnerEnforced,
}
impl AsRef<str> for ObjectOwnership {
    fn as_ref(&self) -> &str {
        match self {
            Self::ObjectWriter => "ObjectWriter",
            Self::BucketOwnerPreferred => "BucketOwnerPreferred",
            Self::BucketOwnerEnforced => "BucketOwnerEnforced",
        }
    }
}
impl TryFrom<&str> for ObjectOwnership {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "ObjectWriter" => Ok(Self::ObjectWriter),
            "BucketOwnerPreferred" => Ok(Self::BucketOwnerPreferred),
            "BucketOwnerEnforced" => Ok(Self::BucketOwnerEnforced),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct PutBucketIntelligentTieringConfigurationRequest {
    pub bucket: Option<BucketName>,
    pub id: Option<IntelligentTieringId>,
    pub intelligent_tiering_configuration: Option<IntelligentTieringConfiguration>,
}
impl PutBucketIntelligentTieringConfigurationRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type RequestToken = String;

#[derive(Debug, Default, Clone)]
pub struct GetBucketWebsite;
impl OperationShape for GetBucketWebsite {
    const NAME: &'static str = "GetBucketWebsite";
    type Input = GetBucketWebsiteRequest;
    type Output = GetBucketWebsiteOutput;
    type Error = ();
}

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

pub type CopySourceIfNoneMatch = String;

#[derive(Debug, Default, Clone)]
pub struct Destination {
    pub encryption_configuration: Option<EncryptionConfiguration>,
    pub access_control_translation: Option<AccessControlTranslation>,
    pub account: Option<AccountId>,
    pub metrics: Option<Metrics>,
    pub replication_time: Option<ReplicationTime>,
    pub storage_class: Option<StorageClass>,
    pub bucket: Option<BucketName>,
}
impl Destination {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetObjectRetentionRequest {
    pub version_id: Option<ObjectVersionId>,
    pub expected_bucket_owner: Option<AccountId>,
    pub request_payer: Option<RequestPayer>,
    pub key: Option<ObjectKey>,
    pub bucket: Option<BucketName>,
}
impl GetObjectRetentionRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type NextKeyMarker = String;

pub type Body = Vec<u8>;

#[derive(Debug, Default, Clone)]
pub struct ObjectNotInActiveTierError {}
impl ObjectNotInActiveTierError {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct PutBucketWebsite;
impl OperationShape for PutBucketWebsite {
    const NAME: &'static str = "PutBucketWebsite";
    type Input = PutBucketWebsiteRequest;
    type Output = ();
    type Error = ();
}

pub type Suffix = String;

#[derive(Debug, Default, Clone)]
pub struct TopicConfiguration {
    pub events: Option<EventList>,
    pub topic_arn: Option<TopicArn>,
    pub filter: Option<NotificationConfigurationFilter>,
    pub id: Option<NotificationId>,
}
impl TopicConfiguration {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
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

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketOwnershipControlsRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}
impl DeleteBucketOwnershipControlsRequest {
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
pub struct WriteGetObjectResponse;
impl OperationShape for WriteGetObjectResponse {
    const NAME: &'static str = "WriteGetObjectResponse";
    type Input = WriteGetObjectResponseRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketEncryptionRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}
impl GetBucketEncryptionRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
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
pub struct ListObjectVersionsRequest {
    pub max_keys: Option<MaxKeys>,
    pub expected_bucket_owner: Option<AccountId>,
    pub key_marker: Option<KeyMarker>,
    pub bucket: Option<BucketName>,
    pub delimiter: Option<Delimiter>,
    pub encoding_type: Option<EncodingType>,
    pub prefix: Option<Prefix>,
    pub version_id_marker: Option<VersionIdMarker>,
}
impl ListObjectVersionsRequest {
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

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketInventoryConfigurationRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
    pub id: Option<InventoryId>,
}
impl DeleteBucketInventoryConfigurationRequest {
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

pub type NextVersionIdMarker = String;

pub type TopicConfigurationList = Vec<TopicConfiguration>;

#[derive(Debug, Default, Clone)]
pub struct UploadPart;
impl OperationShape for UploadPart {
    const NAME: &'static str = "UploadPart";
    type Input = UploadPartRequest;
    type Output = UploadPartOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketTagging;
impl OperationShape for GetBucketTagging {
    const NAME: &'static str = "GetBucketTagging";
    type Input = GetBucketTaggingRequest;
    type Output = GetBucketTaggingOutput;
    type Error = ();
}

pub type ContentRange = String;

#[derive(Debug, Default, Clone)]
pub struct PutBucketIntelligentTieringConfiguration;
impl OperationShape for PutBucketIntelligentTieringConfiguration {
    const NAME: &'static str = "PutBucketIntelligentTieringConfiguration";
    type Input = PutBucketIntelligentTieringConfigurationRequest;
    type Output = ();
    type Error = ();
}

pub type ContentLength = i64;

#[derive(Debug, Default, Clone)]
pub struct GetBucketTaggingRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}
impl GetBucketTaggingRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketLifecycleRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}
impl DeleteBucketLifecycleRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type AllowedOrigin = String;

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum TransitionStorageClass {
    ONEZONE_IA,
    DEEP_ARCHIVE,
    GLACIER,
    STANDARD_IA,
    INTELLIGENT_TIERING,
    GLACIER_IR,
}
impl AsRef<str> for TransitionStorageClass {
    fn as_ref(&self) -> &str {
        match self {
            Self::ONEZONE_IA => "ONEZONE_IA",
            Self::DEEP_ARCHIVE => "DEEP_ARCHIVE",
            Self::GLACIER => "GLACIER",
            Self::STANDARD_IA => "STANDARD_IA",
            Self::INTELLIGENT_TIERING => "INTELLIGENT_TIERING",
            Self::GLACIER_IR => "GLACIER_IR",
        }
    }
}
impl TryFrom<&str> for TransitionStorageClass {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "ONEZONE_IA" => Ok(Self::ONEZONE_IA),
            "DEEP_ARCHIVE" => Ok(Self::DEEP_ARCHIVE),
            "GLACIER" => Ok(Self::GLACIER),
            "STANDARD_IA" => Ok(Self::STANDARD_IA),
            "INTELLIGENT_TIERING" => Ok(Self::INTELLIGENT_TIERING),
            "GLACIER_IR" => Ok(Self::GLACIER_IR),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListBucketAnalyticsConfigurationsOutput {
    pub analytics_configuration_list: Option<AnalyticsConfigurationList>,
    pub next_continuation_token: Option<NextToken>,
    pub continuation_token: Option<Token>,
    pub is_truncated: Option<IsTruncated>,
}
impl ListBucketAnalyticsConfigurationsOutput {
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
pub struct CompleteMultipartUploadOutput {
    pub expiration: Option<Expiration>,
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
    pub version_id: Option<ObjectVersionId>,
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    pub checksum_sha1: Option<ChecksumSha1>,
    pub ssekms_key_id: Option<SsekmsKeyId>,
    pub e_tag: Option<ETag>,
    pub bucket: Option<BucketName>,
    pub checksum_sha256: Option<ChecksumSha256>,
    pub location: Option<Location>,
    pub server_side_encryption: Option<ServerSideEncryption>,
    pub checksum_crc32: Option<ChecksumCrc32>,
    pub key: Option<ObjectKey>,
    pub request_charged: Option<RequestCharged>,
}
impl CompleteMultipartUploadOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type KeyCount = i32;

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
pub struct SourceSelectionCriteria {
    pub replica_modifications: Option<ReplicaModifications>,
    pub sse_kms_encrypted_objects: Option<SseKmsEncryptedObjects>,
}
impl SourceSelectionCriteria {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

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
pub struct AbortMultipartUploadOutput {
    pub request_charged: Option<RequestCharged>,
}
impl AbortMultipartUploadOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type DeletedObjects = Vec<DeletedObject>;

pub type QuoteCharacter = String;

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketMetricsConfiguration;
impl OperationShape for DeleteBucketMetricsConfiguration {
    const NAME: &'static str = "DeleteBucketMetricsConfiguration";
    type Input = DeleteBucketMetricsConfigurationRequest;
    type Output = ();
    type Error = ();
}

pub type StreamingBlob = Arc<hyper::Body>;

pub type BucketKeyEnabled = bool;

pub type ExposeHeaders = Vec<ExposeHeader>;

pub type GrantRead = String;

pub type ExposeHeader = String;

pub type HttpRedirectCode = String;

#[derive(Debug, Default, Clone)]
pub struct PutBucketLifecycleConfigurationRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub lifecycle_configuration: Option<BucketLifecycleConfiguration>,
}
impl PutBucketLifecycleConfigurationRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type Token = String;

#[derive(Debug, Default, Clone)]
pub struct CsvOutput {
    pub quote_fields: Option<QuoteFields>,
    pub record_delimiter: Option<RecordDelimiter>,
    pub field_delimiter: Option<FieldDelimiter>,
    pub quote_escape_character: Option<QuoteEscapeCharacter>,
    pub quote_character: Option<QuoteCharacter>,
}
impl CsvOutput {
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

#[derive(Debug, Default, Clone)]
pub struct ListBucketInventoryConfigurationsOutput {
    pub next_continuation_token: Option<NextToken>,
    pub continuation_token: Option<Token>,
    pub is_truncated: Option<IsTruncated>,
    pub inventory_configuration_list: Option<InventoryConfigurationList>,
}
impl ListBucketInventoryConfigurationsOutput {
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

#[derive(Debug, Default, Clone)]
pub struct LoggingEnabled {
    pub target_bucket: Option<TargetBucket>,
    pub target_grants: Option<TargetGrants>,
    pub target_prefix: Option<TargetPrefix>,
}
impl LoggingEnabled {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct PutBucketTaggingRequest {
    pub bucket: Option<BucketName>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub expected_bucket_owner: Option<AccountId>,
    pub tagging: Option<Tagging>,
    pub content_md5: Option<ContentMd5>,
}
impl PutBucketTaggingRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

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
pub struct DeleteBucketPolicyRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}
impl DeleteBucketPolicyRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct EndEvent {}
impl EndEvent {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct IntelligentTieringFilter {
    pub prefix: Option<Prefix>,
    pub tag: Option<Tag>,
    pub and: Option<IntelligentTieringAndOperator>,
}
impl IntelligentTieringFilter {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type DisplayName = String;

pub type Code = String;

pub type Location = String;

#[derive(Debug, Default, Clone)]
pub struct PutBucketCorsRequest {
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub expected_bucket_owner: Option<AccountId>,
    pub content_md5: Option<ContentMd5>,
    pub cors_configuration: Option<CorsConfiguration>,
    pub bucket: Option<BucketName>,
}
impl PutBucketCorsRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
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

#[derive(Debug, Default, Clone)]
pub struct CreateMultipartUploadOutput {
    pub request_charged: Option<RequestCharged>,
    pub ssekms_encryption_context: Option<SsekmsEncryptionContext>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub abort_rule_id: Option<AbortRuleId>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub server_side_encryption: Option<ServerSideEncryption>,
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    pub abort_date: Option<AbortDate>,
    pub upload_id: Option<MultipartUploadId>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub key: Option<ObjectKey>,
    pub ssekms_key_id: Option<SsekmsKeyId>,
    pub bucket: Option<BucketName>,
}
impl CreateMultipartUploadOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
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
pub struct PutBucketVersioning;
impl OperationShape for PutBucketVersioning {
    const NAME: &'static str = "PutBucketVersioning";
    type Input = PutBucketVersioningRequest;
    type Output = ();
    type Error = ();
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
pub struct ListBucketAnalyticsConfigurationsRequest {
    pub continuation_token: Option<Token>,
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}
impl ListBucketAnalyticsConfigurationsRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type MetricsConfigurationList = Vec<MetricsConfiguration>;

#[derive(Debug, Default, Clone)]
pub struct DeleteMarkerEntry {
    pub owner: Option<Owner>,
    pub key: Option<ObjectKey>,
    pub last_modified: Option<LastModified>,
    pub version_id: Option<ObjectVersionId>,
    pub is_latest: Option<IsLatest>,
}
impl DeleteMarkerEntry {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ReplicaModifications {
    pub status: Option<ReplicaModificationsStatus>,
}
impl ReplicaModifications {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type ObjectLockEnabledForBucket = bool;

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

pub type ExpiredObjectDeleteMarker = bool;

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
pub struct BucketAlreadyOwnedByYou {}
impl BucketAlreadyOwnedByYou {
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
pub struct DeleteBucketReplication;
impl OperationShape for DeleteBucketReplication {
    const NAME: &'static str = "DeleteBucketReplication";
    type Input = DeleteBucketReplicationRequest;
    type Output = ();
    type Error = ();
}

pub type Role = String;

pub type RoutingRules = Vec<RoutingRule>;

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

pub type Message = String;

pub type Initiated = String;

#[derive(Debug, Default, Clone)]
pub struct GetBucketRequestPayment;
impl OperationShape for GetBucketRequestPayment {
    const NAME: &'static str = "GetBucketRequestPayment";
    type Input = GetBucketRequestPaymentRequest;
    type Output = GetBucketRequestPaymentOutput;
    type Error = ();
}

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

pub type IntelligentTieringConfigurationList = Vec<IntelligentTieringConfiguration>;

#[derive(Debug, Default, Clone)]
pub struct OwnershipControlsRule {
    pub object_ownership: Option<ObjectOwnership>,
}
impl OwnershipControlsRule {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type ConfirmRemoveSelfBucketAccess = bool;

pub type UploadIdMarker = String;

pub type Errors = Vec<Error>;

#[derive(Debug, Default, Clone)]
pub struct S3Location {
    pub canned_acl: Option<ObjectCannedAcl>,
    pub access_control_list: Option<Grants>,
    pub encryption: Option<Encryption>,
    pub tagging: Option<Tagging>,
    pub prefix: Option<LocationPrefix>,
    pub bucket_name: Option<BucketName>,
    pub storage_class: Option<StorageClass>,
    pub user_metadata: Option<UserMetadata>,
}
impl S3Location {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Clone)]
pub enum MetricsFilter {
    And(MetricsAndOperator),
    Prefix(Prefix),
    AccessPointArn(AccessPointArn),
    Tag(Tag),
}

pub type ChecksumAlgorithmList = Vec<ChecksumAlgorithm>;

pub type MissingMeta = i32;

pub type VersionIdMarker = String;

pub type Date = String;

pub type NextPartNumberMarker = String;

#[derive(Debug, Default, Clone)]
pub struct CreateMultipartUploadRequest {
    pub content_language: Option<ContentLanguage>,
    pub tagging: Option<TaggingHeader>,
    pub ssekms_encryption_context: Option<SsekmsEncryptionContext>,
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    pub bucket: Option<BucketName>,
    pub ssekms_key_id: Option<SsekmsKeyId>,
    pub grant_write_acp: Option<GrantWriteAcp>,
    pub sse_customer_key: Option<SseCustomerKey>,
    pub grant_full_control: Option<GrantFullControl>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub storage_class: Option<StorageClass>,
    pub object_lock_mode: Option<ObjectLockMode>,
    pub acl: Option<ObjectCannedAcl>,
    pub key: Option<ObjectKey>,
    pub object_lock_legal_hold_status: Option<ObjectLockLegalHoldStatus>,
    pub object_lock_retain_until_date: Option<ObjectLockRetainUntilDate>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub server_side_encryption: Option<ServerSideEncryption>,
    pub content_disposition: Option<ContentDisposition>,
    pub grant_read: Option<GrantRead>,
    pub grant_read_acp: Option<GrantReadAcp>,
    pub cache_control: Option<CacheControl>,
    pub request_payer: Option<RequestPayer>,
    pub expires: Option<Expires>,
    pub content_type: Option<ContentType>,
    pub metadata: Option<Metadata>,
    pub content_encoding: Option<ContentEncoding>,
    pub expected_bucket_owner: Option<AccountId>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub website_redirect_location: Option<WebsiteRedirectLocation>,
}
impl CreateMultipartUploadRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketLogging;
impl OperationShape for GetBucketLogging {
    const NAME: &'static str = "GetBucketLogging";
    type Input = GetBucketLoggingRequest;
    type Output = GetBucketLoggingOutput;
    type Error = ();
}

pub type Marker = String;

#[derive(Debug, Default, Clone)]
pub struct GetBucketLocationRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}
impl GetBucketLocationRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

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
pub struct DeleteObjects;
impl OperationShape for DeleteObjects {
    const NAME: &'static str = "DeleteObjects";
    type Input = DeleteObjectsRequest;
    type Output = DeleteObjectsOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct PutBucketWebsiteRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub content_md5: Option<ContentMd5>,
    pub website_configuration: Option<WebsiteConfiguration>,
    pub bucket: Option<BucketName>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
}
impl PutBucketWebsiteRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum ObjectLockRetentionMode {
    COMPLIANCE,
    GOVERNANCE,
}
impl AsRef<str> for ObjectLockRetentionMode {
    fn as_ref(&self) -> &str {
        match self {
            Self::COMPLIANCE => "COMPLIANCE",
            Self::GOVERNANCE => "GOVERNANCE",
        }
    }
}
impl TryFrom<&str> for ObjectLockRetentionMode {
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
pub struct ListBucketInventoryConfigurationsRequest {
    pub continuation_token: Option<Token>,
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}
impl ListBucketInventoryConfigurationsRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum SseKmsEncryptedObjectsStatus {
    Disabled,
    Enabled,
}
impl AsRef<str> for SseKmsEncryptedObjectsStatus {
    fn as_ref(&self) -> &str {
        match self {
            Self::Disabled => "Disabled",
            Self::Enabled => "Enabled",
        }
    }
}
impl TryFrom<&str> for SseKmsEncryptedObjectsStatus {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "Disabled" => Ok(Self::Disabled),
            "Enabled" => Ok(Self::Enabled),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

pub type Years = i32;

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
pub struct GetBucketMetricsConfigurationOutput {
    pub metrics_configuration: Option<MetricsConfiguration>,
}
impl GetBucketMetricsConfigurationOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
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
pub struct Ssekms {
    pub key_id: Option<SsekmsKeyId>,
}
impl Ssekms {
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

pub type TaggingHeader = String;

pub type Quiet = bool;

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

pub type ContentEncoding = String;

#[derive(Debug, Default, Clone)]
pub struct GetObjectRetentionOutput {
    pub retention: Option<ObjectLockRetention>,
}
impl GetObjectRetentionOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type IfNoneMatch = String;

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

#[derive(Debug, Default, Clone)]
pub struct ObjectPart {
    pub part_number: Option<PartNumber>,
    pub size: Option<Size>,
    pub checksum_sha256: Option<ChecksumSha256>,
    pub checksum_sha1: Option<ChecksumSha1>,
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
    pub checksum_crc32: Option<ChecksumCrc32>,
}
impl ObjectPart {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
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
pub struct ListBucketMetricsConfigurations;
impl OperationShape for ListBucketMetricsConfigurations {
    const NAME: &'static str = "ListBucketMetricsConfigurations";
    type Input = ListBucketMetricsConfigurationsRequest;
    type Output = ListBucketMetricsConfigurationsOutput;
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

pub type HttpErrorCodeReturnedEquals = String;

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

#[derive(Debug, Default, Clone)]
pub struct ObjectIdentifier {
    pub key: Option<ObjectKey>,
    pub version_id: Option<ObjectVersionId>,
}
impl ObjectIdentifier {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct PutBucketReplication;
impl OperationShape for PutBucketReplication {
    const NAME: &'static str = "PutBucketReplication";
    type Input = PutBucketReplicationRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct PutBucketLoggingRequest {
    pub bucket_logging_status: Option<BucketLoggingStatus>,
    pub bucket: Option<BucketName>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub expected_bucket_owner: Option<AccountId>,
    pub content_md5: Option<ContentMd5>,
}
impl PutBucketLoggingRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type ResponseContentLanguage = String;

#[derive(Debug, Default, Clone)]
pub struct PutBucketAcl;
impl OperationShape for PutBucketAcl {
    const NAME: &'static str = "PutBucketAcl";
    type Input = PutBucketAclRequest;
    type Output = ();
    type Error = ();
}

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
pub struct GetBucketOwnershipControls;
impl OperationShape for GetBucketOwnershipControls {
    const NAME: &'static str = "GetBucketOwnershipControls";
    type Input = GetBucketOwnershipControlsRequest;
    type Output = GetBucketOwnershipControlsOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct PutBucketMetricsConfigurationRequest {
    pub bucket: Option<BucketName>,
    pub id: Option<MetricsId>,
    pub expected_bucket_owner: Option<AccountId>,
    pub metrics_configuration: Option<MetricsConfiguration>,
}
impl PutBucketMetricsConfigurationRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type TopicArn = String;

pub type KeyPrefixEquals = String;

#[derive(Debug, Default, Clone)]
pub struct PutObjectRetentionRequest {
    pub content_md5: Option<ContentMd5>,
    pub bucket: Option<BucketName>,
    pub retention: Option<ObjectLockRetention>,
    pub request_payer: Option<RequestPayer>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub version_id: Option<ObjectVersionId>,
    pub expected_bucket_owner: Option<AccountId>,
    pub key: Option<ObjectKey>,
    pub bypass_governance_retention: Option<BypassGovernanceRetention>,
}
impl PutObjectRetentionRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type BytesProcessed = i64;

#[derive(Debug, Default, Clone)]
pub struct PutObjectOutput {
    pub checksum_sha1: Option<ChecksumSha1>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub ssekms_encryption_context: Option<SsekmsEncryptionContext>,
    pub ssekms_key_id: Option<SsekmsKeyId>,
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    pub request_charged: Option<RequestCharged>,
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
    pub server_side_encryption: Option<ServerSideEncryption>,
    pub version_id: Option<ObjectVersionId>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub checksum_sha256: Option<ChecksumSha256>,
    pub expiration: Option<Expiration>,
    pub e_tag: Option<ETag>,
    pub checksum_crc32: Option<ChecksumCrc32>,
}
impl PutObjectOutput {
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

pub type CopySourceSseCustomerAlgorithm = String;

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

#[derive(Debug, Default, Clone)]
pub struct PutObjectLegalHoldRequest {
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub version_id: Option<ObjectVersionId>,
    pub legal_hold: Option<ObjectLockLegalHold>,
    pub key: Option<ObjectKey>,
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
    pub content_md5: Option<ContentMd5>,
    pub request_payer: Option<RequestPayer>,
}
impl PutObjectLegalHoldRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct OutputSerialization {
    pub json: Option<JsonOutput>,
    pub csv: Option<CsvOutput>,
}
impl OutputSerialization {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type SseCustomerAlgorithm = String;

pub type End = i64;

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

pub type Delimiter = String;

#[derive(Debug, Default, Clone)]
pub struct Grantee {
    pub display_name: Option<DisplayName>,
    pub id: Option<Id>,
    pub uri: Option<Uri>,
    pub email_address: Option<EmailAddress>,
    pub r#type: Option<Type>,
}
impl Grantee {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Clone)]
pub enum LifecycleRuleFilter {
    Prefix(Prefix),
    ObjectSizeGreaterThan(ObjectSizeGreaterThanBytes),
    Tag(Tag),
    And(LifecycleRuleAndOperator),
    ObjectSizeLessThan(ObjectSizeLessThanBytes),
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

pub type Value = String;

pub type CompletedPartList = Vec<CompletedPart>;

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
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

#[derive(Debug, Clone)]
pub enum AnalyticsFilter {
    Prefix(Prefix),
    Tag(Tag),
    And(AnalyticsAndOperator),
}

#[derive(Debug, Default, Clone)]
pub struct ListBucketMetricsConfigurationsOutput {
    pub is_truncated: Option<IsTruncated>,
    pub next_continuation_token: Option<NextToken>,
    pub metrics_configuration_list: Option<MetricsConfigurationList>,
    pub continuation_token: Option<Token>,
}
impl ListBucketMetricsConfigurationsOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum ObjectCannedAcl {
    public_read,
    bucket_owner_read,
    public_read_write,
    bucket_owner_full_control,
    aws_exec_read,
    authenticated_read,
    private,
}
impl AsRef<str> for ObjectCannedAcl {
    fn as_ref(&self) -> &str {
        match self {
            Self::public_read => "public-read",
            Self::bucket_owner_read => "bucket-owner-read",
            Self::public_read_write => "public-read-write",
            Self::bucket_owner_full_control => "bucket-owner-full-control",
            Self::aws_exec_read => "aws-exec-read",
            Self::authenticated_read => "authenticated-read",
            Self::private => "private",
        }
    }
}
impl TryFrom<&str> for ObjectCannedAcl {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "public-read" => Ok(Self::public_read),
            "bucket-owner-read" => Ok(Self::bucket_owner_read),
            "public-read-write" => Ok(Self::public_read_write),
            "bucket-owner-full-control" => Ok(Self::bucket_owner_full_control),
            "aws-exec-read" => Ok(Self::aws_exec_read),
            "authenticated-read" => Ok(Self::authenticated_read),
            "private" => Ok(Self::private),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
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

pub type SsekmsKeyId = String;

#[derive(Debug, Default, Clone)]
pub struct DeleteObjectsRequest {
    pub bucket: Option<BucketName>,
    pub delete: Option<Delete>,
    pub expected_bucket_owner: Option<AccountId>,
    pub mfa: Option<Mfa>,
    pub request_payer: Option<RequestPayer>,
    pub bypass_governance_retention: Option<BypassGovernanceRetention>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
}
impl DeleteObjectsRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct PutObjectLockConfiguration;
impl OperationShape for PutObjectLockConfiguration {
    const NAME: &'static str = "PutObjectLockConfiguration";
    type Input = PutObjectLockConfigurationRequest;
    type Output = PutObjectLockConfigurationOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct ListBucketIntelligentTieringConfigurationsOutput {
    pub continuation_token: Option<Token>,
    pub intelligent_tiering_configuration_list: Option<IntelligentTieringConfigurationList>,
    pub is_truncated: Option<IsTruncated>,
    pub next_continuation_token: Option<NextToken>,
}
impl ListBucketIntelligentTieringConfigurationsOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type AllowedMethods = Vec<AllowedMethod>;

pub type Start = i64;

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

pub type ContentType = String;

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

#[derive(Debug, Default, Clone)]
pub struct GetBucketAclOutput {
    pub owner: Option<Owner>,
    pub grants: Option<Grants>,
}
impl GetBucketAclOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct CsvInput {
    pub field_delimiter: Option<FieldDelimiter>,
    pub file_header_info: Option<FileHeaderInfo>,
    pub quote_escape_character: Option<QuoteEscapeCharacter>,
    pub record_delimiter: Option<RecordDelimiter>,
    pub quote_character: Option<QuoteCharacter>,
    pub allow_quoted_record_delimiter: Option<AllowQuotedRecordDelimiter>,
    pub comments: Option<Comments>,
}
impl CsvInput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type DaysAfterInitiation = i32;

#[derive(Debug, Default, Clone)]
pub struct GetObjectAttributesParts {
    pub total_parts_count: Option<PartsCount>,
    pub next_part_number_marker: Option<NextPartNumberMarker>,
    pub parts: Option<PartsList>,
    pub part_number_marker: Option<PartNumberMarker>,
    pub is_truncated: Option<IsTruncated>,
    pub max_parts: Option<MaxParts>,
}
impl GetObjectAttributesParts {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetObjectTorrentRequest {
    pub key: Option<ObjectKey>,
    pub bucket: Option<BucketName>,
    pub request_payer: Option<RequestPayer>,
    pub expected_bucket_owner: Option<AccountId>,
}
impl GetObjectTorrentRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum ObjectStorageClass {
    STANDARD,
    OUTPOSTS,
    INTELLIGENT_TIERING,
    GLACIER_IR,
    STANDARD_IA,
    DEEP_ARCHIVE,
    REDUCED_REDUNDANCY,
    ONEZONE_IA,
    GLACIER,
}
impl AsRef<str> for ObjectStorageClass {
    fn as_ref(&self) -> &str {
        match self {
            Self::STANDARD => "STANDARD",
            Self::OUTPOSTS => "OUTPOSTS",
            Self::INTELLIGENT_TIERING => "INTELLIGENT_TIERING",
            Self::GLACIER_IR => "GLACIER_IR",
            Self::STANDARD_IA => "STANDARD_IA",
            Self::DEEP_ARCHIVE => "DEEP_ARCHIVE",
            Self::REDUCED_REDUNDANCY => "REDUCED_REDUNDANCY",
            Self::ONEZONE_IA => "ONEZONE_IA",
            Self::GLACIER => "GLACIER",
        }
    }
}
impl TryFrom<&str> for ObjectStorageClass {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "STANDARD" => Ok(Self::STANDARD),
            "OUTPOSTS" => Ok(Self::OUTPOSTS),
            "INTELLIGENT_TIERING" => Ok(Self::INTELLIGENT_TIERING),
            "GLACIER_IR" => Ok(Self::GLACIER_IR),
            "STANDARD_IA" => Ok(Self::STANDARD_IA),
            "DEEP_ARCHIVE" => Ok(Self::DEEP_ARCHIVE),
            "REDUCED_REDUNDANCY" => Ok(Self::REDUCED_REDUNDANCY),
            "ONEZONE_IA" => Ok(Self::ONEZONE_IA),
            "GLACIER" => Ok(Self::GLACIER),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

pub type FetchOwner = bool;

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketPolicy;
impl OperationShape for DeleteBucketPolicy {
    const NAME: &'static str = "DeleteBucketPolicy";
    type Input = DeleteBucketPolicyRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct ListMultipartUploads;
impl OperationShape for ListMultipartUploads {
    const NAME: &'static str = "ListMultipartUploads";
    type Input = ListMultipartUploadsRequest;
    type Output = ListMultipartUploadsOutput;
    type Error = ();
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

pub type MetadataValue = String;

#[derive(Debug, Default, Clone)]
pub struct PutObjectTaggingRequest {
    pub content_md5: Option<ContentMd5>,
    pub key: Option<ObjectKey>,
    pub tagging: Option<Tagging>,
    pub bucket: Option<BucketName>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub expected_bucket_owner: Option<AccountId>,
    pub version_id: Option<ObjectVersionId>,
    pub request_payer: Option<RequestPayer>,
}
impl PutObjectTaggingRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type ChecksumCrc32c = String;

pub type PartNumberMarker = String;

pub type ResponseContentDisposition = String;

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
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
pub struct DeleteBucketOwnershipControls;
impl OperationShape for DeleteBucketOwnershipControls {
    const NAME: &'static str = "DeleteBucketOwnershipControls";
    type Input = DeleteBucketOwnershipControlsRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketPolicyStatusOutput {
    pub policy_status: Option<PolicyStatus>,
}
impl GetBucketPolicyStatusOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct EventBridgeConfiguration {}
impl EventBridgeConfiguration {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
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
pub struct ListBucketIntelligentTieringConfigurationsRequest {
    pub continuation_token: Option<Token>,
    pub bucket: Option<BucketName>,
}
impl ListBucketIntelligentTieringConfigurationsRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct Object {
    pub owner: Option<Owner>,
    pub key: Option<ObjectKey>,
    pub storage_class: Option<ObjectStorageClass>,
    pub e_tag: Option<ETag>,
    pub checksum_algorithm: Option<ChecksumAlgorithmList>,
    pub last_modified: Option<LastModified>,
    pub size: Option<Size>,
}
impl Object {
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

#[derive(Debug, Default, Clone)]
pub struct Encryption {
    pub kms_context: Option<KmsContext>,
    pub kms_key_id: Option<SsekmsKeyId>,
    pub encryption_type: Option<ServerSideEncryption>,
}
impl Encryption {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct CompletedMultipartUpload {
    pub parts: Option<CompletedPartList>,
}
impl CompletedMultipartUpload {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketAccelerateConfigurationRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}
impl GetBucketAccelerateConfigurationRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type Policy = String;

#[derive(Debug, Default, Clone)]
pub struct PutBucketLifecycleConfiguration;
impl OperationShape for PutBucketLifecycleConfiguration {
    const NAME: &'static str = "PutBucketLifecycleConfiguration";
    type Input = PutBucketLifecycleConfigurationRequest;
    type Output = ();
    type Error = ();
}

pub type AllowQuotedRecordDelimiter = bool;

pub type AllowedHeaders = Vec<AllowedHeader>;

#[derive(Debug, Default, Clone)]
pub struct WriteGetObjectResponseRequest {
    pub content_range: Option<ContentRange>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub object_lock_mode: Option<ObjectLockMode>,
    pub content_language: Option<ContentLanguage>,
    pub accept_ranges: Option<AcceptRanges>,
    pub request_charged: Option<RequestCharged>,
    pub storage_class: Option<StorageClass>,
    pub parts_count: Option<PartsCount>,
    pub checksum_sha256: Option<ChecksumSha256>,
    pub checksum_crc32: Option<ChecksumCrc32>,
    pub content_encoding: Option<ContentEncoding>,
    pub content_type: Option<ContentType>,
    pub delete_marker: Option<DeleteMarker>,
    pub e_tag: Option<ETag>,
    pub error_message: Option<ErrorMessage>,
    pub ssekms_key_id: Option<SsekmsKeyId>,
    pub body: Option<StreamingBlob>,
    pub replication_status: Option<ReplicationStatus>,
    pub error_code: Option<ErrorCode>,
    pub tag_count: Option<TagCount>,
    pub content_disposition: Option<ContentDisposition>,
    pub request_token: Option<RequestToken>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub last_modified: Option<LastModified>,
    pub object_lock_legal_hold_status: Option<ObjectLockLegalHoldStatus>,
    pub expires: Option<Expires>,
    pub checksum_sha1: Option<ChecksumSha1>,
    pub expiration: Option<Expiration>,
    pub cache_control: Option<CacheControl>,
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    pub restore: Option<Restore>,
    pub content_length: Option<ContentLength>,
    pub object_lock_retain_until_date: Option<ObjectLockRetainUntilDate>,
    pub status_code: Option<GetObjectResponseStatusCode>,
    pub request_route: Option<RequestRoute>,
    pub server_side_encryption: Option<ServerSideEncryption>,
    pub version_id: Option<ObjectVersionId>,
    pub missing_meta: Option<MissingMeta>,
    pub metadata: Option<Metadata>,
}
impl WriteGetObjectResponseRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type FieldDelimiter = String;

#[derive(Debug, Default, Clone)]
pub struct DeleteObjectTaggingRequest {
    pub key: Option<ObjectKey>,
    pub version_id: Option<ObjectVersionId>,
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}
impl DeleteObjectTaggingRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct IntelligentTieringConfiguration {
    pub tierings: Option<TieringList>,
    pub id: Option<IntelligentTieringId>,
    pub status: Option<IntelligentTieringStatus>,
    pub filter: Option<IntelligentTieringFilter>,
}
impl IntelligentTieringConfiguration {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type GrantWrite = String;

#[derive(Debug, Default, Clone)]
pub struct GetBucketVersioning;
impl OperationShape for GetBucketVersioning {
    const NAME: &'static str = "GetBucketVersioning";
    type Input = GetBucketVersioningRequest;
    type Output = GetBucketVersioningOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct ListMultipartUploadsRequest {
    pub upload_id_marker: Option<UploadIdMarker>,
    pub bucket: Option<BucketName>,
    pub max_uploads: Option<MaxUploads>,
    pub delimiter: Option<Delimiter>,
    pub prefix: Option<Prefix>,
    pub expected_bucket_owner: Option<AccountId>,
    pub key_marker: Option<KeyMarker>,
    pub encoding_type: Option<EncodingType>,
}
impl ListMultipartUploadsRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoSuchBucket {}
impl NoSuchBucket {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct PutPublicAccessBlockRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub public_access_block_configuration: Option<PublicAccessBlockConfiguration>,
    pub bucket: Option<BucketName>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub content_md5: Option<ContentMd5>,
}
impl PutPublicAccessBlockRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type IntelligentTieringDays = i32;

pub type CopySourceIfUnmodifiedSince = String;

#[derive(Debug, Default, Clone)]
pub struct CorsConfiguration {
    pub cors_rules: Option<CorsRules>,
}
impl CorsConfiguration {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type NotificationId = String;

#[derive(Debug, Default, Clone)]
pub struct HeadBucketRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}
impl HeadBucketRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoSuchKey {}
impl NoSuchKey {
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

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum BucketLogsPermission {
    READ,
    FULL_CONTROL,
    WRITE,
}
impl AsRef<str> for BucketLogsPermission {
    fn as_ref(&self) -> &str {
        match self {
            Self::READ => "READ",
            Self::FULL_CONTROL => "FULL_CONTROL",
            Self::WRITE => "WRITE",
        }
    }
}
impl TryFrom<&str> for BucketLogsPermission {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "READ" => Ok(Self::READ),
            "FULL_CONTROL" => Ok(Self::FULL_CONTROL),
            "WRITE" => Ok(Self::WRITE),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

pub type BytesScanned = i64;

#[derive(Debug, Default, Clone)]
pub struct GetObjectTaggingRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub key: Option<ObjectKey>,
    pub version_id: Option<ObjectVersionId>,
    pub request_payer: Option<RequestPayer>,
    pub bucket: Option<BucketName>,
}
impl GetObjectTaggingRequest {
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
pub struct GetPublicAccessBlock;
impl OperationShape for GetPublicAccessBlock {
    const NAME: &'static str = "GetPublicAccessBlock";
    type Input = GetPublicAccessBlockRequest;
    type Output = GetPublicAccessBlockOutput;
    type Error = ();
}

pub type MetadataKey = String;

pub type MultipartUploadId = String;

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum ObjectAttributes {
    ETAG,
    STORAGE_CLASS,
    OBJECT_SIZE,
    OBJECT_PARTS,
    CHECKSUM,
}
impl AsRef<str> for ObjectAttributes {
    fn as_ref(&self) -> &str {
        match self {
            Self::ETAG => "ETag",
            Self::STORAGE_CLASS => "StorageClass",
            Self::OBJECT_SIZE => "ObjectSize",
            Self::OBJECT_PARTS => "ObjectParts",
            Self::CHECKSUM => "Checksum",
        }
    }
}
impl TryFrom<&str> for ObjectAttributes {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "ETag" => Ok(Self::ETAG),
            "StorageClass" => Ok(Self::STORAGE_CLASS),
            "ObjectSize" => Ok(Self::OBJECT_SIZE),
            "ObjectParts" => Ok(Self::OBJECT_PARTS),
            "Checksum" => Ok(Self::CHECKSUM),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
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

pub type TransitionList = Vec<Transition>;

#[derive(Debug, Default, Clone)]
pub struct LifecycleRule {
    pub noncurrent_version_expiration: Option<NoncurrentVersionExpiration>,
    pub abort_incomplete_multipart_upload: Option<AbortIncompleteMultipartUpload>,
    pub expiration: Option<LifecycleExpiration>,
    pub filter: Option<LifecycleRuleFilter>,
    pub noncurrent_version_transitions: Option<NoncurrentVersionTransitionList>,
    pub status: Option<ExpirationStatus>,
    pub transitions: Option<TransitionList>,
    pub id: Option<Id>,
    pub prefix: Option<Prefix>,
}
impl LifecycleRule {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct CreateBucketConfiguration {
    pub location_constraint: Option<BucketLocationConstraint>,
}
impl CreateBucketConfiguration {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type Expression = String;

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

#[derive(Debug, Default, Clone)]
pub struct GetObjectRetention;
impl OperationShape for GetObjectRetention {
    const NAME: &'static str = "GetObjectRetention";
    type Input = GetObjectRetentionRequest;
    type Output = GetObjectRetentionOutput;
    type Error = ();
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

#[derive(Debug, Default, Clone)]
pub struct ListPartsOutput {
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub owner: Option<Owner>,
    pub part_number_marker: Option<PartNumberMarker>,
    pub bucket: Option<BucketName>,
    pub key: Option<ObjectKey>,
    pub max_parts: Option<MaxParts>,
    pub abort_date: Option<AbortDate>,
    pub request_charged: Option<RequestCharged>,
    pub upload_id: Option<MultipartUploadId>,
    pub next_part_number_marker: Option<NextPartNumberMarker>,
    pub abort_rule_id: Option<AbortRuleId>,
    pub initiator: Option<Initiator>,
    pub is_truncated: Option<IsTruncated>,
    pub storage_class: Option<StorageClass>,
    pub parts: Option<Parts>,
}
impl ListPartsOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type ResponseCacheControl = String;

pub type FilterRuleValue = String;

#[derive(Debug, Default, Clone)]
pub struct GetBucketCorsRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}
impl GetBucketCorsRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ObjectLockRetention {
    pub retain_until_date: Option<Date>,
    pub mode: Option<ObjectLockRetentionMode>,
}
impl ObjectLockRetention {
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

#[derive(Debug, Default, Clone)]
pub struct SelectParameters {
    pub expression_type: Option<ExpressionType>,
    pub expression: Option<Expression>,
    pub output_serialization: Option<OutputSerialization>,
    pub input_serialization: Option<InputSerialization>,
}
impl SelectParameters {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct InputSerialization {
    pub compression_type: Option<CompressionType>,
    pub csv: Option<CsvInput>,
    pub parquet: Option<ParquetInput>,
    pub json: Option<JsonInput>,
}
impl InputSerialization {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct Initiator {
    pub display_name: Option<DisplayName>,
    pub id: Option<Id>,
}
impl Initiator {
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
pub struct PutBucketEncryption;
impl OperationShape for PutBucketEncryption {
    const NAME: &'static str = "PutBucketEncryption";
    type Input = PutBucketEncryptionRequest;
    type Output = ();
    type Error = ();
}

pub type Expiration = String;

pub type MetricsId = String;

#[derive(Debug, Default, Clone)]
pub struct PutBucketTagging;
impl OperationShape for PutBucketTagging {
    const NAME: &'static str = "PutBucketTagging";
    type Input = PutBucketTaggingRequest;
    type Output = ();
    type Error = ();
}

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
pub struct DeleteBucket;
impl OperationShape for DeleteBucket {
    const NAME: &'static str = "DeleteBucket";
    type Input = DeleteBucketRequest;
    type Output = ();
    type Error = ();
}

pub type UserMetadata = Vec<MetadataEntry>;

pub type KmsContext = String;

pub type StartAfter = String;

#[derive(Debug, Default, Clone)]
pub struct ListBucketIntelligentTieringConfigurations;
impl OperationShape for ListBucketIntelligentTieringConfigurations {
    const NAME: &'static str = "ListBucketIntelligentTieringConfigurations";
    type Input = ListBucketIntelligentTieringConfigurationsRequest;
    type Output = ListBucketIntelligentTieringConfigurationsOutput;
    type Error = ();
}

pub type ReplaceKeyPrefixWith = String;

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

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum BucketCannedAcl {
    public_read,
    public_read_write,
    authenticated_read,
    private,
}
impl AsRef<str> for BucketCannedAcl {
    fn as_ref(&self) -> &str {
        match self {
            Self::public_read => "public-read",
            Self::public_read_write => "public-read-write",
            Self::authenticated_read => "authenticated-read",
            Self::private => "private",
        }
    }
}
impl TryFrom<&str> for BucketCannedAcl {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "public-read" => Ok(Self::public_read),
            "public-read-write" => Ok(Self::public_read_write),
            "authenticated-read" => Ok(Self::authenticated_read),
            "private" => Ok(Self::private),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

pub type FilterRuleList = Vec<FilterRule>;

pub type GrantFullControl = String;

pub type IsPublic = bool;

#[derive(Debug, Default, Clone)]
pub struct GetObjectOutput {
    pub checksum_sha1: Option<ChecksumSha1>,
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
    pub delete_marker: Option<DeleteMarker>,
    pub website_redirect_location: Option<WebsiteRedirectLocation>,
    pub missing_meta: Option<MissingMeta>,
    pub storage_class: Option<StorageClass>,
    pub version_id: Option<ObjectVersionId>,
    pub last_modified: Option<LastModified>,
    pub object_lock_legal_hold_status: Option<ObjectLockLegalHoldStatus>,
    pub checksum_sha256: Option<ChecksumSha256>,
    pub object_lock_mode: Option<ObjectLockMode>,
    pub object_lock_retain_until_date: Option<ObjectLockRetainUntilDate>,
    pub accept_ranges: Option<AcceptRanges>,
    pub tag_count: Option<TagCount>,
    pub checksum_crc32: Option<ChecksumCrc32>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub parts_count: Option<PartsCount>,
    pub ssekms_key_id: Option<SsekmsKeyId>,
    pub replication_status: Option<ReplicationStatus>,
    pub content_language: Option<ContentLanguage>,
    pub e_tag: Option<ETag>,
    pub restore: Option<Restore>,
    pub metadata: Option<Metadata>,
    pub request_charged: Option<RequestCharged>,
    pub content_encoding: Option<ContentEncoding>,
    pub expiration: Option<Expiration>,
    pub content_length: Option<ContentLength>,
    pub cache_control: Option<CacheControl>,
    pub content_range: Option<ContentRange>,
    pub content_type: Option<ContentType>,
    pub content_disposition: Option<ContentDisposition>,
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    pub expires: Option<Expires>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub body: Option<StreamingBlob>,
    pub server_side_encryption: Option<ServerSideEncryption>,
}
impl GetObjectOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type ContentDisposition = String;

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
pub struct Progress {
    pub bytes_scanned: Option<BytesScanned>,
    pub bytes_processed: Option<BytesProcessed>,
    pub bytes_returned: Option<BytesReturned>,
}
impl Progress {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct DefaultRetention {
    pub mode: Option<ObjectLockRetentionMode>,
    pub days: Option<Days>,
    pub years: Option<Years>,
}
impl DefaultRetention {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type RestoreOutputPath = String;

#[derive(Debug, Default, Clone)]
pub struct GetObjectLegalHoldOutput {
    pub legal_hold: Option<ObjectLockLegalHold>,
}
impl GetObjectLegalHoldOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type NextToken = String;

#[derive(Debug, Default, Clone)]
pub struct PutBucketOwnershipControlsRequest {
    pub ownership_controls: Option<OwnershipControls>,
    pub bucket: Option<BucketName>,
    pub content_md5: Option<ContentMd5>,
    pub expected_bucket_owner: Option<AccountId>,
}
impl PutBucketOwnershipControlsRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type IsLatest = bool;

#[derive(Debug, Default, Clone)]
pub struct ListObjectsOutput {
    pub next_marker: Option<NextMarker>,
    pub name: Option<BucketName>,
    pub contents: Option<ObjectList>,
    pub marker: Option<Marker>,
    pub is_truncated: Option<IsTruncated>,
    pub max_keys: Option<MaxKeys>,
    pub encoding_type: Option<EncodingType>,
    pub delimiter: Option<Delimiter>,
    pub prefix: Option<Prefix>,
    pub common_prefixes: Option<CommonPrefixList>,
}
impl ListObjectsOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct PutBucketPolicyRequest {
    pub bucket: Option<BucketName>,
    pub content_md5: Option<ContentMd5>,
    pub confirm_remove_self_bucket_access: Option<ConfirmRemoveSelfBucketAccess>,
    pub expected_bucket_owner: Option<AccountId>,
    pub policy: Option<Policy>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
}
impl PutBucketPolicyRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type ObjectVersionList = Vec<ObjectVersion>;

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
pub struct UploadPartRequest {
    pub request_payer: Option<RequestPayer>,
    pub upload_id: Option<MultipartUploadId>,
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
    pub key: Option<ObjectKey>,
    pub content_length: Option<ContentLength>,
    pub checksum_sha1: Option<ChecksumSha1>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub checksum_crc32: Option<ChecksumCrc32>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub sse_customer_key: Option<SseCustomerKey>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub bucket: Option<BucketName>,
    pub body: Option<StreamingBlob>,
    pub checksum_sha256: Option<ChecksumSha256>,
    pub content_md5: Option<ContentMd5>,
    pub expected_bucket_owner: Option<AccountId>,
    pub part_number: Option<PartNumber>,
}
impl UploadPartRequest {
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

pub type CopySourceIfModifiedSince = String;

pub type SsekmsEncryptionContext = String;

#[derive(Debug, Default, Clone)]
pub struct GetObject;
impl OperationShape for GetObject {
    const NAME: &'static str = "GetObject";
    type Input = GetObjectRequest;
    type Output = GetObjectOutput;
    type Error = ();
}

pub type ObjectLockRetainUntilDate = String;

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
pub struct CopyObjectOutput {
    pub copy_object_result: Option<CopyObjectResult>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub request_charged: Option<RequestCharged>,
    pub ssekms_encryption_context: Option<SsekmsEncryptionContext>,
    pub server_side_encryption: Option<ServerSideEncryption>,
    pub expiration: Option<Expiration>,
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    pub copy_source_version_id: Option<CopySourceVersionId>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub ssekms_key_id: Option<SsekmsKeyId>,
    pub version_id: Option<ObjectVersionId>,
}
impl CopyObjectOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum Tier {
    Expedited,
    Bulk,
    Standard,
}
impl AsRef<str> for Tier {
    fn as_ref(&self) -> &str {
        match self {
            Self::Expedited => "Expedited",
            Self::Bulk => "Bulk",
            Self::Standard => "Standard",
        }
    }
}
impl TryFrom<&str> for Tier {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "Expedited" => Ok(Self::Expedited),
            "Bulk" => Ok(Self::Bulk),
            "Standard" => Ok(Self::Standard),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

pub type ETag = String;

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
pub struct JsonInput {
    pub r#type: Option<JsonType>,
}
impl JsonInput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct PutBucketCors;
impl OperationShape for PutBucketCors {
    const NAME: &'static str = "PutBucketCors";
    type Input = PutBucketCorsRequest;
    type Output = ();
    type Error = ();
}

pub type ObjectSize = i64;

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum StorageClass {
    STANDARD_IA,
    ONEZONE_IA,
    REDUCED_REDUNDANCY,
    STANDARD,
    DEEP_ARCHIVE,
    GLACIER_IR,
    GLACIER,
    INTELLIGENT_TIERING,
    OUTPOSTS,
}
impl AsRef<str> for StorageClass {
    fn as_ref(&self) -> &str {
        match self {
            Self::STANDARD_IA => "STANDARD_IA",
            Self::ONEZONE_IA => "ONEZONE_IA",
            Self::REDUCED_REDUNDANCY => "REDUCED_REDUNDANCY",
            Self::STANDARD => "STANDARD",
            Self::DEEP_ARCHIVE => "DEEP_ARCHIVE",
            Self::GLACIER_IR => "GLACIER_IR",
            Self::GLACIER => "GLACIER",
            Self::INTELLIGENT_TIERING => "INTELLIGENT_TIERING",
            Self::OUTPOSTS => "OUTPOSTS",
        }
    }
}
impl TryFrom<&str> for StorageClass {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "STANDARD_IA" => Ok(Self::STANDARD_IA),
            "ONEZONE_IA" => Ok(Self::ONEZONE_IA),
            "REDUCED_REDUNDANCY" => Ok(Self::REDUCED_REDUNDANCY),
            "STANDARD" => Ok(Self::STANDARD),
            "DEEP_ARCHIVE" => Ok(Self::DEEP_ARCHIVE),
            "GLACIER_IR" => Ok(Self::GLACIER_IR),
            "GLACIER" => Ok(Self::GLACIER),
            "INTELLIGENT_TIERING" => Ok(Self::INTELLIGENT_TIERING),
            "OUTPOSTS" => Ok(Self::OUTPOSTS),
            _ => Err(format!("unknown enum value {}", s)),
        }
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
pub struct PutObject;
impl OperationShape for PutObject {
    const NAME: &'static str = "PutObject";
    type Input = PutObjectRequest;
    type Output = PutObjectOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct ServerSideEncryptionRule {
    pub apply_server_side_encryption_by_default: Option<ServerSideEncryptionByDefault>,
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
}
impl ServerSideEncryptionRule {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketCorsRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}
impl DeleteBucketCorsRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct PutObjectRequest {
    pub storage_class: Option<StorageClass>,
    pub content_disposition: Option<ContentDisposition>,
    pub object_lock_legal_hold_status: Option<ObjectLockLegalHoldStatus>,
    pub body: Option<StreamingBlob>,
    pub bucket: Option<BucketName>,
    pub checksum_sha256: Option<ChecksumSha256>,
    pub tagging: Option<TaggingHeader>,
    pub metadata: Option<Metadata>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub ssekms_key_id: Option<SsekmsKeyId>,
    pub object_lock_mode: Option<ObjectLockMode>,
    pub server_side_encryption: Option<ServerSideEncryption>,
    pub acl: Option<ObjectCannedAcl>,
    pub checksum_sha1: Option<ChecksumSha1>,
    pub grant_full_control: Option<GrantFullControl>,
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    pub content_length: Option<ContentLength>,
    pub expires: Option<Expires>,
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
    pub grant_write_acp: Option<GrantWriteAcp>,
    pub content_encoding: Option<ContentEncoding>,
    pub ssekms_encryption_context: Option<SsekmsEncryptionContext>,
    pub website_redirect_location: Option<WebsiteRedirectLocation>,
    pub key: Option<ObjectKey>,
    pub grant_read: Option<GrantRead>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub content_language: Option<ContentLanguage>,
    pub expected_bucket_owner: Option<AccountId>,
    pub cache_control: Option<CacheControl>,
    pub sse_customer_key: Option<SseCustomerKey>,
    pub content_type: Option<ContentType>,
    pub content_md5: Option<ContentMd5>,
    pub checksum_crc32: Option<ChecksumCrc32>,
    pub grant_read_acp: Option<GrantReadAcp>,
    pub object_lock_retain_until_date: Option<ObjectLockRetainUntilDate>,
    pub request_payer: Option<RequestPayer>,
}
impl PutObjectRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
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
pub struct CopyObjectRequest {
    pub acl: Option<ObjectCannedAcl>,
    pub expected_source_bucket_owner: Option<AccountId>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    pub metadata: Option<Metadata>,
    pub storage_class: Option<StorageClass>,
    pub grant_read_acp: Option<GrantReadAcp>,
    pub copy_source_if_match: Option<CopySourceIfMatch>,
    pub copy_source_sse_customer_key: Option<CopySourceSseCustomerKey>,
    pub tagging_directive: Option<TaggingDirective>,
    pub request_payer: Option<RequestPayer>,
    pub copy_source_sse_customer_algorithm: Option<CopySourceSseCustomerAlgorithm>,
    pub content_encoding: Option<ContentEncoding>,
    pub sse_customer_key: Option<SseCustomerKey>,
    pub copy_source_if_unmodified_since: Option<CopySourceIfUnmodifiedSince>,
    pub content_language: Option<ContentLanguage>,
    pub expires: Option<Expires>,
    pub server_side_encryption: Option<ServerSideEncryption>,
    pub expected_bucket_owner: Option<AccountId>,
    pub cache_control: Option<CacheControl>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub copy_source_if_none_match: Option<CopySourceIfNoneMatch>,
    pub copy_source_sse_customer_key_md5: Option<CopySourceSseCustomerKeyMd5>,
    pub grant_full_control: Option<GrantFullControl>,
    pub grant_read: Option<GrantRead>,
    pub object_lock_mode: Option<ObjectLockMode>,
    pub object_lock_retain_until_date: Option<ObjectLockRetainUntilDate>,
    pub ssekms_encryption_context: Option<SsekmsEncryptionContext>,
    pub content_type: Option<ContentType>,
    pub tagging: Option<TaggingHeader>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub copy_source: Option<CopySource>,
    pub copy_source_if_modified_since: Option<CopySourceIfModifiedSince>,
    pub website_redirect_location: Option<WebsiteRedirectLocation>,
    pub object_lock_legal_hold_status: Option<ObjectLockLegalHoldStatus>,
    pub key: Option<ObjectKey>,
    pub bucket: Option<BucketName>,
    pub grant_write_acp: Option<GrantWriteAcp>,
    pub content_disposition: Option<ContentDisposition>,
    pub metadata_directive: Option<MetadataDirective>,
    pub ssekms_key_id: Option<SsekmsKeyId>,
}
impl CopyObjectRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct DeleteObjectRequest {
    pub version_id: Option<ObjectVersionId>,
    pub bucket: Option<BucketName>,
    pub bypass_governance_retention: Option<BypassGovernanceRetention>,
    pub expected_bucket_owner: Option<AccountId>,
    pub mfa: Option<Mfa>,
    pub request_payer: Option<RequestPayer>,
    pub key: Option<ObjectKey>,
}
impl DeleteObjectRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

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
pub struct LifecycleExpiration {
    pub expired_object_delete_marker: Option<ExpiredObjectDeleteMarker>,
    pub date: Option<Date>,
    pub days: Option<Days>,
}
impl LifecycleExpiration {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct Sses3 {}
impl Sses3 {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type ContentMd5 = String;

pub type CopySourceIfMatch = String;

#[derive(Debug, Default, Clone)]
pub struct InventoryS3BucketDestination {
    pub encryption: Option<InventoryEncryption>,
    pub format: Option<InventoryFormat>,
    pub account_id: Option<AccountId>,
    pub prefix: Option<Prefix>,
    pub bucket: Option<BucketName>,
}
impl InventoryS3BucketDestination {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListBucketAnalyticsConfigurations;
impl OperationShape for ListBucketAnalyticsConfigurations {
    const NAME: &'static str = "ListBucketAnalyticsConfigurations";
    type Input = ListBucketAnalyticsConfigurationsRequest;
    type Output = ListBucketAnalyticsConfigurationsOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct CopyPartResult {
    pub checksum_sha256: Option<ChecksumSha256>,
    pub last_modified: Option<LastModified>,
    pub e_tag: Option<ETag>,
    pub checksum_sha1: Option<ChecksumSha1>,
    pub checksum_crc32: Option<ChecksumCrc32>,
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
}
impl CopyPartResult {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type MaxUploads = i32;

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
pub struct MetricsAndOperator {
    pub prefix: Option<Prefix>,
    pub access_point_arn: Option<AccessPointArn>,
    pub tags: Option<TagSet>,
}
impl MetricsAndOperator {
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
pub struct StorageClassAnalysisDataExport {
    pub destination: Option<AnalyticsExportDestination>,
    pub output_schema_version: Option<StorageClassAnalysisSchemaVersion>,
}
impl StorageClassAnalysisDataExport {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type ReplaceKeyWith = String;

#[derive(Debug, Default, Clone)]
pub struct ListPartsRequest {
    pub bucket: Option<BucketName>,
    pub upload_id: Option<MultipartUploadId>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub request_payer: Option<RequestPayer>,
    pub key: Option<ObjectKey>,
    pub max_parts: Option<MaxParts>,
    pub expected_bucket_owner: Option<AccountId>,
    pub part_number_marker: Option<PartNumberMarker>,
    pub sse_customer_key: Option<SseCustomerKey>,
}
impl ListPartsRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type DeleteMarkers = Vec<DeleteMarkerEntry>;

#[derive(Debug, Default, Clone)]
pub struct PutObjectAclRequest {
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub expected_bucket_owner: Option<AccountId>,
    pub grant_full_control: Option<GrantFullControl>,
    pub grant_write: Option<GrantWrite>,
    pub bucket: Option<BucketName>,
    pub request_payer: Option<RequestPayer>,
    pub grant_read: Option<GrantRead>,
    pub grant_write_acp: Option<GrantWriteAcp>,
    pub access_control_policy: Option<AccessControlPolicy>,
    pub content_md5: Option<ContentMd5>,
    pub version_id: Option<ObjectVersionId>,
    pub acl: Option<ObjectCannedAcl>,
    pub grant_read_acp: Option<GrantReadAcp>,
    pub key: Option<ObjectKey>,
}
impl PutObjectAclRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct PutBucketMetricsConfiguration;
impl OperationShape for PutBucketMetricsConfiguration {
    const NAME: &'static str = "PutBucketMetricsConfiguration";
    type Input = PutBucketMetricsConfigurationRequest;
    type Output = ();
    type Error = ();
}

pub type Range = String;

pub type CommonPrefixList = Vec<CommonPrefix>;

#[derive(Debug, Default, Clone)]
pub struct GetBucketAnalyticsConfigurationOutput {
    pub analytics_configuration: Option<AnalyticsConfiguration>,
}
impl GetBucketAnalyticsConfigurationOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type CopySourceVersionId = String;

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
pub struct GetObjectTagging;
impl OperationShape for GetObjectTagging {
    const NAME: &'static str = "GetObjectTagging";
    type Input = GetObjectTaggingRequest;
    type Output = GetObjectTaggingOutput;
    type Error = ();
}

pub type CopySource = String;

pub type IntelligentTieringId = String;

pub type MultipartUploadList = Vec<MultipartUpload>;

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

#[derive(Debug, Default, Clone)]
pub struct PutBucketAclRequest {
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub content_md5: Option<ContentMd5>,
    pub grant_read: Option<GrantRead>,
    pub bucket: Option<BucketName>,
    pub acl: Option<BucketCannedAcl>,
    pub access_control_policy: Option<AccessControlPolicy>,
    pub expected_bucket_owner: Option<AccountId>,
    pub grant_full_control: Option<GrantFullControl>,
    pub grant_read_acp: Option<GrantReadAcp>,
    pub grant_write: Option<GrantWrite>,
    pub grant_write_acp: Option<GrantWriteAcp>,
}
impl PutBucketAclRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListObjectVersionsOutput {
    pub delimiter: Option<Delimiter>,
    pub prefix: Option<Prefix>,
    pub version_id_marker: Option<VersionIdMarker>,
    pub next_key_marker: Option<NextKeyMarker>,
    pub encoding_type: Option<EncodingType>,
    pub name: Option<BucketName>,
    pub is_truncated: Option<IsTruncated>,
    pub max_keys: Option<MaxKeys>,
    pub delete_markers: Option<DeleteMarkers>,
    pub key_marker: Option<KeyMarker>,
    pub next_version_id_marker: Option<NextVersionIdMarker>,
    pub versions: Option<ObjectVersionList>,
    pub common_prefixes: Option<CommonPrefixList>,
}
impl ListObjectVersionsOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Clone)]
pub enum SelectObjectContentEventStream {
    Stats(StatsEvent),
    Records(RecordsEvent),
    Cont(ContinuationEvent),
    End(EndEvent),
    Progress(ProgressEvent),
}

#[derive(Debug, Default, Clone)]
pub struct SelectObjectContentRequest {
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub expected_bucket_owner: Option<AccountId>,
    pub key: Option<ObjectKey>,
    pub input_serialization: Option<InputSerialization>,
    pub bucket: Option<BucketName>,
    pub expression: Option<Expression>,
    pub output_serialization: Option<OutputSerialization>,
    pub request_progress: Option<RequestProgress>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub sse_customer_key: Option<SseCustomerKey>,
    pub scan_range: Option<ScanRange>,
    pub expression_type: Option<ExpressionType>,
}
impl SelectObjectContentRequest {
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

pub type InventoryOptionalFields = Vec<InventoryOptionalField>;

#[derive(Debug, Default, Clone)]
pub struct CopyObject;
impl OperationShape for CopyObject {
    const NAME: &'static str = "CopyObject";
    type Input = CopyObjectRequest;
    type Output = CopyObjectOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct Error {
    pub key: Option<ObjectKey>,
    pub message: Option<Message>,
    pub version_id: Option<ObjectVersionId>,
    pub code: Option<Code>,
}
impl Error {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct PutBucketAccelerateConfigurationRequest {
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
    pub accelerate_configuration: Option<AccelerateConfiguration>,
}
impl PutBucketAccelerateConfigurationRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct UploadPartCopyOutput {
    pub server_side_encryption: Option<ServerSideEncryption>,
    pub request_charged: Option<RequestCharged>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub copy_part_result: Option<CopyPartResult>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub ssekms_key_id: Option<SsekmsKeyId>,
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    pub copy_source_version_id: Option<CopySourceVersionId>,
}
impl UploadPartCopyOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct PutBucketInventoryConfiguration;
impl OperationShape for PutBucketInventoryConfiguration {
    const NAME: &'static str = "PutBucketInventoryConfiguration";
    type Input = PutBucketInventoryConfigurationRequest;
    type Output = ();
    type Error = ();
}

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

#[derive(Debug, Default, Clone)]
pub struct PolicyStatus {
    pub is_public: Option<IsPublic>,
}
impl PolicyStatus {
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

pub type Setting = bool;

#[derive(Debug, Default, Clone)]
pub struct PutBucketReplicationRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub replication_configuration: Option<ReplicationConfiguration>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub token: Option<ObjectLockToken>,
    pub content_md5: Option<ContentMd5>,
    pub bucket: Option<BucketName>,
}
impl PutBucketReplicationRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

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

pub type ObjectLockToken = String;

pub type ObjectList = Vec<Object>;

#[derive(Debug, Default, Clone)]
pub struct GetBucketWebsiteOutput {
    pub error_document: Option<ErrorDocument>,
    pub index_document: Option<IndexDocument>,
    pub routing_rules: Option<RoutingRules>,
    pub redirect_all_requests_to: Option<RedirectAllRequestsTo>,
}
impl GetBucketWebsiteOutput {
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
pub struct SelectObjectContent;
impl OperationShape for SelectObjectContent {
    const NAME: &'static str = "SelectObjectContent";
    type Input = SelectObjectContentRequest;
    type Output = SelectObjectContentOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct PutObjectLegalHoldOutput {
    pub request_charged: Option<RequestCharged>,
}
impl PutObjectLegalHoldOutput {
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
pub struct GetBucketInventoryConfigurationOutput {
    pub inventory_configuration: Option<InventoryConfiguration>,
}
impl GetBucketInventoryConfigurationOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct AnalyticsS3BucketDestination {
    pub bucket_account_id: Option<AccountId>,
    pub format: Option<AnalyticsS3ExportFileFormat>,
    pub bucket: Option<BucketName>,
    pub prefix: Option<Prefix>,
}
impl AnalyticsS3BucketDestination {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketPolicyRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}
impl GetBucketPolicyRequest {
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

pub type GrantWriteAcp = String;

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
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

pub type OwnershipControlsRules = Vec<OwnershipControlsRule>;

#[derive(Debug, Default, Clone)]
pub struct PutBucketRequestPayment;
impl OperationShape for PutBucketRequestPayment {
    const NAME: &'static str = "PutBucketRequestPayment";
    type Input = PutBucketRequestPaymentRequest;
    type Output = ();
    type Error = ();
}

pub type Id = String;

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum FileHeaderInfo {
    NONE,
    USE,
    IGNORE,
}
impl AsRef<str> for FileHeaderInfo {
    fn as_ref(&self) -> &str {
        match self {
            Self::NONE => "NONE",
            Self::USE => "USE",
            Self::IGNORE => "IGNORE",
        }
    }
}
impl TryFrom<&str> for FileHeaderInfo {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "NONE" => Ok(Self::NONE),
            "USE" => Ok(Self::USE),
            "IGNORE" => Ok(Self::IGNORE),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct SelectObjectContentOutput {
    pub payload: Option<SelectObjectContentEventStream>,
}
impl SelectObjectContentOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type ResponseContentType = String;

pub type WebsiteRedirectLocation = String;

#[derive(Debug, Default, Clone)]
pub struct ListObjects;
impl OperationShape for ListObjects {
    const NAME: &'static str = "ListObjects";
    type Input = ListObjectsRequest;
    type Output = ListObjectsOutput;
    type Error = ();
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
pub struct UploadPartCopyRequest {
    pub copy_source: Option<CopySource>,
    pub upload_id: Option<MultipartUploadId>,
    pub sse_customer_key: Option<SseCustomerKey>,
    pub expected_source_bucket_owner: Option<AccountId>,
    pub key: Option<ObjectKey>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub copy_source_range: Option<CopySourceRange>,
    pub expected_bucket_owner: Option<AccountId>,
    pub part_number: Option<PartNumber>,
    pub copy_source_if_match: Option<CopySourceIfMatch>,
    pub copy_source_if_none_match: Option<CopySourceIfNoneMatch>,
    pub copy_source_sse_customer_key: Option<CopySourceSseCustomerKey>,
    pub copy_source_sse_customer_algorithm: Option<CopySourceSseCustomerAlgorithm>,
    pub copy_source_sse_customer_key_md5: Option<CopySourceSseCustomerKeyMd5>,
    pub request_payer: Option<RequestPayer>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub copy_source_if_modified_since: Option<CopySourceIfModifiedSince>,
    pub copy_source_if_unmodified_since: Option<CopySourceIfUnmodifiedSince>,
    pub bucket: Option<BucketName>,
}
impl UploadPartCopyRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct HeadObjectRequest {
    pub bucket: Option<BucketName>,
    pub if_modified_since: Option<IfModifiedSince>,
    pub if_none_match: Option<IfNoneMatch>,
    pub request_payer: Option<RequestPayer>,
    pub sse_customer_key: Option<SseCustomerKey>,
    pub part_number: Option<PartNumber>,
    pub range: Option<Range>,
    pub if_match: Option<IfMatch>,
    pub key: Option<ObjectKey>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub version_id: Option<ObjectVersionId>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub expected_bucket_owner: Option<AccountId>,
    pub if_unmodified_since: Option<IfUnmodifiedSince>,
    pub checksum_mode: Option<ChecksumMode>,
}
impl HeadObjectRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ObjectVersion {
    pub size: Option<Size>,
    pub version_id: Option<ObjectVersionId>,
    pub e_tag: Option<ETag>,
    pub is_latest: Option<IsLatest>,
    pub checksum_algorithm: Option<ChecksumAlgorithmList>,
    pub owner: Option<Owner>,
    pub key: Option<ObjectKey>,
    pub storage_class: Option<ObjectVersionStorageClass>,
    pub last_modified: Option<LastModified>,
}
impl ObjectVersion {
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

pub type GetObjectResponseStatusCode = i32;

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
pub struct ObjectAlreadyInActiveTierError {}
impl ObjectAlreadyInActiveTierError {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type ObjectKey = String;

#[derive(Debug, Default, Clone)]
pub struct RestoreObject;
impl OperationShape for RestoreObject {
    const NAME: &'static str = "RestoreObject";
    type Input = RestoreObjectRequest;
    type Output = RestoreObjectOutput;
    type Error = ();
}

pub type CorsRules = Vec<CorsRule>;

#[derive(Debug, Default, Clone)]
pub struct CompleteMultipartUploadRequest {
    pub sse_customer_key: Option<SseCustomerKey>,
    pub checksum_sha256: Option<ChecksumSha256>,
    pub checksum_sha1: Option<ChecksumSha1>,
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
    pub key: Option<ObjectKey>,
    pub upload_id: Option<MultipartUploadId>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub multipart_upload: Option<CompletedMultipartUpload>,
    pub bucket: Option<BucketName>,
    pub checksum_crc32: Option<ChecksumCrc32>,
    pub expected_bucket_owner: Option<AccountId>,
    pub request_payer: Option<RequestPayer>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
}
impl CompleteMultipartUploadRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
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

pub type TieringList = Vec<Tiering>;

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
pub struct GetObjectLegalHold;
impl OperationShape for GetObjectLegalHold {
    const NAME: &'static str = "GetObjectLegalHold";
    type Input = GetObjectLegalHoldRequest;
    type Output = GetObjectLegalHoldOutput;
    type Error = ();
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum ChecksumAlgorithm {
    SHA1,
    SHA256,
    CRC32C,
    CRC32,
}
impl AsRef<str> for ChecksumAlgorithm {
    fn as_ref(&self) -> &str {
        match self {
            Self::SHA1 => "SHA1",
            Self::SHA256 => "SHA256",
            Self::CRC32C => "CRC32C",
            Self::CRC32 => "CRC32",
        }
    }
}
impl TryFrom<&str> for ChecksumAlgorithm {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "SHA1" => Ok(Self::SHA1),
            "SHA256" => Ok(Self::SHA256),
            "CRC32C" => Ok(Self::CRC32C),
            "CRC32" => Ok(Self::CRC32),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListBucketMetricsConfigurationsRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
    pub continuation_token: Option<Token>,
}
impl ListBucketMetricsConfigurationsRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type DeleteMarker = bool;

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketEncryption;
impl OperationShape for DeleteBucketEncryption {
    const NAME: &'static str = "DeleteBucketEncryption";
    type Input = DeleteBucketEncryptionRequest;
    type Output = ();
    type Error = ();
}

pub type IsTruncated = bool;

pub type LifecycleRules = Vec<LifecycleRule>;

pub type Description = String;

#[derive(Debug, Default, Clone)]
pub struct AbortMultipartUpload;
impl OperationShape for AbortMultipartUpload {
    const NAME: &'static str = "AbortMultipartUpload";
    type Input = AbortMultipartUploadRequest;
    type Output = AbortMultipartUploadOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct InventorySchedule {
    pub frequency: Option<InventoryFrequency>,
}
impl InventorySchedule {
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

pub type ServerSideEncryptionRules = Vec<ServerSideEncryptionRule>;

#[derive(Debug, Default, Clone)]
pub struct CorsRule {
    pub allowed_methods: Option<AllowedMethods>,
    pub allowed_origins: Option<AllowedOrigins>,
    pub expose_headers: Option<ExposeHeaders>,
    pub max_age_seconds: Option<MaxAgeSeconds>,
    pub id: Option<Id>,
    pub allowed_headers: Option<AllowedHeaders>,
}
impl CorsRule {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type ObjectAttributesList = Vec<ObjectAttributes>;

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
pub struct ListMultipartUploadsOutput {
    pub common_prefixes: Option<CommonPrefixList>,
    pub uploads: Option<MultipartUploadList>,
    pub next_upload_id_marker: Option<NextUploadIdMarker>,
    pub is_truncated: Option<IsTruncated>,
    pub key_marker: Option<KeyMarker>,
    pub max_uploads: Option<MaxUploads>,
    pub encoding_type: Option<EncodingType>,
    pub delimiter: Option<Delimiter>,
    pub prefix: Option<Prefix>,
    pub next_key_marker: Option<NextKeyMarker>,
    pub bucket: Option<BucketName>,
    pub upload_id_marker: Option<UploadIdMarker>,
}
impl ListMultipartUploadsOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type IfModifiedSince = String;

#[derive(Debug, Default, Clone)]
pub struct GetObjectLockConfiguration;
impl OperationShape for GetObjectLockConfiguration {
    const NAME: &'static str = "GetObjectLockConfiguration";
    type Input = GetObjectLockConfigurationRequest;
    type Output = GetObjectLockConfigurationOutput;
    type Error = ();
}

pub type InventoryId = String;

#[derive(Debug, Default, Clone)]
pub struct ListParts;
impl OperationShape for ListParts {
    const NAME: &'static str = "ListParts";
    type Input = ListPartsRequest;
    type Output = ListPartsOutput;
    type Error = ();
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
pub struct GetBucketReplication;
impl OperationShape for GetBucketReplication {
    const NAME: &'static str = "GetBucketReplication";
    type Input = GetBucketReplicationRequest;
    type Output = GetBucketReplicationOutput;
    type Error = ();
}

pub type MaxParts = i32;

#[derive(Debug, Default, Clone)]
pub struct GetObjectAttributesOutput {
    pub request_charged: Option<RequestCharged>,
    pub checksum: Option<Checksum>,
    pub storage_class: Option<StorageClass>,
    pub delete_marker: Option<DeleteMarker>,
    pub object_size: Option<ObjectSize>,
    pub e_tag: Option<ETag>,
    pub last_modified: Option<LastModified>,
    pub version_id: Option<ObjectVersionId>,
    pub object_parts: Option<GetObjectAttributesParts>,
}
impl GetObjectAttributesOutput {
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

pub type KeyMarker = String;

pub type BypassGovernanceRetention = bool;

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

#[derive(Debug, Default, Clone)]
pub struct CreateBucketRequest {
    pub create_bucket_configuration: Option<CreateBucketConfiguration>,
    pub acl: Option<BucketCannedAcl>,
    pub object_lock_enabled_for_bucket: Option<ObjectLockEnabledForBucket>,
    pub bucket: Option<BucketName>,
    pub grant_write: Option<GrantWrite>,
    pub grant_read_acp: Option<GrantReadAcp>,
    pub grant_read: Option<GrantRead>,
    pub grant_write_acp: Option<GrantWriteAcp>,
    pub object_ownership: Option<ObjectOwnership>,
    pub grant_full_control: Option<GrantFullControl>,
}
impl CreateBucketRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type ObjectIdentifierList = Vec<ObjectIdentifier>;

#[derive(Debug, Default, Clone)]
pub struct PutObjectRetention;
impl OperationShape for PutObjectRetention {
    const NAME: &'static str = "PutObjectRetention";
    type Input = PutObjectRetentionRequest;
    type Output = PutObjectRetentionOutput;
    type Error = ();
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
pub struct DeleteObjectTagging;
impl OperationShape for DeleteObjectTagging {
    const NAME: &'static str = "DeleteObjectTagging";
    type Input = DeleteObjectTaggingRequest;
    type Output = DeleteObjectTaggingOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct AmazonS3 {
    pub put_bucket_acl: Option<OpService<PutBucketAcl>>,
    pub get_bucket_intelligent_tiering_configuration:
        Option<OpService<GetBucketIntelligentTieringConfiguration>>,
    pub delete_object: Option<OpService<DeleteObject>>,
    pub get_bucket_accelerate_configuration: Option<OpService<GetBucketAccelerateConfiguration>>,
    pub delete_bucket_policy: Option<OpService<DeleteBucketPolicy>>,
    pub list_parts: Option<OpService<ListParts>>,
    pub get_object_torrent: Option<OpService<GetObjectTorrent>>,
    pub get_bucket_analytics_configuration: Option<OpService<GetBucketAnalyticsConfiguration>>,
    pub create_bucket: Option<OpService<CreateBucket>>,
    pub get_object: Option<OpService<GetObject>>,
    pub put_bucket_encryption: Option<OpService<PutBucketEncryption>>,
    pub put_object_lock_configuration: Option<OpService<PutObjectLockConfiguration>>,
    pub delete_public_access_block: Option<OpService<DeletePublicAccessBlock>>,
    pub get_bucket_lifecycle_configuration: Option<OpService<GetBucketLifecycleConfiguration>>,
    pub put_bucket_lifecycle_configuration: Option<OpService<PutBucketLifecycleConfiguration>>,
    pub get_bucket_replication: Option<OpService<GetBucketReplication>>,
    pub get_object_acl: Option<OpService<GetObjectAcl>>,
    pub put_public_access_block: Option<OpService<PutPublicAccessBlock>>,
    pub put_bucket_logging: Option<OpService<PutBucketLogging>>,
    pub select_object_content: Option<OpService<SelectObjectContent>>,
    pub put_object: Option<OpService<PutObject>>,
    pub put_bucket_website: Option<OpService<PutBucketWebsite>>,
    pub restore_object: Option<OpService<RestoreObject>>,
    pub get_bucket_metrics_configuration: Option<OpService<GetBucketMetricsConfiguration>>,
    pub get_bucket_policy: Option<OpService<GetBucketPolicy>>,
    pub head_object: Option<OpService<HeadObject>>,
    pub upload_part_copy: Option<OpService<UploadPartCopy>>,
    pub copy_object: Option<OpService<CopyObject>>,
    pub put_bucket_analytics_configuration: Option<OpService<PutBucketAnalyticsConfiguration>>,
    pub delete_bucket_metrics_configuration: Option<OpService<DeleteBucketMetricsConfiguration>>,
    pub put_bucket_metrics_configuration: Option<OpService<PutBucketMetricsConfiguration>>,
    pub put_bucket_tagging: Option<OpService<PutBucketTagging>>,
    pub put_object_tagging: Option<OpService<PutObjectTagging>>,
    pub put_bucket_notification_configuration:
        Option<OpService<PutBucketNotificationConfiguration>>,
    pub delete_bucket_intelligent_tiering_configuration:
        Option<OpService<DeleteBucketIntelligentTieringConfiguration>>,
    pub put_bucket_ownership_controls: Option<OpService<PutBucketOwnershipControls>>,
    pub list_objects: Option<OpService<ListObjects>>,
    pub put_bucket_policy: Option<OpService<PutBucketPolicy>>,
    pub get_bucket_ownership_controls: Option<OpService<GetBucketOwnershipControls>>,
    pub delete_object_tagging: Option<OpService<DeleteObjectTagging>>,
    pub create_multipart_upload: Option<OpService<CreateMultipartUpload>>,
    pub complete_multipart_upload: Option<OpService<CompleteMultipartUpload>>,
    pub delete_bucket_replication: Option<OpService<DeleteBucketReplication>>,
    pub delete_bucket_tagging: Option<OpService<DeleteBucketTagging>>,
    pub get_bucket_inventory_configuration: Option<OpService<GetBucketInventoryConfiguration>>,
    pub get_object_attributes: Option<OpService<GetObjectAttributes>>,
    pub delete_bucket_website: Option<OpService<DeleteBucketWebsite>>,
    pub get_bucket_location: Option<OpService<GetBucketLocation>>,
    pub list_multipart_uploads: Option<OpService<ListMultipartUploads>>,
    pub get_bucket_policy_status: Option<OpService<GetBucketPolicyStatus>>,
    pub get_object_lock_configuration: Option<OpService<GetObjectLockConfiguration>>,
    pub list_bucket_metrics_configurations: Option<OpService<ListBucketMetricsConfigurations>>,
    pub get_public_access_block: Option<OpService<GetPublicAccessBlock>>,
    pub list_object_versions: Option<OpService<ListObjectVersions>>,
    pub get_bucket_encryption: Option<OpService<GetBucketEncryption>>,
    pub abort_multipart_upload: Option<OpService<AbortMultipartUpload>>,
    pub delete_bucket_ownership_controls: Option<OpService<DeleteBucketOwnershipControls>>,
    pub get_bucket_acl: Option<OpService<GetBucketAcl>>,
    pub list_bucket_analytics_configurations: Option<OpService<ListBucketAnalyticsConfigurations>>,
    pub get_bucket_website: Option<OpService<GetBucketWebsite>>,
    pub list_buckets: Option<OpService<ListBuckets>>,
    pub put_bucket_cors: Option<OpService<PutBucketCors>>,
    pub put_bucket_replication: Option<OpService<PutBucketReplication>>,
    pub write_get_object_response: Option<OpService<WriteGetObjectResponse>>,
    pub put_bucket_versioning: Option<OpService<PutBucketVersioning>>,
    pub delete_objects: Option<OpService<DeleteObjects>>,
    pub delete_bucket_cors: Option<OpService<DeleteBucketCors>>,
    pub put_bucket_intelligent_tiering_configuration:
        Option<OpService<PutBucketIntelligentTieringConfiguration>>,
    pub put_object_legal_hold: Option<OpService<PutObjectLegalHold>>,
    pub list_bucket_intelligent_tiering_configurations:
        Option<OpService<ListBucketIntelligentTieringConfigurations>>,
    pub list_bucket_inventory_configurations: Option<OpService<ListBucketInventoryConfigurations>>,
    pub get_bucket_cors: Option<OpService<GetBucketCors>>,
    pub head_bucket: Option<OpService<HeadBucket>>,
    pub put_bucket_accelerate_configuration: Option<OpService<PutBucketAccelerateConfiguration>>,
    pub put_bucket_inventory_configuration: Option<OpService<PutBucketInventoryConfiguration>>,
    pub get_bucket_notification_configuration:
        Option<OpService<GetBucketNotificationConfiguration>>,
    pub delete_bucket_inventory_configuration:
        Option<OpService<DeleteBucketInventoryConfiguration>>,
    pub delete_bucket: Option<OpService<DeleteBucket>>,
    pub get_bucket_tagging: Option<OpService<GetBucketTagging>>,
    pub get_bucket_versioning: Option<OpService<GetBucketVersioning>>,
    pub delete_bucket_analytics_configuration:
        Option<OpService<DeleteBucketAnalyticsConfiguration>>,
    pub get_object_tagging: Option<OpService<GetObjectTagging>>,
    pub list_objects_v2: Option<OpService<ListObjectsV2>>,
    pub put_bucket_request_payment: Option<OpService<PutBucketRequestPayment>>,
    pub get_object_retention: Option<OpService<GetObjectRetention>>,
    pub put_object_retention: Option<OpService<PutObjectRetention>>,
    pub put_object_acl: Option<OpService<PutObjectAcl>>,
    pub get_object_legal_hold: Option<OpService<GetObjectLegalHold>>,
    pub upload_part: Option<OpService<UploadPart>>,
    pub get_bucket_logging: Option<OpService<GetBucketLogging>>,
    pub get_bucket_request_payment: Option<OpService<GetBucketRequestPayment>>,
    pub delete_bucket_lifecycle: Option<OpService<DeleteBucketLifecycle>>,
    pub delete_bucket_encryption: Option<OpService<DeleteBucketEncryption>>,
}

pub type Comments = String;

#[derive(Debug, Default, Clone)]
pub struct GetBucketInventoryConfiguration;
impl OperationShape for GetBucketInventoryConfiguration {
    const NAME: &'static str = "GetBucketInventoryConfiguration";
    type Input = GetBucketInventoryConfigurationRequest;
    type Output = GetBucketInventoryConfigurationOutput;
    type Error = ();
}

pub type TagCount = i32;

#[derive(Debug, Default, Clone)]
pub struct DeleteMarkerReplication {
    pub status: Option<DeleteMarkerReplicationStatus>,
}
impl DeleteMarkerReplication {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
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
pub struct UploadPartOutput {
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub server_side_encryption: Option<ServerSideEncryption>,
    pub checksum_crc32: Option<ChecksumCrc32>,
    pub checksum_sha1: Option<ChecksumSha1>,
    pub request_charged: Option<RequestCharged>,
    pub ssekms_key_id: Option<SsekmsKeyId>,
    pub e_tag: Option<ETag>,
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
    pub checksum_sha256: Option<ChecksumSha256>,
}
impl UploadPartOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

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
pub struct ServerSideEncryptionByDefault {
    pub kms_master_key_id: Option<SsekmsKeyId>,
    pub sse_algorithm: Option<ServerSideEncryption>,
}
impl ServerSideEncryptionByDefault {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct HeadObjectOutput {
    pub replication_status: Option<ReplicationStatus>,
    pub delete_marker: Option<DeleteMarker>,
    pub server_side_encryption: Option<ServerSideEncryption>,
    pub metadata: Option<Metadata>,
    pub checksum_crc32: Option<ChecksumCrc32>,
    pub content_encoding: Option<ContentEncoding>,
    pub version_id: Option<ObjectVersionId>,
    pub last_modified: Option<LastModified>,
    pub restore: Option<Restore>,
    pub content_length: Option<ContentLength>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub content_type: Option<ContentType>,
    pub checksum_sha256: Option<ChecksumSha256>,
    pub object_lock_legal_hold_status: Option<ObjectLockLegalHoldStatus>,
    pub archive_status: Option<ArchiveStatus>,
    pub ssekms_key_id: Option<SsekmsKeyId>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub bucket_key_enabled: Option<BucketKeyEnabled>,
    pub checksum_sha1: Option<ChecksumSha1>,
    pub missing_meta: Option<MissingMeta>,
    pub object_lock_mode: Option<ObjectLockMode>,
    pub object_lock_retain_until_date: Option<ObjectLockRetainUntilDate>,
    pub cache_control: Option<CacheControl>,
    pub content_disposition: Option<ContentDisposition>,
    pub storage_class: Option<StorageClass>,
    pub website_redirect_location: Option<WebsiteRedirectLocation>,
    pub request_charged: Option<RequestCharged>,
    pub expires: Option<Expires>,
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
    pub parts_count: Option<PartsCount>,
    pub expiration: Option<Expiration>,
    pub content_language: Option<ContentLanguage>,
    pub e_tag: Option<ETag>,
    pub accept_ranges: Option<AcceptRanges>,
}
impl HeadObjectOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct Condition {
    pub key_prefix_equals: Option<KeyPrefixEquals>,
    pub http_error_code_returned_equals: Option<HttpErrorCodeReturnedEquals>,
}
impl Condition {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
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

#[derive(Debug, Default, Clone)]
pub struct PutBucketNotificationConfigurationRequest {
    pub skip_destination_validation: Option<SkipValidation>,
    pub notification_configuration: Option<NotificationConfiguration>,
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}
impl PutBucketNotificationConfigurationRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type Metadata = HashMap<MetadataKey, MetadataValue>;

pub type Uri = String;

#[derive(Debug, Default, Clone)]
pub struct AbortMultipartUploadRequest {
    pub upload_id: Option<MultipartUploadId>,
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
    pub key: Option<ObjectKey>,
    pub request_payer: Option<RequestPayer>,
}
impl AbortMultipartUploadRequest {
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

pub type BucketName = String;

#[derive(Debug, Default, Clone)]
pub struct CreateBucket;
impl OperationShape for CreateBucket {
    const NAME: &'static str = "CreateBucket";
    type Input = CreateBucketRequest;
    type Output = CreateBucketOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct ListObjectVersions;
impl OperationShape for ListObjectVersions {
    const NAME: &'static str = "ListObjectVersions";
    type Input = ListObjectVersionsRequest;
    type Output = ListObjectVersionsOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketCors;
impl OperationShape for DeleteBucketCors {
    const NAME: &'static str = "DeleteBucketCors";
    type Input = DeleteBucketCorsRequest;
    type Output = ();
    type Error = ();
}

pub type SseCustomerKey = String;

#[derive(Debug, Default, Clone)]
pub struct PutObjectLegalHold;
impl OperationShape for PutObjectLegalHold {
    const NAME: &'static str = "PutObjectLegalHold";
    type Input = PutObjectLegalHoldRequest;
    type Output = PutObjectLegalHoldOutput;
    type Error = ();
}

pub type IfMatch = String;

pub type AccessPointArn = String;

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
pub struct ReplicationTimeValue {
    pub minutes: Option<Minutes>,
}
impl ReplicationTimeValue {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct PublicAccessBlockConfiguration {
    pub restrict_public_buckets: Option<Setting>,
    pub ignore_public_acls: Option<Setting>,
    pub block_public_policy: Option<Setting>,
    pub block_public_acls: Option<Setting>,
}
impl PublicAccessBlockConfiguration {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct CopyObjectResult {
    pub last_modified: Option<LastModified>,
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
    pub checksum_sha256: Option<ChecksumSha256>,
    pub checksum_sha1: Option<ChecksumSha1>,
    pub checksum_crc32: Option<ChecksumCrc32>,
    pub e_tag: Option<ETag>,
}
impl CopyObjectResult {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct Stats {
    pub bytes_processed: Option<BytesProcessed>,
    pub bytes_returned: Option<BytesReturned>,
    pub bytes_scanned: Option<BytesScanned>,
}
impl Stats {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
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
pub struct GetObjectTaggingOutput {
    pub version_id: Option<ObjectVersionId>,
    pub tag_set: Option<TagSet>,
}
impl GetObjectTaggingOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type TagSet = Vec<Tag>;

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
pub struct GetBucketMetricsConfiguration;
impl OperationShape for GetBucketMetricsConfiguration {
    const NAME: &'static str = "GetBucketMetricsConfiguration";
    type Input = GetBucketMetricsConfigurationRequest;
    type Output = GetBucketMetricsConfigurationOutput;
    type Error = ();
}

pub type IfUnmodifiedSince = String;

pub type LambdaFunctionConfigurationList = Vec<LambdaFunctionConfiguration>;

pub type MaxKeys = i32;

pub type ContentLanguage = String;

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

pub type VersionCount = i32;

#[derive(Debug, Default, Clone)]
pub struct DeleteObjectOutput {
    pub version_id: Option<ObjectVersionId>,
    pub request_charged: Option<RequestCharged>,
    pub delete_marker: Option<DeleteMarker>,
}
impl DeleteObjectOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct Redirect {
    pub host_name: Option<HostName>,
    pub replace_key_with: Option<ReplaceKeyWith>,
    pub replace_key_prefix_with: Option<ReplaceKeyPrefixWith>,
    pub protocol: Option<Protocol>,
    pub http_redirect_code: Option<HttpRedirectCode>,
}
impl Redirect {
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

pub type ObjectSizeGreaterThanBytes = i64;

#[derive(Debug, Default, Clone)]
pub struct ReplicationTime {
    pub time: Option<ReplicationTimeValue>,
    pub status: Option<ReplicationTimeStatus>,
}
impl ReplicationTime {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type ResponseContentEncoding = String;

#[derive(Debug, Default, Clone)]
pub struct HeadBucket;
impl OperationShape for HeadBucket {
    const NAME: &'static str = "HeadBucket";
    type Input = HeadBucketRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketLifecycleConfigurationRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}
impl GetBucketLifecycleConfigurationRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type InventoryConfigurationList = Vec<InventoryConfiguration>;

#[derive(Debug, Default, Clone)]
pub struct NoSuchUpload {}
impl NoSuchUpload {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum CompressionType {
    GZIP,
    NONE,
    BZIP2,
}
impl AsRef<str> for CompressionType {
    fn as_ref(&self) -> &str {
        match self {
            Self::GZIP => "GZIP",
            Self::NONE => "NONE",
            Self::BZIP2 => "BZIP2",
        }
    }
}
impl TryFrom<&str> for CompressionType {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "GZIP" => Ok(Self::GZIP),
            "NONE" => Ok(Self::NONE),
            "BZIP2" => Ok(Self::BZIP2),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetObjectAclOutput {
    pub owner: Option<Owner>,
    pub request_charged: Option<RequestCharged>,
    pub grants: Option<Grants>,
}
impl GetObjectAclOutput {
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

pub type CopySourceSseCustomerKeyMd5 = String;

#[derive(Debug, Default, Clone)]
pub struct DeletePublicAccessBlock;
impl OperationShape for DeletePublicAccessBlock {
    const NAME: &'static str = "DeletePublicAccessBlock";
    type Input = DeletePublicAccessBlockRequest;
    type Output = ();
    type Error = ();
}

pub type Priority = i32;

#[derive(Debug, Default, Clone)]
pub struct ReplicationConfiguration {
    pub role: Option<Role>,
    pub rules: Option<ReplicationRules>,
}
impl ReplicationConfiguration {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type ReplicationRules = Vec<ReplicationRule>;

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketAnalyticsConfiguration;
impl OperationShape for DeleteBucketAnalyticsConfiguration {
    const NAME: &'static str = "DeleteBucketAnalyticsConfiguration";
    type Input = DeleteBucketAnalyticsConfigurationRequest;
    type Output = ();
    type Error = ();
}

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

pub type Grants = Vec<Grant>;

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

pub type AllowedOrigins = Vec<AllowedOrigin>;

pub type NextUploadIdMarker = String;

pub type AccountId = String;

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
pub struct IndexDocument {
    pub suffix: Option<Suffix>,
}
impl IndexDocument {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketTagging;
impl OperationShape for DeleteBucketTagging {
    const NAME: &'static str = "DeleteBucketTagging";
    type Input = DeleteBucketTaggingRequest;
    type Output = ();
    type Error = ();
}

pub type AbortDate = String;

#[derive(Debug, Default, Clone)]
pub struct BucketAlreadyExists {}
impl BucketAlreadyExists {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
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
pub enum Permission {
    WRITE_ACP,
    WRITE,
    READ_ACP,
    FULL_CONTROL,
    READ,
}
impl AsRef<str> for Permission {
    fn as_ref(&self) -> &str {
        match self {
            Self::WRITE_ACP => "WRITE_ACP",
            Self::WRITE => "WRITE",
            Self::READ_ACP => "READ_ACP",
            Self::FULL_CONTROL => "FULL_CONTROL",
            Self::READ => "READ",
        }
    }
}
impl TryFrom<&str> for Permission {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "WRITE_ACP" => Ok(Self::WRITE_ACP),
            "WRITE" => Ok(Self::WRITE),
            "READ_ACP" => Ok(Self::READ_ACP),
            "FULL_CONTROL" => Ok(Self::FULL_CONTROL),
            "READ" => Ok(Self::READ),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

pub type LastModified = String;

pub type EventList = Vec<Event>;

pub type QueueArn = String;

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
pub struct DeleteBucketEncryptionRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}
impl DeleteBucketEncryptionRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type TargetPrefix = String;

pub type Restore = String;

pub type BytesReturned = i64;

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
pub struct GetBucketInventoryConfigurationRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub id: Option<InventoryId>,
    pub bucket: Option<BucketName>,
}
impl GetBucketInventoryConfigurationRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketNotificationConfiguration;
impl OperationShape for GetBucketNotificationConfiguration {
    const NAME: &'static str = "GetBucketNotificationConfiguration";
    type Input = GetBucketNotificationConfigurationRequest;
    type Output = NotificationConfiguration;
    type Error = ();
}

pub type ChecksumSha1 = String;

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

pub type CreationDate = String;

#[derive(Debug, Default, Clone)]
pub struct ExistingObjectReplication {
    pub status: Option<ExistingObjectReplicationStatus>,
}
impl ExistingObjectReplication {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type CopySourceRange = String;

#[derive(Debug, Default, Clone)]
pub struct GetObjectAclRequest {
    pub request_payer: Option<RequestPayer>,
    pub key: Option<ObjectKey>,
    pub version_id: Option<ObjectVersionId>,
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}
impl GetObjectAclRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type LambdaFunctionArn = String;

#[derive(Debug, Default, Clone)]
pub struct Tag {
    pub value: Option<Value>,
    pub key: Option<ObjectKey>,
}
impl Tag {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct Checksum {
    pub checksum_sha1: Option<ChecksumSha1>,
    pub checksum_crc32: Option<ChecksumCrc32>,
    pub checksum_crc32_c: Option<ChecksumCrc32c>,
    pub checksum_sha256: Option<ChecksumSha256>,
}
impl Checksum {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type Expires = String;

#[derive(Debug, Default, Clone)]
pub struct RestoreObjectOutput {
    pub request_charged: Option<RequestCharged>,
    pub restore_output_path: Option<RestoreOutputPath>,
}
impl RestoreObjectOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ReplicationRuleAndOperator {
    pub tags: Option<TagSet>,
    pub prefix: Option<Prefix>,
}
impl ReplicationRuleAndOperator {
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
pub struct PutObjectTaggingOutput {
    pub version_id: Option<ObjectVersionId>,
}
impl PutObjectTaggingOutput {
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
pub struct ListObjectsRequest {
    pub max_keys: Option<MaxKeys>,
    pub expected_bucket_owner: Option<AccountId>,
    pub encoding_type: Option<EncodingType>,
    pub delimiter: Option<Delimiter>,
    pub bucket: Option<BucketName>,
    pub marker: Option<Marker>,
    pub prefix: Option<Prefix>,
    pub request_payer: Option<RequestPayer>,
}
impl ListObjectsRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct LambdaFunctionConfiguration {
    pub filter: Option<NotificationConfigurationFilter>,
    pub events: Option<EventList>,
    pub lambda_function_arn: Option<LambdaFunctionArn>,
    pub id: Option<NotificationId>,
}
impl LambdaFunctionConfiguration {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListObjectsV2Request {
    pub max_keys: Option<MaxKeys>,
    pub request_payer: Option<RequestPayer>,
    pub expected_bucket_owner: Option<AccountId>,
    pub start_after: Option<StartAfter>,
    pub prefix: Option<Prefix>,
    pub bucket: Option<BucketName>,
    pub delimiter: Option<Delimiter>,
    pub encoding_type: Option<EncodingType>,
    pub continuation_token: Option<Token>,
    pub fetch_owner: Option<FetchOwner>,
}
impl ListObjectsV2Request {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum ServerSideEncryption {
    aws_kms,
    AES256,
}
impl AsRef<str> for ServerSideEncryption {
    fn as_ref(&self) -> &str {
        match self {
            Self::aws_kms => "aws:kms",
            Self::AES256 => "AES256",
        }
    }
}
impl TryFrom<&str> for ServerSideEncryption {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "aws:kms" => Ok(Self::aws_kms),
            "AES256" => Ok(Self::AES256),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
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

pub type HostName = String;

pub type ReplicaKmsKeyId = String;

#[derive(Debug, Default, Clone)]
pub struct PutBucketEncryptionRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub content_md5: Option<ContentMd5>,
    pub server_side_encryption_configuration: Option<ServerSideEncryptionConfiguration>,
}
impl PutBucketEncryptionRequest {
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

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
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

pub type Size = i64;

#[derive(Debug, Default, Clone)]
pub struct AccessControlTranslation {
    pub owner: Option<OwnerOverride>,
}
impl AccessControlTranslation {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type AnalyticsConfigurationList = Vec<AnalyticsConfiguration>;

pub type PartNumber = i32;

#[derive(Debug, Default, Clone)]
pub struct Bucket {
    pub name: Option<BucketName>,
    pub creation_date: Option<CreationDate>,
}
impl Bucket {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
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

#[derive(Debug, Default, Clone)]
pub struct ListObjectsV2;
impl OperationShape for ListObjectsV2 {
    const NAME: &'static str = "ListObjectsV2";
    type Input = ListObjectsV2Request;
    type Output = ListObjectsV2Output;
    type Error = ();
}

pub type NextMarker = String;

pub type RequestRoute = String;

pub type ChecksumSha256 = String;

pub type TargetBucket = String;

pub type AcceptRanges = String;

#[derive(Debug, Default, Clone)]
pub struct GetBucketReplicationOutput {
    pub replication_configuration: Option<ReplicationConfiguration>,
}
impl GetBucketReplicationOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type ObjectVersionId = String;

#[derive(Debug, Default, Clone)]
pub struct DeletedObject {
    pub delete_marker: Option<DeleteMarker>,
    pub delete_marker_version_id: Option<DeleteMarkerVersionId>,
    pub key: Option<ObjectKey>,
    pub version_id: Option<ObjectVersionId>,
}
impl DeletedObject {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetObjectAttributesRequest {
    pub request_payer: Option<RequestPayer>,
    pub part_number_marker: Option<PartNumberMarker>,
    pub bucket: Option<BucketName>,
    pub sse_customer_algorithm: Option<SseCustomerAlgorithm>,
    pub max_parts: Option<MaxParts>,
    pub key: Option<ObjectKey>,
    pub sse_customer_key_md5: Option<SseCustomerKeyMd5>,
    pub expected_bucket_owner: Option<AccountId>,
    pub sse_customer_key: Option<SseCustomerKey>,
    pub object_attributes: Option<ObjectAttributesList>,
    pub version_id: Option<ObjectVersionId>,
}
impl GetObjectAttributesRequest {
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
pub struct GetBucketAnalyticsConfigurationRequest {
    pub id: Option<AnalyticsId>,
    pub expected_bucket_owner: Option<AccountId>,
    pub bucket: Option<BucketName>,
}
impl GetBucketAnalyticsConfigurationRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct DeleteBucketMetricsConfigurationRequest {
    pub expected_bucket_owner: Option<AccountId>,
    pub id: Option<MetricsId>,
    pub bucket: Option<BucketName>,
}
impl DeleteBucketMetricsConfigurationRequest {
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

#[derive(Debug, Default, Clone)]
pub struct PutBucketAnalyticsConfigurationRequest {
    pub id: Option<AnalyticsId>,
    pub analytics_configuration: Option<AnalyticsConfiguration>,
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}
impl PutBucketAnalyticsConfigurationRequest {
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

#[derive(Debug, Default, Clone)]
pub struct PutObjectAcl;
impl OperationShape for PutObjectAcl {
    const NAME: &'static str = "PutObjectAcl";
    type Input = PutObjectAclRequest;
    type Output = PutObjectAclOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct PutPublicAccessBlock;
impl OperationShape for PutPublicAccessBlock {
    const NAME: &'static str = "PutPublicAccessBlock";
    type Input = PutPublicAccessBlockRequest;
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

pub type QueueConfigurationList = Vec<QueueConfiguration>;

pub type Minutes = i32;

#[derive(Debug, Default, Clone)]
pub struct PutBucketRequestPaymentRequest {
    pub content_md5: Option<ContentMd5>,
    pub request_payment_configuration: Option<RequestPaymentConfiguration>,
    pub bucket: Option<BucketName>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub expected_bucket_owner: Option<AccountId>,
}
impl PutBucketRequestPaymentRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct MultipartUpload {
    pub upload_id: Option<MultipartUploadId>,
    pub initiator: Option<Initiator>,
    pub initiated: Option<Initiated>,
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub key: Option<ObjectKey>,
    pub owner: Option<Owner>,
    pub storage_class: Option<StorageClass>,
}
impl MultipartUpload {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type GrantReadAcp = String;

pub type NoncurrentVersionTransitionList = Vec<NoncurrentVersionTransition>;

pub type AllowedHeader = String;

pub type MaxAgeSeconds = i32;

pub type ObjectSizeLessThanBytes = i64;

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
pub struct GetObjectTorrent;
impl OperationShape for GetObjectTorrent {
    const NAME: &'static str = "GetObjectTorrent";
    type Input = GetObjectTorrentRequest;
    type Output = GetObjectTorrentOutput;
    type Error = ();
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
pub struct OutputLocation {
    pub s3: Option<S3Location>,
}
impl OutputLocation {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ParquetInput {}
impl ParquetInput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type PartsCount = i32;

#[derive(Debug, Default, Clone)]
pub struct RedirectAllRequestsTo {
    pub host_name: Option<HostName>,
    pub protocol: Option<Protocol>,
}
impl RedirectAllRequestsTo {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum ReplicationRuleStatus {
    Disabled,
    Enabled,
}
impl AsRef<str> for ReplicationRuleStatus {
    fn as_ref(&self) -> &str {
        match self {
            Self::Disabled => "Disabled",
            Self::Enabled => "Enabled",
        }
    }
}
impl TryFrom<&str> for ReplicationRuleStatus {
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
pub struct DeleteBucketIntelligentTieringConfigurationRequest {
    pub id: Option<IntelligentTieringId>,
    pub bucket: Option<BucketName>,
}
impl DeleteBucketIntelligentTieringConfigurationRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct QueueConfiguration {
    pub id: Option<NotificationId>,
    pub filter: Option<NotificationConfigurationFilter>,
    pub queue_arn: Option<QueueArn>,
    pub events: Option<EventList>,
}
impl QueueConfiguration {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

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
pub struct DeleteBucketTaggingRequest {
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
}
impl DeleteBucketTaggingRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketLifecycleConfiguration;
impl OperationShape for GetBucketLifecycleConfiguration {
    const NAME: &'static str = "GetBucketLifecycleConfiguration";
    type Input = GetBucketLifecycleConfigurationRequest;
    type Output = GetBucketLifecycleConfigurationOutput;
    type Error = ();
}

pub type Days = i32;

#[derive(Debug, Default, Clone)]
pub struct GetBucketPolicy;
impl OperationShape for GetBucketPolicy {
    const NAME: &'static str = "GetBucketPolicy";
    type Input = GetBucketPolicyRequest;
    type Output = GetBucketPolicyOutput;
    type Error = ();
}

pub type IsEnabled = bool;

#[derive(Debug, Default, Clone)]
pub struct JsonOutput {
    pub record_delimiter: Option<RecordDelimiter>,
}
impl JsonOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type QuoteEscapeCharacter = String;

#[derive(Debug, Default, Clone)]
pub struct RecordsEvent {
    pub payload: Option<Body>,
}
impl RecordsEvent {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Clone)]
pub enum ReplicationRuleFilter {
    And(ReplicationRuleAndOperator),
    Prefix(Prefix),
    Tag(Tag),
}

#[derive(Debug, Default, Clone)]
pub struct GetBucketAnalyticsConfiguration;
impl OperationShape for GetBucketAnalyticsConfiguration {
    const NAME: &'static str = "GetBucketAnalyticsConfiguration";
    type Input = GetBucketAnalyticsConfigurationRequest;
    type Output = GetBucketAnalyticsConfigurationOutput;
    type Error = ();
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum InventoryIncludedObjectVersions {
    Current,
    All,
}
impl AsRef<str> for InventoryIncludedObjectVersions {
    fn as_ref(&self) -> &str {
        match self {
            Self::Current => "Current",
            Self::All => "All",
        }
    }
}
impl TryFrom<&str> for InventoryIncludedObjectVersions {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "Current" => Ok(Self::Current),
            "All" => Ok(Self::All),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

pub type AbortRuleId = String;

#[derive(Debug, Default, Clone)]
pub struct GetBucketPolicyOutput {
    pub policy: Option<Policy>,
}
impl GetBucketPolicyOutput {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListBucketInventoryConfigurations;
impl OperationShape for ListBucketInventoryConfigurations {
    const NAME: &'static str = "ListBucketInventoryConfigurations";
    type Input = ListBucketInventoryConfigurationsRequest;
    type Output = ListBucketInventoryConfigurationsOutput;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct PutBucketNotificationConfiguration;
impl OperationShape for PutBucketNotificationConfiguration {
    const NAME: &'static str = "PutBucketNotificationConfiguration";
    type Input = PutBucketNotificationConfigurationRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum InventoryOptionalField {
    Size,
    LastModifiedDate,
    StorageClass,
    ObjectLockLegalHoldStatus,
    ETag,
    EncryptionStatus,
    BucketKeyStatus,
    IsMultipartUploaded,
    ObjectLockMode,
    ObjectLockRetainUntilDate,
    ChecksumAlgorithm,
    IntelligentTieringAccessTier,
    ReplicationStatus,
}
impl AsRef<str> for InventoryOptionalField {
    fn as_ref(&self) -> &str {
        match self {
            Self::Size => "Size",
            Self::LastModifiedDate => "LastModifiedDate",
            Self::StorageClass => "StorageClass",
            Self::ObjectLockLegalHoldStatus => "ObjectLockLegalHoldStatus",
            Self::ETag => "ETag",
            Self::EncryptionStatus => "EncryptionStatus",
            Self::BucketKeyStatus => "BucketKeyStatus",
            Self::IsMultipartUploaded => "IsMultipartUploaded",
            Self::ObjectLockMode => "ObjectLockMode",
            Self::ObjectLockRetainUntilDate => "ObjectLockRetainUntilDate",
            Self::ChecksumAlgorithm => "ChecksumAlgorithm",
            Self::IntelligentTieringAccessTier => "IntelligentTieringAccessTier",
            Self::ReplicationStatus => "ReplicationStatus",
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
            "ObjectLockLegalHoldStatus" => Ok(Self::ObjectLockLegalHoldStatus),
            "ETag" => Ok(Self::ETag),
            "EncryptionStatus" => Ok(Self::EncryptionStatus),
            "BucketKeyStatus" => Ok(Self::BucketKeyStatus),
            "IsMultipartUploaded" => Ok(Self::IsMultipartUploaded),
            "ObjectLockMode" => Ok(Self::ObjectLockMode),
            "ObjectLockRetainUntilDate" => Ok(Self::ObjectLockRetainUntilDate),
            "ChecksumAlgorithm" => Ok(Self::ChecksumAlgorithm),
            "IntelligentTieringAccessTier" => Ok(Self::IntelligentTieringAccessTier),
            "ReplicationStatus" => Ok(Self::ReplicationStatus),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

pub type EnableRequestProgress = bool;

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

pub type ResponseExpires = String;

#[derive(Debug, Default, Clone)]
pub struct VersioningConfiguration {
    pub mfa_delete: Option<MfaDelete>,
    pub status: Option<BucketVersioningStatus>,
}
impl VersioningConfiguration {
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
pub struct PutBucketVersioningRequest {
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub mfa: Option<Mfa>,
    pub bucket: Option<BucketName>,
    pub versioning_configuration: Option<VersioningConfiguration>,
    pub expected_bucket_owner: Option<AccountId>,
    pub content_md5: Option<ContentMd5>,
}
impl PutBucketVersioningRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct InventoryConfiguration {
    pub filter: Option<InventoryFilter>,
    pub id: Option<InventoryId>,
    pub included_object_versions: Option<InventoryIncludedObjectVersions>,
    pub is_enabled: Option<IsEnabled>,
    pub destination: Option<InventoryDestination>,
    pub optional_fields: Option<InventoryOptionalFields>,
    pub schedule: Option<InventorySchedule>,
}
impl InventoryConfiguration {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct PutObjectLockConfigurationRequest {
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    pub token: Option<ObjectLockToken>,
    pub bucket: Option<BucketName>,
    pub expected_bucket_owner: Option<AccountId>,
    pub request_payer: Option<RequestPayer>,
    pub content_md5: Option<ContentMd5>,
    pub object_lock_configuration: Option<ObjectLockConfiguration>,
}
impl PutObjectLockConfigurationRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

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
pub struct OwnershipControls {
    pub rules: Option<OwnershipControlsRules>,
}
impl OwnershipControls {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct TargetGrant {
    pub permission: Option<BucketLogsPermission>,
    pub grantee: Option<Grantee>,
}
impl TargetGrant {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}
