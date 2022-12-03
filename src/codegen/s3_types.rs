use std :: str :: FromStr ; use std :: collections :: HashMap ; use std :: collections :: HashSet ; trait Op { type Input ; type Output ; }

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum S3Ops { CompleteMultipartUpload , DeleteBucketWebsite , RestoreObject , DeleteBucketTagging , WriteGetObjectResponse , GetBucketInventoryConfiguration , GetObjectTorrent , PutBucketVersioning , GetObjectLegalHold , PutBucketInventoryConfiguration , CopyObject , ListBucketAnalyticsConfigurations , GetBucketVersioning , PutBucketCors , GetBucketCors , ListBucketIntelligentTieringConfigurations , GetBucketAccelerateConfiguration , PutBucketIntelligentTieringConfiguration , PutBucketTagging , GetBucketLogging , ListObjects , HeadBucket , GetPublicAccessBlock , PutBucketAccelerateConfiguration , DeleteObjectTagging , DeleteBucketEncryption , GetBucketOwnershipControls , GetObjectAttributes , PutBucketLifecycleConfiguration , GetObjectTagging , CreateBucket , DeleteBucketCors , PutObject , UploadPart , PutObjectRetention , PutObjectAcl , PutObjectTagging , UploadPartCopy , PutBucketNotificationConfiguration , DeleteObject , AbortMultipartUpload , GetBucketIntelligentTieringConfiguration , PutBucketMetricsConfiguration , GetObjectRetention , ListObjectVersions , DeleteBucketPolicy , PutObjectLockConfiguration , ListBuckets , CreateMultipartUpload , HeadObject , GetObject , DeleteBucketInventoryConfiguration , GetBucketPolicyStatus , DeleteBucketLifecycle , PutBucketEncryption , DeleteBucketOwnershipControls , ListMultipartUploads , DeleteBucketAnalyticsConfiguration , DeleteBucketMetricsConfiguration , PutPublicAccessBlock , GetBucketAnalyticsConfiguration , GetBucketAcl , GetBucketMetricsConfiguration , DeleteObjects , GetBucketWebsite , GetBucketRequestPayment , GetObjectAcl , ListObjectsV2 , PutBucketLogging , PutBucketWebsite , GetBucketLocation , GetBucketEncryption , GetBucketReplication , GetObjectLockConfiguration , PutBucketPolicy , DeleteBucket , ListBucketInventoryConfigurations , PutBucketReplication , GetBucketNotificationConfiguration , PutBucketAnalyticsConfiguration , PutBucketRequestPayment , DeleteBucketIntelligentTieringConfiguration , ListParts , GetBucketTagging , DeletePublicAccessBlock , ListBucketMetricsConfigurations , PutBucketOwnershipControls , DeleteBucketReplication , GetBucketPolicy , GetBucketLifecycleConfiguration , PutBucketAcl , PutObjectLegalHold } # [doc = r" This macro calls a provided $macro for each S3 operation to generate code per op."] macro_rules ! generate_ops_code { ($ macro : ident) => { $ macro ! (CompleteMultipartUpload) ; $ macro ! (DeleteBucketWebsite) ; $ macro ! (RestoreObject) ; $ macro ! (DeleteBucketTagging) ; $ macro ! (WriteGetObjectResponse) ; $ macro ! (GetBucketInventoryConfiguration) ; $ macro ! (GetObjectTorrent) ; $ macro ! (PutBucketVersioning) ; $ macro ! (GetObjectLegalHold) ; $ macro ! (PutBucketInventoryConfiguration) ; $ macro ! (CopyObject) ; $ macro ! (ListBucketAnalyticsConfigurations) ; $ macro ! (GetBucketVersioning) ; $ macro ! (PutBucketCors) ; $ macro ! (GetBucketCors) ; $ macro ! (ListBucketIntelligentTieringConfigurations) ; $ macro ! (GetBucketAccelerateConfiguration) ; $ macro ! (PutBucketIntelligentTieringConfiguration) ; $ macro ! (PutBucketTagging) ; $ macro ! (GetBucketLogging) ; $ macro ! (ListObjects) ; $ macro ! (HeadBucket) ; $ macro ! (GetPublicAccessBlock) ; $ macro ! (PutBucketAccelerateConfiguration) ; $ macro ! (DeleteObjectTagging) ; $ macro ! (DeleteBucketEncryption) ; $ macro ! (GetBucketOwnershipControls) ; $ macro ! (GetObjectAttributes) ; $ macro ! (PutBucketLifecycleConfiguration) ; $ macro ! (GetObjectTagging) ; $ macro ! (CreateBucket) ; $ macro ! (DeleteBucketCors) ; $ macro ! (PutObject) ; $ macro ! (UploadPart) ; $ macro ! (PutObjectRetention) ; $ macro ! (PutObjectAcl) ; $ macro ! (PutObjectTagging) ; $ macro ! (UploadPartCopy) ; $ macro ! (PutBucketNotificationConfiguration) ; $ macro ! (DeleteObject) ; $ macro ! (AbortMultipartUpload) ; $ macro ! (GetBucketIntelligentTieringConfiguration) ; $ macro ! (PutBucketMetricsConfiguration) ; $ macro ! (GetObjectRetention) ; $ macro ! (ListObjectVersions) ; $ macro ! (DeleteBucketPolicy) ; $ macro ! (PutObjectLockConfiguration) ; $ macro ! (ListBuckets) ; $ macro ! (CreateMultipartUpload) ; $ macro ! (HeadObject) ; $ macro ! (GetObject) ; $ macro ! (DeleteBucketInventoryConfiguration) ; $ macro ! (GetBucketPolicyStatus) ; $ macro ! (DeleteBucketLifecycle) ; $ macro ! (PutBucketEncryption) ; $ macro ! (DeleteBucketOwnershipControls) ; $ macro ! (ListMultipartUploads) ; $ macro ! (DeleteBucketAnalyticsConfiguration) ; $ macro ! (DeleteBucketMetricsConfiguration) ; $ macro ! (PutPublicAccessBlock) ; $ macro ! (GetBucketAnalyticsConfiguration) ; $ macro ! (GetBucketAcl) ; $ macro ! (GetBucketMetricsConfiguration) ; $ macro ! (DeleteObjects) ; $ macro ! (GetBucketWebsite) ; $ macro ! (GetBucketRequestPayment) ; $ macro ! (GetObjectAcl) ; $ macro ! (ListObjectsV2) ; $ macro ! (PutBucketLogging) ; $ macro ! (PutBucketWebsite) ; $ macro ! (GetBucketLocation) ; $ macro ! (GetBucketEncryption) ; $ macro ! (GetBucketReplication) ; $ macro ! (GetObjectLockConfiguration) ; $ macro ! (PutBucketPolicy) ; $ macro ! (DeleteBucket) ; $ macro ! (ListBucketInventoryConfigurations) ; $ macro ! (PutBucketReplication) ; $ macro ! (GetBucketNotificationConfiguration) ; $ macro ! (PutBucketAnalyticsConfiguration) ; $ macro ! (PutBucketRequestPayment) ; $ macro ! (DeleteBucketIntelligentTieringConfiguration) ; $ macro ! (ListParts) ; $ macro ! (GetBucketTagging) ; $ macro ! (DeletePublicAccessBlock) ; $ macro ! (ListBucketMetricsConfigurations) ; $ macro ! (PutBucketOwnershipControls) ; $ macro ! (DeleteBucketReplication) ; $ macro ! (GetBucketPolicy) ; $ macro ! (GetBucketLifecycleConfiguration) ; $ macro ! (PutBucketAcl) ; $ macro ! (PutObjectLegalHold) ; } } # [doc = r" This macro calls a provided $macro for each S3 operation to generate item per op."] macro_rules ! generate_ops_items { ($ macro : ident) => { $ macro ! (CompleteMultipartUpload) , $ macro ! (DeleteBucketWebsite) , $ macro ! (RestoreObject) , $ macro ! (DeleteBucketTagging) , $ macro ! (WriteGetObjectResponse) , $ macro ! (GetBucketInventoryConfiguration) , $ macro ! (GetObjectTorrent) , $ macro ! (PutBucketVersioning) , $ macro ! (GetObjectLegalHold) , $ macro ! (PutBucketInventoryConfiguration) , $ macro ! (CopyObject) , $ macro ! (ListBucketAnalyticsConfigurations) , $ macro ! (GetBucketVersioning) , $ macro ! (PutBucketCors) , $ macro ! (GetBucketCors) , $ macro ! (ListBucketIntelligentTieringConfigurations) , $ macro ! (GetBucketAccelerateConfiguration) , $ macro ! (PutBucketIntelligentTieringConfiguration) , $ macro ! (PutBucketTagging) , $ macro ! (GetBucketLogging) , $ macro ! (ListObjects) , $ macro ! (HeadBucket) , $ macro ! (GetPublicAccessBlock) , $ macro ! (PutBucketAccelerateConfiguration) , $ macro ! (DeleteObjectTagging) , $ macro ! (DeleteBucketEncryption) , $ macro ! (GetBucketOwnershipControls) , $ macro ! (GetObjectAttributes) , $ macro ! (PutBucketLifecycleConfiguration) , $ macro ! (GetObjectTagging) , $ macro ! (CreateBucket) , $ macro ! (DeleteBucketCors) , $ macro ! (PutObject) , $ macro ! (UploadPart) , $ macro ! (PutObjectRetention) , $ macro ! (PutObjectAcl) , $ macro ! (PutObjectTagging) , $ macro ! (UploadPartCopy) , $ macro ! (PutBucketNotificationConfiguration) , $ macro ! (DeleteObject) , $ macro ! (AbortMultipartUpload) , $ macro ! (GetBucketIntelligentTieringConfiguration) , $ macro ! (PutBucketMetricsConfiguration) , $ macro ! (GetObjectRetention) , $ macro ! (ListObjectVersions) , $ macro ! (DeleteBucketPolicy) , $ macro ! (PutObjectLockConfiguration) , $ macro ! (ListBuckets) , $ macro ! (CreateMultipartUpload) , $ macro ! (HeadObject) , $ macro ! (GetObject) , $ macro ! (DeleteBucketInventoryConfiguration) , $ macro ! (GetBucketPolicyStatus) , $ macro ! (DeleteBucketLifecycle) , $ macro ! (PutBucketEncryption) , $ macro ! (DeleteBucketOwnershipControls) , $ macro ! (ListMultipartUploads) , $ macro ! (DeleteBucketAnalyticsConfiguration) , $ macro ! (DeleteBucketMetricsConfiguration) , $ macro ! (PutPublicAccessBlock) , $ macro ! (GetBucketAnalyticsConfiguration) , $ macro ! (GetBucketAcl) , $ macro ! (GetBucketMetricsConfiguration) , $ macro ! (DeleteObjects) , $ macro ! (GetBucketWebsite) , $ macro ! (GetBucketRequestPayment) , $ macro ! (GetObjectAcl) , $ macro ! (ListObjectsV2) , $ macro ! (PutBucketLogging) , $ macro ! (PutBucketWebsite) , $ macro ! (GetBucketLocation) , $ macro ! (GetBucketEncryption) , $ macro ! (GetBucketReplication) , $ macro ! (GetObjectLockConfiguration) , $ macro ! (PutBucketPolicy) , $ macro ! (DeleteBucket) , $ macro ! (ListBucketInventoryConfigurations) , $ macro ! (PutBucketReplication) , $ macro ! (GetBucketNotificationConfiguration) , $ macro ! (PutBucketAnalyticsConfiguration) , $ macro ! (PutBucketRequestPayment) , $ macro ! (DeleteBucketIntelligentTieringConfiguration) , $ macro ! (ListParts) , $ macro ! (GetBucketTagging) , $ macro ! (DeletePublicAccessBlock) , $ macro ! (ListBucketMetricsConfigurations) , $ macro ! (PutBucketOwnershipControls) , $ macro ! (DeleteBucketReplication) , $ macro ! (GetBucketPolicy) , $ macro ! (GetBucketLifecycleConfiguration) , $ macro ! (PutBucketAcl) , $ macro ! (PutObjectLegalHold) , } } # [doc = r" This macro matches a variable of S3Ops type and expands the provided $macro"] # [doc = r" for each S3 operation to generate code handler per op."] macro_rules ! generate_ops_match { ($ macro : ident , $ op : expr) => { match ($ op) { S3Ops :: CompleteMultipartUpload => $ macro ! (CompleteMultipartUpload) , S3Ops :: DeleteBucketWebsite => $ macro ! (DeleteBucketWebsite) , S3Ops :: RestoreObject => $ macro ! (RestoreObject) , S3Ops :: DeleteBucketTagging => $ macro ! (DeleteBucketTagging) , S3Ops :: WriteGetObjectResponse => $ macro ! (WriteGetObjectResponse) , S3Ops :: GetBucketInventoryConfiguration => $ macro ! (GetBucketInventoryConfiguration) , S3Ops :: GetObjectTorrent => $ macro ! (GetObjectTorrent) , S3Ops :: PutBucketVersioning => $ macro ! (PutBucketVersioning) , S3Ops :: GetObjectLegalHold => $ macro ! (GetObjectLegalHold) , S3Ops :: PutBucketInventoryConfiguration => $ macro ! (PutBucketInventoryConfiguration) , S3Ops :: CopyObject => $ macro ! (CopyObject) , S3Ops :: ListBucketAnalyticsConfigurations => $ macro ! (ListBucketAnalyticsConfigurations) , S3Ops :: GetBucketVersioning => $ macro ! (GetBucketVersioning) , S3Ops :: PutBucketCors => $ macro ! (PutBucketCors) , S3Ops :: GetBucketCors => $ macro ! (GetBucketCors) , S3Ops :: ListBucketIntelligentTieringConfigurations => $ macro ! (ListBucketIntelligentTieringConfigurations) , S3Ops :: GetBucketAccelerateConfiguration => $ macro ! (GetBucketAccelerateConfiguration) , S3Ops :: PutBucketIntelligentTieringConfiguration => $ macro ! (PutBucketIntelligentTieringConfiguration) , S3Ops :: PutBucketTagging => $ macro ! (PutBucketTagging) , S3Ops :: GetBucketLogging => $ macro ! (GetBucketLogging) , S3Ops :: ListObjects => $ macro ! (ListObjects) , S3Ops :: HeadBucket => $ macro ! (HeadBucket) , S3Ops :: GetPublicAccessBlock => $ macro ! (GetPublicAccessBlock) , S3Ops :: PutBucketAccelerateConfiguration => $ macro ! (PutBucketAccelerateConfiguration) , S3Ops :: DeleteObjectTagging => $ macro ! (DeleteObjectTagging) , S3Ops :: DeleteBucketEncryption => $ macro ! (DeleteBucketEncryption) , S3Ops :: GetBucketOwnershipControls => $ macro ! (GetBucketOwnershipControls) , S3Ops :: GetObjectAttributes => $ macro ! (GetObjectAttributes) , S3Ops :: PutBucketLifecycleConfiguration => $ macro ! (PutBucketLifecycleConfiguration) , S3Ops :: GetObjectTagging => $ macro ! (GetObjectTagging) , S3Ops :: CreateBucket => $ macro ! (CreateBucket) , S3Ops :: DeleteBucketCors => $ macro ! (DeleteBucketCors) , S3Ops :: PutObject => $ macro ! (PutObject) , S3Ops :: UploadPart => $ macro ! (UploadPart) , S3Ops :: PutObjectRetention => $ macro ! (PutObjectRetention) , S3Ops :: PutObjectAcl => $ macro ! (PutObjectAcl) , S3Ops :: PutObjectTagging => $ macro ! (PutObjectTagging) , S3Ops :: UploadPartCopy => $ macro ! (UploadPartCopy) , S3Ops :: PutBucketNotificationConfiguration => $ macro ! (PutBucketNotificationConfiguration) , S3Ops :: DeleteObject => $ macro ! (DeleteObject) , S3Ops :: AbortMultipartUpload => $ macro ! (AbortMultipartUpload) , S3Ops :: GetBucketIntelligentTieringConfiguration => $ macro ! (GetBucketIntelligentTieringConfiguration) , S3Ops :: PutBucketMetricsConfiguration => $ macro ! (PutBucketMetricsConfiguration) , S3Ops :: GetObjectRetention => $ macro ! (GetObjectRetention) , S3Ops :: ListObjectVersions => $ macro ! (ListObjectVersions) , S3Ops :: DeleteBucketPolicy => $ macro ! (DeleteBucketPolicy) , S3Ops :: PutObjectLockConfiguration => $ macro ! (PutObjectLockConfiguration) , S3Ops :: ListBuckets => $ macro ! (ListBuckets) , S3Ops :: CreateMultipartUpload => $ macro ! (CreateMultipartUpload) , S3Ops :: HeadObject => $ macro ! (HeadObject) , S3Ops :: GetObject => $ macro ! (GetObject) , S3Ops :: DeleteBucketInventoryConfiguration => $ macro ! (DeleteBucketInventoryConfiguration) , S3Ops :: GetBucketPolicyStatus => $ macro ! (GetBucketPolicyStatus) , S3Ops :: DeleteBucketLifecycle => $ macro ! (DeleteBucketLifecycle) , S3Ops :: PutBucketEncryption => $ macro ! (PutBucketEncryption) , S3Ops :: DeleteBucketOwnershipControls => $ macro ! (DeleteBucketOwnershipControls) , S3Ops :: ListMultipartUploads => $ macro ! (ListMultipartUploads) , S3Ops :: DeleteBucketAnalyticsConfiguration => $ macro ! (DeleteBucketAnalyticsConfiguration) , S3Ops :: DeleteBucketMetricsConfiguration => $ macro ! (DeleteBucketMetricsConfiguration) , S3Ops :: PutPublicAccessBlock => $ macro ! (PutPublicAccessBlock) , S3Ops :: GetBucketAnalyticsConfiguration => $ macro ! (GetBucketAnalyticsConfiguration) , S3Ops :: GetBucketAcl => $ macro ! (GetBucketAcl) , S3Ops :: GetBucketMetricsConfiguration => $ macro ! (GetBucketMetricsConfiguration) , S3Ops :: DeleteObjects => $ macro ! (DeleteObjects) , S3Ops :: GetBucketWebsite => $ macro ! (GetBucketWebsite) , S3Ops :: GetBucketRequestPayment => $ macro ! (GetBucketRequestPayment) , S3Ops :: GetObjectAcl => $ macro ! (GetObjectAcl) , S3Ops :: ListObjectsV2 => $ macro ! (ListObjectsV2) , S3Ops :: PutBucketLogging => $ macro ! (PutBucketLogging) , S3Ops :: PutBucketWebsite => $ macro ! (PutBucketWebsite) , S3Ops :: GetBucketLocation => $ macro ! (GetBucketLocation) , S3Ops :: GetBucketEncryption => $ macro ! (GetBucketEncryption) , S3Ops :: GetBucketReplication => $ macro ! (GetBucketReplication) , S3Ops :: GetObjectLockConfiguration => $ macro ! (GetObjectLockConfiguration) , S3Ops :: PutBucketPolicy => $ macro ! (PutBucketPolicy) , S3Ops :: DeleteBucket => $ macro ! (DeleteBucket) , S3Ops :: ListBucketInventoryConfigurations => $ macro ! (ListBucketInventoryConfigurations) , S3Ops :: PutBucketReplication => $ macro ! (PutBucketReplication) , S3Ops :: GetBucketNotificationConfiguration => $ macro ! (GetBucketNotificationConfiguration) , S3Ops :: PutBucketAnalyticsConfiguration => $ macro ! (PutBucketAnalyticsConfiguration) , S3Ops :: PutBucketRequestPayment => $ macro ! (PutBucketRequestPayment) , S3Ops :: DeleteBucketIntelligentTieringConfiguration => $ macro ! (DeleteBucketIntelligentTieringConfiguration) , S3Ops :: ListParts => $ macro ! (ListParts) , S3Ops :: GetBucketTagging => $ macro ! (GetBucketTagging) , S3Ops :: DeletePublicAccessBlock => $ macro ! (DeletePublicAccessBlock) , S3Ops :: ListBucketMetricsConfigurations => $ macro ! (ListBucketMetricsConfigurations) , S3Ops :: PutBucketOwnershipControls => $ macro ! (PutBucketOwnershipControls) , S3Ops :: DeleteBucketReplication => $ macro ! (DeleteBucketReplication) , S3Ops :: GetBucketPolicy => $ macro ! (GetBucketPolicy) , S3Ops :: GetBucketLifecycleConfiguration => $ macro ! (GetBucketLifecycleConfiguration) , S3Ops :: PutBucketAcl => $ macro ! (PutBucketAcl) , S3Ops :: PutObjectLegalHold => $ macro ! (PutObjectLegalHold) , } } } pub (crate) use generate_ops_code ; pub (crate) use generate_ops_items ; pub (crate) use generate_ops_match ;

