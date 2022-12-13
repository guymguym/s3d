extern crate s3d;
use clap::Parser;
use std::fmt::Debug;

/// main of the s3d binary
///
/// logging: by default only error! level is logged but it can be changed with env
///     e.g RUST_LOG=trace or RUST_LOG=info,s3d=debug etc.
///
/// Parse the command line args using clap and starts the daemon services.
#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("s3d=trace"));
    // env_logger::init();

    let cli = DaemonCmd::parse();
    log::trace!("parsed command line args: {:?}", cli);

    // fuse mount is configurable with the `fuse` cargo feature
    // #[cfg(feature = "fuse")]
    // {
    //     s3d::daemon::fuse::Fuse::start_fuse_mount().await?;
    // }

    s3d::server::server::serve().await?;
    Ok(())
}

/// DaemonCmd is the root command of cli.
/// Uses clap (derive style) to parse the program args into flags and subcommands.
#[derive(clap::Parser, Debug, Clone)]
#[clap(name = "s3d")]
#[clap(about = clap::crate_description!())]
#[clap(version = clap::crate_version!())]
#[clap(next_display_order = 0)]
pub struct DaemonCmd {
    // TODO For now it doesn't take any input, but it will
}
