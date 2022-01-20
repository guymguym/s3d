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
)

echo ">>> Sync generated code in git"
mkdir -p smithy-codegen-s3d/s3d-codegen/
cp -R smithy-codegen-s3d/build/smithyprojections/codegen-s3d/s3/rust-server-codegen/ smithy-codegen-s3d/s3d-codegen/

echo ">>> Building s3d-rs"
cargo run
