use crate::{
    codegen::s3::{ops, types},
    common::{AuthHandler, HttpRequest, HttpResult, HttpRouter, HttpService, TraitFuture},
    config,
};
use aws_smithy_http_server::operation::{HandlerExt, OperationShape};
use hyper::{
    server::conn::AddrStream,
    service::{make_service_fn, service_fn},
};
use std::{convert::Infallible, net::SocketAddr};

/// serve runs an http rest server for a configured port.
/// constructs an S3 server which is used as an archtype of the server configuration,
/// and will be cloned for every incoming request to avoid locking (rethink if it gets too big).
pub async fn serve() -> anyhow::Result<()> {
    let addr = format!("127.0.0.1:{}", *config::PORT).parse()?;
    let s3 = S3MemServer {
        bucket_name: "mem".to_string(),
    };
    run_server(addr, s3).await
}

pub async fn run_server<S>(addr: SocketAddr, http_service: S) -> anyhow::Result<()>
where
    S: HttpService + 'static,
{
    let service = make_service_fn(|conn: &AddrStream| {
        let http_service = http_service.clone();
        let remote_addr = conn.remote_addr();
        let s = service_fn(move |req| http_service.clone().handle(req, remote_addr));
        async move { Ok::<_, Infallible>(s) }
    });
    let server = hyper::Server::bind(&addr).serve(service);
    log::info!("Serving -> http://{}", addr);
    server.await?;
    Ok(())
}

// implement the htto handling for a given S3Handler and AuthHandler
#[async_trait::async_trait]
impl<S> HttpService for S
where
    S: AuthHandler + HttpRouter<ops::Ops>,
{
    async fn serve(self, req: HttpRequest, remote_address: SocketAddr) -> HttpResult {
        macro_rules! gen_s3_handler_call {
            ($op:ident) => {
                paste::paste! {
                    {
                        let input = <types::[<$op>] as OperationShape>::Input::default();
                        let output = self.[<$op:snake>](input)?;
                        Ok(output.into_response())

                        // HttpResult::Ok(HttpResponse::new(HttpBody::from(format!(
                        //     "<ListAllMyBucketsResult>
                        //         <Owner>
                        //             <ID>1</ID>
                        //             <DisplayName>User1</DisplayName>
                        //         </Owner>
                        //         <Buckets>
                        //             <Bucket>
                        //                 <Name>lala</Name>
                        //                 <CreationDate>19820608T001122Z</CreationDate>
                        //             </Bucket>
                        //         </Buckets>
                        //     </ListAllMyBucketsResult>"
                        // ))))
                    }
                }
            };
        }

        log::debug!("{:?}", req);
        self.verify_sig_auth(&req)?;
        let op = self.route_request(&req)?;
        let res = ops::generate_ops_match!(gen_s3_handler_call, op);
        log::debug!("{:?}", res);
        res
    }
}

#[derive(Debug, Clone)]
pub struct S3MemServer {
    bucket_name: String,
}
// impl NoAuth for S3MemServer {}
impl S3MemServer {
    fn into_service() -> types::AmazonS3 {
        types::AmazonS3 {
            list_buckets: Some(Self::list_buckets.into_service()),
            ..Default::default()
        }
    }
    fn list_buckets(
        self,
        input: <types::ListBuckets as OperationShape>::Input,
    ) -> TraitFuture<
        'static,
        <types::ListBuckets as OperationShape>::Output,
        <types::ListBuckets as OperationShape>::Error,
    > {
        Box::pin(async move {
            Ok(types::ListBucketsOutput {
                buckets: Some(vec![types::Bucket {
                    name: Some(self.bucket_name),
                    creation_date: Some("19820608T001122Z".to_string()),
                }]),
                owner: Some(types::Owner {
                    id: Some("1".to_string()),
                    display_name: Some("User1".to_string()),
                }),
            })
        })
    }
}

