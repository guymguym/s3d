use std :: str :: FromStr ; use std :: collections :: HashMap ; use std :: collections :: HashSet ; use std :: sync :: Arc ; trait Op { type Input ; type Output ; }

# [derive (Debug , Clone)] pub struct NotificationConfigurationFilter { pub key : Option < S3KeyFilter > , }

# [derive (Debug , Clone)] pub struct S3KeyFilter { pub filter_rules : Option < FilterRuleList > , }

# [derive (Debug , Clone)] pub struct InventoryDestination { pub s3_bucket_destination : Option < InventoryS3BucketDestination > , }

pub type AccessPointArn = String ;

pub type IfMatch = String ;

pub type GrantRead = String ;

pub type ContentLength = i64 ;

# [derive (Debug , Clone)] pub struct PutObjectLockConfigurationOutput { pub request_charged : Option < RequestCharged > , }

pub type Value = String ;

# [derive (Debug , Clone)] pub struct GetObjectLegalHoldRequest { pub request_payer : Option < RequestPayer > , pub key : Option < ObjectKey > , pub version_id : Option < ObjectVersionId > , pub bucket : Option < BucketName > , pub expected_bucket_owner : Option < AccountId > , }

# [derive (Debug , Clone)] pub struct DeleteBucketAnalyticsConfigurationRequest { pub id : Option < AnalyticsId > , pub bucket : Option < BucketName > , pub expected_bucket_owner : Option < AccountId > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct PutBucketMetricsConfiguration ; impl Op for PutBucketMetricsConfiguration { type Input = PutBucketMetricsConfigurationRequest ; type Output = () ; }

# [derive (Debug , Clone)] pub struct AbortMultipartUploadOutput { pub request_charged : Option < RequestCharged > , }

# [derive (Debug , Clone)] pub struct ListBucketsOutput { pub buckets : Option < Buckets > , pub owner : Option < Owner > , }

# [derive (Debug , Clone)] pub struct PutBucketPolicyRequest { pub checksum_algorithm : Option < ChecksumAlgorithm > , pub confirm_remove_self_bucket_access : Option < ConfirmRemoveSelfBucketAccess > , pub content_md5 : Option < ContentMd5 > , pub bucket : Option < BucketName > , pub expected_bucket_owner : Option < AccountId > , pub policy : Option < Policy > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct DeleteBucketMetricsConfiguration ; impl Op for DeleteBucketMetricsConfiguration { type Input = DeleteBucketMetricsConfigurationRequest ; type Output = () ; }

# [derive (Debug , Clone)] pub struct DeleteBucketCorsRequest { pub bucket : Option < BucketName > , pub expected_bucket_owner : Option < AccountId > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum Event { _0 , _1 , _2 , _3 , _4 , _5 , _6 , _7 , _8 , _9 , _10 , _11 , _12 , _13 , _14 , _15 , _16 , _17 , _18 , _19 , _20 , _21 , _22 , _23 , _24 , _25 , _26 , } impl AsRef < str > for Event { fn as_ref (& self) -> & str { match self { Self :: _0 => "s3:ReducedRedundancyLostObject" , Self :: _1 => "s3:ObjectCreated:*" , Self :: _2 => "s3:ObjectCreated:Put" , Self :: _3 => "s3:ObjectCreated:Post" , Self :: _4 => "s3:ObjectCreated:Copy" , Self :: _5 => "s3:ObjectCreated:CompleteMultipartUpload" , Self :: _6 => "s3:ObjectRemoved:*" , Self :: _7 => "s3:ObjectRemoved:Delete" , Self :: _8 => "s3:ObjectRemoved:DeleteMarkerCreated" , Self :: _9 => "s3:ObjectRestore:*" , Self :: _10 => "s3:ObjectRestore:Post" , Self :: _11 => "s3:ObjectRestore:Completed" , Self :: _12 => "s3:Replication:*" , Self :: _13 => "s3:Replication:OperationFailedReplication" , Self :: _14 => "s3:Replication:OperationNotTracked" , Self :: _15 => "s3:Replication:OperationMissedThreshold" , Self :: _16 => "s3:Replication:OperationReplicatedAfterThreshold" , Self :: _17 => "s3:ObjectRestore:Delete" , Self :: _18 => "s3:LifecycleTransition" , Self :: _19 => "s3:IntelligentTiering" , Self :: _20 => "s3:ObjectAcl:Put" , Self :: _21 => "s3:LifecycleExpiration:*" , Self :: _22 => "s3:LifecycleExpiration:Delete" , Self :: _23 => "s3:LifecycleExpiration:DeleteMarkerCreated" , Self :: _24 => "s3:ObjectTagging:*" , Self :: _25 => "s3:ObjectTagging:Put" , Self :: _26 => "s3:ObjectTagging:Delete" , } } } impl TryFrom < & str > for Event { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "s3:ReducedRedundancyLostObject" => Ok (Self :: _0) , "s3:ObjectCreated:*" => Ok (Self :: _1) , "s3:ObjectCreated:Put" => Ok (Self :: _2) , "s3:ObjectCreated:Post" => Ok (Self :: _3) , "s3:ObjectCreated:Copy" => Ok (Self :: _4) , "s3:ObjectCreated:CompleteMultipartUpload" => Ok (Self :: _5) , "s3:ObjectRemoved:*" => Ok (Self :: _6) , "s3:ObjectRemoved:Delete" => Ok (Self :: _7) , "s3:ObjectRemoved:DeleteMarkerCreated" => Ok (Self :: _8) , "s3:ObjectRestore:*" => Ok (Self :: _9) , "s3:ObjectRestore:Post" => Ok (Self :: _10) , "s3:ObjectRestore:Completed" => Ok (Self :: _11) , "s3:Replication:*" => Ok (Self :: _12) , "s3:Replication:OperationFailedReplication" => Ok (Self :: _13) , "s3:Replication:OperationNotTracked" => Ok (Self :: _14) , "s3:Replication:OperationMissedThreshold" => Ok (Self :: _15) , "s3:Replication:OperationReplicatedAfterThreshold" => Ok (Self :: _16) , "s3:ObjectRestore:Delete" => Ok (Self :: _17) , "s3:LifecycleTransition" => Ok (Self :: _18) , "s3:IntelligentTiering" => Ok (Self :: _19) , "s3:ObjectAcl:Put" => Ok (Self :: _20) , "s3:LifecycleExpiration:*" => Ok (Self :: _21) , "s3:LifecycleExpiration:Delete" => Ok (Self :: _22) , "s3:LifecycleExpiration:DeleteMarkerCreated" => Ok (Self :: _23) , "s3:ObjectTagging:*" => Ok (Self :: _24) , "s3:ObjectTagging:Put" => Ok (Self :: _25) , "s3:ObjectTagging:Delete" => Ok (Self :: _26) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

# [derive (Debug , Clone)] pub struct MetricsConfiguration { pub id : Option < MetricsId > , pub filter : Option < MetricsFilter > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum Tier { Standard , Bulk , Expedited , } impl AsRef < str > for Tier { fn as_ref (& self) -> & str { match self { Self :: Standard => "Standard" , Self :: Bulk => "Bulk" , Self :: Expedited => "Expedited" , } } } impl TryFrom < & str > for Tier { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "Standard" => Ok (Self :: Standard) , "Bulk" => Ok (Self :: Bulk) , "Expedited" => Ok (Self :: Expedited) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

# [derive (Debug , Clone)] pub struct GetBucketLoggingOutput { pub logging_enabled : Option < LoggingEnabled > , }

# [derive (Debug , Clone)] pub struct PutBucketVersioningRequest { pub mfa : Option < Mfa > , pub bucket : Option < BucketName > , pub versioning_configuration : Option < VersioningConfiguration > , pub checksum_algorithm : Option < ChecksumAlgorithm > , pub content_md5 : Option < ContentMd5 > , pub expected_bucket_owner : Option < AccountId > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum ObjectStorageClass { STANDARD , REDUCED_REDUNDANCY , GLACIER , STANDARD_IA , ONEZONE_IA , INTELLIGENT_TIERING , DEEP_ARCHIVE , OUTPOSTS , GLACIER_IR , } impl AsRef < str > for ObjectStorageClass { fn as_ref (& self) -> & str { match self { Self :: STANDARD => "STANDARD" , Self :: REDUCED_REDUNDANCY => "REDUCED_REDUNDANCY" , Self :: GLACIER => "GLACIER" , Self :: STANDARD_IA => "STANDARD_IA" , Self :: ONEZONE_IA => "ONEZONE_IA" , Self :: INTELLIGENT_TIERING => "INTELLIGENT_TIERING" , Self :: DEEP_ARCHIVE => "DEEP_ARCHIVE" , Self :: OUTPOSTS => "OUTPOSTS" , Self :: GLACIER_IR => "GLACIER_IR" , } } } impl TryFrom < & str > for ObjectStorageClass { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "STANDARD" => Ok (Self :: STANDARD) , "REDUCED_REDUNDANCY" => Ok (Self :: REDUCED_REDUNDANCY) , "GLACIER" => Ok (Self :: GLACIER) , "STANDARD_IA" => Ok (Self :: STANDARD_IA) , "ONEZONE_IA" => Ok (Self :: ONEZONE_IA) , "INTELLIGENT_TIERING" => Ok (Self :: INTELLIGENT_TIERING) , "DEEP_ARCHIVE" => Ok (Self :: DEEP_ARCHIVE) , "OUTPOSTS" => Ok (Self :: OUTPOSTS) , "GLACIER_IR" => Ok (Self :: GLACIER_IR) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub type ResponseExpires = String ;

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct DeleteObjectTagging ; impl Op for DeleteObjectTagging { type Input = DeleteObjectTaggingRequest ; type Output = DeleteObjectTaggingOutput ; }

pub type Start = i64 ;

pub type DeleteMarkerVersionId = String ;

pub type Location = String ;

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct DeletePublicAccessBlock ; impl Op for DeletePublicAccessBlock { type Input = DeletePublicAccessBlockRequest ; type Output = () ; }

pub type QueueArn = String ;

# [derive (Debug , Clone)] pub struct InventoryFilter { pub prefix : Option < Prefix > , }

# [derive (Debug , Clone)] pub struct ListBucketAnalyticsConfigurationsOutput { pub analytics_configuration_list : Option < AnalyticsConfigurationList > , pub next_continuation_token : Option < NextToken > , pub continuation_token : Option < Token > , pub is_truncated : Option < IsTruncated > , }

# [derive (Debug , Clone)] pub struct PutObjectTaggingOutput { pub version_id : Option < ObjectVersionId > , }

# [derive (Debug , Clone)] pub struct AnalyticsExportDestination { pub s3_bucket_destination : Option < AnalyticsS3BucketDestination > , }

pub type RecordDelimiter = String ;

pub type IntelligentTieringId = String ;

pub type LifecycleRules = Vec < LifecycleRule > ;

# [derive (Debug , Clone)] pub struct ListBucketIntelligentTieringConfigurationsRequest { pub bucket : Option < BucketName > , pub continuation_token : Option < Token > , }

# [derive (Debug , Clone)] pub struct GetObjectAttributesOutput { pub delete_marker : Option < DeleteMarker > , pub object_size : Option < ObjectSize > , pub e_tag : Option < ETag > , pub checksum : Option < Checksum > , pub storage_class : Option < StorageClass > , pub version_id : Option < ObjectVersionId > , pub object_parts : Option < GetObjectAttributesParts > , pub request_charged : Option < RequestCharged > , pub last_modified : Option < LastModified > , }

# [derive (Debug , Clone)] pub struct PutBucketMetricsConfigurationRequest { pub id : Option < MetricsId > , pub expected_bucket_owner : Option < AccountId > , pub metrics_configuration : Option < MetricsConfiguration > , pub bucket : Option < BucketName > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct GetBucketInventoryConfiguration ; impl Op for GetBucketInventoryConfiguration { type Input = GetBucketInventoryConfigurationRequest ; type Output = GetBucketInventoryConfigurationOutput ; }

# [derive (Debug , Clone)] pub struct InvalidObjectState { pub access_tier : Option < IntelligentTieringAccessTier > , pub storage_class : Option < StorageClass > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct PutBucketLogging ; impl Op for PutBucketLogging { type Input = PutBucketLoggingRequest ; type Output = () ; }

# [derive (Debug , Clone)] pub struct ReplicationRule { pub delete_marker_replication : Option < DeleteMarkerReplication > , pub id : Option < Id > , pub priority : Option < Priority > , pub existing_object_replication : Option < ExistingObjectReplication > , pub status : Option < ReplicationRuleStatus > , pub filter : Option < ReplicationRuleFilter > , pub prefix : Option < Prefix > , pub source_selection_criteria : Option < SourceSelectionCriteria > , pub destination : Option < Destination > , }

# [derive (Debug , Clone)] pub struct DeletedObject { pub key : Option < ObjectKey > , pub delete_marker_version_id : Option < DeleteMarkerVersionId > , pub version_id : Option < ObjectVersionId > , pub delete_marker : Option < DeleteMarker > , }

# [derive (Debug , Clone)] pub struct DeleteMarkerEntry { pub version_id : Option < ObjectVersionId > , pub is_latest : Option < IsLatest > , pub key : Option < ObjectKey > , pub last_modified : Option < LastModified > , pub owner : Option < Owner > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum CompressionType { NONE , GZIP , BZIP2 , } impl AsRef < str > for CompressionType { fn as_ref (& self) -> & str { match self { Self :: NONE => "NONE" , Self :: GZIP => "GZIP" , Self :: BZIP2 => "BZIP2" , } } } impl TryFrom < & str > for CompressionType { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "NONE" => Ok (Self :: NONE) , "GZIP" => Ok (Self :: GZIP) , "BZIP2" => Ok (Self :: BZIP2) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub type Expiration = String ;

# [derive (Debug , Clone)] pub struct ListObjectVersionsOutput { pub encoding_type : Option < EncodingType > , pub delete_markers : Option < DeleteMarkers > , pub max_keys : Option < MaxKeys > , pub common_prefixes : Option < CommonPrefixList > , pub is_truncated : Option < IsTruncated > , pub key_marker : Option < KeyMarker > , pub name : Option < BucketName > , pub delimiter : Option < Delimiter > , pub next_version_id_marker : Option < NextVersionIdMarker > , pub prefix : Option < Prefix > , pub version_id_marker : Option < VersionIdMarker > , pub versions : Option < ObjectVersionList > , pub next_key_marker : Option < NextKeyMarker > , }

pub type MultipartUploadId = String ;

# [derive (Debug , Clone)] pub struct Object { pub size : Option < Size > , pub checksum_algorithm : Option < ChecksumAlgorithmList > , pub key : Option < ObjectKey > , pub owner : Option < Owner > , pub last_modified : Option < LastModified > , pub storage_class : Option < ObjectStorageClass > , pub e_tag : Option < ETag > , }

pub type ObjectVersionList = Vec < ObjectVersion > ;

# [derive (Debug , Clone)] pub struct ListBucketIntelligentTieringConfigurationsOutput { pub intelligent_tiering_configuration_list : Option < IntelligentTieringConfigurationList > , pub continuation_token : Option < Token > , pub is_truncated : Option < IsTruncated > , pub next_continuation_token : Option < NextToken > , }

pub type Mfa = String ;

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct CreateMultipartUpload ; impl Op for CreateMultipartUpload { type Input = CreateMultipartUploadRequest ; type Output = CreateMultipartUploadOutput ; }

pub type ContentLanguage = String ;

# [derive (Debug , Clone)] pub struct OwnershipControls { pub rules : Option < OwnershipControlsRules > , }

pub type NextMarker = String ;

# [derive (Debug , Clone)] pub struct ObjectPart { pub part_number : Option < PartNumber > , pub checksum_sha1 : Option < ChecksumSha1 > , pub checksum_sha256 : Option < ChecksumSha256 > , pub checksum_crc32 : Option < ChecksumCrc32 > , pub checksum_crc32_c : Option < ChecksumCrc32c > , pub size : Option < Size > , }

# [derive (Debug , Clone)] pub struct JsonOutput { pub record_delimiter : Option < RecordDelimiter > , }

# [derive (Debug , Clone)] pub struct RoutingRule { pub redirect : Option < Redirect > , pub condition : Option < Condition > , }

# [derive (Debug , Clone)] pub struct PutBucketLoggingRequest { pub expected_bucket_owner : Option < AccountId > , pub bucket : Option < BucketName > , pub bucket_logging_status : Option < BucketLoggingStatus > , pub checksum_algorithm : Option < ChecksumAlgorithm > , pub content_md5 : Option < ContentMd5 > , }

# [derive (Debug , Clone)] pub struct Destination { pub encryption_configuration : Option < EncryptionConfiguration > , pub access_control_translation : Option < AccessControlTranslation > , pub account : Option < AccountId > , pub metrics : Option < Metrics > , pub replication_time : Option < ReplicationTime > , pub storage_class : Option < StorageClass > , pub bucket : Option < BucketName > , }

pub type DaysAfterInitiation = i32 ;

pub type Parts = Vec < Part > ;

pub type MetadataValue = String ;

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct GetObject ; impl Op for GetObject { type Input = GetObjectRequest ; type Output = GetObjectOutput ; }

pub type ServerSideEncryptionRules = Vec < ServerSideEncryptionRule > ;

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct UploadPart ; impl Op for UploadPart { type Input = UploadPartRequest ; type Output = UploadPartOutput ; }

pub type CopySourceIfModifiedSince = String ;

pub type Code = String ;

pub type CopySourceIfMatch = String ;

pub type Marker = String ;

# [derive (Debug , Clone)] pub struct GetObjectLegalHoldOutput { pub legal_hold : Option < ObjectLockLegalHold > , }

# [derive (Debug , Clone)] pub struct PutBucketIntelligentTieringConfigurationRequest { pub id : Option < IntelligentTieringId > , pub bucket : Option < BucketName > , pub intelligent_tiering_configuration : Option < IntelligentTieringConfiguration > , }

# [derive (Debug , Clone)] pub struct DeleteObjectTaggingOutput { pub version_id : Option < ObjectVersionId > , }

pub type RequestRoute = String ;

pub type MaxAgeSeconds = i32 ;

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct GetBucketAccelerateConfiguration ; impl Op for GetBucketAccelerateConfiguration { type Input = GetBucketAccelerateConfigurationRequest ; type Output = GetBucketAccelerateConfigurationOutput ; }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct GetBucketPolicy ; impl Op for GetBucketPolicy { type Input = GetBucketPolicyRequest ; type Output = GetBucketPolicyOutput ; }

# [derive (Debug , Clone)] pub struct AnalyticsS3BucketDestination { pub bucket_account_id : Option < AccountId > , pub prefix : Option < Prefix > , pub bucket : Option < BucketName > , pub format : Option < AnalyticsS3ExportFileFormat > , }

pub type StartAfter = String ;

pub type OwnershipControlsRules = Vec < OwnershipControlsRule > ;

# [derive (Debug , Clone)] pub struct Metrics { pub event_threshold : Option < ReplicationTimeValue > , pub status : Option < MetricsStatus > , }

