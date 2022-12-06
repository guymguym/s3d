use crate::codegen::s3_types;
use crate::utils::staticify;
use aws_smithy_http_server::operation::OperationShape;
use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;

pub type HttpBody = hyper::Body;
pub type HttpError = anyhow::Error;
pub type HttpRequest = hyper::Request<HttpBody>;
pub type HttpResponse = hyper::Response<HttpBody>;
pub type HttpResult = Result<HttpResponse, HttpError>;

pub async fn serve() -> anyhow::Result<()> {
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 33333));
    let mksrv = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(s3_handler)) });
    let server = hyper::Server::bind(&addr).serve(mksrv);
    info!("Listening on http://{}", addr);
    server.await?;
    Ok(())
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

async fn s3_handler(req: HttpRequest) -> HttpResult {
    log::debug!("{:?}", req);
    // aws_sigv4
    // aws_sig_auth
    let h = S3NullHandler {};
    if req.uri().path() == "/" {
        let mut op = s3_types::ListBuckets::default();
        op.receive(req);
        h.handle(&mut op);
        op.respond()
    } else {
        Err(anyhow::format_err!("todo"))
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
