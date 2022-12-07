use crate::{
    codegen::s3_types,
    config,
    daemon::handlers::{HttpHandler, HttpRequest, HttpResult, NoAuth, TraitFuture},
};
use aws_smithy_http_server::operation::OperationShape;
use hyper::{
    server::conn::AddrStream,
    service::{make_service_fn, service_fn},
};
use std::{convert::Infallible, net::SocketAddr};

/// serve runs an http rest server for a configured port.
/// constructs an S3 handler which serves as an archtype of the server configuration,
/// and will be cloned by every incoming request to avoid locking (rethink if it gets too big).
pub async fn serve() -> anyhow::Result<()> {
    let addr = format!("127.0.0.1:{}", *config::PORT).parse()?;
    let s3_handler = S3Test {
        bucket_name: "lala".to_string(),
    };
    run_server(addr, s3_handler).await?;
    Ok(())
}

pub async fn run_server<H>(addr: SocketAddr, handler: H) -> anyhow::Result<()>
where
    H: HttpHandler + 'static,
{
    let service = make_service_fn(|conn: &AddrStream| {
        let handler = handler.clone();
        let remote_addr = conn.remote_addr();
        let srv = service_fn(move |req| handler.clone().handle(req, remote_addr));
        async move { Ok::<_, Infallible>(srv) }
    });
    let server = hyper::Server::bind(&addr).serve(service);
    log::info!("Serving -> http://{}", addr);
    server.await?;
    Ok(())
}

#[derive(Debug, Clone)]
pub struct S3Test {
    bucket_name: String,
}
impl NoAuth for S3Test {}
impl S3 for S3Test {
    fn list_buckets(
        self,
        input: <s3_types::ListBuckets as OperationShape>::Input,
    ) -> TraitFuture<
        <s3_types::ListBuckets as OperationShape>::Output,
        <s3_types::ListBuckets as OperationShape>::Error,
    > {
        Box::pin(async move {
            Ok(s3_types::ListBucketsOutput {
                buckets: Some(vec![s3_types::Bucket {
                    name: Some(self.bucket_name),
                    creation_date: Some("19820608T001122Z".to_string()),
                }]),
                owner: Some(s3_types::Owner {
                    id: Some("1".to_string()),
                    display_name: Some("User1".to_string()),
                }),
            })
        })
    }
}

pub trait S3OpReceiver {
    fn receive(&mut self, req: HttpRequest);
}
pub trait S3OpResponder {
    fn respond(&mut self) -> HttpResult;
}

impl S3OpReceiver for s3_types::ListBuckets {
    fn receive(&mut self, req: HttpRequest) {}
}
impl S3OpResponder for s3_types::ListBuckets {
    fn respond(&mut self) -> HttpResult {
        Ok(HttpResponse::new(HttpBody::from(format!(
            "<ListAllMyBucketsResult>
                <Owner>
                    <ID>1</ID>
                    <DisplayName>User1</DisplayName>
                </Owner>
                <Buckets>
                    <Bucket>
                        <Name1>lala</Name1>
                        <CreationDate>19820608T001122Z</CreationDate>
                    </Bucket>
                </Buckets>
            </ListAllMyBucketsResult>"
        ))))
    }
}

// let op = s3_types::[<$op>]::default();
// let input = crate::gen::server::[<$op>]::decode_input(req).await?;
// debug!("input {:?}", input);
// let output = self.s3d_api.[<$op:snake>](input).await.map_err(|err| err.meta().clone())?;
// debug!("output {:?}", output);
// let response = crate::gen::server::[<$op>]::encode_output(output).await?;
// debug!("response {:?}", response);
// response
// responder().body(Body::empty())?

pub trait S3OpHandler<Op> {
    fn handle(self, op: &mut Op);
}

pub struct S3NullHandler {}

impl S3OpHandler<s3_types::ListBuckets> for S3NullHandler {
    fn handle(self, op: &mut s3_types::ListBuckets) {
        op.output.buckets = Some(vec![s3_types::Bucket {
            name: Some("lala".to_string()),
            creation_date: Some("19820608T001122Z".to_string()),
        }]);
        op.output.owner = Some(s3_types::Owner {
            id: Some("1".to_string()),
            display_name: Some("User1".to_string()),
        });
    }
}

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
