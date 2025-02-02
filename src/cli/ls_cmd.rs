use crate::utils::{new_s3_client, parse_bucket_and_prefix};
use aws_smithy_types::date_time::Format;

/// List buckets or objects
#[derive(clap::Parser, Debug, Clone)]
#[clap(aliases = &["list"])]
pub struct LsCmd {
    /// When empty list all buckets.
    /// Otherwise list objects in bucket with optional key prefix (`bucket` or `bucket/prefix`)
    #[clap(name = "bucket[/prefix]", default_value = "")]
    bucket_and_prefix: String,
}

impl LsCmd {
    pub async fn run(&self) -> anyhow::Result<()> {
        let s3 = new_s3_client().await;
        let (bucket, prefix) = parse_bucket_and_prefix(&self.bucket_and_prefix)?;

        if bucket.is_empty() {
            let res = s3.list_buckets().send().await?;
            for it in res.buckets.unwrap_or_default() {
                println!(
                    "{} {}",
                    it.creation_date().unwrap().fmt(Format::DateTime).unwrap(),
                    it.name().unwrap()
                );
            }
        } else {
            let res = s3
                .list_objects_v2()
                .bucket(bucket)
                .prefix(prefix)
                .delimiter("/")
                .send()
                .await?;
            for it in res.common_prefixes.unwrap_or_default() {
                println!(
                    "{:.^20} {:.>12} {}",
                    "PREFIX",
                    "",
                    it.prefix().unwrap()
                );
            }
            for it in res.contents.unwrap_or_default() {
                println!(
                    "{:>20} {:.>12} {}",
                    it.last_modified().unwrap().fmt(Format::DateTime).unwrap(),
                    it.size(),
                    it.key().unwrap(),
                );
            }
        }

        Ok(())
    }
}
