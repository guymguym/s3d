#[derive(Debug, Clone)]
pub enum S3Errors {
    _BadRequest,
    _BucketAlreadyExists,
    _NoSuchBucket,
    _NoSuchKey,
    _InternalError,
}

#[derive(Debug, Clone)]
pub struct S3Error {
    pub status_code: hyper::StatusCode,
    pub code: String,
    pub msg: String,
    pub resource: String,
    pub request_id: String,
}

impl std::error::Error for S3Error {}

impl std::fmt::Display for S3Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::fmt::Display for S3Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl S3Errors {
    fn _name(&self) -> String {
        format!("{}", self)
    }
}

impl S3Error {
    pub fn _from(kind: S3Errors) -> S3Error {
        match kind {
            S3Errors::_BadRequest => S3Error {
                status_code: hyper::StatusCode::BAD_REQUEST,
                code: "BadRequest".to_owned(),
                msg: "".to_owned(),
                resource: "".to_owned(),
                request_id: "".to_owned(),
            },
            S3Errors::_BucketAlreadyExists => S3Error {
                status_code: hyper::StatusCode::CONFLICT,
                code: "BucketAlreadyExists".to_owned(),
                msg: "".to_owned(),
                resource: "".to_owned(),
                request_id: "".to_owned(),
            },
            S3Errors::_NoSuchBucket => S3Error {
                status_code: hyper::StatusCode::NOT_FOUND,
                code: "NoSuchBucket".to_owned(),
                msg: "".to_owned(),
                resource: "".to_owned(),
                request_id: "".to_owned(),
            },
            S3Errors::_NoSuchKey => S3Error {
                status_code: hyper::StatusCode::NOT_FOUND,
                code: "NoSuchKey".to_owned(),
                msg: "".to_owned(),
                resource: "".to_owned(),
                request_id: "".to_owned(),
            },
            _ => S3Error {
                status_code: hyper::StatusCode::INTERNAL_SERVER_ERROR,
                code: "InternalError".to_owned(),
                msg: "".to_owned(),
                resource: "".to_owned(),
                request_id: "".to_owned(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(S3Errors::_BadRequest._name(), "BadRequest");
        assert_eq!(S3Errors::_BucketAlreadyExists._name(), "BucketAlreadyExists");
        assert_eq!(S3Errors::_NoSuchBucket._name(), "NoSuchBucket");
        assert_eq!(S3Errors::_NoSuchKey._name(), "NoSuchKey");
        assert_eq!(S3Errors::_InternalError._name(), "InternalError");
    }
}
