use crate::{conf::Conf, util::*};
use super::*;
use hyper::{
    header,
    service::{make_service_fn, service_fn},
    Body, HeaderMap, Method, Server, StatusCode,
};
use tokio::sync::OnceCell;
use uuid::Uuid;

pub type S3Result = HttpResultOrErr<S3Error>;

// keep the server alive statically
// because we need it for the lifetime of the program
static DAEMON: OnceCell<Daemon> = OnceCell::const_new();

#[derive(Debug)]
pub struct Daemon {
    // api: Box<dyn ApiLayer>,
}

impl Daemon {
    /// Starts http server with s3 service.
    /// This should be called only once at the start of the program.
    pub async fn run(_conf: Conf) -> ResultOrAnyErr<()> {
        let addr = ([127, 0, 0, 1], 3000).into();
        DAEMON.set(Self::new()).unwrap();
        let service = make_service_fn(|_| async {
            Ok::<_, AnyError>(service_fn(|req: HttpRequest| async {
                DAEMON.get().unwrap().handle_request(req).await
            }))
        });
        let server = Server::bind(&addr).serve(service);
        println!("Listening on http://{}", addr);
        server.await?;
        Ok(())
    }

    pub fn new() -> Self {
        // let api = Box::new(MockLayer::new());
        // Self { api }
        Self {}
    }

