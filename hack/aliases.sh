#!/bin/bash

if [ "aliases.sh" = "$(basename $0)" ]
then
    echo "usage:"
    echo "$ . hack/aliases.sh"
    echo "$ . hack/aliases.sh clean"
    exit 1
fi

CMD() { echo -n "💬 $* ... "; "$@"; echo -e "\\r✅ $*"; }

if [ "$1" = "clean" ]
then
    CMD unset S3D_ENDPOINT
    CMD unalias s3d
    CMD unalias s3api
    CMD unalias s3
else
    CMD export S3D_ENDPOINT="http://localhost:33333"
    CMD alias s3d='cargo -q run --'
    CMD alias s3api='aws --endpoint ${S3D_ENDPOINT} s3api'
    CMD alias s3='aws --endpoint ${S3D_ENDPOINT} s3'
fi
echo
echo "S3D_ENDPOINT=$S3D_ENDPOINT"
alias s3d s3api s3
