// Copied from https://github.com/awslabs/aws-sdk-rust/blob/main/sdk/s3/src/middleware.rs
// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

//! Base Middleware Stack

pub use aws_smithy_client::retry::Config as RetryConfig;

use aws_endpoint::AwsEndpointStage;
use aws_http::auth::CredentialsStage;
use aws_http::recursion_detection::RecursionDetectionStage;
use aws_http::user_agent::UserAgentStage;
use aws_sig_auth::middleware::SigV4SigningStage;
use aws_sig_auth::signer::SigV4Signer;
use aws_smithy_http_tower::map_request::{AsyncMapRequestLayer, MapRequestLayer};
use std::fmt::Debug;
use tower::layer::util::{Identity, Stack};
use tower::ServiceBuilder;

type DefaultMiddlewareStack = Stack<
    MapRequestLayer<RecursionDetectionStage>,
    Stack<
        MapRequestLayer<SigV4SigningStage>,
        Stack<
            AsyncMapRequestLayer<CredentialsStage>,
            Stack<
                MapRequestLayer<UserAgentStage>,
                Stack<MapRequestLayer<AwsEndpointStage>, Identity>,
            >,
        >,
    >,
>;

/// AWS Middleware Stack
///
/// This implements the middleware stack for this service. It will:
/// 1. Load credentials asynchronously into the property bag
/// 2. Sign the request with SigV4
/// 3. Resolve an Endpoint for the request
/// 4. Add a user agent to the request
#[derive(Debug, Default, Clone)]
#[non_exhaustive]
pub struct DefaultMiddleware;

impl DefaultMiddleware {
    /// Create a new `DefaultMiddleware` stack
    ///
    /// Note: `DefaultMiddleware` holds no state.
    pub fn new() -> Self {
        DefaultMiddleware::default()
    }
}

// define the middleware stack in a non-generic location to reduce code bloat.
fn base() -> ServiceBuilder<DefaultMiddlewareStack> {
    let credential_provider = AsyncMapRequestLayer::for_mapper(CredentialsStage::new());
    let signer = MapRequestLayer::for_mapper(SigV4SigningStage::new(SigV4Signer::new()));
    let endpoint_resolver = MapRequestLayer::for_mapper(AwsEndpointStage);
    let user_agent = MapRequestLayer::for_mapper(UserAgentStage::new());
    let recursion_detection = MapRequestLayer::for_mapper(RecursionDetectionStage::new());
    // These layers can be considered as occurring in order, that is:
    // 1. Resolve an endpoint
    // 2. Add a user agent
    // 3. Acquire credentials
    // 4. Sign with credentials
    // (5. Dispatch over the wire)
    ServiceBuilder::new()
        .layer(endpoint_resolver)
        .layer(user_agent)
        .layer(credential_provider)
        .layer(signer)
        .layer(recursion_detection)
}

impl<S> tower::Layer<S> for DefaultMiddleware {
    type Service = <DefaultMiddlewareStack as tower::Layer<S>>::Service;

    fn layer(&self, inner: S) -> Self::Service {
        base().service(inner)
    }
}
