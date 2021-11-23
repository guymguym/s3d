use crate::http::S3Error;
pub use aws_smithy_xml::encode::{ScopeWriter, XmlWriter};

pub const S3_XMLNS: &'static str = "http://s3.amazonaws.com/doc/2006-03-01/";

pub fn xml_root<'a, 'b>(x: &'a mut XmlWriter<'_>, root: &'b str) -> ScopeWriter<'a, 'b> {
    x.start_el(root).write_ns(S3_XMLNS, None).finish()
}

pub fn xml_elem<'a, 'b>(w: &'a mut ScopeWriter<'_, '_>, name: &'b str) -> ScopeWriter<'a, 'b> {
    w.start_el(name).finish()
}

pub fn xml_text<T: AsRef<str>>(w: &mut ScopeWriter, tag: &str, data: T) {
    let mut el = xml_elem(w, tag);
    el.data(data.as_ref());
    el.finish();
}

pub fn xml_text_opt<'a, 'b, T: AsRef<str>>(w: &mut ScopeWriter, tag: &str, opt_text: Option<T>) {
    if let Some(text) = opt_text {
        xml_text(w, tag, text)
    }
}

pub fn xml_error(e: S3Error) -> String {
    let mut out = String::new();
    let mut x = XmlWriter::new(&mut out);
    let mut w = xml_root(&mut x, "Error");
    xml_text_opt(&mut w, "Code", e.code());
    xml_text_opt(&mut w, "Message", e.message());
    xml_text_opt(&mut w, "RequestId", e.request_id());
    xml_text_opt(&mut w, "Resource", e.extra("resource"));
    w.finish();
    out
}
