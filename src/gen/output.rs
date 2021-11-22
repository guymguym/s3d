//! TODO This module should be generated from https://github.com/awslabs/smithy-rs

use crate::{err::OutputError, gen::constants::*, http::*, xml::*};
use aws_sdk_s3::output::*;
use aws_smithy_types::instant::Format;
use hyper::{Body, StatusCode};

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

//---------------------------------------------------//
// basic bucket ops                                  //
//---------------------------------------------------//
//-------------------------------//
// gen!(HeadBucket);
// gen!(CreateBucket);
// gen!(DeleteBucket);
// gen!(GetBucketLocation);
//-------------------------------//

pub fn head_bucket(_o: HeadBucketOutput) -> Result<HttpResponse, OutputError> {
    Ok(responder()
        .status(StatusCode::OK)
        .body(Body::empty())
        .unwrap())
}

pub fn create_bucket(o: CreateBucketOutput) -> Result<HttpResponse, OutputError> {
    let mut r = responder().status(StatusCode::OK);
    let h = r.headers_mut().unwrap();
    o.location().set_header(h, H_LOCATION);
    Ok(r.body(Body::empty()).unwrap())
}

pub fn delete_bucket(_o: DeleteBucketOutput) -> Result<HttpResponse, OutputError> {
    Ok(responder()
        .status(StatusCode::NO_CONTENT)
        .body(Body::empty())
        .unwrap())
}

pub fn get_bucket_location(o: GetBucketLocationOutput) -> Result<HttpResponse, OutputError> {
    let mut out = String::new();
    let mut xml = XmlWriter::new(&mut out);
    let mut w = xml_root(&mut xml, "LocationConstraint");
    xml_text_opt(&mut w, "LocationConstraint", o.location_constraint);
    w.finish();
    Ok(responder()
        .status(StatusCode::OK)
        .body(Body::from(out))
        .unwrap())
}

//---------------------------------------------------//
// basic object ops                                  //
//---------------------------------------------------//
//-------------------------------//
// gen!(GetObject);
// gen!(HeadObject);
gen!(PutObject);
gen!(CopyObject);
gen!(DeleteObject);
gen!(DeleteObjects);
//-------------------------------//

pub fn get_object(o: GetObjectOutput) -> Result<HttpResponse, OutputError> {
    let mut r = responder().status(StatusCode::OK);
    let h = r.headers_mut().unwrap();
    // http headers
    o.content_type().set_header(h, H_CONTENT_TYPE);
    o.content_range().set_header(h, H_CONTENT_RANGE);
    o.content_length().set_header(h, H_CONTENT_LENGTH);
    o.content_disposition().set_header(h, H_CONTENT_DISPOSITION);
    o.content_encoding().set_header(h, H_CONTENT_ENCODING);
    o.content_language().set_header(h, H_CONTENT_LANGUAGE);
    o.e_tag().set_header(h, H_ETAG);
    o.expires().set_header(h, H_EXPIRES);
    o.last_modified().set_header(h, H_LAST_MODIFIED);
    o.accept_ranges().set_header(h, H_ACCEPT_RANGES);
    o.cache_control().set_header(h, H_CACHE_CONTROL);
    // x-amz headers
    o.bucket_key_enabled()
        .set_header(h, X_AMZ_SSE_BUCKET_KEY_ENABLED);
    o.delete_marker().set_header(h, X_AMZ_DELETE_MARKER);
    o.expiration().set_header(h, X_AMZ_EXPIRATION);
    if let Some(md) = o.metadata() {
        for (key, val) in md {
            val.set_header_non_static(h, format!("{}{}", X_AMZ_META_PREFIX, key).as_str());
        }
    }
    if o.missing_meta() > 0 {
        o.missing_meta().set_header(h, X_AMZ_MISSING_META);
    }
    o.object_lock_legal_hold_status()
        .set_header(h, X_AMZ_OBJECT_LOCK_LEGAL_HOLD);
    o.object_lock_mode().set_header(h, X_AMZ_OBJECT_LOCK_MODE);
    o.object_lock_retain_until_date()
        .set_header(h, X_AMZ_OBJECT_LOCK_RETAIN_UNTIL_DATE);
    o.parts_count().set_header(h, X_AMZ_MP_PARTS_COUNT);
    o.replication_status()
        .set_header(h, X_AMZ_REPLICAITON_STATUS);
    o.request_charged().set_header(h, X_AMZ_REQUEST_CHARGED);
    o.restore().set_header(h, X_AMZ_RESTORE);
    o.server_side_encryption().set_header(h, X_AMZ_SSE);
    o.sse_customer_algorithm()
        .set_header(h, X_AMZ_SSE_CUSTOMER_ALG);
    o.sse_customer_key_md5()
        .set_header(h, X_AMZ_SSE_CUSTOMER_KEY_MD5);
    o.ssekms_key_id().set_header(h, X_AMZ_SSE_AWS_KMS_KEY_ID);
    o.storage_class().set_header(h, X_AMZ_STORAGE_CLASS);
    o.tag_count().set_header(h, X_AMZ_TAGGING_COUNT);
    o.version_id().set_header(h, X_AMZ_VERSION_ID);
    o.website_redirect_location()
        .set_header(h, X_AMZ_WEBSITE_REDIRECT_LOCATION);
    Ok(r.body(Body::wrap_stream(o.body)).unwrap())
}

