//! This module implements a function per S3 op to encode the output as HTTP response.
//! Currently written by hand which is difficult to maintain long term.
//! TODO This module should be generated from https://github.com/awslabs/smithy-rs.

use crate::{manual::headers::*, http::*, xml::*};
use aws_sdk_s3::output::*;
use hyper::{Body, StatusCode};

//---------------------------------------------------//
// basic bucket ops                                  //
//---------------------------------------------------//

pub fn head_bucket(_o: HeadBucketOutput) -> Result<HttpResponse, S3Error> {
    let r = responder();
    Ok(r.body(Body::empty()).unwrap())
}

pub fn create_bucket(o: CreateBucketOutput) -> Result<HttpResponse, S3Error> {
    let mut r = responder();
    let h = r.headers_mut().unwrap();
    o.location().set_header(h, H_LOCATION);
    Ok(r.body(Body::empty()).unwrap())
}

pub fn delete_bucket(_o: DeleteBucketOutput) -> Result<HttpResponse, S3Error> {
    let r = responder().status(StatusCode::NO_CONTENT);
    Ok(r.body(Body::empty())?)
}

pub fn get_bucket_location(o: GetBucketLocationOutput) -> Result<HttpResponse, S3Error> {
    let xstr = xml_doc!("LocationConstraint", w, {
        xml_text(&mut w, "LocationConstraint", o.location_constraint);
    });
    Ok(responder().body(Body::from(xstr)).unwrap())
}

//---------------------------------------------------//
// basic object ops                                  //
//---------------------------------------------------//

