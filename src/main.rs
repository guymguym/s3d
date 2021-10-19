//! `s3d` is an S3 daemon for the Edge written in Rust
//! - https://s3d.rs
//! - https://github.com/s3d-rs/s3d

#[macro_use]
mod util;

mod cli;
mod conf;
mod s3;

use crate::cli::CLI;
use crate::util::*;

#[tokio::main]
pub async fn main() -> ResultOrAnyErr<()> {
    CLI::run().await
}