pub fn head_object(o: HeadObjectOutput) -> Result<HttpResponse, OutputError> {
    let mut r = responder().status(StatusCode::OK);
    let h = r.headers_mut().unwrap();
    // http headers
    o.content_type().set_header(h, H_CONTENT_TYPE);
    o.content_length().set_header(h, H_CONTENT_LENGTH);
    o.content_disposition().set_header(h, H_CONTENT_DISPOSITION);
    o.content_encoding().set_header(h, H_CONTENT_ENCODING);
    o.content_language().set_header(h, H_CONTENT_LANGUAGE);
    o.e_tag().set_header(h, H_ETAG);
    o.expires().set_header(h, H_EXPIRES);
    o.last_modified().set_header(h, H_LAST_MODIFIED);
    o.accept_ranges().set_header(h, H_ACCEPT_RANGES);
    o.cache_control().set_header(h, H_CACHE_CONTROL);
    // x-amz headers
    o.archive_status().set_header(h, X_AMZ_ARCHIVE_STATUS);
    o.bucket_key_enabled()
        .set_header(h, X_AMZ_SSE_BUCKET_KEY_ENABLED);
    o.delete_marker().set_header(h, X_AMZ_DELETE_MARKER);
    o.expiration().set_header(h, X_AMZ_EXPIRATION);
    if let Some(md) = o.metadata() {
        for (key, val) in md {
            val.set_header_non_static(h, format!("{}{}", X_AMZ_META_PREFIX, key).as_str());
        }
    }
    if o.missing_meta() > 0 {
        o.missing_meta().set_header(h, X_AMZ_MISSING_META);
    }
    o.object_lock_legal_hold_status()
        .set_header(h, X_AMZ_OBJECT_LOCK_LEGAL_HOLD);
    o.object_lock_mode().set_header(h, X_AMZ_OBJECT_LOCK_MODE);
    o.object_lock_retain_until_date()
        .set_header(h, X_AMZ_OBJECT_LOCK_RETAIN_UNTIL_DATE);
    o.parts_count().set_header(h, X_AMZ_MP_PARTS_COUNT);
    o.replication_status()
        .set_header(h, X_AMZ_REPLICAITON_STATUS);
    o.request_charged().set_header(h, X_AMZ_REQUEST_CHARGED);
    o.restore().set_header(h, X_AMZ_RESTORE);
    o.server_side_encryption().set_header(h, X_AMZ_SSE);
    o.sse_customer_algorithm()
        .set_header(h, X_AMZ_SSE_CUSTOMER_ALG);
    o.sse_customer_key_md5()
        .set_header(h, X_AMZ_SSE_CUSTOMER_KEY_MD5);
    o.ssekms_key_id().set_header(h, X_AMZ_SSE_AWS_KMS_KEY_ID);
    o.storage_class().set_header(h, X_AMZ_STORAGE_CLASS);
    o.version_id().set_header(h, X_AMZ_VERSION_ID);
    o.website_redirect_location()
        .set_header(h, X_AMZ_WEBSITE_REDIRECT_LOCATION);
    Ok(r.body(Body::empty()).unwrap())
}

//---------------------------------------------------//
// multipart upload ops                              //
//---------------------------------------------------//
//-------------------------------//
gen!(CreateMultipartUpload);
gen!(CompleteMultipartUpload);
gen!(AbortMultipartUpload);
gen!(UploadPart);
gen!(UploadPartCopy);
//-------------------------------//

