#!/bin/bash
# run me from s3d project root.
set -e -x -o pipefail

echo ">>> Building smithy-rs/codegen-s3d"
(
    # build with smithy-rs wrapped in sub shell to avoid polution of working dir (or env)
    set -e -x -o pipefail
    cd smithy-rs
    ./gradlew -Paws.services=+sts,+sso,+s3 :aws:sdk:assemble
    ./gradlew :codegen-s3d:assemble
    cd ..
)

echo ">>> Building s3d-rs"
cargo run
