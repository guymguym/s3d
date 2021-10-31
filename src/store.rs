use crate::{err::*, util::*};
use hyper::Body;

pub async fn list_buckets() -> S3Result {
    Ok(http_response().body(Body::from("list_buckets")).unwrap())
}

pub async fn put_bucket(_bucket: &str) -> S3Result {
    Ok(http_response().body(Body::from("put_bucket")).unwrap())
}

pub async fn head_bucket(_bucket: &str) -> S3Result {
    Ok(http_response().body(Body::from("head_bucket")).unwrap())
}

pub async fn delete_bucket(_bucket: &str) -> S3Result {
    Ok(http_response().body(Body::from("delete_bucket")).unwrap())
}

pub async fn list_objects(_bucket: &str) -> S3Result {
    Ok(http_response().body(Body::from("list_objects")).unwrap())
}

pub async fn get_object(_bucket: &str, _key: &str) -> S3Result {
    Ok(http_response().body(Body::from("get_object")).unwrap())
}

pub async fn put_object(_bucket: &str, _key: &str) -> S3Result {
    Ok(http_response().body(Body::from("put_object")).unwrap())
}

pub async fn head_object(_bucket: &str, _key: &str) -> S3Result {
    Ok(http_response().body(Body::from("head_object")).unwrap())
}

pub async fn delete_object(_bucket: &str, _key: &str) -> S3Result {
    Ok(http_response().body(Body::from("delete_object")).unwrap())
}

// pub async fn list_buckets() -> S3Result {
//     let s3_config = aws_config::from_env().load().await;
//     let client = aws_sdk_s3::Client::new(&s3_config);
//     // let client = S3Client::new(Region::Custom {
//     //     name: "local".to_string(),
//     //     endpoint: "http://localhost:4572".to_string(),
//     // });
//     let mut buckets = Vec::new();
//     let res = client.list_buckets().send().await.unwrap_or_else(|err| {
//         println!("{:?}", err);
//         Err(S3Error::_from(S3Errors::_BadRequest))
//     })?;
//     for bucket in res.buckets.unwrap_or_default() {
//         println!("_bucket: {:?}", bucket.name.as_deref().unwrap_or_default());
//         buckets.push(bucket.name.unwrap_or_default());
//     }
// }
