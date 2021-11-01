use crate::{
    conf::Conf,
    err::*,
    store::S3Request,
    store::{self, S3BucketSubResource, S3ObjectSubResource},
    util::*,
};
use hyper::{
    header,
    service::{make_service_fn, service_fn},
    Body, HeaderMap, Server,
};
use std::{
    collections::HashMap,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};
use tokio::sync::OnceCell;
use url::Url;
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
    pub async fn serve_request(&self, http_req: HttpRequest) -> HttpResult {
        let mut req = self.new_request(http_req);
        let mut res = self
            .handle_request(&mut req)
            .await
            .unwrap_or_else(|err| self.handle_error(&req, err));
        self.finish_response(&req, &mut res);
        Ok(res)
    }

    fn new_request(&self, http_req: HttpRequest) -> S3Request {
        let (parts, body) = http_req.into_parts();
        let path = parts.uri.path();
        let query = parts.uri.query().unwrap_or("");
        let req = S3Request {
            method: parts.method,
            path: path.to_owned(),
            query: query.to_owned(),
            headers: parts.headers,
            body,
            reqid: Uuid::new_v4().to_string(),
            params: HashMap::<String, String>::new(),
            bucket: "".to_owned(),
            key: "".to_owned(),
            bucket_sub_resource: S3BucketSubResource::None,
            object_sub_resource: S3ObjectSubResource::None,
        };
        info!(
            "==> HTTP {} {} {:?} [{}]",
            req.method, req.path, &req.headers, req.reqid,
        );
        req
    }

    pub fn finish_response(&self, req: &S3Request, res: &mut HttpResponse) {
        self.set_headers_reqid(res.headers_mut(), &req.reqid);
        info!(
            "<== HTTP {} {} {} {:?} [{}]",
            res.status(),
            req.method,
            req.path,
            &req.headers,
            req.reqid,
        );
    }

    pub async fn handle_request(&self, req: &mut S3Request) -> S3Result {
        self.prepare_request(req).await?;
        self.authenticate(&req).await?;
        match req.method.as_str() {
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
        }
    }

    pub async fn prepare_request(&self, req: &mut S3Request) -> Result<(), S3Error> {
        // parse path-style addressing for bucket names
        // TODO: add support for host-style addressing
        assert!(req.path.starts_with("/"));
        let path_items: Vec<_> = req.path[1..].splitn(2, "/").collect();
        let (bucket, key) = match path_items.len() {
            0 => ("", ""),
            1 => (path_items[0], ""),
            2 => (path_items[0], path_items[1]),
            _ => panic!("unexpected path {:?}", req),
        };
        req.bucket = bucket.to_owned();
        req.key = key.to_owned();

        // parse query string
        for (key, val) in Url::parse(&req.query).unwrap().query_pairs() {
            if req.key.is_empty() {
                let sub = S3BucketSubResource::from(key.as_ref());
                if sub != S3BucketSubResource::None && sub != req.bucket_sub_resource {
                    return Err(S3Error::new(S3Errors::BadRequest));
                }
                req.bucket_sub_resource = sub;
            } else {
                let sub = S3ObjectSubResource::from(key.as_ref());
                if sub != S3ObjectSubResource::None && sub != req.object_sub_resource {
                    return Err(S3Error::new(S3Errors::BadRequest));
                }
                req.object_sub_resource = sub;
            }
            req.params.insert(String::from(key), String::from(val));
        }

        Ok(())
    }

    pub async fn authenticate(&self, _req: &S3Request) -> Result<(), S3Error> {
        warn!("TODO authenticate() not yet implemented");
        Ok(())
    }

    pub async fn handle_get(&self, req: &mut S3Request) -> S3Result {
        if req.bucket.is_empty() {
            return store::list_buckets(req).await;
        }
        if req.key.is_empty() {
            return match req.bucket_sub_resource {
                S3BucketSubResource::None => store::list_objects(req).await,
                _ => store::get_bucket_sub_resource(req).await,
            };
        }
        match req.object_sub_resource {
            S3ObjectSubResource::None => store::get_object(req).await,
            _ => store::get_object_sub_resource(req).await,
        }
    }

    pub async fn handle_head(&self, req: &mut S3Request) -> S3Result {
        if req.bucket.is_empty() {
            return Err(S3Error::new(S3Errors::BadRequest));
        }
        if req.key.is_empty() {
            return store::head_bucket(req).await;
        }
        store::head_object(req).await
    }

    pub async fn handle_put(&self, req: &mut S3Request) -> S3Result {
        if req.bucket.is_empty() {
            return Err(S3Error::new(S3Errors::BadRequest));
        }
        if req.key.is_empty() {
            return match req.bucket_sub_resource {
                S3BucketSubResource::None => store::put_bucket(req).await,
                _ => store::put_bucket_sub_resource(req).await,
            };
        }
        match req.object_sub_resource {
            S3ObjectSubResource::None => store::put_object(req).await,
            _ => store::put_object_sub_resource(req).await,
        }
    }

    pub async fn handle_delete(&self, req: &mut S3Request) -> S3Result {
        if req.bucket.is_empty() {
            return Err(S3Error::new(S3Errors::BadRequest));
        }
        if req.key.is_empty() {
            return match req.bucket_sub_resource {
                S3BucketSubResource::None => store::delete_bucket(req).await,
                _ => store::delete_bucket_sub_resource(req).await,
            };
        }
        match req.object_sub_resource {
            S3ObjectSubResource::None => store::delete_object(req).await,
            _ => store::delete_object_sub_resource(req).await,
        }
    }

    pub async fn handle_post(&self, req: &mut S3Request) -> S3Result {
        if req.bucket.is_empty() {
            return Err(S3Error::new(S3Errors::BadRequest));
        }
        if req.key.is_empty() {
            return match req.bucket_sub_resource {
                S3BucketSubResource::None => store::post_object(req).await,
                _ => store::post_bucket_sub_resource(req).await,
            };
        }
        match req.object_sub_resource {
            S3ObjectSubResource::Uploads => store::create_multipart_upload(req).await,
            S3ObjectSubResource::UploadId => store::complete_multipart_upload(req).await,
            _ => Err(S3Error::new(S3Errors::BadRequest)),
        }
    }

    pub async fn handle_options(&self, req: &mut S3Request) -> S3Result {
        let mut res = HttpResponse::new(Body::empty());
        self.set_headers_cors(&req, &mut res);
        Ok(res)
    }

    pub async fn handle_fetch(&self, req: &mut S3Request) -> S3Result {
        store::fetch_flow(req).await
    }

    pub async fn handle_pull(&self, req: &mut S3Request) -> S3Result {
        store::pull_flow(req).await
    }

    pub async fn handle_push(&self, req: &mut S3Request) -> S3Result {
        store::push_flow(req).await
    }

    pub async fn handle_prune(&self, req: &mut S3Request) -> S3Result {
        store::prune_flow(req).await
    }

    pub async fn handle_status(&self, req: &mut S3Request) -> S3Result {
        store::status_flow(req).await
    }

    pub async fn handle_diff(&self, req: &mut S3Request) -> S3Result {
        store::diff_flow(req).await
    }

    fn set_headers_reqid(&self, h: &mut HeaderMap, reqid: &str) {
        let x_amz_request_id = header::HeaderName::from_static("x-amz-request-id");
        let x_amz_id_2 = header::HeaderName::from_static("x-amz-id-2");
        let reqid_val = header::HeaderValue::from_str(reqid).unwrap();
        h.insert(x_amz_request_id, reqid_val.clone());
        h.insert(x_amz_id_2, reqid_val.clone());
    }

    fn set_headers_cors(&self, _req: &S3Request, res: &mut HttpResponse) {
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

    pub fn handle_error(&self, req: &S3Request, err: S3Error) -> HttpResponse {
        http_response()
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
                err.code, err.msg, req.path, req.reqid,
            )))
            .unwrap()
    }
}
