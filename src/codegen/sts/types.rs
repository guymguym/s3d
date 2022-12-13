#![allow(unused)]
#![allow(non_camel_case_types)]
use crate::common::OpService;
use aws_smithy_http_server::body::BoxBody;
use aws_smithy_http_server::operation::{
    Handler, IntoService, Operation, OperationService, OperationShape,
};
use aws_smithy_http_server::proto::rest_xml::RestXml;
use aws_smithy_http_server::response::IntoResponse;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;
use std::sync::Arc;

#[derive(Debug, Default, Clone)]
pub struct GetAccessKeyInfoRequest {
    pub access_key_id: Option<accessKeyIdType>,
}
impl GetAccessKeyInfoRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type invalidAuthorizationMessage = String;

pub type Issuer = String;

pub type clientTokenType = String;

#[derive(Debug, Default, Clone)]
pub struct AssumeRoleWithWebIdentityResponse {
    pub audience: Option<Audience>,
    pub provider: Option<Issuer>,
    pub assumed_role_user: Option<AssumedRoleUser>,
    pub packed_policy_size: Option<nonNegativeIntegerType>,
    pub credentials: Option<Credentials>,
    pub source_identity: Option<sourceIdentityType>,
    pub subject_from_web_identity_token: Option<webIdentitySubjectType>,
}
impl AssumeRoleWithWebIdentityResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct AssumeRoleWithSamlResponse {
    pub credentials: Option<Credentials>,
    pub subject_type: Option<SubjectType>,
    pub name_qualifier: Option<NameQualifier>,
    pub assumed_role_user: Option<AssumedRoleUser>,
    pub packed_policy_size: Option<nonNegativeIntegerType>,
    pub subject: Option<Subject>,
    pub source_identity: Option<sourceIdentityType>,
    pub issuer: Option<Issuer>,
    pub audience: Option<Audience>,
}
impl AssumeRoleWithSamlResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetAccessKeyInfo;
impl OperationShape for GetAccessKeyInfo {
    const NAME: &'static str = "GetAccessKeyInfo";
    type Input = GetAccessKeyInfoRequest;
    type Output = GetAccessKeyInfoResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct AssumeRoleWithSamlRequest {
    pub principal_arn: Option<arnType>,
    pub saml_assertion: Option<SamlAssertionType>,
    pub role_arn: Option<arnType>,
    pub duration_seconds: Option<roleDurationSecondsType>,
    pub policy_arns: Option<policyDescriptorListType>,
    pub policy: Option<sessionPolicyDocumentType>,
}
impl AssumeRoleWithSamlRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type assumedRoleIdType = String;

pub type tagKeyType = String;

pub type userNameType = String;

pub type decodedMessageType = String;

pub type nonNegativeIntegerType = i32;

pub type tokenType = String;

#[derive(Debug, Default, Clone)]
pub struct AssumeRoleRequest {
    pub duration_seconds: Option<roleDurationSecondsType>,
    pub serial_number: Option<serialNumberType>,
    pub tags: Option<tagListType>,
    pub external_id: Option<externalIdType>,
    pub token_code: Option<tokenCodeType>,
    pub source_identity: Option<sourceIdentityType>,
    pub transitive_tag_keys: Option<tagKeyListType>,
    pub policy_arns: Option<policyDescriptorListType>,
    pub policy: Option<sessionPolicyDocumentType>,
    pub role_arn: Option<arnType>,
    pub role_session_name: Option<roleSessionNameType>,
}
impl AssumeRoleRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct DecodeAuthorizationMessage;
impl OperationShape for DecodeAuthorizationMessage {
    const NAME: &'static str = "DecodeAuthorizationMessage";
    type Input = DecodeAuthorizationMessageRequest;
    type Output = DecodeAuthorizationMessageResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct ExpiredTokenException {
    pub message: Option<expiredIdentityTokenMessage>,
}
impl ExpiredTokenException {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type policyDescriptorListType = Vec<PolicyDescriptorType>;

#[derive(Debug, Default, Clone)]
pub struct IdpCommunicationErrorException {
    pub message: Option<idpCommunicationErrorMessage>,
}
impl IdpCommunicationErrorException {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type idpCommunicationErrorMessage = String;

pub type serialNumberType = String;

pub type dateType = String;

pub type packedPolicyTooLargeMessage = String;

#[derive(Debug, Default, Clone)]
pub struct AssumeRoleWithSaml;
impl OperationShape for AssumeRoleWithSaml {
    const NAME: &'static str = "AssumeRoleWithSaml";
    type Input = AssumeRoleWithSamlRequest;
    type Output = AssumeRoleWithSamlResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct RegionDisabledException {
    pub message: Option<regionDisabledMessage>,
}
impl RegionDisabledException {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetAccessKeyInfoResponse {
    pub account: Option<accountType>,
}
impl GetAccessKeyInfoResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type encodedMessageType = String;

#[derive(Debug, Default, Clone)]
pub struct AssumedRoleUser {
    pub arn: Option<arnType>,
    pub assumed_role_id: Option<assumedRoleIdType>,
}
impl AssumedRoleUser {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type durationSecondsType = i32;

#[derive(Debug, Default, Clone)]
pub struct GetFederationTokenRequest {
    pub policy_arns: Option<policyDescriptorListType>,
    pub name: Option<userNameType>,
    pub duration_seconds: Option<durationSecondsType>,
    pub policy: Option<sessionPolicyDocumentType>,
    pub tags: Option<tagListType>,
}
impl GetFederationTokenRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type roleDurationSecondsType = i32;

#[derive(Debug, Default, Clone)]
pub struct GetCallerIdentity;
impl OperationShape for GetCallerIdentity {
    const NAME: &'static str = "GetCallerIdentity";
    type Input = GetCallerIdentityRequest;
    type Output = GetCallerIdentityResponse;
    type Error = ();
}

pub type userIdType = String;

pub type malformedPolicyDocumentMessage = String;

pub type tokenCodeType = String;

pub type regionDisabledMessage = String;

pub type externalIdType = String;

#[derive(Debug, Default, Clone)]
pub struct AssumeRole;
impl OperationShape for AssumeRole {
    const NAME: &'static str = "AssumeRole";
    type Input = AssumeRoleRequest;
    type Output = AssumeRoleResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetCallerIdentityResponse {
    pub user_id: Option<userIdType>,
    pub account: Option<accountType>,
    pub arn: Option<arnType>,
}
impl GetCallerIdentityResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type NameQualifier = String;

pub type sourceIdentityType = String;

#[derive(Debug, Default, Clone)]
pub struct InvalidAuthorizationMessageException {
    pub message: Option<invalidAuthorizationMessage>,
}
impl InvalidAuthorizationMessageException {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type Subject = String;

#[derive(Debug, Default, Clone)]
pub struct Credentials {
    pub access_key_id: Option<accessKeyIdType>,
    pub session_token: Option<tokenType>,
    pub expiration: Option<dateType>,
    pub secret_access_key: Option<accessKeySecretType>,
}
impl Credentials {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type idpRejectedClaimMessage = String;

pub type Audience = String;

#[derive(Debug, Default, Clone)]
pub struct GetFederationTokenResponse {
    pub credentials: Option<Credentials>,
    pub federated_user: Option<FederatedUser>,
    pub packed_policy_size: Option<nonNegativeIntegerType>,
}
impl GetFederationTokenResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type webIdentitySubjectType = String;

#[derive(Debug, Default, Clone)]
pub struct GetCallerIdentityRequest {}
impl GetCallerIdentityRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct Tag {
    pub key: Option<tagKeyType>,
    pub value: Option<tagValueType>,
}
impl Tag {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct DecodeAuthorizationMessageRequest {
    pub encoded_message: Option<encodedMessageType>,
}
impl DecodeAuthorizationMessageRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetSessionToken;
impl OperationShape for GetSessionToken {
    const NAME: &'static str = "GetSessionToken";
    type Input = GetSessionTokenRequest;
    type Output = GetSessionTokenResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct AssumeRoleWithWebIdentityRequest {
    pub web_identity_token: Option<clientTokenType>,
    pub policy_arns: Option<policyDescriptorListType>,
    pub role_session_name: Option<roleSessionNameType>,
    pub policy: Option<sessionPolicyDocumentType>,
    pub duration_seconds: Option<roleDurationSecondsType>,
    pub provider_id: Option<urlType>,
    pub role_arn: Option<arnType>,
}
impl AssumeRoleWithWebIdentityRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type invalidIdentityTokenMessage = String;

pub type arnType = String;

pub type roleSessionNameType = String;

pub type tagKeyListType = Vec<tagKeyType>;

#[derive(Debug, Default, Clone)]
pub struct PackedPolicyTooLargeException {
    pub message: Option<packedPolicyTooLargeMessage>,
}
impl PackedPolicyTooLargeException {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type accountType = String;

pub type sessionPolicyDocumentType = String;

pub type SamlAssertionType = String;

#[derive(Debug, Default, Clone)]
pub struct IdpRejectedClaimException {
    pub message: Option<idpRejectedClaimMessage>,
}
impl IdpRejectedClaimException {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct PolicyDescriptorType {
    pub arn: Option<arnType>,
}
impl PolicyDescriptorType {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type SubjectType = String;

#[derive(Debug, Default, Clone)]
pub struct AssumeRoleResponse {
    pub assumed_role_user: Option<AssumedRoleUser>,
    pub source_identity: Option<sourceIdentityType>,
    pub packed_policy_size: Option<nonNegativeIntegerType>,
    pub credentials: Option<Credentials>,
}
impl AssumeRoleResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetSessionTokenRequest {
    pub token_code: Option<tokenCodeType>,
    pub serial_number: Option<serialNumberType>,
    pub duration_seconds: Option<durationSecondsType>,
}
impl GetSessionTokenRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct AwsSecurityTokenServiceV20110615 {
    pub get_access_key_info: Option<OpService<GetAccessKeyInfo>>,
    pub decode_authorization_message: Option<OpService<DecodeAuthorizationMessage>>,
    pub assume_role_with_saml: Option<OpService<AssumeRoleWithSaml>>,
    pub get_caller_identity: Option<OpService<GetCallerIdentity>>,
    pub get_session_token: Option<OpService<GetSessionToken>>,
    pub assume_role: Option<OpService<AssumeRole>>,
    pub get_federation_token: Option<OpService<GetFederationToken>>,
    pub assume_role_with_web_identity: Option<OpService<AssumeRoleWithWebIdentity>>,
}

#[derive(Debug, Default, Clone)]
pub struct GetSessionTokenResponse {
    pub credentials: Option<Credentials>,
}
impl GetSessionTokenResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct MalformedPolicyDocumentException {
    pub message: Option<malformedPolicyDocumentMessage>,
}
impl MalformedPolicyDocumentException {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type accessKeySecretType = String;

pub type federatedIdType = String;

pub type tagValueType = String;

pub type urlType = String;

pub type accessKeyIdType = String;

#[derive(Debug, Default, Clone)]
pub struct AssumeRoleWithWebIdentity;
impl OperationShape for AssumeRoleWithWebIdentity {
    const NAME: &'static str = "AssumeRoleWithWebIdentity";
    type Input = AssumeRoleWithWebIdentityRequest;
    type Output = AssumeRoleWithWebIdentityResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct FederatedUser {
    pub federated_user_id: Option<federatedIdType>,
    pub arn: Option<arnType>,
}
impl FederatedUser {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct InvalidIdentityTokenException {
    pub message: Option<invalidIdentityTokenMessage>,
}
impl InvalidIdentityTokenException {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct DecodeAuthorizationMessageResponse {
    pub decoded_message: Option<decodedMessageType>,
}
impl DecodeAuthorizationMessageResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetFederationToken;
impl OperationShape for GetFederationToken {
    const NAME: &'static str = "GetFederationToken";
    type Input = GetFederationTokenRequest;
    type Output = GetFederationTokenResponse;
    type Error = ();
}

pub type tagListType = Vec<Tag>;

pub type expiredIdentityTokenMessage = String;
