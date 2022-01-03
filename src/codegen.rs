use crate::store;
use aws_sdk_s3::error::PutObjectError;
use s3d_codegen::{input::*, operation_registry::*, output::*};

pub async fn serve() -> anyhow::Result<()> {
    let ops = OperationRegistryBuilder::default()
        .abort_multipart_upload(|i: AbortMultipartUploadInput| async move {
            info!("abort_multipart_upload: {:?}", i);
            Ok(AbortMultipartUploadOutput::builder().build())
        })
        .complete_multipart_upload(|i: CompleteMultipartUploadInput| async move {
            info!("complete_multipart_upload: {:?}", i);
            CompleteMultipartUploadOutput::builder().build()
        })
        .copy_object(|i: CopyObjectInput| async move {
            info!("copy_object: {:?}", i);
            Ok(CopyObjectOutput::builder().build())
        })
        .create_bucket(|i: CreateBucketInput| async move {
            info!("create_bucket: {:?}", i);
            Ok(CreateBucketOutput::builder().build())
        })
        .create_multipart_upload(|i: CreateMultipartUploadInput| async move {
            info!("create_multipart_upload: {:?}", i);
            CreateMultipartUploadOutput::builder().build()
        })
        .delete_bucket(|i: DeleteBucketInput| async move {
            info!("delete_bucket: {:?}", i);
            delete_bucket_output::Builder::default().build()
        })
        .delete_bucket_analytics_configuration(
            |i: DeleteBucketAnalyticsConfigurationInput| async move {
                info!("delete_bucket_analytics_configuration: {:?}", i);
                delete_bucket_analytics_configuration_output::Builder::default().build()
            },
        )
        .delete_bucket_cors(|i: DeleteBucketCorsInput| async move {
            info!("delete_bucket_cors: {:?}", i);
            delete_bucket_cors_output::Builder::default().build()
        })
        .delete_bucket_encryption(|i: DeleteBucketEncryptionInput| async move {
            info!("delete_bucket_encryption: {:?}", i);
            delete_bucket_encryption_output::Builder::default().build()
        })
        .delete_bucket_intelligent_tiering_configuration(
            |i: DeleteBucketIntelligentTieringConfigurationInput| async move {
                info!("delete_bucket_intelligent_tiering_configuration: {:?}", i);
                delete_bucket_intelligent_tiering_configuration_output::Builder::default().build()
            },
        )
        .delete_bucket_inventory_configuration(
            |i: DeleteBucketInventoryConfigurationInput| async move {
                info!("delete_bucket_inventory_configuration: {:?}", i);
                delete_bucket_inventory_configuration_output::Builder::default().build()
            },
        )
        .delete_bucket_lifecycle(|i: DeleteBucketLifecycleInput| async move {
            info!("delete_bucket_lifecycle: {:?}", i);
            delete_bucket_lifecycle_output::Builder::default().build()
        })
        .delete_bucket_metrics_configuration(
            |i: DeleteBucketMetricsConfigurationInput| async move {
                info!("delete_bucket_metrics_configuration: {:?}", i);
                delete_bucket_metrics_configuration_output::Builder::default().build()
            },
        )
        .delete_bucket_ownership_controls(|i: DeleteBucketOwnershipControlsInput| async move {
            info!("delete_bucket_ownership_controls: {:?}", i);
            delete_bucket_ownership_controls_output::Builder::default().build()
        })
        .delete_bucket_policy(|i: DeleteBucketPolicyInput| async move {
            info!("delete_bucket_policy: {:?}", i);
            delete_bucket_policy_output::Builder::default().build()
        })
        .delete_bucket_replication(|i: DeleteBucketReplicationInput| async move {
            info!("delete_bucket_replication: {:?}", i);
            delete_bucket_replication_output::Builder::default().build()
        })
        .delete_bucket_tagging(|i: DeleteBucketTaggingInput| async move {
            info!("delete_bucket_tagging: {:?}", i);
            delete_bucket_tagging_output::Builder::default().build()
        })
        .delete_bucket_website(|i: DeleteBucketWebsiteInput| async move {
            info!("delete_bucket_website: {:?}", i);
            delete_bucket_website_output::Builder::default().build()
        })
        .delete_object(|i: DeleteObjectInput| async move {
            info!("delete_object: {:?}", i);
            delete_object_output::Builder::default().build()
        })
        .delete_objects(|i: DeleteObjectsInput| async move {
            info!("delete_objects: {:?}", i);
            delete_objects_output::Builder::default().build()
        })
        .delete_object_tagging(|i: DeleteObjectTaggingInput| async move {
            info!("delete_object_tagging: {:?}", i);
            delete_object_tagging_output::Builder::default().build()
        })
        .delete_public_access_block(|i: DeletePublicAccessBlockInput| async move {
            info!("delete_public_access_block: {:?}", i);
            delete_public_access_block_output::Builder::default().build()
        })
        .get_bucket_accelerate_configuration(
            |i: GetBucketAccelerateConfigurationInput| async move {
                info!("get_bucket_accelerate_configuration: {:?}", i);
                get_bucket_accelerate_configuration_output::Builder::default().build()
            },
        )
        .get_bucket_acl(|i: GetBucketAclInput| async move {
            info!("get_bucket_acl: {:?}", i);
            get_bucket_acl_output::Builder::default().build()
        })
        .get_bucket_analytics_configuration(|i: GetBucketAnalyticsConfigurationInput| async move {
            info!("get_bucket_analytics_configuration: {:?}", i);
            get_bucket_analytics_configuration_output::Builder::default().build()
        })
        .get_bucket_cors(|i: GetBucketCorsInput| async move {
            info!("get_bucket_cors: {:?}", i);
            get_bucket_cors_output::Builder::default().build()
        })
        .get_bucket_encryption(|i: GetBucketEncryptionInput| async move {
            info!("get_bucket_encryption: {:?}", i);
            get_bucket_encryption_output::Builder::default().build()
        })
        .get_bucket_intelligent_tiering_configuration(
            |i: GetBucketIntelligentTieringConfigurationInput| async move {
                info!("get_bucket_intelligent_tiering_configuration: {:?}", i);
                get_bucket_intelligent_tiering_configuration_output::Builder::default().build()
            },
        )
        .get_bucket_inventory_configuration(|i: GetBucketInventoryConfigurationInput| async move {
            info!("get_bucket_inventory_configuration: {:?}", i);
            get_bucket_inventory_configuration_output::Builder::default().build()
        })
        .get_bucket_lifecycle_configuration(|i: GetBucketLifecycleConfigurationInput| async move {
            info!("get_bucket_lifecycle_configuration: {:?}", i);
            get_bucket_lifecycle_configuration_output::Builder::default().build()
        })
        .get_bucket_location(|i: GetBucketLocationInput| async move {
            info!("get_bucket_location: {:?}", i);
            get_bucket_location_output::Builder::default().build()
        })
        .get_bucket_logging(|i: GetBucketLoggingInput| async move {
            info!("get_bucket_logging: {:?}", i);
            get_bucket_logging_output::Builder::default().build()
        })
        .get_bucket_metrics_configuration(|i: GetBucketMetricsConfigurationInput| async move {
            info!("get_bucket_metrics_configuration: {:?}", i);
            get_bucket_metrics_configuration_output::Builder::default().build()
        })
        .get_bucket_notification_configuration(
            |i: GetBucketNotificationConfigurationInput| async move {
                info!("get_bucket_notification_configuration: {:?}", i);
                get_bucket_notification_configuration_output::Builder::default().build()
            },
        )
        .get_bucket_ownership_controls(|i: GetBucketOwnershipControlsInput| async move {
            info!("get_bucket_ownership_controls: {:?}", i);
            get_bucket_ownership_controls_output::Builder::default().build()
        })
        .get_bucket_policy(|i: GetBucketPolicyInput| async move {
            info!("get_bucket_policy: {:?}", i);
            get_bucket_policy_output::Builder::default().build()
        })
        .get_bucket_policy_status(|i: GetBucketPolicyStatusInput| async move {
            info!("get_bucket_policy_status: {:?}", i);
            get_bucket_policy_status_output::Builder::default().build()
        })
        .get_bucket_replication(|i: GetBucketReplicationInput| async move {
            info!("get_bucket_replication: {:?}", i);
            get_bucket_replication_output::Builder::default().build()
        })
        .get_bucket_request_payment(|i: GetBucketRequestPaymentInput| async move {
            info!("get_bucket_request_payment: {:?}", i);
            get_bucket_request_payment_output::Builder::default().build()
        })
        .get_bucket_tagging(|i: GetBucketTaggingInput| async move {
            info!("get_bucket_tagging: {:?}", i);
            get_bucket_tagging_output::Builder::default().build()
        })
        .get_bucket_versioning(|i: GetBucketVersioningInput| async move {
            info!("get_bucket_versioning: {:?}", i);
            get_bucket_versioning_output::Builder::default().build()
        })
        .get_bucket_website(|i: GetBucketWebsiteInput| async move {
            info!("get_bucket_website: {:?}", i);
            get_bucket_website_output::Builder::default().build()
        })
        .get_object(|i: GetObjectInput| async move {
            info!("get_object: {:?}", i);
            store::get_object(i).await
        })
        .get_object_acl(|i: GetObjectAclInput| async move {
            info!("get_object_acl: {:?}", i);
            Ok(get_object_acl_output::Builder::default().build())
        })
        .get_object_legal_hold(|i: GetObjectLegalHoldInput| async move {
            info!("get_object_legal_hold: {:?}", i);
            get_object_legal_hold_output::Builder::default().build()
        })
        .get_object_lock_configuration(|i: GetObjectLockConfigurationInput| async move {
            info!("get_object_lock_configuration: {:?}", i);
            get_object_lock_configuration_output::Builder::default().build()
        })
        .get_object_retention(|i: GetObjectRetentionInput| async move {
            info!("get_object_retention: {:?}", i);
            get_object_retention_output::Builder::default().build()
        })
        .get_object_tagging(|i: GetObjectTaggingInput| async move {
            info!("get_object_tagging: {:?}", i);
            get_object_tagging_output::Builder::default().build()
        })
        .get_object_torrent(|i: GetObjectTorrentInput| async move {
            info!("get_object_torrent: {:?}", i);
            get_object_torrent_output::Builder::default().build()
        })
        .get_public_access_block(|i: GetPublicAccessBlockInput| async move {
            info!("get_public_access_block: {:?}", i);
            get_public_access_block_output::Builder::default().build()
        })
        .head_bucket(|i: HeadBucketInput| async move {
            info!("head_bucket: {:?}", i);
            Ok(head_bucket_output::Builder::default().build())
        })
        .head_object(|i: HeadObjectInput| async move {
            info!("head_object: {:?}", i);
            Ok(head_object_output::Builder::default().build())
        })
        .list_bucket_analytics_configurations(
            |i: ListBucketAnalyticsConfigurationsInput| async move {
                info!("list_bucket_analytics_configurations: {:?}", i);
                list_bucket_analytics_configurations_output::Builder::default().build()
            },
        )
        .list_bucket_intelligent_tiering_configurations(
            |i: ListBucketIntelligentTieringConfigurationsInput| async move {
                info!("list_bucket_intelligent_tiering_configurations: {:?}", i);
                list_bucket_intelligent_tiering_configurations_output::Builder::default().build()
            },
        )
        .list_bucket_inventory_configurations(
            |i: ListBucketInventoryConfigurationsInput| async move {
                info!("list_bucket_inventory_configurations: {:?}", i);
                list_bucket_inventory_configurations_output::Builder::default().build()
            },
        )
        .list_bucket_metrics_configurations(|i: ListBucketMetricsConfigurationsInput| async move {
            info!("list_bucket_metrics_configurations: {:?}", i);
            list_bucket_metrics_configurations_output::Builder::default().build()
        })
        .list_buckets(|i: ListBucketsInput| async move {
            info!("list_buckets: {:?}", i);
            list_buckets_output::Builder::default().build()
        })
        .list_multipart_uploads(|i: ListMultipartUploadsInput| async move {
            info!("list_multipart_uploads: {:?}", i);
            list_multipart_uploads_output::Builder::default().build()
        })
        .list_objects(|i: ListObjectsInput| async move {
            info!("list_objects: {:?}", i);
            Ok(list_objects_output::Builder::default().build())
        })
        .list_objects_v2(|i: ListObjectsV2Input| async move {
            info!("list_objects_v2: {:?}", i);
            Ok(list_objects_v2_output::Builder::default().build())
        })
        .list_object_versions(|i: ListObjectVersionsInput| async move {
            info!("list_object_versions: {:?}", i);
            list_object_versions_output::Builder::default().build()
        })
        .list_parts(|i: ListPartsInput| async move {
            info!("list_parts: {:?}", i);
            list_parts_output::Builder::default().build()
        })
        .put_bucket_accelerate_configuration(
            |i: PutBucketAccelerateConfigurationInput| async move {
                info!("put_bucket_accelerate_configuration: {:?}", i);
                put_bucket_accelerate_configuration_output::Builder::default().build()
            },
        )
        .put_bucket_acl(|i: PutBucketAclInput| async move {
            info!("put_bucket_acl: {:?}", i);
            put_bucket_acl_output::Builder::default().build()
        })
        .put_bucket_analytics_configuration(|i: PutBucketAnalyticsConfigurationInput| async move {
            info!("put_bucket_analytics_configuration: {:?}", i);
            put_bucket_analytics_configuration_output::Builder::default().build()
        })
        .put_bucket_cors(|i: PutBucketCorsInput| async move {
            info!("put_bucket_cors: {:?}", i);
            put_bucket_cors_output::Builder::default().build()
        })
        .put_bucket_encryption(|i: PutBucketEncryptionInput| async move {
            info!("put_bucket_encryption: {:?}", i);
            put_bucket_encryption_output::Builder::default().build()
        })
        .put_bucket_intelligent_tiering_configuration(
            |i: PutBucketIntelligentTieringConfigurationInput| async move {
                info!("put_bucket_intelligent_tiering_configuration: {:?}", i);
                put_bucket_intelligent_tiering_configuration_output::Builder::default().build()
            },
        )
        .put_bucket_inventory_configuration(|i: PutBucketInventoryConfigurationInput| async move {
            info!("put_bucket_inventory_configuration: {:?}", i);
            put_bucket_inventory_configuration_output::Builder::default().build()
        })
        .put_bucket_lifecycle_configuration(|i: PutBucketLifecycleConfigurationInput| async move {
            info!("put_bucket_lifecycle_configuration: {:?}", i);
            put_bucket_lifecycle_configuration_output::Builder::default().build()
        })
        .put_bucket_logging(|i: PutBucketLoggingInput| async move {
            info!("put_bucket_logging: {:?}", i);
            put_bucket_logging_output::Builder::default().build()
        })
        .put_bucket_metrics_configuration(|i: PutBucketMetricsConfigurationInput| async move {
            info!("put_bucket_metrics_configuration: {:?}", i);
            put_bucket_metrics_configuration_output::Builder::default().build()
        })
        .put_bucket_notification_configuration(
            |i: PutBucketNotificationConfigurationInput| async move {
                info!("put_bucket_notification_configuration: {:?}", i);
                put_bucket_notification_configuration_output::Builder::default().build()
            },
        )
        .put_bucket_ownership_controls(|i: PutBucketOwnershipControlsInput| async move {
            info!("put_bucket_ownership_controls: {:?}", i);
            put_bucket_ownership_controls_output::Builder::default().build()
        })
        .put_bucket_policy(|i: PutBucketPolicyInput| async move {
            info!("put_bucket_policy: {:?}", i);
            put_bucket_policy_output::Builder::default().build()
        })
        .put_bucket_replication(|i: PutBucketReplicationInput| async move {
            info!("put_bucket_replication: {:?}", i);
            put_bucket_replication_output::Builder::default().build()
        })
        .put_bucket_request_payment(|i: PutBucketRequestPaymentInput| async move {
            info!("put_bucket_request_payment: {:?}", i);
            put_bucket_request_payment_output::Builder::default().build()
        })
        .put_bucket_tagging(|i: PutBucketTaggingInput| async move {
            info!("put_bucket_tagging: {:?}", i);
            put_bucket_tagging_output::Builder::default().build()
        })
        .put_bucket_versioning(|i: PutBucketVersioningInput| async move {
            info!("put_bucket_versioning: {:?}", i);
            put_bucket_versioning_output::Builder::default().build()
        })
        .put_bucket_website(|i: PutBucketWebsiteInput| async move {
            info!("put_bucket_website: {:?}", i);
            put_bucket_website_output::Builder::default().build()
        })
        .put_object(|i: PutObjectInput| async move {
            info!("put_object: {:?}", i);
            store::put_object(i).await.unwrap()
        })
        .put_object_acl(|i: PutObjectAclInput| async move {
            info!("put_object_acl: {:?}", i);
            Ok(put_object_acl_output::Builder::default().build())
        })
        .put_object_legal_hold(|i: PutObjectLegalHoldInput| async move {
            info!("put_object_legal_hold: {:?}", i);
            put_object_legal_hold_output::Builder::default().build()
        })
        .put_object_lock_configuration(|i: PutObjectLockConfigurationInput| async move {
            info!("put_object_lock_configuration: {:?}", i);
            put_object_lock_configuration_output::Builder::default().build()
        })
        .put_object_retention(|i: PutObjectRetentionInput| async move {
            info!("put_object_retention: {:?}", i);
            put_object_retention_output::Builder::default().build()
        })
        .put_object_tagging(|i: PutObjectTaggingInput| async move {
            info!("put_object_tagging: {:?}", i);
            put_object_tagging_output::Builder::default().build()
        })
        .put_public_access_block(|i: PutPublicAccessBlockInput| async move {
            info!("put_public_access_block: {:?}", i);
            put_public_access_block_output::Builder::default().build()
        })
        .restore_object(|i: RestoreObjectInput| async move {
            info!("restore_object: {:?}", i);
            Ok(restore_object_output::Builder::default().build())
        })
        .upload_part(|i: UploadPartInput| async move {
            info!("upload_part: {:?}", i);
            upload_part_output::Builder::default().build()
        })
        .upload_part_copy(|i: UploadPartCopyInput| async move {
            info!("upload_part_copy: {:?}", i);
            upload_part_copy_output::Builder::default().build()
        })
        .write_get_object_response(|i: WriteGetObjectResponseInput| async move {
            info!("write_get_object_response: {:?}", i);
            write_get_object_response_output::Builder::default().build()
        })
        .build()
        .unwrap();

    {
        #[rustfmt::skip]
        let _: &OperationRegistry<hyper::Body,
            _, (), _, (), _, (), _, (), _, (), _, (), _, (), _, (), _, (), _, (),
            _, (), _, (), _, (), _, (), _, (), _, (), _, (), _, (), _, (), _, (),
            _, (), _, (), _, (), _, (), _, (), _, (), _, (), _, (), _, (), _, (),
            _, (), _, (), _, (), _, (), _, (), _, (), _, (), _, (), _, (), _, (),
            _, (), _, (), _, (), _, (), _, (), _, (), _, (), _, (), _, (), _, (),
            _, (), _, (), _, (), _, (), _, (), _, (), _, (), _, (), _, (), _, (),
            _, (), _, (), _, (), _, (), _, (), _, (), _, (), _, (), _, (), _, (),
            _, (), _, (), _, (), _, (), _, (), _, (), _, (), _, (), _, (), _, (),
            _, (), _, (), _, (), _, (), _, (), _, (), _, (), _, (), _, (), _, (),
            _, ()> = &ops;
    }

    let router = aws_smithy_http_server::Router::from(ops);
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3333));
    let server = hyper::Server::bind(&addr).serve(router.into_make_service());
    info!("Listening on http://{}", addr);
    server.await?;
    Ok(())
}
