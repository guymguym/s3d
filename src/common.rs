use aws_smithy_http_server::operation::{Handler, OperationShape};
use std::{future::Future, net::SocketAddr, pin::Pin, sync::Arc};
use tower::Service;

// use aws_smithy_http_server::body::BoxBody;
// use aws_smithy_http_server::proto::rest_xml::RestXml;
// use aws_smithy_http_server::response::IntoResponse;

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

pub type OpFuture<Op>
where
    Op: OperationShape,
= TraitFuture<'static, Op::Output, Op::Error>;

#[derive(Clone)]
pub struct OpService<Op>
where
    Op: OperationShape,
{
    pub op_name: &'static str,
    pub service: Arc<
        dyn Service<Op::Input, Response = Op::Output, Error = Op::Error, Future = OpFuture<Op>>,
    >,
}

impl<Op> std::fmt::Debug for OpService<Op>
where
    Op: OperationShape,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.op_name)
    }
}

#[async_trait::async_trait]
pub trait HttpService: Clone + Sized + Send {
    async fn serve(self, req: HttpRequest, remote_address: SocketAddr) -> HttpResult;
}

pub trait HttpRouter<T>: Clone + Sized + Send {
    fn route_request(&self, req: &hyper::Request<hyper::Body>) -> anyhow::Result<T>;
}

/// aws_sigv4, aws_sig_auth
pub trait AuthHandler: Clone + Sized + Send {
    fn verify_sig_auth(&self, req: &HttpRequest) -> anyhow::Result<()>;
}

pub trait NoAuth {}

impl<T> AuthHandler for T
where
    T: NoAuth + Clone + Send,
{
    fn verify_sig_auth(&self, req: &HttpRequest) -> anyhow::Result<()> {
        Ok(())
    }
}