// pub trait S3OpReceiver {
//     fn receive(&mut self, req: HttpRequest);
// }
// pub trait S3OpResponder {
//     fn respond(&mut self) -> HttpResult;
// }
// impl S3OpReceiver for s3_types::ListBuckets {
//     fn receive(&mut self, req: HttpRequest) {}
// }
// impl S3OpResponder for s3_types::ListBuckets {
//     fn respond(&mut self) -> HttpResult {
//         Ok(HttpResponse::new(HttpBody::from(format!(
//             "<ListAllMyBucketsResult>
//                 <Owner>
//                     <ID>1</ID>
//                     <DisplayName>User1</DisplayName>
//                 </Owner>
//                 <Buckets>
//                     <Bucket>
//                         <Name1>lala</Name1>
//                         <CreationDate>19820608T001122Z</CreationDate>
//                     </Bucket>
//                 </Buckets>
//             </ListAllMyBucketsResult>"
//         ))))
//     }
// }

// let op = s3_types::[<$op>]::default();
// let input = crate::gen::server::[<$op>]::decode_input(req).await?;
// debug!("input {:?}", input);
// let output = self.s3d_api.[<$op:snake>](input).await.map_err(|err| err.meta().clone())?;
// debug!("output {:?}", output);
// let response = crate::gen::server::[<$op>]::encode_output(output).await?;
// debug!("response {:?}", response);
// response
// responder().body(Body::empty())?

// pub trait S3OpHandler<Op> {
//     fn handle(self, op: &mut Op);
// }
// pub struct S3NullHandler {}
// impl S3OpHandler<s3_types::ListBuckets> for S3NullHandler {
//     fn handle(self, op: &mut s3_types::ListBuckets) {
//         op.output.buckets = Some(vec![s3_types::Bucket {
//             name: Some("lala".to_string()),
//             creation_date: Some("19820608T001122Z".to_string()),
//         }]);
//         op.output.owner = Some(s3_types::Owner {
//             id: Some("1".to_string()),
//             display_name: Some("User1".to_string()),
//         });
//     }
// }

/*

use aws_smithy_async::future::now_or_later::BoxFuture;
use aws_smithy_http_server::{
    operation::{Handler, OperationShape, OperationShapeExt},
    routing::request_spec::{PathAndQuerySpec, PathSpec, QuerySpec, RequestSpec, UriSpec},
};

pub type Router = aws_smithy_http_server::Router<hyper::Body>;

// let s3_config = aws_config::load_from_env().await;
// let s3_client: &'static _ = staticify(aws_sdk_s3::Client::new(&s3_config));
// let router = make_router();

pub struct OperationHandler<Op>
where
    Op: OperationShape,
{
    op: Op,
}

pub struct OperationExt {}

impl<Op> Handler<Op, OperationExt> for OperationHandler<Op>
where
    Op: OperationShape,
{
    type Future = BoxFuture<'static, Result<Op::Output, Op::Error>>;

    fn call(&mut self, input: Op::Input, exts: OperationExt) -> Self::Future {
        todo!();
    }
}

pub fn make_router() -> Router {
    type I = <s3_types::ListObjects as OperationShape>::Input;
    type O = <s3_types::ListObjects as OperationShape>::Output;

    // async fn handler(i: I) -> O {
    //     todo!("todo")
    // }
    let handler = OperationHandler {
        op: s3_types::ListObjects {},
    };

    let op = s3_types::ListObjects::from_handler(handler);
    Router::new_rest_xml_router(vec![(
        tower::util::BoxCloneService::new(op.inner),
        RequestSpec::new(
            Method::GET,
            UriSpec {
                host_prefix: None,
                path_and_query: PathAndQuerySpec {
                    path_segments: PathSpec::from_vector_unchecked(vec![]),
                    query_segments: QuerySpec::from_vector_unchecked(vec![]),
                },
            },
        ),
    )])
}
*/
