---
title: Architecture
---

# Architecture

In order to keep `s3d` as simple as possible, and yet use bleeding-edge technology and provide a fully capable service for edge computing stack, `s3d` builds on the shoulders of great open source projects.

The following sections describe those projects, components and concepts used in the making of `s3d`:

## Rust-lang

- The choice of the Rust language was a natural fit for edge systems,
  as it is a modern language with a focus on functionality, safety and performance.
- Building with the rust toolchain into a single, standalone, lightweight binary,
  makes it easy to set up and configure for linux and containers,
  in order to run alongside any application.
- Libraries from crates.io provide a great set of features for building daemons,
  such as the `tokio` library for async I/O, `hyper` for HTTP, etc.

## Smithy-rs

- [awslabs/smithy-rs](https://github.com/awslabs/smithy-rs) is the official AWS SDK for Rust.
- It aims for high API compatibility and provides the solid S3 protocol foundation.
- Using it to generate server and client S3 protocol code, and hook in the added functionality.

## Filters

> TODO: Work in progress

- A simple textual syntax is defined for fine grain object filters
  to include/exclude by bucket-name, key/prefix, tags, headers, meta-data.

## OPA Filters

> TODO: Work in progress

- [OPA (Open Policy Agent)](https://www.openpolicyagent.org/) provides tools
  for defining and evaluating policies.

## FUSE ("Filesystem in Userspace")

> TODO: Work in progress

- FUSE provides POSIX-like data access for applications that do not use the S3 API.
- The daemon binds a FUSE filesystem and creates a mount point that maps to the S3 API.
- FUSE is a good fit for immutable files, and reading small portions of large datasets.
- FUSE is a NOT a good fit for mutable files (overwrites/appends), or file locks (not supported).
- Using [fuser crate](https://crates.io/crates/fuser) for FUSE binding.

## Opentelemetry

> TODO: Work in progress

- Using [opentelemetry crate](https://crates.io/crates/opentelemetry) for monitoring and tracing.
