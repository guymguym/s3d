use crate::{gen::ops::generate_code_for_each_s3_op, http::*};
use aws_sdk_s3::{error::*, input::*, output::*};
use aws_smithy_http::result::SdkError;
use std::future::Future;
use std::pin::Pin;

/// Why we need this TraitFuture:
/// We can't use async_trait macro inside our macro so we use the same thing it does
/// which is this pin-box-dyn-future - see long explanation here:
/// https://smallcultfollowing.com/babysteps/blog/2019/10/26/async-fn-in-traits-are-hard/
pub type TraitFuture<'a, O, E> = Pin<Box<dyn Future<Output = Result<O, E>> + Send + 'a>>;

/// This macro generates a default function for each op.
macro_rules! gen_api_trait_fn {
    ($op:ident) => {
        paste::paste! {
            fn [<$op:snake>](&self, _: [<$op Input>]) -> TraitFuture<[<$op Output>], [<$op Error>]> {
                Box::pin(async move { Err([<$op Error>]::generic(self.not_implemented())) })
            }
        }
    };
}

/// This macro generates a default function for each op.
macro_rules! gen_api_to_client_fn {
    ($op:ident) => {
        paste::paste! {
            fn [<$op:snake>](&self, i: [<$op Input>]) -> TraitFuture<[<$op Output>], [<$op Error>]> {
                Box::pin(async move {
                    self.sm_client
                        .call(i.make_operation(&self.s3_client.conf()).await.unwrap())
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

pub trait S3Api: Sync + Send {
    fn not_implemented(&self) -> S3Error {
        S3Error::builder()
            .code("NotImplemented")
            .message("The requested action is not implemented.")
            .build()
    }
    generate_code_for_each_s3_op!(gen_api_trait_fn);
}

/// S3ApiToClient is the default implementation of S3Api that uses the client SDK to call for each function.
/// This can be connected to any remote S3 service directly.
#[derive(Debug)]
pub struct S3ApiToClient {
    pub s3_client: aws_sdk_s3::Client,
    pub sm_client: aws_hyper::StandardClient,
}

impl S3ApiToClient {
    pub fn new(s3_client: aws_sdk_s3::Client) -> Self {
        let sm_client = aws_hyper::https();
        Self {
            s3_client,
            sm_client,
        }
    }
}

impl S3Api for S3ApiToClient {
    generate_code_for_each_s3_op!(gen_api_to_client_fn);
}

// fn put_object(&self, i: PutObjectInput) -> TraitFuture<PutObjectOutput, PutObjectError> {
//     Box::pin(async move {
//         Ok(PutObjectOutput::builder().build())
//     })
// }
