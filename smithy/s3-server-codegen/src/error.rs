// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Error type for the `AbortMultipartUpload` operation.
/// Each variant represents an error that can occur for the `AbortMultipartUpload` operation.
#[derive(std::fmt::Debug)]
pub enum AbortMultipartUploadError {
    /// <p>The specified multipart upload does not exist.</p>
    NoSuchUpload(crate::error::NoSuchUpload),
}
impl std::fmt::Display for AbortMultipartUploadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            AbortMultipartUploadError::NoSuchUpload(_inner) => _inner.fmt(f),
        }
    }
}
impl AbortMultipartUploadError {
    /// Returns `true` if the error kind is `AbortMultipartUploadError::NoSuchUpload`.
    pub fn is_no_such_upload(&self) -> bool {
        matches!(&self, AbortMultipartUploadError::NoSuchUpload(_))
    }
    /// Returns the error name string by matching the correct variant.
    pub fn name(&self) -> &'static str {
        match &self {
            AbortMultipartUploadError::NoSuchUpload(_inner) => _inner.name(),
        }
    }
}
impl std::error::Error for AbortMultipartUploadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self {
            AbortMultipartUploadError::NoSuchUpload(_inner) => Some(_inner),
        }
    }
}

/// Error type for the `CopyObject` operation.
/// Each variant represents an error that can occur for the `CopyObject` operation.
#[derive(std::fmt::Debug)]
pub enum CopyObjectError {
    /// <p>The source object of the COPY action is not in the active tier and is only stored in
    /// Amazon S3 Glacier.</p>
    ObjectNotInActiveTierError(crate::error::ObjectNotInActiveTierError),
}
impl std::fmt::Display for CopyObjectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            CopyObjectError::ObjectNotInActiveTierError(_inner) => _inner.fmt(f),
        }
    }
}
impl CopyObjectError {
    /// Returns `true` if the error kind is `CopyObjectError::ObjectNotInActiveTierError`.
    pub fn is_object_not_in_active_tier_error(&self) -> bool {
        matches!(&self, CopyObjectError::ObjectNotInActiveTierError(_))
    }
    /// Returns the error name string by matching the correct variant.
    pub fn name(&self) -> &'static str {
        match &self {
            CopyObjectError::ObjectNotInActiveTierError(_inner) => _inner.name(),
        }
    }
}
impl std::error::Error for CopyObjectError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self {
            CopyObjectError::ObjectNotInActiveTierError(_inner) => Some(_inner),
        }
    }
}

/// Error type for the `CreateBucket` operation.
/// Each variant represents an error that can occur for the `CreateBucket` operation.
#[derive(std::fmt::Debug)]
pub enum CreateBucketError {
    /// <p>The bucket you tried to create already exists, and you own it. Amazon S3 returns this error
    /// in all Amazon Web Services Regions except in the North Virginia Region. For legacy compatibility, if you
    /// re-create an existing bucket that you already own in the North Virginia Region, Amazon S3
    /// returns 200 OK and resets the bucket access control lists (ACLs).</p>
    BucketAlreadyOwnedByYou(crate::error::BucketAlreadyOwnedByYou),
    /// <p>The requested bucket name is not available. The bucket namespace is shared by all users
    /// of the system. Select a different name and try again.</p>
    BucketAlreadyExists(crate::error::BucketAlreadyExists),
}
impl std::fmt::Display for CreateBucketError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            CreateBucketError::BucketAlreadyOwnedByYou(_inner) => _inner.fmt(f),
            CreateBucketError::BucketAlreadyExists(_inner) => _inner.fmt(f),
        }
    }
}
impl CreateBucketError {
    /// Returns `true` if the error kind is `CreateBucketError::BucketAlreadyOwnedByYou`.
    pub fn is_bucket_already_owned_by_you(&self) -> bool {
        matches!(&self, CreateBucketError::BucketAlreadyOwnedByYou(_))
    }
    /// Returns `true` if the error kind is `CreateBucketError::BucketAlreadyExists`.
    pub fn is_bucket_already_exists(&self) -> bool {
        matches!(&self, CreateBucketError::BucketAlreadyExists(_))
    }
    /// Returns the error name string by matching the correct variant.
    pub fn name(&self) -> &'static str {
        match &self {
            CreateBucketError::BucketAlreadyOwnedByYou(_inner) => _inner.name(),
            CreateBucketError::BucketAlreadyExists(_inner) => _inner.name(),
        }
    }
}
impl std::error::Error for CreateBucketError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self {
            CreateBucketError::BucketAlreadyOwnedByYou(_inner) => Some(_inner),
            CreateBucketError::BucketAlreadyExists(_inner) => Some(_inner),
        }
    }
}

