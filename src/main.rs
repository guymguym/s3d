//! `s3d` is an S3 daemon for the Edge written in Rust
//! - https://s3d.rs
//! - https://github.com/s3d-rs/s3d

mod cli;
mod conf;
mod daemon;
mod fuse;
mod ops;
mod s3;

#[macro_use]
mod types;

#[macro_use]
extern crate log;

#[macro_use]
extern crate anyhow;

use crate::cli::CLI;

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    match CLI::run().await {
        Ok(_) => {
            info!("Done.");
            Ok(())
        }
        Err(err) => {
            error!("{}", err);
            std::process::exit(1);
        }
    }
}
