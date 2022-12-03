use crate::utils::{new_s3_client, parse_bucket_and_prefix};

/// Change current id/bucket/prefix for subsequent commands
#[derive(clap::Parser, Debug, Clone)]
pub struct CdCmd {
    /// When empty list all buckets.
    /// Otherwise list objects in bucket with optional key prefix (`bucket` or `bucket/prefix`)
    #[clap(name = "bucket[/prefix]", default_value = "")]
    bucket_and_prefix: String,
    
    /// User identity
    #[clap(default_value = "")]
    id: String,
}

impl CdCmd {
    pub async fn run(&self) -> anyhow::Result<()> {
        let s3 = new_s3_client().await;
        let (bucket, prefix) = parse_bucket_and_prefix(&self.bucket_and_prefix)?;

        if bucket.is_empty() {
        } else {
            let res = s3
                .list_objects_v2()
                .bucket(bucket)
                .prefix(prefix)
                .delimiter("/")
                .max_keys(10)
                .send()
                .await?;
        }

        Ok(())
    }
}
