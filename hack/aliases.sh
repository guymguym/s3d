export S3D_ENDPOINT='http://localhost:3333'
alias s3d='cargo -q run --'
alias s3='aws --endpoint ${S3D_ENDPOINT} s3'
alias s3api='aws --endpoint ${S3D_ENDPOINT} s3api'

[ "aliases.sh" = "$(basename $0)" ] && echo "usage: . hack/aliases.sh"
