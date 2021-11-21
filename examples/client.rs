use aws_sdk_s3::{Credentials, Endpoint, Region};
use hyper::Uri;

const S3D_ENDPOINT: &'static str = "http://localhost:5333";

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    let s3c = aws_sdk_s3::Client::from_conf(
        aws_sdk_s3::Config::builder()
            .endpoint_resolver(Endpoint::immutable(Uri::from_static(S3D_ENDPOINT)))
            .region(Region::new("us-east-1"))
            .credentials_provider(Credentials::new("s3d", "s3d", None, None, "s3d"))
            .build(),
    );

    // let r = s3c.list_buckets().send().await?;
    // println!("list_buckets: {:?}", r);

    // let bucket_name = r.buckets.unwrap()[0].name.as_ref().unwrap().clone();
    let bucket_name = String::from("lalalalalalala");

    let r = s3c.head_bucket().bucket(&bucket_name).send().await?;
    println!("head_bucket: {:?}", r);

    // let r = s3c.list_objects().bucket(&bucket_name).send().await?;
    // println!("head_bucket: {:?}", r);

    Ok(())
}
