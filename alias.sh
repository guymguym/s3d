# source me to add aliases to your shell
export S3D_ENDPOINT='http://localhost:5333'
alias s3d='cargo -q run --'
alias s3e='cargo -q run --example client --'
alias s3a='aws --endpoint ${S3D_ENDPOINT} s3api'
