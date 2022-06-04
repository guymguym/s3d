//! # build.rs for s3d
//!
//! This is the cargo build script which is called during build.
//! It is used to generate code for the S3 protocol in addition to what smithy-rs generates.
//! It reads the S3 smithy model as input, and writes generated code out to `$OUT_DIR/`,
//! which is then included! in the src/codegen_include.rs file.
//! Smithy-rs already generates a lot of the code from the model,
//! such as client SDK, and Server SDK, but we need additional code to make it work.
//!
//! The S3 protocol is defined in a Smithy JSON AST model:
//! - https://github.com/awslabs/smithy-rs/blob/main/aws/sdk/aws-models/s3.json
//!
//! NOTE: For now we prefered to write this codegen in rust, although an alternative would be
//! to write this in java/kotlin under `./smithy-rs/s3d/`  which is a dir in our fork here -
//! https://github.com/s3d-rs/smithy-rs (rebased regularly from awslabs/smithy-rs).

// codegen module implements the generation flow (build only module, not under src)
mod codegen;

use crate::{
    codegen::gen_commands::GenCommands,
    codegen::gen_converters::GenConverters,
    codegen::smithy_model::{FromJson, SmithyModel},
};
use std::{env, path::Path};

/// main function of the project's cargo build script
/// See https://doc.rust-lang.org/cargo/reference/build-scripts.html
fn main() {
    println!("cargo:warning=build codegen starting...");

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR env var not set");
    let out_path = Path::new(out_dir.as_str());
    let model_path = Path::new("smithy-rs/aws/sdk/aws-models/s3.json");

    // printing out cargo directives
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=codegen/");
    println!("cargo:rerun-if-changed={}", model_path.display());

    // load the smithy model and invoke code generators
    let model = SmithyModel::from_json_file(&model_path);
    GenCommands::new(&model, &out_path.join("s3_cli.rs")).generate();
    GenConverters::new(&model, &out_path.join("s3_conv.rs")).generate();

    println!("cargo:warning=build codegen done.");
}
