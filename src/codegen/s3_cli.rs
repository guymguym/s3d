#[derive(clap :: Subcommand, Debug, Clone)]
pub enum S3OpsCommands {
    # [clap (about = "This action removes the website configuration for a bucket. Amazon S3 returns a <code>200 OK</code> response upon successfully deleting a website configuration on the specified bucket. You will get a <code>200 OK</code> response if the website configuration you are trying to delete does not exist on the bucket. Amazon S3 returns a <code>404</code> response if the bucket specified in the request does not exist." , long_about = None)]
    DeleteBucketWebsite(DeleteBucketWebsiteCommand),
    # [clap (about = "Gets a metrics configuration (specified by the metrics configuration ID) from the bucket. Note that this doesn't include the daily storage metrics." , long_about = None)]
    GetBucketMetricsConfiguration(GetBucketMetricsConfigurationCommand),
    # [clap (about = "This implementation of the <code>GET</code> action uses the <code>acl</code> subresource to return the access control list (ACL) of a bucket. To use <code>GET</code> to return the ACL of the bucket, you must have <code>READ_ACP</code> access to the bucket. If <code>READ_ACP</code> permission is granted to the anonymous user, you can return the ACL of the bucket without using an authorization header." , long_about = None)]
    GetBucketAcl(GetBucketAclCommand),
    # [clap (about = "Gets the Object Lock configuration for a bucket. The rule specified in the Object Lock configuration will be applied by default to every new object placed in the specified bucket. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lock.html\">Locking Objects</a>." , long_about = None)]
    GetObjectLockConfiguration(GetObjectLockConfigurationCommand),
    # [clap (about = "Returns the logging status of a bucket and the permissions users have to view and modify that status. To use GET, you must be the bucket owner." , long_about = None)]
    GetBucketLogging(GetBucketLoggingCommand),
    # [clap (about = "Creates or modifies <code>OwnershipControls</code> for an Amazon S3 bucket. To use this operation, you must have the <code>s3:PutBucketOwnershipControls</code> permission. For more information about Amazon S3 permissions, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/user-guide/using-with-s3-actions.html\">Specifying permissions in a policy</a>." , long_about = None)]
    PutBucketOwnershipControls(PutBucketOwnershipControlsCommand),
    # [clap (about = "Deletes the lifecycle configuration from the specified bucket. Amazon S3 removes all the lifecycle configuration rules in the lifecycle subresource associated with the bucket. Your objects never expire, and Amazon S3 no longer automatically deletes any objects on the basis of rules contained in the deleted lifecycle configuration." , long_about = None)]
    DeleteBucketLifecycle(DeleteBucketLifecycleCommand),
    # [clap (about = "Applies a legal hold configuration to the specified object. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lock.html\">Locking Objects</a>." , long_about = None)]
    PutObjectLegalHold(PutObjectLegalHoldCommand),
    # [clap (about = "Lists the parts that have been uploaded for a specific multipart upload. This operation must include the upload ID, which you obtain by sending the initiate multipart upload request (see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/API/API_CreateMultipartUpload.html\">CreateMultipartUpload</a>). This request returns a maximum of 1,000 uploaded parts. The default number of parts returned is 1,000 parts. You can restrict the number of parts returned by specifying the <code>max-parts</code> request parameter. If your multipart upload consists of more than 1,000 parts, the response returns an <code>IsTruncated</code> field with the value of true, and a <code>NextPartNumberMarker</code> element. In subsequent <code>ListParts</code> requests you can include the part-number-marker query string parameter and set its value to the <code>NextPartNumberMarker</code> field value from the previous response." , long_about = None)]
    ListParts(ListPartsCommand),
    # [clap (about = "Deletes the S3 Intelligent-Tiering configuration from the specified bucket." , long_about = None)]
    DeleteBucketIntelligentTieringConfiguration(DeleteBucketIntelligentTieringConfigurationCommand),
    # [clap (about = "Returns the notification configuration of a bucket." , long_about = None)]
    GetBucketNotificationConfiguration(GetBucketNotificationConfigurationCommand),
    # [clap (about = "Returns the access control list (ACL) of an object. To use this operation, you must have <code>s3:GetObjectAcl</code> permissions or <code>READ_ACP</code> access to the object. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/userguide/acl-overview.html#acl-access-policy-permission-mapping\">Mapping of ACL permissions and access policy permissions</a> in the <i>Amazon S3 User Guide</i>" , long_about = None)]
    GetObjectAcl(GetObjectAclCommand),
    # [clap (about = "This action enables you to delete multiple objects from a bucket using a single HTTP request. If you know the object keys that you want to delete, then this action provides a suitable alternative to sending individual delete requests, reducing per-request overhead." , long_about = None)]
    DeleteObjects(DeleteObjectsCommand),
    # [clap (about = "Returns the tag-set of an object. You send the GET request against the tagging subresource associated with the object." , long_about = None)]
    GetObjectTagging(GetObjectTaggingCommand),
    # [clap (about = "Returns the versioning state of a bucket." , long_about = None)]
    GetBucketVersioning(GetBucketVersioningCommand),
    # [clap (about = "Returns the website configuration for a bucket. To host website on Amazon S3, you can configure a bucket as website by adding a website configuration. For more information about hosting websites, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/dev/WebsiteHosting.html\">Hosting Websites on Amazon S3</a>." , long_about = None)]
    GetBucketWebsite(GetBucketWebsiteCommand),
    # [clap (about = "Sets the supplied tag-set to an object that already exists in a bucket." , long_about = None)]
    PutObjectTagging(PutObjectTaggingCommand),
    # [clap (about = "Returns torrent files from a bucket. BitTorrent can save you bandwidth when you're distributing large files. For more information about BitTorrent, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/dev/S3Torrent.html\">Using BitTorrent with Amazon S3</a>." , long_about = None)]
    GetObjectTorrent(GetObjectTorrentCommand),
    # [clap (about = "This action lists in-progress multipart uploads. An in-progress multipart upload is a multipart upload that has been initiated using the Initiate Multipart Upload request, but has not yet been completed or aborted." , long_about = None)]
    ListMultipartUploads(ListMultipartUploadsCommand),
    # [clap (about = "Gets the S3 Intelligent-Tiering configuration from the specified bucket." , long_about = None)]
    GetBucketIntelligentTieringConfiguration(GetBucketIntelligentTieringConfigurationCommand),
    # [clap (about = "This implementation of the DELETE action uses the policy subresource to delete the policy of a specified bucket. If you are using an identity other than the root user of the Amazon Web Services account that owns the bucket, the calling identity must have the <code>DeleteBucketPolicy</code> permissions on the specified bucket and belong to the bucket owner's account to use this operation." , long_about = None)]
    DeleteBucketPolicy(DeleteBucketPolicyCommand),
    # [clap (about = "Lists the S3 Intelligent-Tiering configuration from the specified bucket." , long_about = None)]
    ListBucketIntelligentTieringConfigurations(ListBucketIntelligentTieringConfigurationsCommand),
    # [clap (about = "Enables notifications of specified events for a bucket. For more information about event notifications, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/dev/NotificationHowTo.html\">Configuring Event Notifications</a>." , long_about = None)]
    PutBucketNotificationConfiguration(PutBucketNotificationConfigurationCommand),
    # [clap (about = "Lists the analytics configurations for the bucket. You can have up to 1,000 analytics configurations per bucket." , long_about = None)]
    ListBucketAnalyticsConfigurations(ListBucketAnalyticsConfigurationsCommand),
    # [clap (about = "Returns the policy of a specified bucket. If you are using an identity other than the root user of the Amazon Web Services account that owns the bucket, the calling identity must have the <code>GetBucketPolicy</code> permissions on the specified bucket and belong to the bucket owner's account in order to use this operation." , long_about = None)]
    GetBucketPolicy(GetBucketPolicyCommand),
    # [clap (about = "Retrieves the policy status for an Amazon S3 bucket, indicating whether the bucket is public. In order to use this operation, you must have the <code>s3:GetBucketPolicyStatus</code> permission. For more information about Amazon S3 permissions, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/dev/using-with-s3-actions.html\">Specifying Permissions in a Policy</a>." , long_about = None)]
    GetBucketPolicyStatus(GetBucketPolicyStatusCommand),
    # [clap (about = "Places an Object Retention configuration on an object. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lock.html\">Locking Objects</a>. Users or accounts require the <code>s3:PutObjectRetention</code> permission in order to place an Object Retention configuration on objects. Bypassing a Governance Retention configuration requires the <code>s3:BypassGovernanceRetention</code> permission." , long_about = None)]
    PutObjectRetention(PutObjectRetentionCommand),
    # [clap (about = "Deletes the S3 bucket. All objects (including all object versions and delete markers) in the bucket must be deleted before the bucket itself can be deleted." , long_about = None)]
    DeleteBucket(DeleteBucketCommand),
    # [clap (about = "Uploads a part by copying data from an existing object as data source. You specify the data source by adding the request header <code>x-amz-copy-source</code> in your request and a byte range by adding the request header <code>x-amz-copy-source-range</code> in your request." , long_about = None)]
    UploadPartCopy(UploadPartCopyCommand),
    # [clap (about = "Creates or modifies the <code>PublicAccessBlock</code> configuration for an Amazon S3 bucket. To use this operation, you must have the <code>s3:PutBucketPublicAccessBlock</code> permission. For more information about Amazon S3 permissions, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/dev/using-with-s3-actions.html\">Specifying Permissions in a Policy</a>." , long_about = None)]
    PutPublicAccessBlock(PutPublicAccessBlockCommand),
    # [clap (about = "This action aborts a multipart upload. After a multipart upload is aborted, no additional parts can be uploaded using that upload ID. The storage consumed by any previously uploaded parts will be freed. However, if any part uploads are currently in progress, those part uploads might or might not succeed. As a result, it might be necessary to abort a given multipart upload multiple times in order to completely free all storage consumed by all parts." , long_about = None)]
    AbortMultipartUpload(AbortMultipartUploadCommand),
    # [clap (about = "Retrieves the <code>PublicAccessBlock</code> configuration for an Amazon S3 bucket. To use this operation, you must have the <code>s3:GetBucketPublicAccessBlock</code> permission. For more information about Amazon S3 permissions, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/dev/using-with-s3-actions.html\">Specifying Permissions in a Policy</a>." , long_about = None)]
    GetPublicAccessBlock(GetPublicAccessBlockCommand),
    # [clap (about = "This implementation of the GET action returns an analytics configuration (identified by the analytics configuration ID) from the bucket." , long_about = None)]
    GetBucketAnalyticsConfiguration(GetBucketAnalyticsConfigurationCommand),
    # [clap (about = "Set the logging parameters for a bucket and to specify permissions for who can view and modify the logging parameters. All logs are saved to buckets in the same Amazon Web Services Region as the source bucket. To set the logging status of a bucket, you must be the bucket owner." , long_about = None)]
    PutBucketLogging(PutBucketLoggingCommand),
    # [clap (about = "Places an Object Lock configuration on the specified bucket. The rule specified in the Object Lock configuration will be applied by default to every new object placed in the specified bucket. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lock.html\">Locking Objects</a>." , long_about = None)]
    PutObjectLockConfiguration(PutObjectLockConfigurationCommand),
    # [clap (about = "This implementation of the DELETE action removes default encryption from the bucket. For information about the Amazon S3 default encryption feature, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/dev/bucket-encryption.html\">Amazon S3 Default Bucket Encryption</a> in the <i>Amazon S3 User Guide</i>." , long_about = None)]
    DeleteBucketEncryption(DeleteBucketEncryptionCommand),
    # [clap (about = "Returns an inventory configuration (identified by the inventory configuration ID) from the bucket." , long_about = None)]
    GetBucketInventoryConfiguration(GetBucketInventoryConfigurationCommand),
    # [clap (about = "This implementation of the GET action uses the <code>accelerate</code> subresource to return the Transfer Acceleration state of a bucket, which is either <code>Enabled</code> or <code>Suspended</code>. Amazon S3 Transfer Acceleration is a bucket-level feature that enables you to perform faster data transfers to and from Amazon S3." , long_about = None)]
    GetBucketAccelerateConfiguration(GetBucketAccelerateConfigurationCommand),
    # [clap (about = "Sets the versioning state of an existing bucket." , long_about = None)]
    PutBucketVersioning(PutBucketVersioningCommand),
    # [clap (about = "Sets the request payment configuration for a bucket. By default, the bucket owner pays for downloads from the bucket. This configuration parameter enables the bucket owner (only) to specify that the person requesting the download will be charged for the download. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/dev/RequesterPaysBuckets.html\">Requester Pays Buckets</a>." , long_about = None)]
    PutBucketRequestPayment(PutBucketRequestPaymentCommand),
    # [clap (about = "Deletes an inventory configuration (identified by the inventory ID) from the bucket." , long_about = None)]
    DeleteBucketInventoryConfiguration(DeleteBucketInventoryConfigurationCommand),
    # [clap (about = "This implementation of the <code>PUT</code> action adds an inventory configuration (identified by the inventory ID) to the bucket. You can have up to 1,000 inventory configurations per bucket." , long_about = None)]
    PutBucketInventoryConfiguration(PutBucketInventoryConfigurationCommand),
    # [clap (about = "Uploads a part in a multipart upload." , long_about = None)]
    UploadPart(UploadPartCommand),
    # [clap (about = "Returns the request payment configuration of a bucket. To use this version of the operation, you must be the bucket owner. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/dev/RequesterPaysBuckets.html\">Requester Pays Buckets</a>." , long_about = None)]
    GetBucketRequestPayment(GetBucketRequestPaymentCommand),
    # [clap (about = "Retrieves <code>OwnershipControls</code> for an Amazon S3 bucket. To use this operation, you must have the <code>s3:GetBucketOwnershipControls</code> permission. For more information about Amazon S3 permissions, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-with-s3-actions.html\">Specifying permissions in a policy</a>." , long_about = None)]
    GetBucketOwnershipControls(GetBucketOwnershipControlsCommand),
    # [clap (about = "Puts a S3 Intelligent-Tiering configuration to the specified bucket. You can have up to 1,000 S3 Intelligent-Tiering configurations per bucket." , long_about = None)]
    PutBucketIntelligentTieringConfiguration(PutBucketIntelligentTieringConfigurationCommand),
    # [clap (about = "The HEAD action retrieves metadata from an object without returning the object itself. This action is useful if you're only interested in an object's metadata. To use HEAD, you must have READ access to the object." , long_about = None)]
    HeadObject(HeadObjectCommand),
    # [clap (about = "Sets a metrics configuration (specified by the metrics configuration ID) for the bucket. You can have up to 1,000 metrics configurations per bucket. If you're updating an existing metrics configuration, note that this is a full replacement of the existing metrics configuration. If you don't include the elements you want to keep, they are erased." , long_about = None)]
    PutBucketMetricsConfiguration(PutBucketMetricsConfigurationCommand),
    # [clap (about = "Restores an archived copy of an object back into Amazon S3" , long_about = None)]
    RestoreObject(RestoreObjectCommand),
    # [clap (about = "Creates a copy of an object that is already stored in Amazon S3." , long_about = None)]
    CopyObject(CopyObjectCommand),
    # [clap (about = "Applies an Amazon S3 bucket policy to an Amazon S3 bucket. If you are using an identity other than the root user of the Amazon Web Services account that owns the bucket, the calling identity must have the <code>PutBucketPolicy</code> permissions on the specified bucket and belong to the bucket owner's account in order to use this operation." , long_about = None)]
    PutBucketPolicy(PutBucketPolicyCommand),
    # [clap (about = "Gets an object's current legal hold status. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lock.html\">Locking Objects</a>." , long_about = None)]
    GetObjectLegalHold(GetObjectLegalHoldCommand),
    # [clap (about = "Creates a replication configuration or replaces an existing one. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/dev/replication.html\">Replication</a> in the <i>Amazon S3 User Guide</i>." , long_about = None)]
    PutBucketReplication(PutBucketReplicationCommand),
    # [clap (about = "Deletes the <code>cors</code> configuration information set for the bucket." , long_about = None)]
    DeleteBucketCors(DeleteBucketCorsCommand),
    # [clap (about = "Returns the Region the bucket resides in. You set the bucket's Region using the <code>LocationConstraint</code> request parameter in a <code>CreateBucket</code> request. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/API/API_CreateBucket.html\">CreateBucket</a>." , long_about = None)]
    GetBucketLocation(GetBucketLocationCommand),
    # [clap (about = "Deletes an analytics configuration for the bucket (specified by the analytics configuration ID)." , long_about = None)]
    DeleteBucketAnalyticsConfiguration(DeleteBucketAnalyticsConfigurationCommand),
    # [clap (about = "Sets the tags for a bucket." , long_about = None)]
    PutBucketTagging(PutBucketTaggingCommand),
    # [clap (about = "Bucket lifecycle configuration now supports specifying a lifecycle rule using an object key name prefix, one or more object tags, or a combination of both. Accordingly, this section describes the latest API. The response describes the new filter element that you can use to specify a filter to select a subset of objects to which the rule applies. If you are using a previous version of the lifecycle configuration, it still works. For the earlier action, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketLifecycle.html\">GetBucketLifecycle</a>." , long_about = None)]
    GetBucketLifecycleConfiguration(GetBucketLifecycleConfigurationCommand),
    # [clap (about = "This action is useful to determine if a bucket exists and you have permission to access it. The action returns a <code>200 OK</code> if the bucket exists and you have permission to access it." , long_about = None)]
    HeadBucket(HeadBucketCommand),
    # [clap (about = "Uses the <code>acl</code> subresource to set the access control list (ACL) permissions for a new or existing object in an S3 bucket. You must have <code>WRITE_ACP</code> permission to set the ACL of an object. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/dev/acl-overview.html#permissions\">What permissions can I grant?</a> in the <i>Amazon S3 User Guide</i>." , long_about = None)]
    PutObjectAcl(PutObjectAclCommand),
    # [clap (about = "Returns a list of all buckets owned by the authenticated sender of the request. To use this operation, you must have the <code>s3:ListAllMyBuckets</code> permission." , long_about = None)]
    ListBuckets(ListBucketsCommand),
    # [clap (about = "Adds an object to a bucket. You must have WRITE permissions on a bucket to add an object to it." , long_about = None)]
    PutObject(PutObjectCommand),
    # [clap (about = "This action initiates a multipart upload and returns an upload ID. This upload ID is used to associate all of the parts in the specific multipart upload. You specify this upload ID in each of your subsequent upload part requests (see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/API/API_UploadPart.html\">UploadPart</a>). You also include this upload ID in the final request to either complete or abort the multipart upload request." , long_about = None)]
    CreateMultipartUpload(CreateMultipartUploadCommand),
    # [clap (about = "Returns the Cross-Origin Resource Sharing (CORS) configuration information set for the bucket." , long_about = None)]
    GetBucketCors(GetBucketCorsCommand),
    # [clap (about = "Returns a list of inventory configurations for the bucket. You can have up to 1,000 analytics configurations per bucket." , long_about = None)]
    ListBucketInventoryConfigurations(ListBucketInventoryConfigurationsCommand),
    # [clap (about = "Returns the replication configuration of a bucket." , long_about = None)]
    GetBucketReplication(GetBucketReplicationCommand),
    # [clap (about = "Completes a multipart upload by assembling previously uploaded parts." , long_about = None)]
    CompleteMultipartUpload(CompleteMultipartUploadCommand),
    # [clap (about = "Returns the tag set associated with the bucket." , long_about = None)]
    GetBucketTagging(GetBucketTaggingCommand),
    # [clap (about = "Sets the <code>cors</code> configuration for your bucket. If the configuration exists, Amazon S3 replaces it." , long_about = None)]
    PutBucketCors(PutBucketCorsCommand),
    # [clap (about = "Returns some or all (up to 1,000) of the objects in a bucket with each request. You can use the request parameters as selection criteria to return a subset of the objects in a bucket. A <code>200 OK</code> response can contain valid or invalid XML. Make sure to design your application to parse the contents of the response and handle it appropriately. Objects are returned sorted in an ascending order of the respective key names in the list. For more information about listing objects, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/userguide/ListingKeysUsingAPIs.html\">Listing object keys programmatically</a>" , long_about = None)]
    ListObjectsV2(ListObjectsV2Command),
    # [clap (about = "Removes the entire tag set from the specified object. For more information about managing object tags, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/dev/object-tagging.html\"> Object Tagging</a>." , long_about = None)]
    DeleteObjectTagging(DeleteObjectTaggingCommand),
    # [clap (about = "Retrieves all the metadata from an object without returning the object itself. This action is useful if you're interested only in an object's metadata. To use <code>GetObjectAttributes</code>, you must have READ access to the object." , long_about = None)]
    GetObjectAttributes(GetObjectAttributesCommand),
    # [clap (about = "Retrieves an object's retention settings. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lock.html\">Locking Objects</a>." , long_about = None)]
    GetObjectRetention(GetObjectRetentionCommand),
    # [clap (about = "Retrieves objects from Amazon S3. To use <code>GET</code>, you must have <code>READ</code> access to the object. If you grant <code>READ</code> access to the anonymous user, you can return the object without using an authorization header." , long_about = None)]
    GetObject(GetObjectCommand),
    # [clap (about = "Deletes a metrics configuration for the Amazon CloudWatch request metrics (specified by the metrics configuration ID) from the bucket. Note that this doesn't include the daily storage metrics." , long_about = None)]
    DeleteBucketMetricsConfiguration(DeleteBucketMetricsConfigurationCommand),
    # [clap (about = "Sets the permissions on an existing bucket using access control lists (ACL). For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/dev/S3_ACLs_UsingACLs.html\">Using ACLs</a>. To set the ACL of a bucket, you must have <code>WRITE_ACP</code> permission." , long_about = None)]
    PutBucketAcl(PutBucketAclCommand),
    # [clap (about = "This action uses the <code>encryption</code> subresource to configure default encryption and Amazon S3 Bucket Key for an existing bucket." , long_about = None)]
    PutBucketEncryption(PutBucketEncryptionCommand),
    # [clap (about = "Sets the accelerate configuration of an existing bucket. Amazon S3 Transfer Acceleration is a bucket-level feature that enables you to perform faster data transfers to Amazon S3." , long_about = None)]
    PutBucketAccelerateConfiguration(PutBucketAccelerateConfigurationCommand),
    # [clap (about = "Removes the <code>PublicAccessBlock</code> configuration for an Amazon S3 bucket. To use this operation, you must have the <code>s3:PutBucketPublicAccessBlock</code> permission. For more information about permissions, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-with-s3-actions.html#using-with-s3-actions-related-to-bucket-subresources\">Permissions Related to Bucket Subresource Operations</a> and <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/userguide/s3-access-control.html\">Managing Access Permissions to Your Amazon S3 Resources</a>." , long_about = None)]
    DeletePublicAccessBlock(DeletePublicAccessBlockCommand),
    # [clap (about = "Deletes the replication configuration from the bucket." , long_about = None)]
    DeleteBucketReplication(DeleteBucketReplicationCommand),
    # [clap (about = "Sets the configuration of the website that is specified in the <code>website</code> subresource. To configure a bucket as a website, you can add this subresource on the bucket with website configuration information such as the file name of the index document and any redirect rules. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/dev/WebsiteHosting.html\">Hosting Websites on Amazon S3</a>." , long_about = None)]
    PutBucketWebsite(PutBucketWebsiteCommand),
    # [clap (about = "Creates a new S3 bucket. To create a bucket, you must register with Amazon S3 and have a valid Amazon Web Services Access Key ID to authenticate requests. Anonymous requests are never allowed to create buckets. By creating the bucket, you become the bucket owner." , long_about = None)]
    CreateBucket(CreateBucketCommand),
    # [clap (about = "Removes the null version (if there is one) of an object and inserts a delete marker, which becomes the latest version of the object. If there isn't a null version, Amazon S3 does not remove any objects but will still respond that the command was successful." , long_about = None)]
    DeleteObject(DeleteObjectCommand),
    # [clap (about = "Creates a new lifecycle configuration for the bucket or replaces an existing lifecycle configuration. Keep in mind that this will overwrite an existing lifecycle configuration, so if you want to retain any configuration details, they must be included in the new lifecycle configuration. For information about lifecycle configuration, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-lifecycle-mgmt.html\">Managing your storage lifecycle</a>." , long_about = None)]
    PutBucketLifecycleConfiguration(PutBucketLifecycleConfigurationCommand),
    # [clap (about = "Removes <code>OwnershipControls</code> for an Amazon S3 bucket. To use this operation, you must have the <code>s3:PutBucketOwnershipControls</code> permission. For more information about Amazon S3 permissions, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/dev/using-with-s3-actions.html\">Specifying Permissions in a Policy</a>." , long_about = None)]
    DeleteBucketOwnershipControls(DeleteBucketOwnershipControlsCommand),
    # [clap (about = "Returns metadata about all versions of the objects in a bucket. You can also use request parameters as selection criteria to return metadata about a subset of all the object versions." , long_about = None)]
    ListObjectVersions(ListObjectVersionsCommand),
    # [clap (about = "Returns the default encryption configuration for an Amazon S3 bucket. If the bucket does not have a default encryption configuration, GetBucketEncryption returns <code>ServerSideEncryptionConfigurationNotFoundError</code>." , long_about = None)]
    GetBucketEncryption(GetBucketEncryptionCommand),
    # [clap (about = "Deletes the tags from the bucket." , long_about = None)]
    DeleteBucketTagging(DeleteBucketTaggingCommand),
    # [clap (about = "Lists the metrics configurations for the bucket. The metrics configurations are only for the request metrics of the bucket and do not provide information on daily storage metrics. You can have up to 1,000 configurations per bucket." , long_about = None)]
    ListBucketMetricsConfigurations(ListBucketMetricsConfigurationsCommand),
    # [clap (about = "Sets an analytics configuration for the bucket (specified by the analytics configuration ID). You can have up to 1,000 analytics configurations per bucket." , long_about = None)]
    PutBucketAnalyticsConfiguration(PutBucketAnalyticsConfigurationCommand),
    # [clap (about = "Returns some or all (up to 1,000) of the objects in a bucket. You can use the request parameters as selection criteria to return a subset of the objects in a bucket. A 200 OK response can contain valid or invalid XML. Be sure to design your application to parse the contents of the response and handle it appropriately." , long_about = None)]
    ListObjects(ListObjectsCommand),
}
impl S3OpsCommands {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        match self {
            S3OpsCommands::DeleteBucketWebsite(cmd) => cmd.run(s3).await,
            S3OpsCommands::GetBucketMetricsConfiguration(cmd) => cmd.run(s3).await,
            S3OpsCommands::GetBucketAcl(cmd) => cmd.run(s3).await,
            S3OpsCommands::GetObjectLockConfiguration(cmd) => cmd.run(s3).await,
            S3OpsCommands::GetBucketLogging(cmd) => cmd.run(s3).await,
            S3OpsCommands::PutBucketOwnershipControls(cmd) => cmd.run(s3).await,
            S3OpsCommands::DeleteBucketLifecycle(cmd) => cmd.run(s3).await,
            S3OpsCommands::PutObjectLegalHold(cmd) => cmd.run(s3).await,
            S3OpsCommands::ListParts(cmd) => cmd.run(s3).await,
            S3OpsCommands::DeleteBucketIntelligentTieringConfiguration(cmd) => cmd.run(s3).await,
            S3OpsCommands::GetBucketNotificationConfiguration(cmd) => cmd.run(s3).await,
            S3OpsCommands::GetObjectAcl(cmd) => cmd.run(s3).await,
            S3OpsCommands::DeleteObjects(cmd) => cmd.run(s3).await,
            S3OpsCommands::GetObjectTagging(cmd) => cmd.run(s3).await,
            S3OpsCommands::GetBucketVersioning(cmd) => cmd.run(s3).await,
            S3OpsCommands::GetBucketWebsite(cmd) => cmd.run(s3).await,
            S3OpsCommands::PutObjectTagging(cmd) => cmd.run(s3).await,
            S3OpsCommands::GetObjectTorrent(cmd) => cmd.run(s3).await,
            S3OpsCommands::ListMultipartUploads(cmd) => cmd.run(s3).await,
            S3OpsCommands::GetBucketIntelligentTieringConfiguration(cmd) => cmd.run(s3).await,
            S3OpsCommands::DeleteBucketPolicy(cmd) => cmd.run(s3).await,
            S3OpsCommands::ListBucketIntelligentTieringConfigurations(cmd) => cmd.run(s3).await,
            S3OpsCommands::PutBucketNotificationConfiguration(cmd) => cmd.run(s3).await,
            S3OpsCommands::ListBucketAnalyticsConfigurations(cmd) => cmd.run(s3).await,
            S3OpsCommands::GetBucketPolicy(cmd) => cmd.run(s3).await,
            S3OpsCommands::GetBucketPolicyStatus(cmd) => cmd.run(s3).await,
            S3OpsCommands::PutObjectRetention(cmd) => cmd.run(s3).await,
            S3OpsCommands::DeleteBucket(cmd) => cmd.run(s3).await,
            S3OpsCommands::UploadPartCopy(cmd) => cmd.run(s3).await,
            S3OpsCommands::PutPublicAccessBlock(cmd) => cmd.run(s3).await,
            S3OpsCommands::AbortMultipartUpload(cmd) => cmd.run(s3).await,
            S3OpsCommands::GetPublicAccessBlock(cmd) => cmd.run(s3).await,
            S3OpsCommands::GetBucketAnalyticsConfiguration(cmd) => cmd.run(s3).await,
            S3OpsCommands::PutBucketLogging(cmd) => cmd.run(s3).await,
            S3OpsCommands::PutObjectLockConfiguration(cmd) => cmd.run(s3).await,
            S3OpsCommands::DeleteBucketEncryption(cmd) => cmd.run(s3).await,
            S3OpsCommands::GetBucketInventoryConfiguration(cmd) => cmd.run(s3).await,
            S3OpsCommands::GetBucketAccelerateConfiguration(cmd) => cmd.run(s3).await,
            S3OpsCommands::PutBucketVersioning(cmd) => cmd.run(s3).await,
            S3OpsCommands::PutBucketRequestPayment(cmd) => cmd.run(s3).await,
            S3OpsCommands::DeleteBucketInventoryConfiguration(cmd) => cmd.run(s3).await,
            S3OpsCommands::PutBucketInventoryConfiguration(cmd) => cmd.run(s3).await,
            S3OpsCommands::UploadPart(cmd) => cmd.run(s3).await,
            S3OpsCommands::GetBucketRequestPayment(cmd) => cmd.run(s3).await,
            S3OpsCommands::GetBucketOwnershipControls(cmd) => cmd.run(s3).await,
            S3OpsCommands::PutBucketIntelligentTieringConfiguration(cmd) => cmd.run(s3).await,
            S3OpsCommands::HeadObject(cmd) => cmd.run(s3).await,
            S3OpsCommands::PutBucketMetricsConfiguration(cmd) => cmd.run(s3).await,
            S3OpsCommands::RestoreObject(cmd) => cmd.run(s3).await,
            S3OpsCommands::CopyObject(cmd) => cmd.run(s3).await,
            S3OpsCommands::PutBucketPolicy(cmd) => cmd.run(s3).await,
            S3OpsCommands::GetObjectLegalHold(cmd) => cmd.run(s3).await,
            S3OpsCommands::PutBucketReplication(cmd) => cmd.run(s3).await,
            S3OpsCommands::DeleteBucketCors(cmd) => cmd.run(s3).await,
            S3OpsCommands::GetBucketLocation(cmd) => cmd.run(s3).await,
            S3OpsCommands::DeleteBucketAnalyticsConfiguration(cmd) => cmd.run(s3).await,
            S3OpsCommands::PutBucketTagging(cmd) => cmd.run(s3).await,
            S3OpsCommands::GetBucketLifecycleConfiguration(cmd) => cmd.run(s3).await,
            S3OpsCommands::HeadBucket(cmd) => cmd.run(s3).await,
            S3OpsCommands::PutObjectAcl(cmd) => cmd.run(s3).await,
            S3OpsCommands::ListBuckets(cmd) => cmd.run(s3).await,
            S3OpsCommands::PutObject(cmd) => cmd.run(s3).await,
            S3OpsCommands::CreateMultipartUpload(cmd) => cmd.run(s3).await,
            S3OpsCommands::GetBucketCors(cmd) => cmd.run(s3).await,
            S3OpsCommands::ListBucketInventoryConfigurations(cmd) => cmd.run(s3).await,
            S3OpsCommands::GetBucketReplication(cmd) => cmd.run(s3).await,
            S3OpsCommands::CompleteMultipartUpload(cmd) => cmd.run(s3).await,
            S3OpsCommands::GetBucketTagging(cmd) => cmd.run(s3).await,
            S3OpsCommands::PutBucketCors(cmd) => cmd.run(s3).await,
            S3OpsCommands::ListObjectsV2(cmd) => cmd.run(s3).await,
            S3OpsCommands::DeleteObjectTagging(cmd) => cmd.run(s3).await,
            S3OpsCommands::GetObjectAttributes(cmd) => cmd.run(s3).await,
            S3OpsCommands::GetObjectRetention(cmd) => cmd.run(s3).await,
            S3OpsCommands::GetObject(cmd) => cmd.run(s3).await,
            S3OpsCommands::DeleteBucketMetricsConfiguration(cmd) => cmd.run(s3).await,
            S3OpsCommands::PutBucketAcl(cmd) => cmd.run(s3).await,
            S3OpsCommands::PutBucketEncryption(cmd) => cmd.run(s3).await,
            S3OpsCommands::PutBucketAccelerateConfiguration(cmd) => cmd.run(s3).await,
            S3OpsCommands::DeletePublicAccessBlock(cmd) => cmd.run(s3).await,
            S3OpsCommands::DeleteBucketReplication(cmd) => cmd.run(s3).await,
            S3OpsCommands::PutBucketWebsite(cmd) => cmd.run(s3).await,
            S3OpsCommands::CreateBucket(cmd) => cmd.run(s3).await,
            S3OpsCommands::DeleteObject(cmd) => cmd.run(s3).await,
            S3OpsCommands::PutBucketLifecycleConfiguration(cmd) => cmd.run(s3).await,
            S3OpsCommands::DeleteBucketOwnershipControls(cmd) => cmd.run(s3).await,
            S3OpsCommands::ListObjectVersions(cmd) => cmd.run(s3).await,
            S3OpsCommands::GetBucketEncryption(cmd) => cmd.run(s3).await,
            S3OpsCommands::DeleteBucketTagging(cmd) => cmd.run(s3).await,
            S3OpsCommands::ListBucketMetricsConfigurations(cmd) => cmd.run(s3).await,
            S3OpsCommands::PutBucketAnalyticsConfiguration(cmd) => cmd.run(s3).await,
            S3OpsCommands::ListObjects(cmd) => cmd.run(s3).await,
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct DeleteBucketWebsiteCommand {
    # [clap (long = "bucket" , name = "Bucket" , help = "The bucket name for which you want to remove the website configuration." , long_help = None)]
    bucket: String,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
}
impl DeleteBucketWebsiteCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .delete_bucket_website()
            .set_bucket(Some(self.bucket.to_owned()))
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct GetBucketMetricsConfigurationCommand {
    # [clap (long = "id" , name = "Id" , help = "The ID used to identify the metrics configuration." , long_help = None)]
    id: String,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    # [clap (long = "bucket" , name = "Bucket" , help = "The name of the bucket containing the metrics configuration to retrieve." , long_help = None)]
    bucket: String,
}
impl GetBucketMetricsConfigurationCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .get_bucket_metrics_configuration()
            .set_id(Some(self.id.to_owned()))
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_bucket(Some(self.bucket.to_owned()))
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct GetBucketAclCommand {
    # [clap (long = "bucket" , name = "Bucket" , help = "Specifies the S3 bucket whose ACL is being requested." , long_help = None)]
    bucket: String,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
}
impl GetBucketAclCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .get_bucket_acl()
            .set_bucket(Some(self.bucket.to_owned()))
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct GetObjectLockConfigurationCommand {
    # [clap (long = "bucket" , name = "Bucket" , help = "The bucket whose Object Lock configuration you want to retrieve." , long_help = None)]
    bucket: String,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
}
impl GetObjectLockConfigurationCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .get_object_lock_configuration()
            .set_bucket(Some(self.bucket.to_owned()))
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct GetBucketLoggingCommand {
    # [clap (long = "bucket" , name = "Bucket" , help = "The bucket name for which to get the logging information." , long_help = None)]
    bucket: String,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
}
impl GetBucketLoggingCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .get_bucket_logging()
            .set_bucket(Some(self.bucket.to_owned()))
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Args, Debug, Clone)]
pub struct OwnershipControlsArgs {
    #[clap(long = "rules")]
    rules: Option<String>,
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct PutBucketOwnershipControlsCommand {
    # [clap (long = "content-md5" , name = "ContentMD5" , help = "The MD5 hash of the <code>OwnershipControls</code> request body." , long_help = None)]
    content_md5: Option<String>,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    #[clap(flatten)]
    ownership_controls: OwnershipControlsArgs,
    # [clap (long = "bucket" , name = "Bucket" , help = "The name of the Amazon S3 bucket whose <code>OwnershipControls</code> you want to set." , long_help = None)]
    bucket: String,
}
impl PutBucketOwnershipControlsCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .put_bucket_ownership_controls()
            .set_content_md5(self.content_md5.to_owned())
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_ownership_controls(None)
            .set_bucket(Some(self.bucket.to_owned()))
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct DeleteBucketLifecycleCommand {
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    # [clap (long = "bucket" , name = "Bucket" , help = "The bucket name of the lifecycle to delete." , long_help = None)]
    bucket: String,
}
impl DeleteBucketLifecycleCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .delete_bucket_lifecycle()
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_bucket(Some(self.bucket.to_owned()))
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Args, Debug, Clone)]
pub struct ObjectLockLegalHoldArgs {
    #[clap(long = "status")]
    status: Option<String>,
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct PutObjectLegalHoldCommand {
    # [clap (long = "request-payer" , name = "RequestPayer" , help = "" , long_help = None)]
    request_payer: Option<String>,
    # [clap (long = "bucket" , name = "Bucket" , help = "The bucket name containing the object that you want to place a legal hold on." , long_help = None)]
    bucket: String,
    #[clap(flatten)]
    legal_hold: ObjectLockLegalHoldArgs,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    # [clap (long = "key" , name = "Key" , help = "The key name for the object that you want to place a legal hold on." , long_help = None)]
    key: String,
    # [clap (long = "version-id" , name = "VersionId" , help = "The version ID of the object that you want to place a legal hold on." , long_help = None)]
    version_id: Option<String>,
    # [clap (long = "checksum-algorithm" , name = "ChecksumAlgorithm" , help = "Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html\">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>." , long_help = None)]
    checksum_algorithm: Option<String>,
    # [clap (long = "content-md5" , name = "ContentMD5" , help = "The MD5 hash for the request body." , long_help = None)]
    content_md5: Option<String>,
}
impl PutObjectLegalHoldCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .put_object_legal_hold()
            .set_request_payer(None)
            .set_bucket(Some(self.bucket.to_owned()))
            .set_legal_hold(None)
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_key(Some(self.key.to_owned()))
            .set_version_id(self.version_id.to_owned())
            .set_checksum_algorithm(None)
            .set_content_md5(self.content_md5.to_owned())
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct ListPartsCommand {
    # [clap (long = "part-number-marker" , name = "PartNumberMarker" , help = "Specifies the part after which listing should begin. Only parts with higher part numbers will be listed." , long_help = None)]
    part_number_marker: Option<String>,
    # [clap (long = "bucket" , name = "Bucket" , help = "The name of the bucket to which the parts are being uploaded." , long_help = None)]
    bucket: String,
    # [clap (long = "sse-customer-key-md5" , name = "SSECustomerKeyMD5" , help = "The MD5 server-side encryption (SSE) customer managed key. This parameter is needed only when the object was created using a checksum algorithm. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html\">Protecting data using SSE-C keys</a> in the <i>Amazon S3 User Guide</i>." , long_help = None)]
    sse_customer_key_md5: Option<String>,
    # [clap (long = "sse-customer-algorithm" , name = "SSECustomerAlgorithm" , help = "The server-side encryption (SSE) algorithm used to encrypt the object. This parameter is needed only when the object was created using a checksum algorithm. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html\">Protecting data using SSE-C keys</a> in the <i>Amazon S3 User Guide</i>." , long_help = None)]
    sse_customer_algorithm: Option<String>,
    # [clap (long = "sse-customer-key" , name = "SSECustomerKey" , help = "The server-side encryption (SSE) customer managed key. This parameter is needed only when the object was created using a checksum algorithm. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html\">Protecting data using SSE-C keys</a> in the <i>Amazon S3 User Guide</i>." , long_help = None)]
    sse_customer_key: Option<String>,
    # [clap (long = "upload-id" , name = "UploadId" , help = "Upload ID identifying the multipart upload whose parts are being listed." , long_help = None)]
    upload_id: String,
    # [clap (long = "request-payer" , name = "RequestPayer" , help = "" , long_help = None)]
    request_payer: Option<String>,
    # [clap (long = "key" , name = "Key" , help = "Object key for which the multipart upload was initiated." , long_help = None)]
    key: String,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    # [clap (long = "max-parts" , name = "MaxParts" , help = "Sets the maximum number of parts to return." , long_help = None)]
    max_parts: Option<i32>,
}
impl ListPartsCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .list_parts()
            .set_part_number_marker(self.part_number_marker.to_owned())
            .set_bucket(Some(self.bucket.to_owned()))
            .set_sse_customer_key_md5(self.sse_customer_key_md5.to_owned())
            .set_sse_customer_algorithm(self.sse_customer_algorithm.to_owned())
            .set_sse_customer_key(self.sse_customer_key.to_owned())
            .set_upload_id(Some(self.upload_id.to_owned()))
            .set_request_payer(None)
            .set_key(Some(self.key.to_owned()))
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_max_parts(self.max_parts)
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct DeleteBucketIntelligentTieringConfigurationCommand {
    # [clap (long = "bucket" , name = "Bucket" , help = "The name of the Amazon S3 bucket whose configuration you want to modify or retrieve." , long_help = None)]
    bucket: String,
    # [clap (long = "id" , name = "Id" , help = "The ID used to identify the S3 Intelligent-Tiering configuration." , long_help = None)]
    id: String,
}
impl DeleteBucketIntelligentTieringConfigurationCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .delete_bucket_intelligent_tiering_configuration()
            .set_bucket(Some(self.bucket.to_owned()))
            .set_id(Some(self.id.to_owned()))
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct GetBucketNotificationConfigurationCommand {
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    # [clap (long = "bucket" , name = "Bucket" , help = "The name of the bucket for which to get the notification configuration." , long_help = None)]
    bucket: String,
}
impl GetBucketNotificationConfigurationCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .get_bucket_notification_configuration()
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_bucket(Some(self.bucket.to_owned()))
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct GetObjectAclCommand {
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    # [clap (long = "key" , name = "Key" , help = "The key of the object for which to get the ACL information." , long_help = None)]
    key: String,
    # [clap (long = "version-id" , name = "VersionId" , help = "VersionId used to reference a specific version of the object." , long_help = None)]
    version_id: Option<String>,
    # [clap (long = "bucket" , name = "Bucket" , help = "The bucket name that contains the object for which to get the ACL information." , long_help = None)]
    bucket: String,
    # [clap (long = "request-payer" , name = "RequestPayer" , help = "" , long_help = None)]
    request_payer: Option<String>,
}
impl GetObjectAclCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .get_object_acl()
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_key(Some(self.key.to_owned()))
            .set_version_id(self.version_id.to_owned())
            .set_bucket(Some(self.bucket.to_owned()))
            .set_request_payer(None)
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Args, Debug, Clone)]
pub struct DeleteArgs {
    #[clap(long = "objects")]
    objects: Option<String>,
    #[clap(long = "quiet")]
    quiet: Option<String>,
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct DeleteObjectsCommand {
    # [clap (long = "bucket" , name = "Bucket" , help = "The bucket name containing the objects to delete." , long_help = None)]
    bucket: String,
    # [clap (long = "bypass-governance-retention" , name = "BypassGovernanceRetention" , help = "Specifies whether you want to delete this object even if it has a Governance-type Object Lock in place. To use this header, you must have the <code>s3:BypassGovernanceRetention</code> permission." , long_help = None)]
    bypass_governance_retention: Option<bool>,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    # [clap (long = "checksum-algorithm" , name = "ChecksumAlgorithm" , help = "Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html\">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>." , long_help = None)]
    checksum_algorithm: Option<String>,
    # [clap (long = "mfa" , name = "MFA" , help = "The concatenation of the authentication device's serial number, a space, and the value that is displayed on your authentication device. Required to permanently delete a versioned object if versioning is configured with MFA delete enabled." , long_help = None)]
    mfa: Option<String>,
    #[clap(flatten)]
    delete: DeleteArgs,
    # [clap (long = "request-payer" , name = "RequestPayer" , help = "" , long_help = None)]
    request_payer: Option<String>,
}
impl DeleteObjectsCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .delete_objects()
            .set_bucket(Some(self.bucket.to_owned()))
            .set_bypass_governance_retention(self.bypass_governance_retention)
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_checksum_algorithm(None)
            .set_mfa(self.mfa.to_owned())
            .set_delete(None)
            .set_request_payer(None)
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct GetObjectTaggingCommand {
    # [clap (long = "bucket" , name = "Bucket" , help = "The bucket name containing the object for which to get the tagging information." , long_help = None)]
    bucket: String,
    # [clap (long = "key" , name = "Key" , help = "Object key for which to get the tagging information." , long_help = None)]
    key: String,
    # [clap (long = "request-payer" , name = "RequestPayer" , help = "" , long_help = None)]
    request_payer: Option<String>,
    # [clap (long = "version-id" , name = "VersionId" , help = "The versionId of the object for which to get the tagging information." , long_help = None)]
    version_id: Option<String>,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
}
impl GetObjectTaggingCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .get_object_tagging()
            .set_bucket(Some(self.bucket.to_owned()))
            .set_key(Some(self.key.to_owned()))
            .set_request_payer(None)
            .set_version_id(self.version_id.to_owned())
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct GetBucketVersioningCommand {
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    # [clap (long = "bucket" , name = "Bucket" , help = "The name of the bucket for which to get the versioning information." , long_help = None)]
    bucket: String,
}
impl GetBucketVersioningCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .get_bucket_versioning()
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_bucket(Some(self.bucket.to_owned()))
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct GetBucketWebsiteCommand {
    # [clap (long = "bucket" , name = "Bucket" , help = "The bucket name for which to get the website configuration." , long_help = None)]
    bucket: String,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
}
impl GetBucketWebsiteCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .get_bucket_website()
            .set_bucket(Some(self.bucket.to_owned()))
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Args, Debug, Clone)]
pub struct TaggingArgs {
    #[clap(long = "tag-set")]
    tag_set: Option<String>,
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct PutObjectTaggingCommand {
    # [clap (long = "checksum-algorithm" , name = "ChecksumAlgorithm" , help = "Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html\">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>." , long_help = None)]
    checksum_algorithm: Option<String>,
    # [clap (long = "request-payer" , name = "RequestPayer" , help = "" , long_help = None)]
    request_payer: Option<String>,
    # [clap (long = "version-id" , name = "VersionId" , help = "The versionId of the object that the tag-set will be added to." , long_help = None)]
    version_id: Option<String>,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    # [clap (long = "key" , name = "Key" , help = "Name of the object key." , long_help = None)]
    key: String,
    # [clap (long = "content-md5" , name = "ContentMD5" , help = "The MD5 hash for the request body." , long_help = None)]
    content_md5: Option<String>,
    #[clap(flatten)]
    tagging: TaggingArgs,
    # [clap (long = "bucket" , name = "Bucket" , help = "The bucket name containing the object." , long_help = None)]
    bucket: String,
}
impl PutObjectTaggingCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .put_object_tagging()
            .set_checksum_algorithm(None)
            .set_request_payer(None)
            .set_version_id(self.version_id.to_owned())
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_key(Some(self.key.to_owned()))
            .set_content_md5(self.content_md5.to_owned())
            .set_tagging(None)
            .set_bucket(Some(self.bucket.to_owned()))
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct GetObjectTorrentCommand {
    # [clap (long = "key" , name = "Key" , help = "The object key for which to get the information." , long_help = None)]
    key: String,
    # [clap (long = "request-payer" , name = "RequestPayer" , help = "" , long_help = None)]
    request_payer: Option<String>,
    # [clap (long = "bucket" , name = "Bucket" , help = "The name of the bucket containing the object for which to get the torrent files." , long_help = None)]
    bucket: String,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
}
impl GetObjectTorrentCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .get_object_torrent()
            .set_key(Some(self.key.to_owned()))
            .set_request_payer(None)
            .set_bucket(Some(self.bucket.to_owned()))
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct ListMultipartUploadsCommand {
    # [clap (long = "prefix" , name = "Prefix" , help = "Lists in-progress uploads only for those keys that begin with the specified prefix. You can use prefixes to separate a bucket into different grouping of keys. (You can think of using prefix to make groups in the same way you'd use a folder in a file system.)" , long_help = None)]
    prefix: Option<String>,
    # [clap (long = "delimiter" , name = "Delimiter" , help = "Character you use to group keys." , long_help = None)]
    delimiter: Option<String>,
    # [clap (long = "encoding-type" , name = "EncodingType" , help = "" , long_help = None)]
    encoding_type: Option<String>,
    # [clap (long = "max-uploads" , name = "MaxUploads" , help = "Sets the maximum number of multipart uploads, from 1 to 1,000, to return in the response body. 1,000 is the maximum number of uploads that can be returned in a response." , long_help = None)]
    max_uploads: Option<i32>,
    # [clap (long = "upload-id-marker" , name = "UploadIdMarker" , help = "Together with key-marker, specifies the multipart upload after which listing should begin. If key-marker is not specified, the upload-id-marker parameter is ignored. Otherwise, any multipart uploads for a key equal to the key-marker might be included in the list only if they have an upload ID lexicographically greater than the specified <code>upload-id-marker</code>." , long_help = None)]
    upload_id_marker: Option<String>,
    # [clap (long = "bucket" , name = "Bucket" , help = "The name of the bucket to which the multipart upload was initiated." , long_help = None)]
    bucket: String,
    # [clap (long = "key-marker" , name = "KeyMarker" , help = "Together with upload-id-marker, this parameter specifies the multipart upload after which listing should begin." , long_help = None)]
    key_marker: Option<String>,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
}
impl ListMultipartUploadsCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .list_multipart_uploads()
            .set_prefix(self.prefix.to_owned())
            .set_delimiter(self.delimiter.to_owned())
            .set_encoding_type(None)
            .set_max_uploads(self.max_uploads)
            .set_upload_id_marker(self.upload_id_marker.to_owned())
            .set_bucket(Some(self.bucket.to_owned()))
            .set_key_marker(self.key_marker.to_owned())
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct GetBucketIntelligentTieringConfigurationCommand {
    # [clap (long = "bucket" , name = "Bucket" , help = "The name of the Amazon S3 bucket whose configuration you want to modify or retrieve." , long_help = None)]
    bucket: String,
    # [clap (long = "id" , name = "Id" , help = "The ID used to identify the S3 Intelligent-Tiering configuration." , long_help = None)]
    id: String,
}
impl GetBucketIntelligentTieringConfigurationCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .get_bucket_intelligent_tiering_configuration()
            .set_bucket(Some(self.bucket.to_owned()))
            .set_id(Some(self.id.to_owned()))
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct DeleteBucketPolicyCommand {
    # [clap (long = "bucket" , name = "Bucket" , help = "The bucket name." , long_help = None)]
    bucket: String,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
}
impl DeleteBucketPolicyCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .delete_bucket_policy()
            .set_bucket(Some(self.bucket.to_owned()))
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct ListBucketIntelligentTieringConfigurationsCommand {
    # [clap (long = "continuation-token" , name = "ContinuationToken" , help = "The <code>ContinuationToken</code> that represents a placeholder from where this request should begin." , long_help = None)]
    continuation_token: Option<String>,
    # [clap (long = "bucket" , name = "Bucket" , help = "The name of the Amazon S3 bucket whose configuration you want to modify or retrieve." , long_help = None)]
    bucket: String,
}
impl ListBucketIntelligentTieringConfigurationsCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .list_bucket_intelligent_tiering_configurations()
            .set_continuation_token(self.continuation_token.to_owned())
            .set_bucket(Some(self.bucket.to_owned()))
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Args, Debug, Clone)]
pub struct NotificationConfigurationArgs {
    #[clap(long = "event-bridge-configuration")]
    event_bridge_configuration: Option<String>,
    #[clap(long = "queue-configurations")]
    queue_configurations: Option<String>,
    #[clap(long = "topic-configurations")]
    topic_configurations: Option<String>,
    #[clap(long = "lambda-function-configurations")]
    lambda_function_configurations: Option<String>,
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct PutBucketNotificationConfigurationCommand {
    # [clap (long = "bucket" , name = "Bucket" , help = "The name of the bucket." , long_help = None)]
    bucket: String,
    #[clap(flatten)]
    notification_configuration: NotificationConfigurationArgs,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    # [clap (long = "skip-destination-validation" , name = "SkipDestinationValidation" , help = "Skips validation of Amazon SQS, Amazon SNS, and Lambda destinations. True or false value." , long_help = None)]
    skip_destination_validation: Option<bool>,
}
impl PutBucketNotificationConfigurationCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .put_bucket_notification_configuration()
            .set_bucket(Some(self.bucket.to_owned()))
            .set_notification_configuration(None)
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_skip_destination_validation(self.skip_destination_validation)
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct ListBucketAnalyticsConfigurationsCommand {
    # [clap (long = "bucket" , name = "Bucket" , help = "The name of the bucket from which analytics configurations are retrieved." , long_help = None)]
    bucket: String,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    # [clap (long = "continuation-token" , name = "ContinuationToken" , help = "The ContinuationToken that represents a placeholder from where this request should begin." , long_help = None)]
    continuation_token: Option<String>,
}
impl ListBucketAnalyticsConfigurationsCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .list_bucket_analytics_configurations()
            .set_bucket(Some(self.bucket.to_owned()))
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_continuation_token(self.continuation_token.to_owned())
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct GetBucketPolicyCommand {
    # [clap (long = "bucket" , name = "Bucket" , help = "The bucket name for which to get the bucket policy." , long_help = None)]
    bucket: String,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
}
impl GetBucketPolicyCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .get_bucket_policy()
            .set_bucket(Some(self.bucket.to_owned()))
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct GetBucketPolicyStatusCommand {
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    # [clap (long = "bucket" , name = "Bucket" , help = "The name of the Amazon S3 bucket whose policy status you want to retrieve." , long_help = None)]
    bucket: String,
}
impl GetBucketPolicyStatusCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .get_bucket_policy_status()
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_bucket(Some(self.bucket.to_owned()))
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Args, Debug, Clone)]
pub struct ObjectLockRetentionArgs {
    #[clap(long = "retain-until-date")]
    retain_until_date: Option<String>,
    #[clap(long = "mode")]
    mode: Option<String>,
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct PutObjectRetentionCommand {
    # [clap (long = "key" , name = "Key" , help = "The key name for the object that you want to apply this Object Retention configuration to." , long_help = None)]
    key: String,
    # [clap (long = "request-payer" , name = "RequestPayer" , help = "" , long_help = None)]
    request_payer: Option<String>,
    # [clap (long = "bypass-governance-retention" , name = "BypassGovernanceRetention" , help = "Indicates whether this action should bypass Governance-mode restrictions." , long_help = None)]
    bypass_governance_retention: Option<bool>,
    # [clap (long = "checksum-algorithm" , name = "ChecksumAlgorithm" , help = "Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html\">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>." , long_help = None)]
    checksum_algorithm: Option<String>,
    # [clap (long = "bucket" , name = "Bucket" , help = "The bucket name that contains the object you want to apply this Object Retention configuration to." , long_help = None)]
    bucket: String,
    # [clap (long = "version-id" , name = "VersionId" , help = "The version ID for the object that you want to apply this Object Retention configuration to." , long_help = None)]
    version_id: Option<String>,
    # [clap (long = "content-md5" , name = "ContentMD5" , help = "The MD5 hash for the request body." , long_help = None)]
    content_md5: Option<String>,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    #[clap(flatten)]
    retention: ObjectLockRetentionArgs,
}
impl PutObjectRetentionCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .put_object_retention()
            .set_key(Some(self.key.to_owned()))
            .set_request_payer(None)
            .set_bypass_governance_retention(self.bypass_governance_retention)
            .set_checksum_algorithm(None)
            .set_bucket(Some(self.bucket.to_owned()))
            .set_version_id(self.version_id.to_owned())
            .set_content_md5(self.content_md5.to_owned())
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_retention(None)
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct DeleteBucketCommand {
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    # [clap (long = "bucket" , name = "Bucket" , help = "Specifies the bucket being deleted." , long_help = None)]
    bucket: String,
}
impl DeleteBucketCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .delete_bucket()
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_bucket(Some(self.bucket.to_owned()))
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct UploadPartCopyCommand {
    # [clap (long = "expected-source-bucket-owner" , name = "ExpectedSourceBucketOwner" , help = "The account ID of the expected source bucket owner. If the source bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_source_bucket_owner: Option<String>,
    # [clap (long = "upload-id" , name = "UploadId" , help = "Upload ID identifying the multipart upload whose part is being copied." , long_help = None)]
    upload_id: String,
    # [clap (long = "copy-source-if-none-match" , name = "CopySourceIfNoneMatch" , help = "Copies the object if its entity tag (ETag) is different than the specified ETag." , long_help = None)]
    copy_source_if_none_match: Option<String>,
    # [clap (long = "copy-source-if-unmodified-since" , name = "CopySourceIfUnmodifiedSince" , help = "Copies the object if it hasn't been modified since the specified time." , long_help = None)]
    copy_source_if_unmodified_since: Option<String>,
    # [clap (long = "sse-customer-algorithm" , name = "SSECustomerAlgorithm" , help = "Specifies the algorithm to use to when encrypting the object (for example, AES256)." , long_help = None)]
    sse_customer_algorithm: Option<String>,
    # [clap (long = "copy-source-if-match" , name = "CopySourceIfMatch" , help = "Copies the object if its entity tag (ETag) matches the specified tag." , long_help = None)]
    copy_source_if_match: Option<String>,
    # [clap (long = "copy-source" , name = "CopySource" , help = "Specifies the source object for the copy operation. You specify the value in one of two formats, depending on whether you want to access the source object through an <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/userguide/access-points.html\">access point</a>:" , long_help = None)]
    copy_source: String,
    # [clap (long = "copy-source-if-modified-since" , name = "CopySourceIfModifiedSince" , help = "Copies the object if it has been modified since the specified time." , long_help = None)]
    copy_source_if_modified_since: Option<String>,
    # [clap (long = "copy-source-sse-customer-key-md5" , name = "CopySourceSSECustomerKeyMD5" , help = "Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure that the encryption key was transmitted without error." , long_help = None)]
    copy_source_sse_customer_key_md5: Option<String>,
    # [clap (long = "copy-source-sse-customer-key" , name = "CopySourceSSECustomerKey" , help = "Specifies the customer-provided encryption key for Amazon S3 to use to decrypt the source object. The encryption key provided in this header must be one that was used when the source object was created." , long_help = None)]
    copy_source_sse_customer_key: Option<String>,
    # [clap (long = "key" , name = "Key" , help = "Object key for which the multipart upload was initiated." , long_help = None)]
    key: String,
    # [clap (long = "copy-source-range" , name = "CopySourceRange" , help = "The range of bytes to copy from the source object. The range value must use the form bytes=first-last, where the first and last are the zero-based byte offsets to copy. For example, bytes=0-9 indicates that you want to copy the first 10 bytes of the source. You can copy a range only if the source object is greater than 5 MB." , long_help = None)]
    copy_source_range: Option<String>,
    # [clap (long = "bucket" , name = "Bucket" , help = "The bucket name." , long_help = None)]
    bucket: String,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected destination bucket owner. If the destination bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    # [clap (long = "request-payer" , name = "RequestPayer" , help = "" , long_help = None)]
    request_payer: Option<String>,
    # [clap (long = "part-number" , name = "PartNumber" , help = "Part number of part being copied. This is a positive integer between 1 and 10,000." , long_help = None)]
    part_number: i32,
    # [clap (long = "sse-customer-key-md5" , name = "SSECustomerKeyMD5" , help = "Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure that the encryption key was transmitted without error." , long_help = None)]
    sse_customer_key_md5: Option<String>,
    # [clap (long = "copy-source-sse-customer-algorithm" , name = "CopySourceSSECustomerAlgorithm" , help = "Specifies the algorithm to use when decrypting the source object (for example, AES256)." , long_help = None)]
    copy_source_sse_customer_algorithm: Option<String>,
    # [clap (long = "sse-customer-key" , name = "SSECustomerKey" , help = "Specifies the customer-provided encryption key for Amazon S3 to use in encrypting data. This value is used to store the object and then it is discarded; Amazon S3 does not store the encryption key. The key must be appropriate for use with the algorithm specified in the <code>x-amz-server-side-encryption-customer-algorithm</code> header. This must be the same encryption key specified in the initiate multipart upload request." , long_help = None)]
    sse_customer_key: Option<String>,
}
impl UploadPartCopyCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .upload_part_copy()
            .set_expected_source_bucket_owner(self.expected_source_bucket_owner.to_owned())
            .set_upload_id(Some(self.upload_id.to_owned()))
            .set_copy_source_if_none_match(self.copy_source_if_none_match.to_owned())
            .set_copy_source_if_unmodified_since(None)
            .set_sse_customer_algorithm(self.sse_customer_algorithm.to_owned())
            .set_copy_source_if_match(self.copy_source_if_match.to_owned())
            .set_copy_source(Some(self.copy_source.to_owned()))
            .set_copy_source_if_modified_since(None)
            .set_copy_source_sse_customer_key_md5(self.copy_source_sse_customer_key_md5.to_owned())
            .set_copy_source_sse_customer_key(self.copy_source_sse_customer_key.to_owned())
            .set_key(Some(self.key.to_owned()))
            .set_copy_source_range(self.copy_source_range.to_owned())
            .set_bucket(Some(self.bucket.to_owned()))
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_request_payer(None)
            .set_part_number(Some(self.part_number))
            .set_sse_customer_key_md5(self.sse_customer_key_md5.to_owned())
            .set_copy_source_sse_customer_algorithm(
                self.copy_source_sse_customer_algorithm.to_owned(),
            )
            .set_sse_customer_key(self.sse_customer_key.to_owned())
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Args, Debug, Clone)]
pub struct PublicAccessBlockConfigurationArgs {
    #[clap(long = "ignore-public-acls")]
    ignore_public_acls: Option<String>,
    #[clap(long = "block-public-policy")]
    block_public_policy: Option<String>,
    #[clap(long = "restrict-public-buckets")]
    restrict_public_buckets: Option<String>,
    #[clap(long = "block-public-acls")]
    block_public_acls: Option<String>,
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct PutPublicAccessBlockCommand {
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    #[clap(flatten)]
    public_access_block_configuration: PublicAccessBlockConfigurationArgs,
    # [clap (long = "content-md5" , name = "ContentMD5" , help = "The MD5 hash of the <code>PutPublicAccessBlock</code> request body." , long_help = None)]
    content_md5: Option<String>,
    # [clap (long = "bucket" , name = "Bucket" , help = "The name of the Amazon S3 bucket whose <code>PublicAccessBlock</code> configuration you want to set." , long_help = None)]
    bucket: String,
    # [clap (long = "checksum-algorithm" , name = "ChecksumAlgorithm" , help = "Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html\">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>." , long_help = None)]
    checksum_algorithm: Option<String>,
}
impl PutPublicAccessBlockCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .put_public_access_block()
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_public_access_block_configuration(None)
            .set_content_md5(self.content_md5.to_owned())
            .set_bucket(Some(self.bucket.to_owned()))
            .set_checksum_algorithm(None)
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct AbortMultipartUploadCommand {
    # [clap (long = "bucket" , name = "Bucket" , help = "The bucket name to which the upload was taking place." , long_help = None)]
    bucket: String,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    # [clap (long = "request-payer" , name = "RequestPayer" , help = "" , long_help = None)]
    request_payer: Option<String>,
    # [clap (long = "key" , name = "Key" , help = "Key of the object for which the multipart upload was initiated." , long_help = None)]
    key: String,
    # [clap (long = "upload-id" , name = "UploadId" , help = "Upload ID that identifies the multipart upload." , long_help = None)]
    upload_id: String,
}
impl AbortMultipartUploadCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .abort_multipart_upload()
            .set_bucket(Some(self.bucket.to_owned()))
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_request_payer(None)
            .set_key(Some(self.key.to_owned()))
            .set_upload_id(Some(self.upload_id.to_owned()))
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct GetPublicAccessBlockCommand {
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    # [clap (long = "bucket" , name = "Bucket" , help = "The name of the Amazon S3 bucket whose <code>PublicAccessBlock</code> configuration you want to retrieve." , long_help = None)]
    bucket: String,
}
impl GetPublicAccessBlockCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .get_public_access_block()
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_bucket(Some(self.bucket.to_owned()))
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct GetBucketAnalyticsConfigurationCommand {
    # [clap (long = "bucket" , name = "Bucket" , help = "The name of the bucket from which an analytics configuration is retrieved." , long_help = None)]
    bucket: String,
    # [clap (long = "id" , name = "Id" , help = "The ID that identifies the analytics configuration." , long_help = None)]
    id: String,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
}
impl GetBucketAnalyticsConfigurationCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .get_bucket_analytics_configuration()
            .set_bucket(Some(self.bucket.to_owned()))
            .set_id(Some(self.id.to_owned()))
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Args, Debug, Clone)]
pub struct BucketLoggingStatusArgs {
    #[clap(long = "logging-enabled")]
    logging_enabled: Option<String>,
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct PutBucketLoggingCommand {
    # [clap (long = "bucket" , name = "Bucket" , help = "The name of the bucket for which to set the logging parameters." , long_help = None)]
    bucket: String,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    # [clap (long = "checksum-algorithm" , name = "ChecksumAlgorithm" , help = "Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html\">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>." , long_help = None)]
    checksum_algorithm: Option<String>,
    #[clap(flatten)]
    bucket_logging_status: BucketLoggingStatusArgs,
    # [clap (long = "content-md5" , name = "ContentMD5" , help = "The MD5 hash of the <code>PutBucketLogging</code> request body." , long_help = None)]
    content_md5: Option<String>,
}
impl PutBucketLoggingCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .put_bucket_logging()
            .set_bucket(Some(self.bucket.to_owned()))
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_checksum_algorithm(None)
            .set_bucket_logging_status(None)
            .set_content_md5(self.content_md5.to_owned())
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Args, Debug, Clone)]
pub struct ObjectLockConfigurationArgs {
    #[clap(long = "object-lock-enabled")]
    object_lock_enabled: Option<String>,
    #[clap(long = "rule")]
    rule: Option<String>,
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct PutObjectLockConfigurationCommand {
    # [clap (long = "content-md5" , name = "ContentMD5" , help = "The MD5 hash for the request body." , long_help = None)]
    content_md5: Option<String>,
    # [clap (long = "bucket" , name = "Bucket" , help = "The bucket whose Object Lock configuration you want to create or replace." , long_help = None)]
    bucket: String,
    # [clap (long = "checksum-algorithm" , name = "ChecksumAlgorithm" , help = "Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html\">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>." , long_help = None)]
    checksum_algorithm: Option<String>,
    #[clap(flatten)]
    object_lock_configuration: ObjectLockConfigurationArgs,
    # [clap (long = "request-payer" , name = "RequestPayer" , help = "" , long_help = None)]
    request_payer: Option<String>,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    # [clap (long = "token" , name = "Token" , help = "A token to allow Object Lock to be enabled for an existing bucket." , long_help = None)]
    token: Option<String>,
}
impl PutObjectLockConfigurationCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .put_object_lock_configuration()
            .set_content_md5(self.content_md5.to_owned())
            .set_bucket(Some(self.bucket.to_owned()))
            .set_checksum_algorithm(None)
            .set_object_lock_configuration(None)
            .set_request_payer(None)
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_token(self.token.to_owned())
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct DeleteBucketEncryptionCommand {
    # [clap (long = "bucket" , name = "Bucket" , help = "The name of the bucket containing the server-side encryption configuration to delete." , long_help = None)]
    bucket: String,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
}
impl DeleteBucketEncryptionCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .delete_bucket_encryption()
            .set_bucket(Some(self.bucket.to_owned()))
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct GetBucketInventoryConfigurationCommand {
    # [clap (long = "id" , name = "Id" , help = "The ID used to identify the inventory configuration." , long_help = None)]
    id: String,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    # [clap (long = "bucket" , name = "Bucket" , help = "The name of the bucket containing the inventory configuration to retrieve." , long_help = None)]
    bucket: String,
}
impl GetBucketInventoryConfigurationCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .get_bucket_inventory_configuration()
            .set_id(Some(self.id.to_owned()))
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_bucket(Some(self.bucket.to_owned()))
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct GetBucketAccelerateConfigurationCommand {
    # [clap (long = "bucket" , name = "Bucket" , help = "The name of the bucket for which the accelerate configuration is retrieved." , long_help = None)]
    bucket: String,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
}
impl GetBucketAccelerateConfigurationCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .get_bucket_accelerate_configuration()
            .set_bucket(Some(self.bucket.to_owned()))
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Args, Debug, Clone)]
pub struct VersioningConfigurationArgs {
    #[clap(long = "status")]
    status: Option<String>,
    #[clap(long = "mfa-delete")]
    mfa_delete: Option<String>,
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct PutBucketVersioningCommand {
    # [clap (long = "content-md5" , name = "ContentMD5" , help = ">The base64-encoded 128-bit MD5 digest of the data. You must use this header as a message integrity check to verify that the request body was not corrupted in transit. For more information, see <a href=\"http://www.ietf.org/rfc/rfc1864.txt\">RFC 1864</a>." , long_help = None)]
    content_md5: Option<String>,
    # [clap (long = "checksum-algorithm" , name = "ChecksumAlgorithm" , help = "Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html\">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>." , long_help = None)]
    checksum_algorithm: Option<String>,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    # [clap (long = "mfa" , name = "MFA" , help = "The concatenation of the authentication device's serial number, a space, and the value that is displayed on your authentication device." , long_help = None)]
    mfa: Option<String>,
    # [clap (long = "bucket" , name = "Bucket" , help = "The bucket name." , long_help = None)]
    bucket: String,
    #[clap(flatten)]
    versioning_configuration: VersioningConfigurationArgs,
}
impl PutBucketVersioningCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .put_bucket_versioning()
            .set_content_md5(self.content_md5.to_owned())
            .set_checksum_algorithm(None)
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_mfa(self.mfa.to_owned())
            .set_bucket(Some(self.bucket.to_owned()))
            .set_versioning_configuration(None)
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Args, Debug, Clone)]
pub struct RequestPaymentConfigurationArgs {
    #[clap(long = "payer")]
    payer: Option<String>,
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct PutBucketRequestPaymentCommand {
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    #[clap(flatten)]
    request_payment_configuration: RequestPaymentConfigurationArgs,
    # [clap (long = "content-md5" , name = "ContentMD5" , help = "The base64-encoded 128-bit MD5 digest of the data. You must use this header as a message integrity check to verify that the request body was not corrupted in transit. For more information, see <a href=\"http://www.ietf.org/rfc/rfc1864.txt\">RFC 1864</a>." , long_help = None)]
    content_md5: Option<String>,
    # [clap (long = "checksum-algorithm" , name = "ChecksumAlgorithm" , help = "Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html\">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>." , long_help = None)]
    checksum_algorithm: Option<String>,
    # [clap (long = "bucket" , name = "Bucket" , help = "The bucket name." , long_help = None)]
    bucket: String,
}
impl PutBucketRequestPaymentCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .put_bucket_request_payment()
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_request_payment_configuration(None)
            .set_content_md5(self.content_md5.to_owned())
            .set_checksum_algorithm(None)
            .set_bucket(Some(self.bucket.to_owned()))
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct DeleteBucketInventoryConfigurationCommand {
    # [clap (long = "bucket" , name = "Bucket" , help = "The name of the bucket containing the inventory configuration to delete." , long_help = None)]
    bucket: String,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    # [clap (long = "id" , name = "Id" , help = "The ID used to identify the inventory configuration." , long_help = None)]
    id: String,
}
impl DeleteBucketInventoryConfigurationCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .delete_bucket_inventory_configuration()
            .set_bucket(Some(self.bucket.to_owned()))
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_id(Some(self.id.to_owned()))
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Args, Debug, Clone)]
pub struct InventoryConfigurationArgs {
    #[clap(long = "schedule")]
    schedule: Option<String>,
    #[clap(long = "included-object-versions")]
    included_object_versions: Option<String>,
    #[clap(long = "id")]
    id: Option<String>,
    #[clap(long = "filter")]
    filter: Option<String>,
    #[clap(long = "destination")]
    destination: Option<String>,
    #[clap(long = "is-enabled")]
    is_enabled: Option<String>,
    #[clap(long = "optional-fields")]
    optional_fields: Option<String>,
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct PutBucketInventoryConfigurationCommand {
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    # [clap (long = "bucket" , name = "Bucket" , help = "The name of the bucket where the inventory configuration will be stored." , long_help = None)]
    bucket: String,
    # [clap (long = "id" , name = "Id" , help = "The ID used to identify the inventory configuration." , long_help = None)]
    id: String,
    #[clap(flatten)]
    inventory_configuration: InventoryConfigurationArgs,
}
impl PutBucketInventoryConfigurationCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .put_bucket_inventory_configuration()
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_bucket(Some(self.bucket.to_owned()))
            .set_id(Some(self.id.to_owned()))
            .set_inventory_configuration(None)
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct UploadPartCommand {
    # [clap (long = "checksum-sha256" , name = "ChecksumSHA256" , help = "This header can be used as a data integrity check to verify that the data received is the same data that was originally sent. This header specifies the base64-encoded, 256-bit SHA-256 digest of the object. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html\">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>." , long_help = None)]
    checksum_sha256: Option<String>,
    # [clap (long = "checksum-sha1" , name = "ChecksumSHA1" , help = "This header can be used as a data integrity check to verify that the data received is the same data that was originally sent. This header specifies the base64-encoded, 160-bit SHA-1 digest of the object. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html\">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>." , long_help = None)]
    checksum_sha1: Option<String>,
    # [clap (long = "upload-id" , name = "UploadId" , help = "Upload ID identifying the multipart upload whose part is being uploaded." , long_help = None)]
    upload_id: String,
    # [clap (long = "sse-customer-algorithm" , name = "SSECustomerAlgorithm" , help = "Specifies the algorithm to use to when encrypting the object (for example, AES256)." , long_help = None)]
    sse_customer_algorithm: Option<String>,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    # [clap (long = "key" , name = "Key" , help = "Object key for which the multipart upload was initiated." , long_help = None)]
    key: String,
    # [clap (long = "checksum-crc32-c" , name = "ChecksumCRC32C" , help = "This header can be used as a data integrity check to verify that the data received is the same data that was originally sent. This header specifies the base64-encoded, 32-bit CRC32C checksum of the object. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html\">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>." , long_help = None)]
    checksum_crc32_c: Option<String>,
    # [clap (long = "part-number" , name = "PartNumber" , help = "Part number of part being uploaded. This is a positive integer between 1 and 10,000." , long_help = None)]
    part_number: i32,
    # [clap (long = "content-md5" , name = "ContentMD5" , help = "The base64-encoded 128-bit MD5 digest of the part data. This parameter is auto-populated when using the command from the CLI. This parameter is required if object lock parameters are specified." , long_help = None)]
    content_md5: Option<String>,
    # [clap (long = "bucket" , name = "Bucket" , help = "The name of the bucket to which the multipart upload was initiated." , long_help = None)]
    bucket: String,
    # [clap (long = "sse-customer-key" , name = "SSECustomerKey" , help = "Specifies the customer-provided encryption key for Amazon S3 to use in encrypting data. This value is used to store the object and then it is discarded; Amazon S3 does not store the encryption key. The key must be appropriate for use with the algorithm specified in the <code>x-amz-server-side-encryption-customer-algorithm header</code>. This must be the same encryption key specified in the initiate multipart upload request." , long_help = None)]
    sse_customer_key: Option<String>,
    # [clap (long = "checksum-crc32" , name = "ChecksumCRC32" , help = "This header can be used as a data integrity check to verify that the data received is the same data that was originally sent. This header specifies the base64-encoded, 32-bit CRC32 checksum of the object. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html\">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>." , long_help = None)]
    checksum_crc32: Option<String>,
    # [clap (long = "body" , name = "Body" , help = "Object data." , long_help = None)]
    body: Option<Vec<u8>>,
    # [clap (long = "sse-customer-key-md5" , name = "SSECustomerKeyMD5" , help = "Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure that the encryption key was transmitted without error." , long_help = None)]
    sse_customer_key_md5: Option<String>,
    # [clap (long = "checksum-algorithm" , name = "ChecksumAlgorithm" , help = "Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html\">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>." , long_help = None)]
    checksum_algorithm: Option<String>,
    # [clap (long = "request-payer" , name = "RequestPayer" , help = "" , long_help = None)]
    request_payer: Option<String>,
    # [clap (long = "content-length" , name = "ContentLength" , help = "Size of the body in bytes. This parameter is useful when the size of the body cannot be determined automatically." , long_help = None)]
    content_length: Option<i64>,
}
impl UploadPartCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .upload_part()
            .set_checksum_sha256(self.checksum_sha256.to_owned())
            .set_checksum_sha1(self.checksum_sha1.to_owned())
            .set_upload_id(Some(self.upload_id.to_owned()))
            .set_sse_customer_algorithm(self.sse_customer_algorithm.to_owned())
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_key(Some(self.key.to_owned()))
            .set_checksum_crc32_c(self.checksum_crc32_c.to_owned())
            .set_part_number(Some(self.part_number))
            .set_content_md5(self.content_md5.to_owned())
            .set_bucket(Some(self.bucket.to_owned()))
            .set_sse_customer_key(self.sse_customer_key.to_owned())
            .set_checksum_crc32(self.checksum_crc32.to_owned())
            .set_body(None)
            .set_sse_customer_key_md5(self.sse_customer_key_md5.to_owned())
            .set_checksum_algorithm(None)
            .set_request_payer(None)
            .set_content_length(self.content_length)
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct GetBucketRequestPaymentCommand {
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    # [clap (long = "bucket" , name = "Bucket" , help = "The name of the bucket for which to get the payment request configuration" , long_help = None)]
    bucket: String,
}
impl GetBucketRequestPaymentCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .get_bucket_request_payment()
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_bucket(Some(self.bucket.to_owned()))
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct GetBucketOwnershipControlsCommand {
    # [clap (long = "bucket" , name = "Bucket" , help = "The name of the Amazon S3 bucket whose <code>OwnershipControls</code> you want to retrieve." , long_help = None)]
    bucket: String,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
}
impl GetBucketOwnershipControlsCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .get_bucket_ownership_controls()
            .set_bucket(Some(self.bucket.to_owned()))
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Args, Debug, Clone)]
pub struct IntelligentTieringConfigurationArgs {
    #[clap(long = "filter")]
    filter: Option<String>,
    #[clap(long = "status")]
    status: Option<String>,
    #[clap(long = "id")]
    id: Option<String>,
    #[clap(long = "tierings")]
    tierings: Option<String>,
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct PutBucketIntelligentTieringConfigurationCommand {
    # [clap (long = "bucket" , name = "Bucket" , help = "The name of the Amazon S3 bucket whose configuration you want to modify or retrieve." , long_help = None)]
    bucket: String,
    # [clap (long = "id" , name = "Id" , help = "The ID used to identify the S3 Intelligent-Tiering configuration." , long_help = None)]
    id: String,
    #[clap(flatten)]
    intelligent_tiering_configuration: IntelligentTieringConfigurationArgs,
}
impl PutBucketIntelligentTieringConfigurationCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .put_bucket_intelligent_tiering_configuration()
            .set_bucket(Some(self.bucket.to_owned()))
            .set_id(Some(self.id.to_owned()))
            .set_intelligent_tiering_configuration(None)
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct HeadObjectCommand {
    # [clap (long = "if-none-match" , name = "IfNoneMatch" , help = "Return the object only if its entity tag (ETag) is different from the one specified; otherwise, return a 304 (not modified) error." , long_help = None)]
    if_none_match: Option<String>,
    # [clap (long = "checksum-mode" , name = "ChecksumMode" , help = "To retrieve the checksum, this parameter must be enabled." , long_help = None)]
    checksum_mode: Option<String>,
    # [clap (long = "key" , name = "Key" , help = "The object key." , long_help = None)]
    key: String,
    # [clap (long = "bucket" , name = "Bucket" , help = "The name of the bucket containing the object." , long_help = None)]
    bucket: String,
    # [clap (long = "sse-customer-algorithm" , name = "SSECustomerAlgorithm" , help = "Specifies the algorithm to use to when encrypting the object (for example, AES256)." , long_help = None)]
    sse_customer_algorithm: Option<String>,
    # [clap (long = "request-payer" , name = "RequestPayer" , help = "" , long_help = None)]
    request_payer: Option<String>,
    # [clap (long = "if-match" , name = "IfMatch" , help = "Return the object only if its entity tag (ETag) is the same as the one specified; otherwise, return a 412 (precondition failed) error." , long_help = None)]
    if_match: Option<String>,
    # [clap (long = "range" , name = "Range" , help = "Because <code>HeadObject</code> returns only the metadata for an object, this parameter has no effect." , long_help = None)]
    range: Option<String>,
    # [clap (long = "if-modified-since" , name = "IfModifiedSince" , help = "Return the object only if it has been modified since the specified time; otherwise, return a 304 (not modified) error." , long_help = None)]
    if_modified_since: Option<String>,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    # [clap (long = "sse-customer-key" , name = "SSECustomerKey" , help = "Specifies the customer-provided encryption key for Amazon S3 to use in encrypting data. This value is used to store the object and then it is discarded; Amazon S3 does not store the encryption key. The key must be appropriate for use with the algorithm specified in the <code>x-amz-server-side-encryption-customer-algorithm</code> header." , long_help = None)]
    sse_customer_key: Option<String>,
    # [clap (long = "sse-customer-key-md5" , name = "SSECustomerKeyMD5" , help = "Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure that the encryption key was transmitted without error." , long_help = None)]
    sse_customer_key_md5: Option<String>,
    # [clap (long = "version-id" , name = "VersionId" , help = "VersionId used to reference a specific version of the object." , long_help = None)]
    version_id: Option<String>,
    # [clap (long = "if-unmodified-since" , name = "IfUnmodifiedSince" , help = "Return the object only if it has not been modified since the specified time; otherwise, return a 412 (precondition failed) error." , long_help = None)]
    if_unmodified_since: Option<String>,
    # [clap (long = "part-number" , name = "PartNumber" , help = "Part number of the object being read. This is a positive integer between 1 and 10,000. Effectively performs a 'ranged' HEAD request for the part specified. Useful querying about the size of the part and the number of parts in this object." , long_help = None)]
    part_number: Option<i32>,
}
impl HeadObjectCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .head_object()
            .set_if_none_match(self.if_none_match.to_owned())
            .set_checksum_mode(None)
            .set_key(Some(self.key.to_owned()))
            .set_bucket(Some(self.bucket.to_owned()))
            .set_sse_customer_algorithm(self.sse_customer_algorithm.to_owned())
            .set_request_payer(None)
            .set_if_match(self.if_match.to_owned())
            .set_range(self.range.to_owned())
            .set_if_modified_since(None)
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_sse_customer_key(self.sse_customer_key.to_owned())
            .set_sse_customer_key_md5(self.sse_customer_key_md5.to_owned())
            .set_version_id(self.version_id.to_owned())
            .set_if_unmodified_since(None)
            .set_part_number(self.part_number)
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Args, Debug, Clone)]
pub struct MetricsConfigurationArgs {
    #[clap(long = "id")]
    id: Option<String>,
    #[clap(long = "filter")]
    filter: Option<String>,
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct PutBucketMetricsConfigurationCommand {
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    #[clap(flatten)]
    metrics_configuration: MetricsConfigurationArgs,
    # [clap (long = "bucket" , name = "Bucket" , help = "The name of the bucket for which the metrics configuration is set." , long_help = None)]
    bucket: String,
    # [clap (long = "id" , name = "Id" , help = "The ID used to identify the metrics configuration." , long_help = None)]
    id: String,
}
impl PutBucketMetricsConfigurationCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .put_bucket_metrics_configuration()
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_metrics_configuration(None)
            .set_bucket(Some(self.bucket.to_owned()))
            .set_id(Some(self.id.to_owned()))
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Args, Debug, Clone)]
pub struct RestoreRequestArgs {
    #[clap(long = "output-location")]
    output_location: Option<String>,
    #[clap(long = "select-parameters")]
    select_parameters: Option<String>,
    #[clap(long = "type")]
    r#type: Option<String>,
    #[clap(long = "tier")]
    tier: Option<String>,
    #[clap(long = "days")]
    days: Option<String>,
    #[clap(long = "description")]
    description: Option<String>,
    #[clap(long = "glacier-job-parameters")]
    glacier_job_parameters: Option<String>,
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct RestoreObjectCommand {
    # [clap (long = "checksum-algorithm" , name = "ChecksumAlgorithm" , help = "Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html\">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>." , long_help = None)]
    checksum_algorithm: Option<String>,
    # [clap (long = "request-payer" , name = "RequestPayer" , help = "" , long_help = None)]
    request_payer: Option<String>,
    # [clap (long = "version-id" , name = "VersionId" , help = "VersionId used to reference a specific version of the object." , long_help = None)]
    version_id: Option<String>,
    #[clap(flatten)]
    restore_request: RestoreRequestArgs,
    # [clap (long = "key" , name = "Key" , help = "Object key for which the action was initiated." , long_help = None)]
    key: String,
    # [clap (long = "bucket" , name = "Bucket" , help = "The bucket name containing the object to restore." , long_help = None)]
    bucket: String,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
}
impl RestoreObjectCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .restore_object()
            .set_checksum_algorithm(None)
            .set_request_payer(None)
            .set_version_id(self.version_id.to_owned())
            .set_restore_request(None)
            .set_key(Some(self.key.to_owned()))
            .set_bucket(Some(self.bucket.to_owned()))
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct CopyObjectCommand {
    # [clap (long = "tagging-directive" , name = "TaggingDirective" , help = "Specifies whether the object tag-set are copied from the source object or replaced with tag-set provided in the request." , long_help = None)]
    tagging_directive: Option<String>,
    # [clap (long = "grant-write-acp" , name = "GrantWriteACP" , help = "Allows grantee to write the ACL for the applicable object." , long_help = None)]
    grant_write_acp: Option<String>,
    # [clap (long = "sse-customer-key-md5" , name = "SSECustomerKeyMD5" , help = "Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure that the encryption key was transmitted without error." , long_help = None)]
    sse_customer_key_md5: Option<String>,
    # [clap (long = "acl" , name = "ACL" , help = "The canned ACL to apply to the object." , long_help = None)]
    acl: Option<String>,
    # [clap (long = "expires" , name = "Expires" , help = "The date and time at which the object is no longer cacheable." , long_help = None)]
    expires: Option<String>,
    # [clap (long = "copy-source" , name = "CopySource" , help = "Specifies the source object for the copy operation. You specify the value in one of two formats, depending on whether you want to access the source object through an <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/userguide/access-points.html\">access point</a>:" , long_help = None)]
    copy_source: String,
    # [clap (long = "ssekms-key-id" , name = "SSEKMSKeyId" , help = "Specifies the Amazon Web Services KMS key ID to use for object encryption. All GET and PUT requests for an object protected by Amazon Web Services KMS will fail if not made via SSL or using SigV4. For information about configuring using any of the officially supported Amazon Web Services SDKs and Amazon Web Services CLI, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/dev/UsingAWSSDK.html#specify-signature-version\">Specifying the Signature Version in Request Authentication</a> in the <i>Amazon S3 User Guide</i>." , long_help = None)]
    ssekms_key_id: Option<String>,
    # [clap (long = "object-lock-retain-until-date" , name = "ObjectLockRetainUntilDate" , help = "The date and time when you want the copied object's Object Lock to expire." , long_help = None)]
    object_lock_retain_until_date: Option<String>,
    # [clap (long = "copy-source-if-none-match" , name = "CopySourceIfNoneMatch" , help = "Copies the object if its entity tag (ETag) is different than the specified ETag." , long_help = None)]
    copy_source_if_none_match: Option<String>,
    # [clap (long = "copy-source-if-unmodified-since" , name = "CopySourceIfUnmodifiedSince" , help = "Copies the object if it hasn't been modified since the specified time." , long_help = None)]
    copy_source_if_unmodified_since: Option<String>,
    # [clap (long = "expected-source-bucket-owner" , name = "ExpectedSourceBucketOwner" , help = "The account ID of the expected source bucket owner. If the source bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_source_bucket_owner: Option<String>,
    # [clap (long = "content-disposition" , name = "ContentDisposition" , help = "Specifies presentational information for the object." , long_help = None)]
    content_disposition: Option<String>,
    # [clap (long = "storage-class" , name = "StorageClass" , help = "By default, Amazon S3 uses the STANDARD Storage Class to store newly created objects. The STANDARD storage class provides high durability and high availability. Depending on performance needs, you can specify a different Storage Class. Amazon S3 on Outposts only uses the OUTPOSTS Storage Class. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/dev/storage-class-intro.html\">Storage Classes</a> in the <i>Amazon S3 User Guide</i>." , long_help = None)]
    storage_class: Option<String>,
    # [clap (long = "bucket" , name = "Bucket" , help = "The name of the destination bucket." , long_help = None)]
    bucket: String,
    # [clap (long = "metadata" , name = "Metadata" , help = "A map of metadata to store with the object in S3." , long_help = None)]
    metadata: Option<String>,
    # [clap (long = "sse-customer-key" , name = "SSECustomerKey" , help = "Specifies the customer-provided encryption key for Amazon S3 to use in encrypting data. This value is used to store the object and then it is discarded; Amazon S3 does not store the encryption key. The key must be appropriate for use with the algorithm specified in the <code>x-amz-server-side-encryption-customer-algorithm</code> header." , long_help = None)]
    sse_customer_key: Option<String>,
    # [clap (long = "copy-source-if-modified-since" , name = "CopySourceIfModifiedSince" , help = "Copies the object if it has been modified since the specified time." , long_help = None)]
    copy_source_if_modified_since: Option<String>,
    # [clap (long = "sse-customer-algorithm" , name = "SSECustomerAlgorithm" , help = "Specifies the algorithm to use to when encrypting the object (for example, AES256)." , long_help = None)]
    sse_customer_algorithm: Option<String>,
    # [clap (long = "grant-read-acp" , name = "GrantReadACP" , help = "Allows grantee to read the object ACL." , long_help = None)]
    grant_read_acp: Option<String>,
    # [clap (long = "tagging" , name = "Tagging" , help = "The tag-set for the object destination object this value must be used in conjunction with the <code>TaggingDirective</code>. The tag-set must be encoded as URL Query parameters." , long_help = None)]
    tagging: Option<String>,
    # [clap (long = "copy-source-sse-customer-key" , name = "CopySourceSSECustomerKey" , help = "Specifies the customer-provided encryption key for Amazon S3 to use to decrypt the source object. The encryption key provided in this header must be one that was used when the source object was created." , long_help = None)]
    copy_source_sse_customer_key: Option<String>,
    # [clap (long = "copy-source-sse-customer-algorithm" , name = "CopySourceSSECustomerAlgorithm" , help = "Specifies the algorithm to use when decrypting the source object (for example, AES256)." , long_help = None)]
    copy_source_sse_customer_algorithm: Option<String>,
    # [clap (long = "grant-read" , name = "GrantRead" , help = "Allows grantee to read the object data and its metadata." , long_help = None)]
    grant_read: Option<String>,
    # [clap (long = "cache-control" , name = "CacheControl" , help = "Specifies caching behavior along the request/reply chain." , long_help = None)]
    cache_control: Option<String>,
    # [clap (long = "content-language" , name = "ContentLanguage" , help = "The language the content is in." , long_help = None)]
    content_language: Option<String>,
    # [clap (long = "key" , name = "Key" , help = "The key of the destination object." , long_help = None)]
    key: String,
    # [clap (long = "ssekms-encryption-context" , name = "SSEKMSEncryptionContext" , help = "Specifies the Amazon Web Services KMS Encryption Context to use for object encryption. The value of this header is a base64-encoded UTF-8 string holding JSON with the encryption context key-value pairs." , long_help = None)]
    ssekms_encryption_context: Option<String>,
    # [clap (long = "website-redirect-location" , name = "WebsiteRedirectLocation" , help = "If the bucket is configured as a website, redirects requests for this object to another object in the same bucket or to an external URL. Amazon S3 stores the value of this header in the object metadata." , long_help = None)]
    website_redirect_location: Option<String>,
    # [clap (long = "checksum-algorithm" , name = "ChecksumAlgorithm" , help = "Indicates the algorithm you want Amazon S3 to use to create the checksum for the object. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html\">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>." , long_help = None)]
    checksum_algorithm: Option<String>,
    # [clap (long = "copy-source-sse-customer-key-md5" , name = "CopySourceSSECustomerKeyMD5" , help = "Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure that the encryption key was transmitted without error." , long_help = None)]
    copy_source_sse_customer_key_md5: Option<String>,
    # [clap (long = "grant-full-control" , name = "GrantFullControl" , help = "Gives the grantee READ, READ_ACP, and WRITE_ACP permissions on the object." , long_help = None)]
    grant_full_control: Option<String>,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected destination bucket owner. If the destination bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    # [clap (long = "content-type" , name = "ContentType" , help = "A standard MIME type describing the format of the object data." , long_help = None)]
    content_type: Option<String>,
    # [clap (long = "object-lock-legal-hold-status" , name = "ObjectLockLegalHoldStatus" , help = "Specifies whether you want to apply a legal hold to the copied object." , long_help = None)]
    object_lock_legal_hold_status: Option<String>,
    # [clap (long = "request-payer" , name = "RequestPayer" , help = "" , long_help = None)]
    request_payer: Option<String>,
    # [clap (long = "content-encoding" , name = "ContentEncoding" , help = "Specifies what content encodings have been applied to the object and thus what decoding mechanisms must be applied to obtain the media-type referenced by the Content-Type header field." , long_help = None)]
    content_encoding: Option<String>,
    # [clap (long = "copy-source-if-match" , name = "CopySourceIfMatch" , help = "Copies the object if its entity tag (ETag) matches the specified tag." , long_help = None)]
    copy_source_if_match: Option<String>,
    # [clap (long = "server-side-encryption" , name = "ServerSideEncryption" , help = "The server-side encryption algorithm used when storing this object in Amazon S3 (for example, AES256, aws:kms)." , long_help = None)]
    server_side_encryption: Option<String>,
    # [clap (long = "metadata-directive" , name = "MetadataDirective" , help = "Specifies whether the metadata is copied from the source object or replaced with metadata provided in the request." , long_help = None)]
    metadata_directive: Option<String>,
    # [clap (long = "bucket-key-enabled" , name = "BucketKeyEnabled" , help = "Specifies whether Amazon S3 should use an S3 Bucket Key for object encryption with server-side encryption using AWS KMS (SSE-KMS). Setting this header to <code>true</code> causes Amazon S3 to use an S3 Bucket Key for object encryption with SSE-KMS." , long_help = None)]
    bucket_key_enabled: Option<bool>,
    # [clap (long = "object-lock-mode" , name = "ObjectLockMode" , help = "The Object Lock mode that you want to apply to the copied object." , long_help = None)]
    object_lock_mode: Option<String>,
}
impl CopyObjectCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .copy_object()
            .set_tagging_directive(None)
            .set_grant_write_acp(self.grant_write_acp.to_owned())
            .set_sse_customer_key_md5(self.sse_customer_key_md5.to_owned())
            .set_acl(None)
            .set_expires(None)
            .set_copy_source(Some(self.copy_source.to_owned()))
            .set_ssekms_key_id(self.ssekms_key_id.to_owned())
            .set_object_lock_retain_until_date(None)
            .set_copy_source_if_none_match(self.copy_source_if_none_match.to_owned())
            .set_copy_source_if_unmodified_since(None)
            .set_expected_source_bucket_owner(self.expected_source_bucket_owner.to_owned())
            .set_content_disposition(self.content_disposition.to_owned())
            .set_storage_class(None)
            .set_bucket(Some(self.bucket.to_owned()))
            .set_metadata(None)
            .set_sse_customer_key(self.sse_customer_key.to_owned())
            .set_copy_source_if_modified_since(None)
            .set_sse_customer_algorithm(self.sse_customer_algorithm.to_owned())
            .set_grant_read_acp(self.grant_read_acp.to_owned())
            .set_tagging(self.tagging.to_owned())
            .set_copy_source_sse_customer_key(self.copy_source_sse_customer_key.to_owned())
            .set_copy_source_sse_customer_algorithm(
                self.copy_source_sse_customer_algorithm.to_owned(),
            )
            .set_grant_read(self.grant_read.to_owned())
            .set_cache_control(self.cache_control.to_owned())
            .set_content_language(self.content_language.to_owned())
            .set_key(Some(self.key.to_owned()))
            .set_ssekms_encryption_context(self.ssekms_encryption_context.to_owned())
            .set_website_redirect_location(self.website_redirect_location.to_owned())
            .set_checksum_algorithm(None)
            .set_copy_source_sse_customer_key_md5(self.copy_source_sse_customer_key_md5.to_owned())
            .set_grant_full_control(self.grant_full_control.to_owned())
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_content_type(self.content_type.to_owned())
            .set_object_lock_legal_hold_status(None)
            .set_request_payer(None)
            .set_content_encoding(self.content_encoding.to_owned())
            .set_copy_source_if_match(self.copy_source_if_match.to_owned())
            .set_server_side_encryption(None)
            .set_metadata_directive(None)
            .set_bucket_key_enabled(self.bucket_key_enabled)
            .set_object_lock_mode(None)
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct PutBucketPolicyCommand {
    # [clap (long = "bucket" , name = "Bucket" , help = "The name of the bucket." , long_help = None)]
    bucket: String,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    # [clap (long = "confirm-remove-self-bucket-access" , name = "ConfirmRemoveSelfBucketAccess" , help = "Set this parameter to true to confirm that you want to remove your permissions to change this bucket policy in the future." , long_help = None)]
    confirm_remove_self_bucket_access: Option<bool>,
    # [clap (long = "content-md5" , name = "ContentMD5" , help = "The MD5 hash of the request body." , long_help = None)]
    content_md5: Option<String>,
    # [clap (long = "checksum-algorithm" , name = "ChecksumAlgorithm" , help = "Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html\">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>." , long_help = None)]
    checksum_algorithm: Option<String>,
    # [clap (long = "policy" , name = "Policy" , help = "The bucket policy as a JSON document." , long_help = None)]
    policy: String,
}
impl PutBucketPolicyCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .put_bucket_policy()
            .set_bucket(Some(self.bucket.to_owned()))
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_confirm_remove_self_bucket_access(self.confirm_remove_self_bucket_access)
            .set_content_md5(self.content_md5.to_owned())
            .set_checksum_algorithm(None)
            .set_policy(Some(self.policy.to_owned()))
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct GetObjectLegalHoldCommand {
    # [clap (long = "bucket" , name = "Bucket" , help = "The bucket name containing the object whose legal hold status you want to retrieve." , long_help = None)]
    bucket: String,
    # [clap (long = "request-payer" , name = "RequestPayer" , help = "" , long_help = None)]
    request_payer: Option<String>,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    # [clap (long = "version-id" , name = "VersionId" , help = "The version ID of the object whose legal hold status you want to retrieve." , long_help = None)]
    version_id: Option<String>,
    # [clap (long = "key" , name = "Key" , help = "The key name for the object whose legal hold status you want to retrieve." , long_help = None)]
    key: String,
}
impl GetObjectLegalHoldCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .get_object_legal_hold()
            .set_bucket(Some(self.bucket.to_owned()))
            .set_request_payer(None)
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_version_id(self.version_id.to_owned())
            .set_key(Some(self.key.to_owned()))
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Args, Debug, Clone)]
pub struct ReplicationConfigurationArgs {
    #[clap(long = "rules")]
    rules: Option<String>,
    #[clap(long = "role")]
    role: Option<String>,
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct PutBucketReplicationCommand {
    # [clap (long = "bucket" , name = "Bucket" , help = "The name of the bucket" , long_help = None)]
    bucket: String,
    # [clap (long = "token" , name = "Token" , help = "A token to allow Object Lock to be enabled for an existing bucket." , long_help = None)]
    token: Option<String>,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    # [clap (long = "content-md5" , name = "ContentMD5" , help = "The base64-encoded 128-bit MD5 digest of the data. You must use this header as a message integrity check to verify that the request body was not corrupted in transit. For more information, see <a href=\"http://www.ietf.org/rfc/rfc1864.txt\">RFC 1864</a>." , long_help = None)]
    content_md5: Option<String>,
    #[clap(flatten)]
    replication_configuration: ReplicationConfigurationArgs,
    # [clap (long = "checksum-algorithm" , name = "ChecksumAlgorithm" , help = "Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html\">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>." , long_help = None)]
    checksum_algorithm: Option<String>,
}
impl PutBucketReplicationCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .put_bucket_replication()
            .set_bucket(Some(self.bucket.to_owned()))
            .set_token(self.token.to_owned())
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_content_md5(self.content_md5.to_owned())
            .set_replication_configuration(None)
            .set_checksum_algorithm(None)
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct DeleteBucketCorsCommand {
    # [clap (long = "bucket" , name = "Bucket" , help = "Specifies the bucket whose <code>cors</code> configuration is being deleted." , long_help = None)]
    bucket: String,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
}
impl DeleteBucketCorsCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .delete_bucket_cors()
            .set_bucket(Some(self.bucket.to_owned()))
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct GetBucketLocationCommand {
    # [clap (long = "bucket" , name = "Bucket" , help = "The name of the bucket for which to get the location." , long_help = None)]
    bucket: String,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
}
impl GetBucketLocationCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .get_bucket_location()
            .set_bucket(Some(self.bucket.to_owned()))
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct DeleteBucketAnalyticsConfigurationCommand {
    # [clap (long = "id" , name = "Id" , help = "The ID that identifies the analytics configuration." , long_help = None)]
    id: String,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    # [clap (long = "bucket" , name = "Bucket" , help = "The name of the bucket from which an analytics configuration is deleted." , long_help = None)]
    bucket: String,
}
impl DeleteBucketAnalyticsConfigurationCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .delete_bucket_analytics_configuration()
            .set_id(Some(self.id.to_owned()))
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_bucket(Some(self.bucket.to_owned()))
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct PutBucketTaggingCommand {
    # [clap (long = "bucket" , name = "Bucket" , help = "The bucket name." , long_help = None)]
    bucket: String,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    # [clap (long = "checksum-algorithm" , name = "ChecksumAlgorithm" , help = "Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html\">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>." , long_help = None)]
    checksum_algorithm: Option<String>,
    #[clap(flatten)]
    tagging: TaggingArgs,
    # [clap (long = "content-md5" , name = "ContentMD5" , help = "The base64-encoded 128-bit MD5 digest of the data. You must use this header as a message integrity check to verify that the request body was not corrupted in transit. For more information, see <a href=\"http://www.ietf.org/rfc/rfc1864.txt\">RFC 1864</a>." , long_help = None)]
    content_md5: Option<String>,
}
impl PutBucketTaggingCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .put_bucket_tagging()
            .set_bucket(Some(self.bucket.to_owned()))
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_checksum_algorithm(None)
            .set_tagging(None)
            .set_content_md5(self.content_md5.to_owned())
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct GetBucketLifecycleConfigurationCommand {
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    # [clap (long = "bucket" , name = "Bucket" , help = "The name of the bucket for which to get the lifecycle information." , long_help = None)]
    bucket: String,
}
impl GetBucketLifecycleConfigurationCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .get_bucket_lifecycle_configuration()
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_bucket(Some(self.bucket.to_owned()))
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct HeadBucketCommand {
    # [clap (long = "bucket" , name = "Bucket" , help = "The bucket name." , long_help = None)]
    bucket: String,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
}
impl HeadBucketCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .head_bucket()
            .set_bucket(Some(self.bucket.to_owned()))
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Args, Debug, Clone)]
pub struct AccessControlPolicyArgs {
    #[clap(long = "owner")]
    owner: Option<String>,
    #[clap(long = "grants")]
    grants: Option<String>,
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct PutObjectAclCommand {
    # [clap (long = "request-payer" , name = "RequestPayer" , help = "" , long_help = None)]
    request_payer: Option<String>,
    # [clap (long = "version-id" , name = "VersionId" , help = "VersionId used to reference a specific version of the object." , long_help = None)]
    version_id: Option<String>,
    # [clap (long = "bucket" , name = "Bucket" , help = "The bucket name that contains the object to which you want to attach the ACL." , long_help = None)]
    bucket: String,
    # [clap (long = "grant-full-control" , name = "GrantFullControl" , help = "Allows grantee the read, write, read ACP, and write ACP permissions on the bucket." , long_help = None)]
    grant_full_control: Option<String>,
    #[clap(flatten)]
    access_control_policy: AccessControlPolicyArgs,
    # [clap (long = "checksum-algorithm" , name = "ChecksumAlgorithm" , help = "Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html\">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>." , long_help = None)]
    checksum_algorithm: Option<String>,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    # [clap (long = "content-md5" , name = "ContentMD5" , help = "The base64-encoded 128-bit MD5 digest of the data. This header must be used as a message integrity check to verify that the request body was not corrupted in transit. For more information, go to <a href=\"http://www.ietf.org/rfc/rfc1864.txt\">RFC 1864.></a>" , long_help = None)]
    content_md5: Option<String>,
    # [clap (long = "grant-write" , name = "GrantWrite" , help = "Allows grantee to create new objects in the bucket." , long_help = None)]
    grant_write: Option<String>,
    # [clap (long = "acl" , name = "ACL" , help = "The canned ACL to apply to the object. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/dev/acl-overview.html#CannedACL\">Canned ACL</a>." , long_help = None)]
    acl: Option<String>,
    # [clap (long = "grant-write-acp" , name = "GrantWriteACP" , help = "Allows grantee to write the ACL for the applicable bucket." , long_help = None)]
    grant_write_acp: Option<String>,
    # [clap (long = "key" , name = "Key" , help = "Key for which the PUT action was initiated." , long_help = None)]
    key: String,
    # [clap (long = "grant-read" , name = "GrantRead" , help = "Allows grantee to list the objects in the bucket." , long_help = None)]
    grant_read: Option<String>,
    # [clap (long = "grant-read-acp" , name = "GrantReadACP" , help = "Allows grantee to read the bucket ACL." , long_help = None)]
    grant_read_acp: Option<String>,
}
impl PutObjectAclCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .put_object_acl()
            .set_request_payer(None)
            .set_version_id(self.version_id.to_owned())
            .set_bucket(Some(self.bucket.to_owned()))
            .set_grant_full_control(self.grant_full_control.to_owned())
            .set_access_control_policy(None)
            .set_checksum_algorithm(None)
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_content_md5(self.content_md5.to_owned())
            .set_grant_write(self.grant_write.to_owned())
            .set_acl(None)
            .set_grant_write_acp(self.grant_write_acp.to_owned())
            .set_key(Some(self.key.to_owned()))
            .set_grant_read(self.grant_read.to_owned())
            .set_grant_read_acp(self.grant_read_acp.to_owned())
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct ListBucketsCommand {}
impl ListBucketsCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3.list_buckets().send().await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct PutObjectCommand {
    # [clap (long = "checksum-sha256" , name = "ChecksumSHA256" , help = "This header can be used as a data integrity check to verify that the data received is the same data that was originally sent. This header specifies the base64-encoded, 256-bit SHA-256 digest of the object. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html\">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>." , long_help = None)]
    checksum_sha256: Option<String>,
    # [clap (long = "content-encoding" , name = "ContentEncoding" , help = "Specifies what content encodings have been applied to the object and thus what decoding mechanisms must be applied to obtain the media-type referenced by the Content-Type header field. For more information, see <a href=\"http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.11\">http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.11</a>." , long_help = None)]
    content_encoding: Option<String>,
    # [clap (long = "checksum-crc32" , name = "ChecksumCRC32" , help = "This header can be used as a data integrity check to verify that the data received is the same data that was originally sent. This header specifies the base64-encoded, 32-bit CRC32 checksum of the object. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html\">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>." , long_help = None)]
    checksum_crc32: Option<String>,
    # [clap (long = "ssekms-key-id" , name = "SSEKMSKeyId" , help = "If <code>x-amz-server-side-encryption</code> is present and has the value of <code>aws:kms</code>, this header specifies the ID of the Amazon Web Services Key Management Service (Amazon Web Services KMS) symmetrical customer managed key that was used for the object. If you specify <code>x-amz-server-side-encryption:aws:kms</code>, but do not provide<code> x-amz-server-side-encryption-aws-kms-key-id</code>, Amazon S3 uses the Amazon Web Services managed key to protect the data. If the KMS key does not exist in the same account issuing the command, you must use the full ARN and not just the ID." , long_help = None)]
    ssekms_key_id: Option<String>,
    # [clap (long = "expires" , name = "Expires" , help = "The date and time at which the object is no longer cacheable. For more information, see <a href=\"http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.21\">http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.21</a>." , long_help = None)]
    expires: Option<String>,
    # [clap (long = "bucket-key-enabled" , name = "BucketKeyEnabled" , help = "Specifies whether Amazon S3 should use an S3 Bucket Key for object encryption with server-side encryption using AWS KMS (SSE-KMS). Setting this header to <code>true</code> causes Amazon S3 to use an S3 Bucket Key for object encryption with SSE-KMS." , long_help = None)]
    bucket_key_enabled: Option<bool>,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    # [clap (long = "content-type" , name = "ContentType" , help = "A standard MIME type describing the format of the contents. For more information, see <a href=\"http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.17\">http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.17</a>." , long_help = None)]
    content_type: Option<String>,
    # [clap (long = "key" , name = "Key" , help = "Object key for which the PUT action was initiated." , long_help = None)]
    key: String,
    # [clap (long = "grant-read-acp" , name = "GrantReadACP" , help = "Allows grantee to read the object ACL." , long_help = None)]
    grant_read_acp: Option<String>,
    # [clap (long = "sse-customer-key" , name = "SSECustomerKey" , help = "Specifies the customer-provided encryption key for Amazon S3 to use in encrypting data. This value is used to store the object and then it is discarded; Amazon S3 does not store the encryption key. The key must be appropriate for use with the algorithm specified in the <code>x-amz-server-side-encryption-customer-algorithm</code> header." , long_help = None)]
    sse_customer_key: Option<String>,
    # [clap (long = "checksum-sha1" , name = "ChecksumSHA1" , help = "This header can be used as a data integrity check to verify that the data received is the same data that was originally sent. This header specifies the base64-encoded, 160-bit SHA-1 digest of the object. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html\">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>." , long_help = None)]
    checksum_sha1: Option<String>,
    # [clap (long = "website-redirect-location" , name = "WebsiteRedirectLocation" , help = "If the bucket is configured as a website, redirects requests for this object to another object in the same bucket or to an external URL. Amazon S3 stores the value of this header in the object metadata. For information about object metadata, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/dev/UsingMetadata.html\">Object Key and Metadata</a>." , long_help = None)]
    website_redirect_location: Option<String>,
    # [clap (long = "cache-control" , name = "CacheControl" , help = "Can be used to specify caching behavior along the request/reply chain. For more information, see <a href=\"http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.9\">http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.9</a>." , long_help = None)]
    cache_control: Option<String>,
    # [clap (long = "content-length" , name = "ContentLength" , help = "Size of the body in bytes. This parameter is useful when the size of the body cannot be determined automatically. For more information, see <a href=\"http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.13\">http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.13</a>." , long_help = None)]
    content_length: Option<i64>,
    # [clap (long = "request-payer" , name = "RequestPayer" , help = "" , long_help = None)]
    request_payer: Option<String>,
    # [clap (long = "checksum-crc32-c" , name = "ChecksumCRC32C" , help = "This header can be used as a data integrity check to verify that the data received is the same data that was originally sent. This header specifies the base64-encoded, 32-bit CRC32C checksum of the object. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html\">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>." , long_help = None)]
    checksum_crc32_c: Option<String>,
    # [clap (long = "checksum-algorithm" , name = "ChecksumAlgorithm" , help = "Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html\">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>." , long_help = None)]
    checksum_algorithm: Option<String>,
    # [clap (long = "object-lock-mode" , name = "ObjectLockMode" , help = "The Object Lock mode that you want to apply to this object." , long_help = None)]
    object_lock_mode: Option<String>,
    # [clap (long = "grant-full-control" , name = "GrantFullControl" , help = "Gives the grantee READ, READ_ACP, and WRITE_ACP permissions on the object." , long_help = None)]
    grant_full_control: Option<String>,
    # [clap (long = "sse-customer-algorithm" , name = "SSECustomerAlgorithm" , help = "Specifies the algorithm to use to when encrypting the object (for example, AES256)." , long_help = None)]
    sse_customer_algorithm: Option<String>,
    # [clap (long = "storage-class" , name = "StorageClass" , help = "By default, Amazon S3 uses the STANDARD Storage Class to store newly created objects. The STANDARD storage class provides high durability and high availability. Depending on performance needs, you can specify a different Storage Class. Amazon S3 on Outposts only uses the OUTPOSTS Storage Class. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/dev/storage-class-intro.html\">Storage Classes</a> in the <i>Amazon S3 User Guide</i>." , long_help = None)]
    storage_class: Option<String>,
    # [clap (long = "bucket" , name = "Bucket" , help = "The bucket name to which the PUT action was initiated." , long_help = None)]
    bucket: String,
    # [clap (long = "content-language" , name = "ContentLanguage" , help = "The language the content is in." , long_help = None)]
    content_language: Option<String>,
    # [clap (long = "acl" , name = "ACL" , help = "The canned ACL to apply to the object. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/dev/acl-overview.html#CannedACL\">Canned ACL</a>." , long_help = None)]
    acl: Option<String>,
    # [clap (long = "metadata" , name = "Metadata" , help = "A map of metadata to store with the object in S3." , long_help = None)]
    metadata: Option<String>,
    # [clap (long = "grant-read" , name = "GrantRead" , help = "Allows grantee to read the object data and its metadata." , long_help = None)]
    grant_read: Option<String>,
    # [clap (long = "object-lock-legal-hold-status" , name = "ObjectLockLegalHoldStatus" , help = "Specifies whether a legal hold will be applied to this object. For more information about S3 Object Lock, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lock.html\">Object Lock</a>." , long_help = None)]
    object_lock_legal_hold_status: Option<String>,
    # [clap (long = "body" , name = "Body" , help = "Object data." , long_help = None)]
    body: Option<Vec<u8>>,
    # [clap (long = "content-md5" , name = "ContentMD5" , help = "The base64-encoded 128-bit MD5 digest of the message (without the headers) according to RFC 1864. This header can be used as a message integrity check to verify that the data is the same data that was originally sent. Although it is optional, we recommend using the Content-MD5 mechanism as an end-to-end integrity check. For more information about REST request authentication, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/dev/RESTAuthentication.html\">REST Authentication</a>." , long_help = None)]
    content_md5: Option<String>,
    # [clap (long = "tagging" , name = "Tagging" , help = "The tag-set for the object. The tag-set must be encoded as URL Query parameters. (For example, \"Key1=Value1\")" , long_help = None)]
    tagging: Option<String>,
    # [clap (long = "content-disposition" , name = "ContentDisposition" , help = "Specifies presentational information for the object. For more information, see <a href=\"http://www.w3.org/Protocols/rfc2616/rfc2616-sec19.html#sec19.5.1\">http://www.w3.org/Protocols/rfc2616/rfc2616-sec19.html#sec19.5.1</a>." , long_help = None)]
    content_disposition: Option<String>,
    # [clap (long = "sse-customer-key-md5" , name = "SSECustomerKeyMD5" , help = "Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure that the encryption key was transmitted without error." , long_help = None)]
    sse_customer_key_md5: Option<String>,
    # [clap (long = "ssekms-encryption-context" , name = "SSEKMSEncryptionContext" , help = "Specifies the Amazon Web Services KMS Encryption Context to use for object encryption. The value of this header is a base64-encoded UTF-8 string holding JSON with the encryption context key-value pairs." , long_help = None)]
    ssekms_encryption_context: Option<String>,
    # [clap (long = "object-lock-retain-until-date" , name = "ObjectLockRetainUntilDate" , help = "The date and time when you want this object's Object Lock to expire. Must be formatted as a timestamp parameter." , long_help = None)]
    object_lock_retain_until_date: Option<String>,
    # [clap (long = "grant-write-acp" , name = "GrantWriteACP" , help = "Allows grantee to write the ACL for the applicable object." , long_help = None)]
    grant_write_acp: Option<String>,
    # [clap (long = "server-side-encryption" , name = "ServerSideEncryption" , help = "The server-side encryption algorithm used when storing this object in Amazon S3 (for example, AES256, aws:kms)." , long_help = None)]
    server_side_encryption: Option<String>,
}
impl PutObjectCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .put_object()
            .set_checksum_sha256(self.checksum_sha256.to_owned())
            .set_content_encoding(self.content_encoding.to_owned())
            .set_checksum_crc32(self.checksum_crc32.to_owned())
            .set_ssekms_key_id(self.ssekms_key_id.to_owned())
            .set_expires(None)
            .set_bucket_key_enabled(self.bucket_key_enabled)
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_content_type(self.content_type.to_owned())
            .set_key(Some(self.key.to_owned()))
            .set_grant_read_acp(self.grant_read_acp.to_owned())
            .set_sse_customer_key(self.sse_customer_key.to_owned())
            .set_checksum_sha1(self.checksum_sha1.to_owned())
            .set_website_redirect_location(self.website_redirect_location.to_owned())
            .set_cache_control(self.cache_control.to_owned())
            .set_content_length(self.content_length)
            .set_request_payer(None)
            .set_checksum_crc32_c(self.checksum_crc32_c.to_owned())
            .set_checksum_algorithm(None)
            .set_object_lock_mode(None)
            .set_grant_full_control(self.grant_full_control.to_owned())
            .set_sse_customer_algorithm(self.sse_customer_algorithm.to_owned())
            .set_storage_class(None)
            .set_bucket(Some(self.bucket.to_owned()))
            .set_content_language(self.content_language.to_owned())
            .set_acl(None)
            .set_metadata(None)
            .set_grant_read(self.grant_read.to_owned())
            .set_object_lock_legal_hold_status(None)
            .set_body(None)
            .set_content_md5(self.content_md5.to_owned())
            .set_tagging(self.tagging.to_owned())
            .set_content_disposition(self.content_disposition.to_owned())
            .set_sse_customer_key_md5(self.sse_customer_key_md5.to_owned())
            .set_ssekms_encryption_context(self.ssekms_encryption_context.to_owned())
            .set_object_lock_retain_until_date(None)
            .set_grant_write_acp(self.grant_write_acp.to_owned())
            .set_server_side_encryption(None)
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct CreateMultipartUploadCommand {
    # [clap (long = "key" , name = "Key" , help = "Object key for which the multipart upload is to be initiated." , long_help = None)]
    key: String,
    # [clap (long = "sse-customer-key-md5" , name = "SSECustomerKeyMD5" , help = "Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure that the encryption key was transmitted without error." , long_help = None)]
    sse_customer_key_md5: Option<String>,
    # [clap (long = "content-language" , name = "ContentLanguage" , help = "The language the content is in." , long_help = None)]
    content_language: Option<String>,
    # [clap (long = "object-lock-legal-hold-status" , name = "ObjectLockLegalHoldStatus" , help = "Specifies whether you want to apply a legal hold to the uploaded object." , long_help = None)]
    object_lock_legal_hold_status: Option<String>,
    # [clap (long = "tagging" , name = "Tagging" , help = "The tag-set for the object. The tag-set must be encoded as URL Query parameters." , long_help = None)]
    tagging: Option<String>,
    # [clap (long = "ssekms-key-id" , name = "SSEKMSKeyId" , help = "Specifies the ID of the symmetric customer managed key to use for object encryption. All GET and PUT requests for an object protected by Amazon Web Services KMS will fail if not made via SSL or using SigV4. For information about configuring using any of the officially supported Amazon Web Services SDKs and Amazon Web Services CLI, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/dev/UsingAWSSDK.html#specify-signature-version\">Specifying the Signature Version in Request Authentication</a> in the <i>Amazon S3 User Guide</i>." , long_help = None)]
    ssekms_key_id: Option<String>,
    # [clap (long = "website-redirect-location" , name = "WebsiteRedirectLocation" , help = "If the bucket is configured as a website, redirects requests for this object to another object in the same bucket or to an external URL. Amazon S3 stores the value of this header in the object metadata." , long_help = None)]
    website_redirect_location: Option<String>,
    # [clap (long = "grant-read" , name = "GrantRead" , help = "Allows grantee to read the object data and its metadata." , long_help = None)]
    grant_read: Option<String>,
    # [clap (long = "bucket-key-enabled" , name = "BucketKeyEnabled" , help = "Specifies whether Amazon S3 should use an S3 Bucket Key for object encryption with server-side encryption using AWS KMS (SSE-KMS). Setting this header to <code>true</code> causes Amazon S3 to use an S3 Bucket Key for object encryption with SSE-KMS." , long_help = None)]
    bucket_key_enabled: Option<bool>,
    # [clap (long = "grant-read-acp" , name = "GrantReadACP" , help = "Allows grantee to read the object ACL." , long_help = None)]
    grant_read_acp: Option<String>,
    # [clap (long = "content-encoding" , name = "ContentEncoding" , help = "Specifies what content encodings have been applied to the object and thus what decoding mechanisms must be applied to obtain the media-type referenced by the Content-Type header field." , long_help = None)]
    content_encoding: Option<String>,
    # [clap (long = "content-disposition" , name = "ContentDisposition" , help = "Specifies presentational information for the object." , long_help = None)]
    content_disposition: Option<String>,
    # [clap (long = "metadata" , name = "Metadata" , help = "A map of metadata to store with the object in S3." , long_help = None)]
    metadata: Option<String>,
    # [clap (long = "request-payer" , name = "RequestPayer" , help = "" , long_help = None)]
    request_payer: Option<String>,
    # [clap (long = "storage-class" , name = "StorageClass" , help = "By default, Amazon S3 uses the STANDARD Storage Class to store newly created objects. The STANDARD storage class provides high durability and high availability. Depending on performance needs, you can specify a different Storage Class. Amazon S3 on Outposts only uses the OUTPOSTS Storage Class. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/dev/storage-class-intro.html\">Storage Classes</a> in the <i>Amazon S3 User Guide</i>." , long_help = None)]
    storage_class: Option<String>,
    # [clap (long = "object-lock-mode" , name = "ObjectLockMode" , help = "Specifies the Object Lock mode that you want to apply to the uploaded object." , long_help = None)]
    object_lock_mode: Option<String>,
    # [clap (long = "server-side-encryption" , name = "ServerSideEncryption" , help = "The server-side encryption algorithm used when storing this object in Amazon S3 (for example, AES256, aws:kms)." , long_help = None)]
    server_side_encryption: Option<String>,
    # [clap (long = "sse-customer-key" , name = "SSECustomerKey" , help = "Specifies the customer-provided encryption key for Amazon S3 to use in encrypting data. This value is used to store the object and then it is discarded; Amazon S3 does not store the encryption key. The key must be appropriate for use with the algorithm specified in the <code>x-amz-server-side-encryption-customer-algorithm</code> header." , long_help = None)]
    sse_customer_key: Option<String>,
    # [clap (long = "grant-write-acp" , name = "GrantWriteACP" , help = "Allows grantee to write the ACL for the applicable object." , long_help = None)]
    grant_write_acp: Option<String>,
    # [clap (long = "bucket" , name = "Bucket" , help = "The name of the bucket to which to initiate the upload" , long_help = None)]
    bucket: String,
    # [clap (long = "checksum-algorithm" , name = "ChecksumAlgorithm" , help = "Indicates the algorithm you want Amazon S3 to use to create the checksum for the object. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html\">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>." , long_help = None)]
    checksum_algorithm: Option<String>,
    # [clap (long = "cache-control" , name = "CacheControl" , help = "Specifies caching behavior along the request/reply chain." , long_help = None)]
    cache_control: Option<String>,
    # [clap (long = "ssekms-encryption-context" , name = "SSEKMSEncryptionContext" , help = "Specifies the Amazon Web Services KMS Encryption Context to use for object encryption. The value of this header is a base64-encoded UTF-8 string holding JSON with the encryption context key-value pairs." , long_help = None)]
    ssekms_encryption_context: Option<String>,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    # [clap (long = "acl" , name = "ACL" , help = "The canned ACL to apply to the object." , long_help = None)]
    acl: Option<String>,
    # [clap (long = "expires" , name = "Expires" , help = "The date and time at which the object is no longer cacheable." , long_help = None)]
    expires: Option<String>,
    # [clap (long = "object-lock-retain-until-date" , name = "ObjectLockRetainUntilDate" , help = "Specifies the date and time when you want the Object Lock to expire." , long_help = None)]
    object_lock_retain_until_date: Option<String>,
    # [clap (long = "sse-customer-algorithm" , name = "SSECustomerAlgorithm" , help = "Specifies the algorithm to use to when encrypting the object (for example, AES256)." , long_help = None)]
    sse_customer_algorithm: Option<String>,
    # [clap (long = "grant-full-control" , name = "GrantFullControl" , help = "Gives the grantee READ, READ_ACP, and WRITE_ACP permissions on the object." , long_help = None)]
    grant_full_control: Option<String>,
    # [clap (long = "content-type" , name = "ContentType" , help = "A standard MIME type describing the format of the object data." , long_help = None)]
    content_type: Option<String>,
}
impl CreateMultipartUploadCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .create_multipart_upload()
            .set_key(Some(self.key.to_owned()))
            .set_sse_customer_key_md5(self.sse_customer_key_md5.to_owned())
            .set_content_language(self.content_language.to_owned())
            .set_object_lock_legal_hold_status(None)
            .set_tagging(self.tagging.to_owned())
            .set_ssekms_key_id(self.ssekms_key_id.to_owned())
            .set_website_redirect_location(self.website_redirect_location.to_owned())
            .set_grant_read(self.grant_read.to_owned())
            .set_bucket_key_enabled(self.bucket_key_enabled)
            .set_grant_read_acp(self.grant_read_acp.to_owned())
            .set_content_encoding(self.content_encoding.to_owned())
            .set_content_disposition(self.content_disposition.to_owned())
            .set_metadata(None)
            .set_request_payer(None)
            .set_storage_class(None)
            .set_object_lock_mode(None)
            .set_server_side_encryption(None)
            .set_sse_customer_key(self.sse_customer_key.to_owned())
            .set_grant_write_acp(self.grant_write_acp.to_owned())
            .set_bucket(Some(self.bucket.to_owned()))
            .set_checksum_algorithm(None)
            .set_cache_control(self.cache_control.to_owned())
            .set_ssekms_encryption_context(self.ssekms_encryption_context.to_owned())
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_acl(None)
            .set_expires(None)
            .set_object_lock_retain_until_date(None)
            .set_sse_customer_algorithm(self.sse_customer_algorithm.to_owned())
            .set_grant_full_control(self.grant_full_control.to_owned())
            .set_content_type(self.content_type.to_owned())
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct GetBucketCorsCommand {
    # [clap (long = "bucket" , name = "Bucket" , help = "The bucket name for which to get the cors configuration." , long_help = None)]
    bucket: String,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
}
impl GetBucketCorsCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .get_bucket_cors()
            .set_bucket(Some(self.bucket.to_owned()))
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct ListBucketInventoryConfigurationsCommand {
    # [clap (long = "continuation-token" , name = "ContinuationToken" , help = "The marker used to continue an inventory configuration listing that has been truncated. Use the NextContinuationToken from a previously truncated list response to continue the listing. The continuation token is an opaque value that Amazon S3 understands." , long_help = None)]
    continuation_token: Option<String>,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    # [clap (long = "bucket" , name = "Bucket" , help = "The name of the bucket containing the inventory configurations to retrieve." , long_help = None)]
    bucket: String,
}
impl ListBucketInventoryConfigurationsCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .list_bucket_inventory_configurations()
            .set_continuation_token(self.continuation_token.to_owned())
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_bucket(Some(self.bucket.to_owned()))
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct GetBucketReplicationCommand {
    # [clap (long = "bucket" , name = "Bucket" , help = "The bucket name for which to get the replication information." , long_help = None)]
    bucket: String,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
}
impl GetBucketReplicationCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .get_bucket_replication()
            .set_bucket(Some(self.bucket.to_owned()))
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Args, Debug, Clone)]
pub struct CompletedMultipartUploadArgs {
    #[clap(long = "parts")]
    parts: Option<String>,
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct CompleteMultipartUploadCommand {
    # [clap (long = "sse-customer-algorithm" , name = "SSECustomerAlgorithm" , help = "The server-side encryption (SSE) algorithm used to encrypt the object. This parameter is needed only when the object was created using a checksum algorithm. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html\">Protecting data using SSE-C keys</a> in the <i>Amazon S3 User Guide</i>." , long_help = None)]
    sse_customer_algorithm: Option<String>,
    # [clap (long = "checksum-sha256" , name = "ChecksumSHA256" , help = "This header can be used as a data integrity check to verify that the data received is the same data that was originally sent. This header specifies the base64-encoded, 256-bit SHA-256 digest of the object. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html\">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>." , long_help = None)]
    checksum_sha256: Option<String>,
    # [clap (long = "sse-customer-key-md5" , name = "SSECustomerKeyMD5" , help = "The MD5 server-side encryption (SSE) customer managed key. This parameter is needed only when the object was created using a checksum algorithm. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html\">Protecting data using SSE-C keys</a> in the <i>Amazon S3 User Guide</i>." , long_help = None)]
    sse_customer_key_md5: Option<String>,
    # [clap (long = "checksum-crc32" , name = "ChecksumCRC32" , help = "This header can be used as a data integrity check to verify that the data received is the same data that was originally sent. This header specifies the base64-encoded, 32-bit CRC32 checksum of the object. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html\">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>." , long_help = None)]
    checksum_crc32: Option<String>,
    #[clap(flatten)]
    multipart_upload: CompletedMultipartUploadArgs,
    # [clap (long = "checksum-sha1" , name = "ChecksumSHA1" , help = "This header can be used as a data integrity check to verify that the data received is the same data that was originally sent. This header specifies the base64-encoded, 160-bit SHA-1 digest of the object. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html\">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>." , long_help = None)]
    checksum_sha1: Option<String>,
    # [clap (long = "key" , name = "Key" , help = "Object key for which the multipart upload was initiated." , long_help = None)]
    key: String,
    # [clap (long = "upload-id" , name = "UploadId" , help = "ID for the initiated multipart upload." , long_help = None)]
    upload_id: String,
    # [clap (long = "request-payer" , name = "RequestPayer" , help = "" , long_help = None)]
    request_payer: Option<String>,
    # [clap (long = "checksum-crc32-c" , name = "ChecksumCRC32C" , help = "This header can be used as a data integrity check to verify that the data received is the same data that was originally sent. This header specifies the base64-encoded, 32-bit CRC32C checksum of the object. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html\">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>." , long_help = None)]
    checksum_crc32_c: Option<String>,
    # [clap (long = "bucket" , name = "Bucket" , help = "Name of the bucket to which the multipart upload was initiated." , long_help = None)]
    bucket: String,
    # [clap (long = "sse-customer-key" , name = "SSECustomerKey" , help = "The server-side encryption (SSE) customer managed key. This parameter is needed only when the object was created using a checksum algorithm. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html\">Protecting data using SSE-C keys</a> in the <i>Amazon S3 User Guide</i>." , long_help = None)]
    sse_customer_key: Option<String>,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
}
impl CompleteMultipartUploadCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .complete_multipart_upload()
            .set_sse_customer_algorithm(self.sse_customer_algorithm.to_owned())
            .set_checksum_sha256(self.checksum_sha256.to_owned())
            .set_sse_customer_key_md5(self.sse_customer_key_md5.to_owned())
            .set_checksum_crc32(self.checksum_crc32.to_owned())
            .set_multipart_upload(None)
            .set_checksum_sha1(self.checksum_sha1.to_owned())
            .set_key(Some(self.key.to_owned()))
            .set_upload_id(Some(self.upload_id.to_owned()))
            .set_request_payer(None)
            .set_checksum_crc32_c(self.checksum_crc32_c.to_owned())
            .set_bucket(Some(self.bucket.to_owned()))
            .set_sse_customer_key(self.sse_customer_key.to_owned())
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct GetBucketTaggingCommand {
    # [clap (long = "bucket" , name = "Bucket" , help = "The name of the bucket for which to get the tagging information." , long_help = None)]
    bucket: String,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
}
impl GetBucketTaggingCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .get_bucket_tagging()
            .set_bucket(Some(self.bucket.to_owned()))
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Args, Debug, Clone)]
pub struct CorsConfigurationArgs {
    #[clap(long = "cors-rules")]
    cors_rules: Option<String>,
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct PutBucketCorsCommand {
    #[clap(flatten)]
    cors_configuration: CorsConfigurationArgs,
    # [clap (long = "bucket" , name = "Bucket" , help = "Specifies the bucket impacted by the <code>cors</code>configuration." , long_help = None)]
    bucket: String,
    # [clap (long = "checksum-algorithm" , name = "ChecksumAlgorithm" , help = "Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html\">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>." , long_help = None)]
    checksum_algorithm: Option<String>,
    # [clap (long = "content-md5" , name = "ContentMD5" , help = "The base64-encoded 128-bit MD5 digest of the data. This header must be used as a message integrity check to verify that the request body was not corrupted in transit. For more information, go to <a href=\"http://www.ietf.org/rfc/rfc1864.txt\">RFC 1864.</a>" , long_help = None)]
    content_md5: Option<String>,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
}
impl PutBucketCorsCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .put_bucket_cors()
            .set_cors_configuration(None)
            .set_bucket(Some(self.bucket.to_owned()))
            .set_checksum_algorithm(None)
            .set_content_md5(self.content_md5.to_owned())
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct ListObjectsV2Command {
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    # [clap (long = "encoding-type" , name = "EncodingType" , help = "Encoding type used by Amazon S3 to encode object keys in the response." , long_help = None)]
    encoding_type: Option<String>,
    # [clap (long = "delimiter" , name = "Delimiter" , help = "A delimiter is a character you use to group keys." , long_help = None)]
    delimiter: Option<String>,
    # [clap (long = "bucket" , name = "Bucket" , help = "Bucket name to list." , long_help = None)]
    bucket: String,
    # [clap (long = "fetch-owner" , name = "FetchOwner" , help = "The owner field is not present in listV2 by default, if you want to return owner field with each key in the result then set the fetch owner field to true." , long_help = None)]
    fetch_owner: Option<bool>,
    # [clap (long = "max-keys" , name = "MaxKeys" , help = "Sets the maximum number of keys returned in the response. By default the action returns up to 1,000 key names. The response might contain fewer keys but will never contain more." , long_help = None)]
    max_keys: Option<i32>,
    # [clap (long = "prefix" , name = "Prefix" , help = "Limits the response to keys that begin with the specified prefix." , long_help = None)]
    prefix: Option<String>,
    # [clap (long = "request-payer" , name = "RequestPayer" , help = "Confirms that the requester knows that she or he will be charged for the list objects request in V2 style. Bucket owners need not specify this parameter in their requests." , long_help = None)]
    request_payer: Option<String>,
    # [clap (long = "start-after" , name = "StartAfter" , help = "StartAfter is where you want Amazon S3 to start listing from. Amazon S3 starts listing after this specified key. StartAfter can be any key in the bucket." , long_help = None)]
    start_after: Option<String>,
    # [clap (long = "continuation-token" , name = "ContinuationToken" , help = "ContinuationToken indicates Amazon S3 that the list is being continued on this bucket with a token. ContinuationToken is obfuscated and is not a real key." , long_help = None)]
    continuation_token: Option<String>,
}
impl ListObjectsV2Command {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .list_objects_v2()
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_encoding_type(None)
            .set_delimiter(self.delimiter.to_owned())
            .set_bucket(Some(self.bucket.to_owned()))
            .set_fetch_owner(self.fetch_owner)
            .set_max_keys(self.max_keys)
            .set_prefix(self.prefix.to_owned())
            .set_request_payer(None)
            .set_start_after(self.start_after.to_owned())
            .set_continuation_token(self.continuation_token.to_owned())
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct DeleteObjectTaggingCommand {
    # [clap (long = "version-id" , name = "VersionId" , help = "The versionId of the object that the tag-set will be removed from." , long_help = None)]
    version_id: Option<String>,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    # [clap (long = "bucket" , name = "Bucket" , help = "The bucket name containing the objects from which to remove the tags." , long_help = None)]
    bucket: String,
    # [clap (long = "key" , name = "Key" , help = "The key that identifies the object in the bucket from which to remove all tags." , long_help = None)]
    key: String,
}
impl DeleteObjectTaggingCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .delete_object_tagging()
            .set_version_id(self.version_id.to_owned())
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_bucket(Some(self.bucket.to_owned()))
            .set_key(Some(self.key.to_owned()))
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct GetObjectAttributesCommand {
    # [clap (long = "request-payer" , name = "RequestPayer" , help = "" , long_help = None)]
    request_payer: Option<String>,
    # [clap (long = "key" , name = "Key" , help = "The object key." , long_help = None)]
    key: String,
    # [clap (long = "max-parts" , name = "MaxParts" , help = "Sets the maximum number of parts to return." , long_help = None)]
    max_parts: Option<i32>,
    # [clap (long = "sse-customer-algorithm" , name = "SSECustomerAlgorithm" , help = "Specifies the algorithm to use when encrypting the object (for example, AES256)." , long_help = None)]
    sse_customer_algorithm: Option<String>,
    # [clap (long = "version-id" , name = "VersionId" , help = "The version ID used to reference a specific version of the object." , long_help = None)]
    version_id: Option<String>,
    # [clap (long = "object-attributes" , name = "ObjectAttributes" , help = "An XML header that specifies the fields at the root level that you want returned in the response. Fields that you do not specify are not returned." , long_help = None)]
    object_attributes: String,
    # [clap (long = "part-number-marker" , name = "PartNumberMarker" , help = "Specifies the part after which listing should begin. Only parts with higher part numbers will be listed." , long_help = None)]
    part_number_marker: Option<String>,
    # [clap (long = "bucket" , name = "Bucket" , help = "The name of the bucket that contains the object." , long_help = None)]
    bucket: String,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    # [clap (long = "sse-customer-key" , name = "SSECustomerKey" , help = "Specifies the customer-provided encryption key for Amazon S3 to use in encrypting data. This value is used to store the object and then it is discarded; Amazon S3 does not store the encryption key. The key must be appropriate for use with the algorithm specified in the <code>x-amz-server-side-encryption-customer-algorithm</code> header." , long_help = None)]
    sse_customer_key: Option<String>,
    # [clap (long = "sse-customer-key-md5" , name = "SSECustomerKeyMD5" , help = "Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure that the encryption key was transmitted without error." , long_help = None)]
    sse_customer_key_md5: Option<String>,
}
impl GetObjectAttributesCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .get_object_attributes()
            .set_request_payer(None)
            .set_key(Some(self.key.to_owned()))
            .set_max_parts(self.max_parts)
            .set_sse_customer_algorithm(self.sse_customer_algorithm.to_owned())
            .set_version_id(self.version_id.to_owned())
            .set_object_attributes(None)
            .set_part_number_marker(self.part_number_marker.to_owned())
            .set_bucket(Some(self.bucket.to_owned()))
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_sse_customer_key(self.sse_customer_key.to_owned())
            .set_sse_customer_key_md5(self.sse_customer_key_md5.to_owned())
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct GetObjectRetentionCommand {
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    # [clap (long = "bucket" , name = "Bucket" , help = "The bucket name containing the object whose retention settings you want to retrieve." , long_help = None)]
    bucket: String,
    # [clap (long = "request-payer" , name = "RequestPayer" , help = "" , long_help = None)]
    request_payer: Option<String>,
    # [clap (long = "key" , name = "Key" , help = "The key name for the object whose retention settings you want to retrieve." , long_help = None)]
    key: String,
    # [clap (long = "version-id" , name = "VersionId" , help = "The version ID for the object whose retention settings you want to retrieve." , long_help = None)]
    version_id: Option<String>,
}
impl GetObjectRetentionCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .get_object_retention()
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_bucket(Some(self.bucket.to_owned()))
            .set_request_payer(None)
            .set_key(Some(self.key.to_owned()))
            .set_version_id(self.version_id.to_owned())
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct GetObjectCommand {
    # [clap (long = "version-id" , name = "VersionId" , help = "VersionId used to reference a specific version of the object." , long_help = None)]
    version_id: Option<String>,
    # [clap (long = "response-content-disposition" , name = "ResponseContentDisposition" , help = "Sets the <code>Content-Disposition</code> header of the response" , long_help = None)]
    response_content_disposition: Option<String>,
    # [clap (long = "request-payer" , name = "RequestPayer" , help = "" , long_help = None)]
    request_payer: Option<String>,
    # [clap (long = "bucket" , name = "Bucket" , help = "The bucket name containing the object." , long_help = None)]
    bucket: String,
    # [clap (long = "range" , name = "Range" , help = "Downloads the specified range bytes of an object. For more information about the HTTP Range header, see <a href=\"https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.35\">https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.35</a>." , long_help = None)]
    range: Option<String>,
    # [clap (long = "response-cache-control" , name = "ResponseCacheControl" , help = "Sets the <code>Cache-Control</code> header of the response." , long_help = None)]
    response_cache_control: Option<String>,
    # [clap (long = "checksum-mode" , name = "ChecksumMode" , help = "To retrieve the checksum, this mode must be enabled." , long_help = None)]
    checksum_mode: Option<String>,
    # [clap (long = "response-content-encoding" , name = "ResponseContentEncoding" , help = "Sets the <code>Content-Encoding</code> header of the response." , long_help = None)]
    response_content_encoding: Option<String>,
    # [clap (long = "if-unmodified-since" , name = "IfUnmodifiedSince" , help = "Return the object only if it has not been modified since the specified time; otherwise, return a 412 (precondition failed) error." , long_help = None)]
    if_unmodified_since: Option<String>,
    # [clap (long = "sse-customer-algorithm" , name = "SSECustomerAlgorithm" , help = "Specifies the algorithm to use to when decrypting the object (for example, AES256)." , long_help = None)]
    sse_customer_algorithm: Option<String>,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    # [clap (long = "if-match" , name = "IfMatch" , help = "Return the object only if its entity tag (ETag) is the same as the one specified; otherwise, return a 412 (precondition failed) error." , long_help = None)]
    if_match: Option<String>,
    # [clap (long = "key" , name = "Key" , help = "Key of the object to get." , long_help = None)]
    key: String,
    # [clap (long = "response-content-type" , name = "ResponseContentType" , help = "Sets the <code>Content-Type</code> header of the response." , long_help = None)]
    response_content_type: Option<String>,
    # [clap (long = "sse-customer-key" , name = "SSECustomerKey" , help = "Specifies the customer-provided encryption key for Amazon S3 used to encrypt the data. This value is used to decrypt the object when recovering it and must match the one used when storing the data. The key must be appropriate for use with the algorithm specified in the <code>x-amz-server-side-encryption-customer-algorithm</code> header." , long_help = None)]
    sse_customer_key: Option<String>,
    # [clap (long = "response-expires" , name = "ResponseExpires" , help = "Sets the <code>Expires</code> header of the response." , long_help = None)]
    response_expires: Option<String>,
    # [clap (long = "sse-customer-key-md5" , name = "SSECustomerKeyMD5" , help = "Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure that the encryption key was transmitted without error." , long_help = None)]
    sse_customer_key_md5: Option<String>,
    # [clap (long = "if-modified-since" , name = "IfModifiedSince" , help = "Return the object only if it has been modified since the specified time; otherwise, return a 304 (not modified) error." , long_help = None)]
    if_modified_since: Option<String>,
    # [clap (long = "if-none-match" , name = "IfNoneMatch" , help = "Return the object only if its entity tag (ETag) is different from the one specified; otherwise, return a 304 (not modified) error." , long_help = None)]
    if_none_match: Option<String>,
    # [clap (long = "part-number" , name = "PartNumber" , help = "Part number of the object being read. This is a positive integer between 1 and 10,000. Effectively performs a 'ranged' GET request for the part specified. Useful for downloading just a part of an object." , long_help = None)]
    part_number: Option<i32>,
    # [clap (long = "response-content-language" , name = "ResponseContentLanguage" , help = "Sets the <code>Content-Language</code> header of the response." , long_help = None)]
    response_content_language: Option<String>,
}
impl GetObjectCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .get_object()
            .set_version_id(self.version_id.to_owned())
            .set_response_content_disposition(self.response_content_disposition.to_owned())
            .set_request_payer(None)
            .set_bucket(Some(self.bucket.to_owned()))
            .set_range(self.range.to_owned())
            .set_response_cache_control(self.response_cache_control.to_owned())
            .set_checksum_mode(None)
            .set_response_content_encoding(self.response_content_encoding.to_owned())
            .set_if_unmodified_since(None)
            .set_sse_customer_algorithm(self.sse_customer_algorithm.to_owned())
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_if_match(self.if_match.to_owned())
            .set_key(Some(self.key.to_owned()))
            .set_response_content_type(self.response_content_type.to_owned())
            .set_sse_customer_key(self.sse_customer_key.to_owned())
            .set_response_expires(None)
            .set_sse_customer_key_md5(self.sse_customer_key_md5.to_owned())
            .set_if_modified_since(None)
            .set_if_none_match(self.if_none_match.to_owned())
            .set_part_number(self.part_number)
            .set_response_content_language(self.response_content_language.to_owned())
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct DeleteBucketMetricsConfigurationCommand {
    # [clap (long = "id" , name = "Id" , help = "The ID used to identify the metrics configuration." , long_help = None)]
    id: String,
    # [clap (long = "bucket" , name = "Bucket" , help = "The name of the bucket containing the metrics configuration to delete." , long_help = None)]
    bucket: String,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
}
impl DeleteBucketMetricsConfigurationCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .delete_bucket_metrics_configuration()
            .set_id(Some(self.id.to_owned()))
            .set_bucket(Some(self.bucket.to_owned()))
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct PutBucketAclCommand {
    #[clap(flatten)]
    access_control_policy: AccessControlPolicyArgs,
    # [clap (long = "acl" , name = "ACL" , help = "The canned ACL to apply to the bucket." , long_help = None)]
    acl: Option<String>,
    # [clap (long = "grant-full-control" , name = "GrantFullControl" , help = "Allows grantee the read, write, read ACP, and write ACP permissions on the bucket." , long_help = None)]
    grant_full_control: Option<String>,
    # [clap (long = "grant-write-acp" , name = "GrantWriteACP" , help = "Allows grantee to write the ACL for the applicable bucket." , long_help = None)]
    grant_write_acp: Option<String>,
    # [clap (long = "grant-read-acp" , name = "GrantReadACP" , help = "Allows grantee to read the bucket ACL." , long_help = None)]
    grant_read_acp: Option<String>,
    # [clap (long = "checksum-algorithm" , name = "ChecksumAlgorithm" , help = "Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html\">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>." , long_help = None)]
    checksum_algorithm: Option<String>,
    # [clap (long = "content-md5" , name = "ContentMD5" , help = "The base64-encoded 128-bit MD5 digest of the data. This header must be used as a message integrity check to verify that the request body was not corrupted in transit. For more information, go to <a href=\"http://www.ietf.org/rfc/rfc1864.txt\">RFC 1864.</a>" , long_help = None)]
    content_md5: Option<String>,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    # [clap (long = "bucket" , name = "Bucket" , help = "The bucket to which to apply the ACL." , long_help = None)]
    bucket: String,
    # [clap (long = "grant-read" , name = "GrantRead" , help = "Allows grantee to list the objects in the bucket." , long_help = None)]
    grant_read: Option<String>,
    # [clap (long = "grant-write" , name = "GrantWrite" , help = "Allows grantee to create new objects in the bucket." , long_help = None)]
    grant_write: Option<String>,
}
impl PutBucketAclCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .put_bucket_acl()
            .set_access_control_policy(None)
            .set_acl(None)
            .set_grant_full_control(self.grant_full_control.to_owned())
            .set_grant_write_acp(self.grant_write_acp.to_owned())
            .set_grant_read_acp(self.grant_read_acp.to_owned())
            .set_checksum_algorithm(None)
            .set_content_md5(self.content_md5.to_owned())
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_bucket(Some(self.bucket.to_owned()))
            .set_grant_read(self.grant_read.to_owned())
            .set_grant_write(self.grant_write.to_owned())
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Args, Debug, Clone)]
pub struct ServerSideEncryptionConfigurationArgs {
    #[clap(long = "rules")]
    rules: Option<String>,
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct PutBucketEncryptionCommand {
    # [clap (long = "content-md5" , name = "ContentMD5" , help = "The base64-encoded 128-bit MD5 digest of the server-side encryption configuration." , long_help = None)]
    content_md5: Option<String>,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    #[clap(flatten)]
    server_side_encryption_configuration: ServerSideEncryptionConfigurationArgs,
    # [clap (long = "checksum-algorithm" , name = "ChecksumAlgorithm" , help = "Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html\">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>." , long_help = None)]
    checksum_algorithm: Option<String>,
    # [clap (long = "bucket" , name = "Bucket" , help = "Specifies default encryption for a bucket using server-side encryption with Amazon S3-managed keys (SSE-S3) or customer managed keys (SSE-KMS). For information about the Amazon S3 default encryption feature, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/dev/bucket-encryption.html\">Amazon S3 Default Bucket Encryption</a> in the <i>Amazon S3 User Guide</i>." , long_help = None)]
    bucket: String,
}
impl PutBucketEncryptionCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .put_bucket_encryption()
            .set_content_md5(self.content_md5.to_owned())
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_server_side_encryption_configuration(None)
            .set_checksum_algorithm(None)
            .set_bucket(Some(self.bucket.to_owned()))
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Args, Debug, Clone)]
pub struct AccelerateConfigurationArgs {
    #[clap(long = "status")]
    status: Option<String>,
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct PutBucketAccelerateConfigurationCommand {
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    # [clap (long = "checksum-algorithm" , name = "ChecksumAlgorithm" , help = "Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html\">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>." , long_help = None)]
    checksum_algorithm: Option<String>,
    # [clap (long = "bucket" , name = "Bucket" , help = "The name of the bucket for which the accelerate configuration is set." , long_help = None)]
    bucket: String,
    #[clap(flatten)]
    accelerate_configuration: AccelerateConfigurationArgs,
}
impl PutBucketAccelerateConfigurationCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .put_bucket_accelerate_configuration()
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_checksum_algorithm(None)
            .set_bucket(Some(self.bucket.to_owned()))
            .set_accelerate_configuration(None)
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct DeletePublicAccessBlockCommand {
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    # [clap (long = "bucket" , name = "Bucket" , help = "The Amazon S3 bucket whose <code>PublicAccessBlock</code> configuration you want to delete." , long_help = None)]
    bucket: String,
}
impl DeletePublicAccessBlockCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .delete_public_access_block()
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_bucket(Some(self.bucket.to_owned()))
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct DeleteBucketReplicationCommand {
    # [clap (long = "bucket" , name = "Bucket" , help = "The bucket name." , long_help = None)]
    bucket: String,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
}
impl DeleteBucketReplicationCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .delete_bucket_replication()
            .set_bucket(Some(self.bucket.to_owned()))
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Args, Debug, Clone)]
pub struct WebsiteConfigurationArgs {
    #[clap(long = "routing-rules")]
    routing_rules: Option<String>,
    #[clap(long = "index-document")]
    index_document: Option<String>,
    #[clap(long = "redirect-all-requests-to")]
    redirect_all_requests_to: Option<String>,
    #[clap(long = "error-document")]
    error_document: Option<String>,
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct PutBucketWebsiteCommand {
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    # [clap (long = "content-md5" , name = "ContentMD5" , help = "The base64-encoded 128-bit MD5 digest of the data. You must use this header as a message integrity check to verify that the request body was not corrupted in transit. For more information, see <a href=\"http://www.ietf.org/rfc/rfc1864.txt\">RFC 1864</a>." , long_help = None)]
    content_md5: Option<String>,
    #[clap(flatten)]
    website_configuration: WebsiteConfigurationArgs,
    # [clap (long = "bucket" , name = "Bucket" , help = "The bucket name." , long_help = None)]
    bucket: String,
    # [clap (long = "checksum-algorithm" , name = "ChecksumAlgorithm" , help = "Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html\">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>." , long_help = None)]
    checksum_algorithm: Option<String>,
}
impl PutBucketWebsiteCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .put_bucket_website()
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_content_md5(self.content_md5.to_owned())
            .set_website_configuration(None)
            .set_bucket(Some(self.bucket.to_owned()))
            .set_checksum_algorithm(None)
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Args, Debug, Clone)]
pub struct CreateBucketConfigurationArgs {
    #[clap(long = "location-constraint")]
    location_constraint: Option<String>,
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct CreateBucketCommand {
    # [clap (long = "bucket" , name = "Bucket" , help = "The name of the bucket to create." , long_help = None)]
    bucket: String,
    # [clap (long = "grant-read" , name = "GrantRead" , help = "Allows grantee to list the objects in the bucket." , long_help = None)]
    grant_read: Option<String>,
    # [clap (long = "grant-write-acp" , name = "GrantWriteACP" , help = "Allows grantee to write the ACL for the applicable bucket." , long_help = None)]
    grant_write_acp: Option<String>,
    # [clap (long = "grant-full-control" , name = "GrantFullControl" , help = "Allows grantee the read, write, read ACP, and write ACP permissions on the bucket." , long_help = None)]
    grant_full_control: Option<String>,
    # [clap (long = "object-lock-enabled-for-bucket" , name = "ObjectLockEnabledForBucket" , help = "Specifies whether you want S3 Object Lock to be enabled for the new bucket." , long_help = None)]
    object_lock_enabled_for_bucket: Option<bool>,
    # [clap (long = "object-ownership" , name = "ObjectOwnership" , help = "" , long_help = None)]
    object_ownership: Option<String>,
    # [clap (long = "acl" , name = "ACL" , help = "The canned ACL to apply to the bucket." , long_help = None)]
    acl: Option<String>,
    #[clap(flatten)]
    create_bucket_configuration: CreateBucketConfigurationArgs,
    # [clap (long = "grant-read-acp" , name = "GrantReadACP" , help = "Allows grantee to read the bucket ACL." , long_help = None)]
    grant_read_acp: Option<String>,
    # [clap (long = "grant-write" , name = "GrantWrite" , help = "Allows grantee to create new objects in the bucket." , long_help = None)]
    grant_write: Option<String>,
}
impl CreateBucketCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .create_bucket()
            .set_bucket(Some(self.bucket.to_owned()))
            .set_grant_read(self.grant_read.to_owned())
            .set_grant_write_acp(self.grant_write_acp.to_owned())
            .set_grant_full_control(self.grant_full_control.to_owned())
            .set_object_lock_enabled_for_bucket(self.object_lock_enabled_for_bucket)
            .set_object_ownership(None)
            .set_acl(None)
            .set_create_bucket_configuration(None)
            .set_grant_read_acp(self.grant_read_acp.to_owned())
            .set_grant_write(self.grant_write.to_owned())
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct DeleteObjectCommand {
    # [clap (long = "mfa" , name = "MFA" , help = "The concatenation of the authentication device's serial number, a space, and the value that is displayed on your authentication device. Required to permanently delete a versioned object if versioning is configured with MFA delete enabled." , long_help = None)]
    mfa: Option<String>,
    # [clap (long = "bypass-governance-retention" , name = "BypassGovernanceRetention" , help = "Indicates whether S3 Object Lock should bypass Governance-mode restrictions to process this operation. To use this header, you must have the <code>s3:BypassGovernanceRetention</code> permission." , long_help = None)]
    bypass_governance_retention: Option<bool>,
    # [clap (long = "bucket" , name = "Bucket" , help = "The bucket name of the bucket containing the object." , long_help = None)]
    bucket: String,
    # [clap (long = "key" , name = "Key" , help = "Key name of the object to delete." , long_help = None)]
    key: String,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    # [clap (long = "request-payer" , name = "RequestPayer" , help = "" , long_help = None)]
    request_payer: Option<String>,
    # [clap (long = "version-id" , name = "VersionId" , help = "VersionId used to reference a specific version of the object." , long_help = None)]
    version_id: Option<String>,
}
impl DeleteObjectCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .delete_object()
            .set_mfa(self.mfa.to_owned())
            .set_bypass_governance_retention(self.bypass_governance_retention)
            .set_bucket(Some(self.bucket.to_owned()))
            .set_key(Some(self.key.to_owned()))
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_request_payer(None)
            .set_version_id(self.version_id.to_owned())
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Args, Debug, Clone)]
pub struct BucketLifecycleConfigurationArgs {
    #[clap(long = "rules")]
    rules: Option<String>,
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct PutBucketLifecycleConfigurationCommand {
    # [clap (long = "checksum-algorithm" , name = "ChecksumAlgorithm" , help = "Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more information, see <a href=\"https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html\">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>." , long_help = None)]
    checksum_algorithm: Option<String>,
    # [clap (long = "bucket" , name = "Bucket" , help = "The name of the bucket for which to set the configuration." , long_help = None)]
    bucket: String,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    #[clap(flatten)]
    lifecycle_configuration: BucketLifecycleConfigurationArgs,
}
impl PutBucketLifecycleConfigurationCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .put_bucket_lifecycle_configuration()
            .set_checksum_algorithm(None)
            .set_bucket(Some(self.bucket.to_owned()))
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_lifecycle_configuration(None)
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct DeleteBucketOwnershipControlsCommand {
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    # [clap (long = "bucket" , name = "Bucket" , help = "The Amazon S3 bucket whose <code>OwnershipControls</code> you want to delete." , long_help = None)]
    bucket: String,
}
impl DeleteBucketOwnershipControlsCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .delete_bucket_ownership_controls()
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_bucket(Some(self.bucket.to_owned()))
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct ListObjectVersionsCommand {
    # [clap (long = "encoding-type" , name = "EncodingType" , help = "" , long_help = None)]
    encoding_type: Option<String>,
    # [clap (long = "bucket" , name = "Bucket" , help = "The bucket name that contains the objects." , long_help = None)]
    bucket: String,
    # [clap (long = "delimiter" , name = "Delimiter" , help = "A delimiter is a character that you specify to group keys. All keys that contain the same string between the <code>prefix</code> and the first occurrence of the delimiter are grouped under a single result element in CommonPrefixes. These groups are counted as one result against the max-keys limitation. These keys are not returned elsewhere in the response." , long_help = None)]
    delimiter: Option<String>,
    # [clap (long = "prefix" , name = "Prefix" , help = "Use this parameter to select only those keys that begin with the specified prefix. You can use prefixes to separate a bucket into different groupings of keys. (You can think of using prefix to make groups in the same way you'd use a folder in a file system.) You can use prefix with delimiter to roll up numerous objects into a single result under CommonPrefixes." , long_help = None)]
    prefix: Option<String>,
    # [clap (long = "max-keys" , name = "MaxKeys" , help = "Sets the maximum number of keys returned in the response. By default the action returns up to 1,000 key names. The response might contain fewer keys but will never contain more. If additional keys satisfy the search criteria, but were not returned because max-keys was exceeded, the response contains <isTruncated>true</isTruncated>. To return the additional keys, see key-marker and version-id-marker." , long_help = None)]
    max_keys: Option<i32>,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    # [clap (long = "key-marker" , name = "KeyMarker" , help = "Specifies the key to start with when listing objects in a bucket." , long_help = None)]
    key_marker: Option<String>,
    # [clap (long = "version-id-marker" , name = "VersionIdMarker" , help = "Specifies the object version you want to start listing from." , long_help = None)]
    version_id_marker: Option<String>,
}
impl ListObjectVersionsCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .list_object_versions()
            .set_encoding_type(None)
            .set_bucket(Some(self.bucket.to_owned()))
            .set_delimiter(self.delimiter.to_owned())
            .set_prefix(self.prefix.to_owned())
            .set_max_keys(self.max_keys)
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_key_marker(self.key_marker.to_owned())
            .set_version_id_marker(self.version_id_marker.to_owned())
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct GetBucketEncryptionCommand {
    # [clap (long = "bucket" , name = "Bucket" , help = "The name of the bucket from which the server-side encryption configuration is retrieved." , long_help = None)]
    bucket: String,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
}
impl GetBucketEncryptionCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .get_bucket_encryption()
            .set_bucket(Some(self.bucket.to_owned()))
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct DeleteBucketTaggingCommand {
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    # [clap (long = "bucket" , name = "Bucket" , help = "The bucket that has the tag set to be removed." , long_help = None)]
    bucket: String,
}
impl DeleteBucketTaggingCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .delete_bucket_tagging()
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_bucket(Some(self.bucket.to_owned()))
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct ListBucketMetricsConfigurationsCommand {
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    # [clap (long = "continuation-token" , name = "ContinuationToken" , help = "The marker that is used to continue a metrics configuration listing that has been truncated. Use the NextContinuationToken from a previously truncated list response to continue the listing. The continuation token is an opaque value that Amazon S3 understands." , long_help = None)]
    continuation_token: Option<String>,
    # [clap (long = "bucket" , name = "Bucket" , help = "The name of the bucket containing the metrics configurations to retrieve." , long_help = None)]
    bucket: String,
}
impl ListBucketMetricsConfigurationsCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .list_bucket_metrics_configurations()
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_continuation_token(self.continuation_token.to_owned())
            .set_bucket(Some(self.bucket.to_owned()))
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Args, Debug, Clone)]
pub struct AnalyticsConfigurationArgs {
    #[clap(long = "storage-class-analysis")]
    storage_class_analysis: Option<String>,
    #[clap(long = "id")]
    id: Option<String>,
    #[clap(long = "filter")]
    filter: Option<String>,
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct PutBucketAnalyticsConfigurationCommand {
    # [clap (long = "bucket" , name = "Bucket" , help = "The name of the bucket to which an analytics configuration is stored." , long_help = None)]
    bucket: String,
    # [clap (long = "id" , name = "Id" , help = "The ID that identifies the analytics configuration." , long_help = None)]
    id: String,
    #[clap(flatten)]
    analytics_configuration: AnalyticsConfigurationArgs,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
}
impl PutBucketAnalyticsConfigurationCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .put_bucket_analytics_configuration()
            .set_bucket(Some(self.bucket.to_owned()))
            .set_id(Some(self.id.to_owned()))
            .set_analytics_configuration(None)
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}