/// Error type for the `GetObject` operation.
/// Each variant represents an error that can occur for the `GetObject` operation.
#[derive(std::fmt::Debug)]
pub enum GetObjectError {
    /// <p>The specified key does not exist.</p>
    NoSuchKey(crate::error::NoSuchKey),
    /// <p>Object is archived and inaccessible until restored.</p>
    InvalidObjectState(crate::error::InvalidObjectState),
}
impl std::fmt::Display for GetObjectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            GetObjectError::NoSuchKey(_inner) => _inner.fmt(f),
            GetObjectError::InvalidObjectState(_inner) => _inner.fmt(f),
        }
    }
}
impl GetObjectError {
    /// Returns `true` if the error kind is `GetObjectError::NoSuchKey`.
    pub fn is_no_such_key(&self) -> bool {
        matches!(&self, GetObjectError::NoSuchKey(_))
    }
    /// Returns `true` if the error kind is `GetObjectError::InvalidObjectState`.
    pub fn is_invalid_object_state(&self) -> bool {
        matches!(&self, GetObjectError::InvalidObjectState(_))
    }
    /// Returns the error name string by matching the correct variant.
    pub fn name(&self) -> &'static str {
        match &self {
            GetObjectError::NoSuchKey(_inner) => _inner.name(),
            GetObjectError::InvalidObjectState(_inner) => _inner.name(),
        }
    }
}
impl std::error::Error for GetObjectError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self {
            GetObjectError::NoSuchKey(_inner) => Some(_inner),
            GetObjectError::InvalidObjectState(_inner) => Some(_inner),
        }
    }
}

/// Error type for the `GetObjectAcl` operation.
/// Each variant represents an error that can occur for the `GetObjectAcl` operation.
#[derive(std::fmt::Debug)]
pub enum GetObjectAclError {
    /// <p>The specified key does not exist.</p>
    NoSuchKey(crate::error::NoSuchKey),
}
impl std::fmt::Display for GetObjectAclError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            GetObjectAclError::NoSuchKey(_inner) => _inner.fmt(f),
        }
    }
}
impl GetObjectAclError {
    /// Returns `true` if the error kind is `GetObjectAclError::NoSuchKey`.
    pub fn is_no_such_key(&self) -> bool {
        matches!(&self, GetObjectAclError::NoSuchKey(_))
    }
    /// Returns the error name string by matching the correct variant.
    pub fn name(&self) -> &'static str {
        match &self {
            GetObjectAclError::NoSuchKey(_inner) => _inner.name(),
        }
    }
}
impl std::error::Error for GetObjectAclError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self {
            GetObjectAclError::NoSuchKey(_inner) => Some(_inner),
        }
    }
}

/// Error type for the `HeadBucket` operation.
/// Each variant represents an error that can occur for the `HeadBucket` operation.
#[derive(std::fmt::Debug)]
pub enum HeadBucketError {
    /// <p>The specified content does not exist.</p>
    NotFound(crate::error::NotFound),
}
impl std::fmt::Display for HeadBucketError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            HeadBucketError::NotFound(_inner) => _inner.fmt(f),
        }
    }
}
impl HeadBucketError {
    /// Returns `true` if the error kind is `HeadBucketError::NotFound`.
    pub fn is_not_found(&self) -> bool {
        matches!(&self, HeadBucketError::NotFound(_))
    }
    /// Returns the error name string by matching the correct variant.
    pub fn name(&self) -> &'static str {
        match &self {
            HeadBucketError::NotFound(_inner) => _inner.name(),
        }
    }
}
impl std::error::Error for HeadBucketError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self {
            HeadBucketError::NotFound(_inner) => Some(_inner),
        }
    }
}