pub type IsLatest = bool ;

pub type MetricsId = String ;

pub struct BucketLoggingStatus { logging_enabled : Option < LoggingEnabled > , }

pub struct CompleteMultipartUpload ; impl Op for CompleteMultipartUpload { type Input = CompleteMultipartUploadRequest ; type Output = CompleteMultipartUploadOutput ; }

pub type ObjectLockEnabledForBucket = bool ;

pub type ObjectSizeLessThanBytes = i64 ;

pub struct IntelligentTieringConfiguration { tierings : Option < TieringList > , status : Option < IntelligentTieringStatus > , filter : Option < IntelligentTieringFilter > , id : Option < IntelligentTieringId > , }

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum RequestCharged { requester , } impl AsRef < str > for RequestCharged { fn as_ref (& self) -> & str { match self { Self :: requester => "requester" , } } } impl TryFrom < & str > for RequestCharged { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "requester" => Ok (Self :: requester) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub type LastModified = String ;

pub type IfNoneMatch = String ;

pub struct PutObjectAclOutput { request_charged : Option < RequestCharged > , }

pub type SsekmsEncryptionContext = String ;

pub type Range = String ;

pub struct DeleteBucketIntelligentTieringConfigurationRequest { id : Option < IntelligentTieringId > , bucket : Option < BucketName > , }

pub type ObjectVersionList = Vec < ObjectVersion > ;

pub struct NoncurrentVersionExpiration { newer_noncurrent_versions : Option < VersionCount > , noncurrent_days : Option < Days > , }

pub struct RequestProgress { enabled : Option < EnableRequestProgress > , }

pub struct DeleteBucketWebsite ; impl Op for DeleteBucketWebsite { type Input = DeleteBucketWebsiteRequest ; type Output = Destination ; }

pub type ResponseContentType = String ;

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum ChecksumAlgorithm { CRC32 , CRC32C , SHA1 , SHA256 , } impl AsRef < str > for ChecksumAlgorithm { fn as_ref (& self) -> & str { match self { Self :: CRC32 => "CRC32" , Self :: CRC32C => "CRC32C" , Self :: SHA1 => "SHA1" , Self :: SHA256 => "SHA256" , } } } impl TryFrom < & str > for ChecksumAlgorithm { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "CRC32" => Ok (Self :: CRC32) , "CRC32C" => Ok (Self :: CRC32C) , "SHA1" => Ok (Self :: SHA1) , "SHA256" => Ok (Self :: SHA256) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub struct RestoreObject ; impl Op for RestoreObject { type Input = RestoreObjectRequest ; type Output = RestoreObjectOutput ; }

pub struct CorsConfiguration { cors_rules : Option < CorsRules > , }

pub struct InventoryDestination { s3_bucket_destination : Option < InventoryS3BucketDestination > , }

pub struct PutBucketLifecycleConfigurationRequest { expected_bucket_owner : Option < AccountId > , bucket : Option < BucketName > , lifecycle_configuration : Option < BucketLifecycleConfiguration > , checksum_algorithm : Option < ChecksumAlgorithm > , }

pub type AcceptRanges = String ;

pub struct DeleteBucketTagging ; impl Op for DeleteBucketTagging { type Input = DeleteBucketTaggingRequest ; type Output = Destination ; }

pub type ExposeHeader = String ;

pub struct RequestPaymentConfiguration { payer : Option < Payer > , }

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum InventoryIncludedObjectVersions { All , Current , } impl AsRef < str > for InventoryIncludedObjectVersions { fn as_ref (& self) -> & str { match self { Self :: All => "All" , Self :: Current => "Current" , } } } impl TryFrom < & str > for InventoryIncludedObjectVersions { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "All" => Ok (Self :: All) , "Current" => Ok (Self :: Current) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub struct GetObjectLegalHoldRequest { version_id : Option < ObjectVersionId > , bucket : Option < BucketName > , request_payer : Option < RequestPayer > , key : Option < ObjectKey > , expected_bucket_owner : Option < AccountId > , }

pub struct WriteGetObjectResponse ; impl Op for WriteGetObjectResponse { type Input = WriteGetObjectResponseRequest ; type Output = Destination ; }

pub type ResponseExpires = String ;

pub type Initiated = String ;

pub struct InventoryEncryption { ssekms : Option < Ssekms > , sses3 : Option < Sses3 > , }

pub struct WebsiteConfiguration { index_document : Option < IndexDocument > , routing_rules : Option < RoutingRules > , error_document : Option < ErrorDocument > , redirect_all_requests_to : Option < RedirectAllRequestsTo > , }

pub struct ReplicaModifications { status : Option < ReplicaModificationsStatus > , }

pub struct DeleteObjectRequest { request_payer : Option < RequestPayer > , expected_bucket_owner : Option < AccountId > , bucket : Option < BucketName > , bypass_governance_retention : Option < BypassGovernanceRetention > , key : Option < ObjectKey > , mfa : Option < Mfa > , version_id : Option < ObjectVersionId > , }

pub struct GetBucketInventoryConfiguration ; impl Op for GetBucketInventoryConfiguration { type Input = GetBucketInventoryConfigurationRequest ; type Output = GetBucketInventoryConfigurationOutput ; }

pub type End = i64 ;

pub struct GetBucketOwnershipControlsOutput { ownership_controls : Option < OwnershipControls > , }

pub struct GetObjectTorrent ; impl Op for GetObjectTorrent { type Input = GetObjectTorrentRequest ; type Output = GetObjectTorrentOutput ; }

pub type QueueConfigurationList = Vec < QueueConfiguration > ;

pub struct PutBucketVersioning ; impl Op for PutBucketVersioning { type Input = PutBucketVersioningRequest ; type Output = Destination ; }

pub type KeyPrefixEquals = String ;

pub type AccountId = String ;

pub struct GetBucketNotificationConfigurationRequest { bucket : Option < BucketName > , expected_bucket_owner : Option < AccountId > , }

pub struct GetObjectLegalHold ; impl Op for GetObjectLegalHold { type Input = GetObjectLegalHoldRequest ; type Output = GetObjectLegalHoldOutput ; }

pub type GrantWriteAcp = String ;

pub type ContentRange = String ;

pub type Role = String ;

pub struct InputSerialization { json : Option < JsonInput > , parquet : Option < ParquetInput > , compression_type : Option < CompressionType > , csv : Option < CsvInput > , }

pub struct PutBucketTaggingRequest { checksum_algorithm : Option < ChecksumAlgorithm > , expected_bucket_owner : Option < AccountId > , content_md5 : Option < ContentMd5 > , tagging : Option < Tagging > , bucket : Option < BucketName > , }

pub struct GetBucketCorsRequest { expected_bucket_owner : Option < AccountId > , bucket : Option < BucketName > , }

pub struct GetObjectAttributesRequest { part_number_marker : Option < PartNumberMarker > , object_attributes : Option < ObjectAttributesList > , version_id : Option < ObjectVersionId > , max_parts : Option < MaxParts > , expected_bucket_owner : Option < AccountId > , bucket : Option < BucketName > , sse_customer_key : Option < SseCustomerKey > , key : Option < ObjectKey > , request_payer : Option < RequestPayer > , sse_customer_algorithm : Option < SseCustomerAlgorithm > , sse_customer_key_md5 : Option < SseCustomerKeyMd5 > , }

pub type Metadata = HashMap < MetadataKey , MetadataKey > ;

pub struct PutBucketInventoryConfiguration ; impl Op for PutBucketInventoryConfiguration { type Input = PutBucketInventoryConfigurationRequest ; type Output = Destination ; }

pub struct ListObjectsV2Request { max_keys : Option < MaxKeys > , continuation_token : Option < Token > , delimiter : Option < Delimiter > , bucket : Option < BucketName > , start_after : Option < StartAfter > , fetch_owner : Option < FetchOwner > , expected_bucket_owner : Option < AccountId > , prefix : Option < Prefix > , request_payer : Option < RequestPayer > , encoding_type : Option < EncodingType > , }

pub struct PutObjectLockConfigurationOutput { request_charged : Option < RequestCharged > , }

pub struct PutObjectRetentionOutput { request_charged : Option < RequestCharged > , }

pub struct ObjectNotInActiveTierError { }

pub struct GetBucketAccelerateConfigurationOutput { status : Option < BucketAccelerateStatus > , }

pub struct GetBucketEncryptionOutput { server_side_encryption_configuration : Option < ServerSideEncryptionConfiguration > , }

pub type ReplicationRuleFilter = Vec < String > ;

pub type ResponseContentLanguage = String ;

pub struct CopyObject ; impl Op for CopyObject { type Input = CopyObjectRequest ; type Output = CopyObjectOutput ; }

pub struct ListBucketAnalyticsConfigurations ; impl Op for ListBucketAnalyticsConfigurations { type Input = ListBucketAnalyticsConfigurationsRequest ; type Output = ListBucketAnalyticsConfigurationsOutput ; }

pub type SseCustomerAlgorithm = String ;

pub type SseCustomerKey = String ;

pub struct ServerSideEncryptionByDefault { sse_algorithm : Option < ServerSideEncryption > , kms_master_key_id : Option < SsekmsKeyId > , }

pub struct CreateMultipartUploadOutput { abort_rule_id : Option < AbortRuleId > , bucket : Option < BucketName > , sse_customer_algorithm : Option < SseCustomerAlgorithm > , upload_id : Option < MultipartUploadId > , abort_date : Option < AbortDate > , ssekms_encryption_context : Option < SsekmsEncryptionContext > , bucket_key_enabled : Option < BucketKeyEnabled > , checksum_algorithm : Option < ChecksumAlgorithm > , request_charged : Option < RequestCharged > , ssekms_key_id : Option < SsekmsKeyId > , server_side_encryption : Option < ServerSideEncryption > , sse_customer_key_md5 : Option < SseCustomerKeyMd5 > , key : Option < ObjectKey > , }

pub struct DeleteBucketPolicyRequest { expected_bucket_owner : Option < AccountId > , bucket : Option < BucketName > , }

pub type CopySourceVersionId = String ;

pub type Body = Vec < u8 > ;

pub struct OwnershipControls { rules : Option < OwnershipControlsRules > , }

pub struct ListBucketAnalyticsConfigurationsOutput { next_continuation_token : Option < NextToken > , is_truncated : Option < IsTruncated > , analytics_configuration_list : Option < AnalyticsConfigurationList > , continuation_token : Option < Token > , }

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum ObjectOwnership { BucketOwnerPreferred , ObjectWriter , BucketOwnerEnforced , } impl AsRef < str > for ObjectOwnership { fn as_ref (& self) -> & str { match self { Self :: BucketOwnerPreferred => "BucketOwnerPreferred" , Self :: ObjectWriter => "ObjectWriter" , Self :: BucketOwnerEnforced => "BucketOwnerEnforced" , } } } impl TryFrom < & str > for ObjectOwnership { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "BucketOwnerPreferred" => Ok (Self :: BucketOwnerPreferred) , "ObjectWriter" => Ok (Self :: ObjectWriter) , "BucketOwnerEnforced" => Ok (Self :: BucketOwnerEnforced) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub struct ReplicationTime { time : Option < ReplicationTimeValue > , status : Option < ReplicationTimeStatus > , }

pub type EmailAddress = String ;

pub struct CreateBucketOutput { location : Option < Location > , }

pub struct GetBucketMetricsConfigurationRequest { bucket : Option < BucketName > , id : Option < MetricsId > , expected_bucket_owner : Option < AccountId > , }

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum BucketAccelerateStatus { Enabled , Suspended , } impl AsRef < str > for BucketAccelerateStatus { fn as_ref (& self) -> & str { match self { Self :: Enabled => "Enabled" , Self :: Suspended => "Suspended" , } } } impl TryFrom < & str > for BucketAccelerateStatus { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "Enabled" => Ok (Self :: Enabled) , "Suspended" => Ok (Self :: Suspended) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub type CopySourceRange = String ;

pub type FilterRuleList = Vec < FilterRule > ;

pub struct GetBucketCorsOutput { cors_rules : Option < CorsRules > , }

pub struct JsonInput { r#type : Option < JsonType > , }

pub struct PublicAccessBlockConfiguration { block_public_acls : Option < Setting > , restrict_public_buckets : Option < Setting > , ignore_public_acls : Option < Setting > , block_public_policy : Option < Setting > , }

pub type CopySourceIfNoneMatch = String ;

pub struct ListBucketInventoryConfigurationsOutput { next_continuation_token : Option < NextToken > , continuation_token : Option < Token > , inventory_configuration_list : Option < InventoryConfigurationList > , is_truncated : Option < IsTruncated > , }

pub struct ReplicationTimeValue { minutes : Option < Minutes > , }

pub type BucketName = String ;

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum JsonType { DOCUMENT , LINES , } impl AsRef < str > for JsonType { fn as_ref (& self) -> & str { match self { Self :: DOCUMENT => "DOCUMENT" , Self :: LINES => "LINES" , } } } impl TryFrom < & str > for JsonType { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "DOCUMENT" => Ok (Self :: DOCUMENT) , "LINES" => Ok (Self :: LINES) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub struct GetBucketVersioningRequest { bucket : Option < BucketName > , expected_bucket_owner : Option < AccountId > , }

pub struct RoutingRule { condition : Option < Condition > , redirect : Option < Redirect > , }

pub struct GetBucketVersioning ; impl Op for GetBucketVersioning { type Input = GetBucketVersioningRequest ; type Output = GetBucketVersioningOutput ; }

pub type DaysAfterInitiation = i32 ;

pub struct ObjectLockLegalHold { status : Option < ObjectLockLegalHoldStatus > , }

pub struct ScanRange { end : Option < End > , start : Option < Start > , }

pub type SkipValidation = bool ;

pub struct StorageClassAnalysis { data_export : Option < StorageClassAnalysisDataExport > , }

pub struct UploadPartRequest { checksum_sha256 : Option < ChecksumSha256 > , sse_customer_key : Option < SseCustomerKey > , upload_id : Option < MultipartUploadId > , content_length : Option < ContentLength > , part_number : Option < PartNumber > , sse_customer_algorithm : Option < SseCustomerAlgorithm > , content_md5 : Option < ContentMd5 > , sse_customer_key_md5 : Option < SseCustomerKeyMd5 > , checksum_sha1 : Option < ChecksumSha1 > , body : Option < StreamingBlob > , expected_bucket_owner : Option < AccountId > , checksum_crc32 : Option < ChecksumCrc32 > , checksum_crc32_c : Option < ChecksumCrc32c > , key : Option < ObjectKey > , request_payer : Option < RequestPayer > , bucket : Option < BucketName > , checksum_algorithm : Option < ChecksumAlgorithm > , }

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum Tier { Standard , Bulk , Expedited , } impl AsRef < str > for Tier { fn as_ref (& self) -> & str { match self { Self :: Standard => "Standard" , Self :: Bulk => "Bulk" , Self :: Expedited => "Expedited" , } } } impl TryFrom < & str > for Tier { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "Standard" => Ok (Self :: Standard) , "Bulk" => Ok (Self :: Bulk) , "Expedited" => Ok (Self :: Expedited) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub struct DeleteBucketLifecycleRequest { expected_bucket_owner : Option < AccountId > , bucket : Option < BucketName > , }

pub struct PutBucketCors ; impl Op for PutBucketCors { type Input = PutBucketCorsRequest ; type Output = Destination ; }

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum StorageClassAnalysisSchemaVersion { V_1 , } impl AsRef < str > for StorageClassAnalysisSchemaVersion { fn as_ref (& self) -> & str { match self { Self :: V_1 => "V_1" , } } } impl TryFrom < & str > for StorageClassAnalysisSchemaVersion { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "V_1" => Ok (Self :: V_1) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub struct GetBucketCors ; impl Op for GetBucketCors { type Input = GetBucketCorsRequest ; type Output = GetBucketCorsOutput ; }

pub type Message = String ;

pub type FilterRuleValue = String ;

pub struct GetBucketAclOutput { owner : Option < Owner > , grants : Option < Grants > , }

pub struct ListBucketIntelligentTieringConfigurations ; impl Op for ListBucketIntelligentTieringConfigurations { type Input = ListBucketIntelligentTieringConfigurationsRequest ; type Output = ListBucketIntelligentTieringConfigurationsOutput ; }

pub type Date = String ;

pub type ResponseCacheControl = String ;

pub struct CopyObjectResult { checksum_sha256 : Option < ChecksumSha256 > , checksum_crc32_c : Option < ChecksumCrc32c > , checksum_crc32 : Option < ChecksumCrc32 > , checksum_sha1 : Option < ChecksumSha1 > , last_modified : Option < LastModified > , e_tag : Option < ETag > , }

pub struct DeleteObjectTaggingRequest { bucket : Option < BucketName > , expected_bucket_owner : Option < AccountId > , version_id : Option < ObjectVersionId > , key : Option < ObjectKey > , }

pub struct CreateMultipartUploadRequest { content_disposition : Option < ContentDisposition > , expected_bucket_owner : Option < AccountId > , cache_control : Option < CacheControl > , tagging : Option < TaggingHeader > , ssekms_key_id : Option < SsekmsKeyId > , acl : Option < ObjectCannedAcl > , content_encoding : Option < ContentEncoding > , object_lock_legal_hold_status : Option < ObjectLockLegalHoldStatus > , checksum_algorithm : Option < ChecksumAlgorithm > , expires : Option < Expires > , content_language : Option < ContentLanguage > , object_lock_retain_until_date : Option < ObjectLockRetainUntilDate > , grant_write_acp : Option < GrantWriteAcp > , storage_class : Option < StorageClass > , sse_customer_algorithm : Option < SseCustomerAlgorithm > , sse_customer_key_md5 : Option < SseCustomerKeyMd5 > , grant_full_control : Option < GrantFullControl > , grant_read : Option < GrantRead > , object_lock_mode : Option < ObjectLockMode > , key : Option < ObjectKey > , server_side_encryption : Option < ServerSideEncryption > , sse_customer_key : Option < SseCustomerKey > , bucket : Option < BucketName > , bucket_key_enabled : Option < BucketKeyEnabled > , website_redirect_location : Option < WebsiteRedirectLocation > , content_type : Option < ContentType > , ssekms_encryption_context : Option < SsekmsEncryptionContext > , request_payer : Option < RequestPayer > , metadata : Option < Metadata > , grant_read_acp : Option < GrantReadAcp > , }

pub struct AnalyticsS3BucketDestination { prefix : Option < Prefix > , bucket_account_id : Option < AccountId > , format : Option < AnalyticsS3ExportFileFormat > , bucket : Option < BucketName > , }

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum ExpirationStatus { Enabled , Disabled , } impl AsRef < str > for ExpirationStatus { fn as_ref (& self) -> & str { match self { Self :: Enabled => "Enabled" , Self :: Disabled => "Disabled" , } } } impl TryFrom < & str > for ExpirationStatus { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "Enabled" => Ok (Self :: Enabled) , "Disabled" => Ok (Self :: Disabled) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub struct InventoryFilter { prefix : Option < Prefix > , }

pub type CompletedPartList = Vec < CompletedPart > ;

pub type CopySourceSseCustomerKeyMd5 = String ;

pub type CreationDate = String ;

pub struct AbortMultipartUploadOutput { request_charged : Option < RequestCharged > , }

pub struct GetBucketOwnershipControlsRequest { expected_bucket_owner : Option < AccountId > , bucket : Option < BucketName > , }

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum ObjectVersionStorageClass { STANDARD , } impl AsRef < str > for ObjectVersionStorageClass { fn as_ref (& self) -> & str { match self { Self :: STANDARD => "STANDARD" , } } } impl TryFrom < & str > for ObjectVersionStorageClass { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "STANDARD" => Ok (Self :: STANDARD) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub struct WriteGetObjectResponseRequest { last_modified : Option < LastModified > , content_disposition : Option < ContentDisposition > , content_length : Option < ContentLength > , content_type : Option < ContentType > , content_encoding : Option < ContentEncoding > , content_range : Option < ContentRange > , request_charged : Option < RequestCharged > , object_lock_retain_until_date : Option < ObjectLockRetainUntilDate > , replication_status : Option < ReplicationStatus > , e_tag : Option < ETag > , metadata : Option < Metadata > , error_message : Option < ErrorMessage > , object_lock_legal_hold_status : Option < ObjectLockLegalHoldStatus > , restore : Option < Restore > , sse_customer_key_md5 : Option < SseCustomerKeyMd5 > , request_route : Option < RequestRoute > , checksum_sha256 : Option < ChecksumSha256 > , ssekms_key_id : Option < SsekmsKeyId > , storage_class : Option < StorageClass > , accept_ranges : Option < AcceptRanges > , version_id : Option < ObjectVersionId > , error_code : Option < ErrorCode > , missing_meta : Option < MissingMeta > , checksum_crc32_c : Option < ChecksumCrc32c > , bucket_key_enabled : Option < BucketKeyEnabled > , expiration : Option < Expiration > , expires : Option < Expires > , delete_marker : Option < DeleteMarker > , body : Option < StreamingBlob > , checksum_sha1 : Option < ChecksumSha1 > , content_language : Option < ContentLanguage > , object_lock_mode : Option < ObjectLockMode > , request_token : Option < RequestToken > , sse_customer_algorithm : Option < SseCustomerAlgorithm > , server_side_encryption : Option < ServerSideEncryption > , cache_control : Option < CacheControl > , checksum_crc32 : Option < ChecksumCrc32 > , parts_count : Option < PartsCount > , status_code : Option < GetObjectResponseStatusCode > , tag_count : Option < TagCount > , }

pub type NextPartNumberMarker = String ;

pub struct GetBucketAccelerateConfiguration ; impl Op for GetBucketAccelerateConfiguration { type Input = GetBucketAccelerateConfigurationRequest ; type Output = GetBucketAccelerateConfigurationOutput ; }

pub struct GetBucketInventoryConfigurationRequest { id : Option < InventoryId > , bucket : Option < BucketName > , expected_bucket_owner : Option < AccountId > , }

pub struct NotFound { }

pub type TagSet = Vec < Tag > ;

pub struct GetObjectAttributesParts { max_parts : Option < MaxParts > , parts : Option < PartsList > , total_parts_count : Option < PartsCount > , is_truncated : Option < IsTruncated > , next_part_number_marker : Option < NextPartNumberMarker > , part_number_marker : Option < PartNumberMarker > , }

pub type UserMetadata = Vec < MetadataEntry > ;

pub type ETag = String ;

pub struct PutBucketIntelligentTieringConfiguration ; impl Op for PutBucketIntelligentTieringConfiguration { type Input = PutBucketIntelligentTieringConfigurationRequest ; type Output = Destination ; }

pub struct CsvOutput { quote_escape_character : Option < QuoteEscapeCharacter > , record_delimiter : Option < RecordDelimiter > , quote_fields : Option < QuoteFields > , field_delimiter : Option < FieldDelimiter > , quote_character : Option < QuoteCharacter > , }

pub struct DeleteObjectsOutput { errors : Option < Errors > , request_charged : Option < RequestCharged > , deleted : Option < DeletedObjects > , }

pub struct GetBucketAccelerateConfigurationRequest { expected_bucket_owner : Option < AccountId > , bucket : Option < BucketName > , }

pub type AllowedMethods = Vec < AllowedMethod > ;

pub struct InvalidObjectState { access_tier : Option < IntelligentTieringAccessTier > , storage_class : Option < StorageClass > , }

pub struct ListMultipartUploadsOutput { delimiter : Option < Delimiter > , bucket : Option < BucketName > , is_truncated : Option < IsTruncated > , max_uploads : Option < MaxUploads > , prefix : Option < Prefix > , encoding_type : Option < EncodingType > , next_key_marker : Option < NextKeyMarker > , uploads : Option < MultipartUploadList > , upload_id_marker : Option < UploadIdMarker > , next_upload_id_marker : Option < NextUploadIdMarker > , key_marker : Option < KeyMarker > , common_prefixes : Option < CommonPrefixList > , }

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum ChecksumMode { ENABLED , } impl AsRef < str > for ChecksumMode { fn as_ref (& self) -> & str { match self { Self :: ENABLED => "ENABLED" , } } } impl TryFrom < & str > for ChecksumMode { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "ENABLED" => Ok (Self :: ENABLED) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub type GrantFullControl = String ;

pub struct DeleteMarkerReplication { status : Option < DeleteMarkerReplicationStatus > , }

pub struct NoncurrentVersionTransition { newer_noncurrent_versions : Option < VersionCount > , noncurrent_days : Option < Days > , storage_class : Option < TransitionStorageClass > , }

pub struct Owner { display_name : Option < DisplayName > , id : Option < Id > , }

pub struct GetBucketTaggingRequest { expected_bucket_owner : Option < AccountId > , bucket : Option < BucketName > , }

pub type MissingMeta = i32 ;

pub struct InventoryConfiguration { filter : Option < InventoryFilter > , id : Option < InventoryId > , included_object_versions : Option < InventoryIncludedObjectVersions > , destination : Option < InventoryDestination > , is_enabled : Option < IsEnabled > , schedule : Option < InventorySchedule > , optional_fields : Option < InventoryOptionalFields > , }

pub type LocationPrefix = String ;

pub struct DeleteObjectOutput { delete_marker : Option < DeleteMarker > , request_charged : Option < RequestCharged > , version_id : Option < ObjectVersionId > , }

pub struct PutBucketTagging ; impl Op for PutBucketTagging { type Input = PutBucketTaggingRequest ; type Output = Destination ; }

pub type ResponseContentDisposition = String ;

pub struct GetBucketLogging ; impl Op for GetBucketLogging { type Input = GetBucketLoggingRequest ; type Output = GetBucketLoggingOutput ; }

pub struct ListBucketMetricsConfigurationsOutput { is_truncated : Option < IsTruncated > , continuation_token : Option < Token > , next_continuation_token : Option < NextToken > , metrics_configuration_list : Option < MetricsConfigurationList > , }

pub struct PolicyStatus { is_public : Option < IsPublic > , }

pub struct ListObjects ; impl Op for ListObjects { type Input = ListObjectsRequest ; type Output = ListObjectsOutput ; }

pub struct Redirect { host_name : Option < HostName > , http_redirect_code : Option < HttpRedirectCode > , protocol : Option < Protocol > , replace_key_prefix_with : Option < ReplaceKeyPrefixWith > , replace_key_with : Option < ReplaceKeyWith > , }

pub type ResponseContentEncoding = String ;

pub type VersionIdMarker = String ;

pub struct HeadBucket ; impl Op for HeadBucket { type Input = HeadBucketRequest ; type Output = Destination ; }

pub struct SourceSelectionCriteria { replica_modifications : Option < ReplicaModifications > , sse_kms_encrypted_objects : Option < SseKmsEncryptedObjects > , }

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum Protocol { http , https , } impl AsRef < str > for Protocol { fn as_ref (& self) -> & str { match self { Self :: http => "http" , Self :: https => "https" , } } } impl TryFrom < & str > for Protocol { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "http" => Ok (Self :: http) , "https" => Ok (Self :: https) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub struct Stats { bytes_processed : Option < BytesProcessed > , bytes_returned : Option < BytesReturned > , bytes_scanned : Option < BytesScanned > , }

pub type ObjectList = Vec < Object > ;

pub struct GetObjectRequest { response_content_disposition : Option < ResponseContentDisposition > , key : Option < ObjectKey > , sse_customer_algorithm : Option < SseCustomerAlgorithm > , part_number : Option < PartNumber > , response_content_encoding : Option < ResponseContentEncoding > , response_content_type : Option < ResponseContentType > , if_modified_since : Option < IfModifiedSince > , checksum_mode : Option < ChecksumMode > , response_cache_control : Option < ResponseCacheControl > , if_none_match : Option < IfNoneMatch > , request_payer : Option < RequestPayer > , if_match : Option < IfMatch > , if_unmodified_since : Option < IfUnmodifiedSince > , response_expires : Option < ResponseExpires > , sse_customer_key : Option < SseCustomerKey > , bucket : Option < BucketName > , response_content_language : Option < ResponseContentLanguage > , version_id : Option < ObjectVersionId > , expected_bucket_owner : Option < AccountId > , range : Option < Range > , sse_customer_key_md5 : Option < SseCustomerKeyMd5 > , }

pub type DeleteMarkers = Vec < DeleteMarkerEntry > ;

pub type TagCount = i32 ;

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum Permission { FULL_CONTROL , WRITE , WRITE_ACP , READ , READ_ACP , } impl AsRef < str > for Permission { fn as_ref (& self) -> & str { match self { Self :: FULL_CONTROL => "FULL_CONTROL" , Self :: WRITE => "WRITE" , Self :: WRITE_ACP => "WRITE_ACP" , Self :: READ => "READ" , Self :: READ_ACP => "READ_ACP" , } } } impl TryFrom < & str > for Permission { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "FULL_CONTROL" => Ok (Self :: FULL_CONTROL) , "WRITE" => Ok (Self :: WRITE) , "WRITE_ACP" => Ok (Self :: WRITE_ACP) , "READ" => Ok (Self :: READ) , "READ_ACP" => Ok (Self :: READ_ACP) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub struct PutPublicAccessBlockRequest { expected_bucket_owner : Option < AccountId > , bucket : Option < BucketName > , checksum_algorithm : Option < ChecksumAlgorithm > , public_access_block_configuration : Option < PublicAccessBlockConfiguration > , content_md5 : Option < ContentMd5 > , }

pub type Policy = String ;

pub struct PutBucketLoggingRequest { bucket : Option < BucketName > , checksum_algorithm : Option < ChecksumAlgorithm > , bucket_logging_status : Option < BucketLoggingStatus > , content_md5 : Option < ContentMd5 > , expected_bucket_owner : Option < AccountId > , }

pub struct GetPublicAccessBlock ; impl Op for GetPublicAccessBlock { type Input = GetPublicAccessBlockRequest ; type Output = GetPublicAccessBlockOutput ; }

pub struct GetObjectLockConfigurationOutput { object_lock_configuration : Option < ObjectLockConfiguration > , }

pub type Mfa = String ;

pub struct NotificationConfiguration { topic_configurations : Option < TopicConfigurationList > , event_bridge_configuration : Option < EventBridgeConfiguration > , lambda_function_configurations : Option < LambdaFunctionConfigurationList > , queue_configurations : Option < QueueConfigurationList > , }

pub struct ListBucketInventoryConfigurationsRequest { expected_bucket_owner : Option < AccountId > , continuation_token : Option < Token > , bucket : Option < BucketName > , }

pub struct ListPartsRequest { max_parts : Option < MaxParts > , expected_bucket_owner : Option < AccountId > , part_number_marker : Option < PartNumberMarker > , bucket : Option < BucketName > , sse_customer_key : Option < SseCustomerKey > , request_payer : Option < RequestPayer > , upload_id : Option < MultipartUploadId > , sse_customer_key_md5 : Option < SseCustomerKeyMd5 > , key : Option < ObjectKey > , sse_customer_algorithm : Option < SseCustomerAlgorithm > , }

pub struct DeleteBucketOwnershipControlsRequest { expected_bucket_owner : Option < AccountId > , bucket : Option < BucketName > , }

pub struct CopyObjectOutput { copy_source_version_id : Option < CopySourceVersionId > , request_charged : Option < RequestCharged > , bucket_key_enabled : Option < BucketKeyEnabled > , expiration : Option < Expiration > , sse_customer_algorithm : Option < SseCustomerAlgorithm > , copy_object_result : Option < CopyObjectResult > , sse_customer_key_md5 : Option < SseCustomerKeyMd5 > , server_side_encryption : Option < ServerSideEncryption > , ssekms_key_id : Option < SsekmsKeyId > , version_id : Option < ObjectVersionId > , ssekms_encryption_context : Option < SsekmsEncryptionContext > , }

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum BucketVersioningStatus { Enabled , Suspended , } impl AsRef < str > for BucketVersioningStatus { fn as_ref (& self) -> & str { match self { Self :: Enabled => "Enabled" , Self :: Suspended => "Suspended" , } } } impl TryFrom < & str > for BucketVersioningStatus { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "Enabled" => Ok (Self :: Enabled) , "Suspended" => Ok (Self :: Suspended) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub type CommonPrefixList = Vec < CommonPrefix > ;

pub type ErrorCode = String ;

pub type MetadataKey = String ;

pub struct MultipartUpload { storage_class : Option < StorageClass > , upload_id : Option < MultipartUploadId > , owner : Option < Owner > , checksum_algorithm : Option < ChecksumAlgorithm > , initiator : Option < Initiator > , initiated : Option < Initiated > , key : Option < ObjectKey > , }

pub type DeleteMarkerVersionId = String ;

pub struct Checksum { checksum_crc32 : Option < ChecksumCrc32 > , checksum_sha1 : Option < ChecksumSha1 > , checksum_sha256 : Option < ChecksumSha256 > , checksum_crc32_c : Option < ChecksumCrc32c > , }

pub struct GetObjectRetentionRequest { version_id : Option < ObjectVersionId > , request_payer : Option < RequestPayer > , bucket : Option < BucketName > , expected_bucket_owner : Option < AccountId > , key : Option < ObjectKey > , }

pub type IsPublic = bool ;

pub struct GetBucketVersioningOutput { mfa_delete : Option < MfaDeleteStatus > , status : Option < BucketVersioningStatus > , }

pub type Prefix = String ;

pub type PartsCount = i32 ;

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum Payer { Requester , BucketOwner , } impl AsRef < str > for Payer { fn as_ref (& self) -> & str { match self { Self :: Requester => "Requester" , Self :: BucketOwner => "BucketOwner" , } } } impl TryFrom < & str > for Payer { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "Requester" => Ok (Self :: Requester) , "BucketOwner" => Ok (Self :: BucketOwner) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub struct ProgressEvent { details : Option < Progress > , }

pub type TargetPrefix = String ;

pub type Location = String ;

pub struct GetObjectOutput { content_range : Option < ContentRange > , content_disposition : Option < ContentDisposition > , object_lock_mode : Option < ObjectLockMode > , object_lock_retain_until_date : Option < ObjectLockRetainUntilDate > , request_charged : Option < RequestCharged > , version_id : Option < ObjectVersionId > , content_type : Option < ContentType > , checksum_sha256 : Option < ChecksumSha256 > , sse_customer_algorithm : Option < SseCustomerAlgorithm > , delete_marker : Option < DeleteMarker > , ssekms_key_id : Option < SsekmsKeyId > , last_modified : Option < LastModified > , checksum_sha1 : Option < ChecksumSha1 > , content_encoding : Option < ContentEncoding > , expires : Option < Expires > , replication_status : Option < ReplicationStatus > , body : Option < StreamingBlob > , sse_customer_key_md5 : Option < SseCustomerKeyMd5 > , checksum_crc32 : Option < ChecksumCrc32 > , website_redirect_location : Option < WebsiteRedirectLocation > , accept_ranges : Option < AcceptRanges > , content_language : Option < ContentLanguage > , storage_class : Option < StorageClass > , content_length : Option < ContentLength > , parts_count : Option < PartsCount > , checksum_crc32_c : Option < ChecksumCrc32c > , restore : Option < Restore > , object_lock_legal_hold_status : Option < ObjectLockLegalHoldStatus > , e_tag : Option < ETag > , server_side_encryption : Option < ServerSideEncryption > , bucket_key_enabled : Option < BucketKeyEnabled > , cache_control : Option < CacheControl > , metadata : Option < Metadata > , tag_count : Option < TagCount > , expiration : Option < Expiration > , missing_meta : Option < MissingMeta > , }

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum ExistingObjectReplicationStatus { Enabled , Disabled , } impl AsRef < str > for ExistingObjectReplicationStatus { fn as_ref (& self) -> & str { match self { Self :: Enabled => "Enabled" , Self :: Disabled => "Disabled" , } } } impl TryFrom < & str > for ExistingObjectReplicationStatus { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "Enabled" => Ok (Self :: Enabled) , "Disabled" => Ok (Self :: Disabled) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub type ObjectLockToken = String ;

pub struct SseKmsEncryptedObjects { status : Option < SseKmsEncryptedObjectsStatus > , }

pub type DeleteMarker = bool ;

pub type ObjectSize = i64 ;

pub type NotificationId = String ;

pub struct HeadBucketRequest { expected_bucket_owner : Option < AccountId > , bucket : Option < BucketName > , }

pub struct OutputSerialization { csv : Option < CsvOutput > , json : Option < JsonOutput > , }

pub struct ListObjectsRequest { request_payer : Option < RequestPayer > , marker : Option < Marker > , max_keys : Option < MaxKeys > , prefix : Option < Prefix > , delimiter : Option < Delimiter > , expected_bucket_owner : Option < AccountId > , bucket : Option < BucketName > , encoding_type : Option < EncodingType > , }

pub struct PutBucketAccelerateConfiguration ; impl Op for PutBucketAccelerateConfiguration { type Input = PutBucketAccelerateConfigurationRequest ; type Output = Destination ; }

pub struct DeleteObjectTagging ; impl Op for DeleteObjectTagging { type Input = DeleteObjectTaggingRequest ; type Output = DeleteObjectTaggingOutput ; }

pub struct DeleteBucketEncryption ; impl Op for DeleteBucketEncryption { type Input = DeleteBucketEncryptionRequest ; type Output = Destination ; }

pub type MaxUploads = i32 ;

pub type MetricsConfigurationList = Vec < MetricsConfiguration > ;

pub struct GetBucketRequestPaymentRequest { bucket : Option < BucketName > , expected_bucket_owner : Option < AccountId > , }

pub type IsTruncated = bool ;

pub type VersionCount = i32 ;

pub struct GetBucketWebsiteRequest { expected_bucket_owner : Option < AccountId > , bucket : Option < BucketName > , }

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum CompressionType { NONE , GZIP , BZIP2 , } impl AsRef < str > for CompressionType { fn as_ref (& self) -> & str { match self { Self :: NONE => "NONE" , Self :: GZIP => "GZIP" , Self :: BZIP2 => "BZIP2" , } } } impl TryFrom < & str > for CompressionType { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "NONE" => Ok (Self :: NONE) , "GZIP" => Ok (Self :: GZIP) , "BZIP2" => Ok (Self :: BZIP2) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub struct Initiator { display_name : Option < DisplayName > , id : Option < Id > , }

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum FileHeaderInfo { USE , IGNORE , NONE , } impl AsRef < str > for FileHeaderInfo { fn as_ref (& self) -> & str { match self { Self :: USE => "USE" , Self :: IGNORE => "IGNORE" , Self :: NONE => "NONE" , } } } impl TryFrom < & str > for FileHeaderInfo { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "USE" => Ok (Self :: USE) , "IGNORE" => Ok (Self :: IGNORE) , "NONE" => Ok (Self :: NONE) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub type MaxAgeSeconds = i32 ;

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum RestoreRequestType { SELECT , } impl AsRef < str > for RestoreRequestType { fn as_ref (& self) -> & str { match self { Self :: SELECT => "SELECT" , } } } impl TryFrom < & str > for RestoreRequestType { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "SELECT" => Ok (Self :: SELECT) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub struct Error { code : Option < Code > , version_id : Option < ObjectVersionId > , key : Option < ObjectKey > , message : Option < Message > , }

pub type Errors = Vec < Error > ;

pub struct IndexDocument { suffix : Option < Suffix > , }

pub struct Transition { date : Option < Date > , days : Option < Days > , storage_class : Option < TransitionStorageClass > , }

pub struct GetBucketOwnershipControls ; impl Op for GetBucketOwnershipControls { type Input = GetBucketOwnershipControlsRequest ; type Output = GetBucketOwnershipControlsOutput ; }

pub type KeyMarker = String ;

pub struct ListBucketIntelligentTieringConfigurationsOutput { intelligent_tiering_configuration_list : Option < IntelligentTieringConfigurationList > , next_continuation_token : Option < NextToken > , continuation_token : Option < Token > , is_truncated : Option < IsTruncated > , }

pub struct GetObjectAclRequest { version_id : Option < ObjectVersionId > , expected_bucket_owner : Option < AccountId > , key : Option < ObjectKey > , request_payer : Option < RequestPayer > , bucket : Option < BucketName > , }

pub type ChecksumCrc32 = String ;

pub type Grants = Vec < Grant > ;

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum TaggingDirective { COPY , REPLACE , } impl AsRef < str > for TaggingDirective { fn as_ref (& self) -> & str { match self { Self :: COPY => "COPY" , Self :: REPLACE => "REPLACE" , } } } impl TryFrom < & str > for TaggingDirective { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "COPY" => Ok (Self :: COPY) , "REPLACE" => Ok (Self :: REPLACE) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub struct PutBucketWebsiteRequest { website_configuration : Option < WebsiteConfiguration > , expected_bucket_owner : Option < AccountId > , checksum_algorithm : Option < ChecksumAlgorithm > , bucket : Option < BucketName > , content_md5 : Option < ContentMd5 > , }

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum ArchiveStatus { ARCHIVE_ACCESS , DEEP_ARCHIVE_ACCESS , } impl AsRef < str > for ArchiveStatus { fn as_ref (& self) -> & str { match self { Self :: ARCHIVE_ACCESS => "ARCHIVE_ACCESS" , Self :: DEEP_ARCHIVE_ACCESS => "DEEP_ARCHIVE_ACCESS" , } } } impl TryFrom < & str > for ArchiveStatus { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "ARCHIVE_ACCESS" => Ok (Self :: ARCHIVE_ACCESS) , "DEEP_ARCHIVE_ACCESS" => Ok (Self :: DEEP_ARCHIVE_ACCESS) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub type Delimiter = String ;

pub struct DeletePublicAccessBlockRequest { bucket : Option < BucketName > , expected_bucket_owner : Option < AccountId > , }

pub type Expiration = String ;

pub type ContentMd5 = String ;

pub type TopicConfigurationList = Vec < TopicConfiguration > ;

pub struct PutBucketAccelerateConfigurationRequest { expected_bucket_owner : Option < AccountId > , accelerate_configuration : Option < AccelerateConfiguration > , checksum_algorithm : Option < ChecksumAlgorithm > , bucket : Option < BucketName > , }

pub type ObjectAttributesList = Vec < ObjectAttributes > ;

pub type BytesProcessed = i64 ;

pub type Description = String ;

pub struct MetricsAndOperator { prefix : Option < Prefix > , tags : Option < TagSet > , access_point_arn : Option < AccessPointArn > , }

pub type OwnershipControlsRules = Vec < OwnershipControlsRule > ;

pub struct DeleteBucketWebsiteRequest { expected_bucket_owner : Option < AccountId > , bucket : Option < BucketName > , }

pub struct GetObjectAttributes ; impl Op for GetObjectAttributes { type Input = GetObjectAttributesRequest ; type Output = GetObjectAttributesOutput ; }

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum Event { _0 , _1 , _2 , _3 , _4 , _5 , _6 , _7 , _8 , _9 , _10 , _11 , _12 , _13 , _14 , _15 , _16 , _17 , _18 , _19 , _20 , _21 , _22 , _23 , _24 , _25 , _26 , } impl AsRef < str > for Event { fn as_ref (& self) -> & str { match self { Self :: _0 => "s3:ReducedRedundancyLostObject" , Self :: _1 => "s3:ObjectCreated:*" , Self :: _2 => "s3:ObjectCreated:Put" , Self :: _3 => "s3:ObjectCreated:Post" , Self :: _4 => "s3:ObjectCreated:Copy" , Self :: _5 => "s3:ObjectCreated:CompleteMultipartUpload" , Self :: _6 => "s3:ObjectRemoved:*" , Self :: _7 => "s3:ObjectRemoved:Delete" , Self :: _8 => "s3:ObjectRemoved:DeleteMarkerCreated" , Self :: _9 => "s3:ObjectRestore:*" , Self :: _10 => "s3:ObjectRestore:Post" , Self :: _11 => "s3:ObjectRestore:Completed" , Self :: _12 => "s3:Replication:*" , Self :: _13 => "s3:Replication:OperationFailedReplication" , Self :: _14 => "s3:Replication:OperationNotTracked" , Self :: _15 => "s3:Replication:OperationMissedThreshold" , Self :: _16 => "s3:Replication:OperationReplicatedAfterThreshold" , Self :: _17 => "s3:ObjectRestore:Delete" , Self :: _18 => "s3:LifecycleTransition" , Self :: _19 => "s3:IntelligentTiering" , Self :: _20 => "s3:ObjectAcl:Put" , Self :: _21 => "s3:LifecycleExpiration:*" , Self :: _22 => "s3:LifecycleExpiration:Delete" , Self :: _23 => "s3:LifecycleExpiration:DeleteMarkerCreated" , Self :: _24 => "s3:ObjectTagging:*" , Self :: _25 => "s3:ObjectTagging:Put" , Self :: _26 => "s3:ObjectTagging:Delete" , } } } impl TryFrom < & str > for Event { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "s3:ReducedRedundancyLostObject" => Ok (Self :: _0) , "s3:ObjectCreated:*" => Ok (Self :: _1) , "s3:ObjectCreated:Put" => Ok (Self :: _2) , "s3:ObjectCreated:Post" => Ok (Self :: _3) , "s3:ObjectCreated:Copy" => Ok (Self :: _4) , "s3:ObjectCreated:CompleteMultipartUpload" => Ok (Self :: _5) , "s3:ObjectRemoved:*" => Ok (Self :: _6) , "s3:ObjectRemoved:Delete" => Ok (Self :: _7) , "s3:ObjectRemoved:DeleteMarkerCreated" => Ok (Self :: _8) , "s3:ObjectRestore:*" => Ok (Self :: _9) , "s3:ObjectRestore:Post" => Ok (Self :: _10) , "s3:ObjectRestore:Completed" => Ok (Self :: _11) , "s3:Replication:*" => Ok (Self :: _12) , "s3:Replication:OperationFailedReplication" => Ok (Self :: _13) , "s3:Replication:OperationNotTracked" => Ok (Self :: _14) , "s3:Replication:OperationMissedThreshold" => Ok (Self :: _15) , "s3:Replication:OperationReplicatedAfterThreshold" => Ok (Self :: _16) , "s3:ObjectRestore:Delete" => Ok (Self :: _17) , "s3:LifecycleTransition" => Ok (Self :: _18) , "s3:IntelligentTiering" => Ok (Self :: _19) , "s3:ObjectAcl:Put" => Ok (Self :: _20) , "s3:LifecycleExpiration:*" => Ok (Self :: _21) , "s3:LifecycleExpiration:Delete" => Ok (Self :: _22) , "s3:LifecycleExpiration:DeleteMarkerCreated" => Ok (Self :: _23) , "s3:ObjectTagging:*" => Ok (Self :: _24) , "s3:ObjectTagging:Put" => Ok (Self :: _25) , "s3:ObjectTagging:Delete" => Ok (Self :: _26) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub type NextVersionIdMarker = String ;

pub struct ObjectLockConfiguration { rule : Option < ObjectLockRule > , object_lock_enabled : Option < ObjectLockEnabled > , }

pub struct ObjectLockRetention { mode : Option < ObjectLockRetentionMode > , retain_until_date : Option < Date > , }

pub struct GetBucketPolicyStatusRequest { bucket : Option < BucketName > , expected_bucket_owner : Option < AccountId > , }

pub struct GetBucketPolicyStatusOutput { policy_status : Option < PolicyStatus > , }

pub struct PutBucketLifecycleConfiguration ; impl Op for PutBucketLifecycleConfiguration { type Input = PutBucketLifecycleConfigurationRequest ; type Output = Destination ; }

pub struct PutBucketNotificationConfigurationRequest { expected_bucket_owner : Option < AccountId > , skip_destination_validation : Option < SkipValidation > , notification_configuration : Option < NotificationConfiguration > , bucket : Option < BucketName > , }

pub struct GetObjectTagging ; impl Op for GetObjectTagging { type Input = GetObjectTaggingRequest ; type Output = GetObjectTaggingOutput ; }

pub type TargetGrants = Vec < TargetGrant > ;

pub struct ListMultipartUploadsRequest { key_marker : Option < KeyMarker > , expected_bucket_owner : Option < AccountId > , bucket : Option < BucketName > , prefix : Option < Prefix > , upload_id_marker : Option < UploadIdMarker > , delimiter : Option < Delimiter > , encoding_type : Option < EncodingType > , max_uploads : Option < MaxUploads > , }

pub struct GetBucketReplicationOutput { replication_configuration : Option < ReplicationConfiguration > , }

pub struct AnalyticsExportDestination { s3_bucket_destination : Option < AnalyticsS3BucketDestination > , }

pub type AnalyticsFilter = Vec < String > ;

pub type IsEnabled = bool ;

pub struct CreateBucket ; impl Op for CreateBucket { type Input = CreateBucketRequest ; type Output = CreateBucketOutput ; }

pub struct LoggingEnabled { target_bucket : Option < TargetBucket > , target_prefix : Option < TargetPrefix > , target_grants : Option < TargetGrants > , }

pub struct NotificationConfigurationFilter { key : Option < S3KeyFilter > , }

pub type PartNumberMarker = String ;

pub struct PutBucketRequestPaymentRequest { content_md5 : Option < ContentMd5 > , expected_bucket_owner : Option < AccountId > , bucket : Option < BucketName > , request_payment_configuration : Option < RequestPaymentConfiguration > , checksum_algorithm : Option < ChecksumAlgorithm > , }

pub struct SelectObjectContent ; impl Op for SelectObjectContent { type Input = SelectObjectContentRequest ; type Output = SelectObjectContentOutput ; }

pub struct DeleteBucketCors ; impl Op for DeleteBucketCors { type Input = DeleteBucketCorsRequest ; type Output = Destination ; }

pub struct HeadObjectRequest { checksum_mode : Option < ChecksumMode > , if_none_match : Option < IfNoneMatch > , sse_customer_algorithm : Option < SseCustomerAlgorithm > , version_id : Option < ObjectVersionId > , part_number : Option < PartNumber > , range : Option < Range > , request_payer : Option < RequestPayer > , sse_customer_key : Option < SseCustomerKey > , bucket : Option < BucketName > , if_modified_since : Option < IfModifiedSince > , sse_customer_key_md5 : Option < SseCustomerKeyMd5 > , if_unmodified_since : Option < IfUnmodifiedSince > , key : Option < ObjectKey > , if_match : Option < IfMatch > , expected_bucket_owner : Option < AccountId > , }

pub type HostName = String ;

pub struct GetObjectLockConfigurationRequest { bucket : Option < BucketName > , expected_bucket_owner : Option < AccountId > , }

pub struct PutObjectRequest { tagging : Option < TaggingHeader > , grant_read : Option < GrantRead > , checksum_crc32 : Option < ChecksumCrc32 > , checksum_crc32_c : Option < ChecksumCrc32c > , content_md5 : Option < ContentMd5 > , cache_control : Option < CacheControl > , storage_class : Option < StorageClass > , bucket_key_enabled : Option < BucketKeyEnabled > , body : Option < StreamingBlob > , content_type : Option < ContentType > , expected_bucket_owner : Option < AccountId > , grant_full_control : Option < GrantFullControl > , request_payer : Option < RequestPayer > , ssekms_key_id : Option < SsekmsKeyId > , content_length : Option < ContentLength > , content_language : Option < ContentLanguage > , acl : Option < ObjectCannedAcl > , grant_read_acp : Option < GrantReadAcp > , bucket : Option < BucketName > , checksum_sha1 : Option < ChecksumSha1 > , sse_customer_algorithm : Option < SseCustomerAlgorithm > , checksum_sha256 : Option < ChecksumSha256 > , ssekms_encryption_context : Option < SsekmsEncryptionContext > , server_side_encryption : Option < ServerSideEncryption > , object_lock_mode : Option < ObjectLockMode > , sse_customer_key : Option < SseCustomerKey > , grant_write_acp : Option < GrantWriteAcp > , object_lock_retain_until_date : Option < ObjectLockRetainUntilDate > , website_redirect_location : Option < WebsiteRedirectLocation > , content_encoding : Option < ContentEncoding > , content_disposition : Option < ContentDisposition > , key : Option < ObjectKey > , metadata : Option < Metadata > , object_lock_legal_hold_status : Option < ObjectLockLegalHoldStatus > , expires : Option < Expires > , sse_customer_key_md5 : Option < SseCustomerKeyMd5 > , checksum_algorithm : Option < ChecksumAlgorithm > , }

pub struct PutObject ; impl Op for PutObject { type Input = PutObjectRequest ; type Output = PutObjectOutput ; }

pub type EventList = Vec < Event > ;

pub struct HeadObjectOutput { checksum_sha1 : Option < ChecksumSha1 > , sse_customer_algorithm : Option < SseCustomerAlgorithm > , storage_class : Option < StorageClass > , object_lock_mode : Option < ObjectLockMode > , object_lock_retain_until_date : Option < ObjectLockRetainUntilDate > , checksum_sha256 : Option < ChecksumSha256 > , content_encoding : Option < ContentEncoding > , checksum_crc32_c : Option < ChecksumCrc32c > , metadata : Option < Metadata > , object_lock_legal_hold_status : Option < ObjectLockLegalHoldStatus > , replication_status : Option < ReplicationStatus > , request_charged : Option < RequestCharged > , restore : Option < Restore > , missing_meta : Option < MissingMeta > , server_side_encryption : Option < ServerSideEncryption > , ssekms_key_id : Option < SsekmsKeyId > , accept_ranges : Option < AcceptRanges > , parts_count : Option < PartsCount > , content_disposition : Option < ContentDisposition > , expires : Option < Expires > , e_tag : Option < ETag > , content_length : Option < ContentLength > , delete_marker : Option < DeleteMarker > , sse_customer_key_md5 : Option < SseCustomerKeyMd5 > , content_language : Option < ContentLanguage > , expiration : Option < Expiration > , last_modified : Option < LastModified > , content_type : Option < ContentType > , archive_status : Option < ArchiveStatus > , cache_control : Option < CacheControl > , version_id : Option < ObjectVersionId > , website_redirect_location : Option < WebsiteRedirectLocation > , bucket_key_enabled : Option < BucketKeyEnabled > , checksum_crc32 : Option < ChecksumCrc32 > , }

pub type Quiet = bool ;

pub struct UploadPart ; impl Op for UploadPart { type Input = UploadPartRequest ; type Output = UploadPartOutput ; }

pub type MaxParts = i32 ;

pub type ObjectKey = String ;

pub struct ReplicationRule { existing_object_replication : Option < ExistingObjectReplication > , delete_marker_replication : Option < DeleteMarkerReplication > , destination : Option < Destination > , priority : Option < Priority > , filter : Option < ReplicationRuleFilter > , source_selection_criteria : Option < SourceSelectionCriteria > , id : Option < Id > , prefix : Option < Prefix > , status : Option < ReplicationRuleStatus > , }

pub type SsekmsKeyId = String ;

pub type QueueArn = String ;

pub struct RecordsEvent { payload : Option < Body > , }

pub struct Grant { grantee : Option < Grantee > , permission : Option < Permission > , }

pub type Token = String ;

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum MetricsStatus { Enabled , Disabled , } impl AsRef < str > for MetricsStatus { fn as_ref (& self) -> & str { match self { Self :: Enabled => "Enabled" , Self :: Disabled => "Disabled" , } } } impl TryFrom < & str > for MetricsStatus { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "Enabled" => Ok (Self :: Enabled) , "Disabled" => Ok (Self :: Disabled) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub struct PutObjectRetention ; impl Op for PutObjectRetention { type Input = PutObjectRetentionRequest ; type Output = PutObjectRetentionOutput ; }

pub struct RestoreObjectOutput { restore_output_path : Option < RestoreOutputPath > , request_charged : Option < RequestCharged > , }

pub type ServerSideEncryptionRules = Vec < ServerSideEncryptionRule > ;

pub type ConfirmRemoveSelfBucketAccess = bool ;

pub type KeyCount = i32 ;

pub type ChecksumSha1 = String ;

pub struct GetBucketLifecycleConfigurationRequest { bucket : Option < BucketName > , expected_bucket_owner : Option < AccountId > , }

pub type GrantReadAcp = String ;

pub struct DeletedObject { key : Option < ObjectKey > , version_id : Option < ObjectVersionId > , delete_marker_version_id : Option < DeleteMarkerVersionId > , delete_marker : Option < DeleteMarker > , }

pub struct GetPublicAccessBlockOutput { public_access_block_configuration : Option < PublicAccessBlockConfiguration > , }

pub struct Part { part_number : Option < PartNumber > , e_tag : Option < ETag > , checksum_sha1 : Option < ChecksumSha1 > , checksum_crc32 : Option < ChecksumCrc32 > , last_modified : Option < LastModified > , size : Option < Size > , checksum_sha256 : Option < ChecksumSha256 > , checksum_crc32_c : Option < ChecksumCrc32c > , }

pub struct PutObjectAcl ; impl Op for PutObjectAcl { type Input = PutObjectAclRequest ; type Output = PutObjectAclOutput ; }

pub struct PutObjectTagging ; impl Op for PutObjectTagging { type Input = PutObjectTaggingRequest ; type Output = PutObjectTaggingOutput ; }

pub struct UploadPartCopy ; impl Op for UploadPartCopy { type Input = UploadPartCopyRequest ; type Output = UploadPartCopyOutput ; }

pub struct PutBucketNotificationConfiguration ; impl Op for PutBucketNotificationConfiguration { type Input = PutBucketNotificationConfigurationRequest ; type Output = Destination ; }

pub struct PutBucketCorsRequest { bucket : Option < BucketName > , cors_configuration : Option < CorsConfiguration > , checksum_algorithm : Option < ChecksumAlgorithm > , content_md5 : Option < ContentMd5 > , expected_bucket_owner : Option < AccountId > , }

pub struct DeleteObject ; impl Op for DeleteObject { type Input = DeleteObjectRequest ; type Output = DeleteObjectOutput ; }

pub type BypassGovernanceRetention = bool ;

pub struct Condition { http_error_code_returned_equals : Option < HttpErrorCodeReturnedEquals > , key_prefix_equals : Option < KeyPrefixEquals > , }

pub struct RestoreObjectRequest { key : Option < ObjectKey > , expected_bucket_owner : Option < AccountId > , request_payer : Option < RequestPayer > , checksum_algorithm : Option < ChecksumAlgorithm > , restore_request : Option < RestoreRequest > , version_id : Option < ObjectVersionId > , bucket : Option < BucketName > , }

pub type ContentLength = i64 ;

pub type Size = i64 ;

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum SseKmsEncryptedObjectsStatus { Enabled , Disabled , } impl AsRef < str > for SseKmsEncryptedObjectsStatus { fn as_ref (& self) -> & str { match self { Self :: Enabled => "Enabled" , Self :: Disabled => "Disabled" , } } } impl TryFrom < & str > for SseKmsEncryptedObjectsStatus { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "Enabled" => Ok (Self :: Enabled) , "Disabled" => Ok (Self :: Disabled) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub type MultipartUploadId = String ;

pub type ExpiredObjectDeleteMarker = bool ;

pub struct BucketAlreadyOwnedByYou { }

pub struct AbortMultipartUpload ; impl Op for AbortMultipartUpload { type Input = AbortMultipartUploadRequest ; type Output = AbortMultipartUploadOutput ; }

pub type ContentType = String ;

pub struct GetBucketIntelligentTieringConfiguration ; impl Op for GetBucketIntelligentTieringConfiguration { type Input = GetBucketIntelligentTieringConfigurationRequest ; type Output = GetBucketIntelligentTieringConfigurationOutput ; }

pub struct LifecycleRule { abort_incomplete_multipart_upload : Option < AbortIncompleteMultipartUpload > , filter : Option < LifecycleRuleFilter > , prefix : Option < Prefix > , status : Option < ExpirationStatus > , id : Option < Id > , noncurrent_version_expiration : Option < NoncurrentVersionExpiration > , noncurrent_version_transitions : Option < NoncurrentVersionTransitionList > , expiration : Option < LifecycleExpiration > , transitions : Option < TransitionList > , }

pub type LambdaFunctionConfigurationList = Vec < LambdaFunctionConfiguration > ;

pub struct AccelerateConfiguration { status : Option < BucketAccelerateStatus > , }

pub struct QueueConfiguration { events : Option < EventList > , filter : Option < NotificationConfigurationFilter > , queue_arn : Option < QueueArn > , id : Option < NotificationId > , }

pub type Expires = String ;

pub struct ListBucketMetricsConfigurationsRequest { continuation_token : Option < Token > , bucket : Option < BucketName > , expected_bucket_owner : Option < AccountId > , }

pub struct ListBucketsOutput { buckets : Option < Buckets > , owner : Option < Owner > , }

pub type MultipartUploadList = Vec < MultipartUpload > ;

pub struct ContinuationEvent { }

pub struct PutObjectTaggingRequest { tagging : Option < Tagging > , expected_bucket_owner : Option < AccountId > , checksum_algorithm : Option < ChecksumAlgorithm > , key : Option < ObjectKey > , bucket : Option < BucketName > , version_id : Option < ObjectVersionId > , content_md5 : Option < ContentMd5 > , request_payer : Option < RequestPayer > , }

pub type HttpRedirectCode = String ;

pub struct NoSuchUpload { }

pub struct PutBucketMetricsConfiguration ; impl Op for PutBucketMetricsConfiguration { type Input = PutBucketMetricsConfigurationRequest ; type Output = Destination ; }

pub type AnalyticsConfigurationList = Vec < AnalyticsConfiguration > ;

pub struct ListObjectsOutput { next_marker : Option < NextMarker > , contents : Option < ObjectList > , marker : Option < Marker > , delimiter : Option < Delimiter > , encoding_type : Option < EncodingType > , common_prefixes : Option < CommonPrefixList > , name : Option < BucketName > , prefix : Option < Prefix > , is_truncated : Option < IsTruncated > , max_keys : Option < MaxKeys > , }

pub type AllowedHeaders = Vec < AllowedHeader > ;

pub type ReplaceKeyWith = String ;

pub type CopySourceIfMatch = String ;

pub struct GetObjectRetention ; impl Op for GetObjectRetention { type Input = GetObjectRetentionRequest ; type Output = GetObjectRetentionOutput ; }

pub struct DeleteBucketTaggingRequest { expected_bucket_owner : Option < AccountId > , bucket : Option < BucketName > , }

pub struct InventoryS3BucketDestination { account_id : Option < AccountId > , encryption : Option < InventoryEncryption > , prefix : Option < Prefix > , bucket : Option < BucketName > , format : Option < InventoryFormat > , }

pub struct PutBucketInventoryConfigurationRequest { id : Option < InventoryId > , expected_bucket_owner : Option < AccountId > , bucket : Option < BucketName > , inventory_configuration : Option < InventoryConfiguration > , }

pub struct PutObjectLegalHoldOutput { request_charged : Option < RequestCharged > , }

pub struct ListObjectVersions ; impl Op for ListObjectVersions { type Input = ListObjectVersionsRequest ; type Output = ListObjectVersionsOutput ; }

pub type PartsList = Vec < ObjectPart > ;

pub type RequestRoute = String ;

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum BucketCannedAcl { private , public_read , public_read_write , authenticated_read , } impl AsRef < str > for BucketCannedAcl { fn as_ref (& self) -> & str { match self { Self :: private => "private" , Self :: public_read => "public-read" , Self :: public_read_write => "public-read-write" , Self :: authenticated_read => "authenticated-read" , } } } impl TryFrom < & str > for BucketCannedAcl { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "private" => Ok (Self :: private) , "public-read" => Ok (Self :: public_read) , "public-read-write" => Ok (Self :: public_read_write) , "authenticated-read" => Ok (Self :: authenticated_read) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub type EnableRequestProgress = bool ;

pub struct RestoreRequest { output_location : Option < OutputLocation > , select_parameters : Option < SelectParameters > , r#type : Option < RestoreRequestType > , tier : Option < Tier > , glacier_job_parameters : Option < GlacierJobParameters > , days : Option < Days > , description : Option < Description > , }

pub struct DeleteBucketPolicy ; impl Op for DeleteBucketPolicy { type Input = DeleteBucketPolicyRequest ; type Output = Destination ; }

pub type ChecksumCrc32c = String ;

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum ReplicationTimeStatus { Enabled , Disabled , } impl AsRef < str > for ReplicationTimeStatus { fn as_ref (& self) -> & str { match self { Self :: Enabled => "Enabled" , Self :: Disabled => "Disabled" , } } } impl TryFrom < & str > for ReplicationTimeStatus { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "Enabled" => Ok (Self :: Enabled) , "Disabled" => Ok (Self :: Disabled) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub struct StatsEvent { details : Option < Stats > , }

pub type AccessPointArn = String ;

pub type ReplaceKeyPrefixWith = String ;

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum ExpressionType { SQL , } impl AsRef < str > for ExpressionType { fn as_ref (& self) -> & str { match self { Self :: SQL => "SQL" , } } } impl TryFrom < & str > for ExpressionType { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "SQL" => Ok (Self :: SQL) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub struct PutObjectLockConfiguration ; impl Op for PutObjectLockConfiguration { type Input = PutObjectLockConfigurationRequest ; type Output = PutObjectLockConfigurationOutput ; }

pub struct Delete { quiet : Option < Quiet > , objects : Option < ObjectIdentifierList > , }

pub struct GetBucketWebsiteOutput { error_document : Option < ErrorDocument > , routing_rules : Option < RoutingRules > , index_document : Option < IndexDocument > , redirect_all_requests_to : Option < RedirectAllRequestsTo > , }

pub struct ListBuckets ; impl Op for ListBuckets { type Input = Destination ; type Output = ListBucketsOutput ; }

pub struct GetObjectTorrentOutput { body : Option < StreamingBlob > , request_charged : Option < RequestCharged > , }

pub type DisplayName = String ;

pub struct PutObjectLegalHoldRequest { bucket : Option < BucketName > , content_md5 : Option < ContentMd5 > , legal_hold : Option < ObjectLockLegalHold > , expected_bucket_owner : Option < AccountId > , checksum_algorithm : Option < ChecksumAlgorithm > , version_id : Option < ObjectVersionId > , request_payer : Option < RequestPayer > , key : Option < ObjectKey > , }

pub type QuoteCharacter = String ;

pub struct ReplicationRuleAndOperator { prefix : Option < Prefix > , tags : Option < TagSet > , }

pub struct ErrorDocument { key : Option < ObjectKey > , }

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum ServerSideEncryption { AES256 , aws_kms , } impl AsRef < str > for ServerSideEncryption { fn as_ref (& self) -> & str { match self { Self :: AES256 => "AES256" , Self :: aws_kms => "aws:kms" , } } } impl TryFrom < & str > for ServerSideEncryption { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "AES256" => Ok (Self :: AES256) , "aws:kms" => Ok (Self :: aws_kms) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub type Years = i32 ;

pub struct PutObjectOutput { ssekms_key_id : Option < SsekmsKeyId > , server_side_encryption : Option < ServerSideEncryption > , version_id : Option < ObjectVersionId > , bucket_key_enabled : Option < BucketKeyEnabled > , checksum_crc32 : Option < ChecksumCrc32 > , e_tag : Option < ETag > , ssekms_encryption_context : Option < SsekmsEncryptionContext > , sse_customer_algorithm : Option < SseCustomerAlgorithm > , expiration : Option < Expiration > , checksum_crc32_c : Option < ChecksumCrc32c > , checksum_sha1 : Option < ChecksumSha1 > , checksum_sha256 : Option < ChecksumSha256 > , request_charged : Option < RequestCharged > , sse_customer_key_md5 : Option < SseCustomerKeyMd5 > , }

pub struct CreateMultipartUpload ; impl Op for CreateMultipartUpload { type Input = CreateMultipartUploadRequest ; type Output = CreateMultipartUploadOutput ; }

pub struct MetadataEntry { name : Option < MetadataKey > , value : Option < MetadataValue > , }

pub type Setting = bool ;

pub type CopySource = String ;

pub type Start = i64 ;

pub type LifecycleRules = Vec < LifecycleRule > ;

pub struct GetObjectRetentionOutput { retention : Option < ObjectLockRetention > , }

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum AnalyticsS3ExportFileFormat { CSV , } impl AsRef < str > for AnalyticsS3ExportFileFormat { fn as_ref (& self) -> & str { match self { Self :: CSV => "CSV" , } } } impl TryFrom < & str > for AnalyticsS3ExportFileFormat { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "CSV" => Ok (Self :: CSV) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub struct DeleteBucketInventoryConfigurationRequest { id : Option < InventoryId > , bucket : Option < BucketName > , expected_bucket_owner : Option < AccountId > , }

pub struct GlacierJobParameters { tier : Option < Tier > , }

pub struct HeadObject ; impl Op for HeadObject { type Input = HeadObjectRequest ; type Output = HeadObjectOutput ; }

pub struct ListObjectsV2Output { start_after : Option < StartAfter > , is_truncated : Option < IsTruncated > , max_keys : Option < MaxKeys > , common_prefixes : Option < CommonPrefixList > , contents : Option < ObjectList > , delimiter : Option < Delimiter > , key_count : Option < KeyCount > , name : Option < BucketName > , continuation_token : Option < Token > , encoding_type : Option < EncodingType > , next_continuation_token : Option < NextToken > , prefix : Option < Prefix > , }

pub struct AnalyticsAndOperator { prefix : Option < Prefix > , tags : Option < TagSet > , }

pub type InventoryId = String ;

pub struct GetBucketLocationRequest { bucket : Option < BucketName > , expected_bucket_owner : Option < AccountId > , }

pub type NextKeyMarker = String ;

pub struct Ssekms { key_id : Option < SsekmsKeyId > , }

pub struct Object { storage_class : Option < ObjectStorageClass > , checksum_algorithm : Option < ChecksumAlgorithmList > , owner : Option < Owner > , size : Option < Size > , e_tag : Option < ETag > , key : Option < ObjectKey > , last_modified : Option < LastModified > , }

pub struct GetObject ; impl Op for GetObject { type Input = GetObjectRequest ; type Output = GetObjectOutput ; }

pub struct ListPartsOutput { upload_id : Option < MultipartUploadId > , abort_date : Option < AbortDate > , max_parts : Option < MaxParts > , is_truncated : Option < IsTruncated > , next_part_number_marker : Option < NextPartNumberMarker > , part_number_marker : Option < PartNumberMarker > , checksum_algorithm : Option < ChecksumAlgorithm > , request_charged : Option < RequestCharged > , initiator : Option < Initiator > , parts : Option < Parts > , key : Option < ObjectKey > , owner : Option < Owner > , storage_class : Option < StorageClass > , abort_rule_id : Option < AbortRuleId > , bucket : Option < BucketName > , }

pub struct DeleteBucketInventoryConfiguration ; impl Op for DeleteBucketInventoryConfiguration { type Input = DeleteBucketInventoryConfigurationRequest ; type Output = Destination ; }

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum ObjectLockRetentionMode { GOVERNANCE , COMPLIANCE , } impl AsRef < str > for ObjectLockRetentionMode { fn as_ref (& self) -> & str { match self { Self :: GOVERNANCE => "GOVERNANCE" , Self :: COMPLIANCE => "COMPLIANCE" , } } } impl TryFrom < & str > for ObjectLockRetentionMode { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "GOVERNANCE" => Ok (Self :: GOVERNANCE) , "COMPLIANCE" => Ok (Self :: COMPLIANCE) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub type BucketKeyEnabled = bool ;

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum ReplicationStatus { COMPLETE , PENDING , FAILED , REPLICA , } impl AsRef < str > for ReplicationStatus { fn as_ref (& self) -> & str { match self { Self :: COMPLETE => "COMPLETE" , Self :: PENDING => "PENDING" , Self :: FAILED => "FAILED" , Self :: REPLICA => "REPLICA" , } } } impl TryFrom < & str > for ReplicationStatus { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "COMPLETE" => Ok (Self :: COMPLETE) , "PENDING" => Ok (Self :: PENDING) , "FAILED" => Ok (Self :: FAILED) , "REPLICA" => Ok (Self :: REPLICA) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub struct Tagging { tag_set : Option < TagSet > , }

pub struct TargetGrant { grantee : Option < Grantee > , permission : Option < BucketLogsPermission > , }

pub type TransitionList = Vec < Transition > ;

pub struct MetricsConfiguration { filter : Option < MetricsFilter > , id : Option < MetricsId > , }

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum IntelligentTieringStatus { Enabled , Disabled , } impl AsRef < str > for IntelligentTieringStatus { fn as_ref (& self) -> & str { match self { Self :: Enabled => "Enabled" , Self :: Disabled => "Disabled" , } } } impl TryFrom < & str > for IntelligentTieringStatus { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "Enabled" => Ok (Self :: Enabled) , "Disabled" => Ok (Self :: Disabled) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub struct PutObjectTaggingOutput { version_id : Option < ObjectVersionId > , }

pub struct GetObjectTaggingOutput { tag_set : Option < TagSet > , version_id : Option < ObjectVersionId > , }

pub struct CompleteMultipartUploadOutput { checksum_sha256 : Option < ChecksumSha256 > , expiration : Option < Expiration > , key : Option < ObjectKey > , server_side_encryption : Option < ServerSideEncryption > , e_tag : Option < ETag > , checksum_crc32_c : Option < ChecksumCrc32c > , version_id : Option < ObjectVersionId > , checksum_crc32 : Option < ChecksumCrc32 > , ssekms_key_id : Option < SsekmsKeyId > , checksum_sha1 : Option < ChecksumSha1 > , bucket_key_enabled : Option < BucketKeyEnabled > , bucket : Option < BucketName > , location : Option < Location > , request_charged : Option < RequestCharged > , }

pub struct PutBucketReplicationRequest { bucket : Option < BucketName > , content_md5 : Option < ContentMd5 > , checksum_algorithm : Option < ChecksumAlgorithm > , replication_configuration : Option < ReplicationConfiguration > , token : Option < ObjectLockToken > , expected_bucket_owner : Option < AccountId > , }

pub struct GetBucketPolicyStatus ; impl Op for GetBucketPolicyStatus { type Input = GetBucketPolicyStatusRequest ; type Output = GetBucketPolicyStatusOutput ; }

pub struct DeleteBucketLifecycle ; impl Op for DeleteBucketLifecycle { type Input = DeleteBucketLifecycleRequest ; type Output = Destination ; }

pub struct PutBucketEncryption ; impl Op for PutBucketEncryption { type Input = PutBucketEncryptionRequest ; type Output = Destination ; }

pub struct VersioningConfiguration { mfa_delete : Option < MfaDelete > , status : Option < BucketVersioningStatus > , }

pub type TieringList = Vec < Tiering > ;

pub struct DeleteBucketOwnershipControls ; impl Op for DeleteBucketOwnershipControls { type Input = DeleteBucketOwnershipControlsRequest ; type Output = Destination ; }

pub struct LifecycleExpiration { days : Option < Days > , date : Option < Date > , expired_object_delete_marker : Option < ExpiredObjectDeleteMarker > , }

pub struct GetBucketEncryptionRequest { expected_bucket_owner : Option < AccountId > , bucket : Option < BucketName > , }

pub struct ListMultipartUploads ; impl Op for ListMultipartUploads { type Input = ListMultipartUploadsRequest ; type Output = ListMultipartUploadsOutput ; }

pub struct ServerSideEncryptionRule { apply_server_side_encryption_by_default : Option < ServerSideEncryptionByDefault > , bucket_key_enabled : Option < BucketKeyEnabled > , }

pub struct DeleteBucketAnalyticsConfiguration ; impl Op for DeleteBucketAnalyticsConfiguration { type Input = DeleteBucketAnalyticsConfigurationRequest ; type Output = Destination ; }

pub struct TopicConfiguration { id : Option < NotificationId > , filter : Option < NotificationConfigurationFilter > , topic_arn : Option < TopicArn > , events : Option < EventList > , }

pub struct CopyObjectRequest { request_payer : Option < RequestPayer > , sse_customer_algorithm : Option < SseCustomerAlgorithm > , server_side_encryption : Option < ServerSideEncryption > , cache_control : Option < CacheControl > , grant_full_control : Option < GrantFullControl > , content_disposition : Option < ContentDisposition > , acl : Option < ObjectCannedAcl > , object_lock_legal_hold_status : Option < ObjectLockLegalHoldStatus > , ssekms_encryption_context : Option < SsekmsEncryptionContext > , ssekms_key_id : Option < SsekmsKeyId > , bucket : Option < BucketName > , expected_bucket_owner : Option < AccountId > , object_lock_mode : Option < ObjectLockMode > , sse_customer_key : Option < SseCustomerKey > , copy_source_if_match : Option < CopySourceIfMatch > , copy_source_if_none_match : Option < CopySourceIfNoneMatch > , content_encoding : Option < ContentEncoding > , copy_source_sse_customer_key : Option < CopySourceSseCustomerKey > , expires : Option < Expires > , copy_source_sse_customer_algorithm : Option < CopySourceSseCustomerAlgorithm > , checksum_algorithm : Option < ChecksumAlgorithm > , sse_customer_key_md5 : Option < SseCustomerKeyMd5 > , metadata : Option < Metadata > , object_lock_retain_until_date : Option < ObjectLockRetainUntilDate > , tagging_directive : Option < TaggingDirective > , grant_read_acp : Option < GrantReadAcp > , bucket_key_enabled : Option < BucketKeyEnabled > , key : Option < ObjectKey > , storage_class : Option < StorageClass > , website_redirect_location : Option < WebsiteRedirectLocation > , content_language : Option < ContentLanguage > , copy_source : Option < CopySource > , grant_write_acp : Option < GrantWriteAcp > , copy_source_if_modified_since : Option < CopySourceIfModifiedSince > , copy_source_sse_customer_key_md5 : Option < CopySourceSseCustomerKeyMd5 > , content_type : Option < ContentType > , expected_source_bucket_owner : Option < AccountId > , metadata_directive : Option < MetadataDirective > , copy_source_if_unmodified_since : Option < CopySourceIfUnmodifiedSince > , grant_read : Option < GrantRead > , tagging : Option < TaggingHeader > , }

pub struct GetPublicAccessBlockRequest { bucket : Option < BucketName > , expected_bucket_owner : Option < AccountId > , }

pub type UploadIdMarker = String ;

pub struct StorageClassAnalysisDataExport { destination : Option < AnalyticsExportDestination > , output_schema_version : Option < StorageClassAnalysisSchemaVersion > , }

pub type QuoteEscapeCharacter = String ;

pub struct DeleteBucketMetricsConfiguration ; impl Op for DeleteBucketMetricsConfiguration { type Input = DeleteBucketMetricsConfigurationRequest ; type Output = Destination ; }

pub struct PutPublicAccessBlock ; impl Op for PutPublicAccessBlock { type Input = PutPublicAccessBlockRequest ; type Output = Destination ; }

pub struct S3Location { access_control_list : Option < Grants > , canned_acl : Option < ObjectCannedAcl > , tagging : Option < Tagging > , encryption : Option < Encryption > , bucket_name : Option < BucketName > , prefix : Option < LocationPrefix > , storage_class : Option < StorageClass > , user_metadata : Option < UserMetadata > , }

pub type ChecksumAlgorithmList = Vec < ChecksumAlgorithm > ;

pub struct GetObjectTaggingRequest { expected_bucket_owner : Option < AccountId > , version_id : Option < ObjectVersionId > , key : Option < ObjectKey > , request_payer : Option < RequestPayer > , bucket : Option < BucketName > , }

pub struct GetBucketAnalyticsConfiguration ; impl Op for GetBucketAnalyticsConfiguration { type Input = GetBucketAnalyticsConfigurationRequest ; type Output = GetBucketAnalyticsConfigurationOutput ; }

pub type Parts = Vec < Part > ;

pub type IfUnmodifiedSince = String ;

pub struct PutBucketIntelligentTieringConfigurationRequest { intelligent_tiering_configuration : Option < IntelligentTieringConfiguration > , id : Option < IntelligentTieringId > , bucket : Option < BucketName > , }

pub struct GetObjectAclOutput { owner : Option < Owner > , request_charged : Option < RequestCharged > , grants : Option < Grants > , }

pub struct AccessControlPolicy { grants : Option < Grants > , owner : Option < Owner > , }

pub struct DefaultRetention { mode : Option < ObjectLockRetentionMode > , years : Option < Years > , days : Option < Days > , }

pub struct GetBucketAnalyticsConfigurationRequest { expected_bucket_owner : Option < AccountId > , bucket : Option < BucketName > , id : Option < AnalyticsId > , }

pub struct GetBucketTaggingOutput { tag_set : Option < TagSet > , }

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum IntelligentTieringAccessTier { ARCHIVE_ACCESS , DEEP_ARCHIVE_ACCESS , } impl AsRef < str > for IntelligentTieringAccessTier { fn as_ref (& self) -> & str { match self { Self :: ARCHIVE_ACCESS => "ARCHIVE_ACCESS" , Self :: DEEP_ARCHIVE_ACCESS => "DEEP_ARCHIVE_ACCESS" , } } } impl TryFrom < & str > for IntelligentTieringAccessTier { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "ARCHIVE_ACCESS" => Ok (Self :: ARCHIVE_ACCESS) , "DEEP_ARCHIVE_ACCESS" => Ok (Self :: DEEP_ARCHIVE_ACCESS) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub struct ObjectIdentifier { key : Option < ObjectKey > , version_id : Option < ObjectVersionId > , }

pub struct GetBucketAcl ; impl Op for GetBucketAcl { type Input = GetBucketAclRequest ; type Output = GetBucketAclOutput ; }

pub type BytesReturned = i64 ;

pub type Uri = String ;

pub struct GetBucketMetricsConfiguration ; impl Op for GetBucketMetricsConfiguration { type Input = GetBucketMetricsConfigurationRequest ; type Output = GetBucketMetricsConfigurationOutput ; }

pub struct AnalyticsConfiguration { storage_class_analysis : Option < StorageClassAnalysis > , id : Option < AnalyticsId > , filter : Option < AnalyticsFilter > , }

pub type ContentLanguage = String ;

pub type Marker = String ;

pub type RoutingRules = Vec < RoutingRule > ;

pub type TopicArn = String ;

pub struct DeleteObjects ; impl Op for DeleteObjects { type Input = DeleteObjectsRequest ; type Output = DeleteObjectsOutput ; }

pub struct GetBucketWebsite ; impl Op for GetBucketWebsite { type Input = GetBucketWebsiteRequest ; type Output = GetBucketWebsiteOutput ; }

pub struct GetBucketRequestPayment ; impl Op for GetBucketRequestPayment { type Input = GetBucketRequestPaymentRequest ; type Output = GetBucketRequestPaymentOutput ; }

pub struct SelectObjectContentOutput { payload : Option < SelectObjectContentEventStream > , }

pub struct EncryptionConfiguration { replica_kms_key_id : Option < ReplicaKmsKeyId > , }

pub type AllowedOrigins = Vec < AllowedOrigin > ;

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum MetadataDirective { COPY , REPLACE , } impl AsRef < str > for MetadataDirective { fn as_ref (& self) -> & str { match self { Self :: COPY => "COPY" , Self :: REPLACE => "REPLACE" , } } } impl TryFrom < & str > for MetadataDirective { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "COPY" => Ok (Self :: COPY) , "REPLACE" => Ok (Self :: REPLACE) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub struct NoSuchKey { }

pub struct OutputLocation { s3 : Option < S3Location > , }

pub struct ObjectLockRule { default_retention : Option < DefaultRetention > , }

pub struct BucketAlreadyExists { }

pub struct PutBucketEncryptionRequest { server_side_encryption_configuration : Option < ServerSideEncryptionConfiguration > , expected_bucket_owner : Option < AccountId > , content_md5 : Option < ContentMd5 > , checksum_algorithm : Option < ChecksumAlgorithm > , bucket : Option < BucketName > , }

pub type TargetBucket = String ;

pub struct DeleteMarkerEntry { is_latest : Option < IsLatest > , version_id : Option < ObjectVersionId > , last_modified : Option < LastModified > , owner : Option < Owner > , key : Option < ObjectKey > , }

pub type IntelligentTieringId = String ;

pub type InventoryOptionalFields = Vec < InventoryOptionalField > ;

pub struct GetObjectAcl ; impl Op for GetObjectAcl { type Input = GetObjectAclRequest ; type Output = GetObjectAclOutput ; }

pub struct GetBucketPolicyRequest { expected_bucket_owner : Option < AccountId > , bucket : Option < BucketName > , }

pub type BytesScanned = i64 ;

pub type AbortDate = String ;

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum InventoryOptionalField { Size , LastModifiedDate , StorageClass , ETag , IsMultipartUploaded , ReplicationStatus , EncryptionStatus , ObjectLockRetainUntilDate , ObjectLockMode , ObjectLockLegalHoldStatus , IntelligentTieringAccessTier , BucketKeyStatus , ChecksumAlgorithm , } impl AsRef < str > for InventoryOptionalField { fn as_ref (& self) -> & str { match self { Self :: Size => "Size" , Self :: LastModifiedDate => "LastModifiedDate" , Self :: StorageClass => "StorageClass" , Self :: ETag => "ETag" , Self :: IsMultipartUploaded => "IsMultipartUploaded" , Self :: ReplicationStatus => "ReplicationStatus" , Self :: EncryptionStatus => "EncryptionStatus" , Self :: ObjectLockRetainUntilDate => "ObjectLockRetainUntilDate" , Self :: ObjectLockMode => "ObjectLockMode" , Self :: ObjectLockLegalHoldStatus => "ObjectLockLegalHoldStatus" , Self :: IntelligentTieringAccessTier => "IntelligentTieringAccessTier" , Self :: BucketKeyStatus => "BucketKeyStatus" , Self :: ChecksumAlgorithm => "ChecksumAlgorithm" , } } } impl TryFrom < & str > for InventoryOptionalField { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "Size" => Ok (Self :: Size) , "LastModifiedDate" => Ok (Self :: LastModifiedDate) , "StorageClass" => Ok (Self :: StorageClass) , "ETag" => Ok (Self :: ETag) , "IsMultipartUploaded" => Ok (Self :: IsMultipartUploaded) , "ReplicationStatus" => Ok (Self :: ReplicationStatus) , "EncryptionStatus" => Ok (Self :: EncryptionStatus) , "ObjectLockRetainUntilDate" => Ok (Self :: ObjectLockRetainUntilDate) , "ObjectLockMode" => Ok (Self :: ObjectLockMode) , "ObjectLockLegalHoldStatus" => Ok (Self :: ObjectLockLegalHoldStatus) , "IntelligentTieringAccessTier" => Ok (Self :: IntelligentTieringAccessTier) , "BucketKeyStatus" => Ok (Self :: BucketKeyStatus) , "ChecksumAlgorithm" => Ok (Self :: ChecksumAlgorithm) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub struct ObjectPart { checksum_crc32 : Option < ChecksumCrc32 > , part_number : Option < PartNumber > , checksum_crc32_c : Option < ChecksumCrc32c > , checksum_sha1 : Option < ChecksumSha1 > , checksum_sha256 : Option < ChecksumSha256 > , size : Option < Size > , }

pub struct EventBridgeConfiguration { }

pub type ReplicationRules = Vec < ReplicationRule > ;

pub type SelectObjectContentEventStream = Vec < String > ;

pub type ObjectIdentifierList = Vec < ObjectIdentifier > ;

pub struct S3KeyFilter { filter_rules : Option < FilterRuleList > , }

pub struct AbortMultipartUploadRequest { request_payer : Option < RequestPayer > , key : Option < ObjectKey > , upload_id : Option < MultipartUploadId > , expected_bucket_owner : Option < AccountId > , bucket : Option < BucketName > , }

pub struct GetObjectTorrentRequest { expected_bucket_owner : Option < AccountId > , bucket : Option < BucketName > , request_payer : Option < RequestPayer > , key : Option < ObjectKey > , }

pub struct GetBucketReplicationRequest { bucket : Option < BucketName > , expected_bucket_owner : Option < AccountId > , }

pub struct ListObjectsV2 ; impl Op for ListObjectsV2 { type Input = ListObjectsV2Request ; type Output = ListObjectsV2Output ; }

pub struct GetBucketAnalyticsConfigurationOutput { analytics_configuration : Option < AnalyticsConfiguration > , }

pub type LambdaFunctionArn = String ;

pub struct PutBucketLogging ; impl Op for PutBucketLogging { type Input = PutBucketLoggingRequest ; type Output = Destination ; }

pub struct PutBucketOwnershipControlsRequest { expected_bucket_owner : Option < AccountId > , ownership_controls : Option < OwnershipControls > , bucket : Option < BucketName > , content_md5 : Option < ContentMd5 > , }

pub struct ListObjectVersionsOutput { delimiter : Option < Delimiter > , versions : Option < ObjectVersionList > , prefix : Option < Prefix > , key_marker : Option < KeyMarker > , common_prefixes : Option < CommonPrefixList > , delete_markers : Option < DeleteMarkers > , name : Option < BucketName > , next_key_marker : Option < NextKeyMarker > , next_version_id_marker : Option < NextVersionIdMarker > , is_truncated : Option < IsTruncated > , version_id_marker : Option < VersionIdMarker > , max_keys : Option < MaxKeys > , encoding_type : Option < EncodingType > , }

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum TransitionStorageClass { GLACIER , STANDARD_IA , ONEZONE_IA , INTELLIGENT_TIERING , DEEP_ARCHIVE , GLACIER_IR , } impl AsRef < str > for TransitionStorageClass { fn as_ref (& self) -> & str { match self { Self :: GLACIER => "GLACIER" , Self :: STANDARD_IA => "STANDARD_IA" , Self :: ONEZONE_IA => "ONEZONE_IA" , Self :: INTELLIGENT_TIERING => "INTELLIGENT_TIERING" , Self :: DEEP_ARCHIVE => "DEEP_ARCHIVE" , Self :: GLACIER_IR => "GLACIER_IR" , } } } impl TryFrom < & str > for TransitionStorageClass { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "GLACIER" => Ok (Self :: GLACIER) , "STANDARD_IA" => Ok (Self :: STANDARD_IA) , "ONEZONE_IA" => Ok (Self :: ONEZONE_IA) , "INTELLIGENT_TIERING" => Ok (Self :: INTELLIGENT_TIERING) , "DEEP_ARCHIVE" => Ok (Self :: DEEP_ARCHIVE) , "GLACIER_IR" => Ok (Self :: GLACIER_IR) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub struct Tag { key : Option < ObjectKey > , value : Option < Value > , }

pub struct LambdaFunctionConfiguration { events : Option < EventList > , lambda_function_arn : Option < LambdaFunctionArn > , id : Option < NotificationId > , filter : Option < NotificationConfigurationFilter > , }

pub struct PutBucketWebsite ; impl Op for PutBucketWebsite { type Input = PutBucketWebsiteRequest ; type Output = Destination ; }

pub struct ObjectVersion { checksum_algorithm : Option < ChecksumAlgorithmList > , is_latest : Option < IsLatest > , storage_class : Option < ObjectVersionStorageClass > , version_id : Option < ObjectVersionId > , last_modified : Option < LastModified > , key : Option < ObjectKey > , owner : Option < Owner > , size : Option < Size > , e_tag : Option < ETag > , }

pub struct GetBucketLocation ; impl Op for GetBucketLocation { type Input = GetBucketLocationRequest ; type Output = GetBucketLocationOutput ; }

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum Type { CanonicalUser , AmazonCustomerByEmail , Group , } impl AsRef < str > for Type { fn as_ref (& self) -> & str { match self { Self :: CanonicalUser => "CanonicalUser" , Self :: AmazonCustomerByEmail => "AmazonCustomerByEmail" , Self :: Group => "Group" , } } } impl TryFrom < & str > for Type { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "CanonicalUser" => Ok (Self :: CanonicalUser) , "AmazonCustomerByEmail" => Ok (Self :: AmazonCustomerByEmail) , "Group" => Ok (Self :: Group) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub struct UploadPartOutput { bucket_key_enabled : Option < BucketKeyEnabled > , server_side_encryption : Option < ServerSideEncryption > , checksum_sha256 : Option < ChecksumSha256 > , checksum_crc32 : Option < ChecksumCrc32 > , checksum_sha1 : Option < ChecksumSha1 > , request_charged : Option < RequestCharged > , checksum_crc32_c : Option < ChecksumCrc32c > , e_tag : Option < ETag > , sse_customer_algorithm : Option < SseCustomerAlgorithm > , sse_customer_key_md5 : Option < SseCustomerKeyMd5 > , ssekms_key_id : Option < SsekmsKeyId > , }

pub struct GetBucketEncryption ; impl Op for GetBucketEncryption { type Input = GetBucketEncryptionRequest ; type Output = GetBucketEncryptionOutput ; }

pub struct GetBucketPolicyOutput { policy : Option < Policy > , }

pub type AllowQuotedRecordDelimiter = bool ;

pub type CacheControl = String ;

pub struct GetBucketReplication ; impl Op for GetBucketReplication { type Input = GetBucketReplicationRequest ; type Output = GetBucketReplicationOutput ; }

pub type AnalyticsId = String ;

pub struct GetObjectLockConfiguration ; impl Op for GetObjectLockConfiguration { type Input = GetObjectLockConfigurationRequest ; type Output = GetObjectLockConfigurationOutput ; }

pub type AllowedOrigin = String ;

pub type ErrorMessage = String ;

pub type ExposeHeaders = Vec < ExposeHeader > ;

pub type KmsContext = String ;

pub type ContentDisposition = String ;

pub struct DeleteObjectTaggingOutput { version_id : Option < ObjectVersionId > , }

pub struct LifecycleRuleAndOperator { prefix : Option < Prefix > , object_size_greater_than : Option < ObjectSizeGreaterThanBytes > , tags : Option < TagSet > , object_size_less_than : Option < ObjectSizeLessThanBytes > , }

pub type HttpErrorCodeReturnedEquals = String ;

pub struct Metrics { status : Option < MetricsStatus > , event_threshold : Option < ReplicationTimeValue > , }

pub type NextToken = String ;

pub type ReplicaKmsKeyId = String ;

pub struct Sses3 { }

pub type NextUploadIdMarker = String ;

pub type IntelligentTieringDays = i32 ;

pub type GrantWrite = String ;

pub struct SelectObjectContentRequest { output_serialization : Option < OutputSerialization > , sse_customer_key : Option < SseCustomerKey > , expected_bucket_owner : Option < AccountId > , bucket : Option < BucketName > , expression_type : Option < ExpressionType > , request_progress : Option < RequestProgress > , scan_range : Option < ScanRange > , input_serialization : Option < InputSerialization > , sse_customer_algorithm : Option < SseCustomerAlgorithm > , sse_customer_key_md5 : Option < SseCustomerKeyMd5 > , expression : Option < Expression > , key : Option < ObjectKey > , }

pub type MetricsFilter = Vec < String > ;

pub struct PutBucketPolicy ; impl Op for PutBucketPolicy { type Input = PutBucketPolicyRequest ; type Output = Destination ; }

pub type Restore = String ;

pub type ChecksumSha256 = String ;

pub struct GetBucketIntelligentTieringConfigurationRequest { bucket : Option < BucketName > , id : Option < IntelligentTieringId > , }

pub struct ListObjectVersionsRequest { key_marker : Option < KeyMarker > , version_id_marker : Option < VersionIdMarker > , expected_bucket_owner : Option < AccountId > , bucket : Option < BucketName > , delimiter : Option < Delimiter > , max_keys : Option < MaxKeys > , encoding_type : Option < EncodingType > , prefix : Option < Prefix > , }

pub struct UploadPartCopyRequest { copy_source_range : Option < CopySourceRange > , bucket : Option < BucketName > , copy_source_sse_customer_key_md5 : Option < CopySourceSseCustomerKeyMd5 > , sse_customer_key_md5 : Option < SseCustomerKeyMd5 > , expected_source_bucket_owner : Option < AccountId > , copy_source : Option < CopySource > , copy_source_sse_customer_algorithm : Option < CopySourceSseCustomerAlgorithm > , sse_customer_key : Option < SseCustomerKey > , copy_source_if_modified_since : Option < CopySourceIfModifiedSince > , key : Option < ObjectKey > , sse_customer_algorithm : Option < SseCustomerAlgorithm > , copy_source_if_none_match : Option < CopySourceIfNoneMatch > , copy_source_sse_customer_key : Option < CopySourceSseCustomerKey > , expected_bucket_owner : Option < AccountId > , copy_source_if_match : Option < CopySourceIfMatch > , request_payer : Option < RequestPayer > , copy_source_if_unmodified_since : Option < CopySourceIfUnmodifiedSince > , part_number : Option < PartNumber > , upload_id : Option < MultipartUploadId > , }

pub struct GetBucketMetricsConfigurationOutput { metrics_configuration : Option < MetricsConfiguration > , }

pub struct DeleteBucketMetricsConfigurationRequest { expected_bucket_owner : Option < AccountId > , id : Option < MetricsId > , bucket : Option < BucketName > , }

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum ObjectLockMode { GOVERNANCE , COMPLIANCE , } impl AsRef < str > for ObjectLockMode { fn as_ref (& self) -> & str { match self { Self :: GOVERNANCE => "GOVERNANCE" , Self :: COMPLIANCE => "COMPLIANCE" , } } } impl TryFrom < & str > for ObjectLockMode { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "GOVERNANCE" => Ok (Self :: GOVERNANCE) , "COMPLIANCE" => Ok (Self :: COMPLIANCE) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub struct Bucket { creation_date : Option < CreationDate > , name : Option < BucketName > , }

pub type NoncurrentVersionTransitionList = Vec < NoncurrentVersionTransition > ;

pub type Id = String ;

pub struct ReplicationConfiguration { rules : Option < ReplicationRules > , role : Option < Role > , }

pub type NextMarker = String ;

pub struct GetBucketLoggingOutput { logging_enabled : Option < LoggingEnabled > , }

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum OwnerOverride { Destination , } impl AsRef < str > for OwnerOverride { fn as_ref (& self) -> & str { match self { Self :: Destination => "Destination" , } } } impl TryFrom < & str > for OwnerOverride { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "Destination" => Ok (Self :: Destination) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum QuoteFields { ALWAYS , ASNEEDED , } impl AsRef < str > for QuoteFields { fn as_ref (& self) -> & str { match self { Self :: ALWAYS => "ALWAYS" , Self :: ASNEEDED => "ASNEEDED" , } } } impl TryFrom < & str > for QuoteFields { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "ALWAYS" => Ok (Self :: ALWAYS) , "ASNEEDED" => Ok (Self :: ASNEEDED) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub struct Encryption { kms_context : Option < KmsContext > , encryption_type : Option < ServerSideEncryption > , kms_key_id : Option < SsekmsKeyId > , }

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum StorageClass { STANDARD , REDUCED_REDUNDANCY , STANDARD_IA , ONEZONE_IA , INTELLIGENT_TIERING , GLACIER , DEEP_ARCHIVE , OUTPOSTS , GLACIER_IR , } impl AsRef < str > for StorageClass { fn as_ref (& self) -> & str { match self { Self :: STANDARD => "STANDARD" , Self :: REDUCED_REDUNDANCY => "REDUCED_REDUNDANCY" , Self :: STANDARD_IA => "STANDARD_IA" , Self :: ONEZONE_IA => "ONEZONE_IA" , Self :: INTELLIGENT_TIERING => "INTELLIGENT_TIERING" , Self :: GLACIER => "GLACIER" , Self :: DEEP_ARCHIVE => "DEEP_ARCHIVE" , Self :: OUTPOSTS => "OUTPOSTS" , Self :: GLACIER_IR => "GLACIER_IR" , } } } impl TryFrom < & str > for StorageClass { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "STANDARD" => Ok (Self :: STANDARD) , "REDUCED_REDUNDANCY" => Ok (Self :: REDUCED_REDUNDANCY) , "STANDARD_IA" => Ok (Self :: STANDARD_IA) , "ONEZONE_IA" => Ok (Self :: ONEZONE_IA) , "INTELLIGENT_TIERING" => Ok (Self :: INTELLIGENT_TIERING) , "GLACIER" => Ok (Self :: GLACIER) , "DEEP_ARCHIVE" => Ok (Self :: DEEP_ARCHIVE) , "OUTPOSTS" => Ok (Self :: OUTPOSTS) , "GLACIER_IR" => Ok (Self :: GLACIER_IR) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub type MaxKeys = i32 ;

pub struct Progress { bytes_returned : Option < BytesReturned > , bytes_scanned : Option < BytesScanned > , bytes_processed : Option < BytesProcessed > , }

pub struct DeleteBucketEncryptionRequest { expected_bucket_owner : Option < AccountId > , bucket : Option < BucketName > , }

pub struct ObjectAlreadyInActiveTierError { }

pub struct DeleteBucket ; impl Op for DeleteBucket { type Input = DeleteBucketRequest ; type Output = Destination ; }

pub type FetchOwner = bool ;

pub type Code = String ;

pub type WebsiteRedirectLocation = String ;

pub struct ListBucketAnalyticsConfigurationsRequest { bucket : Option < BucketName > , continuation_token : Option < Token > , expected_bucket_owner : Option < AccountId > , }

pub struct PutObjectLockConfigurationRequest { request_payer : Option < RequestPayer > , bucket : Option < BucketName > , token : Option < ObjectLockToken > , checksum_algorithm : Option < ChecksumAlgorithm > , object_lock_configuration : Option < ObjectLockConfiguration > , content_md5 : Option < ContentMd5 > , expected_bucket_owner : Option < AccountId > , }

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum ObjectAttributes { ETAG , CHECKSUM , OBJECT_PARTS , STORAGE_CLASS , OBJECT_SIZE , } impl AsRef < str > for ObjectAttributes { fn as_ref (& self) -> & str { match self { Self :: ETAG => "ETag" , Self :: CHECKSUM => "Checksum" , Self :: OBJECT_PARTS => "ObjectParts" , Self :: STORAGE_CLASS => "StorageClass" , Self :: OBJECT_SIZE => "ObjectSize" , } } } impl TryFrom < & str > for ObjectAttributes { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "ETag" => Ok (Self :: ETAG) , "Checksum" => Ok (Self :: CHECKSUM) , "ObjectParts" => Ok (Self :: OBJECT_PARTS) , "StorageClass" => Ok (Self :: STORAGE_CLASS) , "ObjectSize" => Ok (Self :: OBJECT_SIZE) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub struct ServerSideEncryptionConfiguration { rules : Option < ServerSideEncryptionRules > , }

pub struct ListBucketInventoryConfigurations ; impl Op for ListBucketInventoryConfigurations { type Input = ListBucketInventoryConfigurationsRequest ; type Output = ListBucketInventoryConfigurationsOutput ; }

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum RequestPayer { requester , } impl AsRef < str > for RequestPayer { fn as_ref (& self) -> & str { match self { Self :: requester => "requester" , } } } impl TryFrom < & str > for RequestPayer { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "requester" => Ok (Self :: requester) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub struct AbortIncompleteMultipartUpload { days_after_initiation : Option < DaysAfterInitiation > , }

pub struct InventorySchedule { frequency : Option < InventoryFrequency > , }

pub type Comments = String ;

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum MfaDelete { Enabled , Disabled , } impl AsRef < str > for MfaDelete { fn as_ref (& self) -> & str { match self { Self :: Enabled => "Enabled" , Self :: Disabled => "Disabled" , } } } impl TryFrom < & str > for MfaDelete { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "Enabled" => Ok (Self :: Enabled) , "Disabled" => Ok (Self :: Disabled) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum ReplicationRuleStatus { Enabled , Disabled , } impl AsRef < str > for ReplicationRuleStatus { fn as_ref (& self) -> & str { match self { Self :: Enabled => "Enabled" , Self :: Disabled => "Disabled" , } } } impl TryFrom < & str > for ReplicationRuleStatus { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "Enabled" => Ok (Self :: Enabled) , "Disabled" => Ok (Self :: Disabled) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub type StreamingBlob = Vec < u8 > ;

pub struct DeleteObjectsRequest { delete : Option < Delete > , checksum_algorithm : Option < ChecksumAlgorithm > , bypass_governance_retention : Option < BypassGovernanceRetention > , bucket : Option < BucketName > , mfa : Option < Mfa > , expected_bucket_owner : Option < AccountId > , request_payer : Option < RequestPayer > , }

pub struct PutBucketReplication ; impl Op for PutBucketReplication { type Input = PutBucketReplicationRequest ; type Output = Destination ; }

pub struct GetBucketAclRequest { expected_bucket_owner : Option < AccountId > , bucket : Option < BucketName > , }

pub struct GetBucketNotificationConfiguration ; impl Op for GetBucketNotificationConfiguration { type Input = GetBucketNotificationConfigurationRequest ; type Output = NotificationConfiguration ; }

pub struct PutObjectAclRequest { checksum_algorithm : Option < ChecksumAlgorithm > , expected_bucket_owner : Option < AccountId > , content_md5 : Option < ContentMd5 > , grant_full_control : Option < GrantFullControl > , grant_write_acp : Option < GrantWriteAcp > , grant_write : Option < GrantWrite > , access_control_policy : Option < AccessControlPolicy > , request_payer : Option < RequestPayer > , grant_read_acp : Option < GrantReadAcp > , key : Option < ObjectKey > , bucket : Option < BucketName > , version_id : Option < ObjectVersionId > , acl : Option < ObjectCannedAcl > , grant_read : Option < GrantRead > , }

pub struct Tiering { days : Option < IntelligentTieringDays > , access_tier : Option < IntelligentTieringAccessTier > , }

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum BucketLogsPermission { FULL_CONTROL , READ , WRITE , } impl AsRef < str > for BucketLogsPermission { fn as_ref (& self) -> & str { match self { Self :: FULL_CONTROL => "FULL_CONTROL" , Self :: READ => "READ" , Self :: WRITE => "WRITE" , } } } impl TryFrom < & str > for BucketLogsPermission { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "FULL_CONTROL" => Ok (Self :: FULL_CONTROL) , "READ" => Ok (Self :: READ) , "WRITE" => Ok (Self :: WRITE) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub type FieldDelimiter = String ;

pub struct PutBucketAclRequest { grant_read : Option < GrantRead > , expected_bucket_owner : Option < AccountId > , checksum_algorithm : Option < ChecksumAlgorithm > , acl : Option < BucketCannedAcl > , grant_full_control : Option < GrantFullControl > , grant_write_acp : Option < GrantWriteAcp > , access_control_policy : Option < AccessControlPolicy > , bucket : Option < BucketName > , content_md5 : Option < ContentMd5 > , grant_read_acp : Option < GrantReadAcp > , grant_write : Option < GrantWrite > , }

pub struct CreateBucketConfiguration { location_constraint : Option < BucketLocationConstraint > , }

pub type RequestToken = String ;

pub struct DeleteBucketRequest { bucket : Option < BucketName > , expected_bucket_owner : Option < AccountId > , }

pub struct PutBucketAnalyticsConfiguration ; impl Op for PutBucketAnalyticsConfiguration { type Input = PutBucketAnalyticsConfigurationRequest ; type Output = Destination ; }

pub struct PutBucketRequestPayment ; impl Op for PutBucketRequestPayment { type Input = PutBucketRequestPaymentRequest ; type Output = Destination ; }

pub struct DeleteBucketAnalyticsConfigurationRequest { expected_bucket_owner : Option < AccountId > , id : Option < AnalyticsId > , bucket : Option < BucketName > , }

pub struct DeleteBucketIntelligentTieringConfiguration ; impl Op for DeleteBucketIntelligentTieringConfiguration { type Input = DeleteBucketIntelligentTieringConfigurationRequest ; type Output = Destination ; }

pub type PartNumber = i32 ;

pub struct PutBucketVersioningRequest { expected_bucket_owner : Option < AccountId > , versioning_configuration : Option < VersioningConfiguration > , content_md5 : Option < ContentMd5 > , mfa : Option < Mfa > , checksum_algorithm : Option < ChecksumAlgorithm > , bucket : Option < BucketName > , }

pub struct ParquetInput { }

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum ReplicaModificationsStatus { Enabled , Disabled , } impl AsRef < str > for ReplicaModificationsStatus { fn as_ref (& self) -> & str { match self { Self :: Enabled => "Enabled" , Self :: Disabled => "Disabled" , } } } impl TryFrom < & str > for ReplicaModificationsStatus { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "Enabled" => Ok (Self :: Enabled) , "Disabled" => Ok (Self :: Disabled) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub struct GetBucketInventoryConfigurationOutput { inventory_configuration : Option < InventoryConfiguration > , }

pub struct CorsRule { id : Option < Id > , allowed_headers : Option < AllowedHeaders > , allowed_origins : Option < AllowedOrigins > , expose_headers : Option < ExposeHeaders > , allowed_methods : Option < AllowedMethods > , max_age_seconds : Option < MaxAgeSeconds > , }

pub struct JsonOutput { record_delimiter : Option < RecordDelimiter > , }

pub struct ListParts ; impl Op for ListParts { type Input = ListPartsRequest ; type Output = ListPartsOutput ; }

pub struct CompleteMultipartUploadRequest { multipart_upload : Option < CompletedMultipartUpload > , sse_customer_algorithm : Option < SseCustomerAlgorithm > , request_payer : Option < RequestPayer > , sse_customer_key : Option < SseCustomerKey > , upload_id : Option < MultipartUploadId > , checksum_crc32 : Option < ChecksumCrc32 > , checksum_sha1 : Option < ChecksumSha1 > , checksum_sha256 : Option < ChecksumSha256 > , expected_bucket_owner : Option < AccountId > , checksum_crc32_c : Option < ChecksumCrc32c > , sse_customer_key_md5 : Option < SseCustomerKeyMd5 > , key : Option < ObjectKey > , bucket : Option < BucketName > , }

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum ObjectStorageClass { STANDARD , REDUCED_REDUNDANCY , GLACIER , STANDARD_IA , ONEZONE_IA , INTELLIGENT_TIERING , DEEP_ARCHIVE , OUTPOSTS , GLACIER_IR , } impl AsRef < str > for ObjectStorageClass { fn as_ref (& self) -> & str { match self { Self :: STANDARD => "STANDARD" , Self :: REDUCED_REDUNDANCY => "REDUCED_REDUNDANCY" , Self :: GLACIER => "GLACIER" , Self :: STANDARD_IA => "STANDARD_IA" , Self :: ONEZONE_IA => "ONEZONE_IA" , Self :: INTELLIGENT_TIERING => "INTELLIGENT_TIERING" , Self :: DEEP_ARCHIVE => "DEEP_ARCHIVE" , Self :: OUTPOSTS => "OUTPOSTS" , Self :: GLACIER_IR => "GLACIER_IR" , } } } impl TryFrom < & str > for ObjectStorageClass { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "STANDARD" => Ok (Self :: STANDARD) , "REDUCED_REDUNDANCY" => Ok (Self :: REDUCED_REDUNDANCY) , "GLACIER" => Ok (Self :: GLACIER) , "STANDARD_IA" => Ok (Self :: STANDARD_IA) , "ONEZONE_IA" => Ok (Self :: ONEZONE_IA) , "INTELLIGENT_TIERING" => Ok (Self :: INTELLIGENT_TIERING) , "DEEP_ARCHIVE" => Ok (Self :: DEEP_ARCHIVE) , "OUTPOSTS" => Ok (Self :: OUTPOSTS) , "GLACIER_IR" => Ok (Self :: GLACIER_IR) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub struct PutBucketAnalyticsConfigurationRequest { id : Option < AnalyticsId > , analytics_configuration : Option < AnalyticsConfiguration > , bucket : Option < BucketName > , expected_bucket_owner : Option < AccountId > , }

pub struct CommonPrefix { prefix : Option < Prefix > , }

pub struct RedirectAllRequestsTo { protocol : Option < Protocol > , host_name : Option < HostName > , }

pub struct CompletedPart { checksum_crc32 : Option < ChecksumCrc32 > , checksum_crc32_c : Option < ChecksumCrc32c > , checksum_sha256 : Option < ChecksumSha256 > , e_tag : Option < ETag > , part_number : Option < PartNumber > , checksum_sha1 : Option < ChecksumSha1 > , }

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum FilterRuleName { prefix , suffix , } impl AsRef < str > for FilterRuleName { fn as_ref (& self) -> & str { match self { Self :: prefix => "prefix" , Self :: suffix => "suffix" , } } } impl TryFrom < & str > for FilterRuleName { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "prefix" => Ok (Self :: prefix) , "suffix" => Ok (Self :: suffix) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum BucketLocationConstraint { af_south_1 , ap_east_1 , ap_northeast_1 , ap_northeast_2 , ap_northeast_3 , ap_south_1 , ap_southeast_1 , ap_southeast_2 , ca_central_1 , cn_north_1 , cn_northwest_1 , EU , eu_central_1 , eu_north_1 , eu_south_1 , eu_west_1 , eu_west_2 , eu_west_3 , me_south_1 , sa_east_1 , us_east_2 , us_gov_east_1 , us_gov_west_1 , us_west_1 , us_west_2 , } impl AsRef < str > for BucketLocationConstraint { fn as_ref (& self) -> & str { match self { Self :: af_south_1 => "af-south-1" , Self :: ap_east_1 => "ap-east-1" , Self :: ap_northeast_1 => "ap-northeast-1" , Self :: ap_northeast_2 => "ap-northeast-2" , Self :: ap_northeast_3 => "ap-northeast-3" , Self :: ap_south_1 => "ap-south-1" , Self :: ap_southeast_1 => "ap-southeast-1" , Self :: ap_southeast_2 => "ap-southeast-2" , Self :: ca_central_1 => "ca-central-1" , Self :: cn_north_1 => "cn-north-1" , Self :: cn_northwest_1 => "cn-northwest-1" , Self :: EU => "EU" , Self :: eu_central_1 => "eu-central-1" , Self :: eu_north_1 => "eu-north-1" , Self :: eu_south_1 => "eu-south-1" , Self :: eu_west_1 => "eu-west-1" , Self :: eu_west_2 => "eu-west-2" , Self :: eu_west_3 => "eu-west-3" , Self :: me_south_1 => "me-south-1" , Self :: sa_east_1 => "sa-east-1" , Self :: us_east_2 => "us-east-2" , Self :: us_gov_east_1 => "us-gov-east-1" , Self :: us_gov_west_1 => "us-gov-west-1" , Self :: us_west_1 => "us-west-1" , Self :: us_west_2 => "us-west-2" , } } } impl TryFrom < & str > for BucketLocationConstraint { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "af-south-1" => Ok (Self :: af_south_1) , "ap-east-1" => Ok (Self :: ap_east_1) , "ap-northeast-1" => Ok (Self :: ap_northeast_1) , "ap-northeast-2" => Ok (Self :: ap_northeast_2) , "ap-northeast-3" => Ok (Self :: ap_northeast_3) , "ap-south-1" => Ok (Self :: ap_south_1) , "ap-southeast-1" => Ok (Self :: ap_southeast_1) , "ap-southeast-2" => Ok (Self :: ap_southeast_2) , "ca-central-1" => Ok (Self :: ca_central_1) , "cn-north-1" => Ok (Self :: cn_north_1) , "cn-northwest-1" => Ok (Self :: cn_northwest_1) , "EU" => Ok (Self :: EU) , "eu-central-1" => Ok (Self :: eu_central_1) , "eu-north-1" => Ok (Self :: eu_north_1) , "eu-south-1" => Ok (Self :: eu_south_1) , "eu-west-1" => Ok (Self :: eu_west_1) , "eu-west-2" => Ok (Self :: eu_west_2) , "eu-west-3" => Ok (Self :: eu_west_3) , "me-south-1" => Ok (Self :: me_south_1) , "sa-east-1" => Ok (Self :: sa_east_1) , "us-east-2" => Ok (Self :: us_east_2) , "us-gov-east-1" => Ok (Self :: us_gov_east_1) , "us-gov-west-1" => Ok (Self :: us_gov_west_1) , "us-west-1" => Ok (Self :: us_west_1) , "us-west-2" => Ok (Self :: us_west_2) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub type LifecycleRuleFilter = Vec < String > ;

pub type StartAfter = String ;

pub type DeletedObjects = Vec < DeletedObject > ;

pub struct GetBucketTagging ; impl Op for GetBucketTagging { type Input = GetBucketTaggingRequest ; type Output = GetBucketTaggingOutput ; }

pub type MetadataValue = String ;

pub type ObjectVersionId = String ;

pub struct CreateBucketRequest { acl : Option < BucketCannedAcl > , bucket : Option < BucketName > , object_lock_enabled_for_bucket : Option < ObjectLockEnabledForBucket > , create_bucket_configuration : Option < CreateBucketConfiguration > , grant_read_acp : Option < GrantReadAcp > , object_ownership : Option < ObjectOwnership > , grant_full_control : Option < GrantFullControl > , grant_read : Option < GrantRead > , grant_write_acp : Option < GrantWriteAcp > , grant_write : Option < GrantWrite > , }

pub type IfMatch = String ;

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum ObjectCannedAcl { private , public_read , public_read_write , authenticated_read , aws_exec_read , bucket_owner_read , bucket_owner_full_control , } impl AsRef < str > for ObjectCannedAcl { fn as_ref (& self) -> & str { match self { Self :: private => "private" , Self :: public_read => "public-read" , Self :: public_read_write => "public-read-write" , Self :: authenticated_read => "authenticated-read" , Self :: aws_exec_read => "aws-exec-read" , Self :: bucket_owner_read => "bucket-owner-read" , Self :: bucket_owner_full_control => "bucket-owner-full-control" , } } } impl TryFrom < & str > for ObjectCannedAcl { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "private" => Ok (Self :: private) , "public-read" => Ok (Self :: public_read) , "public-read-write" => Ok (Self :: public_read_write) , "authenticated-read" => Ok (Self :: authenticated_read) , "aws-exec-read" => Ok (Self :: aws_exec_read) , "bucket-owner-read" => Ok (Self :: bucket_owner_read) , "bucket-owner-full-control" => Ok (Self :: bucket_owner_full_control) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub struct PutBucketMetricsConfigurationRequest { id : Option < MetricsId > , bucket : Option < BucketName > , expected_bucket_owner : Option < AccountId > , metrics_configuration : Option < MetricsConfiguration > , }

pub type CopySourceSseCustomerAlgorithm = String ;

pub struct DeletePublicAccessBlock ; impl Op for DeletePublicAccessBlock { type Input = DeletePublicAccessBlockRequest ; type Output = Destination ; }

pub struct AccessControlTranslation { owner : Option < OwnerOverride > , }

pub type ObjectLockRetainUntilDate = String ;

pub type CopySourceIfModifiedSince = String ;

pub type RestoreOutputPath = String ;

pub struct UploadPartCopyOutput { copy_part_result : Option < CopyPartResult > , sse_customer_key_md5 : Option < SseCustomerKeyMd5 > , sse_customer_algorithm : Option < SseCustomerAlgorithm > , copy_source_version_id : Option < CopySourceVersionId > , ssekms_key_id : Option < SsekmsKeyId > , request_charged : Option < RequestCharged > , server_side_encryption : Option < ServerSideEncryption > , bucket_key_enabled : Option < BucketKeyEnabled > , }

pub struct FilterRule { name : Option < FilterRuleName > , value : Option < FilterRuleValue > , }

pub struct GetBucketIntelligentTieringConfigurationOutput { intelligent_tiering_configuration : Option < IntelligentTieringConfiguration > , }

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum EncodingType { url , } impl AsRef < str > for EncodingType { fn as_ref (& self) -> & str { match self { Self :: url => "url" , } } } impl TryFrom < & str > for EncodingType { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "url" => Ok (Self :: url) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub struct GetBucketRequestPaymentOutput { payer : Option < Payer > , }

pub struct DeleteBucketCorsRequest { bucket : Option < BucketName > , expected_bucket_owner : Option < AccountId > , }

pub struct Grantee { r#type : Option < Type > , uri : Option < Uri > , display_name : Option < DisplayName > , email_address : Option < EmailAddress > , id : Option < Id > , }

pub struct IntelligentTieringAndOperator { prefix : Option < Prefix > , tags : Option < TagSet > , }

pub struct PutObjectRetentionRequest { bypass_governance_retention : Option < BypassGovernanceRetention > , checksum_algorithm : Option < ChecksumAlgorithm > , expected_bucket_owner : Option < AccountId > , version_id : Option < ObjectVersionId > , content_md5 : Option < ContentMd5 > , key : Option < ObjectKey > , bucket : Option < BucketName > , retention : Option < ObjectLockRetention > , request_payer : Option < RequestPayer > , }

pub struct SelectParameters { input_serialization : Option < InputSerialization > , expression_type : Option < ExpressionType > , output_serialization : Option < OutputSerialization > , expression : Option < Expression > , }

pub type Suffix = String ;

pub type ContentEncoding = String ;

pub struct GetBucketLoggingRequest { bucket : Option < BucketName > , expected_bucket_owner : Option < AccountId > , }

pub type SseCustomerKeyMd5 = String ;

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum DeleteMarkerReplicationStatus { Enabled , Disabled , } impl AsRef < str > for DeleteMarkerReplicationStatus { fn as_ref (& self) -> & str { match self { Self :: Enabled => "Enabled" , Self :: Disabled => "Disabled" , } } } impl TryFrom < & str > for DeleteMarkerReplicationStatus { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "Enabled" => Ok (Self :: Enabled) , "Disabled" => Ok (Self :: Disabled) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub type Expression = String ;

pub struct IntelligentTieringFilter { and : Option < IntelligentTieringAndOperator > , prefix : Option < Prefix > , tag : Option < Tag > , }

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum InventoryFrequency { Daily , Weekly , } impl AsRef < str > for InventoryFrequency { fn as_ref (& self) -> & str { match self { Self :: Daily => "Daily" , Self :: Weekly => "Weekly" , } } } impl TryFrom < & str > for InventoryFrequency { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "Daily" => Ok (Self :: Daily) , "Weekly" => Ok (Self :: Weekly) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub struct CsvInput { record_delimiter : Option < RecordDelimiter > , quote_escape_character : Option < QuoteEscapeCharacter > , allow_quoted_record_delimiter : Option < AllowQuotedRecordDelimiter > , comments : Option < Comments > , field_delimiter : Option < FieldDelimiter > , file_header_info : Option < FileHeaderInfo > , quote_character : Option < QuoteCharacter > , }

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum MfaDeleteStatus { Enabled , Disabled , } impl AsRef < str > for MfaDeleteStatus { fn as_ref (& self) -> & str { match self { Self :: Enabled => "Enabled" , Self :: Disabled => "Disabled" , } } } impl TryFrom < & str > for MfaDeleteStatus { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "Enabled" => Ok (Self :: Enabled) , "Disabled" => Ok (Self :: Disabled) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub type AllowedMethod = String ;

pub struct ListBucketMetricsConfigurations ; impl Op for ListBucketMetricsConfigurations { type Input = ListBucketMetricsConfigurationsRequest ; type Output = ListBucketMetricsConfigurationsOutput ; }

pub struct GetBucketLifecycleConfigurationOutput { rules : Option < LifecycleRules > , }

pub type Minutes = i32 ;

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum ObjectLockEnabled { Enabled , } impl AsRef < str > for ObjectLockEnabled { fn as_ref (& self) -> & str { match self { Self :: Enabled => "Enabled" , } } } impl TryFrom < & str > for ObjectLockEnabled { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "Enabled" => Ok (Self :: Enabled) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub struct PutBucketPolicyRequest { checksum_algorithm : Option < ChecksumAlgorithm > , content_md5 : Option < ContentMd5 > , expected_bucket_owner : Option < AccountId > , bucket : Option < BucketName > , confirm_remove_self_bucket_access : Option < ConfirmRemoveSelfBucketAccess > , policy : Option < Policy > , }

pub type Priority = i32 ;

pub type CorsRules = Vec < CorsRule > ;

pub struct NoSuchBucket { }

pub struct CompletedMultipartUpload { parts : Option < CompletedPartList > , }

pub struct GetBucketLocationOutput { location_constraint : Option < BucketLocationConstraint > , }

pub struct CopyPartResult { checksum_sha1 : Option < ChecksumSha1 > , last_modified : Option < LastModified > , checksum_crc32 : Option < ChecksumCrc32 > , checksum_crc32_c : Option < ChecksumCrc32c > , checksum_sha256 : Option < ChecksumSha256 > , e_tag : Option < ETag > , }

pub struct GetObjectAttributesOutput { storage_class : Option < StorageClass > , request_charged : Option < RequestCharged > , object_size : Option < ObjectSize > , object_parts : Option < GetObjectAttributesParts > , delete_marker : Option < DeleteMarker > , version_id : Option < ObjectVersionId > , e_tag : Option < ETag > , checksum : Option < Checksum > , last_modified : Option < LastModified > , }

pub struct EndEvent { }

pub type GrantRead = String ;

pub type InventoryConfigurationList = Vec < InventoryConfiguration > ;

pub type ObjectSizeGreaterThanBytes = i64 ;

pub struct PutBucketOwnershipControls ; impl Op for PutBucketOwnershipControls { type Input = PutBucketOwnershipControlsRequest ; type Output = Destination ; }

pub type TaggingHeader = String ;

pub type Buckets = Vec < Bucket > ;

pub type CopySourceIfUnmodifiedSince = String ;

pub struct DeleteBucketReplication ; impl Op for DeleteBucketReplication { type Input = DeleteBucketReplicationRequest ; type Output = Destination ; }

pub struct ListBucketIntelligentTieringConfigurationsRequest { continuation_token : Option < Token > , bucket : Option < BucketName > , }

pub struct BucketLifecycleConfiguration { rules : Option < LifecycleRules > , }

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum ObjectLockLegalHoldStatus { ON , OFF , } impl AsRef < str > for ObjectLockLegalHoldStatus { fn as_ref (& self) -> & str { match self { Self :: ON => "ON" , Self :: OFF => "OFF" , } } } impl TryFrom < & str > for ObjectLockLegalHoldStatus { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "ON" => Ok (Self :: ON) , "OFF" => Ok (Self :: OFF) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub struct GetBucketPolicy ; impl Op for GetBucketPolicy { type Input = GetBucketPolicyRequest ; type Output = GetBucketPolicyOutput ; }

pub type AllowedHeader = String ;

pub struct GetBucketLifecycleConfiguration ; impl Op for GetBucketLifecycleConfiguration { type Input = GetBucketLifecycleConfigurationRequest ; type Output = GetBucketLifecycleConfigurationOutput ; }

pub struct GetObjectLegalHoldOutput { legal_hold : Option < ObjectLockLegalHold > , }

pub type IntelligentTieringConfigurationList = Vec < IntelligentTieringConfiguration > ;

pub type AbortRuleId = String ;

pub type Days = i32 ;

pub struct PutBucketAcl ; impl Op for PutBucketAcl { type Input = PutBucketAclRequest ; type Output = Destination ; }

pub type RecordDelimiter = String ;

pub struct DeleteBucketReplicationRequest { expected_bucket_owner : Option < AccountId > , bucket : Option < BucketName > , }

# [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum InventoryFormat { CSV , ORC , Parquet , } impl AsRef < str > for InventoryFormat { fn as_ref (& self) -> & str { match self { Self :: CSV => "CSV" , Self :: ORC => "ORC" , Self :: Parquet => "Parquet" , } } } impl TryFrom < & str > for InventoryFormat { type Error = String ; fn try_from (s : & str) -> Result < Self , Self :: Error > { match s { "CSV" => Ok (Self :: CSV) , "ORC" => Ok (Self :: ORC) , "Parquet" => Ok (Self :: Parquet) , _ => Err (format ! ("unknown enum value {}" , s)) , } } }

pub struct OwnershipControlsRule { object_ownership : Option < ObjectOwnership > , }

pub type GetObjectResponseStatusCode = i32 ;

pub struct ExistingObjectReplication { status : Option < ExistingObjectReplicationStatus > , }

pub struct PutObjectLegalHold ; impl Op for PutObjectLegalHold { type Input = PutObjectLegalHoldRequest ; type Output = PutObjectLegalHoldOutput ; }

pub struct Destination { bucket : Option < BucketName > , replication_time : Option < ReplicationTime > , encryption_configuration : Option < EncryptionConfiguration > , account : Option < AccountId > , metrics : Option < Metrics > , storage_class : Option < StorageClass > , access_control_translation : Option < AccessControlTranslation > , }

pub type CopySourceSseCustomerKey = String ;

pub type IfModifiedSince = String ;

pub type Value = String ;

