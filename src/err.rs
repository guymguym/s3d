use crate::http::*;
use aws_smithy_http::operation::BuildError;
use std::fmt;

/// InputError are errors that can occur when parsing the input from the HTTP request
#[derive(Debug)]
pub enum InputError {
    NotImplemented(&'static str),
    BadRequest(BuildError),
    Unhandled(anyhow::Error),
}
impl std::error::Error for InputError {}
impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InputError::NotImplemented(msg) => write!(f, "NotImplemented({})", msg),
            InputError::BadRequest(err) => write!(f, "BadRequest({})", err),
            InputError::Unhandled(err) => write!(f, "Unhandled({})", err),
        }
    }
}
impl From<BuildError> for InputError {
    fn from(err: BuildError) -> Self {
        InputError::BadRequest(err)
    }
}
impl From<anyhow::Error> for InputError {
    fn from(err: anyhow::Error) -> Self {
        InputError::Unhandled(err)
    }
}

/// OutputError are errors that can occur when parsing the input from the HTTP request
#[derive(Debug)]
pub enum OutputError {
    NotImplemented(&'static str),
    BadResponse(hyper::http::Error),
    Unhandled(anyhow::Error),
}
impl std::error::Error for OutputError {}
impl std::fmt::Display for OutputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputError::NotImplemented(msg) => write!(f, "NotImplemented({})", msg),
            OutputError::BadResponse(err) => write!(f, "BadResponse({})", err),
            OutputError::Unhandled(err) => write!(f, "Unhandled({})", err),
        }
    }
}
impl From<hyper::http::Error> for OutputError {
    fn from(err: hyper::http::Error) -> Self {
        OutputError::BadResponse(err)
    }
}
impl From<anyhow::Error> for OutputError {
    fn from(err: anyhow::Error) -> Self {
        OutputError::Unhandled(err)
    }
}

/// ServerError are errors that can occur when serving the HTTP request
#[derive(Debug)]
pub enum ServerError {
    InputError(InputError),
    ApiError(S3Error),
    OutputError(OutputError),
}
impl std::error::Error for ServerError {}
impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServerError::InputError(err) => write!(f, "InputError({})", err),
            ServerError::ApiError(err) => write!(f, "ApiError({})", err),
            ServerError::OutputError(err) => write!(f, "OutputError({})", err),
        }
    }
}
impl From<InputError> for ServerError {
    fn from(e: InputError) -> Self {
        ServerError::InputError(e)
    }
}
impl From<S3Error> for ServerError {
    fn from(e: S3Error) -> Self {
        ServerError::ApiError(e)
    }
}
impl From<OutputError> for ServerError {
    fn from(e: OutputError) -> Self {
        ServerError::OutputError(e)
    }
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
