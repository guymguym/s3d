#!/bin/bash

if [ "dev.sh" = "$(basename $0)" ]
then
    echo "usage:"
    echo "$ . hack/dev.sh"
    echo "$ . hack/dev.sh clean"
    exit 1
fi

CMD() { echo -n "💬 $* ... "; "$@"; echo -e "\\r✅ $*"; }

if [ "$1" = "clean" ]
then
    CMD unset RUST_BACKTRACE
    CMD unset RUST_LOG
else
    CMD export RUST_BACKTRACE=1
    CMD export RUST_LOG=info,s3d:trace
fi
echo
echo "RUST_BACKTRACE=$RUST_BACKTRACE"
echo "RUST_LOG=$RUST_LOG"
echo
. hack/aliases.sh $*
