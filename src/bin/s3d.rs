extern crate s3d;
use clap::Parser;
use std::fmt::Debug;

/// main of the s3d binary.
/// parses the command line args using clap and starts the daemon services.
#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    // env_logger::init();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("warn,s3d=debug"));
    DaemonCmd::parse().run().await
}

#[derive(clap::Parser, Debug, Clone)]
#[clap(name = "s3d")]
#[clap(about = clap::crate_description!())]
#[clap(version = clap::crate_version!())]
#[clap(next_display_order = 0)]
pub struct DaemonCmd {}

impl DaemonCmd {
    pub async fn run(self) -> anyhow::Result<()> {
        log::debug!("{:?}", self);
        // #[cfg(feature = "fuse")]
        // {
        //     s3d::daemon::fuse::Fuse::start_fuse_mount().await?;
        // }
        s3d::daemon::server::serve().await?;
        Ok(())
    }
}
