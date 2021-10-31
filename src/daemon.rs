use crate::{conf::Conf, err::*, store, util::*};
use hyper::{
    header,
    service::{make_service_fn, service_fn},
    Body, HeaderMap, Method, Server,
};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::sync::OnceCell;
use uuid::Uuid;

#[derive(Debug)]
pub struct Daemon {}

// keep the server alive statically
// because we need it for the lifetime of the program
static DAEMON: OnceCell<Daemon> = OnceCell::const_new();

/// Starts http server with s3 service.
/// This should be called only once at the start of the program.
pub async fn run(conf: Conf) -> anyhow::Result<()> {
    let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let addr = SocketAddr::new(ip, conf.local.port);
    DAEMON.set(Daemon {}).unwrap();
    let service = make_service_fn(|_| async {
        Ok::<_, anyhow::Error>(service_fn(|req: HttpRequest| async {
            DAEMON.get().unwrap().serve_request(req).await
        }))
    });
    let server = Server::bind(&addr).serve(service);
    info!("Listening on http://{}", addr);
    server.await?;
    Ok(())
}

impl Daemon {
    pub async fn serve_request(&self, mut req: HttpRequest) -> HttpResult {
        // generate request uuid
        let reqid = Uuid::new_v4().to_string();

        info!(
            "<== HTTP {} {} {:?} [{}]",
            req.method(),
            req.uri(),
            &req.headers(),
            reqid,
        );

        self.set_headers_reqid(req.headers_mut(), &reqid);

        // handle errors
        let mut res = self
            .handle_request(&req)
            .await
            .unwrap_or_else(|err| self.handle_error(&req, &reqid, err));

        // set response headers
        self.set_headers_reqid(res.headers_mut(), &reqid);
        self.set_headers_cors(&req, &mut res);

        info!(
            "==> HTTP {} {} {} {:?} [{}]",
            res.status(),
            req.method(),
            req.uri(),
            &res.headers(),
            reqid,
        );

        Ok(res)
    }

    pub async fn handle_request(&self, req: &HttpRequest) -> S3Result {
        let (bucket, key) = self.parse_request(&req)?;

        self.authenticate(&req, &bucket, &key).await?;

        // call an op handler
        if req.method() == Method::OPTIONS {
            Ok(HttpResponse::new(Body::empty()))
        } else if bucket.is_empty() {
            self.handle_service_ops(&req).await
        } else if key.is_empty() {
            self.handle_bucket_ops(&req, &bucket).await
        } else {
            self.handle_object_ops(&req, &bucket, &key).await
        }
    }

    fn parse_request(&self, req: &HttpRequest) -> Result<(String, String), S3Error> {
        // parse path-style addressing for bucket names
        // TODO: add support for host-style addressing
        assert!(req.uri().path().starts_with("/"));
        let path_items: Vec<_> = req.uri().path()[1..].splitn(2, "/").collect();
        let (bucket, key) = match path_items.len() {
            0 => ("", ""),
            1 => (path_items[0], ""),
            2 => (path_items[0], path_items[1]),
            _ => return Err(S3Error::new(S3Errors::BadRequest)),
        };
        Ok((bucket.to_owned(), key.to_owned()))
    }

    async fn handle_service_ops(&self, req: &HttpRequest) -> S3Result {
        match *req.method() {
            Method::GET => store::list_buckets().await,
            _ => Err(S3Error::new(S3Errors::BadRequest)),
        }
    }

    async fn handle_bucket_ops(&self, req: &HttpRequest, bucket: &str) -> S3Result {
        match *req.method() {
            Method::GET => store::list_objects(bucket).await,
            Method::PUT => store::put_bucket(bucket).await,
            Method::HEAD => store::head_bucket(bucket).await,
            Method::DELETE => store::delete_bucket(bucket).await,
            _ => Err(S3Error::new(S3Errors::BadRequest)),
        }
    }

    async fn handle_object_ops(&self, req: &HttpRequest, bucket: &str, key: &str) -> S3Result {
        match *req.method() {
            Method::GET => store::get_object(bucket, key).await,
            Method::PUT => store::put_object(bucket, key).await,
            Method::HEAD => store::head_object(bucket, key).await,
            Method::DELETE => store::delete_object(bucket, key).await,
            _ => Err(S3Error::new(S3Errors::BadRequest)),
        }
    }

    fn set_headers_reqid(&self, h: &mut HeaderMap, reqid: &str) {
        let x_amz_request_id = header::HeaderName::from_static("x-amz-request-id");
        let x_amz_id_2 = header::HeaderName::from_static("x-amz-id-2");
        let reqid_val = header::HeaderValue::from_str(reqid).unwrap();
        h.insert(x_amz_request_id, reqid_val.clone());
        h.insert(x_amz_id_2, reqid_val.clone());
    }

    fn set_headers_cors(&self, _req: &HttpRequest, res: &mut HttpResponse) {
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

    pub fn handle_error(&self, req: &HttpRequest, reqid: &str, err: S3Error) -> HttpResponse {
        http_response()
            .status(err.status_code)
            .body(Body::from(format!(
                "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
                <Error>\n\
                    <Code>{}</Code>\n\
                    <Message>{}</Message>\n\
                    <Resource>{}</HostId>\n\
                    <RequestId>{}</RequestId>\n\
                </Error>\n",
                err.code,
                err.msg,
                req.uri(),
                reqid
            )))
            .unwrap()
    }

    pub async fn authenticate(
        &self,
        _req: &HttpRequest,
        _bucket: &str,
        _key: &str,
    ) -> Result<(), S3Error> {
        warn!("TODO authenticate() not yet implemented");
        Ok(())
    }
}