/// Error type for the `HeadObject` operation.
/// Each variant represents an error that can occur for the `HeadObject` operation.
#[derive(std::fmt::Debug)]
pub enum HeadObjectError {
    /// <p>The specified content does not exist.</p>
    NotFound(crate::error::NotFound),
}
impl std::fmt::Display for HeadObjectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            HeadObjectError::NotFound(_inner) => _inner.fmt(f),
        }
    }
}
impl HeadObjectError {
    /// Returns `true` if the error kind is `HeadObjectError::NotFound`.
    pub fn is_not_found(&self) -> bool {
        matches!(&self, HeadObjectError::NotFound(_))
    }
    /// Returns the error name string by matching the correct variant.
    pub fn name(&self) -> &'static str {
        match &self {
            HeadObjectError::NotFound(_inner) => _inner.name(),
        }
    }
}
impl std::error::Error for HeadObjectError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self {
            HeadObjectError::NotFound(_inner) => Some(_inner),
        }
    }
}

/// Error type for the `ListObjects` operation.
/// Each variant represents an error that can occur for the `ListObjects` operation.
#[derive(std::fmt::Debug)]
pub enum ListObjectsError {
    /// <p>The specified bucket does not exist.</p>
    NoSuchBucket(crate::error::NoSuchBucket),
}
impl std::fmt::Display for ListObjectsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ListObjectsError::NoSuchBucket(_inner) => _inner.fmt(f),
        }
    }
}
impl ListObjectsError {
    /// Returns `true` if the error kind is `ListObjectsError::NoSuchBucket`.
    pub fn is_no_such_bucket(&self) -> bool {
        matches!(&self, ListObjectsError::NoSuchBucket(_))
    }
    /// Returns the error name string by matching the correct variant.
    pub fn name(&self) -> &'static str {
        match &self {
            ListObjectsError::NoSuchBucket(_inner) => _inner.name(),
        }
    }
}
impl std::error::Error for ListObjectsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self {
            ListObjectsError::NoSuchBucket(_inner) => Some(_inner),
        }
    }
}

/// Error type for the `ListObjectsV2` operation.
/// Each variant represents an error that can occur for the `ListObjectsV2` operation.
#[derive(std::fmt::Debug)]
pub enum ListObjectsV2Error {
    /// <p>The specified bucket does not exist.</p>
    NoSuchBucket(crate::error::NoSuchBucket),
}
impl std::fmt::Display for ListObjectsV2Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ListObjectsV2Error::NoSuchBucket(_inner) => _inner.fmt(f),
        }
    }
}
impl ListObjectsV2Error {
    /// Returns `true` if the error kind is `ListObjectsV2Error::NoSuchBucket`.
    pub fn is_no_such_bucket(&self) -> bool {
        matches!(&self, ListObjectsV2Error::NoSuchBucket(_))
    }
    /// Returns the error name string by matching the correct variant.
    pub fn name(&self) -> &'static str {
        match &self {
            ListObjectsV2Error::NoSuchBucket(_inner) => _inner.name(),
        }
    }
}
impl std::error::Error for ListObjectsV2Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self {
            ListObjectsV2Error::NoSuchBucket(_inner) => Some(_inner),
        }
    }
}

/// Error type for the `PutObjectAcl` operation.
/// Each variant represents an error that can occur for the `PutObjectAcl` operation.
#[derive(std::fmt::Debug)]
pub enum PutObjectAclError {
    /// <p>The specified key does not exist.</p>
    NoSuchKey(crate::error::NoSuchKey),
}
impl std::fmt::Display for PutObjectAclError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            PutObjectAclError::NoSuchKey(_inner) => _inner.fmt(f),
        }
    }
}
impl PutObjectAclError {
    /// Returns `true` if the error kind is `PutObjectAclError::NoSuchKey`.
    pub fn is_no_such_key(&self) -> bool {
        matches!(&self, PutObjectAclError::NoSuchKey(_))
    }
    /// Returns the error name string by matching the correct variant.
    pub fn name(&self) -> &'static str {
        match &self {
            PutObjectAclError::NoSuchKey(_inner) => _inner.name(),
        }
    }
}
impl std::error::Error for PutObjectAclError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self {
            PutObjectAclError::NoSuchKey(_inner) => Some(_inner),
        }
    }
}

