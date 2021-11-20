//! TODO This module should be generated from https://github.com/awslabs/smithy-rs

#[derive(Debug)]
pub enum S3Resource {
    Service,
    Bucket(S3BucketResource),
    Object(S3ObjectResource),
}

impl S3Resource {
    pub fn get_bucket(&self) -> &str {
        match self {
            S3Resource::Bucket(b) => b.bucket.as_str(),
            S3Resource::Object(o) => o.bucket.as_str(),
            _ => panic!("Expected bucket resource type: {:?}", self),
        }
    }
    pub fn get_key(&self) -> &str {
        match self {
            S3Resource::Object(o) => o.key.as_str(),
            _ => panic!("Expected object resource type: {:?}", self),
        }
    }
}

#[derive(Debug)]
pub struct S3BucketResource {
    pub bucket: String,
    pub sub_resource: S3BucketSubResource,
}

#[derive(Debug)]
pub struct S3ObjectResource {
    pub bucket: String,
    pub key: String,
    pub sub_resource: S3ObjectSubResource,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum S3BucketSubResource {
    None,
    Accelerate,
    Acl,
    Analytics,
    Cors,
    Encryption,
    IntelligentTiering,
    Inventory,
    Lifecycle,
    Location,
    Logging,
    Metrics,
    Notification,
    OwnershipControls,
    Policy,
    PolicyStatus,
    PublicAccessBlock,
    Replication,
    RequestPayment,
    Tagging,
    Versioning,
    Website,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum S3ObjectSubResource {
    None,
    Acl,
    LegalHold,
    ObjectLock,
    Restore,
    Retention,
    SelectObjectContent,
    Tagging,
    Torrent,
    Uploads,
    UploadId,
    Versions,
}

impl From<&str> for S3BucketSubResource {
    fn from(s: &str) -> Self {
        match s {
            "accelerate" => Self::Accelerate,
            "acl" => Self::Acl,
            "analytics" => Self::Analytics,
            "cors" => Self::Cors,
            "encryption" => Self::Encryption,
            "intelligent-tiering" => Self::IntelligentTiering,
            "inventory" => Self::Inventory,
            "lifecycle" => Self::Lifecycle,
            "location" => Self::Location,
            "logging" => Self::Logging,
            "metrics" => Self::Metrics,
            "notification" => Self::Notification,
            "ownershipControls" => Self::OwnershipControls,
            "policy" => Self::Policy,
            "policyStatus" => Self::PolicyStatus,
            "publicAccessBlock" => Self::PublicAccessBlock,
            "replication" => Self::Replication,
            "requestPayment" => Self::RequestPayment,
            "tagging" => Self::Tagging,
            "versioning" => Self::Versioning,
            "website" => Self::Website,
            _ => Self::None,
        }
    }
}

impl From<&str> for S3ObjectSubResource {
    fn from(s: &str) -> Self {
        match s {
            "acl" => Self::Acl,
            "legal-hold" => Self::LegalHold,
            "object-lock" => Self::ObjectLock,
            "restore" => Self::Restore,
            "retention" => Self::Retention,
            "select" => Self::SelectObjectContent,
            "tagging" => Self::Tagging,
            "torrent" => Self::Torrent,
            "uploads" => Self::Uploads,
            "uploadId" => Self::UploadId,
            "versions" => Self::Versions,
            _ => Self::None,
        }
    }
}
