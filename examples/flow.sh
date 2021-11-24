#!/bin/bash
set -e -o pipefail

export S3D_ENDPOINT='http://localhost:3333'

function NL() {
    echo ""
}

function LOG() {
    echo "*** $@"
}

function CURL() {
    LOG "curl $@"
    NL
    curl -s -i ${S3D_ENDPOINT}$@
    NL
}

function S3() {
    LOG "aws s3 $@"
    NL
    aws --endpoint ${S3D_ENDPOINT} s3 $@
    NL
}

function S3API() {
    LOG "aws s3api $@"
    NL
    aws --endpoint ${S3D_ENDPOINT} s3api $@
    NL
}

# setup
NL
S3 ls
S3 mb s3://lala
S3API head-bucket --bucket lala
S3 ls
# NL
# CURL /
# CURL /lala -X PUT
# CURL /lala -I
# CURL /

# test
S3 ls s3://lala
S3 cp README.md s3://lala/README.md
S3 cp s3://lala/README.md -
# CURL /lala -X GET
# CURL /lala/README.md -X PUT -d @README.md
# CURL /lala/README.md -X GET

# cleanup
S3 rb s3://lala
S3 ls
# CURL /lala -X DELETE
# CURL /