/// Error type for the `RestoreObject` operation.
/// Each variant represents an error that can occur for the `RestoreObject` operation.
#[derive(std::fmt::Debug)]
pub enum RestoreObjectError {
    /// <p>This action is not allowed against this storage tier.</p>
    ObjectAlreadyInActiveTierError(crate::error::ObjectAlreadyInActiveTierError),
}
impl std::fmt::Display for RestoreObjectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            RestoreObjectError::ObjectAlreadyInActiveTierError(_inner) => _inner.fmt(f),
        }
    }
}
impl RestoreObjectError {
    /// Returns `true` if the error kind is `RestoreObjectError::ObjectAlreadyInActiveTierError`.
    pub fn is_object_already_in_active_tier_error(&self) -> bool {
        matches!(&self, RestoreObjectError::ObjectAlreadyInActiveTierError(_))
    }
    /// Returns the error name string by matching the correct variant.
    pub fn name(&self) -> &'static str {
        match &self {
            RestoreObjectError::ObjectAlreadyInActiveTierError(_inner) => _inner.name(),
        }
    }
}
impl std::error::Error for RestoreObjectError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self {
            RestoreObjectError::ObjectAlreadyInActiveTierError(_inner) => Some(_inner),
        }
    }
}

/// <p>This action is not allowed against this storage tier.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ObjectAlreadyInActiveTierError {}
impl std::fmt::Debug for ObjectAlreadyInActiveTierError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ObjectAlreadyInActiveTierError");
        formatter.finish()
    }
}
impl ObjectAlreadyInActiveTierError {
    /// Returns the error message.
    pub fn message(&self) -> Option<&str> {
        None
    }
    #[doc(hidden)]
    /// Returns the error name.
    pub fn name(&self) -> &'static str {
        "ObjectAlreadyInActiveTierError"
    }
}
impl std::fmt::Display for ObjectAlreadyInActiveTierError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ObjectAlreadyInActiveTierError")?;
        Ok(())
    }
}
impl std::error::Error for ObjectAlreadyInActiveTierError {}
/// See [`ObjectAlreadyInActiveTierError`](crate::error::ObjectAlreadyInActiveTierError)
pub mod object_already_in_active_tier_error {
    /// A builder for [`ObjectAlreadyInActiveTierError`](crate::error::ObjectAlreadyInActiveTierError)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`ObjectAlreadyInActiveTierError`](crate::error::ObjectAlreadyInActiveTierError)
        pub fn build(self) -> crate::error::ObjectAlreadyInActiveTierError {
            crate::error::ObjectAlreadyInActiveTierError {}
        }
    }
}
impl ObjectAlreadyInActiveTierError {
    /// Creates a new builder-style object to manufacture [`ObjectAlreadyInActiveTierError`](crate::error::ObjectAlreadyInActiveTierError)
    pub fn builder() -> crate::error::object_already_in_active_tier_error::Builder {
        crate::error::object_already_in_active_tier_error::Builder::default()
    }
}

