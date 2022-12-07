use crate::codegen::s3_types;
use aws_smithy_http_server::operation::OperationShape;
use std::{net::SocketAddr, pin::Pin, future::Future};

pub type HttpBody = hyper::Body;
pub type HttpError = anyhow::Error;
pub type HttpRequest = hyper::Request<HttpBody>;
pub type HttpResponse = hyper::Response<HttpBody>;
pub type HttpResult = Result<HttpResponse, HttpError>;

/// Why we need this TraitFuture:
/// We can't use async_trait macro inside our macro so we use the same thing it does
/// which is this pin-box-dyn-future - see long explanation here:
/// https://smallcultfollowing.com/babysteps/blog/2019/10/26/async-fn-in-traits-are-hard/
pub type TraitFuture<'a, O, E> = Pin<Box<dyn Future<Output = Result<O, E>> + Send + 'a>>;

macro_rules! gen_s3_handler_fn {
    ($op:ident) => {
        paste::paste! {
            fn [<$op:snake>](
                self,
                input: <s3_types::[<$op>] as OperationShape>::Input,
            ) -> TraitFuture<
                <s3_types::[<$op>] as OperationShape>::Output,
                <s3_types::[<$op>] as OperationShape>::Error,
            > {
                panic!("S3Server [<$op>] not implemented")
            }
        }
    };
}

#[async_trait::async_trait]
pub trait S3Handler: Clone + Sized + Send {
    s3_types::generate_ops_code!(gen_s3_handler_fn);
}

pub trait S3Router: Clone + Sized + Send {
    fn route_request(&self, req: &HttpRequest) -> anyhow::Result<s3_types::S3Ops>;
}

/// aws_sigv4, aws_sig_auth
pub trait AuthHandler: Clone + Sized + Send {
    fn verify_sig_auth(&self, req: &HttpRequest) -> anyhow::Result<()>;
}

pub trait NoAuth {}

impl<T: NoAuth> AuthHandler for T {
    fn verify_sig_auth(&self, req: &HttpRequest) -> anyhow::Result<()> {
        Ok(())
    }
}

pub trait S3InputParser: Clone + Sized + Send {
    fn parse(&self, req: &HttpRequest) -> anyhow::Result<s3_types::S3Ops>;
}



#[async_trait::async_trait]
pub trait HttpHandler: Clone + Sized + Send {
    async fn handle(self, req: HttpRequest, remote_address: SocketAddr) -> HttpResult;
}

// implement the htto handling for a given S3Handler and AuthHandler
#[async_trait::async_trait]
impl<S3> HttpHandler for S3
where
    S3: AuthHandler + S3Router + S3Handler,
{
    async fn handle(self, req: HttpRequest, _remote_address: SocketAddr) -> HttpResult {
        macro_rules! gen_s3_handler_call {
            ($op:ident) => {
                paste::paste! {
                    {
                        let input = <s3_types::[<$op>] as OperationShape>::Input::default();
                        let output = self.[<$op:snake>](input)?;
                        
                        HttpResult::Ok(HttpResponse::new(HttpBody::from(format!(
                            "<ListAllMyBucketsResult>
                                <Owner>
                                    <ID>1</ID>
                                    <DisplayName>User1</DisplayName>
                                </Owner>
                                <Buckets>
                                    <Bucket>
                                        <Name>lala</Name>
                                        <CreationDate>19820608T001122Z</CreationDate>
                                    </Bucket>
                                </Buckets>
                            </ListAllMyBucketsResult>"
                        ))))
                    }
                }
            };
        }

        log::debug!("{:?}", req);
        self.verify_sig_auth(&req)?;
        let op = self.route_request(&req)?;
        let res = s3_types::generate_ops_match!(gen_s3_handler_call, op);
        log::debug!("{:?}", res);
        res
    }
}

// if req.uri().path() == "/" {
//     // let mut op = s3_types::ListBuckets::default();
//     // op.receive(req);
//     // h.handle(&mut op);
//     let res = self.list_buckets(()).unwrap();
//     log::debug!("res {:?}", res);
//     Ok(HttpResponse::new(HttpBody::from(format!(
//         "<ListAllMyBucketsResult>
//             <Owner>
//                 <ID>1</ID>
//                 <DisplayName>User1</DisplayName>
//             </Owner>
//             <Buckets>
//                 <Bucket>
//                     <Name>lala</Name>
//                     <CreationDate>19820608T001122Z</CreationDate>
//                 </Bucket>
//             </Buckets>
//         </ListAllMyBucketsResult>"
//     ))))
// } else {
//     Err(anyhow::format_err!("todo"))
// }
