// TODO generate this file automatically with https://github.com/awslabs/smithy-rs

use async_trait::async_trait;
use aws_sdk_s3::{error::*, input::*, output::*};

#[cfg_attr(rustfmt, rustfmt_skip)]
#[async_trait]
pub trait S3Api : Sync + Send {
    async fn abort_multipart_upload(&self, _input: AbortMultipartUploadInput)
        -> Result<AbortMultipartUploadOutput, AbortMultipartUploadError> {
        Err(AbortMultipartUploadError::generic(self.not_implemented()))
    }
    async fn complete_multipart_upload(&self, _input: CompleteMultipartUploadInput)
        -> Result<CompleteMultipartUploadOutput, CompleteMultipartUploadError> {
        Err(CompleteMultipartUploadError::generic(self.not_implemented()))
    }
    async fn copy_object(&self, _input: CopyObjectInput)
        -> Result<CopyObjectOutput, CopyObjectError> {
        Err(CopyObjectError::generic(self.not_implemented()))
    }
    async fn create_bucket(&self, _input: CreateBucketInput)
        -> Result<CreateBucketOutput, CreateBucketError> {
        Err(CreateBucketError::generic(self.not_implemented()))
    }
    async fn create_multipart_upload(&self, _input: CreateMultipartUploadInput)
        -> Result<CreateMultipartUploadOutput, CreateMultipartUploadError> {
        Err(CreateMultipartUploadError::generic(self.not_implemented()))
    }
    async fn delete_bucket(&self, _input: DeleteBucketInput)
        -> Result<DeleteBucketOutput, DeleteBucketError> {
        Err(DeleteBucketError::generic(self.not_implemented()))
    }
    async fn delete_bucket_analytics_configuration(&self, _input: DeleteBucketAnalyticsConfigurationInput)
        -> Result<DeleteBucketAnalyticsConfigurationOutput, DeleteBucketAnalyticsConfigurationError> {
        Err(DeleteBucketAnalyticsConfigurationError::generic(self.not_implemented()))
    }
    async fn delete_bucket_cors(&self, _input: DeleteBucketCorsInput)
        -> Result<DeleteBucketCorsOutput, DeleteBucketCorsError> {
        Err(DeleteBucketCorsError::generic(self.not_implemented()))
    }
    async fn delete_bucket_encryption(&self, _input: DeleteBucketEncryptionInput)
        -> Result<DeleteBucketEncryptionOutput, DeleteBucketEncryptionError> {
        Err(DeleteBucketEncryptionError::generic(self.not_implemented()))
    }
    async fn delete_bucket_intelligent_tiering_configuration(&self, _input: DeleteBucketIntelligentTieringConfigurationInput)
        -> Result<DeleteBucketIntelligentTieringConfigurationOutput, DeleteBucketIntelligentTieringConfigurationError> {
        Err(DeleteBucketIntelligentTieringConfigurationError::generic(self.not_implemented()))
    }
    async fn delete_bucket_inventory_configuration(&self, _input: DeleteBucketInventoryConfigurationInput)
        -> Result<DeleteBucketInventoryConfigurationOutput, DeleteBucketInventoryConfigurationError> {
        Err(DeleteBucketInventoryConfigurationError::generic(self.not_implemented()))
    }
    async fn delete_bucket_lifecycle(&self, _input: DeleteBucketLifecycleInput)
        -> Result<DeleteBucketLifecycleOutput, DeleteBucketLifecycleError> {
        Err(DeleteBucketLifecycleError::generic(self.not_implemented()))
    }
    async fn delete_bucket_metrics_configuration(&self, _input: DeleteBucketMetricsConfigurationInput)
        -> Result<DeleteBucketMetricsConfigurationOutput, DeleteBucketMetricsConfigurationError> {
        Err(DeleteBucketMetricsConfigurationError::generic(self.not_implemented()))
    }
    async fn delete_bucket_ownership_controls(&self, _input: DeleteBucketOwnershipControlsInput)
        -> Result<DeleteBucketOwnershipControlsOutput, DeleteBucketOwnershipControlsError> {
        Err(DeleteBucketOwnershipControlsError::generic(self.not_implemented()))
    }
    async fn delete_bucket_policy(&self, _input: DeleteBucketPolicyInput)
        -> Result<DeleteBucketPolicyOutput, DeleteBucketPolicyError> {
        Err(DeleteBucketPolicyError::generic(self.not_implemented()))
    }
    async fn delete_bucket_replication(&self, _input: DeleteBucketReplicationInput)
        -> Result<DeleteBucketReplicationOutput, DeleteBucketReplicationError> {
        Err(DeleteBucketReplicationError::generic(self.not_implemented()))
    }
    async fn delete_bucket_tagging(&self, _input: DeleteBucketTaggingInput)
        -> Result<DeleteBucketTaggingOutput, DeleteBucketTaggingError> {
        Err(DeleteBucketTaggingError::generic(self.not_implemented()))
    }
    async fn delete_bucket_website(&self, _input: DeleteBucketWebsiteInput)
        -> Result<DeleteBucketWebsiteOutput, DeleteBucketWebsiteError> {
        Err(DeleteBucketWebsiteError::generic(self.not_implemented()))
    }
    async fn delete_object(&self, _input: DeleteObjectInput)
        -> Result<DeleteObjectOutput, DeleteObjectError> {
        Err(DeleteObjectError::generic(self.not_implemented()))
    }
    async fn delete_objects(&self, _input: DeleteObjectsInput)
        -> Result<DeleteObjectsOutput, DeleteObjectsError> {
        Err(DeleteObjectsError::generic(self.not_implemented()))
    }
    async fn delete_object_tagging(&self, _input: DeleteObjectTaggingInput)
        -> Result<DeleteObjectTaggingOutput, DeleteObjectTaggingError> {
        Err(DeleteObjectTaggingError::generic(self.not_implemented()))
    }
    async fn delete_public_access_block(&self, _input: DeletePublicAccessBlockInput)
        -> Result<DeletePublicAccessBlockOutput, DeletePublicAccessBlockError> {
        Err(DeletePublicAccessBlockError::generic(self.not_implemented()))
    }
    async fn get_bucket_accelerate_configuration(&self, _input: GetBucketAccelerateConfigurationInput)
        -> Result<GetBucketAccelerateConfigurationOutput, GetBucketAccelerateConfigurationError> {
        Err(GetBucketAccelerateConfigurationError::generic(self.not_implemented()))
    }
    async fn get_bucket_acl(&self, _input: GetBucketAclInput)
        -> Result<GetBucketAclOutput, GetBucketAclError> {
        Err(GetBucketAclError::generic(self.not_implemented()))
    }
    async fn get_bucket_analytics_configuration(&self, _input: GetBucketAnalyticsConfigurationInput)
        -> Result<GetBucketAnalyticsConfigurationOutput, GetBucketAnalyticsConfigurationError> {
        Err(GetBucketAnalyticsConfigurationError::generic(self.not_implemented()))
    }
    async fn get_bucket_cors(&self, _input: GetBucketCorsInput)
        -> Result<GetBucketCorsOutput, GetBucketCorsError> {
        Err(GetBucketCorsError::generic(self.not_implemented()))
    }
    async fn get_bucket_encryption(&self, _input: GetBucketEncryptionInput)
        -> Result<GetBucketEncryptionOutput, GetBucketEncryptionError> {
        Err(GetBucketEncryptionError::generic(self.not_implemented()))
    }
    async fn get_bucket_intelligent_tiering_configuration(&self, _input: GetBucketIntelligentTieringConfigurationInput)
        -> Result<GetBucketIntelligentTieringConfigurationOutput, GetBucketIntelligentTieringConfigurationError> {
        Err(GetBucketIntelligentTieringConfigurationError::generic(self.not_implemented()))
    }
    async fn get_bucket_inventory_configuration(&self, _input: GetBucketInventoryConfigurationInput)
        -> Result<GetBucketInventoryConfigurationOutput, GetBucketInventoryConfigurationError> {
        Err(GetBucketInventoryConfigurationError::generic(self.not_implemented()))
    }
    async fn get_bucket_lifecycle_configuration(&self, _input: GetBucketLifecycleConfigurationInput)
        -> Result<GetBucketLifecycleConfigurationOutput, GetBucketLifecycleConfigurationError> {
        Err(GetBucketLifecycleConfigurationError::generic(self.not_implemented()))
    }
    async fn get_bucket_location(&self, _input: GetBucketLocationInput)
        -> Result<GetBucketLocationOutput, GetBucketLocationError> {
        Err(GetBucketLocationError::generic(self.not_implemented()))
    }
    async fn get_bucket_logging(&self, _input: GetBucketLoggingInput)
        -> Result<GetBucketLoggingOutput, GetBucketLoggingError> {
        Err(GetBucketLoggingError::generic(self.not_implemented()))
    }
    async fn get_bucket_metrics_configuration(&self, _input: GetBucketMetricsConfigurationInput)
        -> Result<GetBucketMetricsConfigurationOutput, GetBucketMetricsConfigurationError> {
        Err(GetBucketMetricsConfigurationError::generic(self.not_implemented()))
    }
    async fn get_bucket_notification_configuration(&self, _input: GetBucketNotificationConfigurationInput)
        -> Result<GetBucketNotificationConfigurationOutput, GetBucketNotificationConfigurationError> {
        Err(GetBucketNotificationConfigurationError::generic(self.not_implemented()))
    }
    async fn get_bucket_ownership_controls(&self, _input: GetBucketOwnershipControlsInput)
        -> Result<GetBucketOwnershipControlsOutput, GetBucketOwnershipControlsError> {
        Err(GetBucketOwnershipControlsError::generic(self.not_implemented()))
    }
    async fn get_bucket_policy(&self, _input: GetBucketPolicyInput)
        -> Result<GetBucketPolicyOutput, GetBucketPolicyError> {
        Err(GetBucketPolicyError::generic(self.not_implemented()))
    }
    async fn get_bucket_policy_status(&self, _input: GetBucketPolicyStatusInput)
        -> Result<GetBucketPolicyStatusOutput, GetBucketPolicyStatusError> {
        Err(GetBucketPolicyStatusError::generic(self.not_implemented()))
    }
    async fn get_bucket_replication(&self, _input: GetBucketReplicationInput)
        -> Result<GetBucketReplicationOutput, GetBucketReplicationError> {
        Err(GetBucketReplicationError::generic(self.not_implemented()))
    }
    async fn get_bucket_request_payment(&self, _input: GetBucketRequestPaymentInput)
        -> Result<GetBucketRequestPaymentOutput, GetBucketRequestPaymentError> {
        Err(GetBucketRequestPaymentError::generic(self.not_implemented()))
    }
    async fn get_bucket_tagging(&self, _input: GetBucketTaggingInput)
        -> Result<GetBucketTaggingOutput, GetBucketTaggingError> {
        Err(GetBucketTaggingError::generic(self.not_implemented()))
    }
    async fn get_bucket_versioning(&self, _input: GetBucketVersioningInput)
        -> Result<GetBucketVersioningOutput, GetBucketVersioningError> {
        Err(GetBucketVersioningError::generic(self.not_implemented()))
    }
    async fn get_bucket_website(&self, _input: GetBucketWebsiteInput)
        -> Result<GetBucketWebsiteOutput, GetBucketWebsiteError> {
        Err(GetBucketWebsiteError::generic(self.not_implemented()))
    }
    async fn get_object(&self, _input: GetObjectInput)
        -> Result<GetObjectOutput, GetObjectError> {
        Err(GetObjectError::generic(self.not_implemented()))
    }
    async fn get_object_acl(&self, _input: GetObjectAclInput)
        -> Result<GetObjectAclOutput, GetObjectAclError> {
        Err(GetObjectAclError::generic(self.not_implemented()))
    }
    async fn get_object_legal_hold(&self, _input: GetObjectLegalHoldInput)
        -> Result<GetObjectLegalHoldOutput, GetObjectLegalHoldError> {
        Err(GetObjectLegalHoldError::generic(self.not_implemented()))
    }
    async fn get_object_lock_configuration(&self, _input: GetObjectLockConfigurationInput)
        -> Result<GetObjectLockConfigurationOutput, GetObjectLockConfigurationError> {
        Err(GetObjectLockConfigurationError::generic(self.not_implemented()))
    }
    async fn get_object_retention(&self, _input: GetObjectRetentionInput)
        -> Result<GetObjectRetentionOutput, GetObjectRetentionError> {
        Err(GetObjectRetentionError::generic(self.not_implemented()))
    }
    async fn get_object_tagging(&self, _input: GetObjectTaggingInput)
        -> Result<GetObjectTaggingOutput, GetObjectTaggingError> {
        Err(GetObjectTaggingError::generic(self.not_implemented()))
    }
    async fn get_object_torrent(&self, _input: GetObjectTorrentInput)
        -> Result<GetObjectTorrentOutput, GetObjectTorrentError> {
        Err(GetObjectTorrentError::generic(self.not_implemented()))
    }
    async fn get_public_access_block(&self, _input: GetPublicAccessBlockInput)
        -> Result<GetPublicAccessBlockOutput, GetPublicAccessBlockError> {
        Err(GetPublicAccessBlockError::generic(self.not_implemented()))
    }
    async fn head_bucket(&self, _input: HeadBucketInput)
        -> Result<HeadBucketOutput, HeadBucketError> {
        Err(HeadBucketError::generic(self.not_implemented()))
    }
    async fn head_object(&self, _input: HeadObjectInput)
        -> Result<HeadObjectOutput, HeadObjectError> {
        Err(HeadObjectError::generic(self.not_implemented()))
    }
    async fn list_bucket_analytics_configurations(&self, _input: ListBucketAnalyticsConfigurationsInput)
        -> Result<ListBucketAnalyticsConfigurationsOutput, ListBucketAnalyticsConfigurationsError> {
        Err(ListBucketAnalyticsConfigurationsError::generic(self.not_implemented()))
    }
    async fn list_bucket_intelligent_tiering_configurations(&self, _input: ListBucketIntelligentTieringConfigurationsInput)
        -> Result<ListBucketIntelligentTieringConfigurationsOutput, ListBucketIntelligentTieringConfigurationsError> {
        Err(ListBucketIntelligentTieringConfigurationsError::generic(self.not_implemented()))
    }
    async fn list_bucket_inventory_configurations(&self, _input: ListBucketInventoryConfigurationsInput)
        -> Result<ListBucketInventoryConfigurationsOutput, ListBucketInventoryConfigurationsError> {
        Err(ListBucketInventoryConfigurationsError::generic(self.not_implemented()))
    }
    async fn list_bucket_metrics_configurations(&self, _input: ListBucketMetricsConfigurationsInput)
        -> Result<ListBucketMetricsConfigurationsOutput, ListBucketMetricsConfigurationsError> {
        Err(ListBucketMetricsConfigurationsError::generic(self.not_implemented()))
    }
    async fn list_buckets(&self, _input: ListBucketsInput)
        -> Result<ListBucketsOutput, ListBucketsError> {
        Err(ListBucketsError::generic(self.not_implemented()))
    }
    async fn list_multipart_uploads(&self, _input: ListMultipartUploadsInput)
        -> Result<ListMultipartUploadsOutput, ListMultipartUploadsError> {
        Err(ListMultipartUploadsError::generic(self.not_implemented()))
    }
    async fn list_objects(&self, _input: ListObjectsInput)
        -> Result<ListObjectsOutput, ListObjectsError> {
        Err(ListObjectsError::generic(self.not_implemented()))
    }
    async fn list_objects_v2(&self, _input: ListObjectsV2Input)
        -> Result<ListObjectsV2Output, ListObjectsV2Error> {
        Err(ListObjectsV2Error::generic(self.not_implemented()))
    }
    async fn list_object_versions(&self, _input: ListObjectVersionsInput)
        -> Result<ListObjectVersionsOutput, ListObjectVersionsError> {
        Err(ListObjectVersionsError::generic(self.not_implemented()))
    }
    async fn list_parts(&self, _input: ListPartsInput)
        -> Result<ListPartsOutput, ListPartsError> {
        Err(ListPartsError::generic(self.not_implemented()))
    }
    async fn put_bucket_accelerate_configuration(&self, _input: PutBucketAccelerateConfigurationInput)
        -> Result<PutBucketAccelerateConfigurationOutput, PutBucketAccelerateConfigurationError> {
        Err(PutBucketAccelerateConfigurationError::generic(self.not_implemented()))
    }
    async fn put_bucket_acl(&self, _input: PutBucketAclInput)
        -> Result<PutBucketAclOutput, PutBucketAclError> {
        Err(PutBucketAclError::generic(self.not_implemented()))
    }
    async fn put_bucket_analytics_configuration(&self, _input: PutBucketAnalyticsConfigurationInput)
        -> Result<PutBucketAnalyticsConfigurationOutput, PutBucketAnalyticsConfigurationError> {
        Err(PutBucketAnalyticsConfigurationError::generic(self.not_implemented()))
    }
    async fn put_bucket_cors(&self, _input: PutBucketCorsInput)
        -> Result<PutBucketCorsOutput, PutBucketCorsError> {
        Err(PutBucketCorsError::generic(self.not_implemented()))
    }
    async fn put_bucket_encryption(&self, _input: PutBucketEncryptionInput)
        -> Result<PutBucketEncryptionOutput, PutBucketEncryptionError> {
        Err(PutBucketEncryptionError::generic(self.not_implemented()))
    }
    async fn put_bucket_intelligent_tiering_configuration(&self, _input: PutBucketIntelligentTieringConfigurationInput)
        -> Result<PutBucketIntelligentTieringConfigurationOutput, PutBucketIntelligentTieringConfigurationError> {
        Err(PutBucketIntelligentTieringConfigurationError::generic(self.not_implemented()))
    }
    async fn put_bucket_inventory_configuration(&self, _input: PutBucketInventoryConfigurationInput)
        -> Result<PutBucketInventoryConfigurationOutput, PutBucketInventoryConfigurationError> {
        Err(PutBucketInventoryConfigurationError::generic(self.not_implemented()))
    }
    async fn put_bucket_lifecycle_configuration(&self, _input: PutBucketLifecycleConfigurationInput)
        -> Result<PutBucketLifecycleConfigurationOutput, PutBucketLifecycleConfigurationError> {
        Err(PutBucketLifecycleConfigurationError::generic(self.not_implemented()))
    }
    async fn put_bucket_logging(&self, _input: PutBucketLoggingInput)
        -> Result<PutBucketLoggingOutput, PutBucketLoggingError> {
        Err(PutBucketLoggingError::generic(self.not_implemented()))
    }
    async fn put_bucket_metrics_configuration(&self, _input: PutBucketMetricsConfigurationInput)
        -> Result<PutBucketMetricsConfigurationOutput, PutBucketMetricsConfigurationError> {
        Err(PutBucketMetricsConfigurationError::generic(self.not_implemented()))
    }
    async fn put_bucket_notification_configuration(&self, _input: PutBucketNotificationConfigurationInput)
        -> Result<PutBucketNotificationConfigurationOutput, PutBucketNotificationConfigurationError> {
        Err(PutBucketNotificationConfigurationError::generic(self.not_implemented()))
    }
    async fn put_bucket_ownership_controls(&self, _input: PutBucketOwnershipControlsInput)
        -> Result<PutBucketOwnershipControlsOutput, PutBucketOwnershipControlsError> {
        Err(PutBucketOwnershipControlsError::generic(self.not_implemented()))
    }
    async fn put_bucket_policy(&self, _input: PutBucketPolicyInput)
        -> Result<PutBucketPolicyOutput, PutBucketPolicyError> {
        Err(PutBucketPolicyError::generic(self.not_implemented()))
    }
    async fn put_bucket_replication(&self, _input: PutBucketReplicationInput)
        -> Result<PutBucketReplicationOutput, PutBucketReplicationError> {
        Err(PutBucketReplicationError::generic(self.not_implemented()))
    }
    async fn put_bucket_request_payment(&self, _input: PutBucketRequestPaymentInput)
        -> Result<PutBucketRequestPaymentOutput, PutBucketRequestPaymentError> {
        Err(PutBucketRequestPaymentError::generic(self.not_implemented()))
    }
    async fn put_bucket_tagging(&self, _input: PutBucketTaggingInput)
        -> Result<PutBucketTaggingOutput, PutBucketTaggingError> {
        Err(PutBucketTaggingError::generic(self.not_implemented()))
    }
    async fn put_bucket_versioning(&self, _input: PutBucketVersioningInput)
        -> Result<PutBucketVersioningOutput, PutBucketVersioningError> {
        Err(PutBucketVersioningError::generic(self.not_implemented()))
    }
    async fn put_bucket_website(&self, _input: PutBucketWebsiteInput)
        -> Result<PutBucketWebsiteOutput, PutBucketWebsiteError> {
        Err(PutBucketWebsiteError::generic(self.not_implemented()))
    }
    async fn put_object(&self, _input: PutObjectInput)
        -> Result<PutObjectOutput, PutObjectError> {
        Err(PutObjectError::generic(self.not_implemented()))
    }
    async fn put_object_acl(&self, _input: PutObjectAclInput)
        -> Result<PutObjectAclOutput, PutObjectAclError> {
        Err(PutObjectAclError::generic(self.not_implemented()))
    }
    async fn put_object_legal_hold(&self, _input: PutObjectLegalHoldInput)
        -> Result<PutObjectLegalHoldOutput, PutObjectLegalHoldError> {
        Err(PutObjectLegalHoldError::generic(self.not_implemented()))
    }
    async fn put_object_lock_configuration(&self, _input: PutObjectLockConfigurationInput)
        -> Result<PutObjectLockConfigurationOutput, PutObjectLockConfigurationError> {
        Err(PutObjectLockConfigurationError::generic(self.not_implemented()))
    }
    async fn put_object_retention(&self, _input: PutObjectRetentionInput)
        -> Result<PutObjectRetentionOutput, PutObjectRetentionError> {
        Err(PutObjectRetentionError::generic(self.not_implemented()))
    }
    async fn put_object_tagging(&self, _input: PutObjectTaggingInput)
        -> Result<PutObjectTaggingOutput, PutObjectTaggingError> {
        Err(PutObjectTaggingError::generic(self.not_implemented()))
    }
    async fn put_public_access_block(&self, _input: PutPublicAccessBlockInput)
        -> Result<PutPublicAccessBlockOutput, PutPublicAccessBlockError> {
        Err(PutPublicAccessBlockError::generic(self.not_implemented()))
    }
    async fn restore_object(&self, _input: RestoreObjectInput)
        -> Result<RestoreObjectOutput, RestoreObjectError> {
        Err(RestoreObjectError::generic(self.not_implemented()))
    }
    async fn select_object_content(&self, _input: SelectObjectContentInput)
        -> Result<SelectObjectContentOutput, SelectObjectContentError> {
        Err(SelectObjectContentError::generic(self.not_implemented()))
    }
    async fn upload_part(&self, _input: UploadPartInput)
        -> Result<UploadPartOutput, UploadPartError> {
        Err(UploadPartError::generic(self.not_implemented()))
    }
    async fn upload_part_copy(&self, _input: UploadPartCopyInput)
        -> Result<UploadPartCopyOutput, UploadPartCopyError> {
        Err(UploadPartCopyError::generic(self.not_implemented()))
    }
    async fn write_get_object_response(&self, _input: WriteGetObjectResponseInput)
        -> Result<WriteGetObjectResponseOutput, WriteGetObjectResponseError> {
        Err(WriteGetObjectResponseError::generic(self.not_implemented()))
    }

    fn not_implemented(&self) -> aws_smithy_types::Error {
        aws_smithy_types::Error::builder()
            .code("NotImplemented")
            .message("The requested action is not implemented.")
            .build()
    }
}
