#!/bin/bash
# 
# for now this script assumes that:
# 1. smithy-rs cloned in ../smithy-rs
# 2. s3d branch checked out (TBD location)
#
set -e -x -o pipefail

SMITHY_DIR="../smithy-rs"
TARGET_DIR="smithy"
S3_MODEL_SOURCE="$SMITHY_DIR/aws/sdk/aws-models/s3.json"
S3_MODEL_TARGET="$TARGET_DIR/models/s3.json"
S3_SERVER_SOURCE="$SMITHY_DIR/s3d/build/smithyprojections/s3d/s3/rust-server-codegen/src"
S3_SERVER_TARGET="$TARGET_DIR/s3-server-codegen/src"
TARGETS="$S3_MODEL_TARGET $S3_SERVER_TARGET"

true && (cd $SMITHY_DIR && exec ./gradlew :s3d:assemble)
rm -rf $TARGETS
mkdir -p $(dirname $S3_MODEL_TARGET) $(dirname $S3_SERVER_TARGET)
cp -R $S3_MODEL_SOURCE $S3_MODEL_TARGET
cp -R $S3_SERVER_SOURCE $S3_SERVER_TARGET
git add $TARGETS
