/// TODO generate this file automatically with https://github.com/awslabs/smithy-rs
use crate::s3::{output::*, types::*};
use aws_sdk_s3::output::*;
use aws_smithy_xml::encode::{ScopeWriter, XmlWriter};
use hyper::Body;

const S3_XMLNS: &'static str = "http://s3.amazonaws.com/doc/2006-03-01/";

pub struct S3OutputImpl {}

impl S3Output for S3OutputImpl {
    fn list_buckets(&self, o: ListBucketsOutput) -> Result<HttpResponse, OutputError> {
        let mut out = String::new();
        let mut x = XmlWriter::new(&mut out);
        let mut w = xml_root(&mut x, "ListAllMyBucketsResult");
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

    fn list_objects(&self, o: ListObjectsOutput) -> Result<HttpResponse, OutputError> {
        let mut out = String::new();
        let mut x = XmlWriter::new(&mut out);
        let mut w = xml_root(&mut x, "ListBucketResult");
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
        }
        w.finish();
        Ok(responder().body(Body::from(out)).unwrap())
    }

    fn get_object(&self, o: GetObjectOutput) -> Result<HttpResponse, OutputError> {
        // let body = Body::from(o.body.collect().await);
        Ok(responder().body(Body::from("get_object")).unwrap())
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
