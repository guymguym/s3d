pub mod headers;
pub mod input;
pub mod models;
pub mod ops;
pub mod output;
pub mod resources;

include!(concat!(env!("OUT_DIR"), "/s3.rs"));