pub fn get_object(o: GetObjectOutput) -> Result<HttpResponse, S3Error> {
    let mut r = responder();
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

pub fn head_object(o: HeadObjectOutput) -> Result<HttpResponse, S3Error> {
    let mut r = responder();
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

pub fn put_object(o: PutObjectOutput) -> Result<HttpResponse, S3Error> {
    let mut r = responder();
    let h = r.headers_mut().unwrap();
    // http headers
    o.e_tag().set_header(h, H_ETAG);
    // x-amz headers
    o.bucket_key_enabled()
        .set_header(h, X_AMZ_SSE_BUCKET_KEY_ENABLED);
    o.expiration().set_header(h, X_AMZ_EXPIRATION);
    o.request_charged().set_header(h, X_AMZ_REQUEST_CHARGED);
    o.server_side_encryption().set_header(h, X_AMZ_SSE);
    o.sse_customer_algorithm()
        .set_header(h, X_AMZ_SSE_CUSTOMER_ALG);
    o.sse_customer_key_md5()
        .set_header(h, X_AMZ_SSE_CUSTOMER_KEY_MD5);
    o.ssekms_encryption_context()
        .set_header(h, X_AMZ_SSE_CONTEXT);
    o.ssekms_key_id().set_header(h, X_AMZ_SSE_AWS_KMS_KEY_ID);
    o.version_id().set_header(h, X_AMZ_VERSION_ID);
    Ok(r.body(Body::empty()).unwrap())
}

pub fn copy_object(o: CopyObjectOutput) -> Result<HttpResponse, S3Error> {
    let mut r = responder();
    let h = r.headers_mut().unwrap();
    // x-amz headers
    o.bucket_key_enabled()
        .set_header(h, X_AMZ_SSE_BUCKET_KEY_ENABLED);
    o.copy_source_version_id()
        .set_header(h, X_AMZ_COPY_SOURCE_VERSION_ID);
    o.expiration().set_header(h, X_AMZ_EXPIRATION);
    o.request_charged().set_header(h, X_AMZ_REQUEST_CHARGED);
    o.server_side_encryption().set_header(h, X_AMZ_SSE);
    o.sse_customer_algorithm()
        .set_header(h, X_AMZ_SSE_CUSTOMER_ALG);
    o.sse_customer_key_md5()
        .set_header(h, X_AMZ_SSE_CUSTOMER_KEY_MD5);
    o.ssekms_encryption_context()
        .set_header(h, X_AMZ_SSE_CONTEXT);
    o.ssekms_key_id().set_header(h, X_AMZ_SSE_AWS_KMS_KEY_ID);
    o.version_id().set_header(h, X_AMZ_VERSION_ID);
    let xstr = xml_doc!("CopyObjectResult", w, {
        if let Some(res) = o.copy_object_result {
            xml_date(&mut w, "LastModified", res.last_modified);
            xml_text(&mut w, "ETag", res.e_tag);
        }
    });
    Ok(r.body(Body::from(xstr)).unwrap())
}

pub fn delete_object(o: DeleteObjectOutput) -> Result<HttpResponse, S3Error> {
    let mut r = responder().status(StatusCode::NO_CONTENT);
    let h = r.headers_mut().unwrap();
    o.delete_marker().set_header(h, X_AMZ_DELETE_MARKER);
    o.request_charged().set_header(h, X_AMZ_REQUEST_CHARGED);
    o.version_id().set_header(h, X_AMZ_VERSION_ID);
    Ok(r.body(Body::empty()).unwrap())
}

pub fn delete_objects(o: DeleteObjectsOutput) -> Result<HttpResponse, S3Error> {
    let mut r = responder();
    let h = r.headers_mut().unwrap();
    o.request_charged().set_header(h, X_AMZ_REQUEST_CHARGED);
    let xstr = xml_doc!("DeleteResult", w, {
        for d in o.deleted.unwrap_or_default() {
            xml_tag!("Deleted", w, {
                xml_text(&mut w, "DeleteMarker", Some(d.delete_marker.to_string()));
                xml_text(&mut w, "DeleteMarkerVersionId", d.delete_marker_version_id);
                xml_text(&mut w, "Key", d.key);
                xml_text(&mut w, "VersionId", d.version_id);
            });
        }
        for e in o.errors.unwrap_or_default() {
            xml_tag!("Error", w, {
                xml_text(&mut w, "Code", e.code);
                xml_text(&mut w, "Key", e.key);
                xml_text(&mut w, "Message", e.message);
                xml_text(&mut w, "VersionId", e.version_id);
            });
        }
    });
    Ok(r.body(Body::from(xstr)).unwrap())
}

//---------------------------------------------------//
// multipart upload ops                              //
//---------------------------------------------------//

//---------------------------------------------------//
// list ops                                          //
//---------------------------------------------------//

pub fn list_buckets(o: ListBucketsOutput) -> Result<HttpResponse, S3Error> {
    let xstr = xml_doc!("ListAllMyBucketsResult", w, {
        xml_tag!("Buckets", w, {
            for b in o.buckets.unwrap_or_default() {
                xml_tag!("Bucket", w, {
                    xml_text(&mut w, "Name", b.name);
                    xml_date(&mut w, "CreationDate", b.creation_date);
                });
            }
        });
        xml_owner(&mut w, o.owner);
    });
    Ok(responder().body(Body::from(xstr)).unwrap())
}

pub fn list_objects(o: ListObjectsOutput) -> Result<HttpResponse, S3Error> {
    let xstr = xml_doc!("ListBucketResult", w, {
        xml_text(&mut w, "Name", o.name);
        xml_text(&mut w, "Prefix", o.prefix);
        xml_text(&mut w, "Delimiter", o.delimiter);
        xml_text(&mut w, "EncodingType", o.encoding_type);
        xml_text(&mut w, "Marker", o.marker);
        xml_text(&mut w, "MaxKeys", Some(o.max_keys.to_string()));
        xml_text(&mut w, "IsTruncated", Some(o.is_truncated.to_string()));
        xml_text(&mut w, "NextMarker", o.next_marker);

        for obj in o.contents.unwrap_or_default() {
            xml_tag!("Contents", w, {
                xml_text(&mut w, "Key", obj.key);
                xml_date(&mut w, "LastModified", obj.last_modified);
                xml_text(&mut w, "ETag", obj.e_tag);
                xml_text(&mut w, "Size", Some(obj.size.to_string()));
                xml_text(&mut w, "StorageClass", obj.storage_class);
                xml_owner(&mut w, obj.owner);
            });
        }
        for p in o.common_prefixes.unwrap_or_default() {
            xml_tag!("CommonPrefixes", w, {
                xml_text(&mut w, "Prefix", p.prefix);
            });
        }
    });
    Ok(responder().body(Body::from(xstr)).unwrap())
}

pub fn list_objects_v2(o: ListObjectsV2Output) -> Result<HttpResponse, S3Error> {
    let xstr = xml_doc!("ListBucketResult", w, {
        xml_text(&mut w, "Name", o.name);
        xml_text(&mut w, "Prefix", o.prefix);
        xml_text(&mut w, "Delimiter", o.delimiter);
        xml_text(&mut w, "EncodingType", o.encoding_type);
        xml_text(&mut w, "StartAfter", o.start_after);
        xml_text(&mut w, "ContinuationToken", o.continuation_token);
        xml_text(&mut w, "MaxKeys", Some(o.max_keys.to_string()));
        xml_text(&mut w, "KeyCount", Some(o.key_count.to_string()));
        xml_text(&mut w, "IsTruncated", Some(o.is_truncated.to_string()));
        xml_text(&mut w, "NextContinuationToken", o.next_continuation_token);

        for obj in o.contents.unwrap_or_default() {
            xml_tag!("Contents", w, {
                xml_text(&mut w, "Key", obj.key);
                xml_date(&mut w, "LastModified", obj.last_modified);
                xml_text(&mut w, "ETag", obj.e_tag);
                xml_text(&mut w, "Size", Some(obj.size.to_string()));
                xml_text(&mut w, "StorageClass", obj.storage_class);
                xml_owner(&mut w, obj.owner);
            });
        }
        for p in o.common_prefixes.unwrap_or_default() {
            xml_tag!("CommonPrefixes", w, {
                xml_text(&mut w, "Prefix", p.prefix);
            });
        }
    });
    Ok(responder().body(Body::from(xstr)).unwrap())
}

//---------------------------------------------------//
// advanced object ops                               //
//---------------------------------------------------//

//---------------------------------------------------//
// advanced bucket ops                               //
//---------------------------------------------------//

/// This macro generates a default parser function per op
/// to make it possible to implement it in stages.
macro_rules! gen {
    ($op:ident) => {
        paste::paste! {
            pub fn [<$op:snake>](_: [<$op Output>]) -> Result<HttpResponse, S3Error> {
                Err(S3Error::builder()
                    .code("NotImplemented")
                    .message(format!(
                        "Not implemented {}",
                        stringify!(s3::output::parsers::[<$op:snake>])
                    ))
                    .build()
                    .into())
            }
        }
    };
}

//-------------------------------//
// gen!(HeadBucket);
// gen!(CreateBucket);
// gen!(DeleteBucket);
// gen!(GetBucketLocation);
//-------------------------------//

//-------------------------------//
// gen!(GetObject);
// gen!(HeadObject);
// gen!(PutObject);
// gen!(CopyObject);
// gen!(DeleteObject);
// gen!(DeleteObjects);
//-------------------------------//

//-------------------------------//
gen!(CreateMultipartUpload);
gen!(CompleteMultipartUpload);
gen!(AbortMultipartUpload);
gen!(UploadPart);
gen!(UploadPartCopy);
//-------------------------------//

//-------------------------------//
// gen!(ListBuckets);
// gen!(ListObjects);
// gen!(ListObjectsV2);
gen!(ListObjectVersions);
gen!(ListMultipartUploads);
gen!(ListParts);
//-------------------------------//

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
