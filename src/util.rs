use aws_sdk_s3::ByteStream;
use hyper::Body;

pub type HttpRequest = hyper::Request<hyper::Body>;
pub type HttpResponse = hyper::Response<Body>;
pub type HttpResult = anyhow::Result<HttpResponse>;
pub type HttpResultOrErr<E> = Result<HttpResponse, E>;

pub fn http_response() -> hyper::http::response::Builder {
    hyper::http::response::Builder::new()
}

pub struct XmlWriter2 {
    buf: Vec<u8>,
    // writer: quick_xml::Writer
}

impl XmlWriter2 {
    pub fn new() -> Self {
        let mut w = Self { buf: Vec::new() };
        // BodyWriter { writer: Writer::new(Cursor::new(Vec::new())); }
        w.append_raw("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        w
    }
    pub fn append_raw(&mut self, s: &str) -> &mut Self {
        self.buf.extend_from_slice(s.as_bytes());
        self
    }
    pub fn start_tag(&mut self, tag: &str) -> &mut Self {
        self.append_raw(format!("<{}>", tag).as_str())
    }
    pub fn end_tag(&mut self, tag: &str) -> &mut Self {
        self.append_raw(format!("</{}>", tag).as_str())
    }
    pub fn text_tag<T: AsRef<str>>(&mut self, tag: &str, val: T) -> &mut Self {
        self.append_raw(format!("<{0}>{1}</{0}>", tag, val.as_ref()).as_str())
        // writer.write_event(Event::Start(BytesStart::borrowed_name(tag)))
        // writer.write_event(Event::Text(BytesText::borrowed(tag)))
        // writer.write_event(Event::End(BytesEnd::borrowed(tag)))
    }
    pub fn text_tag_opt<T: AsRef<str>>(&mut self, tag: &str, opt: Option<T>) -> &mut Self {
        match opt {
            Some(val) => self.text_tag(tag, val.as_ref()),
            None => self,
        }
    }
    pub fn _text_tag_opt_map<T>(
        &mut self,
        tag: &str,
        opt: Option<T>,
        to_str: &dyn Fn(T) -> String,
    ) -> &mut Self {
        match opt {
            Some(val) => self.text_tag(tag, to_str(val).as_str()),
            None => self,
        }
    }
    pub fn _str(self) -> String {
        String::from_utf8(self.buf).unwrap()
    }
    pub fn body(self) -> Body {
        Body::from(self.buf)
    }
}
