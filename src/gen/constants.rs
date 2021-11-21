pub const P_PREFIX: &str = "prefix";
pub const P_DELIMITER: &str = "delimiter";
pub const P_MAX_KEYS: &str = "max-keys";
pub const P_MAX_UPLOADS: &str = "max-uploads";
pub const P_MAX_PARTS: &str = "max-parts";
pub const P_FETCH_OWNER: &str = "fetch-owner";
pub const P_START_AFTER: &str = "start-after";
pub const P_ENCODING_TYPE: &str = "encoding-type";
pub const P_CONTINUATION_TOKEN: &str = "continuation-token";
pub const P_MARKER: &str = "marker";
pub const P_KEY_MARKER: &str = "key-marker";
pub const P_VERSION_ID: &str = "versionId";
pub const P_VERSION_ID_MARKER: &str = "version-id-marker";
pub const P_UPLOAD_ID: &str = "uploadId";
pub const P_UPLOAD_ID_MARKER: &str = "upload-id-marker";
pub const P_PART_NUMBER: &str = "partNumber";
pub const P_PART_NUMBER_MARKER: &str = "part-number-marker";

pub const P_RESPONSE_CACHE_CONTROL: &str = "response-cache-control";
pub const P_RESPONSE_CONTENT_DISPOSITION: &str = "response-content-disposition";
pub const P_RESPONSE_CONTENT_ENCODING: &str = "response-content-encoding";
pub const P_RESPONSE_CONTENT_LANGUAGE: &str = "response-content-language";
pub const P_RESPONSE_CONTENT_TYPE: &str = "response-content-type";
pub const P_RESPONSE_EXPIRES: &str = "response-expires";

pub const H_RANGE: &str = "Range";
pub const H_EXPIRES: &str = "Expires";
pub const H_CACHE_CONTROL: &str = "Cache-Control";
pub const H_CONTENT_MD5: &str = "Content-MD5";
pub const H_CONTENT_TYPE: &str = "Content-Type";
pub const H_CONTENT_LENGTH: &str = "Content-Length";
pub const H_CONTENT_ENCODING: &str = "Content-Encoding";
pub const H_CONTENT_LANGUAGE: &str = "Content-Language";
pub const H_CONTENT_DISPOSITION: &str = "Content-Disposition";

pub const H_IF_MATCH: &str = "If-Match";
pub const H_IF_NONE_MATCH: &str = "If-None-Match";
pub const H_IF_MODIFIED_SINCE: &str = "If-Modified-Since";
pub const H_IF_UNMODIFIED_SINCE: &str = "If-Unmodified-Since";

pub const X_AMZ_ACL: &str = "x-amz-acl";
pub const X_AMZ_DATE: &str = "x-amz-date";
pub const X_AMZ_TAGGING: &str = "x-amz-tagging";
pub const X_AMZ_META_PREFIX: &str = "x-amz-meta-";
pub const X_AMZ_STORAGE_CLASS: &str = "x-amz-storage-class";
pub const X_AMZ_REQUEST_PAYER: &str = "x-amz-request-payer";
pub const X_AMZ_EXPECTED_BUCKET_OWNER: &str = "x-amz-expected-bucket-owner";
pub const X_AMZ_WEBSITE_REDIRECT_LOCATION: &str = "x-amz-website-redirect-location";

pub const X_AMZ_MFA: &str = "x-amz-mfa";
pub const X_AMZ_OBJECT_LOCK_MODE: &str = "x-amz-object-lock-mode";
pub const X_AMZ_OBJECT_LOCK_LEGAL_HOLD: &str = "x-amz-object-lock-legal-hold";
pub const X_AMZ_OBJECT_LOCK_RETAIN_UNTIL_DATE: &str = "x-amz-object-lock-retain-until-date";
pub const X_AMZ_BUCKET_OBJECT_LOCK_ENABLED: &str = "x-amz-bucket-object-lock-enabled";
pub const X_AMZ_BUCKET_OBJECT_LOCK_TOKEN: &str = "x-amz-bucket-object-lock-token";
pub const X_AMZ_BYPASS_GOVERNANCE_RETENTION: &str = "x-amz-bypass-governance-retention";

pub const X_AMZ_GRANT_FULL_CONTROL: &str = "x-amz-grant-full-control";
pub const X_AMZ_GRANT_READ: &str = "x-amz-grant-read";
pub const X_AMZ_GRANT_READ_ACP: &str = "x-amz-grant-read-acp";
pub const X_AMZ_GRANT_WRITE: &str = "x-amz-grant-write";
pub const X_AMZ_GRANT_WRITE_ACP: &str = "x-amz-grant-write-acp";

pub const X_AMZ_SSE: &str = "x-amz-server-side-encryption";
pub const X_AMZ_SSE_CUSTOMER_ALG: &str = "x-amz-server-side-encryption-customer-algorithm";
pub const X_AMZ_SSE_CUSTOMER_KEY: &str = "x-amz-server-side-encryption-customer-key";
pub const X_AMZ_SSE_CUSTOMER_KEY_MD5: &str = "x-amz-server-side-encryption-customer-key-MD5";
pub const X_AMZ_SSE_AWS_KMS_KEY_ID: &str = "x-amz-server-side-encryption-aws-kms-key-id";
pub const X_AMZ_SSE_CONTEXT: &str = "x-amz-server-side-encryption-context";
pub const X_AMZ_SSE_BUCKET_KEY_ENABLED: &str = "x-amz-server-side-encryption-bucket-key-enabled";

pub const X_AMZ_COPY_SOURCE: &str = "x-amz-copy-source";
pub const X_AMZ_COPY_SOURCE_RANGE: &str = "x-amz-copy-source-range";
pub const X_AMZ_COPY_SOURCE_IF_MATCH: &str = "x-amz-copy-source-if-match";
pub const X_AMZ_COPY_SOURCE_IF_MODIFIED_SINCE: &str = "x-amz-copy-source-if-modified-since";
pub const X_AMZ_COPY_SOURCE_IF_NONE_MATCH: &str = "x-amz-copy-source-if-none-match";
pub const X_AMZ_COPY_SOURCE_IF_UNMODIFIED_SINCE: &str = "x-amz-copy-source-if-unmodified-since";
pub const X_AMZ_COPY_SOURCE_SSE_CUSTOMER_ALG: &str =
    "x-amz-copy-source-server-side-encryption-customer-algorithm";
pub const X_AMZ_COPY_SOURCE_SSE_CUSTOMER_KEY: &str =
    "x-amz-copy-source-server-side-encryption-customer-key";
pub const X_AMZ_COPY_SOURCE_SSE_CUSTOMER_KEY_MD5: &str =
    "x-amz-copy-source-server-side-encryption-customer-key-MD5";
pub const X_AMZ_METADATA_DIRECTIVE: &str = "x-amz-metadata-directive";
pub const X_AMZ_TAGGING_DIRECTIVE: &str = "x-amz-tagging-directive";
pub const X_AMZ_SOURCE_EXPECTED_BUCKET_OWNER: &str = "x-amz-source-expected-bucket-owner";
