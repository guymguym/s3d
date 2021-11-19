//! TODO This module should be generated from https://github.com/awslabs/smithy-rs

use crate::types::*;
use aws_sdk_s3::output::*;
use aws_smithy_xml::encode::{ScopeWriter, XmlWriter};
use hyper::Body;

pub const S3_XMLNS: &'static str = "http://s3.amazonaws.com/doc/2006-03-01/";

/// OutputError are errors that can occur when parsing the input from the HTTP request
#[derive(Debug)]
pub enum OutputError {
    NotImplemented(&'static str),
    BadResponse(hyper::http::Error),
    Unhandled(anyhow::Error),
}

impl std::error::Error for OutputError {}

impl From<hyper::http::Error> for OutputError {
    fn from(err: hyper::http::Error) -> Self {
        OutputError::BadResponse(err)
    }
}

impl From<anyhow::Error> for OutputError {
    fn from(err: anyhow::Error) -> Self {
        OutputError::Unhandled(err)
    }
}

impl std::fmt::Display for OutputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputError::NotImplemented(msg) => write!(f, "NotImplemented({})", msg),
            OutputError::BadResponse(err) => write!(f, "BadResponse({})", err),
            OutputError::Unhandled(err) => write!(f, "Unhandled({})", err),
        }
    }
}

fn xml_root<'a, 'b>(x: &'a mut XmlWriter<'_>, root: &'b str) -> ScopeWriter<'a, 'b> {
    x.start_el(root).write_ns(S3_XMLNS, None).finish()
}
fn xml_elem<'a, 'b>(w: &'a mut ScopeWriter<'_, '_>, name: &'b str) -> ScopeWriter<'a, 'b> {
    w.start_el(name).write_ns(S3_XMLNS, None).finish()
}
fn xml_text<T: AsRef<str>>(w: &mut ScopeWriter, tag: &str, data: T) {
    let mut el = xml_elem(w, tag);
    el.data(data.as_ref());
    el.finish();
}
fn xml_text_opt<'a, 'b, T: AsRef<str>>(w: &mut ScopeWriter, tag: &str, opt_text: Option<T>) {
    if let Some(text) = opt_text {
        xml_text(w, tag, text)
    }
}

pub fn s3_error_meta_output(e: S3ErrorMeta) -> S3Result {
    let mut out = String::new();
    let mut x = XmlWriter::new(&mut out);
    let mut w = xml_root(&mut x, "Error");
    xml_text_opt(&mut w, "Code", e.code());
    xml_text_opt(&mut w, "Message", e.message());
    xml_text_opt(&mut w, "RequestId", e.request_id());
    xml_text_opt(&mut w, "Resource", e.extra("resource"));
    w.finish();
    Ok(responder().body(Body::from(out)).unwrap())
}

pub mod parsers {
    pub use super::not_implemented::*;
    use super::*;

    pub fn list_buckets(o: ListBucketsOutput) -> Result<HttpResponse, OutputError> {
        let mut out = String::new();
        let mut xml = XmlWriter::new(&mut out);
        let mut w = xml_root(&mut xml, "ListAllMyBucketsResult");
        {
            let mut w = xml_elem(&mut w, "Buckets");
            for b in o.buckets.unwrap_or_default() {
                let mut w = xml_elem(&mut w, "Bucket");
                xml_text_opt(&mut w, "Name", b.name);
                xml_text_opt(
                    &mut w,
                    "CreationDate",
                    b.creation_date.map(|t| t.to_chrono().to_rfc3339()),
                );
                w.finish();
            }
            w.finish();
        }
        {
            let owner = o.owner.unwrap();
            let mut w = xml_elem(&mut w, "Owner");
            xml_text_opt(&mut w, "ID", owner.id);
            xml_text_opt(&mut w, "DisplayName", owner.display_name);
            w.finish();
        }
        w.finish();
        Ok(responder().body(Body::from(out)).unwrap())
    }

    pub fn list_objects(o: ListObjectsOutput) -> Result<HttpResponse, OutputError> {
        let mut out = String::new();
        let mut xml = XmlWriter::new(&mut out);
        let mut w = xml_root(&mut xml, "ListBucketResult");
        xml_text_opt(&mut w, "Name", o.name);
        xml_text_opt(&mut w, "Prefix", o.prefix);
        xml_text_opt(&mut w, "Delimiter", o.delimiter);
        xml_text_opt(&mut w, "Marker", o.marker);
        xml_text_opt(&mut w, "EncodingType", o.encoding_type);
        xml_text(&mut w, "MaxKeys", o.max_keys.to_string());
        xml_text(&mut w, "IsTruncated", o.is_truncated.to_string());
        xml_text_opt(&mut w, "NextMarker", o.next_marker);

        for obj in o.contents.unwrap_or_default() {
            let mut w = xml_elem(&mut w, "Contents");
            xml_text_opt(&mut w, "Key", obj.key);
            xml_text_opt(
                &mut w,
                "LastModified",
                obj.last_modified.map(|t| t.to_chrono().to_rfc3339()),
            );
            xml_text_opt(&mut w, "ETag", obj.e_tag);
            xml_text(&mut w, "Size", obj.size.to_string());
            xml_text_opt(&mut w, "StorageClass", obj.storage_class);
            {
                let owner = obj.owner.unwrap();
                let mut w = xml_elem(&mut w, "Owner");
                xml_text_opt(&mut w, "ID", owner.id);
                xml_text_opt(&mut w, "DisplayName", owner.display_name);
                w.finish();
            }
            w.finish();
        }
        for p in o.common_prefixes.unwrap_or_default() {
            let mut w = xml_elem(&mut w, "CommonPrefixes");
            xml_text_opt(&mut w, "Prefix", p.prefix);
            w.finish();
        }
        w.finish();
        Ok(responder().body(Body::from(out)).unwrap())
    }

