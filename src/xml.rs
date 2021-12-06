use crate::http::S3Error;
use aws_smithy_types::{date_time::Format, DateTime};
use aws_smithy_xml::decode::ScopedDecoder;
pub use aws_smithy_xml::encode::{ScopeWriter, XmlWriter};
use std::str::FromStr;

pub const XML_META: &'static str = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n";
pub const XMLNS_S3: &'static str = "http://s3.amazonaws.com/doc/2006-03-01/";

macro_rules! xml_doc {
    ($root: literal, $w: ident, $code: block) => {{
        let mut xstr = String::from(XML_META);
        let mut xml = XmlWriter::new(&mut xstr);
        let mut $w = xml.start_el($root).write_ns(XMLNS_S3, None).finish();
        $code
        $w.finish();
        xstr
    }}
}

macro_rules! xml_tag {
    ($tag: literal, $w: ident, $code: block) => {{
        let mut $w = $w.start_el($tag).finish();
        $code
        $w.finish();
    }}
}

pub(crate) use xml_doc;
pub(crate) use xml_tag;

pub fn xml_text<T: AsRef<str>>(w: &mut ScopeWriter, tag: &str, text: Option<T>) {
    if let Some(data) = text {
        let mut el = w.start_el(tag).finish();
        el.data(data.as_ref());
        el.finish();
    }
}

pub fn xml_date(w: &mut ScopeWriter, tag: &str, date: Option<DateTime>) {
    xml_text(w, tag, date.and_then(|x| x.fmt(Format::DateTime).ok()));
}

// pub fn xml_owner(w: &mut ScopeWriter, owner: Option<aws_sdk_s3::model::Owner>) {
//     if let Some(owner) = owner {
//         xml_tag!("Owner", w, {
//             xml_text(&mut w, "ID", owner.id());
//             xml_text(&mut w, "DisplayName", owner.display_name());
//         });
//     }
// }

pub fn xml_error(e: S3Error) -> String {
    xml_doc!("Error", w, {
        xml_text(&mut w, "Code", e.inner.code());
        xml_text(&mut w, "Message", e.inner.message());
        xml_text(&mut w, "RequestId", e.inner.request_id());
        xml_text(&mut w, "Resource", e.inner.extra("resource"));
    })
}

pub fn xml_to_data<T: FromStr>(d: &mut ScopedDecoder) -> Option<T> {
    match aws_smithy_xml::decode::try_data(d) {
        Ok(data) => data.parse::<T>().ok(),
        Err(_) => None,
    }
}

pub fn xml_to_date(d: &mut ScopedDecoder) -> Option<DateTime> {
    match aws_smithy_xml::decode::try_data(d) {
        Ok(data) => DateTime::from_str(&data, Format::DateTime).ok(),
        Err(_) => None,
    }
}
