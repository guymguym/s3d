use crate::utils::{new_s3_client, parse_bucket_and_prefix};
use aws_smithy_types::date_time::Format;

/// Get identity information
#[derive(clap::Parser, Debug, Clone)]
pub struct IdCmd {
    /// User identity
    #[clap(default_value = "")]
    id: String,
}

impl IdCmd {
    pub async fn run(&self) -> anyhow::Result<()> {
        Ok(())
    }
}