/// <p>The specified key does not exist.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct NoSuchKey {}
impl std::fmt::Debug for NoSuchKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("NoSuchKey");
        formatter.finish()
    }
}
impl NoSuchKey {
    /// Returns the error message.
    pub fn message(&self) -> Option<&str> {
        None
    }
    #[doc(hidden)]
    /// Returns the error name.
    pub fn name(&self) -> &'static str {
        "NoSuchKey"
    }
}
impl std::fmt::Display for NoSuchKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NoSuchKey")?;
        Ok(())
    }
}
impl std::error::Error for NoSuchKey {}
/// See [`NoSuchKey`](crate::error::NoSuchKey)
pub mod no_such_key {
    /// A builder for [`NoSuchKey`](crate::error::NoSuchKey)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`NoSuchKey`](crate::error::NoSuchKey)
        pub fn build(self) -> crate::error::NoSuchKey {
            crate::error::NoSuchKey {}
        }
    }
}
impl NoSuchKey {
    /// Creates a new builder-style object to manufacture [`NoSuchKey`](crate::error::NoSuchKey)
    pub fn builder() -> crate::error::no_such_key::Builder {
        crate::error::no_such_key::Builder::default()
    }
}

/// <p>The specified bucket does not exist.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct NoSuchBucket {}
impl std::fmt::Debug for NoSuchBucket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("NoSuchBucket");
        formatter.finish()
    }
}
impl NoSuchBucket {
    /// Returns the error message.
    pub fn message(&self) -> Option<&str> {
        None
    }
    #[doc(hidden)]
    /// Returns the error name.
    pub fn name(&self) -> &'static str {
        "NoSuchBucket"
    }
}
impl std::fmt::Display for NoSuchBucket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NoSuchBucket")?;
        Ok(())
    }
}
impl std::error::Error for NoSuchBucket {}
/// See [`NoSuchBucket`](crate::error::NoSuchBucket)
pub mod no_such_bucket {
    /// A builder for [`NoSuchBucket`](crate::error::NoSuchBucket)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`NoSuchBucket`](crate::error::NoSuchBucket)
        pub fn build(self) -> crate::error::NoSuchBucket {
            crate::error::NoSuchBucket {}
        }
    }
}
impl NoSuchBucket {
    /// Creates a new builder-style object to manufacture [`NoSuchBucket`](crate::error::NoSuchBucket)
    pub fn builder() -> crate::error::no_such_bucket::Builder {
        crate::error::no_such_bucket::Builder::default()
    }
}

/// <p>The specified content does not exist.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct NotFound {}
impl std::fmt::Debug for NotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("NotFound");
        formatter.finish()
    }
}
impl NotFound {
    /// Returns the error message.
    pub fn message(&self) -> Option<&str> {
        None
    }
    #[doc(hidden)]
    /// Returns the error name.
    pub fn name(&self) -> &'static str {
        "NotFound"
    }
}
impl std::fmt::Display for NotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NotFound")?;
        Ok(())
    }
}
impl std::error::Error for NotFound {}
/// See [`NotFound`](crate::error::NotFound)
pub mod not_found {
    /// A builder for [`NotFound`](crate::error::NotFound)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`NotFound`](crate::error::NotFound)
        pub fn build(self) -> crate::error::NotFound {
            crate::error::NotFound {}
        }
    }
}
impl NotFound {
    /// Creates a new builder-style object to manufacture [`NotFound`](crate::error::NotFound)
    pub fn builder() -> crate::error::not_found::Builder {
        crate::error::not_found::Builder::default()
    }
}

