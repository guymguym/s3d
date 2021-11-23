use crate::gen::{api::S3Api, api::S3ApiToClient};

pub struct Intercept {
    pub s3_api: S3ApiToClient,
}

impl S3Api for Intercept {

}
