use crate::gen::generate_code_for_each_s3_op;
use crate::proto::TraitFuture;
use crate::store;
use aws_sdk_s3::{error::*, input::*, output::*};
use aws_smithy_http::result::SdkError;

/// This macro generates a default function for each op.
macro_rules! gen_api_default_fn {
    ($op:ident) => {
        paste::paste! {
            fn [<$op:snake>](&self, i: [<$op Input>]) -> TraitFuture<[<$op Output>], [<$op Error>]> {
                Box::pin(async move {
                    self.get_sm_client()
                        .call(i.make_operation(self.get_s3_conf()).await.unwrap())
                        .await
                        .map_err(|err| match err {
                            SdkError::ServiceError { err, .. } => err,
                            _ => [<$op Error>]::unhandled(err),
                        })
                })
            }
        }
    };
}

/// S3Api is a trait with default function per S3 op that calls and underlying SDK client.
/// This can be connected to any S3 service directly.
pub trait S3Api: Sync + Send {
    fn get_s3_conf<'a>(&'a self) -> &'a aws_sdk_s3::Config;
    fn get_sm_client<'a>(&'a self) -> &'a aws_hyper::StandardClient;
    generate_code_for_each_s3_op!(gen_api_default_fn);
}

/// S3DApi our implementation of S3Api.
#[derive(Debug)]
pub struct S3DApi {
    pub s3_client: aws_sdk_s3::Client,
    pub sm_client: aws_hyper::StandardClient,
}

impl S3DApi {
    pub fn new(s3_client: aws_sdk_s3::Client) -> Self {
        let sm_client = aws_hyper::https();
        Self {
            s3_client,
            sm_client,
        }
    }
}

impl S3Api for S3DApi {
    fn get_s3_conf<'a>(&'a self) -> &'a aws_sdk_s3::Config {
        &self.s3_client.conf()
    }
    fn get_sm_client<'a>(&'a self) -> &'a aws_hyper::StandardClient {
        &self.sm_client
    }

    fn put_object(&self, mut i: PutObjectInput) -> TraitFuture<PutObjectOutput, PutObjectError> {
        Box::pin(async move { store::put_object(i).await })
    }

    // fn get_object(&self, _i: GetObjectInput) -> TraitFuture<GetObjectOutput, GetObjectError> {
    //     Box::pin(async move {
    //         Ok(GetObjectOutput::builder()
    //             .e_tag("\"aaa-123\"")
    //             .last_modified(DateTime::from_epoch_seconds(1576540080))
    //             .content_length(5)
    //             .content_type("text/plain")
    //             .body(ByteStream::from_static(b"hello"))
    //             .build())
    //     })
    // }
}
