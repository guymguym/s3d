<div id="top"></div>
<div align="center" style="display: block; background-color: black; text-align: center">
  <a alt="s3d logo" href="https://s3d.rs" style="display: block; background-color: black; text-align: center">
    <img alt="s3d" src="s3d.png" width="200" />
  </a>
</div>
<br />
<div align="center">
  <a alt="crate" href="https://crates.io/crates/s3d">
    <img src="https://img.shields.io/crates/v/s3d.svg?logo=rust&color=success" />
  </a>
  <a alt="license" href="LICENSE">
    <img src="https://img.shields.io/badge/license-Apache--2.0-success.svg" />
  </a>
  <a alt="build" href="https://github.com/s3d-rs/s3d/actions">
    <img src="https://github.com/s3d-rs/s3d/workflows/build/badge.svg" />
  </a>
  <!--
  <a alt="releases" href="https://github.com/s3d-rs/s3d/releases/latest">
    <img src="https://img.shields.io/github/v/release/s3d-rs/s3d" />
  </a>
  <a alt="s3d at docs.rs" href="http://docs.rs/s3d">
    <img src="https://docs.rs/s3d/badge.svg" />
  </a>
  <img src="https://img.shields.io/badge/build-WIP-yellow.svg" />
  -->
</div>
<br />


# S3 Daemon

`s3d` is a daemon for efficient data access to remote S3 API storage.
The promise of `s3d` is simple - Applications that deploy remotely from their main S3 storage, 
  can run it as their S3 gateway to optimize data availability, performance, and costs.

The need for a solution like `s3d` emerges from Edge computing use cases,
where data is owned by a super-massive main S3 storage,
but the applications are deployed to a remote location:

