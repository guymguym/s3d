#!/bin/bash
set -e -x -o pipefail

# git submodule update --init --remote --recursive

cd smithy-rs
./gradlew -Paws.services=+s3,+sts :aws:sdk:assemble
./gradlew :s3d:assemble