    pub async fn handle_request(&self, mut req: HttpRequest) -> HttpResult {
        // generate request uuid
        let reqid = Uuid::new_v4().to_string();

        println!(
            "<== HTTP {} {} {:?} [{}]",
            req.method(),
            req.uri(),
            &req.headers(),
            reqid,
        );

        self.set_headers_reqid(req.headers_mut(), &reqid);

        let (bucket, key) = self.parse_request(&req);

        let mut res = {
            self.authenticate(&req, &bucket, &key).await?;
            // ?????????????????????????????????????????????????????????????????????????????????
            // ??? does this '?' above return from the entire fn or just the current closure ???
            // ?????????????????????????????????????????????????????????????????????????????????

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
        // handle errors
        .unwrap_or_else(|err| self.handle_error(&req, err));

        // set response headers
        self.set_headers_reqid(res.headers_mut(), &reqid);
        self.set_headers_cors(&req, &mut res);

        println!(
            "==> HTTP {} {} {} {:?} [{}]",
            res.status(),
            req.method(),
            req.uri(),
            &res.headers(),
            reqid,
        );

        Ok(res)
    }

    fn parse_request(&self, req: &HttpRequest) -> (String, String) {
        // parse path style addressing for bucket names (TODO: host style addressing)
        assert!(req.uri().path().starts_with("/"));
        let path_items: Vec<_> = req.uri().path()[1..].splitn(2, "/").collect();
        let (bucket, key) = match path_items.len() {
            0 => ("", ""),
            1 => (path_items[0], ""),
            2 => (path_items[0], path_items[1]),
            _ => panic!("invalid path"),
        };
        (bucket.to_owned(), key.to_owned())
    }

    async fn handle_service_ops(&self, req: &HttpRequest) -> S3Result {
        match *req.method() {
            // Method::GET => self
            //     .api
            //     .list_buckets(ListBucketsParams::from_request(req, "", ""))
            //     .await
            //     .into(),
            // _ => Err(S3Error::from(S3Errors::BadRequest)),
            _ => (),
        }
        Ok(HttpResponse::new(Body::from("hello from s3d!")))
    }

    async fn handle_bucket_ops(&self, req: &HttpRequest, _bucket: &str) -> S3Result {
        match *req.method() {
            // Method::GET => self
            //     .api
            //     .list_objects(ListObjectsParams::parse(req, bucket, ""))
            //     .await
            //     .reply(),
            // _ => Err(S3Error::from(S3Errors::BadRequest)),
            _ => (),
        };
        Ok(HttpResponse::new(Body::from("hello from s3d!")))
    }

    async fn handle_object_ops(&self, req: &HttpRequest, _bucket: &str, _key: &str) -> S3Result {
        match *req.method() {
            // Method::GET => self
            //     .api
            //     .get_object(GetObjectParams::parse(req, bucket, key))
            //     .await
            //     .reply(),
            // _ => Err(S3Error::from(S3Errors::BadRequest)),
            _ => (),
        };
        Ok(HttpResponse::new(Body::from("hello from s3d!")))
    }

    fn set_headers_reqid(&self, h: &mut HeaderMap, reqid: &str) {
        let x_amz_request_id = header::HeaderName::from_static("x-amz-request-id");
        let x_amz_id_2 = header::HeaderName::from_static("x-amz-id-2");

        let reqid_val = header::HeaderValue::from_str(reqid).unwrap();
        h.insert(x_amz_request_id, reqid_val.clone());
        h.insert(x_amz_id_2, reqid_val.clone());
    }

    fn set_headers_cors(&self, _req: &HttpRequest, res: &mut HttpResponse) {
        // note that browsers will not allow origin=* with credentials
        // but anyway we allow it by the agent server.
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

    pub fn handle_error(&self, _req: &HttpRequest, _err: S3Error) -> HttpResponse {
        println!("TODO handle_error not yet implemented");
        http_response()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from("ERROR FROM S3D !!!"))
            .unwrap()
    }

    pub async fn authenticate(
        &self,
        _req: &HttpRequest,
        _bucket: &str,
        _key: &str,
    ) -> ResultOrAnyErr<()> {
        println!("TODO authenticate() not yet implemented");
        Ok(())
    }
}

//     let res: HttpResult = match op_match {
//         LIST_BUCKETS => self
//             .api
//             .list_buckets(ListBucketsParams::parse(req, bucket, key))
//             .await
//             .write(),
//         GET_BUCKET => self
//             .api
//             .get_bucket(get_bucket::Req::parse(req, bucket, key))
//             .await
//             .write(),
//         PUT_BUCKET => self
//             .api
//             .put_bucket(put_bucket::Req::parse(req, bucket, key))
//             .await
//             .write(),
//         DELETE_BUCKET => self
//             .api
//             .delete_bucket(delete_bucket::Req::parse(req, bucket, key))
//             .await
//             .write(),
//
//         LIST_OBJECTS => self
//             .api
//             .resolve_object_api(bucket)
//             .await
//             .list_objects(list_objects::Req::parse(req, bucket, key))
//             .await
//             .write(),
//         GET_OBJECT | HEAD_OBJECT => self
//             .api
//             .resolve_object_api(bucket)
//             .await
//             .get_object(get_object::Req::parse(req, bucket, key))
//             .await
//             .write(),
//         PUT_OBJECT => self
//             .api
//             .resolve_object_api(bucket)
//             .await
//             .put_object(put_object::Req::parse(req, bucket, key))
//             .await
//             .write(),
//         DELETE_OBJECT => self
//             .api
//             .resolve_object_api(bucket)
//             .await
//             .delete_object(delete_object::Req::parse(req, bucket, key))
//             .await
//             .write(),
//
//         _ => Ok(S3Error::BadRequest).write(),

// /// OpMatch is a tuple for choosing the requested op based on:
// /// 1. the http method
// /// 2. the existence of a bucket name in the host or path
// /// 3. the existence of a key in the path
// type OpMatch = (Method, bool, bool);
// const LIST_BUCKETS: OpMatch = (Method::GET, false, false);
// const LIST_OBJECTS: OpMatch = (Method::GET, true, false);
// const GET_BUCKET: OpMatch = (Method::HEAD, true, false);
// const GET_OBJECT: OpMatch = (Method::GET, true, true);
// const HEAD_OBJECT: OpMatch = (Method::HEAD, true, true);
// const PUT_BUCKET: OpMatch = (Method::PUT, true, false);
// const PUT_OBJECT: OpMatch = (Method::PUT, true, true);
// const DELETE_BUCKET: OpMatch = (Method::DELETE, true, false);
// const DELETE_OBJECT: OpMatch = (Method::DELETE, true, true);
