use hyper::Body;

pub type HttpRequest = hyper::Request<hyper::Body>;
pub type HttpResponse = hyper::Response<hyper::Body>;
pub type HttpResult = anyhow::Result<HttpResponse>;
pub type HttpResultOrErr<E> = Result<HttpResponse, E>;

pub fn http_response() -> hyper::http::response::Builder {
    hyper::http::response::Builder::new()
}

pub struct BodyWriter {
    buf: Vec<u8>,
    // writer: quick_xml::Writer
}

impl BodyWriter {
    pub fn new() -> Self {
        BodyWriter { buf: Vec::new() }
        // BodyWriter { writer: Writer::new(Cursor::new(Vec::new())); }
    }
    pub fn new_xml() -> Self {
        let mut w = Self::new();
        w.append("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        w
    }
    pub fn append(&mut self, s: &str) -> &mut Self {
        self.buf.extend_from_slice(s.as_bytes());
        self
    }
    pub fn append_xml(&mut self, tag: &str, content: &str) -> &mut Self {
        self.append(format!("<{0}>{1}</{0}>", tag, content).as_str())
        // writer.write_event(Event::Start(BytesStart::borrowed_name(tag)))
        // writer.write_event(Event::Text(BytesText::borrowed(tag)))
        // writer.write_event(Event::End(BytesEnd::borrowed(tag)))
    }
    pub fn _str(self) -> String {
        String::from_utf8(self.buf).unwrap()
    }
    pub fn body(self) -> Body {
        Body::from(self.buf)
    }
}
