use crate::{s3::api::S3Api, s3::ops::S3Ops, types::*};
use async_trait::async_trait;
use aws_sdk_s3::{error::*, input::*, output::*};

pub enum ServeError {
    InputError(InputError),
    S3Error(S3Error),
    OutputError(OutputError),
}

pub enum InputError {
    Unknown,
}
pub enum OutputError {
    Unknown,
}
impl From<InputError> for ServeError {
    fn from(e: InputError) -> Self {
        ServeError::InputError(e)
    }
}
impl From<S3Error> for ServeError {
    fn from(e: S3Error) -> Self {
        ServeError::S3Error(e)
    }
}
impl From<OutputError> for ServeError {
    fn from(e: OutputError) -> Self {
        ServeError::OutputError(e)
    }
}

#[async_trait]
pub trait ServeOperation {
    type Input;
    type Output;
    type Error;
    fn input(req: HttpRequest) -> Result<Self::Input, InputError>;
    async fn call(api: &dyn S3Api, input: Self::Input) -> Result<Self::Output, Self::Error>;
    fn output(output: Self::Output) -> Result<HttpResponse, OutputError>;
}

pub struct GetObjectOperation;
#[async_trait]
impl ServeOperation for GetObjectOperation {
    type Input = GetObjectInput;
    type Output = GetObjectOutput;
    type Error = GetObjectError;
    fn input(req: HttpRequest) -> Result<Self::Input, InputError> {
        // Input::from_http_request(req)
        Err(InputError::Unknown)
    }
    async fn call(api: &dyn S3Api, input: Self::Input) -> Result<Self::Output, Self::Error> {
        api.get_object(input).await
    }
    fn output(output: Self::Output) -> Result<HttpResponse, OutputError> {
        // output.to_http_response()
        Err(OutputError::Unknown)
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub async fn serve_http(api: &dyn S3Api, op: S3Ops, req: HttpRequest) -> Result<HttpResponse, ServeError> {
    match op {
        S3Ops::GetObject => {
            let input = GetObjectOperation::input(req)?;
            let output = GetObjectOperation::call(api, input).await.map_err(|err| err.meta().clone())?;
            let response = GetObjectOperation::output(output)?;
            Ok(response)
        },
        _ => Err(ServeError::InputError(InputError::Unknown)),
    }
}