    pub fn get_object(o: GetObjectOutput) -> Result<HttpResponse, OutputError> {
        // let body = Body::from(o.body.collect().await);
        Ok(responder().body(Body::from("get_object")).unwrap())
    }
}

mod not_implemented {
    use super::*;

    /// This macro generates a default parser function per op.
    macro_rules! s3_out {
        ($name:ident) => {
            paste::paste! {
                pub fn [<$name:snake>](o: [<$name Output>]) -> Result<HttpResponse, OutputError> {
                    Err(OutputError::NotImplemented(stringify!(s3::output::parsers::[<$name:snake>])))
                }
            }
        };
    }

    s3_out!(AbortMultipartUpload);
    s3_out!(CompleteMultipartUpload);
    s3_out!(CopyObject);
    s3_out!(CreateBucket);
    s3_out!(CreateMultipartUpload);
    s3_out!(DeleteBucket);
    s3_out!(DeleteBucketAnalyticsConfiguration);
    s3_out!(DeleteBucketCors);
    s3_out!(DeleteBucketEncryption);
    s3_out!(DeleteBucketIntelligentTieringConfiguration);
    s3_out!(DeleteBucketInventoryConfiguration);
    s3_out!(DeleteBucketLifecycle);
    s3_out!(DeleteBucketMetricsConfiguration);
    s3_out!(DeleteBucketOwnershipControls);
    s3_out!(DeleteBucketPolicy);
    s3_out!(DeleteBucketReplication);
    s3_out!(DeleteBucketTagging);
    s3_out!(DeleteBucketWebsite);
    s3_out!(DeleteObject);
    s3_out!(DeleteObjects);
    s3_out!(DeleteObjectTagging);
    s3_out!(DeletePublicAccessBlock);
    s3_out!(GetBucketAccelerateConfiguration);
    s3_out!(GetBucketAcl);
    s3_out!(GetBucketAnalyticsConfiguration);
    s3_out!(GetBucketCors);
    s3_out!(GetBucketEncryption);
    s3_out!(GetBucketIntelligentTieringConfiguration);
    s3_out!(GetBucketInventoryConfiguration);
    s3_out!(GetBucketLifecycleConfiguration);
    s3_out!(GetBucketLocation);
    s3_out!(GetBucketLogging);
    s3_out!(GetBucketMetricsConfiguration);
    s3_out!(GetBucketNotificationConfiguration);
    s3_out!(GetBucketOwnershipControls);
    s3_out!(GetBucketPolicy);
    s3_out!(GetBucketPolicyStatus);
    s3_out!(GetBucketReplication);
    s3_out!(GetBucketRequestPayment);
    s3_out!(GetBucketTagging);
    s3_out!(GetBucketVersioning);
    s3_out!(GetBucketWebsite);
    s3_out!(GetObject);
    s3_out!(GetObjectAcl);
    s3_out!(GetObjectLegalHold);
    s3_out!(GetObjectLockConfiguration);
    s3_out!(GetObjectRetention);
    s3_out!(GetObjectTagging);
    s3_out!(GetObjectTorrent);
    s3_out!(GetPublicAccessBlock);
    s3_out!(HeadBucket);
    s3_out!(HeadObject);
    s3_out!(ListBucketAnalyticsConfigurations);
    s3_out!(ListBucketIntelligentTieringConfigurations);
    s3_out!(ListBucketInventoryConfigurations);
    s3_out!(ListBucketMetricsConfigurations);
    s3_out!(ListBuckets);
    s3_out!(ListMultipartUploads);
    s3_out!(ListObjects);
    s3_out!(ListObjectsV2);
    s3_out!(ListObjectVersions);
    s3_out!(ListParts);
    s3_out!(PutBucketAccelerateConfiguration);
    s3_out!(PutBucketAcl);
    s3_out!(PutBucketAnalyticsConfiguration);
    s3_out!(PutBucketCors);
    s3_out!(PutBucketEncryption);
    s3_out!(PutBucketIntelligentTieringConfiguration);
    s3_out!(PutBucketInventoryConfiguration);
    s3_out!(PutBucketLifecycleConfiguration);
    s3_out!(PutBucketLogging);
    s3_out!(PutBucketMetricsConfiguration);
    s3_out!(PutBucketNotificationConfiguration);
    s3_out!(PutBucketOwnershipControls);
    s3_out!(PutBucketPolicy);
    s3_out!(PutBucketReplication);
    s3_out!(PutBucketRequestPayment);
    s3_out!(PutBucketTagging);
    s3_out!(PutBucketVersioning);
    s3_out!(PutBucketWebsite);
    s3_out!(PutObject);
    s3_out!(PutObjectAcl);
    s3_out!(PutObjectLegalHold);
    s3_out!(PutObjectLockConfiguration);
    s3_out!(PutObjectRetention);
    s3_out!(PutObjectTagging);
    s3_out!(PutPublicAccessBlock);
    s3_out!(RestoreObject);
    s3_out!(SelectObjectContent);
    s3_out!(UploadPart);
    s3_out!(UploadPartCopy);
    s3_out!(WriteGetObjectResponse);
}