> Edge computing is a distributed computing paradigm that brings computation and **data storage closer to the sources of data**.\
> This is expected to improve response times and save bandwidth.\
> (follow to [wikipedia - Edge computing](https://en.wikipedia.org/wiki/Edge_computing))

# Features

1. **S3-API** - generate protocol code with awslabs/smithy-rs 
  which is highly compatible and parses S3 requests and responses.
1. **UPLOAD-QUEUE** - writing new objects to local filesystem first,
  and pushing in the background to the main storage to tolerate connection issues.
1. **READ-CACHE** - store cached and prefetched objects in local filesystem
  to reduce egress costs and latency of reads from main storage.
1. **SELECTORS** - choose which objects to include/exclude for upload/cache/sync
  by bucket name, bucket tags, object keys (or prefixes), object tags, and object meta-data.
1. **SYNC-FOLDER** - continuous and bidirectional background sync of remote buckets
  with a local dir (aka "dropbox folder").
1. **FUSE-MOUNT** - filesystem in userspace lets the daemon provide a mount point
  and map FS requests to S3 API for applications that do not use S3.

# Architecture

`s3d` aims to integrate with open source tools and platforms and keep itself as simple as possible, 
while providing a fully capable service for the edge computing technology stack.

The following are the key components and concepts used in the making of `s3d`:

### Rust-lang
- The choice of the Rust language was a natural fit for edge systems, 
  as it is a modern language with a focus on functionality, safety and performance.
- Building with the rust toolchain into a single, standalone, lightweight binary,
  makes it easy to set up and configure for linux and containers,
  in order to run alongside any application.
- Libraries from crates.io provide a great set of features for building daemons,
  such as the `tokio` library for async I/O, `hyper` for HTTP, etc.

### S3 protocol by awslabs/smithy-rs
- [Smithy-rs](https://github.com/awslabs/smithy-rs) is the library that makes the official AWS SDK for Rust.
- It is therefore aiming for high API compatibility and provides a solid base S3 protocol foundation.
- Using it to generate client and server protocol code, and hook in the extended features.

### Data flows
- Using a local (filesystem) storage for implementing inline and background data flow features
  such as queueing uploads, caching downloads, and synching objects of the remote S3.
- `TODO:` Work in progress
- `TODO:` Sync-folder with change notifications

### Selectors
- Defining a simple selector syntax for fine grain control over which objects
  to include/exclude for each feature by bucket-name, key/prefix, tags, meta-data.
- The syntax is loosly based on [css selectors](https://www.w3schools.com/cssref/css_selectors.asp) 
  where css classes => s3 tags, and css attributes => s3 meta-data/headers.
- `TODO:` Work in progress

### Filesystem in Userspace (FUSE)
- This option provides access for applications that do not use the S3 API.
- The daemon binds a FUSE filesystem and provides a mount point that maps to the S3 API.
- FUSE is a good fit for immutable files, and reading small portions of large datasets.
- FUSE is a bad fit for mutable files (overwrites/appends), or file locks (not supported).
- Using [fuser crate](https://crates.io/crates/fuser) for FUSE binding.
- `TODO:` Work in progress
- `TODO:` Need details on the mapping of FUSE -> S3

### Monitoring
- Using [opentelemetry crate](https://crates.io/crates/opentelemetry) for monitoring and tracing.
- `TODO:` Work in progress


# How-To

## Install

Installing `s3d` requires the [rust toolchain](https://www.rust-lang.org/tools/install) 
which can be used to install the latest release from crates.io:
```shell
cargo install s3d
```

Run `s3d` in foreground:
```shell
s3d run
```

Use `s3d help` for a list of commands and options.

## Connect to main storage

`s3d` reads the standard S3 config and credential files and environment variables just like any other S3 SDK client in order to connect to its main storage.

In addition, to support S3 compatible endpoints, it reads the `S3_ENDPOINT` environment variable.

The credentials provided for `s3d` in the aws config files should be valid for the main storage, and the identity provided to `s3d` is the one it will use in all the requests to the main storage.

To check the remote S3 storage status, run:
```
s3d remote status
```

## Connect S3 Clients

Redirect S3 clients to the `s3d` endpoint at `localhost:33333`, for example with aws-cli:

```shell
export S3D_ENDPOINT='http://localhost:33333'
alias s3='aws --endpoint $S3D_ENDPOINT s3'
alias s3api='aws --endpoint $S3D_ENDPOINT s3api'

s3 ls s3://bucket/prefix/
s3 cp file s3://bucket/key
s3 cp s3://bucket/key file

s3api get-object-tagging --bucket bucket --key key
```

`s3d` itself can be used as a CLI to access the running daemon to make it easy for users to access it locally:

```shell
s3d get bucket/my-key > file
s3d put bucket/my-key < file
s3d ls [bucket/prefix]
```

## Local store

`s3d` uses the filesystem as a local storage, which is used for queueing, caching, and synching data from and to the remote storage.

The following environment variables can be used to configure the local store:
- `S3D_LOCAL_STORE` - path to the local store. Defaults to `.`.

## Upload-queue

When enabled, `s3d` first writes new objects to files in the local store, and will push them to the main storage in the background. This is to mitigate connection issues and improve performance.

The following environment variables can be used to configure the upload-queue:
- `S3D_UPLOAD_QUEUE` - true/false, default false.
- `S3D_UPLOAD_QUEUE_MAX_SIZE` - maximum size of the queue in bytes, default 1GB.
- `S3D_UPLOAD_QUEUE_MAX_FILES` - maximum number of files in the queue, default 100.
- `S3D_UPLOAD_QUEUE_MAX_AGE` - maximum age of uploads in the queue in seconds, default 3600.
- `S3D_UPLOAD_QUEUE_PUSH_SELECTOR` - selector syntax, default all.


When the limits are exceeded, new upload requests will not be able to add to the queue, instead it will be sent directly to the main storage, or wait for previous uploads to push.

See filters for fine grain control of which data to upload.

## Read-cache

When enabled, `s3d` will store objects in the local store on read, in order to reduce egress costs and latency on repeated reads from the main storage. 

The following environment variables can be used to configure the read-cache:
- `S3D_READ_CACHE` - true/false, default false.
- `S3D_READ_CACHE_MAX_SIZE` - maximum size of the cache in bytes, default 1GB.
- `S3D_READ_CACHE_MAX_FILES` - maximum number of files in the cache, default 100.
- `S3D_READ_CACHE_MAX_AGE` - maximum age of files in the cache in seconds, default 3600.

When the limits are exceeded, old items from the cache will be pruned.
See filters for fine grain control of which data to cache.

## Sync-folder

When enabled, `s3d` will perform a continuous bidirectional background sync of the remote buckets with a local dir (aka "dropbox folder").

The following environment variables can be used to configure the sync-folder:
- `S3D_SYNC_FOLDER` - true/false, default false.
- `S3D_SYNC_FOLDER_DIR` - directory to store the folder, default $HOME/.s3d/sync-folder.
- `S3D_SYNC_FOLDER_MAX_SIZE` - maximum size of the folder in bytes, default 1GB.
- `S3D_SYNC_FOLDER_MAX_FILES` - maximum number of files in the folder, default 100.
- `S3D_SYNC_FOLDER_MAX_AGE` - maximum age of (unsync-ed) files in the folder in seconds, default 3600.

When the limits are exceeded, sync will skip adding new data to the local folder.
See filters for fine grain control of which data to sync.

## Fuse-mount

When enabled, `s3d` will set up a FUSE mount point, which exposes the same buckets and objects through a POSIX-like file interface.

The following environment variables can be used to configure the fuse-mount:
- `S3D_FUSE_MOUNT` - true/false, default false.
- `S3D_FUSE_MOUNT_DIR` - directory to store the folder, default $HOME/.s3d/fuse-mount.

## Filters

By default, `s3d` will include all objects eligible for upload-queue, read-cache, and sync-folder. However for fine control over which objects to include, filters can be configured.

The following environment variables can be used to configure the filters:
- `S3D_FILTER` - true/false, default false.
- `S3D_FILTER_BUCKET_PREFIXES` - comma separated list of bucket-prefixes to include/exclude, default all. Use `!` in front of a bucket-prefix to exclude it.
- `S3D_FILTER_OBJECT_TAGS` - comma separated list of object-tags to include, default all. Use `!` in front of an object-tag to exclude it. Use `tag=value` or `tag=*`.

When the filters exclude a file:
- when the file is in upload-queue, it will remain in the queue and will not be uploaded.
- when the file is read, it will not be added to the cache.
- when the file is synced, it will not be added to the folder, and will not be synced to the main storage.

In order to set object tags, the S3 put-object-tagging API can be used, e.g:

```shell
export S3D_ENDPOINT='http://localhost:33333'
alias s3api='aws --endpoint $S3D_ENDPOINT s3api'

s3api put-object-tagging --bucket bucket --key key --tagging '{"TagSet":[{"Key":"s3d.filter","Value":"false"}]}'
```

Notice that put-object-tagging is overriding the entire tag set, so in order to add a tag to existing set, you will need to use get-object-tagging, append to the TagSet array and then put-object-tagging.

## Develop

Clone from repo (use a fork if you want to contribute back upstream):

```shell
git clone https://github.com/s3d-rs/s3d.git
cd s3d
```

Build and execute in one command:

```shell
cargo run -- <args>
```

Or in two commands:

```shell
cargo build
./target/debug/s3d <args>
```

Additional developer scripts are in `hack/` dir, e.g:

```shell
. hack/aliases.sh
```

## Deploy

See examples of using `s3d` in container images and kubernetes yamls in `deploy/` dir:

```shell
IMG="<username>/s3d"
docker build . -t $IMG
docker push $IMG
# update image in yaml ... TODO kustomize
kubectl create -f deploy/deployment.yaml
```


# Project

This project was initiated at [Red Hat emerging technologies](https://github.com/redhat-et).

## Status

This project is still  🛸🛸🛸  **Experimental**  🚀🚀🚀 , which means it's a great time to affect its direction, and we welcome contributions and open discussions

All internal and external interfaces are considered unstable and subject to change without notice.

## Links

- [Github](https://github.com/s3d-rs/s3d) - repository code, issues, PR’s, etc.
- [Discord](https://discord.gg/kPWHDuCdhh) - discussion channels.
- [License](https://github.com/s3d-rs/s3d/blob/main/LICENSE) - Apache-2.0.

## Roadmap

Directions for future development:

1. Multi-tenancy:
    1. IAM - Identity and Access Management (long-term credentials)
    1. STS - Secure Token Service (short-term credentials)
    1. IMDSv2 - Instance Meta-Data Service (integrated credential provider)


