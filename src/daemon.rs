use crate::{
    conf::Conf,
    err::*,
    ops::{self, S3BucketSubResource, S3ObjectSubResource, S3Request},
    parse,
    util::*,
};
use hyper::{
    header,
    server::conn::AddrStream,
    service::{make_service_fn, service_fn},
    Body, HeaderMap, Server,
};
use std::{collections::HashMap, convert::Infallible, net::SocketAddr};
use tokio::sync::OnceCell;
use url::Url;
use uuid::Uuid;

pub type S3C = aws_sdk_s3::Client;
pub type AC = aws_smithy_client::Client<aws_hyper::DynConnector, aws_hyper::AwsMiddleware>;

#[derive(Debug)]
pub struct Daemon {
    pub conf: Conf,
    pub s3c: S3C,
    pub ac: AC,
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
        let s3_config = aws_config::from_env().load().await;
        let s3c = aws_sdk_s3::Client::new(&s3_config);
        let retry_config = s3_config.retry_config().unwrap();
        let ac = aws_hyper::Client::https().with_retry_config(retry_config.clone().into());
        Daemon { conf, s3c, ac }
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
        let server = Server::bind(&addr).serve(mksrv);
        info!("Listening on http://{}", addr);
        server.await?;
        Ok(())
    }

    pub async fn handle_http(&self, http_req: HttpRequest, remote_addr: SocketAddr) -> HttpResult {
        // self.handle_auth(&http_req).await?;
        // let op = self.parse_http(http_req);
        // self.handle_op(op).await?

        let mut req = self.new_request(http_req, remote_addr);
        info!("==> HTTP {} {} [{}]", req.method, req.url.path(), req.reqid);
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

    pub fn new_request(&self, http_req: HttpRequest, remote_addr: SocketAddr) -> S3Request {
        let (parts, body) = http_req.into_parts();
        let host = parts.headers.get(header::HOST).unwrap().to_str().unwrap();
        let base_url = Url::parse(&format!("http://{}", host)).unwrap();
        let url = base_url.join(&parts.uri.to_string()).unwrap();
        let reqid = Uuid::new_v4().to_string();
        S3Request {
            remote_addr,
            url,
            body,
            method: parts.method,
            headers: parts.headers,
            // generate a unique id for each request
            reqid,
            // initialize empty fields and prepare_request() will fill later
            params: HashMap::<String, String>::new(),
            bucket: "".to_owned(),
            key: "".to_owned(),
            bucket_subresource: S3BucketSubResource::None,
            object_subresource: S3ObjectSubResource::None,
        }
    }

    pub async fn handle_request(&self, req: &mut S3Request) -> S3Result {
        self.prepare_request(req).await?;
        self.handle_auth(req).await?;
        let mut res = match req.method.as_str() {
            "GET" => self.handle_get(req).await,
            "HEAD" => self.handle_head(req).await,
            "PUT" => self.handle_put(req).await,
            "POST" => self.handle_post(req).await,
            "DELETE" => self.handle_delete(req).await,
            "OPTIONS" => self.handle_options(req).await,
            "FETCH" => self.handle_fetch(req).await,
            "PULL" => self.handle_pull(req).await,
            "PUSH" => self.handle_push(req).await,
            "PRUNE" => self.handle_prune(req).await,
            "STATUS" => self.handle_status(req).await,
            "DIFF" => self.handle_diff(req).await,
            _ => Err(S3Error::new(S3Errors::BadRequest)),
        };
        if let Ok(ref mut res) = res {
            self.set_headers_reqid(res.headers_mut(), &req.reqid);
        }
        res
    }

    pub async fn prepare_request(&self, req: &mut S3Request) -> Result<(), S3Error> {
        // parse path-style addressing for bucket names
        // TODO: add support for host-style addressing
        assert!(req.url.path().starts_with("/"));
        let path_items: Vec<_> = req.url.path()[1..].splitn(2, "/").collect();

        match path_items.len() {
            0 => {
                for (key, val) in req.url.query_pairs() {
                    req.params.insert(String::from(key), String::from(val));
                }
            }
            1 => {
                req.bucket = path_items[0].to_owned();
                for (key, val) in req.url.query_pairs() {
                    let sub = S3BucketSubResource::from(key.as_ref());
                    if sub != S3BucketSubResource::None
                        && req.bucket_subresource != S3BucketSubResource::None
                        && sub != req.bucket_subresource
                    {
                        error!(
                            "Multiple bucket subresources specified: {:?} and {:?}",
                            req.bucket_subresource, sub
                        );
                        return Err(S3Error::new(S3Errors::BadRequest));
                    }
                    req.bucket_subresource = sub;
                    req.params.insert(String::from(key), String::from(val));
                }
            }
            2 => {
                req.bucket = path_items[0].to_owned();
                req.key = path_items[1].to_owned();
                for (key, val) in req.url.query_pairs() {
                    let sub = S3ObjectSubResource::from(key.as_ref());
                    if sub != S3ObjectSubResource::None
                        && req.object_subresource != S3ObjectSubResource::None
                        && sub != req.object_subresource
                    {
                        error!(
                            "Multiple object subresources specified: {:?} and {:?}",
                            req.object_subresource, sub
                        );
                        return Err(S3Error::new(S3Errors::BadRequest));
                    }
                    req.object_subresource = sub;
                    req.params.insert(String::from(key), String::from(val));
                }
            }
            _ => panic!("unexpected path items split {:?}", req),
        };

        debug!(
            "==> HTTP {} {} headers {:#?} [{}]",
            req.method,
            req.url.path(),
            &req.headers,
            req.reqid
        );

        Ok(())
    }

    /// Authenticate and authorize the request.
    pub async fn handle_auth(&self, req: &S3Request) -> Result<(), S3Error> {
        // TODO implement authenticate
        if !req.remote_addr.ip().is_loopback() {
            warn!(
                "Received request from non-local address {}",
                req.remote_addr
            );
            return Err(S3Error::new(S3Errors::Forbidden));
        }

        // std::process::Command::new("lsof")
        //     .arg("-l")
        //     .arg("-P")
        //     .arg("-Fpu0")
        //     .arg(format!("-iTCP@localhost:{}", req.remote_addr.port())).output()

        Ok(())
    }

    pub fn handle_error(&self, req: &S3Request, err: S3Error) -> HttpResponse {
        let mut res = http_response()
            .status(err.status_code)
            .header(header::CONTENT_TYPE, "application/xml")
            .body(Body::from(format!(
                "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
                <Error>\
                    <Code>{}</Code>\
                    <Message>{}</Message>\
                    <Resource>{}</HostId>\
                    <RequestId>{}</RequestId>\
                </Error>",
                err.code,
                err.msg,
                req.url.path(),
                req.reqid,
            )))
            .unwrap();
        self.set_headers_reqid(res.headers_mut(), &req.reqid);
        res
    }

    pub async fn handle_get(&self, req: &S3Request) -> S3Result {
        if req.bucket.is_empty() {
            return parse::list_buckets_output(
                ops::list_buckets(&self.s3c, &self.ac, parse::list_buckets_input(req)?).await?,
            );
        }
        if req.key.is_empty() {
            return match req.bucket_subresource {
                S3BucketSubResource::None => parse::list_objects_output(
                    ops::list_objects(&self.s3c, &self.ac, parse::list_objects_input(req)?).await?,
                ),
                _ => ops::get_bucket_subresource(req).await,
            };
        }
        match req.object_subresource {
            S3ObjectSubResource::None => parse::get_object_output(
                ops::get_object(&self.s3c, &self.ac, parse::get_object_input(req)?).await?,
            ),
            _ => ops::get_object_subresource(req).await,
        }
    }

    pub async fn handle_head(&self, req: &S3Request) -> S3Result {
        if req.bucket.is_empty() {
            return Err(S3Error::new(S3Errors::BadRequest));
        }
        if req.key.is_empty() {
            return ops::head_bucket(req).await;
        }
        ops::head_object(req).await
    }

    pub async fn handle_put(&self, req: &S3Request) -> S3Result {
        if req.bucket.is_empty() {
            return Err(S3Error::new(S3Errors::BadRequest));
        }
        if req.key.is_empty() {
            return match req.bucket_subresource {
                S3BucketSubResource::None => ops::put_bucket(req).await,
                _ => ops::put_bucket_subresource(req).await,
            };
        }
        match req.object_subresource {
            S3ObjectSubResource::None => ops::put_object(req).await,
            _ => ops::put_object_subresource(req).await,
        }
    }

    pub async fn handle_delete(&self, req: &S3Request) -> S3Result {
        if req.bucket.is_empty() {
            return Err(S3Error::new(S3Errors::BadRequest));
        }
        if req.key.is_empty() {
            return match req.bucket_subresource {
                S3BucketSubResource::None => ops::delete_bucket(req).await,
                _ => ops::delete_bucket_subresource(req).await,
            };
        }
        match req.object_subresource {
            S3ObjectSubResource::None => ops::delete_object(req).await,
            _ => ops::delete_object_subresource(req).await,
        }
    }

    pub async fn handle_post(&self, req: &S3Request) -> S3Result {
        if req.bucket.is_empty() {
            return Err(S3Error::new(S3Errors::BadRequest));
        }
        if req.key.is_empty() {
            return match req.bucket_subresource {
                S3BucketSubResource::None => ops::post_object(req).await,
                _ => ops::post_bucket_subresource(req).await,
            };
        }
        match req.object_subresource {
            S3ObjectSubResource::Uploads => ops::create_multipart_upload(req).await,
            S3ObjectSubResource::UploadId => ops::complete_multipart_upload(req).await,
            _ => Err(S3Error::new(S3Errors::BadRequest)),
        }
    }

    pub async fn handle_options(&self, req: &S3Request) -> S3Result {
        let mut res = HttpResponse::new(Body::empty());
        self.set_headers_cors(&req, &mut res);
        Ok(res)
    }

    pub async fn handle_fetch(&self, req: &S3Request) -> S3Result {
        ops::fetch_flow(req).await
    }

    pub async fn handle_pull(&self, req: &S3Request) -> S3Result {
        ops::pull_flow(req).await
    }

    pub async fn handle_push(&self, req: &S3Request) -> S3Result {
        ops::push_flow(req).await
    }

    pub async fn handle_prune(&self, req: &S3Request) -> S3Result {
        ops::prune_flow(req).await
    }

    pub async fn handle_status(&self, req: &S3Request) -> S3Result {
        ops::status_flow(req).await
    }

    pub async fn handle_diff(&self, req: &S3Request) -> S3Result {
        ops::diff_flow(req).await
    }

    pub fn set_headers_reqid(&self, h: &mut HeaderMap, reqid: &str) {
        let x_amz_request_id = header::HeaderName::from_static("x-amz-request-id");
        let x_amz_id_2 = header::HeaderName::from_static("x-amz-id-2");
        let reqid_val = header::HeaderValue::from_str(reqid).unwrap();
        h.insert(x_amz_request_id, reqid_val.clone());
        h.insert(x_amz_id_2, reqid_val.clone());
    }

    pub fn set_headers_cors(&self, _req: &S3Request, res: &mut HttpResponse) {
        // note that browsers do not really allow origin=* with allow credentials
        let h = res.headers_mut();
        h.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*".parse().unwrap());
        h.insert(
            header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
            "true".parse().unwrap(),
        );
        h.insert(
            header::ACCESS_CONTROL_ALLOW_METHODS,
            "GET,POST,PUT,DELETE,OPTIONS".parse().unwrap(),
        );
        h.insert(header::ACCESS_CONTROL_ALLOW_HEADERS,
            "Content-Type,Content-MD5,Authorization,X-Amz-User-Agent,X-Amz-Date,ETag,X-Amz-Content-Sha256".parse().unwrap());
        h.insert(
            header::ACCESS_CONTROL_EXPOSE_HEADERS,
            "ETag,X-Amz-Version-Id".parse().unwrap(),
        );
    }
}
