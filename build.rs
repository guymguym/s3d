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
    codegen::gen_ops::GenOps,
    codegen::gen_types::GenTypes,
    codegen::smithy_model::{FromJson, SmithyModel},
};
use std::path::Path;

/// main function of the project's cargo build script
/// See https://doc.rust-lang.org/cargo/reference/build-scripts.html
fn main() {
    println!("cargo:warning=build codegen starting...");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=codegen/");

    for model_name in ["s3", "sts", "iam"] {
        let model_file = format!("smithy-rs/aws/sdk/aws-models/{}.json", model_name);
        let out_dir = format!("src/codegen/{}", model_name);
        println!("cargo:warning=build codegen for model {}.", model_file);

        // we could should use OUT_DIR for temporary build output,
        // but for now we keep the output in source control under src/codegen just to ease debugging.
        // let out_dir = env::var("OUT_DIR").expect("OUT_DIR env is not defined");

        println!("cargo:rerun-if-changed={}", model_file);

        // load the smithy model and invoke code generators
        let model_path = Path::new(&model_file);
        let out_path = Path::new(&out_dir);
        let model = SmithyModel::from_json_file(&model_path);

        GenTypes::new(&model, &out_path.join("types.rs")).generate();
        GenOps::new(&model, &out_path.join("ops.rs")).generate();
        // GenCLI::new(&model, &out_path.join("cli.rs")).generate();
        // GenClient::new(&model, &out_path.join("client.rs")).generate();
    }

    println!("cargo:warning=build codegen done.");
}
