use crate::{conf::Conf, gen::*, types::*};
use hyper::{
    header,
    server::conn::AddrStream,
    service::{make_service_fn, service_fn},
    Body,
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
        let s3_config = aws_config::from_env().load().await;
        let retry_config = s3_config.retry_config().unwrap();
        let s3c = aws_sdk_s3::Client::new(&s3_config);
        let smc = aws_hyper::Client::https().with_retry_config(retry_config.clone().into());
        Daemon {
            conf,
            s3_api: S3ApiToClient { s3c, smc },
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
        if req.method == hyper::Method::OPTIONS {
            return self.handle_options(req);
        }
        let mut res = crate::gen::handle_s3_request(req, &self.s3_api).await?;
        self.set_headers_ids(req, &mut res);
        Ok(res)
    }

    /// Authenticate and authorize the request.
    pub async fn check_auth(&self, req: &S3Request) -> S3ResultNull {
        // TODO implement authenticate
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
            .status(hyper::StatusCode::OK)
            .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
            .header(header::ACCESS_CONTROL_ALLOW_CREDENTIALS, "true")
            .header(header::ACCESS_CONTROL_ALLOW_METHODS, 
                    "GET,HEAD,PUT,POST,DELETE,OPTIONS,FETCH,PULL,PUSH,PRUNE,STATUS,DIFF")
            .header(header::ACCESS_CONTROL_ALLOW_HEADERS, 
                "Content-Type,Content-MD5,Authorization,X-Amz-User-Agent,X-Amz-Date,ETag,X-Amz-Content-Sha256")
            .header(header::ACCESS_CONTROL_EXPOSE_HEADERS, "ETag,X-Amz-Version-Id")
            .body(Body::empty())
            .unwrap()
        )
    }

    pub fn set_headers_ids(&self, req: &S3Request, res: &mut HttpResponse) {
        let x_amz_request_id = header::HeaderName::from_static("x-amz-request-id");
        let x_amz_id_2 = header::HeaderName::from_static("x-amz-id-2");
        let reqid_val = header::HeaderValue::from_str(&req.reqid).unwrap();
        let hostid_val = header::HeaderValue::from_str(&req.hostid).unwrap();
        let h = res.headers_mut();
        h.insert(x_amz_request_id, reqid_val.clone());
        h.insert(x_amz_id_2, hostid_val.clone());
    }

    pub fn handle_error(&self, req: &S3Request, err: S3Error) -> HttpResponse {
        let res = if let Ok(err) = err.clone().try_into() {
            crate::gen::output::s3_error_output(err)
        } else {
            error!("{:?}", err);
            crate::gen::output::s3_error_output(
                S3Error::builder()
                    .code("InternalError")
                    .message("Internal error")
                    .build(),
            )
        };
        let mut res = res.unwrap();
        // .unwrap_or(S3Error::Unhandled(err.into()))
        // if let Some(err) = err.downcast_ref::<S3Error>() {
        //     err.clone()
        // } else if let Some(err) = err.downcast_ref::<S3Error>() {
        //     S3Error::Unhandled(err.into)
        // } else {
        //     S3Error::Unhandled(err.into)
        //     responder()
        //         .status(hyper::StatusCode::from(err))
        //         .body(Body::from(err.message()))
        //         .unwrap()

        //     S3Error::Unhandled(
        //         S3Error::builder()
        //             .code("InternalError")
        //             .message("An internal error occurred")
        //             .request_id(self.reqid)
        //             .build()
        //             .into(),
        //     )
        // }
        // let mut res = if let Some(err) = err.downcast_ref::<aws_smithy_types::Error>() {
        //     responder()
        //         .status(hyper::StatusCode::from(err))
        //         .body(Body::from(err.message()))
        //         .unwrap()
        // } else {
        //     responder()
        //         .status(hyper::StatusCode::INTERNAL_SERVER_ERROR)
        //         .header(header::CONTENT_TYPE, "application/xml")
        //         .body(Body::from(format!(
        //             "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
        //         <Error>\
        //             <Code>{}</Code>\
        //             <Message>{}</Message>\
        //             <Resource>{}</HostId>\
        //             <RequestId>{}</RequestId>\
        //         </Error>",
        //             err.code,
        //             err.msg,
        //             req.url.path(),
        //             req.reqid,
        //         )))
        //         .unwrap()
        // };
        self.set_headers_ids(req, &mut res);
        res
    }

    // pub async fn handle_all(&self, req: &S3Request) -> S3Result {
    //     let mut res = match req.method.as_str() {
    //         "GET" => self.handle_get(req).await,
    //         "HEAD" => self.handle_head(req).await,
    //         "PUT" => self.handle_put(req).await,
    //         "POST" => self.handle_post(req).await,
    //         "DELETE" => self.handle_delete(req).await,
    //         "OPTIONS" => self.handle_options(req).await,
    //         "FETCH" => self.handle_fetch(req).await,
    //         "PULL" => self.handle_pull(req).await,
    //         "PUSH" => self.handle_push(req).await,
    //         "PRUNE" => self.handle_prune(req).await,
    //         "STATUS" => self.handle_status(req).await,
    //         "DIFF" => self.handle_diff(req).await,
    //         _ => Err(S3Error::builder()
    //             .code("MethodNotAllowed")
    //             .build()
    //             .into()),
    //     };
    // }
    // 
    // pub async fn handle_get(&self, req: &S3Request) -> S3Result {
    //     if req.bucket.is_empty() {
    //         return parse::list_buckets_output(
    //             ops::list_buckets(&self.s3c, &self.ac, parse::list_buckets_input(req).unwrap())
    //                 .await
    //                 .unwrap(),
    //         );
    //     }
    //     if req.key.is_empty() {
    //         return match req.bucket_subresource {
    //             S3BucketSubResource::None => parse::list_objects_output(
    //                 ops::list_objects(&self.s3c, &self.ac, parse::list_objects_input(req).unwrap())
    //                     .await
    //                     .unwrap(),
    //             ),
    //             _ => ops::get_bucket_subresource(req).await,
    //         };
    //     }
    //     match req.object_subresource {
    //         S3ObjectSubResource::None => parse::get_object_output(
    //             ops::get_object(&self.s3c, &self.ac, parse::get_object_input(req).unwrap())
    //                 .await
    //                 .unwrap(),
    //         ),
    //         _ => ops::get_object_subresource(req).await,
    //     }
    // }

    // pub async fn handle_head(&self, req: &S3Request) -> S3Result {
    //     if req.bucket.is_empty() {
    //         return Err(S3Error::builder().code("BadRequest").build());
    //     }
    //     if req.key.is_empty() {
    //         return ops::head_bucket(req).await;
    //     }
    //     ops::head_object(req).await
    // }

    // pub async fn handle_put(&self, req: &S3Request) -> S3Result {
    //     if req.bucket.is_empty() {
    //         return Err(S3Error::builder().code("BadRequest").build());
    //     }
    //     if req.key.is_empty() {
    //         return match req.bucket_subresource {
    //             S3BucketSubResource::None => ops::put_bucket(req).await,
    //             _ => ops::put_bucket_subresource(req).await,
    //         };
    //     }
    //     match req.object_subresource {
    //         S3ObjectSubResource::None => ops::put_object(req).await,
    //         _ => ops::put_object_subresource(req).await,
    //     }
    // }

    // pub async fn handle_delete(&self, req: &S3Request) -> S3Result {
    //     if req.bucket.is_empty() {
    //         return Err(S3Error::builder().code("BadRequest").build());
    //     }
    //     if req.key.is_empty() {
    //         return match req.bucket_subresource {
    //             S3BucketSubResource::None => ops::delete_bucket(req).await,
    //             _ => ops::delete_bucket_subresource(req).await,
    //         };
    //     }
    //     match req.object_subresource {
    //         S3ObjectSubResource::None => ops::delete_object(req).await,
    //         _ => ops::delete_object_subresource(req).await,
    //     }
    // }

    // pub async fn handle_post(&self, req: &S3Request) -> S3Result {
    //     if req.bucket.is_empty() {
    //         return Err(S3Error::builder().code("BadRequest").build());
    //     }
    //     if req.key.is_empty() {
    //         return match req.bucket_subresource {
    //             S3BucketSubResource::None => ops::post_object(req).await,
    //             _ => ops::post_bucket_subresource(req).await,
    //         };
    //     }
    //     match req.object_subresource {
    //         S3ObjectSubResource::Uploads => ops::create_multipart_upload(req).await,
    //         S3ObjectSubResource::UploadId => ops::complete_multipart_upload(req).await,
    //         _ => Err(S3Error::builder().code("BadRequest").build()),
    //     }
    // }
}

impl From<ServerError> for S3Error {
    fn from(err: ServerError) -> Self {
        match err {
            ServerError::InputError(err) => match err {
                InputError::BadRequest(err) => S3Error::builder()
                    .code("BadRequest")
                    .message(err.to_string())
                    .build(),
                InputError::NotImplemented(msg) => S3Error::builder()
                    .code("NotImplemented")
                    .message(msg)
                    .build(),
                InputError::Unhandled(err) => S3Error::builder()
                    .code("InternalError")
                    .message(err.to_string())
                    .build(),
            },
            ServerError::ApiError(err) => err.into(),
            ServerError::OutputError(err) => match err {
                OutputError::BadResponse(err) => S3Error::builder()
                    .code("InternalError")
                    .message(err.to_string())
                    .build(),
                OutputError::NotImplemented(msg) => S3Error::builder()
                    .code("NotImplemented")
                    .message(msg)
                    .build(),
                OutputError::Unhandled(err) => S3Error::builder()
                    .code("NotImplemented")
                    .message(err.to_string())
                    .build(),
            },
        }
    }
}
