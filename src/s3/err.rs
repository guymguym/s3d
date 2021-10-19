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

impl S3Error {
    pub fn from(kind: S3Errors) -> S3Error {
        match kind {
            S3Errors::BadRequest => S3Error {
                status_code: hyper::StatusCode::BAD_REQUEST,
                code: "BadRequest".to_owned(),
                msg: "".to_owned(),
                resource: "".to_owned(),
                request_id: "".to_owned(),
            },
            S3Errors::BucketAlreadyExists => S3Error {
                status_code: hyper::StatusCode::CONFLICT,
                code: "BucketAlreadyExists".to_owned(),
                msg: "".to_owned(),
                resource: "".to_owned(),
                request_id: "".to_owned(),
            },
            S3Errors::NoSuchBucket => S3Error {
                status_code: hyper::StatusCode::NOT_FOUND,
                code: "NoSuchBucket".to_owned(),
                msg: "".to_owned(),
                resource: "".to_owned(),
                request_id: "".to_owned(),
            },
            S3Errors::NoSuchKey => S3Error {
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

#[derive(Debug, Clone)]
pub enum S3Errors {
    BadRequest,
    BucketAlreadyExists,
    NoSuchBucket,
    NoSuchKey,
    InternalError,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let arr = [
            S3Errors::BadRequest,
            S3Errors::BucketAlreadyExists,
            S3Errors::NoSuchBucket,
            S3Errors::NoSuchKey,
            S3Errors::InternalError,
        ];
        for i in arr.iter() {
            println!("{:?}", i);
        }
    }
}
