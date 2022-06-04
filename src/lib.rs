//! `s3d` is an S3 daemon for the Edge written in Rust
//! - https://s3d.rs
//! - https://github.com/s3d-rs/s3d

// #![doc = include_str!("../README.md")]
// #![allow(unused)]

pub mod cli;
pub mod codegen_include;
pub mod config;
pub mod daemon;
pub mod utils;

#[macro_use]
extern crate log;

// #[macro_use]
// extern crate clap;

// #[macro_use]
// extern crate anyhow;
