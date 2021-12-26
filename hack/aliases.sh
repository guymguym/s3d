# source me to add aliases to the shell
export S3D_ENDPOINT='http://localhost:3333'
alias s3d='cargo -q run --'
alias s3w='cargo -q run --example writer --'
alias s3r='cargo -q run --example reader --'
alias s3c='aws --endpoint ${S3D_ENDPOINT} s3'
alias s3a='aws --endpoint ${S3D_ENDPOINT} s3api'
