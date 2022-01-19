---
title: Developer Guide
---

# Developer Guide

Clone from repo (use a fork if you want to contribute back upstream):

```bash
git clone --recurse-submodules https://github.com/s3d-rs/s3d.git
cd s3d
```

Build and execute in one command:

```bash
cargo run -- <args>
```

Or in two commands:

```bash
cargo build
./target/debug/s3d <args>
```

Additional developer scripts are in `hack/` dir, e.g:

```bash
. hack/aliases.sh
```

In order to run smithy-rs codegen, you need to have java and run:

```bash
hack/submodules.sh # updates the smithy-rs submodule HEAD
hack/codegen.sh # builds smithy-rs and runs the codegen for s3
```
