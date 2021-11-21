//! TODO This module should be generated from https://github.com/awslabs/smithy-rs

use crate::types::*;
use aws_sdk_s3::output::*;
use aws_smithy_types::instant::Format;
use aws_smithy_xml::encode::{ScopeWriter, XmlWriter};
use hyper::{Body, StatusCode};

pub const S3_XMLNS: &'static str = "http://s3.amazonaws.com/doc/2006-03-01/";

/// OutputError are errors that can occur when parsing the input from the HTTP request
#[derive(Debug)]
pub enum OutputError {
    NotImplemented(&'static str),
    BadResponse(hyper::http::Error),
    Unhandled(anyhow::Error),
}
impl std::error::Error for OutputError {}
impl std::fmt::Display for OutputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputError::NotImplemented(msg) => write!(f, "NotImplemented({})", msg),
            OutputError::BadResponse(err) => write!(f, "BadResponse({})", err),
            OutputError::Unhandled(err) => write!(f, "Unhandled({})", err),
        }
    }
}
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

pub fn s3_error_output(e: S3Error) -> HttpResponse {
    let mut out = String::new();
    let mut x = XmlWriter::new(&mut out);
    let mut w = xml_root(&mut x, "Error");
    xml_text_opt(&mut w, "Code", e.code());
    xml_text_opt(&mut w, "Message", e.message());
    xml_text_opt(&mut w, "RequestId", e.request_id());
    xml_text_opt(&mut w, "Resource", e.extra("resource"));
    w.finish();
    responder()
        .status(StatusCode::BAD_REQUEST)
        .body(Body::from(out))
        .unwrap()
}

pub mod parsers {

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
                    b.creation_date.map(|t| t.fmt(Format::DateTime)),
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
                obj.last_modified.map(|t| t.fmt(Format::DateTime)),
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

    pub fn get_object(_o: GetObjectOutput) -> Result<HttpResponse, OutputError> {
        // let body = Body::from(o.body.collect().await);
        Ok(responder().body(Body::from("get_object")).unwrap())
    }

    /// This macro generates a default parser function per op
    /// to make it possible to implement it in stages.
    macro_rules! gen {
        ($name:ident) => {
            paste::paste! {
                pub fn [<$name:snake>](_: [<$name Output>]) -> Result<HttpResponse, OutputError> {
                    Err(OutputError::NotImplemented(stringify!(s3::output::parsers::[<$name:snake>])))
                }
            }
        };
    }

    gen!(AbortMultipartUpload);
    gen!(CompleteMultipartUpload);
    gen!(CopyObject);
    gen!(CreateBucket);
    gen!(CreateMultipartUpload);
    gen!(DeleteBucket);
    gen!(DeleteBucketAnalyticsConfiguration);
    gen!(DeleteBucketCors);
    gen!(DeleteBucketEncryption);
    gen!(DeleteBucketIntelligentTieringConfiguration);
    gen!(DeleteBucketInventoryConfiguration);
    gen!(DeleteBucketLifecycle);
    gen!(DeleteBucketMetricsConfiguration);
    gen!(DeleteBucketOwnershipControls);
    gen!(DeleteBucketPolicy);
    gen!(DeleteBucketReplication);
    gen!(DeleteBucketTagging);
    gen!(DeleteBucketWebsite);
    gen!(DeleteObject);
    gen!(DeleteObjects);
    gen!(DeleteObjectTagging);
    gen!(DeletePublicAccessBlock);
    gen!(GetBucketAccelerateConfiguration);
    gen!(GetBucketAcl);
    gen!(GetBucketAnalyticsConfiguration);
    gen!(GetBucketCors);
    gen!(GetBucketEncryption);
    gen!(GetBucketIntelligentTieringConfiguration);
    gen!(GetBucketInventoryConfiguration);
    gen!(GetBucketLifecycleConfiguration);
    gen!(GetBucketLocation);
    gen!(GetBucketLogging);
    gen!(GetBucketMetricsConfiguration);
    gen!(GetBucketNotificationConfiguration);
    gen!(GetBucketOwnershipControls);
    gen!(GetBucketPolicy);
    gen!(GetBucketPolicyStatus);
    gen!(GetBucketReplication);
    gen!(GetBucketRequestPayment);
    gen!(GetBucketTagging);
    gen!(GetBucketVersioning);
    gen!(GetBucketWebsite);
    // gen!(GetObject);
    gen!(GetObjectAcl);
    gen!(GetObjectLegalHold);
    gen!(GetObjectLockConfiguration);
    gen!(GetObjectRetention);
    gen!(GetObjectTagging);
    gen!(GetObjectTorrent);
    gen!(GetPublicAccessBlock);
    gen!(HeadBucket);
    gen!(HeadObject);
    gen!(ListBucketAnalyticsConfigurations);
    gen!(ListBucketIntelligentTieringConfigurations);
    gen!(ListBucketInventoryConfigurations);
    gen!(ListBucketMetricsConfigurations);
    // gen!(ListBuckets);
    gen!(ListMultipartUploads);
    // gen!(ListObjects);
    gen!(ListObjectsV2);
    gen!(ListObjectVersions);
    gen!(ListParts);
    gen!(PutBucketAccelerateConfiguration);
    gen!(PutBucketAcl);
    gen!(PutBucketAnalyticsConfiguration);
    gen!(PutBucketCors);
    gen!(PutBucketEncryption);
    gen!(PutBucketIntelligentTieringConfiguration);
    gen!(PutBucketInventoryConfiguration);
    gen!(PutBucketLifecycleConfiguration);
    gen!(PutBucketLogging);
    gen!(PutBucketMetricsConfiguration);
    gen!(PutBucketNotificationConfiguration);
    gen!(PutBucketOwnershipControls);
    gen!(PutBucketPolicy);
    gen!(PutBucketReplication);
    gen!(PutBucketRequestPayment);
    gen!(PutBucketTagging);
    gen!(PutBucketVersioning);
    gen!(PutBucketWebsite);
    gen!(PutObject);
    gen!(PutObjectAcl);
    gen!(PutObjectLegalHold);
    gen!(PutObjectLockConfiguration);
    gen!(PutObjectRetention);
    gen!(PutObjectTagging);
    gen!(PutPublicAccessBlock);
    gen!(RestoreObject);
    gen!(SelectObjectContent);
    gen!(UploadPart);
    gen!(UploadPartCopy);
    gen!(WriteGetObjectResponse);
}
