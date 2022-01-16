# S3D Codegen

This generates smithy-rs s3 server code for s3d.
The Smithy build plugin invokes our codegen machinery and generates Rust crates under `./build`.

From the project root of s3d, use:
```
./hack/codegen.sh
```

From within the project root of smithy-rs this would essentially be like running:

```
./gradlew -Paws.services=+s3,+sts :aws:sdk:assemble
./gradlew :s3d:assemble
```

The `test` task also validates that the generated Rust compiles and lints (`cargo check`, `cargo clippy`, etc)
```
./gradlew :s3d:test
```
