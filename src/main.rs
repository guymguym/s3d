//! `s3d` is an S3 daemon for the Edge written in Rust
//! - https://s3d.rs
//! - https://github.com/s3d-rs/s3d

mod cli;
mod conf;
mod daemon;
mod status;
mod err;

#[macro_use]
mod util;

#[macro_use]
extern crate log;

use crate::cli::CLI;
use crate::util::*;

#[tokio::main]
pub async fn main() -> ResultOrAnyErr<()> {
    match CLI::run().await {
        Ok(_) => {
            info!("Done.");
            Ok(())
        }
        Err(err) => {
            error!("Exit on Error: {}", err);
            std::process::exit(1);
        }
    }
}
