pub const P_DELIMITER: &str = "delimiter";
pub const P_MARKER: &str = "marker";
pub const P_MAX_KEYS: &str = "max-keys";
pub const P_PREFIX: &str = "prefix";
pub const P_VERSION_ID: &str = "versionId";
pub const P_PART_NUMBER: &str = "partNumber";
pub const P_ENCODING_TYPE: &str = "encoding-type";

pub const P_RESPONSE_CACHE_CONTROL: &str = "response-cache-control";
pub const P_RESPONSE_CONTENT_DISPOSITION: &str = "response-content-disposition";
pub const P_RESPONSE_CONTENT_ENCODING: &str = "response-content-encoding";
pub const P_RESPONSE_CONTENT_LANGUAGE: &str = "response-content-language";
pub const P_RESPONSE_CONTENT_TYPE: &str = "response-content-type";
pub const P_RESPONSE_EXPIRES: &str = "response-expires";

pub const H_RANGE: &str = "Range";
pub const H_IF_MATCH: &str = "If-Match";
pub const H_IF_NONE_MATCH: &str = "If-None-Match";
pub const H_IF_MODIFIED_SINCE: &str = "If-Modified-Since";
pub const H_IF_UNMODIFIED_SINCE: &str = "If-Unmodified-Since";

pub const X_AMZ_DATE: &str = "x-amz-date";
pub const X_AMZ_ACL: &str = "x-amz-acl";
pub const X_AMZ_GRANT_FULL_CONTROL: &str = "x-amz-grant-full-control";
pub const X_AMZ_GRANT_READ: &str = "x-amz-grant-read";
pub const X_AMZ_GRANT_READ_ACP: &str = "x-amz-grant-read-acp";
pub const X_AMZ_GRANT_WRITE: &str = "x-amz-grant-write";
pub const X_AMZ_GRANT_WRITE_ACP: &str = "x-amz-grant-write-acp";
pub const X_AMZ_BUCKET_OBJECT_LOCK_ENABLED: &str = "x-amz-bucket-object-lock-enabled";
pub const X_AMZ_REQUEST_PAYER: &str = "x-amz-request-payer";
pub const X_AMZ_EXPECTED_BUCKET_OWNER: &str = "x-amz-expected-bucket-owner";
pub const X_AMZ_SSE_CUSTOMER_ALG: &str = "x-amz-server-side-encryption-customer-algorithm";
pub const X_AMZ_SSE_CUSTOMER_KEY_MD5: &str = "x-amz-server-side-encryption-customer-key-MD5";
pub const X_AMZ_SSE_AWS_KMS_KEY_ID: &str = "x-amz-server-side-encryption-aws-kms-key-id";