use crate::{
    conf::Conf,
    gen::{api_to_client::S3ApiToClient, server::serve_s3_request},
    http::*,
    xml::xml_error,
};
use hyper::{
    Body, Method, StatusCode, header::{HeaderValue, HeaderName},
    server::conn::AddrStream,
    service::{make_service_fn, service_fn},
};
use std::{convert::Infallible, net::SocketAddr};
use tokio::sync::OnceCell;

#[derive(Debug)]
pub struct Daemon {
    pub conf: Conf,
    pub s3_api: S3ApiToClient,
}

/// Daemon singleton static instance
/// Initialized once and lives throughout the program
/// because we need it to serve requests asynchronously
static DAEMON: OnceCell<Daemon> = OnceCell::const_new();

/// Run the daemon.
/// Should be called once at the start of the program.
pub async fn run(conf: Conf) -> anyhow::Result<()> {
    DAEMON.set(Daemon::new(conf).await).unwrap();
    tokio::try_join!(
        DAEMON.get().unwrap().start_fuse_mount(),
        DAEMON.get().unwrap().start_http_server(),
    )?;
    Ok(())
}

impl Daemon {
    /// Initialize the daemon with the given configuration.
    pub async fn new(conf: Conf) -> Self {
        let aws_config = aws_config::from_env().load().await;
        let s3_client = aws_sdk_s3::Client::new(&aws_config);
        Daemon {
            conf,
            s3_api: S3ApiToClient::new(s3_client),
        }
    }

    /// Starts http server with s3 service.
    pub async fn start_http_server(&'static self) -> anyhow::Result<()> {
        let addr = SocketAddr::from(([127, 0, 0, 1], self.conf.local.port));
        // using `move` to pass ownership from closure to the async service function (for remote_addr)
        let mksrv = make_service_fn(move |conn: &AddrStream| {
            let remote_addr = conn.remote_addr();
            let srv = service_fn(move |req| self.handle_http(req, remote_addr));
            async move { Ok::<_, Infallible>(srv) }
        });
        let server = hyper::Server::bind(&addr).serve(mksrv);
        info!("Listening on http://{}", addr);
        server.await?;
        Ok(())
    }

    pub async fn handle_http(
        &self,
        http_req: HttpRequest,
        remote_addr: SocketAddr,
    ) -> Result<HttpResponse, Infallible> {
        let mut req = S3Request::new(http_req, remote_addr);

        info!("==> HTTP {} {} [{}]", req.method, req.url.path(), req.reqid);

        // on debug we print also the full headers
        debug!(
            "==> HTTP {} {} headers {:#?} [{}]",
            req.method,
            req.url.path(),
            &req.headers,
            req.reqid
        );

        let res = self
            .handle_request(&mut req)
            .await
            .unwrap_or_else(|err| self.handle_error(&req, err));

        info!(
            "<== HTTP {} {} {} [{}]",
            res.status(),
            req.method,
            req.url.path(),
            req.reqid,
        );

        Ok(res)
    }

    pub async fn handle_request(&self, req: &mut S3Request) -> S3Result {
        self.check_auth(req).await?;
        if req.method == Method::OPTIONS {
            return self.handle_options(req);
        }
        let mut res = serve_s3_request(req, &self.s3_api).await?;
        self.set_headers_ids(req, &mut res);
        Ok(res)
    }

    pub fn handle_error(&self, req: &S3Request, err: S3Error) -> HttpResponse {
        error!("=== HTTP ERROR {:?}", err);
        let mut res = responder()
            .status(StatusCode::BAD_REQUEST) // TODO translate error status codes
            .body(Body::from(xml_error(err)))
            .unwrap();

        // let mut res = crate::gen::output::s3_error_output(
        //     S3Error::builder().code("InternalError").message("Internal error").build());
        self.set_headers_ids(req, &mut res);
        res
    }

    /// Authenticate and authorize the request.
    pub async fn check_auth(&self, req: &S3Request) -> Result<(), S3Error> {
        // TODO implement check_auth

        if !req.remote_addr.ip().is_loopback() {
            return Err(S3Error::builder()
                .code("Forbidden")
                .message(format!(
                    "Received request from non-local address {}",
                    req.remote_addr
                ))
                .build()
                .into());
        }

        // std::process::Command::new("lsof")
        //     .arg("-l")
        //     .arg("-P")
        //     .arg("-Fpu0")
        //     .arg(format!("-iTCP@localhost:{}", req.remote_addr.port())).output()

        Ok(())
    }

    pub fn handle_options(&self, _req: &S3Request) -> S3Result {
        Ok(responder()
            .status(StatusCode::OK)
            .header(hyper::header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
            .header(hyper::header::ACCESS_CONTROL_ALLOW_CREDENTIALS, "true")
            .header(hyper::header::ACCESS_CONTROL_ALLOW_METHODS, "GET,HEAD,PUT,POST,DELETE,OPTIONS")
            .header(hyper::header::ACCESS_CONTROL_ALLOW_HEADERS, 
                "Content-Type,Content-MD5,Authorization,X-Amz-User-Agent,X-Amz-Date,ETag,X-Amz-Content-Sha256")
            .header(hyper::header::ACCESS_CONTROL_EXPOSE_HEADERS, "ETag,X-Amz-Version-Id")
            .body(Body::empty())
            .unwrap()
        )
    }

    pub fn set_headers_ids(&self, req: &S3Request, res: &mut HttpResponse) {
        let x_amz_request_id = HeaderName::from_static("x-amz-request-id");
        let x_amz_id_2 = HeaderName::from_static("x-amz-id-2");
        let reqid_val = HeaderValue::from_str(&req.reqid).unwrap();
        let hostid_val = HeaderValue::from_str(&base64::encode(req.hostid.as_bytes())).unwrap();
        let h = res.headers_mut();
        h.insert(x_amz_request_id, reqid_val.clone());
        h.insert(x_amz_id_2, hostid_val.clone());
    }

}
