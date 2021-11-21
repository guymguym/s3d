use aws_sdk_s3::Endpoint;
use hyper::Uri;

const S3D_ENDPOINT: &'static str = "http://localhost:3000";

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    let s3_config = aws_sdk_s3::Config::builder()
        .endpoint_resolver(Endpoint::immutable(Uri::from_static(S3D_ENDPOINT)))
        .build();
    let s3c = aws_sdk_s3::Client::from_conf(s3_config);

    let r = s3c.list_buckets().send().await?;
    println!("list_buckets: {:?}", r);

    let bucket_name = r.buckets.unwrap()[0].name.as_ref().unwrap().clone();

    let r = s3c.head_bucket().bucket(&bucket_name).send().await?;
    println!("head_bucket: {:?}", r);

    let r = s3c.list_objects().bucket(&bucket_name).send().await?;
    println!("head_bucket: {:?}", r);

    Ok(())
}