pub type VersionCount = i32 ;

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum MetricsStatus { Enabled , Disabled , } impl AsRef < str > for MetricsStatus { fn as_ref (& self) -> & str { match self { Self :: Enabled => "Enabled" , Self :: Disabled => "Disabled" , } } } impl TryFrom < & str > for MetricsStatus { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "Enabled" => Ok (Self :: Enabled) , "Disabled" => Ok (Self :: Disabled) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct DeleteBucketCors ; impl Op for DeleteBucketCors { type Input = DeleteBucketCorsRequest ; type Output = () ; }

pub type ObjectList = Vec < Object > ;

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum ReplicationTimeStatus { Enabled , Disabled , } impl AsRef < str > for ReplicationTimeStatus { fn as_ref (& self) -> & str { match self { Self :: Enabled => "Enabled" , Self :: Disabled => "Disabled" , } } } impl TryFrom < & str > for ReplicationTimeStatus { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "Enabled" => Ok (Self :: Enabled) , "Disabled" => Ok (Self :: Disabled) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum IntelligentTieringStatus { Enabled , Disabled , } impl AsRef < str > for IntelligentTieringStatus { fn as_ref (& self) -> & str { match self { Self :: Enabled => "Enabled" , Self :: Disabled => "Disabled" , } } } impl TryFrom < & str > for IntelligentTieringStatus { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "Enabled" => Ok (Self :: Enabled) , "Disabled" => Ok (Self :: Disabled) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub type KmsContext = String ;

# [derive (Debug , Clone)] pub struct LifecycleRuleAndOperator { pub tags : Option < TagSet > , pub object_size_greater_than : Option < ObjectSizeGreaterThanBytes > , pub object_size_less_than : Option < ObjectSizeLessThanBytes > , pub prefix : Option < Prefix > , }

pub type ResponseCacheControl = String ;

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct SelectObjectContent ; impl Op for SelectObjectContent { type Input = SelectObjectContentRequest ; type Output = SelectObjectContentOutput ; }

# [derive (Debug , Clone)] pub struct DeleteBucketTaggingRequest { pub bucket : Option < BucketName > , pub expected_bucket_owner : Option < AccountId > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct CreateBucket ; impl Op for CreateBucket { type Input = CreateBucketRequest ; type Output = CreateBucketOutput ; }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct ListObjectVersions ; impl Op for ListObjectVersions { type Input = ListObjectVersionsRequest ; type Output = ListObjectVersionsOutput ; }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct DeleteBucketWebsite ; impl Op for DeleteBucketWebsite { type Input = DeleteBucketWebsiteRequest ; type Output = () ; }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct GetBucketAnalyticsConfiguration ; impl Op for GetBucketAnalyticsConfiguration { type Input = GetBucketAnalyticsConfigurationRequest ; type Output = GetBucketAnalyticsConfigurationOutput ; }

# [derive (Debug , Clone)] pub struct FilterRule { pub value : Option < FilterRuleValue > , pub name : Option < FilterRuleName > , }

# [derive (Debug , Clone)] pub struct Encryption { pub encryption_type : Option < ServerSideEncryption > , pub kms_context : Option < KmsContext > , pub kms_key_id : Option < SsekmsKeyId > , }

# [derive (Debug , Clone)] pub struct AccessControlTranslation { pub owner : Option < OwnerOverride > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct ListObjectsV2 ; impl Op for ListObjectsV2 { type Input = ListObjectsV2Request ; type Output = ListObjectsV2Output ; }

# [derive (Debug , Clone)] pub struct GetBucketInventoryConfigurationOutput { pub inventory_configuration : Option < InventoryConfiguration > , }

pub type ContentMd5 = String ;

pub type Metadata = HashMap < MetadataKey , MetadataValue > ;

# [derive (Debug , Clone)] pub struct ObjectLockLegalHold { pub status : Option < ObjectLockLegalHoldStatus > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum ObjectLockRetentionMode { GOVERNANCE , COMPLIANCE , } impl AsRef < str > for ObjectLockRetentionMode { fn as_ref (& self) -> & str { match self { Self :: GOVERNANCE => "GOVERNANCE" , Self :: COMPLIANCE => "COMPLIANCE" , } } } impl TryFrom < & str > for ObjectLockRetentionMode { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "GOVERNANCE" => Ok (Self :: GOVERNANCE) , "COMPLIANCE" => Ok (Self :: COMPLIANCE) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum ChecksumMode { ENABLED , } impl AsRef < str > for ChecksumMode { fn as_ref (& self) -> & str { match self { Self :: ENABLED => "ENABLED" , } } } impl TryFrom < & str > for ChecksumMode { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "ENABLED" => Ok (Self :: ENABLED) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub type LastModified = String ;

pub type MetadataKey = String ;

# [derive (Debug , Clone)] pub struct CopyObjectRequest { pub content_encoding : Option < ContentEncoding > , pub expected_source_bucket_owner : Option < AccountId > , pub expected_bucket_owner : Option < AccountId > , pub checksum_algorithm : Option < ChecksumAlgorithm > , pub metadata : Option < Metadata > , pub copy_source_if_match : Option < CopySourceIfMatch > , pub metadata_directive : Option < MetadataDirective > , pub object_lock_mode : Option < ObjectLockMode > , pub key : Option < ObjectKey > , pub server_side_encryption : Option < ServerSideEncryption > , pub copy_source_if_modified_since : Option < CopySourceIfModifiedSince > , pub ssekms_encryption_context : Option < SsekmsEncryptionContext > , pub tagging : Option < TaggingHeader > , pub tagging_directive : Option < TaggingDirective > , pub website_redirect_location : Option < WebsiteRedirectLocation > , pub content_language : Option < ContentLanguage > , pub grant_full_control : Option < GrantFullControl > , pub bucket : Option < BucketName > , pub grant_read : Option < GrantRead > , pub cache_control : Option < CacheControl > , pub copy_source_sse_customer_key_md5 : Option < CopySourceSseCustomerKeyMd5 > , pub grant_read_acp : Option < GrantReadAcp > , pub sse_customer_key_md5 : Option < SseCustomerKeyMd5 > , pub copy_source_if_unmodified_since : Option < CopySourceIfUnmodifiedSince > , pub copy_source : Option < CopySource > , pub acl : Option < ObjectCannedAcl > , pub sse_customer_key : Option < SseCustomerKey > , pub copy_source_sse_customer_algorithm : Option < CopySourceSseCustomerAlgorithm > , pub content_type : Option < ContentType > , pub storage_class : Option < StorageClass > , pub object_lock_retain_until_date : Option < ObjectLockRetainUntilDate > , pub content_disposition : Option < ContentDisposition > , pub object_lock_legal_hold_status : Option < ObjectLockLegalHoldStatus > , pub ssekms_key_id : Option < SsekmsKeyId > , pub copy_source_sse_customer_key : Option < CopySourceSseCustomerKey > , pub bucket_key_enabled : Option < BucketKeyEnabled > , pub expires : Option < Expires > , pub request_payer : Option < RequestPayer > , pub sse_customer_algorithm : Option < SseCustomerAlgorithm > , pub grant_write_acp : Option < GrantWriteAcp > , pub copy_source_if_none_match : Option < CopySourceIfNoneMatch > , }

# [derive (Debug , Clone)] pub struct ListObjectVersionsRequest { pub prefix : Option < Prefix > , pub version_id_marker : Option < VersionIdMarker > , pub max_keys : Option < MaxKeys > , pub encoding_type : Option < EncodingType > , pub expected_bucket_owner : Option < AccountId > , pub key_marker : Option < KeyMarker > , pub delimiter : Option < Delimiter > , pub bucket : Option < BucketName > , }

pub type Quiet = bool ;

# [derive (Debug , Clone)] pub struct ObjectIdentifier { pub version_id : Option < ObjectVersionId > , pub key : Option < ObjectKey > , }

# [derive (Debug , Clone)] pub struct ListBucketInventoryConfigurationsRequest { pub expected_bucket_owner : Option < AccountId > , pub continuation_token : Option < Token > , pub bucket : Option < BucketName > , }

pub type CompletedPartList = Vec < CompletedPart > ;

# [derive (Debug , Clone)] pub struct Sses3 { }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum BucketCannedAcl { private , public_read , public_read_write , authenticated_read , } impl AsRef < str > for BucketCannedAcl { fn as_ref (& self) -> & str { match self { Self :: private => "private" , Self :: public_read => "public-read" , Self :: public_read_write => "public-read-write" , Self :: authenticated_read => "authenticated-read" , } } } impl TryFrom < & str > for BucketCannedAcl { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "private" => Ok (Self :: private) , "public-read" => Ok (Self :: public_read) , "public-read-write" => Ok (Self :: public_read_write) , "authenticated-read" => Ok (Self :: authenticated_read) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

# [derive (Debug , Clone)] pub struct ExistingObjectReplication { pub status : Option < ExistingObjectReplicationStatus > , }

# [derive (Debug , Clone)] pub struct ListMultipartUploadsOutput { pub next_upload_id_marker : Option < NextUploadIdMarker > , pub is_truncated : Option < IsTruncated > , pub max_uploads : Option < MaxUploads > , pub prefix : Option < Prefix > , pub bucket : Option < BucketName > , pub upload_id_marker : Option < UploadIdMarker > , pub uploads : Option < MultipartUploadList > , pub key_marker : Option < KeyMarker > , pub delimiter : Option < Delimiter > , pub encoding_type : Option < EncodingType > , pub next_key_marker : Option < NextKeyMarker > , pub common_prefixes : Option < CommonPrefixList > , }

# [derive (Debug , Clone)] pub struct SelectParameters { pub output_serialization : Option < OutputSerialization > , pub expression : Option < Expression > , pub expression_type : Option < ExpressionType > , pub input_serialization : Option < InputSerialization > , }

pub type PartsCount = i32 ;

pub type Minutes = i32 ;

# [derive (Debug , Clone)] pub struct Ssekms { pub key_id : Option < SsekmsKeyId > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum IntelligentTieringAccessTier { ARCHIVE_ACCESS , DEEP_ARCHIVE_ACCESS , } impl AsRef < str > for IntelligentTieringAccessTier { fn as_ref (& self) -> & str { match self { Self :: ARCHIVE_ACCESS => "ARCHIVE_ACCESS" , Self :: DEEP_ARCHIVE_ACCESS => "DEEP_ARCHIVE_ACCESS" , } } } impl TryFrom < & str > for IntelligentTieringAccessTier { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "ARCHIVE_ACCESS" => Ok (Self :: ARCHIVE_ACCESS) , "DEEP_ARCHIVE_ACCESS" => Ok (Self :: DEEP_ARCHIVE_ACCESS) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

# [derive (Debug , Clone)] pub struct GetBucketLifecycleConfigurationRequest { pub expected_bucket_owner : Option < AccountId > , pub bucket : Option < BucketName > , }

# [derive (Debug , Clone)] pub struct GetBucketInventoryConfigurationRequest { pub expected_bucket_owner : Option < AccountId > , pub bucket : Option < BucketName > , pub id : Option < InventoryId > , }

# [derive (Debug , Clone)] pub struct PutObjectLegalHoldRequest { pub legal_hold : Option < ObjectLockLegalHold > , pub bucket : Option < BucketName > , pub expected_bucket_owner : Option < AccountId > , pub content_md5 : Option < ContentMd5 > , pub key : Option < ObjectKey > , pub version_id : Option < ObjectVersionId > , pub request_payer : Option < RequestPayer > , pub checksum_algorithm : Option < ChecksumAlgorithm > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct GetBucketIntelligentTieringConfiguration ; impl Op for GetBucketIntelligentTieringConfiguration { type Input = GetBucketIntelligentTieringConfigurationRequest ; type Output = GetBucketIntelligentTieringConfigurationOutput ; }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct GetBucketAcl ; impl Op for GetBucketAcl { type Input = GetBucketAclRequest ; type Output = GetBucketAclOutput ; }

pub type AccountId = String ;

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum MetadataDirective { COPY , REPLACE , } impl AsRef < str > for MetadataDirective { fn as_ref (& self) -> & str { match self { Self :: COPY => "COPY" , Self :: REPLACE => "REPLACE" , } } } impl TryFrom < & str > for MetadataDirective { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "COPY" => Ok (Self :: COPY) , "REPLACE" => Ok (Self :: REPLACE) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

# [derive (Debug , Clone)] pub struct OutputLocation { pub s3 : Option < S3Location > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct GetPublicAccessBlock ; impl Op for GetPublicAccessBlock { type Input = GetPublicAccessBlockRequest ; type Output = GetPublicAccessBlockOutput ; }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum MfaDelete { Enabled , Disabled , } impl AsRef < str > for MfaDelete { fn as_ref (& self) -> & str { match self { Self :: Enabled => "Enabled" , Self :: Disabled => "Disabled" , } } } impl TryFrom < & str > for MfaDelete { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "Enabled" => Ok (Self :: Enabled) , "Disabled" => Ok (Self :: Disabled) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub type PartNumber = i32 ;

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum EncodingType { url , } impl AsRef < str > for EncodingType { fn as_ref (& self) -> & str { match self { Self :: url => "url" , } } } impl TryFrom < & str > for EncodingType { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "url" => Ok (Self :: url) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub type InventoryConfigurationList = Vec < InventoryConfiguration > ;

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum Protocol { http , https , } impl AsRef < str > for Protocol { fn as_ref (& self) -> & str { match self { Self :: http => "http" , Self :: https => "https" , } } } impl TryFrom < & str > for Protocol { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "http" => Ok (Self :: http) , "https" => Ok (Self :: https) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

# [derive (Debug , Clone)] pub struct Stats { pub bytes_returned : Option < BytesReturned > , pub bytes_processed : Option < BytesProcessed > , pub bytes_scanned : Option < BytesScanned > , }

# [derive (Debug , Clone)] pub struct PutBucketReplicationRequest { pub bucket : Option < BucketName > , pub checksum_algorithm : Option < ChecksumAlgorithm > , pub expected_bucket_owner : Option < AccountId > , pub replication_configuration : Option < ReplicationConfiguration > , pub content_md5 : Option < ContentMd5 > , pub token : Option < ObjectLockToken > , }

pub type ChecksumCrc32 = String ;

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct GetObjectTagging ; impl Op for GetObjectTagging { type Input = GetObjectTaggingRequest ; type Output = GetObjectTaggingOutput ; }

# [derive (Debug , Clone)] pub struct NoncurrentVersionExpiration { pub newer_noncurrent_versions : Option < VersionCount > , pub noncurrent_days : Option < Days > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct GetBucketNotificationConfiguration ; impl Op for GetBucketNotificationConfiguration { type Input = GetBucketNotificationConfigurationRequest ; type Output = NotificationConfiguration ; }

# [derive (Debug , Clone)] pub struct GetObjectLockConfigurationRequest { pub bucket : Option < BucketName > , pub expected_bucket_owner : Option < AccountId > , }

pub type ErrorMessage = String ;

pub type IfNoneMatch = String ;

# [derive (Debug , Clone)] pub struct IntelligentTieringAndOperator { pub prefix : Option < Prefix > , pub tags : Option < TagSet > , }

# [derive (Debug , Clone)] pub struct Redirect { pub replace_key_with : Option < ReplaceKeyWith > , pub http_redirect_code : Option < HttpRedirectCode > , pub protocol : Option < Protocol > , pub host_name : Option < HostName > , pub replace_key_prefix_with : Option < ReplaceKeyPrefixWith > , }

pub type Date = String ;

pub type Description = String ;

pub type ObjectIdentifierList = Vec < ObjectIdentifier > ;

# [derive (Debug , Clone)] pub struct PutObjectTaggingRequest { pub version_id : Option < ObjectVersionId > , pub request_payer : Option < RequestPayer > , pub expected_bucket_owner : Option < AccountId > , pub tagging : Option < Tagging > , pub key : Option < ObjectKey > , pub checksum_algorithm : Option < ChecksumAlgorithm > , pub bucket : Option < BucketName > , pub content_md5 : Option < ContentMd5 > , }

pub type PartsList = Vec < ObjectPart > ;

pub type Suffix = String ;

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct PutObjectLockConfiguration ; impl Op for PutObjectLockConfiguration { type Input = PutObjectLockConfigurationRequest ; type Output = PutObjectLockConfigurationOutput ; }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct PutBucketIntelligentTieringConfiguration ; impl Op for PutBucketIntelligentTieringConfiguration { type Input = PutBucketIntelligentTieringConfigurationRequest ; type Output = () ; }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum ObjectCannedAcl { private , public_read , public_read_write , authenticated_read , aws_exec_read , bucket_owner_read , bucket_owner_full_control , } impl AsRef < str > for ObjectCannedAcl { fn as_ref (& self) -> & str { match self { Self :: private => "private" , Self :: public_read => "public-read" , Self :: public_read_write => "public-read-write" , Self :: authenticated_read => "authenticated-read" , Self :: aws_exec_read => "aws-exec-read" , Self :: bucket_owner_read => "bucket-owner-read" , Self :: bucket_owner_full_control => "bucket-owner-full-control" , } } } impl TryFrom < & str > for ObjectCannedAcl { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "private" => Ok (Self :: private) , "public-read" => Ok (Self :: public_read) , "public-read-write" => Ok (Self :: public_read_write) , "authenticated-read" => Ok (Self :: authenticated_read) , "aws-exec-read" => Ok (Self :: aws_exec_read) , "bucket-owner-read" => Ok (Self :: bucket_owner_read) , "bucket-owner-full-control" => Ok (Self :: bucket_owner_full_control) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub type HttpErrorCodeReturnedEquals = String ;

# [derive (Debug , Clone)] pub struct CommonPrefix { pub prefix : Option < Prefix > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct DeleteObject ; impl Op for DeleteObject { type Input = DeleteObjectRequest ; type Output = DeleteObjectOutput ; }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct ListBuckets ; impl Op for ListBuckets { type Input = () ; type Output = ListBucketsOutput ; }

# [derive (Debug , Clone)] pub struct GetBucketTaggingRequest { pub bucket : Option < BucketName > , pub expected_bucket_owner : Option < AccountId > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct PutBucketAnalyticsConfiguration ; impl Op for PutBucketAnalyticsConfiguration { type Input = PutBucketAnalyticsConfigurationRequest ; type Output = () ; }

# [derive (Debug , Clone)] pub struct DeleteMarkerReplication { pub status : Option < DeleteMarkerReplicationStatus > , }

pub type TagSet = Vec < Tag > ;

pub type ContentRange = String ;

# [derive (Debug , Clone)] pub struct GetObjectAclOutput { pub owner : Option < Owner > , pub grants : Option < Grants > , pub request_charged : Option < RequestCharged > , }

# [derive (Debug , Clone)] pub struct PutBucketInventoryConfigurationRequest { pub expected_bucket_owner : Option < AccountId > , pub inventory_configuration : Option < InventoryConfiguration > , pub bucket : Option < BucketName > , pub id : Option < InventoryId > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct HeadBucket ; impl Op for HeadBucket { type Input = HeadBucketRequest ; type Output = () ; }

pub type FilterRuleList = Vec < FilterRule > ;

pub type TargetPrefix = String ;

# [derive (Debug , Clone)] pub struct SourceSelectionCriteria { pub replica_modifications : Option < ReplicaModifications > , pub sse_kms_encrypted_objects : Option < SseKmsEncryptedObjects > , }

# [derive (Debug , Clone)] pub struct GetBucketIntelligentTieringConfigurationOutput { pub intelligent_tiering_configuration : Option < IntelligentTieringConfiguration > , }

pub type DeleteMarkers = Vec < DeleteMarkerEntry > ;

# [derive (Debug , Clone)] pub struct GetBucketLocationRequest { pub bucket : Option < BucketName > , pub expected_bucket_owner : Option < AccountId > , }

# [derive (Debug , Clone)] pub struct ListBucketInventoryConfigurationsOutput { pub is_truncated : Option < IsTruncated > , pub next_continuation_token : Option < NextToken > , pub continuation_token : Option < Token > , pub inventory_configuration_list : Option < InventoryConfigurationList > , }

pub type MaxParts = i32 ;

pub type Id = String ;

pub type NoncurrentVersionTransitionList = Vec < NoncurrentVersionTransition > ;

# [derive (Debug , Clone)] pub struct PolicyStatus { pub is_public : Option < IsPublic > , }

pub type IntelligentTieringConfigurationList = Vec < IntelligentTieringConfiguration > ;

pub type Uri = String ;

pub type Range = String ;

# [derive (Debug , Clone)] pub struct CreateBucketRequest { pub grant_write_acp : Option < GrantWriteAcp > , pub grant_full_control : Option < GrantFullControl > , pub acl : Option < BucketCannedAcl > , pub bucket : Option < BucketName > , pub grant_read_acp : Option < GrantReadAcp > , pub grant_write : Option < GrantWrite > , pub object_lock_enabled_for_bucket : Option < ObjectLockEnabledForBucket > , pub grant_read : Option < GrantRead > , pub object_ownership : Option < ObjectOwnership > , pub create_bucket_configuration : Option < CreateBucketConfiguration > , }

pub type LambdaFunctionArn = String ;

# [derive (Debug , Clone)] pub struct AccessControlPolicy { pub grants : Option < Grants > , pub owner : Option < Owner > , }

# [derive (Debug , Clone)] pub struct AnalyticsConfiguration { pub filter : Option < AnalyticsFilter > , pub id : Option < AnalyticsId > , pub storage_class_analysis : Option < StorageClassAnalysis > , }

pub type MetricsId = String ;

pub type LocationPrefix = String ;

pub type AcceptRanges = String ;

pub type AllowedMethods = Vec < AllowedMethod > ;

pub type ReplaceKeyWith = String ;

# [derive (Debug , Clone)] pub struct ListObjectsRequest { pub marker : Option < Marker > , pub bucket : Option < BucketName > , pub delimiter : Option < Delimiter > , pub prefix : Option < Prefix > , pub max_keys : Option < MaxKeys > , pub encoding_type : Option < EncodingType > , pub expected_bucket_owner : Option < AccountId > , pub request_payer : Option < RequestPayer > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum BucketAccelerateStatus { Enabled , Suspended , } impl AsRef < str > for BucketAccelerateStatus { fn as_ref (& self) -> & str { match self { Self :: Enabled => "Enabled" , Self :: Suspended => "Suspended" , } } } impl TryFrom < & str > for BucketAccelerateStatus { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "Enabled" => Ok (Self :: Enabled) , "Suspended" => Ok (Self :: Suspended) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

# [derive (Debug , Clone)] pub enum ReplicationRuleFilter { And (ReplicationRuleAndOperator) , Tag (Tag) , Prefix (Prefix) , }

# [derive (Debug , Clone)] pub struct CorsConfiguration { pub cors_rules : Option < CorsRules > , }

# [derive (Debug , Clone)] pub struct GetObjectTaggingOutput { pub version_id : Option < ObjectVersionId > , pub tag_set : Option < TagSet > , }

pub type MaxUploads = i32 ;

# [derive (Debug , Clone)] pub struct GetObjectAttributesRequest { pub part_number_marker : Option < PartNumberMarker > , pub sse_customer_key_md5 : Option < SseCustomerKeyMd5 > , pub version_id : Option < ObjectVersionId > , pub sse_customer_key : Option < SseCustomerKey > , pub expected_bucket_owner : Option < AccountId > , pub object_attributes : Option < ObjectAttributesList > , pub request_payer : Option < RequestPayer > , pub bucket : Option < BucketName > , pub max_parts : Option < MaxParts > , pub key : Option < ObjectKey > , pub sse_customer_algorithm : Option < SseCustomerAlgorithm > , }

# [derive (Debug , Clone)] pub struct GetBucketAccelerateConfigurationOutput { pub status : Option < BucketAccelerateStatus > , }

pub type Days = i32 ;

pub type AnalyticsId = String ;

# [derive (Debug , Clone)] pub struct GetBucketVersioningOutput { pub status : Option < BucketVersioningStatus > , pub mfa_delete : Option < MfaDeleteStatus > , }

# [derive (Debug , Clone)] pub struct LifecycleRule { pub prefix : Option < Prefix > , pub transitions : Option < TransitionList > , pub filter : Option < LifecycleRuleFilter > , pub id : Option < Id > , pub expiration : Option < LifecycleExpiration > , pub noncurrent_version_transitions : Option < NoncurrentVersionTransitionList > , pub noncurrent_version_expiration : Option < NoncurrentVersionExpiration > , pub status : Option < ExpirationStatus > , pub abort_incomplete_multipart_upload : Option < AbortIncompleteMultipartUpload > , }

pub type FilterRuleValue = String ;

pub type UserMetadata = Vec < MetadataEntry > ;

pub type ObjectLockToken = String ;

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum TransitionStorageClass { GLACIER , STANDARD_IA , ONEZONE_IA , INTELLIGENT_TIERING , DEEP_ARCHIVE , GLACIER_IR , } impl AsRef < str > for TransitionStorageClass { fn as_ref (& self) -> & str { match self { Self :: GLACIER => "GLACIER" , Self :: STANDARD_IA => "STANDARD_IA" , Self :: ONEZONE_IA => "ONEZONE_IA" , Self :: INTELLIGENT_TIERING => "INTELLIGENT_TIERING" , Self :: DEEP_ARCHIVE => "DEEP_ARCHIVE" , Self :: GLACIER_IR => "GLACIER_IR" , } } } impl TryFrom < & str > for TransitionStorageClass { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "GLACIER" => Ok (Self :: GLACIER) , "STANDARD_IA" => Ok (Self :: STANDARD_IA) , "ONEZONE_IA" => Ok (Self :: ONEZONE_IA) , "INTELLIGENT_TIERING" => Ok (Self :: INTELLIGENT_TIERING) , "DEEP_ARCHIVE" => Ok (Self :: DEEP_ARCHIVE) , "GLACIER_IR" => Ok (Self :: GLACIER_IR) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct DeleteBucketInventoryConfiguration ; impl Op for DeleteBucketInventoryConfiguration { type Input = DeleteBucketInventoryConfigurationRequest ; type Output = () ; }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum DeleteMarkerReplicationStatus { Enabled , Disabled , } impl AsRef < str > for DeleteMarkerReplicationStatus { fn as_ref (& self) -> & str { match self { Self :: Enabled => "Enabled" , Self :: Disabled => "Disabled" , } } } impl TryFrom < & str > for DeleteMarkerReplicationStatus { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "Enabled" => Ok (Self :: Enabled) , "Disabled" => Ok (Self :: Disabled) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

# [derive (Debug , Clone)] pub struct GetBucketAccelerateConfigurationRequest { pub expected_bucket_owner : Option < AccountId > , pub bucket : Option < BucketName > , }

pub type KeyPrefixEquals = String ;

# [derive (Debug , Clone)] pub struct LifecycleExpiration { pub date : Option < Date > , pub days : Option < Days > , pub expired_object_delete_marker : Option < ExpiredObjectDeleteMarker > , }

# [derive (Debug , Clone)] pub struct MultipartUpload { pub initiator : Option < Initiator > , pub owner : Option < Owner > , pub storage_class : Option < StorageClass > , pub key : Option < ObjectKey > , pub initiated : Option < Initiated > , pub checksum_algorithm : Option < ChecksumAlgorithm > , pub upload_id : Option < MultipartUploadId > , }

# [derive (Debug , Clone)] pub struct OwnershipControlsRule { pub object_ownership : Option < ObjectOwnership > , }

pub type Setting = bool ;

# [derive (Debug , Clone)] pub struct CreateBucketOutput { pub location : Option < Location > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct DeleteObjects ; impl Op for DeleteObjects { type Input = DeleteObjectsRequest ; type Output = DeleteObjectsOutput ; }

# [derive (Debug , Clone)] pub struct BucketLoggingStatus { pub logging_enabled : Option < LoggingEnabled > , }

pub type FetchOwner = bool ;

# [derive (Debug , Clone)] pub struct ListObjectsOutput { pub contents : Option < ObjectList > , pub delimiter : Option < Delimiter > , pub encoding_type : Option < EncodingType > , pub prefix : Option < Prefix > , pub common_prefixes : Option < CommonPrefixList > , pub max_keys : Option < MaxKeys > , pub next_marker : Option < NextMarker > , pub marker : Option < Marker > , pub name : Option < BucketName > , pub is_truncated : Option < IsTruncated > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct ListParts ; impl Op for ListParts { type Input = ListPartsRequest ; type Output = ListPartsOutput ; }

pub type Initiated = String ;

# [derive (Debug , Clone)] pub struct ReplicaModifications { pub status : Option < ReplicaModificationsStatus > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum InventoryFormat { CSV , ORC , Parquet , } impl AsRef < str > for InventoryFormat { fn as_ref (& self) -> & str { match self { Self :: CSV => "CSV" , Self :: ORC => "ORC" , Self :: Parquet => "Parquet" , } } } impl TryFrom < & str > for InventoryFormat { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "CSV" => Ok (Self :: CSV) , "ORC" => Ok (Self :: ORC) , "Parquet" => Ok (Self :: Parquet) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum BucketVersioningStatus { Enabled , Suspended , } impl AsRef < str > for BucketVersioningStatus { fn as_ref (& self) -> & str { match self { Self :: Enabled => "Enabled" , Self :: Suspended => "Suspended" , } } } impl TryFrom < & str > for BucketVersioningStatus { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "Enabled" => Ok (Self :: Enabled) , "Suspended" => Ok (Self :: Suspended) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct PutBucketInventoryConfiguration ; impl Op for PutBucketInventoryConfiguration { type Input = PutBucketInventoryConfigurationRequest ; type Output = () ; }

# [derive (Debug , Clone)] pub struct GetBucketVersioningRequest { pub expected_bucket_owner : Option < AccountId > , pub bucket : Option < BucketName > , }

# [derive (Debug , Clone)] pub struct RedirectAllRequestsTo { pub protocol : Option < Protocol > , pub host_name : Option < HostName > , }

# [derive (Debug , Clone)] pub struct RequestProgress { pub enabled : Option < EnableRequestProgress > , }

pub type CopySourceSseCustomerKey = String ;

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct GetBucketTagging ; impl Op for GetBucketTagging { type Input = GetBucketTaggingRequest ; type Output = GetBucketTaggingOutput ; }

# [derive (Debug , Clone)] pub struct SseKmsEncryptedObjects { pub status : Option < SseKmsEncryptedObjectsStatus > , }

pub type End = i64 ;

# [derive (Debug , Clone)] pub struct ObjectAlreadyInActiveTierError { }

# [derive (Debug , Clone)] pub struct DefaultRetention { pub years : Option < Years > , pub days : Option < Days > , pub mode : Option < ObjectLockRetentionMode > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct DeleteBucketIntelligentTieringConfiguration ; impl Op for DeleteBucketIntelligentTieringConfiguration { type Input = DeleteBucketIntelligentTieringConfigurationRequest ; type Output = () ; }

pub type InventoryOptionalFields = Vec < InventoryOptionalField > ;

# [derive (Debug , Clone)] pub struct PublicAccessBlockConfiguration { pub block_public_acls : Option < Setting > , pub block_public_policy : Option < Setting > , pub restrict_public_buckets : Option < Setting > , pub ignore_public_acls : Option < Setting > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct PutObjectRetention ; impl Op for PutObjectRetention { type Input = PutObjectRetentionRequest ; type Output = PutObjectRetentionOutput ; }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct PutBucketVersioning ; impl Op for PutBucketVersioning { type Input = PutBucketVersioningRequest ; type Output = () ; }

pub type Priority = i32 ;

# [derive (Debug , Clone)] pub struct GetBucketLifecycleConfigurationOutput { pub rules : Option < LifecycleRules > , }

pub type IsEnabled = bool ;

pub type HostName = String ;

# [derive (Debug , Clone)] pub struct ObjectLockRule { pub default_retention : Option < DefaultRetention > , }

# [derive (Debug , Clone)] pub struct GetBucketLocationOutput { pub location_constraint : Option < BucketLocationConstraint > , }

pub type Restore = String ;

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum ChecksumAlgorithm { CRC32 , CRC32C , SHA1 , SHA256 , } impl AsRef < str > for ChecksumAlgorithm { fn as_ref (& self) -> & str { match self { Self :: CRC32 => "CRC32" , Self :: CRC32C => "CRC32C" , Self :: SHA1 => "SHA1" , Self :: SHA256 => "SHA256" , } } } impl TryFrom < & str > for ChecksumAlgorithm { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "CRC32" => Ok (Self :: CRC32) , "CRC32C" => Ok (Self :: CRC32C) , "SHA1" => Ok (Self :: SHA1) , "SHA256" => Ok (Self :: SHA256) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub type HttpRedirectCode = String ;

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct PutBucketRequestPayment ; impl Op for PutBucketRequestPayment { type Input = PutBucketRequestPaymentRequest ; type Output = () ; }

# [derive (Debug , Clone)] pub struct DeleteBucketEncryptionRequest { pub bucket : Option < BucketName > , pub expected_bucket_owner : Option < AccountId > , }

# [derive (Debug , Clone)] pub struct GetPublicAccessBlockOutput { pub public_access_block_configuration : Option < PublicAccessBlockConfiguration > , }

# [derive (Debug , Clone)] pub struct PutBucketWebsiteRequest { pub expected_bucket_owner : Option < AccountId > , pub website_configuration : Option < WebsiteConfiguration > , pub bucket : Option < BucketName > , pub checksum_algorithm : Option < ChecksumAlgorithm > , pub content_md5 : Option < ContentMd5 > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum ServerSideEncryption { AES256 , aws_kms , } impl AsRef < str > for ServerSideEncryption { fn as_ref (& self) -> & str { match self { Self :: AES256 => "AES256" , Self :: aws_kms => "aws:kms" , } } } impl TryFrom < & str > for ServerSideEncryption { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "AES256" => Ok (Self :: AES256) , "aws:kms" => Ok (Self :: aws_kms) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

# [derive (Debug , Clone)] pub struct UploadPartCopyRequest { pub copy_source : Option < CopySource > , pub request_payer : Option < RequestPayer > , pub part_number : Option < PartNumber > , pub copy_source_range : Option < CopySourceRange > , pub copy_source_sse_customer_key_md5 : Option < CopySourceSseCustomerKeyMd5 > , pub sse_customer_key : Option < SseCustomerKey > , pub copy_source_if_modified_since : Option < CopySourceIfModifiedSince > , pub copy_source_if_match : Option < CopySourceIfMatch > , pub copy_source_sse_customer_algorithm : Option < CopySourceSseCustomerAlgorithm > , pub bucket : Option < BucketName > , pub upload_id : Option < MultipartUploadId > , pub copy_source_if_none_match : Option < CopySourceIfNoneMatch > , pub copy_source_if_unmodified_since : Option < CopySourceIfUnmodifiedSince > , pub expected_bucket_owner : Option < AccountId > , pub copy_source_sse_customer_key : Option < CopySourceSseCustomerKey > , pub expected_source_bucket_owner : Option < AccountId > , pub key : Option < ObjectKey > , pub sse_customer_algorithm : Option < SseCustomerAlgorithm > , pub sse_customer_key_md5 : Option < SseCustomerKeyMd5 > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum InventoryIncludedObjectVersions { All , Current , } impl AsRef < str > for InventoryIncludedObjectVersions { fn as_ref (& self) -> & str { match self { Self :: All => "All" , Self :: Current => "Current" , } } } impl TryFrom < & str > for InventoryIncludedObjectVersions { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "All" => Ok (Self :: All) , "Current" => Ok (Self :: Current) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum ObjectAttributes { ETAG , CHECKSUM , OBJECT_PARTS , STORAGE_CLASS , OBJECT_SIZE , } impl AsRef < str > for ObjectAttributes { fn as_ref (& self) -> & str { match self { Self :: ETAG => "ETag" , Self :: CHECKSUM => "Checksum" , Self :: OBJECT_PARTS => "ObjectParts" , Self :: STORAGE_CLASS => "StorageClass" , Self :: OBJECT_SIZE => "ObjectSize" , } } } impl TryFrom < & str > for ObjectAttributes { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "ETag" => Ok (Self :: ETAG) , "Checksum" => Ok (Self :: CHECKSUM) , "ObjectParts" => Ok (Self :: OBJECT_PARTS) , "StorageClass" => Ok (Self :: STORAGE_CLASS) , "ObjectSize" => Ok (Self :: OBJECT_SIZE) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

# [derive (Debug , Clone)] pub struct PutObjectAclRequest { pub access_control_policy : Option < AccessControlPolicy > , pub content_md5 : Option < ContentMd5 > , pub expected_bucket_owner : Option < AccountId > , pub grant_write : Option < GrantWrite > , pub checksum_algorithm : Option < ChecksumAlgorithm > , pub acl : Option < ObjectCannedAcl > , pub grant_write_acp : Option < GrantWriteAcp > , pub request_payer : Option < RequestPayer > , pub grant_full_control : Option < GrantFullControl > , pub grant_read : Option < GrantRead > , pub grant_read_acp : Option < GrantReadAcp > , pub key : Option < ObjectKey > , pub version_id : Option < ObjectVersionId > , pub bucket : Option < BucketName > , }

# [derive (Debug , Clone)] pub struct ReplicationRuleAndOperator { pub prefix : Option < Prefix > , pub tags : Option < TagSet > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct GetBucketPolicyStatus ; impl Op for GetBucketPolicyStatus { type Input = GetBucketPolicyStatusRequest ; type Output = GetBucketPolicyStatusOutput ; }

# [derive (Debug , Clone)] pub struct Checksum { pub checksum_sha256 : Option < ChecksumSha256 > , pub checksum_crc32_c : Option < ChecksumCrc32c > , pub checksum_crc32 : Option < ChecksumCrc32 > , pub checksum_sha1 : Option < ChecksumSha1 > , }

# [derive (Debug , Clone)] pub struct GetBucketPolicyStatusRequest { pub expected_bucket_owner : Option < AccountId > , pub bucket : Option < BucketName > , }

# [derive (Debug , Clone)] pub struct HeadBucketRequest { pub bucket : Option < BucketName > , pub expected_bucket_owner : Option < AccountId > , }

pub type NextVersionIdMarker = String ;

pub type InventoryId = String ;

pub type Token = String ;

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum OwnerOverride { Destination , } impl AsRef < str > for OwnerOverride { fn as_ref (& self) -> & str { match self { Self :: Destination => "Destination" , } } } impl TryFrom < & str > for OwnerOverride { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "Destination" => Ok (Self :: Destination) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct PutBucketLifecycleConfiguration ; impl Op for PutBucketLifecycleConfiguration { type Input = PutBucketLifecycleConfigurationRequest ; type Output = () ; }

# [derive (Debug , Clone)] pub struct EndEvent { }

pub type IsTruncated = bool ;

pub type AllowedMethod = String ;

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct DeleteBucketOwnershipControls ; impl Op for DeleteBucketOwnershipControls { type Input = DeleteBucketOwnershipControlsRequest ; type Output = () ; }

pub type ETag = String ;

# [derive (Debug , Clone)] pub struct GetBucketEncryptionOutput { pub server_side_encryption_configuration : Option < ServerSideEncryptionConfiguration > , }

# [derive (Debug , Clone)] pub struct PutObjectLegalHoldOutput { pub request_charged : Option < RequestCharged > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum RestoreRequestType { SELECT , } impl AsRef < str > for RestoreRequestType { fn as_ref (& self) -> & str { match self { Self :: SELECT => "SELECT" , } } } impl TryFrom < & str > for RestoreRequestType { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "SELECT" => Ok (Self :: SELECT) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub type MetricsConfigurationList = Vec < MetricsConfiguration > ;

# [derive (Debug , Clone)] pub struct DeleteObjectsRequest { pub checksum_algorithm : Option < ChecksumAlgorithm > , pub delete : Option < Delete > , pub expected_bucket_owner : Option < AccountId > , pub mfa : Option < Mfa > , pub request_payer : Option < RequestPayer > , pub bucket : Option < BucketName > , pub bypass_governance_retention : Option < BypassGovernanceRetention > , }

pub type TargetGrants = Vec < TargetGrant > ;

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct GetBucketReplication ; impl Op for GetBucketReplication { type Input = GetBucketReplicationRequest ; type Output = GetBucketReplicationOutput ; }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct GetBucketLifecycleConfiguration ; impl Op for GetBucketLifecycleConfiguration { type Input = GetBucketLifecycleConfigurationRequest ; type Output = GetBucketLifecycleConfigurationOutput ; }

pub type MissingMeta = i32 ;

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct AbortMultipartUpload ; impl Op for AbortMultipartUpload { type Input = AbortMultipartUploadRequest ; type Output = AbortMultipartUploadOutput ; }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum BucketLogsPermission { FULL_CONTROL , READ , WRITE , } impl AsRef < str > for BucketLogsPermission { fn as_ref (& self) -> & str { match self { Self :: FULL_CONTROL => "FULL_CONTROL" , Self :: READ => "READ" , Self :: WRITE => "WRITE" , } } } impl TryFrom < & str > for BucketLogsPermission { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "FULL_CONTROL" => Ok (Self :: FULL_CONTROL) , "READ" => Ok (Self :: READ) , "WRITE" => Ok (Self :: WRITE) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub type BytesScanned = i64 ;

# [derive (Debug , Clone)] pub enum SelectObjectContentEventStream { Cont (ContinuationEvent) , Stats (StatsEvent) , Records (RecordsEvent) , End (EndEvent) , Progress (ProgressEvent) , }

pub type NextKeyMarker = String ;

# [derive (Debug , Clone)] pub struct GetBucketPolicyStatusOutput { pub policy_status : Option < PolicyStatus > , }

# [derive (Debug , Clone)] pub struct NotificationConfiguration { pub topic_configurations : Option < TopicConfigurationList > , pub lambda_function_configurations : Option < LambdaFunctionConfigurationList > , pub queue_configurations : Option < QueueConfigurationList > , pub event_bridge_configuration : Option < EventBridgeConfiguration > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum ReplicationStatus { COMPLETE , PENDING , FAILED , REPLICA , } impl AsRef < str > for ReplicationStatus { fn as_ref (& self) -> & str { match self { Self :: COMPLETE => "COMPLETE" , Self :: PENDING => "PENDING" , Self :: FAILED => "FAILED" , Self :: REPLICA => "REPLICA" , } } } impl TryFrom < & str > for ReplicationStatus { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "COMPLETE" => Ok (Self :: COMPLETE) , "PENDING" => Ok (Self :: PENDING) , "FAILED" => Ok (Self :: FAILED) , "REPLICA" => Ok (Self :: REPLICA) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

# [derive (Debug , Clone)] pub struct Tiering { pub days : Option < IntelligentTieringDays > , pub access_tier : Option < IntelligentTieringAccessTier > , }

pub type CopySourceRange = String ;

pub type ContentEncoding = String ;

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct GetObjectRetention ; impl Op for GetObjectRetention { type Input = GetObjectRetentionRequest ; type Output = GetObjectRetentionOutput ; }

# [derive (Debug , Clone)] pub struct HeadObjectRequest { pub part_number : Option < PartNumber > , pub sse_customer_algorithm : Option < SseCustomerAlgorithm > , pub sse_customer_key : Option < SseCustomerKey > , pub sse_customer_key_md5 : Option < SseCustomerKeyMd5 > , pub checksum_mode : Option < ChecksumMode > , pub range : Option < Range > , pub request_payer : Option < RequestPayer > , pub expected_bucket_owner : Option < AccountId > , pub version_id : Option < ObjectVersionId > , pub key : Option < ObjectKey > , pub bucket : Option < BucketName > , pub if_match : Option < IfMatch > , pub if_modified_since : Option < IfModifiedSince > , pub if_none_match : Option < IfNoneMatch > , pub if_unmodified_since : Option < IfUnmodifiedSince > , }

# [derive (Debug , Clone)] pub struct IntelligentTieringConfiguration { pub filter : Option < IntelligentTieringFilter > , pub tierings : Option < TieringList > , pub id : Option < IntelligentTieringId > , pub status : Option < IntelligentTieringStatus > , }

# [derive (Debug , Clone)] pub struct GetBucketPolicyRequest { pub expected_bucket_owner : Option < AccountId > , pub bucket : Option < BucketName > , }

# [derive (Debug , Clone)] pub struct CopyPartResult { pub checksum_sha1 : Option < ChecksumSha1 > , pub checksum_crc32 : Option < ChecksumCrc32 > , pub checksum_sha256 : Option < ChecksumSha256 > , pub e_tag : Option < ETag > , pub checksum_crc32_c : Option < ChecksumCrc32c > , pub last_modified : Option < LastModified > , }

pub type ObjectKey = String ;

pub type ObjectLockRetainUntilDate = String ;

pub type AllowedOrigin = String ;

# [derive (Debug , Clone)] pub struct LambdaFunctionConfiguration { pub events : Option < EventList > , pub filter : Option < NotificationConfigurationFilter > , pub lambda_function_arn : Option < LambdaFunctionArn > , pub id : Option < NotificationId > , }

pub type SseCustomerKey = String ;

# [derive (Debug , Clone)] pub struct PutObjectRetentionRequest { pub expected_bucket_owner : Option < AccountId > , pub version_id : Option < ObjectVersionId > , pub bucket : Option < BucketName > , pub content_md5 : Option < ContentMd5 > , pub bypass_governance_retention : Option < BypassGovernanceRetention > , pub request_payer : Option < RequestPayer > , pub checksum_algorithm : Option < ChecksumAlgorithm > , pub retention : Option < ObjectLockRetention > , pub key : Option < ObjectKey > , }

# [derive (Debug , Clone)] pub struct CompleteMultipartUploadRequest { pub request_payer : Option < RequestPayer > , pub expected_bucket_owner : Option < AccountId > , pub checksum_sha1 : Option < ChecksumSha1 > , pub checksum_sha256 : Option < ChecksumSha256 > , pub checksum_crc32_c : Option < ChecksumCrc32c > , pub sse_customer_algorithm : Option < SseCustomerAlgorithm > , pub checksum_crc32 : Option < ChecksumCrc32 > , pub multipart_upload : Option < CompletedMultipartUpload > , pub sse_customer_key : Option < SseCustomerKey > , pub key : Option < ObjectKey > , pub sse_customer_key_md5 : Option < SseCustomerKeyMd5 > , pub bucket : Option < BucketName > , pub upload_id : Option < MultipartUploadId > , }

pub type Buckets = Vec < Bucket > ;

pub type Expires = String ;

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct GetBucketVersioning ; impl Op for GetBucketVersioning { type Input = GetBucketVersioningRequest ; type Output = GetBucketVersioningOutput ; }

# [derive (Debug , Clone)] pub struct ObjectNotInActiveTierError { }

pub type Policy = String ;

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct PutBucketNotificationConfiguration ; impl Op for PutBucketNotificationConfiguration { type Input = PutBucketNotificationConfigurationRequest ; type Output = () ; }

# [derive (Debug , Clone)] pub struct TopicConfiguration { pub events : Option < EventList > , pub topic_arn : Option < TopicArn > , pub filter : Option < NotificationConfigurationFilter > , pub id : Option < NotificationId > , }

pub type UploadIdMarker = String ;

pub type IntelligentTieringDays = i32 ;

# [derive (Debug , Clone)] pub struct RestoreObjectRequest { pub bucket : Option < BucketName > , pub checksum_algorithm : Option < ChecksumAlgorithm > , pub expected_bucket_owner : Option < AccountId > , pub key : Option < ObjectKey > , pub request_payer : Option < RequestPayer > , pub restore_request : Option < RestoreRequest > , pub version_id : Option < ObjectVersionId > , }

# [derive (Debug , Clone)] pub struct PutBucketCorsRequest { pub checksum_algorithm : Option < ChecksumAlgorithm > , pub cors_configuration : Option < CorsConfiguration > , pub content_md5 : Option < ContentMd5 > , pub bucket : Option < BucketName > , pub expected_bucket_owner : Option < AccountId > , }

pub type GrantFullControl = String ;

pub type AllowedOrigins = Vec < AllowedOrigin > ;

pub type LambdaFunctionConfigurationList = Vec < LambdaFunctionConfiguration > ;

# [derive (Debug , Clone)] pub struct InventoryConfiguration { pub schedule : Option < InventorySchedule > , pub optional_fields : Option < InventoryOptionalFields > , pub filter : Option < InventoryFilter > , pub id : Option < InventoryId > , pub destination : Option < InventoryDestination > , pub is_enabled : Option < IsEnabled > , pub included_object_versions : Option < InventoryIncludedObjectVersions > , }

# [derive (Debug , Clone)] pub struct ObjectLockRetention { pub mode : Option < ObjectLockRetentionMode > , pub retain_until_date : Option < Date > , }

pub type TieringList = Vec < Tiering > ;

pub type AllowedHeaders = Vec < AllowedHeader > ;

# [derive (Debug , Clone)] pub struct NotFound { }

# [derive (Debug , Clone)] pub struct PutObjectOutput { pub request_charged : Option < RequestCharged > , pub bucket_key_enabled : Option < BucketKeyEnabled > , pub checksum_crc32_c : Option < ChecksumCrc32c > , pub server_side_encryption : Option < ServerSideEncryption > , pub version_id : Option < ObjectVersionId > , pub ssekms_encryption_context : Option < SsekmsEncryptionContext > , pub checksum_crc32 : Option < ChecksumCrc32 > , pub checksum_sha256 : Option < ChecksumSha256 > , pub expiration : Option < Expiration > , pub ssekms_key_id : Option < SsekmsKeyId > , pub sse_customer_key_md5 : Option < SseCustomerKeyMd5 > , pub e_tag : Option < ETag > , pub checksum_sha1 : Option < ChecksumSha1 > , pub sse_customer_algorithm : Option < SseCustomerAlgorithm > , }

# [derive (Debug , Clone)] pub struct JsonInput { pub r#type : Option < JsonType > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct ListBucketIntelligentTieringConfigurations ; impl Op for ListBucketIntelligentTieringConfigurations { type Input = ListBucketIntelligentTieringConfigurationsRequest ; type Output = ListBucketIntelligentTieringConfigurationsOutput ; }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum FileHeaderInfo { USE , IGNORE , NONE , } impl AsRef < str > for FileHeaderInfo { fn as_ref (& self) -> & str { match self { Self :: USE => "USE" , Self :: IGNORE => "IGNORE" , Self :: NONE => "NONE" , } } } impl TryFrom < & str > for FileHeaderInfo { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "USE" => Ok (Self :: USE) , "IGNORE" => Ok (Self :: IGNORE) , "NONE" => Ok (Self :: NONE) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

# [derive (Debug , Clone)] pub struct BucketAlreadyExists { }

# [derive (Debug , Clone)] pub struct GetObjectRequest { pub response_content_language : Option < ResponseContentLanguage > , pub if_modified_since : Option < IfModifiedSince > , pub sse_customer_algorithm : Option < SseCustomerAlgorithm > , pub sse_customer_key_md5 : Option < SseCustomerKeyMd5 > , pub range : Option < Range > , pub if_unmodified_since : Option < IfUnmodifiedSince > , pub if_match : Option < IfMatch > , pub key : Option < ObjectKey > , pub request_payer : Option < RequestPayer > , pub if_none_match : Option < IfNoneMatch > , pub response_content_disposition : Option < ResponseContentDisposition > , pub response_content_type : Option < ResponseContentType > , pub part_number : Option < PartNumber > , pub response_expires : Option < ResponseExpires > , pub sse_customer_key : Option < SseCustomerKey > , pub checksum_mode : Option < ChecksumMode > , pub expected_bucket_owner : Option < AccountId > , pub version_id : Option < ObjectVersionId > , pub bucket : Option < BucketName > , pub response_cache_control : Option < ResponseCacheControl > , pub response_content_encoding : Option < ResponseContentEncoding > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum ObjectLockMode { GOVERNANCE , COMPLIANCE , } impl AsRef < str > for ObjectLockMode { fn as_ref (& self) -> & str { match self { Self :: GOVERNANCE => "GOVERNANCE" , Self :: COMPLIANCE => "COMPLIANCE" , } } } impl TryFrom < & str > for ObjectLockMode { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "GOVERNANCE" => Ok (Self :: GOVERNANCE) , "COMPLIANCE" => Ok (Self :: COMPLIANCE) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub type AllowedHeader = String ;

# [derive (Debug , Clone)] pub struct DeleteBucketRequest { pub bucket : Option < BucketName > , pub expected_bucket_owner : Option < AccountId > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct GetBucketRequestPayment ; impl Op for GetBucketRequestPayment { type Input = GetBucketRequestPaymentRequest ; type Output = GetBucketRequestPaymentOutput ; }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct DeleteBucketReplication ; impl Op for DeleteBucketReplication { type Input = DeleteBucketReplicationRequest ; type Output = () ; }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum RequestCharged { requester , } impl AsRef < str > for RequestCharged { fn as_ref (& self) -> & str { match self { Self :: requester => "requester" , } } } impl TryFrom < & str > for RequestCharged { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "requester" => Ok (Self :: requester) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

# [derive (Debug , Clone)] pub struct RestoreObjectOutput { pub request_charged : Option < RequestCharged > , pub restore_output_path : Option < RestoreOutputPath > , }

# [derive (Debug , Clone)] pub struct QueueConfiguration { pub filter : Option < NotificationConfigurationFilter > , pub queue_arn : Option < QueueArn > , pub events : Option < EventList > , pub id : Option < NotificationId > , }

# [derive (Debug , Clone)] pub struct PutObjectRetentionOutput { pub request_charged : Option < RequestCharged > , }

pub type KeyCount = i32 ;

# [derive (Debug , Clone)] pub struct PutBucketNotificationConfigurationRequest { pub bucket : Option < BucketName > , pub expected_bucket_owner : Option < AccountId > , pub notification_configuration : Option < NotificationConfiguration > , pub skip_destination_validation : Option < SkipValidation > , }

# [derive (Debug , Clone)] pub struct AccelerateConfiguration { pub status : Option < BucketAccelerateStatus > , }

# [derive (Debug , Clone)] pub struct IntelligentTieringFilter { pub and : Option < IntelligentTieringAndOperator > , pub prefix : Option < Prefix > , pub tag : Option < Tag > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct DeleteBucketEncryption ; impl Op for DeleteBucketEncryption { type Input = DeleteBucketEncryptionRequest ; type Output = () ; }

pub type GrantWrite = String ;

# [derive (Debug , Clone)] pub struct GetBucketRequestPaymentOutput { pub payer : Option < Payer > , }

pub type EnableRequestProgress = bool ;

pub type IfModifiedSince = String ;

# [derive (Debug , Clone)] pub struct ListBucketAnalyticsConfigurationsRequest { pub expected_bucket_owner : Option < AccountId > , pub bucket : Option < BucketName > , pub continuation_token : Option < Token > , }

# [derive (Debug , Clone)] pub enum AnalyticsFilter { Tag (Tag) , Prefix (Prefix) , And (AnalyticsAndOperator) , }

pub type MaxKeys = i32 ;

# [derive (Debug , Clone)] pub struct DeleteBucketLifecycleRequest { pub expected_bucket_owner : Option < AccountId > , pub bucket : Option < BucketName > , }

# [derive (Debug , Clone)] pub struct DeleteBucketOwnershipControlsRequest { pub expected_bucket_owner : Option < AccountId > , pub bucket : Option < BucketName > , }

# [derive (Debug , Clone)] pub struct NoncurrentVersionTransition { pub newer_noncurrent_versions : Option < VersionCount > , pub storage_class : Option < TransitionStorageClass > , pub noncurrent_days : Option < Days > , }

pub type ObjectSizeLessThanBytes = i64 ;

# [derive (Debug , Clone)] pub struct GetBucketRequestPaymentRequest { pub expected_bucket_owner : Option < AccountId > , pub bucket : Option < BucketName > , }

# [derive (Debug , Clone)] pub struct Owner { pub id : Option < Id > , pub display_name : Option < DisplayName > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct PutObjectLegalHold ; impl Op for PutObjectLegalHold { type Input = PutObjectLegalHoldRequest ; type Output = PutObjectLegalHoldOutput ; }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct PutPublicAccessBlock ; impl Op for PutPublicAccessBlock { type Input = PutPublicAccessBlockRequest ; type Output = () ; }

# [derive (Debug , Clone)] pub struct GetBucketCorsRequest { pub expected_bucket_owner : Option < AccountId > , pub bucket : Option < BucketName > , }

# [derive (Debug , Clone)] pub struct Initiator { pub display_name : Option < DisplayName > , pub id : Option < Id > , }

pub type Errors = Vec < Error > ;

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct PutBucketTagging ; impl Op for PutBucketTagging { type Input = PutBucketTaggingRequest ; type Output = () ; }

# [derive (Debug , Clone)] pub struct WriteGetObjectResponseRequest { pub e_tag : Option < ETag > , pub error_message : Option < ErrorMessage > , pub checksum_sha256 : Option < ChecksumSha256 > , pub content_length : Option < ContentLength > , pub delete_marker : Option < DeleteMarker > , pub object_lock_legal_hold_status : Option < ObjectLockLegalHoldStatus > , pub expires : Option < Expires > , pub object_lock_mode : Option < ObjectLockMode > , pub error_code : Option < ErrorCode > , pub missing_meta : Option < MissingMeta > , pub request_charged : Option < RequestCharged > , pub restore : Option < Restore > , pub last_modified : Option < LastModified > , pub cache_control : Option < CacheControl > , pub content_encoding : Option < ContentEncoding > , pub accept_ranges : Option < AcceptRanges > , pub request_token : Option < RequestToken > , pub bucket_key_enabled : Option < BucketKeyEnabled > , pub server_side_encryption : Option < ServerSideEncryption > , pub sse_customer_algorithm : Option < SseCustomerAlgorithm > , pub checksum_crc32 : Option < ChecksumCrc32 > , pub checksum_sha1 : Option < ChecksumSha1 > , pub status_code : Option < GetObjectResponseStatusCode > , pub content_range : Option < ContentRange > , pub sse_customer_key_md5 : Option < SseCustomerKeyMd5 > , pub tag_count : Option < TagCount > , pub replication_status : Option < ReplicationStatus > , pub parts_count : Option < PartsCount > , pub object_lock_retain_until_date : Option < ObjectLockRetainUntilDate > , pub ssekms_key_id : Option < SsekmsKeyId > , pub version_id : Option < ObjectVersionId > , pub body : Option < StreamingBlob > , pub content_language : Option < ContentLanguage > , pub content_type : Option < ContentType > , pub metadata : Option < Metadata > , pub checksum_crc32_c : Option < ChecksumCrc32c > , pub content_disposition : Option < ContentDisposition > , pub expiration : Option < Expiration > , pub storage_class : Option < StorageClass > , pub request_route : Option < RequestRoute > , }

pub type GrantWriteAcp = String ;

pub type Prefix = String ;

pub type QueueConfigurationList = Vec < QueueConfiguration > ;

pub type SkipValidation = bool ;

# [derive (Debug , Clone)] pub struct StorageClassAnalysis { pub data_export : Option < StorageClassAnalysisDataExport > , }

# [derive (Debug , Clone)] pub struct ErrorDocument { pub key : Option < ObjectKey > , }

pub type StreamingBlob = Arc < hyper :: Body > ;

# [derive (Debug , Clone)] pub struct NoSuchBucket { }

# [derive (Debug , Clone)] pub struct GetObjectLockConfigurationOutput { pub object_lock_configuration : Option < ObjectLockConfiguration > , }

# [derive (Debug , Clone)] pub struct ContinuationEvent { }

# [derive (Debug , Clone)] pub struct Transition { pub storage_class : Option < TransitionStorageClass > , pub days : Option < Days > , pub date : Option < Date > , }

pub type CopySourceSseCustomerAlgorithm = String ;

# [derive (Debug , Clone)] pub struct PutObjectRequest { pub sse_customer_key_md5 : Option < SseCustomerKeyMd5 > , pub ssekms_key_id : Option < SsekmsKeyId > , pub content_language : Option < ContentLanguage > , pub storage_class : Option < StorageClass > , pub grant_write_acp : Option < GrantWriteAcp > , pub ssekms_encryption_context : Option < SsekmsEncryptionContext > , pub bucket_key_enabled : Option < BucketKeyEnabled > , pub tagging : Option < TaggingHeader > , pub checksum_sha1 : Option < ChecksumSha1 > , pub checksum_sha256 : Option < ChecksumSha256 > , pub content_encoding : Option < ContentEncoding > , pub object_lock_legal_hold_status : Option < ObjectLockLegalHoldStatus > , pub metadata : Option < Metadata > , pub sse_customer_algorithm : Option < SseCustomerAlgorithm > , pub content_type : Option < ContentType > , pub grant_full_control : Option < GrantFullControl > , pub checksum_crc32_c : Option < ChecksumCrc32c > , pub checksum_algorithm : Option < ChecksumAlgorithm > , pub grant_read_acp : Option < GrantReadAcp > , pub key : Option < ObjectKey > , pub request_payer : Option < RequestPayer > , pub checksum_crc32 : Option < ChecksumCrc32 > , pub server_side_encryption : Option < ServerSideEncryption > , pub website_redirect_location : Option < WebsiteRedirectLocation > , pub content_length : Option < ContentLength > , pub grant_read : Option < GrantRead > , pub bucket : Option < BucketName > , pub cache_control : Option < CacheControl > , pub object_lock_mode : Option < ObjectLockMode > , pub body : Option < StreamingBlob > , pub object_lock_retain_until_date : Option < ObjectLockRetainUntilDate > , pub acl : Option < ObjectCannedAcl > , pub expected_bucket_owner : Option < AccountId > , pub expires : Option < Expires > , pub content_md5 : Option < ContentMd5 > , pub sse_customer_key : Option < SseCustomerKey > , pub content_disposition : Option < ContentDisposition > , }

# [derive (Debug , Clone)] pub struct GetBucketAclOutput { pub grants : Option < Grants > , pub owner : Option < Owner > , }

# [derive (Debug , Clone)] pub struct CopyObjectResult { pub checksum_crc32 : Option < ChecksumCrc32 > , pub e_tag : Option < ETag > , pub last_modified : Option < LastModified > , pub checksum_crc32_c : Option < ChecksumCrc32c > , pub checksum_sha256 : Option < ChecksumSha256 > , pub checksum_sha1 : Option < ChecksumSha1 > , }

# [derive (Debug , Clone)] pub struct PutBucketLifecycleConfigurationRequest { pub expected_bucket_owner : Option < AccountId > , pub bucket : Option < BucketName > , pub lifecycle_configuration : Option < BucketLifecycleConfiguration > , pub checksum_algorithm : Option < ChecksumAlgorithm > , }

pub type NextToken = String ;

pub type NotificationId = String ;

# [derive (Debug , Clone)] pub struct ProgressEvent { pub details : Option < Progress > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum Payer { Requester , BucketOwner , } impl AsRef < str > for Payer { fn as_ref (& self) -> & str { match self { Self :: Requester => "Requester" , Self :: BucketOwner => "BucketOwner" , } } } impl TryFrom < & str > for Payer { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "Requester" => Ok (Self :: Requester) , "BucketOwner" => Ok (Self :: BucketOwner) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub type NextUploadIdMarker = String ;

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct PutBucketWebsite ; impl Op for PutBucketWebsite { type Input = PutBucketWebsiteRequest ; type Output = () ; }

# [derive (Debug , Clone)] pub struct Tag { pub value : Option < Value > , pub key : Option < ObjectKey > , }

# [derive (Debug , Clone)] pub struct InputSerialization { pub json : Option < JsonInput > , pub csv : Option < CsvInput > , pub parquet : Option < ParquetInput > , pub compression_type : Option < CompressionType > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct UploadPartCopy ; impl Op for UploadPartCopy { type Input = UploadPartCopyRequest ; type Output = UploadPartCopyOutput ; }

# [derive (Debug , Clone)] pub struct VersioningConfiguration { pub mfa_delete : Option < MfaDelete > , pub status : Option < BucketVersioningStatus > , }

# [derive (Debug , Clone)] pub struct ListBucketMetricsConfigurationsRequest { pub expected_bucket_owner : Option < AccountId > , pub bucket : Option < BucketName > , pub continuation_token : Option < Token > , }

# [derive (Debug , Clone)] pub struct Part { pub checksum_sha256 : Option < ChecksumSha256 > , pub last_modified : Option < LastModified > , pub size : Option < Size > , pub checksum_crc32 : Option < ChecksumCrc32 > , pub checksum_crc32_c : Option < ChecksumCrc32c > , pub e_tag : Option < ETag > , pub part_number : Option < PartNumber > , pub checksum_sha1 : Option < ChecksumSha1 > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct PutBucketPolicy ; impl Op for PutBucketPolicy { type Input = PutBucketPolicyRequest ; type Output = () ; }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum StorageClass { STANDARD , REDUCED_REDUNDANCY , STANDARD_IA , ONEZONE_IA , INTELLIGENT_TIERING , GLACIER , DEEP_ARCHIVE , OUTPOSTS , GLACIER_IR , } impl AsRef < str > for StorageClass { fn as_ref (& self) -> & str { match self { Self :: STANDARD => "STANDARD" , Self :: REDUCED_REDUNDANCY => "REDUCED_REDUNDANCY" , Self :: STANDARD_IA => "STANDARD_IA" , Self :: ONEZONE_IA => "ONEZONE_IA" , Self :: INTELLIGENT_TIERING => "INTELLIGENT_TIERING" , Self :: GLACIER => "GLACIER" , Self :: DEEP_ARCHIVE => "DEEP_ARCHIVE" , Self :: OUTPOSTS => "OUTPOSTS" , Self :: GLACIER_IR => "GLACIER_IR" , } } } impl TryFrom < & str > for StorageClass { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "STANDARD" => Ok (Self :: STANDARD) , "REDUCED_REDUNDANCY" => Ok (Self :: REDUCED_REDUNDANCY) , "STANDARD_IA" => Ok (Self :: STANDARD_IA) , "ONEZONE_IA" => Ok (Self :: ONEZONE_IA) , "INTELLIGENT_TIERING" => Ok (Self :: INTELLIGENT_TIERING) , "GLACIER" => Ok (Self :: GLACIER) , "DEEP_ARCHIVE" => Ok (Self :: DEEP_ARCHIVE) , "OUTPOSTS" => Ok (Self :: OUTPOSTS) , "GLACIER_IR" => Ok (Self :: GLACIER_IR) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

# [derive (Debug , Clone)] pub struct Grant { pub grantee : Option < Grantee > , pub permission : Option < Permission > , }

# [derive (Debug , Clone)] pub struct StorageClassAnalysisDataExport { pub destination : Option < AnalyticsExportDestination > , pub output_schema_version : Option < StorageClassAnalysisSchemaVersion > , }

# [derive (Debug , Clone)] pub struct GlacierJobParameters { pub tier : Option < Tier > , }

# [derive (Debug , Clone)] pub struct GetBucketAclRequest { pub bucket : Option < BucketName > , pub expected_bucket_owner : Option < AccountId > , }

pub type EmailAddress = String ;

# [derive (Debug , Clone)] pub struct Error { pub version_id : Option < ObjectVersionId > , pub message : Option < Message > , pub code : Option < Code > , pub key : Option < ObjectKey > , }

pub type ObjectSize = i64 ;

pub type ReplicaKmsKeyId = String ;

# [derive (Debug , Clone)] pub struct DeleteBucketInventoryConfigurationRequest { pub bucket : Option < BucketName > , pub id : Option < InventoryId > , pub expected_bucket_owner : Option < AccountId > , }

pub type EventList = Vec < Event > ;

# [derive (Debug , Clone)] pub struct ListBucketMetricsConfigurationsOutput { pub next_continuation_token : Option < NextToken > , pub is_truncated : Option < IsTruncated > , pub continuation_token : Option < Token > , pub metrics_configuration_list : Option < MetricsConfigurationList > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum TaggingDirective { COPY , REPLACE , } impl AsRef < str > for TaggingDirective { fn as_ref (& self) -> & str { match self { Self :: COPY => "COPY" , Self :: REPLACE => "REPLACE" , } } } impl TryFrom < & str > for TaggingDirective { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "COPY" => Ok (Self :: COPY) , "REPLACE" => Ok (Self :: REPLACE) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

# [derive (Debug , Clone)] pub struct CreateBucketConfiguration { pub location_constraint : Option < BucketLocationConstraint > , }

# [derive (Debug , Clone)] pub struct Tagging { pub tag_set : Option < TagSet > , }

pub type DisplayName = String ;

pub type AbortRuleId = String ;

# [derive (Debug , Clone)] pub struct CompletedPart { pub checksum_crc32_c : Option < ChecksumCrc32c > , pub checksum_sha256 : Option < ChecksumSha256 > , pub checksum_sha1 : Option < ChecksumSha1 > , pub checksum_crc32 : Option < ChecksumCrc32 > , pub e_tag : Option < ETag > , pub part_number : Option < PartNumber > , }

pub type Message = String ;

# [derive (Debug , Clone)] pub struct CompleteMultipartUploadOutput { pub checksum_sha256 : Option < ChecksumSha256 > , pub expiration : Option < Expiration > , pub request_charged : Option < RequestCharged > , pub version_id : Option < ObjectVersionId > , pub server_side_encryption : Option < ServerSideEncryption > , pub key : Option < ObjectKey > , pub e_tag : Option < ETag > , pub ssekms_key_id : Option < SsekmsKeyId > , pub checksum_crc32_c : Option < ChecksumCrc32c > , pub checksum_sha1 : Option < ChecksumSha1 > , pub checksum_crc32 : Option < ChecksumCrc32 > , pub location : Option < Location > , pub bucket : Option < BucketName > , pub bucket_key_enabled : Option < BucketKeyEnabled > , }

pub type ContentDisposition = String ;

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum InventoryOptionalField { Size , LastModifiedDate , StorageClass , ETag , IsMultipartUploaded , ReplicationStatus , EncryptionStatus , ObjectLockRetainUntilDate , ObjectLockMode , ObjectLockLegalHoldStatus , IntelligentTieringAccessTier , BucketKeyStatus , ChecksumAlgorithm , } impl AsRef < str > for InventoryOptionalField { fn as_ref (& self) -> & str { match self { Self :: Size => "Size" , Self :: LastModifiedDate => "LastModifiedDate" , Self :: StorageClass => "StorageClass" , Self :: ETag => "ETag" , Self :: IsMultipartUploaded => "IsMultipartUploaded" , Self :: ReplicationStatus => "ReplicationStatus" , Self :: EncryptionStatus => "EncryptionStatus" , Self :: ObjectLockRetainUntilDate => "ObjectLockRetainUntilDate" , Self :: ObjectLockMode => "ObjectLockMode" , Self :: ObjectLockLegalHoldStatus => "ObjectLockLegalHoldStatus" , Self :: IntelligentTieringAccessTier => "IntelligentTieringAccessTier" , Self :: BucketKeyStatus => "BucketKeyStatus" , Self :: ChecksumAlgorithm => "ChecksumAlgorithm" , } } } impl TryFrom < & str > for InventoryOptionalField { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "Size" => Ok (Self :: Size) , "LastModifiedDate" => Ok (Self :: LastModifiedDate) , "StorageClass" => Ok (Self :: StorageClass) , "ETag" => Ok (Self :: ETag) , "IsMultipartUploaded" => Ok (Self :: IsMultipartUploaded) , "ReplicationStatus" => Ok (Self :: ReplicationStatus) , "EncryptionStatus" => Ok (Self :: EncryptionStatus) , "ObjectLockRetainUntilDate" => Ok (Self :: ObjectLockRetainUntilDate) , "ObjectLockMode" => Ok (Self :: ObjectLockMode) , "ObjectLockLegalHoldStatus" => Ok (Self :: ObjectLockLegalHoldStatus) , "IntelligentTieringAccessTier" => Ok (Self :: IntelligentTieringAccessTier) , "BucketKeyStatus" => Ok (Self :: BucketKeyStatus) , "ChecksumAlgorithm" => Ok (Self :: ChecksumAlgorithm) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub type ObjectAttributesList = Vec < ObjectAttributes > ;

# [derive (Debug , Clone)] pub struct ScanRange { pub end : Option < End > , pub start : Option < Start > , }

# [derive (Debug , Clone)] pub struct DeleteBucketReplicationRequest { pub expected_bucket_owner : Option < AccountId > , pub bucket : Option < BucketName > , }

# [derive (Debug , Clone)] pub struct MetadataEntry { pub value : Option < MetadataValue > , pub name : Option < MetadataKey > , }

# [derive (Debug , Clone)] pub struct DeleteBucketMetricsConfigurationRequest { pub bucket : Option < BucketName > , pub expected_bucket_owner : Option < AccountId > , pub id : Option < MetricsId > , }

# [derive (Debug , Clone)] pub struct InventorySchedule { pub frequency : Option < InventoryFrequency > , }

# [derive (Debug , Clone)] pub struct GetObjectTaggingRequest { pub bucket : Option < BucketName > , pub request_payer : Option < RequestPayer > , pub expected_bucket_owner : Option < AccountId > , pub version_id : Option < ObjectVersionId > , pub key : Option < ObjectKey > , }

# [derive (Debug , Clone)] pub struct GetObjectAclRequest { pub expected_bucket_owner : Option < AccountId > , pub request_payer : Option < RequestPayer > , pub key : Option < ObjectKey > , pub bucket : Option < BucketName > , pub version_id : Option < ObjectVersionId > , }

pub type ChecksumSha256 = String ;

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct GetBucketEncryption ; impl Op for GetBucketEncryption { type Input = GetBucketEncryptionRequest ; type Output = GetBucketEncryptionOutput ; }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum ObjectLockLegalHoldStatus { ON , OFF , } impl AsRef < str > for ObjectLockLegalHoldStatus { fn as_ref (& self) -> & str { match self { Self :: ON => "ON" , Self :: OFF => "OFF" , } } } impl TryFrom < & str > for ObjectLockLegalHoldStatus { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "ON" => Ok (Self :: ON) , "OFF" => Ok (Self :: OFF) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct PutBucketAccelerateConfiguration ; impl Op for PutBucketAccelerateConfiguration { type Input = PutBucketAccelerateConfigurationRequest ; type Output = () ; }

# [derive (Debug , Clone)] pub struct PutBucketEncryptionRequest { pub expected_bucket_owner : Option < AccountId > , pub bucket : Option < BucketName > , pub checksum_algorithm : Option < ChecksumAlgorithm > , pub content_md5 : Option < ContentMd5 > , pub server_side_encryption_configuration : Option < ServerSideEncryptionConfiguration > , }

pub type Years = i32 ;

# [derive (Debug , Clone)] pub struct NoSuchUpload { }

# [derive (Debug , Clone)] pub struct RestoreRequest { pub glacier_job_parameters : Option < GlacierJobParameters > , pub days : Option < Days > , pub output_location : Option < OutputLocation > , pub r#type : Option < RestoreRequestType > , pub description : Option < Description > , pub select_parameters : Option < SelectParameters > , pub tier : Option < Tier > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct GetBucketLocation ; impl Op for GetBucketLocation { type Input = GetBucketLocationRequest ; type Output = GetBucketLocationOutput ; }

# [derive (Debug , Clone)] pub struct IndexDocument { pub suffix : Option < Suffix > , }

pub type IsPublic = bool ;

# [derive (Debug , Clone)] pub struct InventoryS3BucketDestination { pub bucket : Option < BucketName > , pub prefix : Option < Prefix > , pub format : Option < InventoryFormat > , pub account_id : Option < AccountId > , pub encryption : Option < InventoryEncryption > , }

# [derive (Debug , Clone)] pub struct GetBucketReplicationOutput { pub replication_configuration : Option < ReplicationConfiguration > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct PutObjectAcl ; impl Op for PutObjectAcl { type Input = PutObjectAclRequest ; type Output = PutObjectAclOutput ; }

pub type BytesReturned = i64 ;

# [derive (Debug , Clone)] pub struct ListMultipartUploadsRequest { pub expected_bucket_owner : Option < AccountId > , pub upload_id_marker : Option < UploadIdMarker > , pub bucket : Option < BucketName > , pub max_uploads : Option < MaxUploads > , pub prefix : Option < Prefix > , pub encoding_type : Option < EncodingType > , pub key_marker : Option < KeyMarker > , pub delimiter : Option < Delimiter > , }

# [derive (Debug , Clone)] pub struct EventBridgeConfiguration { }

# [derive (Debug , Clone)] pub struct UploadPartOutput { pub checksum_sha256 : Option < ChecksumSha256 > , pub checksum_crc32_c : Option < ChecksumCrc32c > , pub request_charged : Option < RequestCharged > , pub bucket_key_enabled : Option < BucketKeyEnabled > , pub server_side_encryption : Option < ServerSideEncryption > , pub checksum_sha1 : Option < ChecksumSha1 > , pub sse_customer_algorithm : Option < SseCustomerAlgorithm > , pub sse_customer_key_md5 : Option < SseCustomerKeyMd5 > , pub e_tag : Option < ETag > , pub ssekms_key_id : Option < SsekmsKeyId > , pub checksum_crc32 : Option < ChecksumCrc32 > , }

# [derive (Debug , Clone)] pub struct SelectObjectContentRequest { pub sse_customer_key_md5 : Option < SseCustomerKeyMd5 > , pub request_progress : Option < RequestProgress > , pub output_serialization : Option < OutputSerialization > , pub bucket : Option < BucketName > , pub key : Option < ObjectKey > , pub sse_customer_key : Option < SseCustomerKey > , pub expected_bucket_owner : Option < AccountId > , pub input_serialization : Option < InputSerialization > , pub scan_range : Option < ScanRange > , pub expression : Option < Expression > , pub sse_customer_algorithm : Option < SseCustomerAlgorithm > , pub expression_type : Option < ExpressionType > , }

# [derive (Debug , Clone)] pub struct ReplicationTime { pub time : Option < ReplicationTimeValue > , pub status : Option < ReplicationTimeStatus > , }

# [derive (Debug , Clone)] pub struct GetBucketWebsiteOutput { pub redirect_all_requests_to : Option < RedirectAllRequestsTo > , pub routing_rules : Option < RoutingRules > , pub error_document : Option < ErrorDocument > , pub index_document : Option < IndexDocument > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct ListBucketAnalyticsConfigurations ; impl Op for ListBucketAnalyticsConfigurations { type Input = ListBucketAnalyticsConfigurationsRequest ; type Output = ListBucketAnalyticsConfigurationsOutput ; }

pub type BypassGovernanceRetention = bool ;

# [derive (Debug , Clone)] pub struct ObjectVersion { pub checksum_algorithm : Option < ChecksumAlgorithmList > , pub key : Option < ObjectKey > , pub last_modified : Option < LastModified > , pub size : Option < Size > , pub version_id : Option < ObjectVersionId > , pub owner : Option < Owner > , pub is_latest : Option < IsLatest > , pub storage_class : Option < ObjectVersionStorageClass > , pub e_tag : Option < ETag > , }

# [derive (Debug , Clone)] pub struct HeadObjectOutput { pub parts_count : Option < PartsCount > , pub content_type : Option < ContentType > , pub delete_marker : Option < DeleteMarker > , pub e_tag : Option < ETag > , pub expires : Option < Expires > , pub storage_class : Option < StorageClass > , pub replication_status : Option < ReplicationStatus > , pub archive_status : Option < ArchiveStatus > , pub server_side_encryption : Option < ServerSideEncryption > , pub expiration : Option < Expiration > , pub cache_control : Option < CacheControl > , pub sse_customer_algorithm : Option < SseCustomerAlgorithm > , pub restore : Option < Restore > , pub ssekms_key_id : Option < SsekmsKeyId > , pub content_disposition : Option < ContentDisposition > , pub checksum_sha256 : Option < ChecksumSha256 > , pub object_lock_legal_hold_status : Option < ObjectLockLegalHoldStatus > , pub content_length : Option < ContentLength > , pub object_lock_mode : Option < ObjectLockMode > , pub last_modified : Option < LastModified > , pub request_charged : Option < RequestCharged > , pub sse_customer_key_md5 : Option < SseCustomerKeyMd5 > , pub checksum_crc32_c : Option < ChecksumCrc32c > , pub checksum_sha1 : Option < ChecksumSha1 > , pub bucket_key_enabled : Option < BucketKeyEnabled > , pub accept_ranges : Option < AcceptRanges > , pub content_encoding : Option < ContentEncoding > , pub missing_meta : Option < MissingMeta > , pub website_redirect_location : Option < WebsiteRedirectLocation > , pub content_language : Option < ContentLanguage > , pub version_id : Option < ObjectVersionId > , pub checksum_crc32 : Option < ChecksumCrc32 > , pub metadata : Option < Metadata > , pub object_lock_retain_until_date : Option < ObjectLockRetainUntilDate > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum ArchiveStatus { ARCHIVE_ACCESS , DEEP_ARCHIVE_ACCESS , } impl AsRef < str > for ArchiveStatus { fn as_ref (& self) -> & str { match self { Self :: ARCHIVE_ACCESS => "ARCHIVE_ACCESS" , Self :: DEEP_ARCHIVE_ACCESS => "DEEP_ARCHIVE_ACCESS" , } } } impl TryFrom < & str > for ArchiveStatus { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "ARCHIVE_ACCESS" => Ok (Self :: ARCHIVE_ACCESS) , "DEEP_ARCHIVE_ACCESS" => Ok (Self :: DEEP_ARCHIVE_ACCESS) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

# [derive (Debug , Clone)] pub struct TargetGrant { pub grantee : Option < Grantee > , pub permission : Option < BucketLogsPermission > , }

# [derive (Debug , Clone)] pub struct Bucket { pub creation_date : Option < CreationDate > , pub name : Option < BucketName > , }

pub type ExposeHeader = String ;

# [derive (Debug , Clone)] pub struct RequestPaymentConfiguration { pub payer : Option < Payer > , }

# [derive (Debug , Clone)] pub struct GetBucketMetricsConfigurationRequest { pub id : Option < MetricsId > , pub expected_bucket_owner : Option < AccountId > , pub bucket : Option < BucketName > , }

pub type CopySourceSseCustomerKeyMd5 = String ;

# [derive (Debug , Clone)] pub struct DeleteBucketIntelligentTieringConfigurationRequest { pub bucket : Option < BucketName > , pub id : Option < IntelligentTieringId > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum AnalyticsS3ExportFileFormat { CSV , } impl AsRef < str > for AnalyticsS3ExportFileFormat { fn as_ref (& self) -> & str { match self { Self :: CSV => "CSV" , } } } impl TryFrom < & str > for AnalyticsS3ExportFileFormat { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "CSV" => Ok (Self :: CSV) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub type AbortDate = String ;

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct CopyObject ; impl Op for CopyObject { type Input = CopyObjectRequest ; type Output = CopyObjectOutput ; }

pub type ExpiredObjectDeleteMarker = bool ;

# [derive (Debug , Clone)] pub struct GetBucketEncryptionRequest { pub bucket : Option < BucketName > , pub expected_bucket_owner : Option < AccountId > , }

# [derive (Debug , Clone)] pub enum MetricsFilter { And (MetricsAndOperator) , Tag (Tag) , Prefix (Prefix) , AccessPointArn (AccessPointArn) , }

pub type MultipartUploadList = Vec < MultipartUpload > ;

pub type ResponseContentType = String ;

pub type TaggingHeader = String ;

# [derive (Debug , Clone)] pub struct AbortMultipartUploadRequest { pub bucket : Option < BucketName > , pub request_payer : Option < RequestPayer > , pub expected_bucket_owner : Option < AccountId > , pub key : Option < ObjectKey > , pub upload_id : Option < MultipartUploadId > , }

pub type Body = Vec < u8 > ;

# [derive (Debug , Clone)] pub struct PutBucketAclRequest { pub content_md5 : Option < ContentMd5 > , pub expected_bucket_owner : Option < AccountId > , pub acl : Option < BucketCannedAcl > , pub grant_write : Option < GrantWrite > , pub bucket : Option < BucketName > , pub grant_full_control : Option < GrantFullControl > , pub grant_read : Option < GrantRead > , pub checksum_algorithm : Option < ChecksumAlgorithm > , pub grant_read_acp : Option < GrantReadAcp > , pub grant_write_acp : Option < GrantWriteAcp > , pub access_control_policy : Option < AccessControlPolicy > , }

# [derive (Debug , Clone)] pub struct SelectObjectContentOutput { pub payload : Option < SelectObjectContentEventStream > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct GetBucketCors ; impl Op for GetBucketCors { type Input = GetBucketCorsRequest ; type Output = GetBucketCorsOutput ; }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct GetObjectLegalHold ; impl Op for GetObjectLegalHold { type Input = GetObjectLegalHoldRequest ; type Output = GetObjectLegalHoldOutput ; }

pub type Grants = Vec < Grant > ;

pub type RestoreOutputPath = String ;

pub type TopicConfigurationList = Vec < TopicConfiguration > ;

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct WriteGetObjectResponse ; impl Op for WriteGetObjectResponse { type Input = WriteGetObjectResponseRequest ; type Output = () ; }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum FilterRuleName { prefix , suffix , } impl AsRef < str > for FilterRuleName { fn as_ref (& self) -> & str { match self { Self :: prefix => "prefix" , Self :: suffix => "suffix" , } } } impl TryFrom < & str > for FilterRuleName { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "prefix" => Ok (Self :: prefix) , "suffix" => Ok (Self :: suffix) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub type Role = String ;

pub type PartNumberMarker = String ;

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct PutBucketOwnershipControls ; impl Op for PutBucketOwnershipControls { type Input = PutBucketOwnershipControlsRequest ; type Output = () ; }

# [derive (Debug , Clone)] pub struct PutBucketAnalyticsConfigurationRequest { pub analytics_configuration : Option < AnalyticsConfiguration > , pub id : Option < AnalyticsId > , pub expected_bucket_owner : Option < AccountId > , pub bucket : Option < BucketName > , }

pub type SseCustomerAlgorithm = String ;

pub type TopicArn = String ;

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct PutBucketAcl ; impl Op for PutBucketAcl { type Input = PutBucketAclRequest ; type Output = () ; }

# [derive (Debug , Clone)] pub struct GetBucketAnalyticsConfigurationOutput { pub analytics_configuration : Option < AnalyticsConfiguration > , }

pub type Expression = String ;

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum ReplicaModificationsStatus { Enabled , Disabled , } impl AsRef < str > for ReplicaModificationsStatus { fn as_ref (& self) -> & str { match self { Self :: Enabled => "Enabled" , Self :: Disabled => "Disabled" , } } } impl TryFrom < & str > for ReplicaModificationsStatus { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "Enabled" => Ok (Self :: Enabled) , "Disabled" => Ok (Self :: Disabled) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub type TagCount = i32 ;

# [derive (Debug , Clone)] pub struct GetBucketPolicyOutput { pub policy : Option < Policy > , }

# [derive (Debug , Clone)] pub struct GetObjectRetentionOutput { pub retention : Option < ObjectLockRetention > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct GetBucketMetricsConfiguration ; impl Op for GetBucketMetricsConfiguration { type Input = GetBucketMetricsConfigurationRequest ; type Output = GetBucketMetricsConfigurationOutput ; }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum QuoteFields { ALWAYS , ASNEEDED , } impl AsRef < str > for QuoteFields { fn as_ref (& self) -> & str { match self { Self :: ALWAYS => "ALWAYS" , Self :: ASNEEDED => "ASNEEDED" , } } } impl TryFrom < & str > for QuoteFields { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "ALWAYS" => Ok (Self :: ALWAYS) , "ASNEEDED" => Ok (Self :: ASNEEDED) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct GetObjectAttributes ; impl Op for GetObjectAttributes { type Input = GetObjectAttributesRequest ; type Output = GetObjectAttributesOutput ; }

# [derive (Debug , Clone)] pub enum LifecycleRuleFilter { ObjectSizeGreaterThan (ObjectSizeGreaterThanBytes) , And (LifecycleRuleAndOperator) , ObjectSizeLessThan (ObjectSizeLessThanBytes) , Prefix (Prefix) , Tag (Tag) , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum BucketLocationConstraint { af_south_1 , ap_east_1 , ap_northeast_1 , ap_northeast_2 , ap_northeast_3 , ap_south_1 , ap_southeast_1 , ap_southeast_2 , ca_central_1 , cn_north_1 , cn_northwest_1 , EU , eu_central_1 , eu_north_1 , eu_south_1 , eu_west_1 , eu_west_2 , eu_west_3 , me_south_1 , sa_east_1 , us_east_2 , us_gov_east_1 , us_gov_west_1 , us_west_1 , us_west_2 , } impl AsRef < str > for BucketLocationConstraint { fn as_ref (& self) -> & str { match self { Self :: af_south_1 => "af-south-1" , Self :: ap_east_1 => "ap-east-1" , Self :: ap_northeast_1 => "ap-northeast-1" , Self :: ap_northeast_2 => "ap-northeast-2" , Self :: ap_northeast_3 => "ap-northeast-3" , Self :: ap_south_1 => "ap-south-1" , Self :: ap_southeast_1 => "ap-southeast-1" , Self :: ap_southeast_2 => "ap-southeast-2" , Self :: ca_central_1 => "ca-central-1" , Self :: cn_north_1 => "cn-north-1" , Self :: cn_northwest_1 => "cn-northwest-1" , Self :: EU => "EU" , Self :: eu_central_1 => "eu-central-1" , Self :: eu_north_1 => "eu-north-1" , Self :: eu_south_1 => "eu-south-1" , Self :: eu_west_1 => "eu-west-1" , Self :: eu_west_2 => "eu-west-2" , Self :: eu_west_3 => "eu-west-3" , Self :: me_south_1 => "me-south-1" , Self :: sa_east_1 => "sa-east-1" , Self :: us_east_2 => "us-east-2" , Self :: us_gov_east_1 => "us-gov-east-1" , Self :: us_gov_west_1 => "us-gov-west-1" , Self :: us_west_1 => "us-west-1" , Self :: us_west_2 => "us-west-2" , } } } impl TryFrom < & str > for BucketLocationConstraint { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "af-south-1" => Ok (Self :: af_south_1) , "ap-east-1" => Ok (Self :: ap_east_1) , "ap-northeast-1" => Ok (Self :: ap_northeast_1) , "ap-northeast-2" => Ok (Self :: ap_northeast_2) , "ap-northeast-3" => Ok (Self :: ap_northeast_3) , "ap-south-1" => Ok (Self :: ap_south_1) , "ap-southeast-1" => Ok (Self :: ap_southeast_1) , "ap-southeast-2" => Ok (Self :: ap_southeast_2) , "ca-central-1" => Ok (Self :: ca_central_1) , "cn-north-1" => Ok (Self :: cn_north_1) , "cn-northwest-1" => Ok (Self :: cn_northwest_1) , "EU" => Ok (Self :: EU) , "eu-central-1" => Ok (Self :: eu_central_1) , "eu-north-1" => Ok (Self :: eu_north_1) , "eu-south-1" => Ok (Self :: eu_south_1) , "eu-west-1" => Ok (Self :: eu_west_1) , "eu-west-2" => Ok (Self :: eu_west_2) , "eu-west-3" => Ok (Self :: eu_west_3) , "me-south-1" => Ok (Self :: me_south_1) , "sa-east-1" => Ok (Self :: sa_east_1) , "us-east-2" => Ok (Self :: us_east_2) , "us-gov-east-1" => Ok (Self :: us_gov_east_1) , "us-gov-west-1" => Ok (Self :: us_gov_west_1) , "us-west-1" => Ok (Self :: us_west_1) , "us-west-2" => Ok (Self :: us_west_2) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub type GrantReadAcp = String ;

# [derive (Debug , Clone)] pub struct DeleteBucketPolicyRequest { pub bucket : Option < BucketName > , pub expected_bucket_owner : Option < AccountId > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct DeleteBucketAnalyticsConfiguration ; impl Op for DeleteBucketAnalyticsConfiguration { type Input = DeleteBucketAnalyticsConfigurationRequest ; type Output = () ; }

pub type ErrorCode = String ;

# [derive (Debug , Clone)] pub struct OutputSerialization { pub json : Option < JsonOutput > , pub csv : Option < CsvOutput > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct CompleteMultipartUpload ; impl Op for CompleteMultipartUpload { type Input = CompleteMultipartUploadRequest ; type Output = CompleteMultipartUploadOutput ; }

pub type ContentType = String ;

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum InventoryFrequency { Daily , Weekly , } impl AsRef < str > for InventoryFrequency { fn as_ref (& self) -> & str { match self { Self :: Daily => "Daily" , Self :: Weekly => "Weekly" , } } } impl TryFrom < & str > for InventoryFrequency { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "Daily" => Ok (Self :: Daily) , "Weekly" => Ok (Self :: Weekly) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

# [derive (Debug , Clone)] pub struct UploadPartCopyOutput { pub server_side_encryption : Option < ServerSideEncryption > , pub ssekms_key_id : Option < SsekmsKeyId > , pub copy_source_version_id : Option < CopySourceVersionId > , pub request_charged : Option < RequestCharged > , pub copy_part_result : Option < CopyPartResult > , pub sse_customer_algorithm : Option < SseCustomerAlgorithm > , pub bucket_key_enabled : Option < BucketKeyEnabled > , pub sse_customer_key_md5 : Option < SseCustomerKeyMd5 > , }

# [derive (Debug , Clone)] pub struct DeletePublicAccessBlockRequest { pub bucket : Option < BucketName > , pub expected_bucket_owner : Option < AccountId > , }

pub type CopySourceVersionId = String ;

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum ExpressionType { SQL , } impl AsRef < str > for ExpressionType { fn as_ref (& self) -> & str { match self { Self :: SQL => "SQL" , } } } impl TryFrom < & str > for ExpressionType { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "SQL" => Ok (Self :: SQL) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct GetBucketOwnershipControls ; impl Op for GetBucketOwnershipControls { type Input = GetBucketOwnershipControlsRequest ; type Output = GetBucketOwnershipControlsOutput ; }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct GetObjectLockConfiguration ; impl Op for GetObjectLockConfiguration { type Input = GetObjectLockConfigurationRequest ; type Output = GetObjectLockConfigurationOutput ; }

# [derive (Debug , Clone)] pub struct GetObjectTorrentOutput { pub body : Option < StreamingBlob > , pub request_charged : Option < RequestCharged > , }

# [derive (Debug , Clone)] pub struct LoggingEnabled { pub target_bucket : Option < TargetBucket > , pub target_prefix : Option < TargetPrefix > , pub target_grants : Option < TargetGrants > , }

# [derive (Debug , Clone)] pub struct Progress { pub bytes_returned : Option < BytesReturned > , pub bytes_scanned : Option < BytesScanned > , pub bytes_processed : Option < BytesProcessed > , }

pub type RoutingRules = Vec < RoutingRule > ;

pub type TargetBucket = String ;

# [derive (Debug , Clone)] pub struct ListObjectsV2Output { pub next_continuation_token : Option < NextToken > , pub delimiter : Option < Delimiter > , pub common_prefixes : Option < CommonPrefixList > , pub prefix : Option < Prefix > , pub start_after : Option < StartAfter > , pub contents : Option < ObjectList > , pub name : Option < BucketName > , pub key_count : Option < KeyCount > , pub encoding_type : Option < EncodingType > , pub continuation_token : Option < Token > , pub max_keys : Option < MaxKeys > , pub is_truncated : Option < IsTruncated > , }

# [derive (Debug , Clone)] pub struct BucketAlreadyOwnedByYou { }

# [derive (Debug , Clone)] pub struct CreateMultipartUploadRequest { pub grant_full_control : Option < GrantFullControl > , pub cache_control : Option < CacheControl > , pub content_encoding : Option < ContentEncoding > , pub acl : Option < ObjectCannedAcl > , pub bucket_key_enabled : Option < BucketKeyEnabled > , pub key : Option < ObjectKey > , pub bucket : Option < BucketName > , pub metadata : Option < Metadata > , pub sse_customer_algorithm : Option < SseCustomerAlgorithm > , pub expires : Option < Expires > , pub ssekms_encryption_context : Option < SsekmsEncryptionContext > , pub website_redirect_location : Option < WebsiteRedirectLocation > , pub content_language : Option < ContentLanguage > , pub expected_bucket_owner : Option < AccountId > , pub content_type : Option < ContentType > , pub grant_write_acp : Option < GrantWriteAcp > , pub grant_read : Option < GrantRead > , pub checksum_algorithm : Option < ChecksumAlgorithm > , pub tagging : Option < TaggingHeader > , pub grant_read_acp : Option < GrantReadAcp > , pub request_payer : Option < RequestPayer > , pub ssekms_key_id : Option < SsekmsKeyId > , pub sse_customer_key : Option < SseCustomerKey > , pub server_side_encryption : Option < ServerSideEncryption > , pub content_disposition : Option < ContentDisposition > , pub sse_customer_key_md5 : Option < SseCustomerKeyMd5 > , pub object_lock_retain_until_date : Option < ObjectLockRetainUntilDate > , pub storage_class : Option < StorageClass > , pub object_lock_legal_hold_status : Option < ObjectLockLegalHoldStatus > , pub object_lock_mode : Option < ObjectLockMode > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum ExistingObjectReplicationStatus { Enabled , Disabled , } impl AsRef < str > for ExistingObjectReplicationStatus { fn as_ref (& self) -> & str { match self { Self :: Enabled => "Enabled" , Self :: Disabled => "Disabled" , } } } impl TryFrom < & str > for ExistingObjectReplicationStatus { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "Enabled" => Ok (Self :: Enabled) , "Disabled" => Ok (Self :: Disabled) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

# [derive (Debug , Clone)] pub struct GetObjectTorrentRequest { pub bucket : Option < BucketName > , pub key : Option < ObjectKey > , pub expected_bucket_owner : Option < AccountId > , pub request_payer : Option < RequestPayer > , }

# [derive (Debug , Clone)] pub struct CorsRule { pub allowed_origins : Option < AllowedOrigins > , pub allowed_headers : Option < AllowedHeaders > , pub expose_headers : Option < ExposeHeaders > , pub id : Option < Id > , pub max_age_seconds : Option < MaxAgeSeconds > , pub allowed_methods : Option < AllowedMethods > , }

# [derive (Debug , Clone)] pub struct Condition { pub key_prefix_equals : Option < KeyPrefixEquals > , pub http_error_code_returned_equals : Option < HttpErrorCodeReturnedEquals > , }

pub type FieldDelimiter = String ;

# [derive (Debug , Clone)] pub struct GetBucketReplicationRequest { pub bucket : Option < BucketName > , pub expected_bucket_owner : Option < AccountId > , }

# [derive (Debug , Clone)] pub struct DeleteBucketWebsiteRequest { pub expected_bucket_owner : Option < AccountId > , pub bucket : Option < BucketName > , }

# [derive (Debug , Clone)] pub struct GetBucketWebsiteRequest { pub bucket : Option < BucketName > , pub expected_bucket_owner : Option < AccountId > , }

# [derive (Debug , Clone)] pub struct NoSuchKey { }

pub type IfUnmodifiedSince = String ;

# [derive (Debug , Clone)] pub struct CreateMultipartUploadOutput { pub sse_customer_key_md5 : Option < SseCustomerKeyMd5 > , pub bucket_key_enabled : Option < BucketKeyEnabled > , pub sse_customer_algorithm : Option < SseCustomerAlgorithm > , pub abort_rule_id : Option < AbortRuleId > , pub bucket : Option < BucketName > , pub checksum_algorithm : Option < ChecksumAlgorithm > , pub ssekms_key_id : Option < SsekmsKeyId > , pub upload_id : Option < MultipartUploadId > , pub key : Option < ObjectKey > , pub request_charged : Option < RequestCharged > , pub server_side_encryption : Option < ServerSideEncryption > , pub ssekms_encryption_context : Option < SsekmsEncryptionContext > , pub abort_date : Option < AbortDate > , }

# [derive (Debug , Clone)] pub struct GetObjectRetentionRequest { pub version_id : Option < ObjectVersionId > , pub expected_bucket_owner : Option < AccountId > , pub key : Option < ObjectKey > , pub bucket : Option < BucketName > , pub request_payer : Option < RequestPayer > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct HeadObject ; impl Op for HeadObject { type Input = HeadObjectRequest ; type Output = HeadObjectOutput ; }

# [derive (Debug , Clone)] pub struct InventoryEncryption { pub ssekms : Option < Ssekms > , pub sses3 : Option < Sses3 > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct ListBucketMetricsConfigurations ; impl Op for ListBucketMetricsConfigurations { type Input = ListBucketMetricsConfigurationsRequest ; type Output = ListBucketMetricsConfigurationsOutput ; }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct PutBucketReplication ; impl Op for PutBucketReplication { type Input = PutBucketReplicationRequest ; type Output = () ; }

# [derive (Debug , Clone)] pub struct S3Location { pub tagging : Option < Tagging > , pub canned_acl : Option < ObjectCannedAcl > , pub prefix : Option < LocationPrefix > , pub access_control_list : Option < Grants > , pub storage_class : Option < StorageClass > , pub user_metadata : Option < UserMetadata > , pub bucket_name : Option < BucketName > , pub encryption : Option < Encryption > , }

pub type ReplicationRules = Vec < ReplicationRule > ;

pub type NextPartNumberMarker = String ;

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum JsonType { DOCUMENT , LINES , } impl AsRef < str > for JsonType { fn as_ref (& self) -> & str { match self { Self :: DOCUMENT => "DOCUMENT" , Self :: LINES => "LINES" , } } } impl TryFrom < & str > for JsonType { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "DOCUMENT" => Ok (Self :: DOCUMENT) , "LINES" => Ok (Self :: LINES) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct ListObjects ; impl Op for ListObjects { type Input = ListObjectsRequest ; type Output = ListObjectsOutput ; }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum Permission { FULL_CONTROL , WRITE , WRITE_ACP , READ , READ_ACP , } impl AsRef < str > for Permission { fn as_ref (& self) -> & str { match self { Self :: FULL_CONTROL => "FULL_CONTROL" , Self :: WRITE => "WRITE" , Self :: WRITE_ACP => "WRITE_ACP" , Self :: READ => "READ" , Self :: READ_ACP => "READ_ACP" , } } } impl TryFrom < & str > for Permission { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "FULL_CONTROL" => Ok (Self :: FULL_CONTROL) , "WRITE" => Ok (Self :: WRITE) , "WRITE_ACP" => Ok (Self :: WRITE_ACP) , "READ" => Ok (Self :: READ) , "READ_ACP" => Ok (Self :: READ_ACP) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub type Comments = String ;

# [derive (Debug , Clone)] pub struct ServerSideEncryptionConfiguration { pub rules : Option < ServerSideEncryptionRules > , }

# [derive (Debug , Clone)] pub struct ServerSideEncryptionRule { pub apply_server_side_encryption_by_default : Option < ServerSideEncryptionByDefault > , pub bucket_key_enabled : Option < BucketKeyEnabled > , }

# [derive (Debug , Clone)] pub struct GetBucketAnalyticsConfigurationRequest { pub expected_bucket_owner : Option < AccountId > , pub id : Option < AnalyticsId > , pub bucket : Option < BucketName > , }

pub type KeyMarker = String ;

pub type SsekmsEncryptionContext = String ;

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct DeleteBucket ; impl Op for DeleteBucket { type Input = DeleteBucketRequest ; type Output = () ; }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct GetObjectAcl ; impl Op for GetObjectAcl { type Input = GetObjectAclRequest ; type Output = GetObjectAclOutput ; }

# [derive (Debug , Clone)] pub struct GetObjectAttributesParts { pub next_part_number_marker : Option < NextPartNumberMarker > , pub parts : Option < PartsList > , pub is_truncated : Option < IsTruncated > , pub max_parts : Option < MaxParts > , pub part_number_marker : Option < PartNumberMarker > , pub total_parts_count : Option < PartsCount > , }

pub type RequestToken = String ;

# [derive (Debug , Clone)] pub struct GetPublicAccessBlockRequest { pub expected_bucket_owner : Option < AccountId > , pub bucket : Option < BucketName > , }

# [derive (Debug , Clone)] pub struct ObjectLockConfiguration { pub rule : Option < ObjectLockRule > , pub object_lock_enabled : Option < ObjectLockEnabled > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct RestoreObject ; impl Op for RestoreObject { type Input = RestoreObjectRequest ; type Output = RestoreObjectOutput ; }

# [derive (Debug , Clone)] pub struct WebsiteConfiguration { pub index_document : Option < IndexDocument > , pub routing_rules : Option < RoutingRules > , pub error_document : Option < ErrorDocument > , pub redirect_all_requests_to : Option < RedirectAllRequestsTo > , }

pub type CreationDate = String ;

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct PutBucketCors ; impl Op for PutBucketCors { type Input = PutBucketCorsRequest ; type Output = () ; }

# [derive (Debug , Clone)] pub struct CsvOutput { pub quote_escape_character : Option < QuoteEscapeCharacter > , pub record_delimiter : Option < RecordDelimiter > , pub field_delimiter : Option < FieldDelimiter > , pub quote_fields : Option < QuoteFields > , pub quote_character : Option < QuoteCharacter > , }

pub type CopySourceIfUnmodifiedSince = String ;

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum ExpirationStatus { Enabled , Disabled , } impl AsRef < str > for ExpirationStatus { fn as_ref (& self) -> & str { match self { Self :: Enabled => "Enabled" , Self :: Disabled => "Disabled" , } } } impl TryFrom < & str > for ExpirationStatus { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "Enabled" => Ok (Self :: Enabled) , "Disabled" => Ok (Self :: Disabled) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct DeleteBucketPolicy ; impl Op for DeleteBucketPolicy { type Input = DeleteBucketPolicyRequest ; type Output = () ; }

pub type ExposeHeaders = Vec < ExposeHeader > ;

# [derive (Debug , Clone)] pub struct GetBucketOwnershipControlsRequest { pub bucket : Option < BucketName > , pub expected_bucket_owner : Option < AccountId > , }

pub type TransitionList = Vec < Transition > ;

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum Type { CanonicalUser , AmazonCustomerByEmail , Group , } impl AsRef < str > for Type { fn as_ref (& self) -> & str { match self { Self :: CanonicalUser => "CanonicalUser" , Self :: AmazonCustomerByEmail => "AmazonCustomerByEmail" , Self :: Group => "Group" , } } } impl TryFrom < & str > for Type { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "CanonicalUser" => Ok (Self :: CanonicalUser) , "AmazonCustomerByEmail" => Ok (Self :: AmazonCustomerByEmail) , "Group" => Ok (Self :: Group) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct ListBucketInventoryConfigurations ; impl Op for ListBucketInventoryConfigurations { type Input = ListBucketInventoryConfigurationsRequest ; type Output = ListBucketInventoryConfigurationsOutput ; }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum RequestPayer { requester , } impl AsRef < str > for RequestPayer { fn as_ref (& self) -> & str { match self { Self :: requester => "requester" , } } } impl TryFrom < & str > for RequestPayer { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "requester" => Ok (Self :: requester) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub type ResponseContentDisposition = String ;

# [derive (Debug , Clone)] pub struct EncryptionConfiguration { pub replica_kms_key_id : Option < ReplicaKmsKeyId > , }

# [derive (Debug , Clone)] pub struct PutObjectAclOutput { pub request_charged : Option < RequestCharged > , }

pub type CommonPrefixList = Vec < CommonPrefix > ;

# [derive (Debug , Clone)] pub struct DeleteObjectOutput { pub delete_marker : Option < DeleteMarker > , pub request_charged : Option < RequestCharged > , pub version_id : Option < ObjectVersionId > , }

pub type IsLatest = bool ;

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct PutObjectTagging ; impl Op for PutObjectTagging { type Input = PutObjectTaggingRequest ; type Output = PutObjectTaggingOutput ; }

pub type CacheControl = String ;

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum MfaDeleteStatus { Enabled , Disabled , } impl AsRef < str > for MfaDeleteStatus { fn as_ref (& self) -> & str { match self { Self :: Enabled => "Enabled" , Self :: Disabled => "Disabled" , } } } impl TryFrom < & str > for MfaDeleteStatus { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "Enabled" => Ok (Self :: Enabled) , "Disabled" => Ok (Self :: Disabled) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub type AllowQuotedRecordDelimiter = bool ;

pub type ObjectLockEnabledForBucket = bool ;

pub type ChecksumAlgorithmList = Vec < ChecksumAlgorithm > ;

# [derive (Debug , Clone)] pub struct PutBucketAccelerateConfigurationRequest { pub accelerate_configuration : Option < AccelerateConfiguration > , pub checksum_algorithm : Option < ChecksumAlgorithm > , pub bucket : Option < BucketName > , pub expected_bucket_owner : Option < AccountId > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct PutBucketEncryption ; impl Op for PutBucketEncryption { type Input = PutBucketEncryptionRequest ; type Output = () ; }

# [derive (Debug , Clone)] pub struct StatsEvent { pub details : Option < Stats > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct PutObject ; impl Op for PutObject { type Input = PutObjectRequest ; type Output = PutObjectOutput ; }

# [derive (Debug , Clone)] pub struct RecordsEvent { pub payload : Option < Body > , }

pub type VersionIdMarker = String ;

# [derive (Debug , Clone)] pub struct ListPartsRequest { pub max_parts : Option < MaxParts > , pub sse_customer_algorithm : Option < SseCustomerAlgorithm > , pub sse_customer_key : Option < SseCustomerKey > , pub sse_customer_key_md5 : Option < SseCustomerKeyMd5 > , pub bucket : Option < BucketName > , pub key : Option < ObjectKey > , pub request_payer : Option < RequestPayer > , pub upload_id : Option < MultipartUploadId > , pub expected_bucket_owner : Option < AccountId > , pub part_number_marker : Option < PartNumberMarker > , }

pub type WebsiteRedirectLocation = String ;

# [derive (Debug , Clone)] pub struct GetObjectOutput { pub expires : Option < Expires > , pub sse_customer_key_md5 : Option < SseCustomerKeyMd5 > , pub storage_class : Option < StorageClass > , pub tag_count : Option < TagCount > , pub restore : Option < Restore > , pub accept_ranges : Option < AcceptRanges > , pub cache_control : Option < CacheControl > , pub website_redirect_location : Option < WebsiteRedirectLocation > , pub body : Option < StreamingBlob > , pub checksum_sha1 : Option < ChecksumSha1 > , pub object_lock_mode : Option < ObjectLockMode > , pub content_disposition : Option < ContentDisposition > , pub content_encoding : Option < ContentEncoding > , pub last_modified : Option < LastModified > , pub sse_customer_algorithm : Option < SseCustomerAlgorithm > , pub content_type : Option < ContentType > , pub delete_marker : Option < DeleteMarker > , pub object_lock_retain_until_date : Option < ObjectLockRetainUntilDate > , pub replication_status : Option < ReplicationStatus > , pub server_side_encryption : Option < ServerSideEncryption > , pub version_id : Option < ObjectVersionId > , pub parts_count : Option < PartsCount > , pub content_range : Option < ContentRange > , pub object_lock_legal_hold_status : Option < ObjectLockLegalHoldStatus > , pub checksum_crc32 : Option < ChecksumCrc32 > , pub request_charged : Option < RequestCharged > , pub checksum_sha256 : Option < ChecksumSha256 > , pub ssekms_key_id : Option < SsekmsKeyId > , pub metadata : Option < Metadata > , pub expiration : Option < Expiration > , pub content_language : Option < ContentLanguage > , pub checksum_crc32_c : Option < ChecksumCrc32c > , pub content_length : Option < ContentLength > , pub e_tag : Option < ETag > , pub bucket_key_enabled : Option < BucketKeyEnabled > , pub missing_meta : Option < MissingMeta > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct GetBucketLogging ; impl Op for GetBucketLogging { type Input = GetBucketLoggingRequest ; type Output = GetBucketLoggingOutput ; }

# [derive (Debug , Clone)] pub struct UploadPartRequest { pub content_length : Option < ContentLength > , pub sse_customer_key_md5 : Option < SseCustomerKeyMd5 > , pub checksum_algorithm : Option < ChecksumAlgorithm > , pub checksum_crc32 : Option < ChecksumCrc32 > , pub sse_customer_key : Option < SseCustomerKey > , pub body : Option < StreamingBlob > , pub key : Option < ObjectKey > , pub checksum_sha256 : Option < ChecksumSha256 > , pub checksum_sha1 : Option < ChecksumSha1 > , pub sse_customer_algorithm : Option < SseCustomerAlgorithm > , pub upload_id : Option < MultipartUploadId > , pub request_payer : Option < RequestPayer > , pub expected_bucket_owner : Option < AccountId > , pub part_number : Option < PartNumber > , pub content_md5 : Option < ContentMd5 > , pub checksum_crc32_c : Option < ChecksumCrc32c > , pub bucket : Option < BucketName > , }

pub type BucketName = String ;

pub type CopySourceIfNoneMatch = String ;

# [derive (Debug , Clone)] pub struct AnalyticsAndOperator { pub tags : Option < TagSet > , pub prefix : Option < Prefix > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct DeleteBucketTagging ; impl Op for DeleteBucketTagging { type Input = DeleteBucketTaggingRequest ; type Output = () ; }

# [derive (Debug , Clone)] pub struct ParquetInput { }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct GetBucketWebsite ; impl Op for GetBucketWebsite { type Input = GetBucketWebsiteRequest ; type Output = GetBucketWebsiteOutput ; }

# [derive (Debug , Clone)] pub struct PutBucketRequestPaymentRequest { pub content_md5 : Option < ContentMd5 > , pub expected_bucket_owner : Option < AccountId > , pub request_payment_configuration : Option < RequestPaymentConfiguration > , pub checksum_algorithm : Option < ChecksumAlgorithm > , pub bucket : Option < BucketName > , }

pub type ResponseContentLanguage = String ;

pub type AnalyticsConfigurationList = Vec < AnalyticsConfiguration > ;

pub type Size = i64 ;

# [derive (Debug , Clone)] pub struct DeleteObjectRequest { pub mfa : Option < Mfa > , pub bypass_governance_retention : Option < BypassGovernanceRetention > , pub key : Option < ObjectKey > , pub bucket : Option < BucketName > , pub request_payer : Option < RequestPayer > , pub expected_bucket_owner : Option < AccountId > , pub version_id : Option < ObjectVersionId > , }

pub type ObjectVersionId = String ;

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum ObjectOwnership { BucketOwnerPreferred , ObjectWriter , BucketOwnerEnforced , } impl AsRef < str > for ObjectOwnership { fn as_ref (& self) -> & str { match self { Self :: BucketOwnerPreferred => "BucketOwnerPreferred" , Self :: ObjectWriter => "ObjectWriter" , Self :: BucketOwnerEnforced => "BucketOwnerEnforced" , } } } impl TryFrom < & str > for ObjectOwnership { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "BucketOwnerPreferred" => Ok (Self :: BucketOwnerPreferred) , "ObjectWriter" => Ok (Self :: ObjectWriter) , "BucketOwnerEnforced" => Ok (Self :: BucketOwnerEnforced) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

# [derive (Debug , Clone)] pub struct ServerSideEncryptionByDefault { pub kms_master_key_id : Option < SsekmsKeyId > , pub sse_algorithm : Option < ServerSideEncryption > , }

# [derive (Debug , Clone)] pub struct GetBucketOwnershipControlsOutput { pub ownership_controls : Option < OwnershipControls > , }

pub type QuoteCharacter = String ;

pub type SseCustomerKeyMd5 = String ;

# [derive (Debug , Clone)] pub struct DeleteObjectTaggingRequest { pub bucket : Option < BucketName > , pub expected_bucket_owner : Option < AccountId > , pub version_id : Option < ObjectVersionId > , pub key : Option < ObjectKey > , }

pub type ChecksumSha1 = String ;

pub type ChecksumCrc32c = String ;

pub type DeletedObjects = Vec < DeletedObject > ;

# [derive (Debug , Clone)] pub struct GetBucketMetricsConfigurationOutput { pub metrics_configuration : Option < MetricsConfiguration > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum ObjectLockEnabled { Enabled , } impl AsRef < str > for ObjectLockEnabled { fn as_ref (& self) -> & str { match self { Self :: Enabled => "Enabled" , } } } impl TryFrom < & str > for ObjectLockEnabled { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "Enabled" => Ok (Self :: Enabled) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

# [derive (Debug , Clone)] pub struct CompletedMultipartUpload { pub parts : Option < CompletedPartList > , }

# [derive (Debug , Clone)] pub struct PutBucketTaggingRequest { pub expected_bucket_owner : Option < AccountId > , pub bucket : Option < BucketName > , pub content_md5 : Option < ContentMd5 > , pub tagging : Option < Tagging > , pub checksum_algorithm : Option < ChecksumAlgorithm > , }

pub type BucketKeyEnabled = bool ;

# [derive (Debug , Clone)] pub struct MetricsAndOperator { pub access_point_arn : Option < AccessPointArn > , pub prefix : Option < Prefix > , pub tags : Option < TagSet > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum ObjectVersionStorageClass { STANDARD , } impl AsRef < str > for ObjectVersionStorageClass { fn as_ref (& self) -> & str { match self { Self :: STANDARD => "STANDARD" , } } } impl TryFrom < & str > for ObjectVersionStorageClass { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "STANDARD" => Ok (Self :: STANDARD) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

# [derive (Debug , Clone)] pub struct PutObjectLockConfigurationRequest { pub checksum_algorithm : Option < ChecksumAlgorithm > , pub expected_bucket_owner : Option < AccountId > , pub request_payer : Option < RequestPayer > , pub token : Option < ObjectLockToken > , pub content_md5 : Option < ContentMd5 > , pub object_lock_configuration : Option < ObjectLockConfiguration > , pub bucket : Option < BucketName > , }

pub type ResponseContentEncoding = String ;

# [derive (Debug , Clone)] pub struct CopyObjectOutput { pub expiration : Option < Expiration > , pub ssekms_encryption_context : Option < SsekmsEncryptionContext > , pub ssekms_key_id : Option < SsekmsKeyId > , pub sse_customer_key_md5 : Option < SseCustomerKeyMd5 > , pub server_side_encryption : Option < ServerSideEncryption > , pub sse_customer_algorithm : Option < SseCustomerAlgorithm > , pub version_id : Option < ObjectVersionId > , pub copy_source_version_id : Option < CopySourceVersionId > , pub bucket_key_enabled : Option < BucketKeyEnabled > , pub request_charged : Option < RequestCharged > , pub copy_object_result : Option < CopyObjectResult > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct ListMultipartUploads ; impl Op for ListMultipartUploads { type Input = ListMultipartUploadsRequest ; type Output = ListMultipartUploadsOutput ; }

# [derive (Debug , Clone)] pub struct Grantee { pub uri : Option < Uri > , pub id : Option < Id > , pub email_address : Option < EmailAddress > , pub display_name : Option < DisplayName > , pub r#type : Option < Type > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct GetObjectTorrent ; impl Op for GetObjectTorrent { type Input = GetObjectTorrentRequest ; type Output = GetObjectTorrentOutput ; }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum StorageClassAnalysisSchemaVersion { V_1 , } impl AsRef < str > for StorageClassAnalysisSchemaVersion { fn as_ref (& self) -> & str { match self { Self :: V_1 => "V_1" , } } } impl TryFrom < & str > for StorageClassAnalysisSchemaVersion { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "V_1" => Ok (Self :: V_1) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub type BytesProcessed = i64 ;

# [derive (Debug , Clone)] pub struct Delete { pub quiet : Option < Quiet > , pub objects : Option < ObjectIdentifierList > , }

# [derive (Debug , Clone)] pub struct GetBucketCorsOutput { pub cors_rules : Option < CorsRules > , }

pub type QuoteEscapeCharacter = String ;

pub type ConfirmRemoveSelfBucketAccess = bool ;

# [derive (Debug , Clone)] pub struct ReplicationConfiguration { pub role : Option < Role > , pub rules : Option < ReplicationRules > , }

pub type CorsRules = Vec < CorsRule > ;

# [derive (Debug , Clone)] pub struct PutBucketOwnershipControlsRequest { pub bucket : Option < BucketName > , pub content_md5 : Option < ContentMd5 > , pub ownership_controls : Option < OwnershipControls > , pub expected_bucket_owner : Option < AccountId > , }

# [derive (Debug , Clone)] pub struct DeleteObjectsOutput { pub errors : Option < Errors > , pub request_charged : Option < RequestCharged > , pub deleted : Option < DeletedObjects > , }

# [derive (Debug , Clone)] pub struct CsvInput { pub record_delimiter : Option < RecordDelimiter > , pub comments : Option < Comments > , pub file_header_info : Option < FileHeaderInfo > , pub allow_quoted_record_delimiter : Option < AllowQuotedRecordDelimiter > , pub quote_character : Option < QuoteCharacter > , pub field_delimiter : Option < FieldDelimiter > , pub quote_escape_character : Option < QuoteEscapeCharacter > , }

# [derive (Debug , Clone)] pub struct GetBucketIntelligentTieringConfigurationRequest { pub bucket : Option < BucketName > , pub id : Option < IntelligentTieringId > , }

# [derive (Debug , Clone)] pub struct GetBucketTaggingOutput { pub tag_set : Option < TagSet > , }

# [derive (Debug , Clone)] pub struct ListObjectsV2Request { pub expected_bucket_owner : Option < AccountId > , pub prefix : Option < Prefix > , pub request_payer : Option < RequestPayer > , pub delimiter : Option < Delimiter > , pub start_after : Option < StartAfter > , pub encoding_type : Option < EncodingType > , pub continuation_token : Option < Token > , pub bucket : Option < BucketName > , pub fetch_owner : Option < FetchOwner > , pub max_keys : Option < MaxKeys > , }

# [derive (Debug , Clone)] pub struct AbortIncompleteMultipartUpload { pub days_after_initiation : Option < DaysAfterInitiation > , }

# [derive (Debug , Clone)] pub struct GetBucketLoggingRequest { pub bucket : Option < BucketName > , pub expected_bucket_owner : Option < AccountId > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct DeleteBucketLifecycle ; impl Op for DeleteBucketLifecycle { type Input = DeleteBucketLifecycleRequest ; type Output = () ; }

# [derive (Debug , Clone)] pub struct ListPartsOutput { pub request_charged : Option < RequestCharged > , pub parts : Option < Parts > , pub checksum_algorithm : Option < ChecksumAlgorithm > , pub storage_class : Option < StorageClass > , pub part_number_marker : Option < PartNumberMarker > , pub is_truncated : Option < IsTruncated > , pub bucket : Option < BucketName > , pub upload_id : Option < MultipartUploadId > , pub initiator : Option < Initiator > , pub abort_rule_id : Option < AbortRuleId > , pub max_parts : Option < MaxParts > , pub owner : Option < Owner > , pub next_part_number_marker : Option < NextPartNumberMarker > , pub abort_date : Option < AbortDate > , pub key : Option < ObjectKey > , }

pub type CopySource = String ;

pub type ObjectSizeGreaterThanBytes = i64 ;

pub type ReplaceKeyPrefixWith = String ;

pub type SsekmsKeyId = String ;

# [derive (Debug , Clone)] pub struct GetBucketNotificationConfigurationRequest { pub bucket : Option < BucketName > , pub expected_bucket_owner : Option < AccountId > , }

# [derive (Debug , Clone)] pub struct PutPublicAccessBlockRequest { pub checksum_algorithm : Option < ChecksumAlgorithm > , pub content_md5 : Option < ContentMd5 > , pub bucket : Option < BucketName > , pub expected_bucket_owner : Option < AccountId > , pub public_access_block_configuration : Option < PublicAccessBlockConfiguration > , }

pub type Delimiter = String ;

pub type DeleteMarker = bool ;

# [derive (Debug , Clone)] pub struct BucketLifecycleConfiguration { pub rules : Option < LifecycleRules > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum ReplicationRuleStatus { Enabled , Disabled , } impl AsRef < str > for ReplicationRuleStatus { fn as_ref (& self) -> & str { match self { Self :: Enabled => "Enabled" , Self :: Disabled => "Disabled" , } } } impl TryFrom < & str > for ReplicationRuleStatus { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "Enabled" => Ok (Self :: Enabled) , "Disabled" => Ok (Self :: Disabled) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub type GetObjectResponseStatusCode = i32 ;

# [derive (Debug , Clone)] pub struct ReplicationTimeValue { pub minutes : Option < Minutes > , }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum SseKmsEncryptedObjectsStatus { Enabled , Disabled , } impl AsRef < str > for SseKmsEncryptedObjectsStatus { fn as_ref (& self) -> & str { match self { Self :: Enabled => "Enabled" , Self :: Disabled => "Disabled" , } } } impl TryFrom < & str > for SseKmsEncryptedObjectsStatus { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "Enabled" => Ok (Self :: Enabled) , "Disabled" => Ok (Self :: Disabled) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum S3Ops { PutBucketMetricsConfiguration , DeleteBucketMetricsConfiguration , DeleteObjectTagging , DeletePublicAccessBlock , GetBucketInventoryConfiguration , PutBucketLogging , CreateMultipartUpload , GetObject , UploadPart , GetBucketAccelerateConfiguration , GetBucketPolicy , DeleteBucketCors , CreateBucket , ListObjectVersions , DeleteBucketWebsite , GetBucketAnalyticsConfiguration , ListObjectsV2 , GetBucketIntelligentTieringConfiguration , GetBucketAcl , GetPublicAccessBlock , GetObjectTagging , GetBucketNotificationConfiguration , PutObjectLockConfiguration , PutBucketIntelligentTieringConfiguration , DeleteObject , ListBuckets , PutBucketAnalyticsConfiguration , HeadBucket , DeleteBucketInventoryConfiguration , DeleteObjects , ListParts , PutBucketInventoryConfiguration , GetBucketTagging , DeleteBucketIntelligentTieringConfiguration , PutObjectRetention , PutBucketVersioning , PutBucketRequestPayment , GetBucketPolicyStatus , PutBucketLifecycleConfiguration , DeleteBucketOwnershipControls , GetBucketReplication , GetBucketLifecycleConfiguration , AbortMultipartUpload , GetObjectRetention , GetBucketVersioning , PutBucketNotificationConfiguration , ListBucketIntelligentTieringConfigurations , GetBucketRequestPayment , DeleteBucketReplication , DeleteBucketEncryption , PutObjectLegalHold , PutPublicAccessBlock , PutBucketTagging , PutBucketWebsite , UploadPartCopy , PutBucketPolicy , GetBucketEncryption , PutBucketAccelerateConfiguration , GetBucketLocation , PutObjectAcl , ListBucketAnalyticsConfigurations , CopyObject , GetBucketCors , GetObjectLegalHold , WriteGetObjectResponse , PutBucketOwnershipControls , PutBucketAcl , GetBucketMetricsConfiguration , GetObjectAttributes , DeleteBucketAnalyticsConfiguration , CompleteMultipartUpload , GetBucketOwnershipControls , GetObjectLockConfiguration , HeadObject , ListBucketMetricsConfigurations , PutBucketReplication , ListObjects , DeleteBucket , GetObjectAcl , RestoreObject , PutBucketCors , DeleteBucketPolicy , ListBucketInventoryConfigurations , PutObjectTagging , PutBucketEncryption , PutObject , GetBucketLogging , DeleteBucketTagging , GetBucketWebsite , ListMultipartUploads , GetObjectTorrent , DeleteBucketLifecycle } # [doc = r" This macro calls a provided $macro for each S3 operation to generate code per op."] macro_rules ! generate_ops_code { ($ macro : ident) => { $ macro ! (PutBucketMetricsConfiguration) ; $ macro ! (DeleteBucketMetricsConfiguration) ; $ macro ! (DeleteObjectTagging) ; $ macro ! (DeletePublicAccessBlock) ; $ macro ! (GetBucketInventoryConfiguration) ; $ macro ! (PutBucketLogging) ; $ macro ! (CreateMultipartUpload) ; $ macro ! (GetObject) ; $ macro ! (UploadPart) ; $ macro ! (GetBucketAccelerateConfiguration) ; $ macro ! (GetBucketPolicy) ; $ macro ! (DeleteBucketCors) ; $ macro ! (CreateBucket) ; $ macro ! (ListObjectVersions) ; $ macro ! (DeleteBucketWebsite) ; $ macro ! (GetBucketAnalyticsConfiguration) ; $ macro ! (ListObjectsV2) ; $ macro ! (GetBucketIntelligentTieringConfiguration) ; $ macro ! (GetBucketAcl) ; $ macro ! (GetPublicAccessBlock) ; $ macro ! (GetObjectTagging) ; $ macro ! (GetBucketNotificationConfiguration) ; $ macro ! (PutObjectLockConfiguration) ; $ macro ! (PutBucketIntelligentTieringConfiguration) ; $ macro ! (DeleteObject) ; $ macro ! (ListBuckets) ; $ macro ! (PutBucketAnalyticsConfiguration) ; $ macro ! (HeadBucket) ; $ macro ! (DeleteBucketInventoryConfiguration) ; $ macro ! (DeleteObjects) ; $ macro ! (ListParts) ; $ macro ! (PutBucketInventoryConfiguration) ; $ macro ! (GetBucketTagging) ; $ macro ! (DeleteBucketIntelligentTieringConfiguration) ; $ macro ! (PutObjectRetention) ; $ macro ! (PutBucketVersioning) ; $ macro ! (PutBucketRequestPayment) ; $ macro ! (GetBucketPolicyStatus) ; $ macro ! (PutBucketLifecycleConfiguration) ; $ macro ! (DeleteBucketOwnershipControls) ; $ macro ! (GetBucketReplication) ; $ macro ! (GetBucketLifecycleConfiguration) ; $ macro ! (AbortMultipartUpload) ; $ macro ! (GetObjectRetention) ; $ macro ! (GetBucketVersioning) ; $ macro ! (PutBucketNotificationConfiguration) ; $ macro ! (ListBucketIntelligentTieringConfigurations) ; $ macro ! (GetBucketRequestPayment) ; $ macro ! (DeleteBucketReplication) ; $ macro ! (DeleteBucketEncryption) ; $ macro ! (PutObjectLegalHold) ; $ macro ! (PutPublicAccessBlock) ; $ macro ! (PutBucketTagging) ; $ macro ! (PutBucketWebsite) ; $ macro ! (UploadPartCopy) ; $ macro ! (PutBucketPolicy) ; $ macro ! (GetBucketEncryption) ; $ macro ! (PutBucketAccelerateConfiguration) ; $ macro ! (GetBucketLocation) ; $ macro ! (PutObjectAcl) ; $ macro ! (ListBucketAnalyticsConfigurations) ; $ macro ! (CopyObject) ; $ macro ! (GetBucketCors) ; $ macro ! (GetObjectLegalHold) ; $ macro ! (WriteGetObjectResponse) ; $ macro ! (PutBucketOwnershipControls) ; $ macro ! (PutBucketAcl) ; $ macro ! (GetBucketMetricsConfiguration) ; $ macro ! (GetObjectAttributes) ; $ macro ! (DeleteBucketAnalyticsConfiguration) ; $ macro ! (CompleteMultipartUpload) ; $ macro ! (GetBucketOwnershipControls) ; $ macro ! (GetObjectLockConfiguration) ; $ macro ! (HeadObject) ; $ macro ! (ListBucketMetricsConfigurations) ; $ macro ! (PutBucketReplication) ; $ macro ! (ListObjects) ; $ macro ! (DeleteBucket) ; $ macro ! (GetObjectAcl) ; $ macro ! (RestoreObject) ; $ macro ! (PutBucketCors) ; $ macro ! (DeleteBucketPolicy) ; $ macro ! (ListBucketInventoryConfigurations) ; $ macro ! (PutObjectTagging) ; $ macro ! (PutBucketEncryption) ; $ macro ! (PutObject) ; $ macro ! (GetBucketLogging) ; $ macro ! (DeleteBucketTagging) ; $ macro ! (GetBucketWebsite) ; $ macro ! (ListMultipartUploads) ; $ macro ! (GetObjectTorrent) ; $ macro ! (DeleteBucketLifecycle) ; } } # [doc = r" This macro calls a provided $macro for each S3 operation to generate item per op."] macro_rules ! generate_ops_items { ($ macro : ident) => { $ macro ! (PutBucketMetricsConfiguration) , $ macro ! (DeleteBucketMetricsConfiguration) , $ macro ! (DeleteObjectTagging) , $ macro ! (DeletePublicAccessBlock) , $ macro ! (GetBucketInventoryConfiguration) , $ macro ! (PutBucketLogging) , $ macro ! (CreateMultipartUpload) , $ macro ! (GetObject) , $ macro ! (UploadPart) , $ macro ! (GetBucketAccelerateConfiguration) , $ macro ! (GetBucketPolicy) , $ macro ! (DeleteBucketCors) , $ macro ! (CreateBucket) , $ macro ! (ListObjectVersions) , $ macro ! (DeleteBucketWebsite) , $ macro ! (GetBucketAnalyticsConfiguration) , $ macro ! (ListObjectsV2) , $ macro ! (GetBucketIntelligentTieringConfiguration) , $ macro ! (GetBucketAcl) , $ macro ! (GetPublicAccessBlock) , $ macro ! (GetObjectTagging) , $ macro ! (GetBucketNotificationConfiguration) , $ macro ! (PutObjectLockConfiguration) , $ macro ! (PutBucketIntelligentTieringConfiguration) , $ macro ! (DeleteObject) , $ macro ! (ListBuckets) , $ macro ! (PutBucketAnalyticsConfiguration) , $ macro ! (HeadBucket) , $ macro ! (DeleteBucketInventoryConfiguration) , $ macro ! (DeleteObjects) , $ macro ! (ListParts) , $ macro ! (PutBucketInventoryConfiguration) , $ macro ! (GetBucketTagging) , $ macro ! (DeleteBucketIntelligentTieringConfiguration) , $ macro ! (PutObjectRetention) , $ macro ! (PutBucketVersioning) , $ macro ! (PutBucketRequestPayment) , $ macro ! (GetBucketPolicyStatus) , $ macro ! (PutBucketLifecycleConfiguration) , $ macro ! (DeleteBucketOwnershipControls) , $ macro ! (GetBucketReplication) , $ macro ! (GetBucketLifecycleConfiguration) , $ macro ! (AbortMultipartUpload) , $ macro ! (GetObjectRetention) , $ macro ! (GetBucketVersioning) , $ macro ! (PutBucketNotificationConfiguration) , $ macro ! (ListBucketIntelligentTieringConfigurations) , $ macro ! (GetBucketRequestPayment) , $ macro ! (DeleteBucketReplication) , $ macro ! (DeleteBucketEncryption) , $ macro ! (PutObjectLegalHold) , $ macro ! (PutPublicAccessBlock) , $ macro ! (PutBucketTagging) , $ macro ! (PutBucketWebsite) , $ macro ! (UploadPartCopy) , $ macro ! (PutBucketPolicy) , $ macro ! (GetBucketEncryption) , $ macro ! (PutBucketAccelerateConfiguration) , $ macro ! (GetBucketLocation) , $ macro ! (PutObjectAcl) , $ macro ! (ListBucketAnalyticsConfigurations) , $ macro ! (CopyObject) , $ macro ! (GetBucketCors) , $ macro ! (GetObjectLegalHold) , $ macro ! (WriteGetObjectResponse) , $ macro ! (PutBucketOwnershipControls) , $ macro ! (PutBucketAcl) , $ macro ! (GetBucketMetricsConfiguration) , $ macro ! (GetObjectAttributes) , $ macro ! (DeleteBucketAnalyticsConfiguration) , $ macro ! (CompleteMultipartUpload) , $ macro ! (GetBucketOwnershipControls) , $ macro ! (GetObjectLockConfiguration) , $ macro ! (HeadObject) , $ macro ! (ListBucketMetricsConfigurations) , $ macro ! (PutBucketReplication) , $ macro ! (ListObjects) , $ macro ! (DeleteBucket) , $ macro ! (GetObjectAcl) , $ macro ! (RestoreObject) , $ macro ! (PutBucketCors) , $ macro ! (DeleteBucketPolicy) , $ macro ! (ListBucketInventoryConfigurations) , $ macro ! (PutObjectTagging) , $ macro ! (PutBucketEncryption) , $ macro ! (PutObject) , $ macro ! (GetBucketLogging) , $ macro ! (DeleteBucketTagging) , $ macro ! (GetBucketWebsite) , $ macro ! (ListMultipartUploads) , $ macro ! (GetObjectTorrent) , $ macro ! (DeleteBucketLifecycle) , } } # [doc = r" This macro matches a variable of S3Ops type and expands the provided $macro"] # [doc = r" for each S3 operation to generate code handler per op."] macro_rules ! generate_ops_match { ($ macro : ident , $ op : expr) => { match ($ op) { S3Ops :: PutBucketMetricsConfiguration => $ macro ! (PutBucketMetricsConfiguration) , S3Ops :: DeleteBucketMetricsConfiguration => $ macro ! (DeleteBucketMetricsConfiguration) , S3Ops :: DeleteObjectTagging => $ macro ! (DeleteObjectTagging) , S3Ops :: DeletePublicAccessBlock => $ macro ! (DeletePublicAccessBlock) , S3Ops :: GetBucketInventoryConfiguration => $ macro ! (GetBucketInventoryConfiguration) , S3Ops :: PutBucketLogging => $ macro ! (PutBucketLogging) , S3Ops :: CreateMultipartUpload => $ macro ! (CreateMultipartUpload) , S3Ops :: GetObject => $ macro ! (GetObject) , S3Ops :: UploadPart => $ macro ! (UploadPart) , S3Ops :: GetBucketAccelerateConfiguration => $ macro ! (GetBucketAccelerateConfiguration) , S3Ops :: GetBucketPolicy => $ macro ! (GetBucketPolicy) , S3Ops :: DeleteBucketCors => $ macro ! (DeleteBucketCors) , S3Ops :: CreateBucket => $ macro ! (CreateBucket) , S3Ops :: ListObjectVersions => $ macro ! (ListObjectVersions) , S3Ops :: DeleteBucketWebsite => $ macro ! (DeleteBucketWebsite) , S3Ops :: GetBucketAnalyticsConfiguration => $ macro ! (GetBucketAnalyticsConfiguration) , S3Ops :: ListObjectsV2 => $ macro ! (ListObjectsV2) , S3Ops :: GetBucketIntelligentTieringConfiguration => $ macro ! (GetBucketIntelligentTieringConfiguration) , S3Ops :: GetBucketAcl => $ macro ! (GetBucketAcl) , S3Ops :: GetPublicAccessBlock => $ macro ! (GetPublicAccessBlock) , S3Ops :: GetObjectTagging => $ macro ! (GetObjectTagging) , S3Ops :: GetBucketNotificationConfiguration => $ macro ! (GetBucketNotificationConfiguration) , S3Ops :: PutObjectLockConfiguration => $ macro ! (PutObjectLockConfiguration) , S3Ops :: PutBucketIntelligentTieringConfiguration => $ macro ! (PutBucketIntelligentTieringConfiguration) , S3Ops :: DeleteObject => $ macro ! (DeleteObject) , S3Ops :: ListBuckets => $ macro ! (ListBuckets) , S3Ops :: PutBucketAnalyticsConfiguration => $ macro ! (PutBucketAnalyticsConfiguration) , S3Ops :: HeadBucket => $ macro ! (HeadBucket) , S3Ops :: DeleteBucketInventoryConfiguration => $ macro ! (DeleteBucketInventoryConfiguration) , S3Ops :: DeleteObjects => $ macro ! (DeleteObjects) , S3Ops :: ListParts => $ macro ! (ListParts) , S3Ops :: PutBucketInventoryConfiguration => $ macro ! (PutBucketInventoryConfiguration) , S3Ops :: GetBucketTagging => $ macro ! (GetBucketTagging) , S3Ops :: DeleteBucketIntelligentTieringConfiguration => $ macro ! (DeleteBucketIntelligentTieringConfiguration) , S3Ops :: PutObjectRetention => $ macro ! (PutObjectRetention) , S3Ops :: PutBucketVersioning => $ macro ! (PutBucketVersioning) , S3Ops :: PutBucketRequestPayment => $ macro ! (PutBucketRequestPayment) , S3Ops :: GetBucketPolicyStatus => $ macro ! (GetBucketPolicyStatus) , S3Ops :: PutBucketLifecycleConfiguration => $ macro ! (PutBucketLifecycleConfiguration) , S3Ops :: DeleteBucketOwnershipControls => $ macro ! (DeleteBucketOwnershipControls) , S3Ops :: GetBucketReplication => $ macro ! (GetBucketReplication) , S3Ops :: GetBucketLifecycleConfiguration => $ macro ! (GetBucketLifecycleConfiguration) , S3Ops :: AbortMultipartUpload => $ macro ! (AbortMultipartUpload) , S3Ops :: GetObjectRetention => $ macro ! (GetObjectRetention) , S3Ops :: GetBucketVersioning => $ macro ! (GetBucketVersioning) , S3Ops :: PutBucketNotificationConfiguration => $ macro ! (PutBucketNotificationConfiguration) , S3Ops :: ListBucketIntelligentTieringConfigurations => $ macro ! (ListBucketIntelligentTieringConfigurations) , S3Ops :: GetBucketRequestPayment => $ macro ! (GetBucketRequestPayment) , S3Ops :: DeleteBucketReplication => $ macro ! (DeleteBucketReplication) , S3Ops :: DeleteBucketEncryption => $ macro ! (DeleteBucketEncryption) , S3Ops :: PutObjectLegalHold => $ macro ! (PutObjectLegalHold) , S3Ops :: PutPublicAccessBlock => $ macro ! (PutPublicAccessBlock) , S3Ops :: PutBucketTagging => $ macro ! (PutBucketTagging) , S3Ops :: PutBucketWebsite => $ macro ! (PutBucketWebsite) , S3Ops :: UploadPartCopy => $ macro ! (UploadPartCopy) , S3Ops :: PutBucketPolicy => $ macro ! (PutBucketPolicy) , S3Ops :: GetBucketEncryption => $ macro ! (GetBucketEncryption) , S3Ops :: PutBucketAccelerateConfiguration => $ macro ! (PutBucketAccelerateConfiguration) , S3Ops :: GetBucketLocation => $ macro ! (GetBucketLocation) , S3Ops :: PutObjectAcl => $ macro ! (PutObjectAcl) , S3Ops :: ListBucketAnalyticsConfigurations => $ macro ! (ListBucketAnalyticsConfigurations) , S3Ops :: CopyObject => $ macro ! (CopyObject) , S3Ops :: GetBucketCors => $ macro ! (GetBucketCors) , S3Ops :: GetObjectLegalHold => $ macro ! (GetObjectLegalHold) , S3Ops :: WriteGetObjectResponse => $ macro ! (WriteGetObjectResponse) , S3Ops :: PutBucketOwnershipControls => $ macro ! (PutBucketOwnershipControls) , S3Ops :: PutBucketAcl => $ macro ! (PutBucketAcl) , S3Ops :: GetBucketMetricsConfiguration => $ macro ! (GetBucketMetricsConfiguration) , S3Ops :: GetObjectAttributes => $ macro ! (GetObjectAttributes) , S3Ops :: DeleteBucketAnalyticsConfiguration => $ macro ! (DeleteBucketAnalyticsConfiguration) , S3Ops :: CompleteMultipartUpload => $ macro ! (CompleteMultipartUpload) , S3Ops :: GetBucketOwnershipControls => $ macro ! (GetBucketOwnershipControls) , S3Ops :: GetObjectLockConfiguration => $ macro ! (GetObjectLockConfiguration) , S3Ops :: HeadObject => $ macro ! (HeadObject) , S3Ops :: ListBucketMetricsConfigurations => $ macro ! (ListBucketMetricsConfigurations) , S3Ops :: PutBucketReplication => $ macro ! (PutBucketReplication) , S3Ops :: ListObjects => $ macro ! (ListObjects) , S3Ops :: DeleteBucket => $ macro ! (DeleteBucket) , S3Ops :: GetObjectAcl => $ macro ! (GetObjectAcl) , S3Ops :: RestoreObject => $ macro ! (RestoreObject) , S3Ops :: PutBucketCors => $ macro ! (PutBucketCors) , S3Ops :: DeleteBucketPolicy => $ macro ! (DeleteBucketPolicy) , S3Ops :: ListBucketInventoryConfigurations => $ macro ! (ListBucketInventoryConfigurations) , S3Ops :: PutObjectTagging => $ macro ! (PutObjectTagging) , S3Ops :: PutBucketEncryption => $ macro ! (PutBucketEncryption) , S3Ops :: PutObject => $ macro ! (PutObject) , S3Ops :: GetBucketLogging => $ macro ! (GetBucketLogging) , S3Ops :: DeleteBucketTagging => $ macro ! (DeleteBucketTagging) , S3Ops :: GetBucketWebsite => $ macro ! (GetBucketWebsite) , S3Ops :: ListMultipartUploads => $ macro ! (ListMultipartUploads) , S3Ops :: GetObjectTorrent => $ macro ! (GetObjectTorrent) , S3Ops :: DeleteBucketLifecycle => $ macro ! (DeleteBucketLifecycle) , } } } pub (crate) use generate_ops_code ; pub (crate) use generate_ops_items ; pub (crate) use generate_ops_match ;

