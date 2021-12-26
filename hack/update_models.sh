set -e -o pipefail

# set this to the version of the models you want to update
BRANCH_OR_TAG=main

curl https://raw.githubusercontent.com/awslabs/smithy-rs/$BRANCH_OR_TAG/aws/sdk/aws-models/s3.json > smithy/models/s3.json