//---------------------------------------------------//
// list ops                                          //
//---------------------------------------------------//
//-------------------------------//
// gen!(ListBuckets);
// gen!(ListObjects);
gen!(ListObjectsV2);
gen!(ListObjectVersions);
gen!(ListMultipartUploads);
gen!(ListParts);
//-------------------------------//

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

//---------------------------------------------------//
// advanced object ops                               //
//---------------------------------------------------//
//-------------------------------//
gen!(GetObjectAcl);
gen!(PutObjectAcl);
//-------------------------------//
gen!(GetObjectTagging);
gen!(PutObjectTagging);
gen!(DeleteObjectTagging);
//-------------------------------//
gen!(GetObjectRetention);
gen!(PutObjectRetention);
//-------------------------------//
gen!(GetObjectLegalHold);
gen!(PutObjectLegalHold);
//-------------------------------//
gen!(RestoreObject);
gen!(GetObjectTorrent);
gen!(SelectObjectContent);
//-------------------------------//

//---------------------------------------------------//
// advanced bucket ops                               //
//---------------------------------------------------//
//-------------------------------//
gen!(GetBucketAcl);
gen!(PutBucketAcl);
//-------------------------------//
gen!(GetBucketTagging);
gen!(PutBucketTagging);
gen!(DeleteBucketTagging);
//-------------------------------//
gen!(GetObjectLockConfiguration);
gen!(PutObjectLockConfiguration);
//-------------------------------//
gen!(GetBucketCors);
gen!(PutBucketCors);
gen!(DeleteBucketCors);
//-------------------------------//
gen!(GetBucketPolicy);
gen!(PutBucketPolicy);
gen!(DeleteBucketPolicy);
gen!(GetBucketPolicyStatus);
//-------------------------------//
gen!(GetBucketReplication);
gen!(PutBucketReplication);
gen!(DeleteBucketReplication);
//-------------------------------//
gen!(GetBucketEncryption);
gen!(PutBucketEncryption);
gen!(DeleteBucketEncryption);
//-------------------------------//
gen!(GetBucketWebsite);
gen!(PutBucketWebsite);
gen!(DeleteBucketWebsite);
//-------------------------------//
gen!(GetBucketLifecycleConfiguration);
gen!(PutBucketLifecycleConfiguration);
gen!(DeleteBucketLifecycle);
//-------------------------------//
gen!(GetBucketLogging);
gen!(PutBucketLogging);
//-------------------------------//
gen!(GetBucketVersioning);
gen!(PutBucketVersioning);
//-------------------------------//
gen!(GetBucketRequestPayment);
gen!(PutBucketRequestPayment);
//-------------------------------//
gen!(GetBucketAccelerateConfiguration);
gen!(PutBucketAccelerateConfiguration);
//-------------------------------//
gen!(GetBucketNotificationConfiguration);
gen!(PutBucketNotificationConfiguration);
//-------------------------------//
gen!(GetBucketAnalyticsConfiguration);
gen!(PutBucketAnalyticsConfiguration);
gen!(DeleteBucketAnalyticsConfiguration);
gen!(ListBucketAnalyticsConfigurations);
//-------------------------------//
gen!(GetBucketIntelligentTieringConfiguration);
gen!(PutBucketIntelligentTieringConfiguration);
gen!(DeleteBucketIntelligentTieringConfiguration);
gen!(ListBucketIntelligentTieringConfigurations);
//-------------------------------//
gen!(GetBucketInventoryConfiguration);
gen!(PutBucketInventoryConfiguration);
gen!(DeleteBucketInventoryConfiguration);
gen!(ListBucketInventoryConfigurations);
//-------------------------------//
gen!(GetBucketMetricsConfiguration);
gen!(PutBucketMetricsConfiguration);
gen!(DeleteBucketMetricsConfiguration);
gen!(ListBucketMetricsConfigurations);
//-------------------------------//
gen!(GetBucketOwnershipControls);
gen!(PutBucketOwnershipControls);
gen!(DeleteBucketOwnershipControls);
//-------------------------------//
gen!(GetPublicAccessBlock);
gen!(PutPublicAccessBlock);
gen!(DeletePublicAccessBlock);
//-------------------------------//
gen!(WriteGetObjectResponse);
//-------------------------------//