#[derive(clap :: Parser, Debug, Clone)]
pub struct ListObjectsCommand {
    # [clap (long = "bucket" , name = "Bucket" , help = "The name of the bucket containing the objects." , long_help = None)]
    bucket: String,
    # [clap (long = "delimiter" , name = "Delimiter" , help = "A delimiter is a character you use to group keys." , long_help = None)]
    delimiter: Option<String>,
    # [clap (long = "prefix" , name = "Prefix" , help = "Limits the response to keys that begin with the specified prefix." , long_help = None)]
    prefix: Option<String>,
    # [clap (long = "marker" , name = "Marker" , help = "Marker is where you want Amazon S3 to start listing from. Amazon S3 starts listing after this specified key. Marker can be any key in the bucket." , long_help = None)]
    marker: Option<String>,
    # [clap (long = "expected-bucket-owner" , name = "ExpectedBucketOwner" , help = "The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied)." , long_help = None)]
    expected_bucket_owner: Option<String>,
    # [clap (long = "max-keys" , name = "MaxKeys" , help = "Sets the maximum number of keys returned in the response. By default the action returns up to 1,000 key names. The response might contain fewer keys but will never contain more." , long_help = None)]
    max_keys: Option<i32>,
    # [clap (long = "request-payer" , name = "RequestPayer" , help = "Confirms that the requester knows that she or he will be charged for the list objects request. Bucket owners need not specify this parameter in their requests." , long_help = None)]
    request_payer: Option<String>,
    # [clap (long = "encoding-type" , name = "EncodingType" , help = "" , long_help = None)]
    encoding_type: Option<String>,
}
impl ListObjectsCommand {
    pub async fn run(&self, s3: &'static aws_sdk_s3::Client) {
        log::info!("{:#?}", self);
        let res = s3
            .list_objects()
            .set_bucket(Some(self.bucket.to_owned()))
            .set_delimiter(self.delimiter.to_owned())
            .set_prefix(self.prefix.to_owned())
            .set_marker(self.marker.to_owned())
            .set_expected_bucket_owner(self.expected_bucket_owner.to_owned())
            .set_max_keys(self.max_keys)
            .set_request_payer(None)
            .set_encoding_type(None)
            .send()
            .await;
        match res {
            Ok(out) => {
                log::info!("{:#?}", out);
            }
            Err(err) => match err {
                aws_smithy_http::result::SdkError::ServiceError { err, raw } => {
                    log::error!("{:#?}", err);
                }
                _ => {
                    log::error!("{:#?}", err);
                }
            },
        }
    }
}