/// <p>Object is archived and inaccessible until restored.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct InvalidObjectState {
    #[allow(missing_docs)] // documentation missing in model
    pub storage_class: std::option::Option<crate::model::StorageClass>,
    #[allow(missing_docs)] // documentation missing in model
    pub access_tier: std::option::Option<crate::model::IntelligentTieringAccessTier>,
}
impl InvalidObjectState {
    #[allow(missing_docs)] // documentation missing in model
    pub fn storage_class(&self) -> std::option::Option<&crate::model::StorageClass> {
        self.storage_class.as_ref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn access_tier(&self) -> std::option::Option<&crate::model::IntelligentTieringAccessTier> {
        self.access_tier.as_ref()
    }
}
impl std::fmt::Debug for InvalidObjectState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("InvalidObjectState");
        formatter.field("storage_class", &self.storage_class);
        formatter.field("access_tier", &self.access_tier);
        formatter.finish()
    }
}
impl InvalidObjectState {
    /// Returns the error message.
    pub fn message(&self) -> Option<&str> {
        None
    }
    #[doc(hidden)]
    /// Returns the error name.
    pub fn name(&self) -> &'static str {
        "InvalidObjectState"
    }
}
impl std::fmt::Display for InvalidObjectState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InvalidObjectState")?;
        Ok(())
    }
}
impl std::error::Error for InvalidObjectState {}
/// See [`InvalidObjectState`](crate::error::InvalidObjectState)
pub mod invalid_object_state {
    /// A builder for [`InvalidObjectState`](crate::error::InvalidObjectState)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) storage_class: std::option::Option<crate::model::StorageClass>,
        pub(crate) access_tier: std::option::Option<crate::model::IntelligentTieringAccessTier>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn storage_class(mut self, input: crate::model::StorageClass) -> Self {
            self.storage_class = Some(input);
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_storage_class(
            mut self,
            input: std::option::Option<crate::model::StorageClass>,
        ) -> Self {
            self.storage_class = input;
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn access_tier(mut self, input: crate::model::IntelligentTieringAccessTier) -> Self {
            self.access_tier = Some(input);
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_access_tier(
            mut self,
            input: std::option::Option<crate::model::IntelligentTieringAccessTier>,
        ) -> Self {
            self.access_tier = input;
            self
        }
        /// Consumes the builder and constructs a [`InvalidObjectState`](crate::error::InvalidObjectState)
        pub fn build(self) -> crate::error::InvalidObjectState {
            crate::error::InvalidObjectState {
                storage_class: self.storage_class,
                access_tier: self.access_tier,
            }
        }
    }
}
impl InvalidObjectState {
    /// Creates a new builder-style object to manufacture [`InvalidObjectState`](crate::error::InvalidObjectState)
    pub fn builder() -> crate::error::invalid_object_state::Builder {
        crate::error::invalid_object_state::Builder::default()
    }
}

/// <p>The requested bucket name is not available. The bucket namespace is shared by all users
/// of the system. Select a different name and try again.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct BucketAlreadyExists {}
impl std::fmt::Debug for BucketAlreadyExists {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("BucketAlreadyExists");
        formatter.finish()
    }
}
impl BucketAlreadyExists {
    /// Returns the error message.
    pub fn message(&self) -> Option<&str> {
        None
    }
    #[doc(hidden)]
    /// Returns the error name.
    pub fn name(&self) -> &'static str {
        "BucketAlreadyExists"
    }
}
impl std::fmt::Display for BucketAlreadyExists {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BucketAlreadyExists")?;
        Ok(())
    }
}
impl std::error::Error for BucketAlreadyExists {}
/// See [`BucketAlreadyExists`](crate::error::BucketAlreadyExists)
pub mod bucket_already_exists {
    /// A builder for [`BucketAlreadyExists`](crate::error::BucketAlreadyExists)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`BucketAlreadyExists`](crate::error::BucketAlreadyExists)
        pub fn build(self) -> crate::error::BucketAlreadyExists {
            crate::error::BucketAlreadyExists {}
        }
    }
}
impl BucketAlreadyExists {
    /// Creates a new builder-style object to manufacture [`BucketAlreadyExists`](crate::error::BucketAlreadyExists)
    pub fn builder() -> crate::error::bucket_already_exists::Builder {
        crate::error::bucket_already_exists::Builder::default()
    }
}

