#!/bin/bash
set -e -x -o pipefail

# git submodule update --init --remote --recursive

cd smithy-rs
./gradlew -Paws.services=+s3,+sts :aws:sdk:assemble
./gradlew :s3d:finalize
cd ..

cargo run

echo ""
echo ""
echo "----------------------------"
echo "RUN WITH:"
echo "- . hack/aliases.sh"
echo "- s3d run"
echo "- s3c ls"
echo "- s3a list-buckets"
echo "----------------------------"
echo ""
echo ""
