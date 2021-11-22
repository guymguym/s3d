use aws_smithy_types::instant::{Format, Instant};
use hyper::{
    header::{HeaderName, HeaderValue},
    HeaderMap,
};
use std::str::FromStr;

pub trait ToHeader: Sized {
    fn to_header(self) -> Option<HeaderValue>;
    fn set_header(self, h: &mut HeaderMap<HeaderValue>, key: &'static str) {
        if let Some(value) = self.to_header() {
            h.insert(key, value);
        }
    }
    fn set_header_non_static(self, h: &mut HeaderMap<HeaderValue>, key: &str) {
        if let Some(value) = self.to_header() {
            let k = HeaderName::from_str(key).unwrap();
            h.insert(k, value);
        }
    }
}

impl ToHeader for HeaderValue {
    fn to_header(self) -> Option<HeaderValue> {
        Some(self)
    }
}
impl ToHeader for &str {
    fn to_header(self) -> Option<HeaderValue> {
        HeaderValue::from_str(self).ok()
    }
}
impl ToHeader for &Instant {
    fn to_header(self) -> Option<HeaderValue> {
        self.fmt(Format::DateTime).to_header()
    }
}
impl<T: ToHeader> ToHeader for Option<T> {
    fn to_header(self) -> Option<HeaderValue> {
        self.and_then(|s| s.to_header())
    }
}

macro_rules! to_header_to_string {
    ($t:ty) => {
        impl ToHeader for $t {
            fn to_header(self) -> Option<HeaderValue> {
                self.to_string().to_header()
            }
        }
    };
}

macro_rules! to_header_as_str {
    ($t:ty) => {
        impl ToHeader for &$t {
            fn to_header(self) -> Option<HeaderValue> {
                self.as_str().to_header()
            }
        }
    };
}

to_header_to_string!(i64);
to_header_to_string!(i32);
to_header_to_string!(bool);

to_header_as_str!(aws_sdk_s3::model::StorageClass);
to_header_as_str!(aws_sdk_s3::model::ArchiveStatus);
to_header_as_str!(aws_sdk_s3::model::RequestCharged);
to_header_as_str!(aws_sdk_s3::model::ReplicationStatus);
to_header_as_str!(aws_sdk_s3::model::ServerSideEncryption);
to_header_as_str!(aws_sdk_s3::model::ObjectLockMode);
to_header_as_str!(aws_sdk_s3::model::ObjectLockLegalHoldStatus);
