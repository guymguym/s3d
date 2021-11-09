use crate::{
    conf::Conf,
    err::*,
    ops::{self, S3BucketSubResource, S3ObjectSubResource, S3Request},
    parse,
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
    time::Duration,
};
use tokio::sync::OnceCell;
use url::Url;
use uuid::Uuid;

#[derive(Debug)]
pub struct Daemon {
    pub conf: Conf,
    pub s3c: aws_sdk_s3::Client,
}

// keep the server alive statically
// because we need it for the lifetime of the program
static DAEMON: OnceCell<Daemon> = OnceCell::const_new();

/// Starts http server with s3 service.
/// This should be called only once at the start of the program.
pub async fn run(conf: Conf) -> anyhow::Result<()> {
    let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let addr = SocketAddr::new(ip, conf.local.port);
    let mountpoint = conf.local.fuse_mount_point.to_owned();

    DAEMON.set(Daemon::new(conf).await).unwrap();

    fuser::mount2(
        DAEMON.get().unwrap(),
        mountpoint,
        &[fuser::MountOption::AutoUnmount],
    )?;

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
    pub async fn new(conf: Conf) -> Self {
        let s3_config = aws_config::from_env().load().await;
        let s3c = aws_sdk_s3::Client::new(&s3_config);
        Daemon { conf, s3c }
    }

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
        let host = parts.headers.get(header::HOST).unwrap().to_str().unwrap();
        let base_url = Url::parse(&format!("http://{}", host)).unwrap();
        let url = base_url.join(&parts.uri.to_string()).unwrap();
        let req = S3Request {
            url,
            body,
            method: parts.method,
            headers: parts.headers,
            // generate a unique id for each request
            reqid: Uuid::new_v4().to_string(),
            // initialize empty fields and init_request() will fill later
            params: HashMap::<String, String>::new(),
            bucket: "".to_owned(),
            key: "".to_owned(),
            bucket_subresource: S3BucketSubResource::None,
            object_subresource: S3ObjectSubResource::None,
        };
        info!("==> HTTP {} {} [{}]", req.method, req.url.path(), req.reqid);
        req
    }

    pub async fn handle_request(&self, req: &mut S3Request) -> S3Result {
        self.init_request(req).await?;
        self.authenticate(req).await?;
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

    pub async fn init_request(&self, req: &mut S3Request) -> Result<(), S3Error> {
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

    pub fn finish_response(&self, req: &S3Request, res: &mut HttpResponse) {
        self.set_headers_reqid(res.headers_mut(), &req.reqid);
        info!(
            "<== HTTP {} {} {} [{}]",
            res.status(),
            req.method,
            req.url.path(),
            req.reqid,
        );
    }

    pub async fn authenticate(&self, _req: &S3Request) -> Result<(), S3Error> {
        // TODO implement authenticate
        Ok(())
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
                err.code,
                err.msg,
                req.url.path(),
                req.reqid,
            )))
            .unwrap()
    }

    pub async fn handle_get(&self, req: &S3Request) -> S3Result {
        if req.bucket.is_empty() {
            return parse::list_buckets_output(
                ops::list_buckets(&self.s3c, parse::list_buckets_input(req)?).await?,
            );
        }
        if req.key.is_empty() {
            return match req.bucket_subresource {
                S3BucketSubResource::None => parse::list_objects_output(
                    ops::list_objects(&self.s3c, parse::list_objects_input(req)?).await?,
                ),
                _ => ops::get_bucket_subresource(req).await,
            };
        }
        match req.object_subresource {
            S3ObjectSubResource::None => parse::get_object_output(
                ops::get_object(&self.s3c, parse::get_object_input(req)?).await?,
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

    fn make_attr(&self, ino: u64, kind: fuser::FileType, size: u64) -> fuser::FileAttr {
        let now = std::time::SystemTime::now();
        let blksize: u32 = 512;
        fuser::FileAttr {
            ino,
            size,
            blocks: (size + (blksize as u64) - 1) / blksize as u64,
            atime: now,
            mtime: now,
            ctime: now,
            crtime: now,
            kind,
            perm: 0o644,
            nlink: 1,
            uid: 0,
            gid: 0,
            rdev: 0,
            blksize,
            flags: 0,
        }
    }
}

impl fuser::Filesystem for &Daemon {
    fn statfs(&mut self, _req: &fuser::Request<'_>, ino: u64, reply: fuser::ReplyStatfs) {
        info!("FUSE::statfs() ino={}", ino);
        reply.statfs(0, 0, 0, 0, 0, 512, 255, 0);
    }

    fn open(&mut self, _req: &fuser::Request<'_>, ino: u64, flags: i32, reply: fuser::ReplyOpen) {
        info!("FUSE::open() ino={} flags={}", ino, flags);
        reply.opened(ino, flags as u32);
    }

    fn release(
        &mut self,
        _req: &fuser::Request<'_>,
        ino: u64,
        fh: u64,
        flags: i32,
        lock_owner: Option<u64>,
        flush: bool,
        reply: fuser::ReplyEmpty,
    ) {
        info!(
            "FUSE::release() ino={} fh={} flags={} lock_owner={:?} flush={}",
            ino, fh, flags, lock_owner, flush
        );
        reply.ok();
    }

    fn opendir(
        &mut self,
        _req: &fuser::Request<'_>,
        ino: u64,
        flags: i32,
        reply: fuser::ReplyOpen,
    ) {
        info!("FUSE::opendir() ino={} flags={}", ino, flags);
        if ino < 1000 {
            reply.opened(ino, flags as u32);
        } else {
            reply.error(libc::ENOTDIR);
        }
    }

    fn releasedir(
        &mut self,
        _req: &fuser::Request<'_>,
        ino: u64,
        fh: u64,
        flags: i32,
        reply: fuser::ReplyEmpty,
    ) {
        info!("FUSE::releasedir() ino={} fh={} flags={}", ino, fh, flags);
        if ino < 1000 {
            reply.ok();
        } else {
            reply.error(libc::ENOTDIR);
        }
    }

    fn lookup(
        &mut self,
        _req: &fuser::Request<'_>,
        ino: u64,
        name: &std::ffi::OsStr,
        reply: fuser::ReplyEntry,
    ) {
        let name = name.to_str().unwrap();
        info!("FUSE::lookup() ino={} name={}", ino, name);
        if ino >= 1000 {
            reply.error(libc::ENOTDIR);
            return;
        }
        if !name.starts_with("file") {
            reply.error(libc::ENOENT);
            return;
        }
        let i = name[4..].parse::<u64>().unwrap();
        let kind = if i < 1000 {
            fuser::FileType::Directory
        } else {
            fuser::FileType::RegularFile
        };
        let attr = self.make_attr(i, kind, 10);
        let ttl = Duration::from_secs(60);
        reply.entry(&ttl, &attr, 0);
    }

    fn readdir(
        &mut self,
        _req: &fuser::Request<'_>,
        ino: u64,
        fh: u64,
        offset: i64,
        mut reply: fuser::ReplyDirectory,
    ) {
        info!("FUSE::readdir() ino={} fh={} offset={}", ino, fh, offset);
        if ino >= 1000 {
            reply.error(libc::ENOTDIR);
            return;
        }
        for i in 1000..1003 as u64 {
            if i >= offset as u64 {
                reply.add(
                    i,
                    i as i64,
                    fuser::FileType::RegularFile,
                    &format!("file{}", i),
                );
            }
        }
        reply.ok();
    }

    fn readdirplus(
        &mut self,
        _req: &fuser::Request<'_>,
        ino: u64,
        fh: u64,
        offset: i64,
        mut reply: fuser::ReplyDirectoryPlus,
    ) {
        info!(
            "FUSE::readdirplus() ino={} fh={} offset={}",
            ino, fh, offset
        );
        if ino >= 1000 {
            reply.error(libc::ENOTDIR);
            return;
        }
        let ttl = Duration::from_secs(60);
        for i in 1000..1003 as u64 {
            if i >= offset as u64 {
                let attr = self.make_attr(i as u64, fuser::FileType::RegularFile, 10);
                reply.add(i, i as i64, &format!("file{}", i), &ttl, &attr, 0);
            }
        }
        reply.ok();
    }

    fn getattr(&mut self, _req: &fuser::Request<'_>, ino: u64, reply: fuser::ReplyAttr) {
        info!("FUSE::getattr() ino={}", ino);
        let ttl = Duration::from_secs(60);
        if ino < 1000 {
            let attr = self.make_attr(ino, fuser::FileType::Directory, 0);
            reply.attr(&ttl, &attr);
        } else {
            let attr = self.make_attr(ino, fuser::FileType::RegularFile, 10);
            reply.attr(&ttl, &attr);
        }
    }

    fn read(
        &mut self,
        _req: &fuser::Request<'_>,
        ino: u64,
        fh: u64,
        offset: i64,
        size: u32,
        flags: i32,
        lock_owner: Option<u64>,
        reply: fuser::ReplyData,
    ) {
        info!(
            "FUSE::read() ino={} fh={} offset={} size={} flags={} lock_owner={:?}",
            ino, fh, offset, size, flags, lock_owner
        );
        if ino < 1000 {
            reply.error(libc::EISDIR);
        } else {
            reply.data("0123456789\n".to_string().as_bytes());
        }
    }
}