/// <p>The bucket you tried to create already exists, and you own it. Amazon S3 returns this error
/// in all Amazon Web Services Regions except in the North Virginia Region. For legacy compatibility, if you
/// re-create an existing bucket that you already own in the North Virginia Region, Amazon S3
/// returns 200 OK and resets the bucket access control lists (ACLs).</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct BucketAlreadyOwnedByYou {}
impl std::fmt::Debug for BucketAlreadyOwnedByYou {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("BucketAlreadyOwnedByYou");
        formatter.finish()
    }
}
impl BucketAlreadyOwnedByYou {
    /// Returns the error message.
    pub fn message(&self) -> Option<&str> {
        None
    }
    #[doc(hidden)]
    /// Returns the error name.
    pub fn name(&self) -> &'static str {
        "BucketAlreadyOwnedByYou"
    }
}
impl std::fmt::Display for BucketAlreadyOwnedByYou {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BucketAlreadyOwnedByYou")?;
        Ok(())
    }
}
impl std::error::Error for BucketAlreadyOwnedByYou {}
/// See [`BucketAlreadyOwnedByYou`](crate::error::BucketAlreadyOwnedByYou)
pub mod bucket_already_owned_by_you {
    /// A builder for [`BucketAlreadyOwnedByYou`](crate::error::BucketAlreadyOwnedByYou)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`BucketAlreadyOwnedByYou`](crate::error::BucketAlreadyOwnedByYou)
        pub fn build(self) -> crate::error::BucketAlreadyOwnedByYou {
            crate::error::BucketAlreadyOwnedByYou {}
        }
    }
}
impl BucketAlreadyOwnedByYou {
    /// Creates a new builder-style object to manufacture [`BucketAlreadyOwnedByYou`](crate::error::BucketAlreadyOwnedByYou)
    pub fn builder() -> crate::error::bucket_already_owned_by_you::Builder {
        crate::error::bucket_already_owned_by_you::Builder::default()
    }
}

/// <p>The source object of the COPY action is not in the active tier and is only stored in
/// Amazon S3 Glacier.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ObjectNotInActiveTierError {}
impl std::fmt::Debug for ObjectNotInActiveTierError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ObjectNotInActiveTierError");
        formatter.finish()
    }
}
impl ObjectNotInActiveTierError {
    /// Returns the error message.
    pub fn message(&self) -> Option<&str> {
        None
    }
    #[doc(hidden)]
    /// Returns the error name.
    pub fn name(&self) -> &'static str {
        "ObjectNotInActiveTierError"
    }
}
impl std::fmt::Display for ObjectNotInActiveTierError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ObjectNotInActiveTierError")?;
        Ok(())
    }
}
impl std::error::Error for ObjectNotInActiveTierError {}
/// See [`ObjectNotInActiveTierError`](crate::error::ObjectNotInActiveTierError)
pub mod object_not_in_active_tier_error {
    /// A builder for [`ObjectNotInActiveTierError`](crate::error::ObjectNotInActiveTierError)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`ObjectNotInActiveTierError`](crate::error::ObjectNotInActiveTierError)
        pub fn build(self) -> crate::error::ObjectNotInActiveTierError {
            crate::error::ObjectNotInActiveTierError {}
        }
    }
}
impl ObjectNotInActiveTierError {
    /// Creates a new builder-style object to manufacture [`ObjectNotInActiveTierError`](crate::error::ObjectNotInActiveTierError)
    pub fn builder() -> crate::error::object_not_in_active_tier_error::Builder {
        crate::error::object_not_in_active_tier_error::Builder::default()
    }
}

/// <p>The specified multipart upload does not exist.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct NoSuchUpload {}
impl std::fmt::Debug for NoSuchUpload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("NoSuchUpload");
        formatter.finish()
    }
}
impl NoSuchUpload {
    /// Returns the error message.
    pub fn message(&self) -> Option<&str> {
        None
    }
    #[doc(hidden)]
    /// Returns the error name.
    pub fn name(&self) -> &'static str {
        "NoSuchUpload"
    }
}
impl std::fmt::Display for NoSuchUpload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NoSuchUpload")?;
        Ok(())
    }
}
impl std::error::Error for NoSuchUpload {}
/// See [`NoSuchUpload`](crate::error::NoSuchUpload)
pub mod no_such_upload {
    /// A builder for [`NoSuchUpload`](crate::error::NoSuchUpload)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`NoSuchUpload`](crate::error::NoSuchUpload)
        pub fn build(self) -> crate::error::NoSuchUpload {
            crate::error::NoSuchUpload {}
        }
    }
}
impl NoSuchUpload {
    /// Creates a new builder-style object to manufacture [`NoSuchUpload`](crate::error::NoSuchUpload)
    pub fn builder() -> crate::error::no_such_upload::Builder {
        crate::error::no_such_upload::Builder::default()
    }
}