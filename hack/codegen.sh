#!/bin/bash

# run me from s3d project root.

set -e -x -o pipefail

# build with smithy-rs wrapped in sub shell to avoid polution of env
(
    set -e -x -o pipefail
    cd smithy-rs
    ./gradlew -Paws.services=+s3,+sts :aws:sdk:assemble
    ./gradlew :s3d:assemble
    #./gradlew :s3d:test
    cd ..
)

cargo run

echo ""
echo ""
echo "----------------------------"
echo "USE WITH:"
echo "- . hack/aliases.sh"
echo "- s3d run"
echo "- s3c ls"
echo "- s3a list-buckets"
echo "----------------------------"
echo ""
echo ""
