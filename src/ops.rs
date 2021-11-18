use crate::types::*;
use aws_sdk_s3::{error::*, input::*, output::*};
use aws_smithy_http::result::SdkError;
use hyper::Body;

pub async fn list_buckets(
    s3c: &S3C,
    ac: &AC,
    i: ListBucketsInput,
) -> Result<ListBucketsOutput, ListBucketsError> {
    ac.call(i.make_operation(s3c.conf()).await.unwrap())
        .await
        .map_err(|err| match err {
            SdkError::ServiceError { err, .. } => err,
            _ => ListBucketsError::unhandled(err),
        })
}

pub async fn list_objects(
    s3c: &S3C,
    ac: &AC,
    i: ListObjectsInput,
) -> Result<ListObjectsOutput, ListObjectsError> {
    ac.call(i.make_operation(s3c.conf()).await.unwrap())
        .await
        .map_err(|err| match err {
            SdkError::ServiceError { err, .. } => err,
            _ => ListObjectsError::unhandled(err),
        })
}

pub async fn get_object(s3c: &S3C, ac: &AC, i: GetObjectInput) -> Result<GetObjectOutput, S3Errors> {
    Ok(ac.call(i.make_operation(s3c.conf()).await.unwrap()).await?)
}

pub async fn head_bucket(_req: &S3Request) -> S3Result {
    Ok(responder().body(Body::from("head_bucket\n")).unwrap())
}

pub async fn head_object(_req: &S3Request) -> S3Result {
    Ok(responder().body(Body::from("head_object\n")).unwrap())
}

pub async fn put_bucket(_req: &S3Request) -> S3Result {
    Ok(responder().body(Body::from("put_bucket\n")).unwrap())
}

pub async fn put_object(_req: &S3Request) -> S3Result {
    Ok(responder().body(Body::from("put_object\n")).unwrap())
}

pub async fn delete_bucket(_req: &S3Request) -> S3Result {
    Ok(responder().body(Body::from("delete_bucket\n")).unwrap())
}

pub async fn delete_object(_req: &S3Request) -> S3Result {
    Ok(responder().body(Body::from("delete_object\n")).unwrap())
}

pub async fn post_object(_req: &S3Request) -> S3Result {
    Ok(responder().body(Body::from("post_object\n")).unwrap())
}

pub async fn get_bucket_subresource(req: &S3Request) -> S3Result {
    Ok(responder()
        .body(Body::from(format!(
            "get_bucket_subresource {:?}\n",
            req.bucket_subresource
        )))
        .unwrap())
}

pub async fn get_object_subresource(req: &S3Request) -> S3Result {
    Ok(responder()
        .body(Body::from(format!(
            "get_object_subresource {:?}\n",
            req.object_subresource
        )))
        .unwrap())
}

pub async fn put_bucket_subresource(req: &S3Request) -> S3Result {
    Ok(responder()
        .body(Body::from(format!(
            "put_bucket_subresource {:?}\n",
            req.bucket_subresource
        )))
        .unwrap())
}

pub async fn put_object_subresource(req: &S3Request) -> S3Result {
    Ok(responder()
        .body(Body::from(format!(
            "put_object_subresource {:?}\n",
            req.object_subresource
        )))
        .unwrap())
}

pub async fn delete_bucket_subresource(req: &S3Request) -> S3Result {
    Ok(responder()
        .body(Body::from(format!(
            "delete_bucket_subresource {:?}\n",
            req.bucket_subresource
        )))
        .unwrap())
}

pub async fn delete_object_subresource(req: &S3Request) -> S3Result {
    Ok(responder()
        .body(Body::from(format!(
            "delete_object_subresource {:?}\n",
            req.object_subresource
        )))
        .unwrap())
}

pub async fn post_bucket_subresource(req: &S3Request) -> S3Result {
    Ok(responder()
        .body(Body::from(format!(
            "post_bucket_subresource {:?}\n",
            req.bucket_subresource
        )))
        .unwrap())
}

pub async fn create_multipart_upload(req: &S3Request) -> S3Result {
    Ok(responder()
        .body(Body::from(format!(
            "create_multipart_upload {:?}\n",
            req.object_subresource
        )))
        .unwrap())
}

pub async fn complete_multipart_upload(req: &S3Request) -> S3Result {
    Ok(responder()
        .body(Body::from(format!(
            "complete_multipart_upload {:?}\n",
            req.object_subresource
        )))
        .unwrap())
}

pub async fn fetch_flow(_req: &S3Request) -> S3Result {
    Ok(responder().body(Body::from("fetch_flow\n")).unwrap())
}

pub async fn pull_flow(_req: &S3Request) -> S3Result {
    Ok(responder().body(Body::from("pull_flow\n")).unwrap())
}

pub async fn push_flow(_req: &S3Request) -> S3Result {
    Ok(responder().body(Body::from("push_flow\n")).unwrap())
}

pub async fn prune_flow(_req: &S3Request) -> S3Result {
    Ok(responder().body(Body::from("prune_flow\n")).unwrap())
}

pub async fn status_flow(_req: &S3Request) -> S3Result {
    Ok(responder().body(Body::from("status_flow\n")).unwrap())
}

pub async fn diff_flow(_req: &S3Request) -> S3Result {
    Ok(responder().body(Body::from("diff_flow\n")).unwrap())
}

// pub async fn list_buckets() -> S3Result {
//     let s3_config = aws_config::from_env().load().await;
//     let client = S3C::new(&s3_config);
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
