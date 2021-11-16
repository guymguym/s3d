use crate::{err::*, ops::S3Request, util::*};
use aws_sdk_s3::{input::*, output::*};
use aws_smithy_xml::encode::{ScopeWriter, XmlWriter};
use hyper::Body;

const S3_XMLNS: &'static str = "http://s3.amazonaws.com/doc/2006-03-01/";

fn xml_root<'a, 'b>(x: &'a mut XmlWriter<'_>, root: &'b str) -> ScopeWriter<'a, 'b> {
    x.start_el(root).write_ns(S3_XMLNS, None).finish()
}
fn xml_elem<'a, 'b>(w: &'a mut ScopeWriter<'_, '_>, name: &'b str) -> ScopeWriter<'a, 'b> {
    w.start_el(name).write_ns(S3_XMLNS, None).finish()
}
fn xml_text(w: &mut ScopeWriter, tag: &str, data: String) {
    let mut el = xml_elem(w, tag);
    el.data(&data);
    el.finish();
}
fn xml_text_opt<'a, 'b>(w: &mut ScopeWriter, tag: &str, opt_text: Option<String>) {
    if let Some(text) = opt_text {
        xml_text(w, tag, text)
    }
}

pub fn list_buckets_input(_req: &S3Request) -> Result<ListBucketsInput, S3Error> {
    ListBucketsInput::builder()
        .build()
        .or_else(|err| Err(S3Error::new(S3Errors::InternalError)))
}

pub fn list_buckets_output(o: ListBucketsOutput) -> S3Result {
    let mut out = String::new();
    let mut x = XmlWriter::new(&mut out);
    let mut r = xml_root(&mut x, "ListAllMyBucketsResult");
    {
        let mut s = xml_elem(&mut r, "Buckets");
        for b in o.buckets.unwrap_or_default() {
            let mut s = xml_elem(&mut s, "Bucket");
            xml_text_opt(&mut s, "Name", b.name);
            xml_text_opt(
                &mut s,
                "CreationDate",
                b.creation_date.map(|t| t.to_chrono().to_rfc3339()),
            );
            s.finish();
        }
        s.finish();
    }
    {
        let mut s = xml_elem(&mut r, "Owner");
        let owner = o.owner.unwrap();
        xml_text_opt(&mut s, "ID", owner.id);
        xml_text_opt(&mut s, "DisplayName", owner.display_name);
        s.finish();
    }
    r.finish();
    Ok(http_response().body(Body::from(out)).unwrap())
}

pub fn list_objects_input(req: &S3Request) -> Result<ListObjectsInput, S3Error> {
    ListObjectsInput::builder()
        .set_bucket(Some(req.bucket.clone()))
        .set_delimiter(get_param(req, "delimiter"))
        .set_marker(get_param(req, "marker"))
        .set_prefix(get_param(req, "prefix"))
        .set_max_keys(get_param_i32(req, "max-keys"))
        // .set_encoding_type(i.expected_bucket_owner)
        // .set_expected_bucket_owner(i.expected_bucket_owner)
        // .set_request_payer(i.request_payer)
        .build()
        .or_else(|err| Err(S3Error::new(S3Errors::InternalError)))
}

pub fn list_objects_output(o: ListObjectsOutput) -> S3Result {
    let mut w = XmlWriter2::new();
    w.start_tag("ListBucketResult");
    w.text_tag_opt("Name", o.name);
    w.text_tag_opt("Prefix", o.prefix);
    w.text_tag_opt("Delimiter", o.delimiter);
    w.text_tag_opt("Marker", o.marker);
    w.text_tag_opt("EncodingType", o.encoding_type);
    w.text_tag("MaxKeys", o.max_keys.to_string());
    w.text_tag("IsTruncated", o.is_truncated.to_string());
    w.text_tag_opt("NextMarker", o.next_marker);
    for obj in o.contents.unwrap_or_default() {
        w.start_tag("Contents");
        w.text_tag_opt("Key", obj.key);
        w.text_tag_opt(
            "LastModified",
            obj.last_modified.map(|t| t.to_chrono().to_rfc3339()),
        );
        w.text_tag_opt("ETag", obj.e_tag);
        w.text_tag("Size", obj.size.to_string());
        w.text_tag_opt("StorageClass", obj.storage_class);
        let owner = obj.owner.unwrap();
        w.start_tag("Owner");
        w.text_tag_opt("ID", owner.id);
        w.text_tag_opt("DisplayName", owner.display_name);
        w.end_tag("Owner");
        w.end_tag("Contents");
    }
    for p in o.common_prefixes.unwrap_or_default() {
        w.start_tag("CommonPrefixes");
        w.text_tag_opt("Prefix", p.prefix);
        w.end_tag("CommonPrefixes");
    }
    w.end_tag("ListBucketResult");
    Ok(http_response().body(w.body()).unwrap())
}

pub fn get_object_input(req: &S3Request) -> Result<GetObjectInput, S3Error> {
    GetObjectInput::builder()
        .set_bucket(Some(req.bucket.clone()))
        .set_key(Some(req.key.clone()))
        .set_part_number(get_param_i32(req, "partNumber"))
        .set_range(get_header(req, "range"))
        .set_version_id(get_param(req, "versionId"))
        .build()
        .or_else(|err| Err(S3Error::new(S3Errors::InternalError)))
}

pub fn get_object_output(o: GetObjectOutput) -> S3Result {
    // let body = Body::from(o.body.collect().await);
    Ok(http_response().body(Body::from("get_object")).unwrap())
}

fn get_param(req: &S3Request, name: &str) -> Option<String> {
    req.params.get(name).map(|x| x.clone())
}
fn get_param_i32(req: &S3Request, name: &str) -> Option<i32> {
    req.params.get(name).map(|x| x.parse().unwrap())
}
fn get_header(req: &S3Request, name: &str) -> Option<String> {
    req.headers
        .get(name)
        .map_or(None, |x| x.to_str().map_or(None, |s| Some(s.to_string())))
}
