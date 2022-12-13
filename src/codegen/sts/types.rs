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
pub struct GetAccessKeyInfoResponse {
    pub account: Option<accountType>,
}
impl GetAccessKeyInfoResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetFederationTokenRequest {
    pub name: Option<userNameType>,
    pub duration_seconds: Option<durationSecondsType>,
    pub policy_arns: Option<policyDescriptorListType>,
    pub tags: Option<tagListType>,
    pub policy: Option<sessionPolicyDocumentType>,
}
impl GetFederationTokenRequest {
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

#[derive(Debug, Default, Clone)]
pub struct InvalidAuthorizationMessageException {
    pub message: Option<invalidAuthorizationMessage>,
}
impl InvalidAuthorizationMessageException {
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

pub type Subject = String;

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
pub struct GetAccessKeyInfo;
impl OperationShape for GetAccessKeyInfo {
    const NAME: &'static str = "GetAccessKeyInfo";
    type Input = GetAccessKeyInfoRequest;
    type Output = GetAccessKeyInfoResponse;
    type Error = ();
}

pub type expiredIdentityTokenMessage = String;

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

pub type packedPolicyTooLargeMessage = String;

pub type accessKeySecretType = String;

#[derive(Debug, Default, Clone)]
pub struct GetSessionTokenRequest {
    pub duration_seconds: Option<durationSecondsType>,
    pub token_code: Option<tokenCodeType>,
    pub serial_number: Option<serialNumberType>,
}
impl GetSessionTokenRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct AssumeRoleWithSamlResponse {
    pub packed_policy_size: Option<nonNegativeIntegerType>,
    pub subject: Option<Subject>,
    pub assumed_role_user: Option<AssumedRoleUser>,
    pub audience: Option<Audience>,
    pub issuer: Option<Issuer>,
    pub name_qualifier: Option<NameQualifier>,
    pub source_identity: Option<sourceIdentityType>,
    pub subject_type: Option<SubjectType>,
    pub credentials: Option<Credentials>,
}
impl AssumeRoleWithSamlResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type roleSessionNameType = String;

pub type malformedPolicyDocumentMessage = String;

#[derive(Debug, Default, Clone)]
pub struct PolicyDescriptorType {
    pub arn: Option<arnType>,
}
impl PolicyDescriptorType {
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

pub type tagListType = Vec<Tag>;

pub type durationSecondsType = i32;

pub type accessKeyIdType = String;

#[derive(Debug, Default, Clone)]
pub struct AssumeRoleWithWebIdentityResponse {
    pub packed_policy_size: Option<nonNegativeIntegerType>,
    pub assumed_role_user: Option<AssumedRoleUser>,
    pub source_identity: Option<sourceIdentityType>,
    pub subject_from_web_identity_token: Option<webIdentitySubjectType>,
    pub provider: Option<Issuer>,
    pub audience: Option<Audience>,
    pub credentials: Option<Credentials>,
}
impl AssumeRoleWithWebIdentityResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct Credentials {
    pub access_key_id: Option<accessKeyIdType>,
    pub secret_access_key: Option<accessKeySecretType>,
    pub expiration: Option<dateType>,
    pub session_token: Option<tokenType>,
}
impl Credentials {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
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
pub struct PackedPolicyTooLargeException {
    pub message: Option<packedPolicyTooLargeMessage>,
}
impl PackedPolicyTooLargeException {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type accountType = String;

pub type assumedRoleIdType = String;

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
pub struct GetCallerIdentity;
impl OperationShape for GetCallerIdentity {
    const NAME: &'static str = "GetCallerIdentity";
    type Input = GetCallerIdentityRequest;
    type Output = GetCallerIdentityResponse;
    type Error = ();
}

pub type dateType = String;

pub type nonNegativeIntegerType = i32;

pub type policyDescriptorListType = Vec<PolicyDescriptorType>;

pub type tokenType = String;

pub type userIdType = String;

#[derive(Debug, Default, Clone)]
pub struct IdpCommunicationErrorException {
    pub message: Option<idpCommunicationErrorMessage>,
}
impl IdpCommunicationErrorException {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type roleDurationSecondsType = i32;

pub type sessionPolicyDocumentType = String;

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
pub struct AssumeRoleWithSaml;
impl OperationShape for AssumeRoleWithSaml {
    const NAME: &'static str = "AssumeRoleWithSaml";
    type Input = AssumeRoleWithSamlRequest;
    type Output = AssumeRoleWithSamlResponse;
    type Error = ();
}

pub type tagKeyType = String;

pub type tagValueType = String;

pub type federatedIdType = String;

#[derive(Debug, Default, Clone)]
pub struct AssumeRole;
impl OperationShape for AssumeRole {
    const NAME: &'static str = "AssumeRole";
    type Input = AssumeRoleRequest;
    type Output = AssumeRoleResponse;
    type Error = ();
}

pub type invalidIdentityTokenMessage = String;

pub type tokenCodeType = String;

pub type decodedMessageType = String;

#[derive(Debug, Default, Clone)]
pub struct AssumeRoleWithWebIdentity;
impl OperationShape for AssumeRoleWithWebIdentity {
    const NAME: &'static str = "AssumeRoleWithWebIdentity";
    type Input = AssumeRoleWithWebIdentityRequest;
    type Output = AssumeRoleWithWebIdentityResponse;
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

pub type clientTokenType = String;

pub type regionDisabledMessage = String;

pub type serialNumberType = String;

#[derive(Debug, Default, Clone)]
pub struct AssumeRoleRequest {
    pub role_session_name: Option<roleSessionNameType>,
    pub source_identity: Option<sourceIdentityType>,
    pub tags: Option<tagListType>,
    pub serial_number: Option<serialNumberType>,
    pub transitive_tag_keys: Option<tagKeyListType>,
    pub role_arn: Option<arnType>,
    pub policy_arns: Option<policyDescriptorListType>,
    pub external_id: Option<externalIdType>,
    pub token_code: Option<tokenCodeType>,
    pub duration_seconds: Option<roleDurationSecondsType>,
    pub policy: Option<sessionPolicyDocumentType>,
}
impl AssumeRoleRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type externalIdType = String;

pub type sourceIdentityType = String;

pub type NameQualifier = String;

pub type SamlAssertionType = String;

#[derive(Debug, Default, Clone)]
pub struct AwsSecurityTokenServiceV20110615 {
    pub assume_role: Option<OpService<AssumeRole>>,
    pub get_caller_identity: Option<OpService<GetCallerIdentity>>,
    pub get_session_token: Option<OpService<GetSessionToken>>,
    pub assume_role_with_web_identity: Option<OpService<AssumeRoleWithWebIdentity>>,
    pub assume_role_with_saml: Option<OpService<AssumeRoleWithSaml>>,
    pub decode_authorization_message: Option<OpService<DecodeAuthorizationMessage>>,
    pub get_federation_token: Option<OpService<GetFederationToken>>,
    pub get_access_key_info: Option<OpService<GetAccessKeyInfo>>,
}

pub type webIdentitySubjectType = String;

#[derive(Debug, Default, Clone)]
pub struct AssumeRoleWithWebIdentityRequest {
    pub role_arn: Option<arnType>,
    pub duration_seconds: Option<roleDurationSecondsType>,
    pub policy: Option<sessionPolicyDocumentType>,
    pub provider_id: Option<urlType>,
    pub web_identity_token: Option<clientTokenType>,
    pub role_session_name: Option<roleSessionNameType>,
    pub policy_arns: Option<policyDescriptorListType>,
}
impl AssumeRoleWithWebIdentityRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetAccessKeyInfoRequest {
    pub access_key_id: Option<accessKeyIdType>,
}
impl GetAccessKeyInfoRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type Audience = String;

pub type idpCommunicationErrorMessage = String;

pub type urlType = String;

pub type SubjectType = String;

pub type arnType = String;

pub type tagKeyListType = Vec<tagKeyType>;

#[derive(Debug, Default, Clone)]
pub struct GetCallerIdentityRequest {}
impl GetCallerIdentityRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type userNameType = String;

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
pub struct DecodeAuthorizationMessage;
impl OperationShape for DecodeAuthorizationMessage {
    const NAME: &'static str = "DecodeAuthorizationMessage";
    type Input = DecodeAuthorizationMessageRequest;
    type Output = DecodeAuthorizationMessageResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetCallerIdentityResponse {
    pub account: Option<accountType>,
    pub arn: Option<arnType>,
    pub user_id: Option<userIdType>,
}
impl GetCallerIdentityResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type invalidAuthorizationMessage = String;

#[derive(Debug, Default, Clone)]
pub struct GetFederationTokenResponse {
    pub federated_user: Option<FederatedUser>,
    pub packed_policy_size: Option<nonNegativeIntegerType>,
    pub credentials: Option<Credentials>,
}
impl GetFederationTokenResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type Issuer = String;

pub type encodedMessageType = String;

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
pub struct AssumeRoleWithSamlRequest {
    pub duration_seconds: Option<roleDurationSecondsType>,
    pub policy: Option<sessionPolicyDocumentType>,
    pub policy_arns: Option<policyDescriptorListType>,
    pub principal_arn: Option<arnType>,
    pub saml_assertion: Option<SamlAssertionType>,
    pub role_arn: Option<arnType>,
}
impl AssumeRoleWithSamlRequest {
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

pub type idpRejectedClaimMessage = String;
