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
pub struct DeleteAccountAlias;
impl OperationShape for DeleteAccountAlias {
    const NAME: &'static str = "DeleteAccountAlias";
    type Input = DeleteAccountAliasRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GenerateServiceLastAccessedDetailsRequest {
    pub granularity: Option<AccessAdvisorUsageGranularityType>,
    pub arn: Option<arnType>,
}
impl GenerateServiceLastAccessedDetailsRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct Group {
    pub create_date: Option<dateType>,
    pub arn: Option<arnType>,
    pub group_name: Option<groupNameType>,
    pub path: Option<pathType>,
    pub group_id: Option<idType>,
}
impl Group {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListRolePoliciesResponse {
    pub marker: Option<responseMarkerType>,
    pub is_truncated: Option<booleanType>,
    pub policy_names: Option<policyNameListType>,
}
impl ListRolePoliciesResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct PermissionsBoundaryDecisionDetail {
    pub allowed_by_permissions_boundary: Option<booleanType>,
}
impl PermissionsBoundaryDecisionDetail {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type PolicyRoleListType = Vec<PolicyRole>;

#[derive(Debug, Default, Clone)]
pub struct AccessKey {
    pub secret_access_key: Option<accessKeySecretType>,
    pub create_date: Option<dateType>,
    pub status: Option<statusType>,
    pub user_name: Option<userNameType>,
    pub access_key_id: Option<accessKeyIdType>,
}
impl AccessKey {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct UpdateAssumeRolePolicy;
impl OperationShape for UpdateAssumeRolePolicy {
    const NAME: &'static str = "UpdateAssumeRolePolicy";
    type Input = UpdateAssumeRolePolicyRequest;
    type Output = ();
    type Error = ();
}

pub type listPolicyGrantingServiceAccessResponseListType =
    Vec<ListPoliciesGrantingServiceAccessEntry>;

#[derive(Debug, Default, Clone)]
pub struct DeleteGroup;
impl OperationShape for DeleteGroup {
    const NAME: &'static str = "DeleteGroup";
    type Input = DeleteGroupRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct CredentialReportNotReadyException {
    pub message: Option<credentialReportNotReadyExceptionMessage>,
}
impl CredentialReportNotReadyException {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type serviceNotSupportedMessage = String;

pub type dateType = String;

#[derive(Debug, Default, Clone)]
pub struct ServiceFailureException {
    pub message: Option<serviceFailureExceptionMessage>,
}
impl ServiceFailureException {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListPoliciesGrantingServiceAccessEntry {
    pub policies: Option<policyGrantingServiceAccessListType>,
    pub service_namespace: Option<serviceNamespaceType>,
}
impl ListPoliciesGrantingServiceAccessEntry {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct UnmodifiableEntityException {
    pub message: Option<unmodifiableEntityMessage>,
}
impl UnmodifiableEntityException {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListGroupPoliciesResponse {
    pub marker: Option<responseMarkerType>,
    pub policy_names: Option<policyNameListType>,
    pub is_truncated: Option<booleanType>,
}
impl ListGroupPoliciesResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct DeleteInstanceProfileRequest {
    pub instance_profile_name: Option<instanceProfileNameType>,
}
impl DeleteInstanceProfileRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetOrganizationsAccessReport;
impl OperationShape for GetOrganizationsAccessReport {
    const NAME: &'static str = "GetOrganizationsAccessReport";
    type Input = GetOrganizationsAccessReportRequest;
    type Output = GetOrganizationsAccessReportResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct TagRoleRequest {
    pub role_name: Option<roleNameType>,
    pub tags: Option<tagListType>,
}
impl TagRoleRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetSshPublicKeyResponse {
    pub ssh_public_key: Option<SshPublicKey>,
}
impl GetSshPublicKeyResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListInstanceProfileTagsRequest {
    pub max_items: Option<maxItemsType>,
    pub marker: Option<markerType>,
    pub instance_profile_name: Option<instanceProfileNameType>,
}
impl ListInstanceProfileTagsRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type credentialReportNotPresentExceptionMessage = String;

pub type serviceName = String;

#[derive(Debug, Default, Clone)]
pub struct MalformedPolicyDocumentException {
    pub message: Option<malformedPolicyDocumentMessage>,
}
impl MalformedPolicyDocumentException {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type serverCertificateMetadataListType = Vec<ServerCertificateMetadata>;

#[derive(Debug, Default, Clone)]
pub struct DeleteSamlProvider;
impl OperationShape for DeleteSamlProvider {
    const NAME: &'static str = "DeleteSamlProvider";
    type Input = DeleteSamlProviderRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct AttachGroupPolicyRequest {
    pub group_name: Option<groupNameType>,
    pub policy_arn: Option<arnType>,
}
impl AttachGroupPolicyRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct CreateInstanceProfileRequest {
    pub tags: Option<tagListType>,
    pub instance_profile_name: Option<instanceProfileNameType>,
    pub path: Option<pathType>,
}
impl CreateInstanceProfileRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct DeleteServiceLinkedRoleResponse {
    pub deletion_task_id: Option<DeletionTaskIdType>,
}
impl DeleteServiceLinkedRoleResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetAccountSummary;
impl OperationShape for GetAccountSummary {
    const NAME: &'static str = "GetAccountSummary";
    type Input = ();
    type Output = GetAccountSummaryResponse;
    type Error = ();
}

pub type responseMarkerType = String;

#[derive(Debug, Default, Clone)]
pub struct ListPolicyVersionsRequest {
    pub marker: Option<markerType>,
    pub max_items: Option<maxItemsType>,
    pub policy_arn: Option<arnType>,
}
impl ListPolicyVersionsRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct CreatePolicyRequest {
    pub policy_name: Option<policyNameType>,
    pub description: Option<policyDescriptionType>,
    pub policy_document: Option<policyDocumentType>,
    pub tags: Option<tagListType>,
    pub path: Option<policyPathType>,
}
impl CreatePolicyRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct Policy {
    pub is_attachable: Option<booleanType>,
    pub path: Option<policyPathType>,
    pub policy_id: Option<idType>,
    pub description: Option<policyDescriptionType>,
    pub create_date: Option<dateType>,
    pub default_version_id: Option<policyVersionIdType>,
    pub permissions_boundary_usage_count: Option<attachmentCountType>,
    pub tags: Option<tagListType>,
    pub arn: Option<arnType>,
    pub policy_name: Option<policyNameType>,
    pub attachment_count: Option<attachmentCountType>,
    pub update_date: Option<dateType>,
}
impl Policy {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type duplicateSshPublicKeyMessage = String;

#[derive(Debug, Default, Clone)]
pub struct GetAccountAuthorizationDetails;
impl OperationShape for GetAccountAuthorizationDetails {
    const NAME: &'static str = "GetAccountAuthorizationDetails";
    type Input = GetAccountAuthorizationDetailsRequest;
    type Output = GetAccountAuthorizationDetailsResponse;
    type Error = ();
}

pub type TrackedActionsLastAccessed = Vec<TrackedActionLastAccessed>;

pub type entityDetailsListType = Vec<EntityDetails>;

#[derive(Debug, Default, Clone)]
pub struct AttachedPermissionsBoundary {
    pub permissions_boundary_arn: Option<arnType>,
    pub permissions_boundary_type: Option<PermissionsBoundaryAttachmentType>,
}
impl AttachedPermissionsBoundary {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type BootstrapDatum = Vec<u8>;

#[derive(Debug, Default, Clone)]
pub struct TagPolicyRequest {
    pub policy_arn: Option<arnType>,
    pub tags: Option<tagListType>,
}
impl TagPolicyRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListRolesResponse {
    pub marker: Option<responseMarkerType>,
    pub roles: Option<roleListType>,
    pub is_truncated: Option<booleanType>,
}
impl ListRolesResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum PolicyUsageType {
    PermissionsBoundary,
    PermissionsPolicy,
}
impl AsRef<str> for PolicyUsageType {
    fn as_ref(&self) -> &str {
        match self {
            Self::PermissionsBoundary => "PermissionsBoundary",
            Self::PermissionsPolicy => "PermissionsPolicy",
        }
    }
}
impl TryFrom<&str> for PolicyUsageType {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "PermissionsBoundary" => Ok(Self::PermissionsBoundary),
            "PermissionsPolicy" => Ok(Self::PermissionsPolicy),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListPolicyTagsResponse {
    pub is_truncated: Option<booleanType>,
    pub marker: Option<responseMarkerType>,
    pub tags: Option<tagListType>,
}
impl ListPolicyTagsResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct RoleDetail {
    pub role_last_used: Option<RoleLastUsed>,
    pub arn: Option<arnType>,
    pub role_id: Option<idType>,
    pub role_policy_list: Option<policyDetailListType>,
    pub role_name: Option<roleNameType>,
    pub tags: Option<tagListType>,
    pub permissions_boundary: Option<AttachedPermissionsBoundary>,
    pub attached_managed_policies: Option<attachedPoliciesListType>,
    pub create_date: Option<dateType>,
    pub assume_role_policy_document: Option<policyDocumentType>,
    pub instance_profile_list: Option<instanceProfileListType>,
    pub path: Option<pathType>,
}
impl RoleDetail {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct TagPolicy;
impl OperationShape for TagPolicy {
    const NAME: &'static str = "TagPolicy";
    type Input = TagPolicyRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct UpdateRoleDescriptionResponse {
    pub role: Option<Role>,
}
impl UpdateRoleDescriptionResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct DeactivateMfaDeviceRequest {
    pub serial_number: Option<serialNumberType>,
    pub user_name: Option<existingUserNameType>,
}
impl DeactivateMfaDeviceRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type certificateChainType = String;

#[derive(Debug, Default, Clone)]
pub struct CreateAccountAliasRequest {
    pub account_alias: Option<accountAliasType>,
}
impl CreateAccountAliasRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GenerateOrganizationsAccessReport;
impl OperationShape for GenerateOrganizationsAccessReport {
    const NAME: &'static str = "GenerateOrganizationsAccessReport";
    type Input = GenerateOrganizationsAccessReportRequest;
    type Output = GenerateOrganizationsAccessReportResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetGroup;
impl OperationShape for GetGroup {
    const NAME: &'static str = "GetGroup";
    type Input = GetGroupRequest;
    type Output = GetGroupResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct ResourceSpecificResult {
    pub eval_resource_name: Option<ResourceNameType>,
    pub eval_resource_decision: Option<PolicyEvaluationDecisionType>,
    pub missing_context_values: Option<ContextKeyNamesResultListType>,
    pub permissions_boundary_decision_detail: Option<PermissionsBoundaryDecisionDetail>,
    pub eval_decision_details: Option<EvalDecisionDetailsType>,
    pub matched_statements: Option<StatementListType>,
}
impl ResourceSpecificResult {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type organizationsEntityPathType = String;

#[derive(Debug, Default, Clone)]
pub struct SshPublicKeyMetadata {
    pub upload_date: Option<dateType>,
    pub status: Option<statusType>,
    pub ssh_public_key_id: Option<publicKeyIdType>,
    pub user_name: Option<userNameType>,
}
impl SshPublicKeyMetadata {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListSamlProviderTagsRequest {
    pub marker: Option<markerType>,
    pub max_items: Option<maxItemsType>,
    pub saml_provider_arn: Option<arnType>,
}
impl ListSamlProviderTagsRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type summaryValueType = i32;

#[derive(Debug, Default, Clone)]
pub struct UntagMfaDeviceRequest {
    pub serial_number: Option<serialNumberType>,
    pub tag_keys: Option<tagKeyListType>,
}
impl UntagMfaDeviceRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type entityTemporarilyUnmodifiableMessage = String;

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum DeletionTaskStatusType {
    NOT_STARTED,
    FAILED,
    IN_PROGRESS,
    SUCCEEDED,
}
impl AsRef<str> for DeletionTaskStatusType {
    fn as_ref(&self) -> &str {
        match self {
            Self::NOT_STARTED => "NOT_STARTED",
            Self::FAILED => "FAILED",
            Self::IN_PROGRESS => "IN_PROGRESS",
            Self::SUCCEEDED => "SUCCEEDED",
        }
    }
}
impl TryFrom<&str> for DeletionTaskStatusType {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "NOT_STARTED" => Ok(Self::NOT_STARTED),
            "FAILED" => Ok(Self::FAILED),
            "IN_PROGRESS" => Ok(Self::IN_PROGRESS),
            "SUCCEEDED" => Ok(Self::SUCCEEDED),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

pub type roleDetailListType = Vec<RoleDetail>;

#[derive(Debug, Default, Clone)]
pub struct UpdateServerCertificate;
impl OperationShape for UpdateServerCertificate {
    const NAME: &'static str = "UpdateServerCertificate";
    type Input = UpdateServerCertificateRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetUserResponse {
    pub user: Option<User>,
}
impl GetUserResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListAttachedUserPoliciesResponse {
    pub marker: Option<responseMarkerType>,
    pub attached_policies: Option<attachedPoliciesListType>,
    pub is_truncated: Option<booleanType>,
}
impl ListAttachedUserPoliciesResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListOpenIdConnectProviderTags;
impl OperationShape for ListOpenIdConnectProviderTags {
    const NAME: &'static str = "ListOpenIdConnectProviderTags";
    type Input = ListOpenIdConnectProviderTagsRequest;
    type Output = ListOpenIdConnectProviderTagsResponse;
    type Error = ();
}

pub type serverCertificateNameType = String;

#[derive(Debug, Default, Clone)]
pub struct ListEntitiesForPolicyResponse {
    pub marker: Option<responseMarkerType>,
    pub policy_users: Option<PolicyUserListType>,
    pub is_truncated: Option<booleanType>,
    pub policy_groups: Option<PolicyGroupListType>,
    pub policy_roles: Option<PolicyRoleListType>,
}
impl ListEntitiesForPolicyResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListInstanceProfiles;
impl OperationShape for ListInstanceProfiles {
    const NAME: &'static str = "ListInstanceProfiles";
    type Input = ListInstanceProfilesRequest;
    type Output = ListInstanceProfilesResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct CreatePolicyVersionResponse {
    pub policy_version: Option<PolicyVersion>,
}
impl CreatePolicyVersionResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct DeleteRolePermissionsBoundaryRequest {
    pub role_name: Option<roleNameType>,
}
impl DeleteRolePermissionsBoundaryRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct CreateLoginProfile;
impl OperationShape for CreateLoginProfile {
    const NAME: &'static str = "CreateLoginProfile";
    type Input = CreateLoginProfileRequest;
    type Output = CreateLoginProfileResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct ChangePasswordRequest {
    pub old_password: Option<passwordType>,
    pub new_password: Option<passwordType>,
}
impl ChangePasswordRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct EntityDetails {
    pub entity_info: Option<EntityInfo>,
    pub last_authenticated: Option<dateType>,
}
impl EntityDetails {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct PolicyRole {
    pub role_id: Option<idType>,
    pub role_name: Option<roleNameType>,
}
impl PolicyRole {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetServerCertificateRequest {
    pub server_certificate_name: Option<serverCertificateNameType>,
}
impl GetServerCertificateRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetContextKeysForPrincipalPolicyRequest {
    pub policy_source_arn: Option<arnType>,
    pub policy_input_list: Option<SimulationPolicyListType>,
}
impl GetContextKeysForPrincipalPolicyRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ServiceSpecificCredentialMetadata {
    pub create_date: Option<dateType>,
    pub service_user_name: Option<serviceUserName>,
    pub service_name: Option<serviceName>,
    pub service_specific_credential_id: Option<serviceSpecificCredentialId>,
    pub user_name: Option<userNameType>,
    pub status: Option<statusType>,
}
impl ServiceSpecificCredentialMetadata {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct SimulateCustomPolicyRequest {
    pub resource_handling_option: Option<ResourceHandlingOptionType>,
    pub context_entries: Option<ContextEntryListType>,
    pub resource_policy: Option<policyDocumentType>,
    pub resource_arns: Option<ResourceNameListType>,
    pub resource_owner: Option<ResourceNameType>,
    pub permissions_boundary_policy_input_list: Option<SimulationPolicyListType>,
    pub caller_arn: Option<ResourceNameType>,
    pub action_names: Option<ActionNameListType>,
    pub max_items: Option<maxItemsType>,
    pub marker: Option<markerType>,
    pub policy_input_list: Option<SimulationPolicyListType>,
}
impl SimulateCustomPolicyRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct SimulatePrincipalPolicyRequest {
    pub max_items: Option<maxItemsType>,
    pub policy_input_list: Option<SimulationPolicyListType>,
    pub marker: Option<markerType>,
    pub caller_arn: Option<ResourceNameType>,
    pub permissions_boundary_policy_input_list: Option<SimulationPolicyListType>,
    pub resource_arns: Option<ResourceNameListType>,
    pub policy_source_arn: Option<arnType>,
    pub context_entries: Option<ContextEntryListType>,
    pub resource_handling_option: Option<ResourceHandlingOptionType>,
    pub resource_policy: Option<policyDocumentType>,
    pub action_names: Option<ActionNameListType>,
    pub resource_owner: Option<ResourceNameType>,
}
impl SimulatePrincipalPolicyRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct UntagRoleRequest {
    pub role_name: Option<roleNameType>,
    pub tag_keys: Option<tagKeyListType>,
}
impl UntagRoleRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type passwordReusePreventionType = i32;

pub type summaryMapType = HashMap<summaryKeyType, summaryValueType>;

pub type userListType = Vec<User>;

#[derive(Debug, Default, Clone)]
pub struct CreatePolicyResponse {
    pub policy: Option<Policy>,
}
impl CreatePolicyResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListAttachedGroupPoliciesResponse {
    pub is_truncated: Option<booleanType>,
    pub attached_policies: Option<attachedPoliciesListType>,
    pub marker: Option<responseMarkerType>,
}
impl ListAttachedGroupPoliciesResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type malformedCertificateMessage = String;

#[derive(Debug, Default, Clone)]
pub struct DeleteLoginProfileRequest {
    pub user_name: Option<userNameType>,
}
impl DeleteLoginProfileRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListPoliciesGrantingServiceAccess;
impl OperationShape for ListPoliciesGrantingServiceAccess {
    const NAME: &'static str = "ListPoliciesGrantingServiceAccess";
    type Input = ListPoliciesGrantingServiceAccessRequest;
    type Output = ListPoliciesGrantingServiceAccessResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct SigningCertificate {
    pub certificate_body: Option<certificateBodyType>,
    pub certificate_id: Option<certificateIdType>,
    pub user_name: Option<userNameType>,
    pub upload_date: Option<dateType>,
    pub status: Option<statusType>,
}
impl SigningCertificate {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ResyncMfaDevice;
impl OperationShape for ResyncMfaDevice {
    const NAME: &'static str = "ResyncMfaDevice";
    type Input = ResyncMfaDeviceRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct CreateLoginProfileResponse {
    pub login_profile: Option<LoginProfile>,
}
impl CreateLoginProfileResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetGroupPolicyResponse {
    pub group_name: Option<groupNameType>,
    pub policy_document: Option<policyDocumentType>,
    pub policy_name: Option<policyNameType>,
}
impl GetGroupPolicyResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetRolePolicyResponse {
    pub policy_name: Option<policyNameType>,
    pub policy_document: Option<policyDocumentType>,
    pub role_name: Option<roleNameType>,
}
impl GetRolePolicyResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct RemoveUserFromGroupRequest {
    pub group_name: Option<groupNameType>,
    pub user_name: Option<existingUserNameType>,
}
impl RemoveUserFromGroupRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type userNameType = String;

#[derive(Debug, Default, Clone)]
pub struct DeleteRoleRequest {
    pub role_name: Option<roleNameType>,
}
impl DeleteRoleRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct AttachRolePolicyRequest {
    pub policy_arn: Option<arnType>,
    pub role_name: Option<roleNameType>,
}
impl AttachRolePolicyRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct CreateLoginProfileRequest {
    pub password: Option<passwordType>,
    pub user_name: Option<userNameType>,
    pub password_reset_required: Option<booleanType>,
}
impl CreateLoginProfileRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct CreateServiceLinkedRoleResponse {
    pub role: Option<Role>,
}
impl CreateServiceLinkedRoleResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct UploadSigningCertificateResponse {
    pub certificate: Option<SigningCertificate>,
}
impl UploadSigningCertificateResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct PutRolePermissionsBoundary;
impl OperationShape for PutRolePermissionsBoundary {
    const NAME: &'static str = "PutRolePermissionsBoundary";
    type Input = PutRolePermissionsBoundaryRequest;
    type Output = ();
    type Error = ();
}

pub type invalidAuthenticationCodeMessage = String;

#[derive(Debug, Default, Clone)]
pub struct DeleteConflictException {
    pub message: Option<deleteConflictMessage>,
}
impl DeleteConflictException {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct UpdateSigningCertificate;
impl OperationShape for UpdateSigningCertificate {
    const NAME: &'static str = "UpdateSigningCertificate";
    type Input = UpdateSigningCertificateRequest;
    type Output = ();
    type Error = ();
}

pub type policyGrantingServiceAccessListType = Vec<PolicyGrantingServiceAccess>;

#[derive(Debug, Default, Clone)]
pub struct ManagedPolicyDetail {
    pub is_attachable: Option<booleanType>,
    pub permissions_boundary_usage_count: Option<attachmentCountType>,
    pub policy_id: Option<idType>,
    pub policy_name: Option<policyNameType>,
    pub policy_version_list: Option<policyDocumentVersionListType>,
    pub path: Option<policyPathType>,
    pub arn: Option<arnType>,
    pub description: Option<policyDescriptionType>,
    pub update_date: Option<dateType>,
    pub attachment_count: Option<attachmentCountType>,
    pub create_date: Option<dateType>,
    pub default_version_id: Option<policyVersionIdType>,
}
impl ManagedPolicyDetail {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct DetachGroupPolicyRequest {
    pub group_name: Option<groupNameType>,
    pub policy_arn: Option<arnType>,
}
impl DetachGroupPolicyRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetAccessKeyLastUsedRequest {
    pub access_key_id: Option<accessKeyIdType>,
}
impl GetAccessKeyLastUsedRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type thumbprintType = String;

#[derive(Debug, Default, Clone)]
pub struct GetContextKeysForCustomPolicy;
impl OperationShape for GetContextKeysForCustomPolicy {
    const NAME: &'static str = "GetContextKeysForCustomPolicy";
    type Input = GetContextKeysForCustomPolicyRequest;
    type Output = GetContextKeysForPolicyResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct ListServerCertificateTags;
impl OperationShape for ListServerCertificateTags {
    const NAME: &'static str = "ListServerCertificateTags";
    type Input = ListServerCertificateTagsRequest;
    type Output = ListServerCertificateTagsResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct RemoveRoleFromInstanceProfile;
impl OperationShape for RemoveRoleFromInstanceProfile {
    const NAME: &'static str = "RemoveRoleFromInstanceProfile";
    type Input = RemoveRoleFromInstanceProfileRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct ResetServiceSpecificCredential;
impl OperationShape for ResetServiceSpecificCredential {
    const NAME: &'static str = "ResetServiceSpecificCredential";
    type Input = ResetServiceSpecificCredentialRequest;
    type Output = ResetServiceSpecificCredentialResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct CreateAccessKeyResponse {
    pub access_key: Option<AccessKey>,
}
impl CreateAccessKeyResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetRoleRequest {
    pub role_name: Option<roleNameType>,
}
impl GetRoleRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type PolicyUserListType = Vec<PolicyUser>;

pub type ManagedPolicyDetailListType = Vec<ManagedPolicyDetail>;

#[derive(Debug, Default, Clone)]
pub struct User {
    pub path: Option<pathType>,
    pub permissions_boundary: Option<AttachedPermissionsBoundary>,
    pub user_name: Option<userNameType>,
    pub arn: Option<arnType>,
    pub password_last_used: Option<dateType>,
    pub create_date: Option<dateType>,
    pub tags: Option<tagListType>,
    pub user_id: Option<idType>,
}
impl User {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type malformedPolicyDocumentMessage = String;

#[derive(Debug, Default, Clone)]
pub struct GetLoginProfile;
impl OperationShape for GetLoginProfile {
    const NAME: &'static str = "GetLoginProfile";
    type Input = GetLoginProfileRequest;
    type Output = GetLoginProfileResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct MfaDevice {
    pub user_name: Option<userNameType>,
    pub enable_date: Option<dateType>,
    pub serial_number: Option<serialNumberType>,
}
impl MfaDevice {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type ActionNameType = String;

#[derive(Debug, Default, Clone)]
pub struct ListPolicyTagsRequest {
    pub marker: Option<markerType>,
    pub max_items: Option<maxItemsType>,
    pub policy_arn: Option<arnType>,
}
impl ListPolicyTagsRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type ReportStateDescriptionType = String;

pub type ResourceNameType = String;

#[derive(Debug, Default, Clone)]
pub struct UntagUserRequest {
    pub tag_keys: Option<tagKeyListType>,
    pub user_name: Option<existingUserNameType>,
}
impl UntagUserRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct UpdateRoleRequest {
    pub role_name: Option<roleNameType>,
    pub description: Option<roleDescriptionType>,
    pub max_session_duration: Option<roleMaxSessionDurationType>,
}
impl UpdateRoleRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct UploadSshPublicKeyRequest {
    pub ssh_public_key_body: Option<publicKeyMaterialType>,
    pub user_name: Option<userNameType>,
}
impl UploadSshPublicKeyRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct DeleteSamlProviderRequest {
    pub saml_provider_arn: Option<arnType>,
}
impl DeleteSamlProviderRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type entityAlreadyExistsMessage = String;

#[derive(Debug, Default, Clone)]
pub struct ListPolicyVersions;
impl OperationShape for ListPolicyVersions {
    const NAME: &'static str = "ListPolicyVersions";
    type Input = ListPolicyVersionsRequest;
    type Output = ListPolicyVersionsResponse;
    type Error = ();
}

pub type SshPublicKeyListType = Vec<SshPublicKeyMetadata>;

#[derive(Debug, Default, Clone)]
pub struct ListMfaDeviceTagsResponse {
    pub tags: Option<tagListType>,
    pub marker: Option<responseMarkerType>,
    pub is_truncated: Option<booleanType>,
}
impl ListMfaDeviceTagsResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct AddRoleToInstanceProfile;
impl OperationShape for AddRoleToInstanceProfile {
    const NAME: &'static str = "AddRoleToInstanceProfile";
    type Input = AddRoleToInstanceProfileRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct TrackedActionLastAccessed {
    pub last_accessed_time: Option<dateType>,
    pub action_name: Option<stringType>,
    pub last_accessed_entity: Option<arnType>,
    pub last_accessed_region: Option<stringType>,
}
impl TrackedActionLastAccessed {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type policyNameType = String;

#[derive(Debug, Default, Clone)]
pub struct ListSamlProvidersResponse {
    pub saml_provider_list: Option<SamlProviderListType>,
}
impl ListSamlProvidersResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct CreateAccountAlias;
impl OperationShape for CreateAccountAlias {
    const NAME: &'static str = "CreateAccountAlias";
    type Input = CreateAccountAliasRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct CreateServiceSpecificCredential;
impl OperationShape for CreateServiceSpecificCredential {
    const NAME: &'static str = "CreateServiceSpecificCredential";
    type Input = CreateServiceSpecificCredentialRequest;
    type Output = CreateServiceSpecificCredentialResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct UpdateAssumeRolePolicyRequest {
    pub policy_document: Option<policyDocumentType>,
    pub role_name: Option<roleNameType>,
}
impl UpdateAssumeRolePolicyRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListInstanceProfilesRequest {
    pub path_prefix: Option<pathPrefixType>,
    pub marker: Option<markerType>,
    pub max_items: Option<maxItemsType>,
}
impl ListInstanceProfilesRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type unrecognizedPublicKeyEncodingMessage = String;

#[derive(Debug, Default, Clone)]
pub struct GetPolicyRequest {
    pub policy_arn: Option<arnType>,
}
impl GetPolicyRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct DeleteUserPolicy;
impl OperationShape for DeleteUserPolicy {
    const NAME: &'static str = "DeleteUserPolicy";
    type Input = DeleteUserPolicyRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetRolePolicyRequest {
    pub policy_name: Option<policyNameType>,
    pub role_name: Option<roleNameType>,
}
impl GetRolePolicyRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetUserPolicyResponse {
    pub user_name: Option<existingUserNameType>,
    pub policy_name: Option<policyNameType>,
    pub policy_document: Option<policyDocumentType>,
}
impl GetUserPolicyResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetOpenIdConnectProvider;
impl OperationShape for GetOpenIdConnectProvider {
    const NAME: &'static str = "GetOpenIdConnectProvider";
    type Input = GetOpenIdConnectProviderRequest;
    type Output = GetOpenIdConnectProviderResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct PutGroupPolicy;
impl OperationShape for PutGroupPolicy {
    const NAME: &'static str = "PutGroupPolicy";
    type Input = PutGroupPolicyRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum policyOwnerEntityType {
    GROUP,
    ROLE,
    USER,
}
impl AsRef<str> for policyOwnerEntityType {
    fn as_ref(&self) -> &str {
        match self {
            Self::GROUP => "GROUP",
            Self::ROLE => "ROLE",
            Self::USER => "USER",
        }
    }
}
impl TryFrom<&str> for policyOwnerEntityType {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "GROUP" => Ok(Self::GROUP),
            "ROLE" => Ok(Self::ROLE),
            "USER" => Ok(Self::USER),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct CreateInstanceProfileResponse {
    pub instance_profile: Option<InstanceProfile>,
}
impl CreateInstanceProfileResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct DuplicateSshPublicKeyException {
    pub message: Option<duplicateSshPublicKeyMessage>,
}
impl DuplicateSshPublicKeyException {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct EntityAlreadyExistsException {
    pub message: Option<entityAlreadyExistsMessage>,
}
impl EntityAlreadyExistsException {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type LineNumber = i32;

#[derive(Debug, Default, Clone)]
pub struct AttachGroupPolicy;
impl OperationShape for AttachGroupPolicy {
    const NAME: &'static str = "AttachGroupPolicy";
    type Input = AttachGroupPolicyRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetServiceLinkedRoleDeletionStatusRequest {
    pub deletion_task_id: Option<DeletionTaskIdType>,
}
impl GetServiceLinkedRoleDeletionStatusRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListMfaDevices;
impl OperationShape for ListMfaDevices {
    const NAME: &'static str = "ListMfaDevices";
    type Input = ListMfaDevicesRequest;
    type Output = ListMfaDevicesResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct CreateGroup;
impl OperationShape for CreateGroup {
    const NAME: &'static str = "CreateGroup";
    type Input = CreateGroupRequest;
    type Output = CreateGroupResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct ListSamlProviderTagsResponse {
    pub marker: Option<responseMarkerType>,
    pub tags: Option<tagListType>,
    pub is_truncated: Option<booleanType>,
}
impl ListSamlProviderTagsResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct UpdateAccessKeyRequest {
    pub user_name: Option<existingUserNameType>,
    pub access_key_id: Option<accessKeyIdType>,
    pub status: Option<statusType>,
}
impl UpdateAccessKeyRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type reportGenerationLimitExceededMessage = String;

#[derive(Debug, Default, Clone)]
pub struct DeletePolicyVersionRequest {
    pub policy_arn: Option<arnType>,
    pub version_id: Option<policyVersionIdType>,
}
impl DeletePolicyVersionRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type ActionNameListType = Vec<ActionNameType>;

#[derive(Debug, Default, Clone)]
pub struct DeleteServiceSpecificCredentialRequest {
    pub user_name: Option<userNameType>,
    pub service_specific_credential_id: Option<serviceSpecificCredentialId>,
}
impl DeleteServiceSpecificCredentialRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GenerateCredentialReport;
impl OperationShape for GenerateCredentialReport {
    const NAME: &'static str = "GenerateCredentialReport";
    type Input = ();
    type Output = GenerateCredentialReportResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct ListAccountAliases;
impl OperationShape for ListAccountAliases {
    const NAME: &'static str = "ListAccountAliases";
    type Input = ListAccountAliasesRequest;
    type Output = ListAccountAliasesResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct ListGroupsResponse {
    pub groups: Option<groupListType>,
    pub is_truncated: Option<booleanType>,
    pub marker: Option<responseMarkerType>,
}
impl ListGroupsResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct DetachRolePolicyRequest {
    pub policy_arn: Option<arnType>,
    pub role_name: Option<roleNameType>,
}
impl DetachRolePolicyRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListMfaDeviceTags;
impl OperationShape for ListMfaDeviceTags {
    const NAME: &'static str = "ListMfaDeviceTags";
    type Input = ListMfaDeviceTagsRequest;
    type Output = ListMfaDeviceTagsResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct UntagRole;
impl OperationShape for UntagRole {
    const NAME: &'static str = "UntagRole";
    type Input = UntagRoleRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct UpdateSshPublicKeyRequest {
    pub ssh_public_key_id: Option<publicKeyIdType>,
    pub user_name: Option<userNameType>,
    pub status: Option<statusType>,
}
impl UpdateSshPublicKeyRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetPolicyVersionRequest {
    pub version_id: Option<policyVersionIdType>,
    pub policy_arn: Option<arnType>,
}
impl GetPolicyVersionRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct SamlProviderListEntry {
    pub arn: Option<arnType>,
    pub create_date: Option<dateType>,
    pub valid_until: Option<dateType>,
}
impl SamlProviderListEntry {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListRoles;
impl OperationShape for ListRoles {
    const NAME: &'static str = "ListRoles";
    type Input = ListRolesRequest;
    type Output = ListRolesResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct ListPoliciesRequest {
    pub max_items: Option<maxItemsType>,
    pub marker: Option<markerType>,
    pub only_attached: Option<booleanType>,
    pub path_prefix: Option<policyPathType>,
    pub policy_usage_filter: Option<PolicyUsageType>,
    pub scope: Option<policyScopeType>,
}
impl ListPoliciesRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type accessKeyMetadataListType = Vec<AccessKeyMetadata>;

#[derive(Debug, Default, Clone)]
pub struct GenerateServiceLastAccessedDetails;
impl OperationShape for GenerateServiceLastAccessedDetails {
    const NAME: &'static str = "GenerateServiceLastAccessedDetails";
    type Input = GenerateServiceLastAccessedDetailsRequest;
    type Output = GenerateServiceLastAccessedDetailsResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetRolePolicy;
impl OperationShape for GetRolePolicy {
    const NAME: &'static str = "GetRolePolicy";
    type Input = GetRolePolicyRequest;
    type Output = GetRolePolicyResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct ListOpenIdConnectProviderTagsResponse {
    pub tags: Option<tagListType>,
    pub is_truncated: Option<booleanType>,
    pub marker: Option<responseMarkerType>,
}
impl ListOpenIdConnectProviderTagsResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type invalidPublicKeyMessage = String;

pub type ReasonType = String;

#[derive(Debug, Default, Clone)]
pub struct ListAttachedGroupPoliciesRequest {
    pub max_items: Option<maxItemsType>,
    pub path_prefix: Option<policyPathType>,
    pub group_name: Option<groupNameType>,
    pub marker: Option<markerType>,
}
impl ListAttachedGroupPoliciesRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct CreateSamlProvider;
impl OperationShape for CreateSamlProvider {
    const NAME: &'static str = "CreateSamlProvider";
    type Input = CreateSamlProviderRequest;
    type Output = CreateSamlProviderResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct CreateVirtualMfaDeviceRequest {
    pub virtual_mfa_device_name: Option<virtualMfaDeviceName>,
    pub path: Option<pathType>,
    pub tags: Option<tagListType>,
}
impl CreateVirtualMfaDeviceRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetLoginProfileResponse {
    pub login_profile: Option<LoginProfile>,
}
impl GetLoginProfileResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct VirtualMfaDevice {
    pub enable_date: Option<dateType>,
    pub qr_code_png: Option<BootstrapDatum>,
    pub tags: Option<tagListType>,
    pub serial_number: Option<serialNumberType>,
    pub user: Option<User>,
    pub base32_string_seed: Option<BootstrapDatum>,
}
impl VirtualMfaDevice {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type serviceNameType = String;

#[derive(Debug, Default, Clone)]
pub struct CreateSamlProviderResponse {
    pub saml_provider_arn: Option<arnType>,
    pub tags: Option<tagListType>,
}
impl CreateSamlProviderResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetServiceLinkedRoleDeletionStatusResponse {
    pub reason: Option<DeletionTaskFailureReasonType>,
    pub status: Option<DeletionTaskStatusType>,
}
impl GetServiceLinkedRoleDeletionStatusResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type policyDocumentType = String;

#[derive(Debug, Default, Clone)]
pub struct CreateGroupRequest {
    pub group_name: Option<groupNameType>,
    pub path: Option<pathType>,
}
impl CreateGroupRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type accessKeyIdType = String;

pub type pathPrefixType = String;

#[derive(Debug, Default, Clone)]
pub struct GetServiceLastAccessedDetailsWithEntities;
impl OperationShape for GetServiceLastAccessedDetailsWithEntities {
    const NAME: &'static str = "GetServiceLastAccessedDetailsWithEntities";
    type Input = GetServiceLastAccessedDetailsWithEntitiesRequest;
    type Output = GetServiceLastAccessedDetailsWithEntitiesResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct CreateRoleResponse {
    pub role: Option<Role>,
}
impl CreateRoleResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct DeleteAccountAliasRequest {
    pub account_alias: Option<accountAliasType>,
}
impl DeleteAccountAliasRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct UpdateAccountPasswordPolicyRequest {
    pub hard_expiry: Option<booleanObjectType>,
    pub minimum_password_length: Option<minimumPasswordLengthType>,
    pub password_reuse_prevention: Option<passwordReusePreventionType>,
    pub require_symbols: Option<booleanType>,
    pub require_uppercase_characters: Option<booleanType>,
    pub max_password_age: Option<maxPasswordAgeType>,
    pub require_numbers: Option<booleanType>,
    pub require_lowercase_characters: Option<booleanType>,
    pub allow_users_to_change_password: Option<booleanType>,
}
impl UpdateAccountPasswordPolicyRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ServerCertificate {
    pub server_certificate_metadata: Option<ServerCertificateMetadata>,
    pub certificate_chain: Option<certificateChainType>,
    pub certificate_body: Option<certificateBodyType>,
    pub tags: Option<tagListType>,
}
impl ServerCertificate {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct UpdateServerCertificateRequest {
    pub new_server_certificate_name: Option<serverCertificateNameType>,
    pub server_certificate_name: Option<serverCertificateNameType>,
    pub new_path: Option<pathType>,
}
impl UpdateServerCertificateRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct UploadSshPublicKey;
impl OperationShape for UploadSshPublicKey {
    const NAME: &'static str = "UploadSshPublicKey";
    type Input = UploadSshPublicKeyRequest;
    type Output = UploadSshPublicKeyResponse;
    type Error = ();
}

pub type passwordType = String;

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum policyType {
    INLINE,
    MANAGED,
}
impl AsRef<str> for policyType {
    fn as_ref(&self) -> &str {
        match self {
            Self::INLINE => "INLINE",
            Self::MANAGED => "MANAGED",
        }
    }
}
impl TryFrom<&str> for policyType {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "INLINE" => Ok(Self::INLINE),
            "MANAGED" => Ok(Self::MANAGED),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct DeleteGroupRequest {
    pub group_name: Option<groupNameType>,
}
impl DeleteGroupRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct OpenIdConnectProviderListEntry {
    pub arn: Option<arnType>,
}
impl OpenIdConnectProviderListEntry {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct UnrecognizedPublicKeyEncodingException {
    pub message: Option<unrecognizedPublicKeyEncodingMessage>,
}
impl UnrecognizedPublicKeyEncodingException {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct SshPublicKey {
    pub upload_date: Option<dateType>,
    pub fingerprint: Option<publicKeyFingerprintType>,
    pub ssh_public_key_body: Option<publicKeyMaterialType>,
    pub ssh_public_key_id: Option<publicKeyIdType>,
    pub status: Option<statusType>,
    pub user_name: Option<userNameType>,
}
impl SshPublicKey {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type noSuchEntityMessage = String;

#[derive(Debug, Default, Clone)]
pub struct InvalidCertificateException {
    pub message: Option<invalidCertificateMessage>,
}
impl InvalidCertificateException {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetAccessKeyLastUsedResponse {
    pub access_key_last_used: Option<AccessKeyLastUsed>,
    pub user_name: Option<existingUserNameType>,
}
impl GetAccessKeyLastUsedResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct InvalidUserTypeException {
    pub message: Option<invalidUserTypeMessage>,
}
impl InvalidUserTypeException {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct PolicyGrantingServiceAccess {
    pub entity_type: Option<policyOwnerEntityType>,
    pub policy_arn: Option<arnType>,
    pub policy_name: Option<policyNameType>,
    pub entity_name: Option<entityNameType>,
    pub policy_type: Option<policyType>,
}
impl PolicyGrantingServiceAccess {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct UploadServerCertificate;
impl OperationShape for UploadServerCertificate {
    const NAME: &'static str = "UploadServerCertificate";
    type Input = UploadServerCertificateRequest;
    type Output = UploadServerCertificateResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct ListRoleTagsRequest {
    pub max_items: Option<maxItemsType>,
    pub role_name: Option<roleNameType>,
    pub marker: Option<markerType>,
}
impl ListRoleTagsRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct RemoveClientIdFromOpenIdConnectProvider;
impl OperationShape for RemoveClientIdFromOpenIdConnectProvider {
    const NAME: &'static str = "RemoveClientIdFromOpenIdConnectProvider";
    type Input = RemoveClientIdFromOpenIdConnectProviderRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct UntagPolicy;
impl OperationShape for UntagPolicy {
    const NAME: &'static str = "UntagPolicy";
    type Input = UntagPolicyRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct UploadServerCertificateResponse {
    pub tags: Option<tagListType>,
    pub server_certificate_metadata: Option<ServerCertificateMetadata>,
}
impl UploadServerCertificateResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListMfaDeviceTagsRequest {
    pub marker: Option<markerType>,
    pub serial_number: Option<serialNumberType>,
    pub max_items: Option<maxItemsType>,
}
impl ListMfaDeviceTagsRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ServiceSpecificCredential {
    pub create_date: Option<dateType>,
    pub service_name: Option<serviceName>,
    pub service_password: Option<servicePassword>,
    pub status: Option<statusType>,
    pub service_user_name: Option<serviceUserName>,
    pub service_specific_credential_id: Option<serviceSpecificCredentialId>,
    pub user_name: Option<userNameType>,
}
impl ServiceSpecificCredential {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type maxItemsType = i32;

pub type roleListType = Vec<Role>;

#[derive(Debug, Default, Clone)]
pub struct RemoveUserFromGroup;
impl OperationShape for RemoveUserFromGroup {
    const NAME: &'static str = "RemoveUserFromGroup";
    type Input = RemoveUserFromGroupRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct AddClientIdToOpenIdConnectProvider;
impl OperationShape for AddClientIdToOpenIdConnectProvider {
    const NAME: &'static str = "AddClientIdToOpenIdConnectProvider";
    type Input = AddClientIdToOpenIdConnectProviderRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetSamlProvider;
impl OperationShape for GetSamlProvider {
    const NAME: &'static str = "GetSamlProvider";
    type Input = GetSamlProviderRequest;
    type Output = GetSamlProviderResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct PutRolePermissionsBoundaryRequest {
    pub permissions_boundary: Option<arnType>,
    pub role_name: Option<roleNameType>,
}
impl PutRolePermissionsBoundaryRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct UntagOpenIdConnectProviderRequest {
    pub open_id_connect_provider_arn: Option<arnType>,
    pub tag_keys: Option<tagKeyListType>,
}
impl UntagOpenIdConnectProviderRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListAccessKeysResponse {
    pub marker: Option<responseMarkerType>,
    pub access_key_metadata: Option<accessKeyMetadataListType>,
    pub is_truncated: Option<booleanType>,
}
impl ListAccessKeysResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListUsersResponse {
    pub is_truncated: Option<booleanType>,
    pub marker: Option<responseMarkerType>,
    pub users: Option<userListType>,
}
impl ListUsersResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct UntagPolicyRequest {
    pub tag_keys: Option<tagKeyListType>,
    pub policy_arn: Option<arnType>,
}
impl UntagPolicyRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type publicKeyIdType = String;

#[derive(Debug, Default, Clone)]
pub struct CreateRole;
impl OperationShape for CreateRole {
    const NAME: &'static str = "CreateRole";
    type Input = CreateRoleRequest;
    type Output = CreateRoleResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GenerateOrganizationsAccessReportRequest {
    pub organizations_policy_id: Option<organizationsPolicyIdType>,
    pub entity_path: Option<organizationsEntityPathType>,
}
impl GenerateOrganizationsAccessReportRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetInstanceProfileResponse {
    pub instance_profile: Option<InstanceProfile>,
}
impl GetInstanceProfileResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type ServicesLastAccessed = Vec<ServiceLastAccessed>;

pub type ContextKeyValueType = String;

#[derive(Debug, Default, Clone)]
pub struct ListAccountAliasesResponse {
    pub account_aliases: Option<accountAliasListType>,
    pub marker: Option<responseMarkerType>,
    pub is_truncated: Option<booleanType>,
}
impl ListAccountAliasesResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type ServiceSpecificCredentialsListType = Vec<ServiceSpecificCredentialMetadata>;

#[derive(Debug, Default, Clone)]
pub struct ListRolePoliciesRequest {
    pub max_items: Option<maxItemsType>,
    pub marker: Option<markerType>,
    pub role_name: Option<roleNameType>,
}
impl ListRolePoliciesRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct CredentialReportExpiredException {
    pub message: Option<credentialReportExpiredExceptionMessage>,
}
impl CredentialReportExpiredException {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct DeleteServiceLinkedRole;
impl OperationShape for DeleteServiceLinkedRole {
    const NAME: &'static str = "DeleteServiceLinkedRole";
    type Input = DeleteServiceLinkedRoleRequest;
    type Output = DeleteServiceLinkedRoleResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetUser;
impl OperationShape for GetUser {
    const NAME: &'static str = "GetUser";
    type Input = GetUserRequest;
    type Output = GetUserResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetAccountAuthorizationDetailsRequest {
    pub filter: Option<entityListType>,
    pub max_items: Option<maxItemsType>,
    pub marker: Option<markerType>,
}
impl GetAccountAuthorizationDetailsRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListUserPolicies;
impl OperationShape for ListUserPolicies {
    const NAME: &'static str = "ListUserPolicies";
    type Input = ListUserPoliciesRequest;
    type Output = ListUserPoliciesResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct UpdateOpenIdConnectProviderThumbprintRequest {
    pub thumbprint_list: Option<thumbprintListType>,
    pub open_id_connect_provider_arn: Option<arnType>,
}
impl UpdateOpenIdConnectProviderThumbprintRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct InvalidPublicKeyException {
    pub message: Option<invalidPublicKeyMessage>,
}
impl InvalidPublicKeyException {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct RemoveClientIdFromOpenIdConnectProviderRequest {
    pub open_id_connect_provider_arn: Option<arnType>,
    pub client_id: Option<clientIdType>,
}
impl RemoveClientIdFromOpenIdConnectProviderRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type tagKeyType = String;

#[derive(Debug, Default, Clone)]
pub struct CreateSamlProviderRequest {
    pub tags: Option<tagListType>,
    pub saml_metadata_document: Option<SamlMetadataDocumentType>,
    pub name: Option<SamlProviderNameType>,
}
impl CreateSamlProviderRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type groupNameListType = Vec<groupNameType>;

pub type entityListType = Vec<EntityType>;

#[derive(Debug, Default, Clone)]
pub struct UntagOpenIdConnectProvider;
impl OperationShape for UntagOpenIdConnectProvider {
    const NAME: &'static str = "UntagOpenIdConnectProvider";
    type Input = UntagOpenIdConnectProviderRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct TagSamlProviderRequest {
    pub tags: Option<tagListType>,
    pub saml_provider_arn: Option<arnType>,
}
impl TagSamlProviderRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type limitExceededMessage = String;

#[derive(Debug, Default, Clone)]
pub struct DeleteRolePermissionsBoundary;
impl OperationShape for DeleteRolePermissionsBoundary {
    const NAME: &'static str = "DeleteRolePermissionsBoundary";
    type Input = DeleteRolePermissionsBoundaryRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct ListAttachedUserPolicies;
impl OperationShape for ListAttachedUserPolicies {
    const NAME: &'static str = "ListAttachedUserPolicies";
    type Input = ListAttachedUserPoliciesRequest;
    type Output = ListAttachedUserPoliciesResponse;
    type Error = ();
}

pub type ResourceNameListType = Vec<ResourceNameType>;

pub type userDetailListType = Vec<UserDetail>;

#[derive(Debug, Default, Clone)]
pub struct UpdateAccountPasswordPolicy;
impl OperationShape for UpdateAccountPasswordPolicy {
    const NAME: &'static str = "UpdateAccountPasswordPolicy";
    type Input = UpdateAccountPasswordPolicyRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct ListPoliciesGrantingServiceAccessResponse {
    pub marker: Option<responseMarkerType>,
    pub is_truncated: Option<booleanType>,
    pub policies_granting_service_access: Option<listPolicyGrantingServiceAccessResponseListType>,
}
impl ListPoliciesGrantingServiceAccessResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct PasswordPolicy {
    pub max_password_age: Option<maxPasswordAgeType>,
    pub minimum_password_length: Option<minimumPasswordLengthType>,
    pub require_numbers: Option<booleanType>,
    pub require_uppercase_characters: Option<booleanType>,
    pub password_reuse_prevention: Option<passwordReusePreventionType>,
    pub require_lowercase_characters: Option<booleanType>,
    pub allow_users_to_change_password: Option<booleanType>,
    pub hard_expiry: Option<booleanObjectType>,
    pub require_symbols: Option<booleanType>,
    pub expire_passwords: Option<booleanType>,
}
impl PasswordPolicy {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetOrganizationsAccessReportRequest {
    pub max_items: Option<maxItemsType>,
    pub sort_key: Option<sortKeyType>,
    pub marker: Option<markerType>,
    pub job_id: Option<jobIdType>,
}
impl GetOrganizationsAccessReportRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type roleDescriptionType = String;

#[derive(Debug, Default, Clone)]
pub struct NoSuchEntityException {
    pub message: Option<noSuchEntityMessage>,
}
impl NoSuchEntityException {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetServiceLastAccessedDetailsWithEntitiesResponse {
    pub is_truncated: Option<booleanType>,
    pub entity_details_list: Option<entityDetailsListType>,
    pub error: Option<ErrorDetails>,
    pub job_creation_date: Option<dateType>,
    pub job_completion_date: Option<dateType>,
    pub job_status: Option<jobStatusType>,
    pub marker: Option<responseMarkerType>,
}
impl GetServiceLastAccessedDetailsWithEntitiesResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct TagInstanceProfileRequest {
    pub instance_profile_name: Option<instanceProfileNameType>,
    pub tags: Option<tagListType>,
}
impl TagInstanceProfileRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type policyDocumentVersionListType = Vec<PolicyVersion>;

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum encodingType {
    SSH,
    PEM,
}
impl AsRef<str> for encodingType {
    fn as_ref(&self) -> &str {
        match self {
            Self::SSH => "SSH",
            Self::PEM => "PEM",
        }
    }
}
impl TryFrom<&str> for encodingType {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "SSH" => Ok(Self::SSH),
            "PEM" => Ok(Self::PEM),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ReportGenerationLimitExceededException {
    pub message: Option<reportGenerationLimitExceededMessage>,
}
impl ReportGenerationLimitExceededException {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct UpdateServiceSpecificCredential;
impl OperationShape for UpdateServiceSpecificCredential {
    const NAME: &'static str = "UpdateServiceSpecificCredential";
    type Input = UpdateServiceSpecificCredentialRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetPolicy;
impl OperationShape for GetPolicy {
    const NAME: &'static str = "GetPolicy";
    type Input = GetPolicyRequest;
    type Output = GetPolicyResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct RoleLastUsed {
    pub region: Option<stringType>,
    pub last_used_date: Option<dateType>,
}
impl RoleLastUsed {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct DeleteAccessKey;
impl OperationShape for DeleteAccessKey {
    const NAME: &'static str = "DeleteAccessKey";
    type Input = DeleteAccessKeyRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct ListServerCertificatesResponse {
    pub marker: Option<responseMarkerType>,
    pub server_certificate_metadata_list: Option<serverCertificateMetadataListType>,
    pub is_truncated: Option<booleanType>,
}
impl ListServerCertificatesResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type duplicateCertificateMessage = String;

#[derive(Debug, Default, Clone)]
pub struct EnableMfaDeviceRequest {
    pub authentication_code1: Option<authenticationCodeType>,
    pub serial_number: Option<serialNumberType>,
    pub user_name: Option<existingUserNameType>,
    pub authentication_code2: Option<authenticationCodeType>,
}
impl EnableMfaDeviceRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type policyNotAttachableMessage = String;

#[derive(Debug, Default, Clone)]
pub struct SimulatePrincipalPolicy;
impl OperationShape for SimulatePrincipalPolicy {
    const NAME: &'static str = "SimulatePrincipalPolicy";
    type Input = SimulatePrincipalPolicyRequest;
    type Output = SimulatePolicyResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetGroupPolicyRequest {
    pub group_name: Option<groupNameType>,
    pub policy_name: Option<policyNameType>,
}
impl GetGroupPolicyRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum summaryKeyType {
    GroupPolicySizeQuota,
    PolicySizeQuota,
    GroupsPerUserQuota,
    AccountSigningCertificatesPresent,
    PolicyVersionsInUse,
    GroupsQuota,
    AccountMFAEnabled,
    PoliciesQuota,
    MFADevices,
    ServerCertificatesQuota,
    UserPolicySizeQuota,
    UsersQuota,
    MFADevicesInUse,
    VersionsPerPolicyQuota,
    AttachedPoliciesPerRoleQuota,
    AccessKeysPerUserQuota,
    AttachedPoliciesPerUserQuota,
    PolicyVersionsInUseQuota,
    ServerCertificates,
    GlobalEndpointTokenVersion,
    Policies,
    Users,
    SigningCertificatesPerUserQuota,
    AccountAccessKeysPresent,
    Groups,
    AttachedPoliciesPerGroupQuota,
}
impl AsRef<str> for summaryKeyType {
    fn as_ref(&self) -> &str {
        match self {
            Self::GroupPolicySizeQuota => "GroupPolicySizeQuota",
            Self::PolicySizeQuota => "PolicySizeQuota",
            Self::GroupsPerUserQuota => "GroupsPerUserQuota",
            Self::AccountSigningCertificatesPresent => "AccountSigningCertificatesPresent",
            Self::PolicyVersionsInUse => "PolicyVersionsInUse",
            Self::GroupsQuota => "GroupsQuota",
            Self::AccountMFAEnabled => "AccountMFAEnabled",
            Self::PoliciesQuota => "PoliciesQuota",
            Self::MFADevices => "MFADevices",
            Self::ServerCertificatesQuota => "ServerCertificatesQuota",
            Self::UserPolicySizeQuota => "UserPolicySizeQuota",
            Self::UsersQuota => "UsersQuota",
            Self::MFADevicesInUse => "MFADevicesInUse",
            Self::VersionsPerPolicyQuota => "VersionsPerPolicyQuota",
            Self::AttachedPoliciesPerRoleQuota => "AttachedPoliciesPerRoleQuota",
            Self::AccessKeysPerUserQuota => "AccessKeysPerUserQuota",
            Self::AttachedPoliciesPerUserQuota => "AttachedPoliciesPerUserQuota",
            Self::PolicyVersionsInUseQuota => "PolicyVersionsInUseQuota",
            Self::ServerCertificates => "ServerCertificates",
            Self::GlobalEndpointTokenVersion => "GlobalEndpointTokenVersion",
            Self::Policies => "Policies",
            Self::Users => "Users",
            Self::SigningCertificatesPerUserQuota => "SigningCertificatesPerUserQuota",
            Self::AccountAccessKeysPresent => "AccountAccessKeysPresent",
            Self::Groups => "Groups",
            Self::AttachedPoliciesPerGroupQuota => "AttachedPoliciesPerGroupQuota",
        }
    }
}
impl TryFrom<&str> for summaryKeyType {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "GroupPolicySizeQuota" => Ok(Self::GroupPolicySizeQuota),
            "PolicySizeQuota" => Ok(Self::PolicySizeQuota),
            "GroupsPerUserQuota" => Ok(Self::GroupsPerUserQuota),
            "AccountSigningCertificatesPresent" => Ok(Self::AccountSigningCertificatesPresent),
            "PolicyVersionsInUse" => Ok(Self::PolicyVersionsInUse),
            "GroupsQuota" => Ok(Self::GroupsQuota),
            "AccountMFAEnabled" => Ok(Self::AccountMFAEnabled),
            "PoliciesQuota" => Ok(Self::PoliciesQuota),
            "MFADevices" => Ok(Self::MFADevices),
            "ServerCertificatesQuota" => Ok(Self::ServerCertificatesQuota),
            "UserPolicySizeQuota" => Ok(Self::UserPolicySizeQuota),
            "UsersQuota" => Ok(Self::UsersQuota),
            "MFADevicesInUse" => Ok(Self::MFADevicesInUse),
            "VersionsPerPolicyQuota" => Ok(Self::VersionsPerPolicyQuota),
            "AttachedPoliciesPerRoleQuota" => Ok(Self::AttachedPoliciesPerRoleQuota),
            "AccessKeysPerUserQuota" => Ok(Self::AccessKeysPerUserQuota),
            "AttachedPoliciesPerUserQuota" => Ok(Self::AttachedPoliciesPerUserQuota),
            "PolicyVersionsInUseQuota" => Ok(Self::PolicyVersionsInUseQuota),
            "ServerCertificates" => Ok(Self::ServerCertificates),
            "GlobalEndpointTokenVersion" => Ok(Self::GlobalEndpointTokenVersion),
            "Policies" => Ok(Self::Policies),
            "Users" => Ok(Self::Users),
            "SigningCertificatesPerUserQuota" => Ok(Self::SigningCertificatesPerUserQuota),
            "AccountAccessKeysPresent" => Ok(Self::AccountAccessKeysPresent),
            "Groups" => Ok(Self::Groups),
            "AttachedPoliciesPerGroupQuota" => Ok(Self::AttachedPoliciesPerGroupQuota),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListSamlProviders;
impl OperationShape for ListSamlProviders {
    const NAME: &'static str = "ListSamlProviders";
    type Input = ListSamlProvidersRequest;
    type Output = ListSamlProvidersResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct DeleteRolePolicy;
impl OperationShape for DeleteRolePolicy {
    const NAME: &'static str = "DeleteRolePolicy";
    type Input = DeleteRolePolicyRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct ContextEntry {
    pub context_key_name: Option<ContextKeyNameType>,
    pub context_key_type: Option<ContextKeyTypeEnum>,
    pub context_key_values: Option<ContextKeyValueListType>,
}
impl ContextEntry {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct DeleteGroupPolicy;
impl OperationShape for DeleteGroupPolicy {
    const NAME: &'static str = "DeleteGroupPolicy";
    type Input = DeleteGroupPolicyRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct DeleteServerCertificate;
impl OperationShape for DeleteServerCertificate {
    const NAME: &'static str = "DeleteServerCertificate";
    type Input = DeleteServerCertificateRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct ListGroupsForUser;
impl OperationShape for ListGroupsForUser {
    const NAME: &'static str = "ListGroupsForUser";
    type Input = ListGroupsForUserRequest;
    type Output = ListGroupsForUserResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct SetDefaultPolicyVersion;
impl OperationShape for SetDefaultPolicyVersion {
    const NAME: &'static str = "SetDefaultPolicyVersion";
    type Input = SetDefaultPolicyVersionRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum globalEndpointTokenVersion {
    v1Token,
    v2Token,
}
impl AsRef<str> for globalEndpointTokenVersion {
    fn as_ref(&self) -> &str {
        match self {
            Self::v1Token => "v1Token",
            Self::v2Token => "v2Token",
        }
    }
}
impl TryFrom<&str> for globalEndpointTokenVersion {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "v1Token" => Ok(Self::v1Token),
            "v2Token" => Ok(Self::v2Token),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

pub type tagValueType = String;

pub type OpenIdConnectProviderListType = Vec<OpenIdConnectProviderListEntry>;

#[derive(Debug, Default, Clone)]
pub struct PutGroupPolicyRequest {
    pub policy_name: Option<policyNameType>,
    pub group_name: Option<groupNameType>,
    pub policy_document: Option<policyDocumentType>,
}
impl PutGroupPolicyRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct PolicyGroup {
    pub group_id: Option<idType>,
    pub group_name: Option<groupNameType>,
}
impl PolicyGroup {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct TagInstanceProfile;
impl OperationShape for TagInstanceProfile {
    const NAME: &'static str = "TagInstanceProfile";
    type Input = TagInstanceProfileRequest;
    type Output = ();
    type Error = ();
}

pub type maxPasswordAgeType = i32;

#[derive(Debug, Default, Clone)]
pub struct GetInstanceProfile;
impl OperationShape for GetInstanceProfile {
    const NAME: &'static str = "GetInstanceProfile";
    type Input = GetInstanceProfileRequest;
    type Output = GetInstanceProfileResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetContextKeysForPrincipalPolicy;
impl OperationShape for GetContextKeysForPrincipalPolicy {
    const NAME: &'static str = "GetContextKeysForPrincipalPolicy";
    type Input = GetContextKeysForPrincipalPolicyRequest;
    type Output = GetContextKeysForPolicyResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct UntagInstanceProfileRequest {
    pub instance_profile_name: Option<instanceProfileNameType>,
    pub tag_keys: Option<tagKeyListType>,
}
impl UntagInstanceProfileRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct UpdateSshPublicKey;
impl OperationShape for UpdateSshPublicKey {
    const NAME: &'static str = "UpdateSshPublicKey";
    type Input = UpdateSshPublicKeyRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct EntityInfo {
    pub path: Option<pathType>,
    pub arn: Option<arnType>,
    pub id: Option<idType>,
    pub name: Option<userNameType>,
    pub r#type: Option<policyOwnerEntityType>,
}
impl EntityInfo {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListAttachedRolePoliciesResponse {
    pub marker: Option<responseMarkerType>,
    pub is_truncated: Option<booleanType>,
    pub attached_policies: Option<attachedPoliciesListType>,
}
impl ListAttachedRolePoliciesResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetPolicyResponse {
    pub policy: Option<Policy>,
}
impl GetPolicyResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListSigningCertificatesRequest {
    pub marker: Option<markerType>,
    pub user_name: Option<existingUserNameType>,
    pub max_items: Option<maxItemsType>,
}
impl ListSigningCertificatesRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type policyDescriptionType = String;

#[derive(Debug, Default, Clone)]
pub struct ListSshPublicKeysResponse {
    pub is_truncated: Option<booleanType>,
    pub marker: Option<responseMarkerType>,
    pub ssh_public_keys: Option<SshPublicKeyListType>,
}
impl ListSshPublicKeysResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type StatementListType = Vec<Statement>;

#[derive(Debug, Default, Clone)]
pub struct DeleteUserPolicyRequest {
    pub user_name: Option<existingUserNameType>,
    pub policy_name: Option<policyNameType>,
}
impl DeleteUserPolicyRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct UploadSigningCertificate;
impl OperationShape for UploadSigningCertificate {
    const NAME: &'static str = "UploadSigningCertificate";
    type Input = UploadSigningCertificateRequest;
    type Output = UploadSigningCertificateResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct ListInstanceProfilesForRoleResponse {
    pub is_truncated: Option<booleanType>,
    pub instance_profiles: Option<instanceProfileListType>,
    pub marker: Option<responseMarkerType>,
}
impl ListInstanceProfilesForRoleResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type minimumPasswordLengthType = i32;

#[derive(Debug, Default, Clone)]
pub struct RoleUsageType {
    pub region: Option<RegionNameType>,
    pub resources: Option<ArnListType>,
}
impl RoleUsageType {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct TagServerCertificate;
impl OperationShape for TagServerCertificate {
    const NAME: &'static str = "TagServerCertificate";
    type Input = TagServerCertificateRequest;
    type Output = ();
    type Error = ();
}

pub type idType = String;

#[derive(Debug, Default, Clone)]
pub struct CreateAccessKeyRequest {
    pub user_name: Option<existingUserNameType>,
}
impl CreateAccessKeyRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct DeletePolicy;
impl OperationShape for DeletePolicy {
    const NAME: &'static str = "DeletePolicy";
    type Input = DeletePolicyRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetSshPublicKeyRequest {
    pub encoding: Option<encodingType>,
    pub user_name: Option<userNameType>,
    pub ssh_public_key_id: Option<publicKeyIdType>,
}
impl GetSshPublicKeyRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListMfaDevicesRequest {
    pub user_name: Option<existingUserNameType>,
    pub marker: Option<markerType>,
    pub max_items: Option<maxItemsType>,
}
impl ListMfaDevicesRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListPolicyTags;
impl OperationShape for ListPolicyTags {
    const NAME: &'static str = "ListPolicyTags";
    type Input = ListPolicyTagsRequest;
    type Output = ListPolicyTagsResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct DeleteUserRequest {
    pub user_name: Option<existingUserNameType>,
}
impl DeleteUserRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListUserTags;
impl OperationShape for ListUserTags {
    const NAME: &'static str = "ListUserTags";
    type Input = ListUserTagsRequest;
    type Output = ListUserTagsResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct PasswordPolicyViolationException {
    pub message: Option<passwordPolicyViolationMessage>,
}
impl PasswordPolicyViolationException {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct PolicyNotAttachableException {
    pub message: Option<policyNotAttachableMessage>,
}
impl PolicyNotAttachableException {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListOpenIdConnectProvidersRequest {}
impl ListOpenIdConnectProvidersRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type RegionNameType = String;

pub type certificateBodyType = String;

pub type clientIdListType = Vec<clientIdType>;

#[derive(Debug, Default, Clone)]
pub struct DetachUserPolicy;
impl OperationShape for DetachUserPolicy {
    const NAME: &'static str = "DetachUserPolicy";
    type Input = DetachUserPolicyRequest;
    type Output = ();
    type Error = ();
}

pub type policyVersionIdType = String;

#[derive(Debug, Default, Clone)]
pub struct ListEntitiesForPolicyRequest {
    pub max_items: Option<maxItemsType>,
    pub marker: Option<markerType>,
    pub path_prefix: Option<pathType>,
    pub entity_filter: Option<EntityType>,
    pub policy_arn: Option<arnType>,
    pub policy_usage_filter: Option<PolicyUsageType>,
}
impl ListEntitiesForPolicyRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct CreateUser;
impl OperationShape for CreateUser {
    const NAME: &'static str = "CreateUser";
    type Input = CreateUserRequest;
    type Output = CreateUserResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct AccessKeyMetadata {
    pub create_date: Option<dateType>,
    pub access_key_id: Option<accessKeyIdType>,
    pub status: Option<statusType>,
    pub user_name: Option<userNameType>,
}
impl AccessKeyMetadata {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct SetSecurityTokenServicePreferences;
impl OperationShape for SetSecurityTokenServicePreferences {
    const NAME: &'static str = "SetSecurityTokenServicePreferences";
    type Input = SetSecurityTokenServicePreferencesRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct ListSshPublicKeysRequest {
    pub marker: Option<markerType>,
    pub max_items: Option<maxItemsType>,
    pub user_name: Option<userNameType>,
}
impl ListSshPublicKeysRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct PolicyVersion {
    pub create_date: Option<dateType>,
    pub document: Option<policyDocumentType>,
    pub version_id: Option<policyVersionIdType>,
    pub is_default_version: Option<booleanType>,
}
impl PolicyVersion {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct CreateServiceSpecificCredentialResponse {
    pub service_specific_credential: Option<ServiceSpecificCredential>,
}
impl CreateServiceSpecificCredentialResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct TagOpenIdConnectProviderRequest {
    pub open_id_connect_provider_arn: Option<arnType>,
    pub tags: Option<tagListType>,
}
impl TagOpenIdConnectProviderRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct UploadServerCertificateRequest {
    pub tags: Option<tagListType>,
    pub certificate_body: Option<certificateBodyType>,
    pub certificate_chain: Option<certificateChainType>,
    pub path: Option<pathType>,
    pub private_key: Option<privateKeyType>,
    pub server_certificate_name: Option<serverCertificateNameType>,
}
impl UploadServerCertificateRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type serviceSpecificCredentialId = String;

pub type groupListType = Vec<Group>;

#[derive(Debug, Default, Clone)]
pub struct LimitExceededException {
    pub message: Option<limitExceededMessage>,
}
impl LimitExceededException {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ResetServiceSpecificCredentialResponse {
    pub service_specific_credential: Option<ServiceSpecificCredential>,
}
impl ResetServiceSpecificCredentialResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type RoleUsageListType = Vec<RoleUsageType>;

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum jobStatusType {
    IN_PROGRESS,
    COMPLETED,
    FAILED,
}
impl AsRef<str> for jobStatusType {
    fn as_ref(&self) -> &str {
        match self {
            Self::IN_PROGRESS => "IN_PROGRESS",
            Self::COMPLETED => "COMPLETED",
            Self::FAILED => "FAILED",
        }
    }
}
impl TryFrom<&str> for jobStatusType {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "IN_PROGRESS" => Ok(Self::IN_PROGRESS),
            "COMPLETED" => Ok(Self::COMPLETED),
            "FAILED" => Ok(Self::FAILED),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct MalformedCertificateException {
    pub message: Option<malformedCertificateMessage>,
}
impl MalformedCertificateException {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct UntagSamlProvider;
impl OperationShape for UntagSamlProvider {
    const NAME: &'static str = "UntagSamlProvider";
    type Input = UntagSamlProviderRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct CreateVirtualMfaDeviceResponse {
    pub virtual_mfa_device: Option<VirtualMfaDevice>,
}
impl CreateVirtualMfaDeviceResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct AddUserToGroupRequest {
    pub group_name: Option<groupNameType>,
    pub user_name: Option<existingUserNameType>,
}
impl AddUserToGroupRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetOrganizationsAccessReportResponse {
    pub job_completion_date: Option<dateType>,
    pub is_truncated: Option<booleanType>,
    pub error_details: Option<ErrorDetails>,
    pub number_of_services_not_accessed: Option<integerType>,
    pub marker: Option<markerType>,
    pub job_status: Option<jobStatusType>,
    pub access_details: Option<AccessDetails>,
    pub number_of_services_accessible: Option<integerType>,
    pub job_creation_date: Option<dateType>,
}
impl GetOrganizationsAccessReportResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type stringType = String;

#[derive(Debug, Default, Clone)]
pub struct AttachRolePolicy;
impl OperationShape for AttachRolePolicy {
    const NAME: &'static str = "AttachRolePolicy";
    type Input = AttachRolePolicyRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum assignmentStatusType {
    Any,
    Assigned,
    Unassigned,
}
impl AsRef<str> for assignmentStatusType {
    fn as_ref(&self) -> &str {
        match self {
            Self::Any => "Any",
            Self::Assigned => "Assigned",
            Self::Unassigned => "Unassigned",
        }
    }
}
impl TryFrom<&str> for assignmentStatusType {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "Any" => Ok(Self::Any),
            "Assigned" => Ok(Self::Assigned),
            "Unassigned" => Ok(Self::Unassigned),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

pub type mfaDeviceListType = Vec<MfaDevice>;

#[derive(Debug, Default, Clone)]
pub struct ListPoliciesGrantingServiceAccessRequest {
    pub marker: Option<markerType>,
    pub arn: Option<arnType>,
    pub service_namespaces: Option<serviceNamespaceListType>,
}
impl ListPoliciesGrantingServiceAccessRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type ContextKeyNamesResultListType = Vec<ContextKeyNameType>;

pub type SamlProviderNameType = String;

#[derive(Debug, Default, Clone)]
pub struct UntagServerCertificate;
impl OperationShape for UntagServerCertificate {
    const NAME: &'static str = "UntagServerCertificate";
    type Input = UntagServerCertificateRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct DeletionTaskFailureReasonType {
    pub role_usage_list: Option<RoleUsageListType>,
    pub reason: Option<ReasonType>,
}
impl DeletionTaskFailureReasonType {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type attachedPoliciesListType = Vec<AttachedPolicy>;

#[derive(Debug, Default, Clone)]
pub struct CreateServiceLinkedRole;
impl OperationShape for CreateServiceLinkedRole {
    const NAME: &'static str = "CreateServiceLinkedRole";
    type Input = CreateServiceLinkedRoleRequest;
    type Output = CreateServiceLinkedRoleResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct PolicyEvaluationException {
    pub message: Option<policyEvaluationErrorMessage>,
}
impl PolicyEvaluationException {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListServiceSpecificCredentialsRequest {
    pub user_name: Option<userNameType>,
    pub service_name: Option<serviceName>,
}
impl ListServiceSpecificCredentialsRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type EvalDecisionDetailsType = HashMap<EvalDecisionSourceType, PolicyEvaluationDecisionType>;

#[derive(Debug, Default, Clone)]
pub struct ListSigningCertificatesResponse {
    pub certificates: Option<certificateListType>,
    pub marker: Option<responseMarkerType>,
    pub is_truncated: Option<booleanType>,
}
impl ListSigningCertificatesResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct UpdateSamlProvider;
impl OperationShape for UpdateSamlProvider {
    const NAME: &'static str = "UpdateSamlProvider";
    type Input = UpdateSamlProviderRequest;
    type Output = UpdateSamlProviderResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct AttachUserPolicyRequest {
    pub user_name: Option<userNameType>,
    pub policy_arn: Option<arnType>,
}
impl AttachUserPolicyRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListUsers;
impl OperationShape for ListUsers {
    const NAME: &'static str = "ListUsers";
    type Input = ListUsersRequest;
    type Output = ListUsersResponse;
    type Error = ();
}

pub type serviceNamespaceType = String;

#[derive(Debug, Default, Clone)]
pub struct DeleteLoginProfile;
impl OperationShape for DeleteLoginProfile {
    const NAME: &'static str = "DeleteLoginProfile";
    type Input = DeleteLoginProfileRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct ListGroupsForUserRequest {
    pub max_items: Option<maxItemsType>,
    pub marker: Option<markerType>,
    pub user_name: Option<existingUserNameType>,
}
impl ListGroupsForUserRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct UpdateAccessKey;
impl OperationShape for UpdateAccessKey {
    const NAME: &'static str = "UpdateAccessKey";
    type Input = UpdateAccessKeyRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct CreatePolicyVersion;
impl OperationShape for CreatePolicyVersion {
    const NAME: &'static str = "CreatePolicyVersion";
    type Input = CreatePolicyVersionRequest;
    type Output = CreatePolicyVersionResponse;
    type Error = ();
}

pub type tagListType = Vec<Tag>;

#[derive(Debug, Default, Clone)]
pub struct DeleteSigningCertificate;
impl OperationShape for DeleteSigningCertificate {
    const NAME: &'static str = "DeleteSigningCertificate";
    type Input = DeleteSigningCertificateRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct Position {
    pub column: Option<ColumnNumber>,
    pub line: Option<LineNumber>,
}
impl Position {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type roleMaxSessionDurationType = i32;

#[derive(Debug, Default, Clone)]
pub struct CreatePolicy;
impl OperationShape for CreatePolicy {
    const NAME: &'static str = "CreatePolicy";
    type Input = CreatePolicyRequest;
    type Output = CreatePolicyResponse;
    type Error = ();
}

pub type instanceProfileListType = Vec<InstanceProfile>;

#[derive(Debug, Default, Clone)]
pub struct ListAccountAliasesRequest {
    pub max_items: Option<maxItemsType>,
    pub marker: Option<markerType>,
}
impl ListAccountAliasesRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct DeleteAccessKeyRequest {
    pub access_key_id: Option<accessKeyIdType>,
    pub user_name: Option<existingUserNameType>,
}
impl DeleteAccessKeyRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum PermissionsBoundaryAttachmentType {
    Policy,
}
impl AsRef<str> for PermissionsBoundaryAttachmentType {
    fn as_ref(&self) -> &str {
        match self {
            Self::Policy => "PermissionsBoundaryPolicy",
        }
    }
}
impl TryFrom<&str> for PermissionsBoundaryAttachmentType {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "PermissionsBoundaryPolicy" => Ok(Self::Policy),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct SimulatePolicyResponse {
    pub evaluation_results: Option<EvaluationResultsListType>,
    pub is_truncated: Option<booleanType>,
    pub marker: Option<responseMarkerType>,
}
impl SimulatePolicyResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct DeleteServiceLinkedRoleRequest {
    pub role_name: Option<roleNameType>,
}
impl DeleteServiceLinkedRoleRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct DeleteServiceSpecificCredential;
impl OperationShape for DeleteServiceSpecificCredential {
    const NAME: &'static str = "DeleteServiceSpecificCredential";
    type Input = DeleteServiceSpecificCredentialRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct ListUserPoliciesRequest {
    pub user_name: Option<existingUserNameType>,
    pub marker: Option<markerType>,
    pub max_items: Option<maxItemsType>,
}
impl ListUserPoliciesRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type ResourceSpecificResultListType = Vec<ResourceSpecificResult>;

#[derive(Debug, Default, Clone)]
pub struct DeletePolicyRequest {
    pub policy_arn: Option<arnType>,
}
impl DeletePolicyRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListUserPoliciesResponse {
    pub policy_names: Option<policyNameListType>,
    pub is_truncated: Option<booleanType>,
    pub marker: Option<responseMarkerType>,
}
impl ListUserPoliciesResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct UpdateUser;
impl OperationShape for UpdateUser {
    const NAME: &'static str = "UpdateUser";
    type Input = UpdateUserRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct ListUserTagsResponse {
    pub marker: Option<responseMarkerType>,
    pub tags: Option<tagListType>,
    pub is_truncated: Option<booleanType>,
}
impl ListUserTagsResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type accountAliasType = String;

pub type ReportContentType = Vec<u8>;

pub type servicePassword = String;

#[derive(Debug, Default, Clone)]
pub struct GetServiceLastAccessedDetailsWithEntitiesRequest {
    pub job_id: Option<jobIdType>,
    pub max_items: Option<maxItemsType>,
    pub service_namespace: Option<serviceNamespaceType>,
    pub marker: Option<markerType>,
}
impl GetServiceLastAccessedDetailsWithEntitiesRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct AwsIdentityManagementV20100508 {
    pub untag_open_id_connect_provider: Option<OpService<UntagOpenIdConnectProvider>>,
    pub update_group: Option<OpService<UpdateGroup>>,
    pub create_open_id_connect_provider: Option<OpService<CreateOpenIdConnectProvider>>,
    pub put_user_policy: Option<OpService<PutUserPolicy>>,
    pub set_security_token_service_preferences:
        Option<OpService<SetSecurityTokenServicePreferences>>,
    pub list_signing_certificates: Option<OpService<ListSigningCertificates>>,
    pub list_entities_for_policy: Option<OpService<ListEntitiesForPolicy>>,
    pub list_user_tags: Option<OpService<ListUserTags>>,
    pub get_policy_version: Option<OpService<GetPolicyVersion>>,
    pub list_groups: Option<OpService<ListGroups>>,
    pub list_mfa_device_tags: Option<OpService<ListMfaDeviceTags>>,
    pub delete_role_policy: Option<OpService<DeleteRolePolicy>>,
    pub detach_group_policy: Option<OpService<DetachGroupPolicy>>,
    pub get_user_policy: Option<OpService<GetUserPolicy>>,
    pub tag_user: Option<OpService<TagUser>>,
    pub update_role: Option<OpService<UpdateRole>>,
    pub upload_server_certificate: Option<OpService<UploadServerCertificate>>,
    pub tag_mfa_device: Option<OpService<TagMfaDevice>>,
    pub attach_user_policy: Option<OpService<AttachUserPolicy>>,
    pub add_client_id_to_open_id_connect_provider:
        Option<OpService<AddClientIdToOpenIdConnectProvider>>,
    pub get_organizations_access_report: Option<OpService<GetOrganizationsAccessReport>>,
    pub list_policies_granting_service_access: Option<OpService<ListPoliciesGrantingServiceAccess>>,
    pub list_users: Option<OpService<ListUsers>>,
    pub delete_open_id_connect_provider: Option<OpService<DeleteOpenIdConnectProvider>>,
    pub generate_service_last_accessed_details:
        Option<OpService<GenerateServiceLastAccessedDetails>>,
    pub get_server_certificate: Option<OpService<GetServerCertificate>>,
    pub tag_saml_provider: Option<OpService<TagSamlProvider>>,
    pub create_saml_provider: Option<OpService<CreateSamlProvider>>,
    pub create_account_alias: Option<OpService<CreateAccountAlias>>,
    pub add_role_to_instance_profile: Option<OpService<AddRoleToInstanceProfile>>,
    pub list_attached_group_policies: Option<OpService<ListAttachedGroupPolicies>>,
    pub delete_user_permissions_boundary: Option<OpService<DeleteUserPermissionsBoundary>>,
    pub delete_group: Option<OpService<DeleteGroup>>,
    pub get_role: Option<OpService<GetRole>>,
    pub list_mfa_devices: Option<OpService<ListMfaDevices>>,
    pub put_role_permissions_boundary: Option<OpService<PutRolePermissionsBoundary>>,
    pub add_user_to_group: Option<OpService<AddUserToGroup>>,
    pub create_instance_profile: Option<OpService<CreateInstanceProfile>>,
    pub get_open_id_connect_provider: Option<OpService<GetOpenIdConnectProvider>>,
    pub simulate_principal_policy: Option<OpService<SimulatePrincipalPolicy>>,
    pub list_server_certificates: Option<OpService<ListServerCertificates>>,
    pub untag_instance_profile: Option<OpService<UntagInstanceProfile>>,
    pub get_policy: Option<OpService<GetPolicy>>,
    pub list_policy_tags: Option<OpService<ListPolicyTags>>,
    pub list_saml_provider_tags: Option<OpService<ListSamlProviderTags>>,
    pub update_role_description: Option<OpService<UpdateRoleDescription>>,
    pub update_service_specific_credential: Option<OpService<UpdateServiceSpecificCredential>>,
    pub upload_ssh_public_key: Option<OpService<UploadSshPublicKey>>,
    pub delete_group_policy: Option<OpService<DeleteGroupPolicy>>,
    pub remove_client_id_from_open_id_connect_provider:
        Option<OpService<RemoveClientIdFromOpenIdConnectProvider>>,
    pub get_service_last_accessed_details_with_entities:
        Option<OpService<GetServiceLastAccessedDetailsWithEntities>>,
    pub put_group_policy: Option<OpService<PutGroupPolicy>>,
    pub list_group_policies: Option<OpService<ListGroupPolicies>>,
    pub list_server_certificate_tags: Option<OpService<ListServerCertificateTags>>,
    pub update_signing_certificate: Option<OpService<UpdateSigningCertificate>>,
    pub remove_role_from_instance_profile: Option<OpService<RemoveRoleFromInstanceProfile>>,
    pub detach_role_policy: Option<OpService<DetachRolePolicy>>,
    pub untag_policy: Option<OpService<UntagPolicy>>,
    pub put_role_policy: Option<OpService<PutRolePolicy>>,
    pub deactivate_mfa_device: Option<OpService<DeactivateMfaDevice>>,
    pub get_group_policy: Option<OpService<GetGroupPolicy>>,
    pub untag_mfa_device: Option<OpService<UntagMfaDevice>>,
    pub list_roles: Option<OpService<ListRoles>>,
    pub create_policy: Option<OpService<CreatePolicy>>,
    pub get_account_authorization_details: Option<OpService<GetAccountAuthorizationDetails>>,
    pub untag_user: Option<OpService<UntagUser>>,
    pub change_password: Option<OpService<ChangePassword>>,
    pub generate_credential_report: Option<OpService<GenerateCredentialReport>>,
    pub create_service_linked_role: Option<OpService<CreateServiceLinkedRole>>,
    pub get_context_keys_for_custom_policy: Option<OpService<GetContextKeysForCustomPolicy>>,
    pub update_open_id_connect_provider_thumbprint:
        Option<OpService<UpdateOpenIdConnectProviderThumbprint>>,
    pub get_group: Option<OpService<GetGroup>>,
    pub attach_group_policy: Option<OpService<AttachGroupPolicy>>,
    pub update_account_password_policy: Option<OpService<UpdateAccountPasswordPolicy>>,
    pub list_user_policies: Option<OpService<ListUserPolicies>>,
    pub tag_server_certificate: Option<OpService<TagServerCertificate>>,
    pub create_login_profile: Option<OpService<CreateLoginProfile>>,
    pub untag_saml_provider: Option<OpService<UntagSamlProvider>>,
    pub list_attached_role_policies: Option<OpService<ListAttachedRolePolicies>>,
    pub list_groups_for_user: Option<OpService<ListGroupsForUser>>,
    pub delete_signing_certificate: Option<OpService<DeleteSigningCertificate>>,
    pub delete_saml_provider: Option<OpService<DeleteSamlProvider>>,
    pub generate_organizations_access_report: Option<OpService<GenerateOrganizationsAccessReport>>,
    pub create_group: Option<OpService<CreateGroup>>,
    pub delete_access_key: Option<OpService<DeleteAccessKey>>,
    pub create_access_key: Option<OpService<CreateAccessKey>>,
    pub delete_user_policy: Option<OpService<DeleteUserPolicy>>,
    pub list_instance_profile_tags: Option<OpService<ListInstanceProfileTags>>,
    pub delete_user: Option<OpService<DeleteUser>>,
    pub list_policies: Option<OpService<ListPolicies>>,
    pub delete_account_alias: Option<OpService<DeleteAccountAlias>>,
    pub update_saml_provider: Option<OpService<UpdateSamlProvider>>,
    pub remove_user_from_group: Option<OpService<RemoveUserFromGroup>>,
    pub list_service_specific_credentials: Option<OpService<ListServiceSpecificCredentials>>,
    pub delete_ssh_public_key: Option<OpService<DeleteSshPublicKey>>,
    pub list_saml_providers: Option<OpService<ListSamlProviders>>,
    pub delete_service_specific_credential: Option<OpService<DeleteServiceSpecificCredential>>,
    pub simulate_custom_policy: Option<OpService<SimulateCustomPolicy>>,
    pub get_account_password_policy: Option<OpService<GetAccountPasswordPolicy>>,
    pub update_ssh_public_key: Option<OpService<UpdateSshPublicKey>>,
    pub delete_virtual_mfa_device: Option<OpService<DeleteVirtualMfaDevice>>,
    pub delete_policy_version: Option<OpService<DeletePolicyVersion>>,
    pub get_saml_provider: Option<OpService<GetSamlProvider>>,
    pub list_virtual_mfa_devices: Option<OpService<ListVirtualMfaDevices>>,
    pub create_role: Option<OpService<CreateRole>>,
    pub upload_signing_certificate: Option<OpService<UploadSigningCertificate>>,
    pub set_default_policy_version: Option<OpService<SetDefaultPolicyVersion>>,
    pub list_role_tags: Option<OpService<ListRoleTags>>,
    pub list_policy_versions: Option<OpService<ListPolicyVersions>>,
    pub put_user_permissions_boundary: Option<OpService<PutUserPermissionsBoundary>>,
    pub get_credential_report: Option<OpService<GetCredentialReport>>,
    pub create_user: Option<OpService<CreateUser>>,
    pub update_server_certificate: Option<OpService<UpdateServerCertificate>>,
    pub create_virtual_mfa_device: Option<OpService<CreateVirtualMfaDevice>>,
    pub list_instance_profiles: Option<OpService<ListInstanceProfiles>>,
    pub create_policy_version: Option<OpService<CreatePolicyVersion>>,
    pub delete_account_password_policy: Option<OpService<DeleteAccountPasswordPolicy>>,
    pub list_attached_user_policies: Option<OpService<ListAttachedUserPolicies>>,
    pub update_access_key: Option<OpService<UpdateAccessKey>>,
    pub list_ssh_public_keys: Option<OpService<ListSshPublicKeys>>,
    pub tag_policy: Option<OpService<TagPolicy>>,
    pub update_user: Option<OpService<UpdateUser>>,
    pub delete_role: Option<OpService<DeleteRole>>,
    pub tag_instance_profile: Option<OpService<TagInstanceProfile>>,
    pub delete_service_linked_role: Option<OpService<DeleteServiceLinkedRole>>,
    pub get_user: Option<OpService<GetUser>>,
    pub list_access_keys: Option<OpService<ListAccessKeys>>,
    pub enable_mfa_device: Option<OpService<EnableMfaDevice>>,
    pub tag_open_id_connect_provider: Option<OpService<TagOpenIdConnectProvider>>,
    pub list_account_aliases: Option<OpService<ListAccountAliases>>,
    pub get_instance_profile: Option<OpService<GetInstanceProfile>>,
    pub tag_role: Option<OpService<TagRole>>,
    pub get_role_policy: Option<OpService<GetRolePolicy>>,
    pub get_service_linked_role_deletion_status:
        Option<OpService<GetServiceLinkedRoleDeletionStatus>>,
    pub untag_role: Option<OpService<UntagRole>>,
    pub list_role_policies: Option<OpService<ListRolePolicies>>,
    pub resync_mfa_device: Option<OpService<ResyncMfaDevice>>,
    pub delete_login_profile: Option<OpService<DeleteLoginProfile>>,
    pub get_account_summary: Option<OpService<GetAccountSummary>>,
    pub list_instance_profiles_for_role: Option<OpService<ListInstanceProfilesForRole>>,
    pub list_open_id_connect_provider_tags: Option<OpService<ListOpenIdConnectProviderTags>>,
    pub delete_policy: Option<OpService<DeletePolicy>>,
    pub get_login_profile: Option<OpService<GetLoginProfile>>,
    pub list_open_id_connect_providers: Option<OpService<ListOpenIdConnectProviders>>,
    pub update_login_profile: Option<OpService<UpdateLoginProfile>>,
    pub untag_server_certificate: Option<OpService<UntagServerCertificate>>,
    pub delete_instance_profile: Option<OpService<DeleteInstanceProfile>>,
    pub get_ssh_public_key: Option<OpService<GetSshPublicKey>>,
    pub update_assume_role_policy: Option<OpService<UpdateAssumeRolePolicy>>,
    pub delete_role_permissions_boundary: Option<OpService<DeleteRolePermissionsBoundary>>,
    pub delete_server_certificate: Option<OpService<DeleteServerCertificate>>,
    pub attach_role_policy: Option<OpService<AttachRolePolicy>>,
    pub get_access_key_last_used: Option<OpService<GetAccessKeyLastUsed>>,
    pub get_context_keys_for_principal_policy: Option<OpService<GetContextKeysForPrincipalPolicy>>,
    pub get_service_last_accessed_details: Option<OpService<GetServiceLastAccessedDetails>>,
    pub detach_user_policy: Option<OpService<DetachUserPolicy>>,
    pub create_service_specific_credential: Option<OpService<CreateServiceSpecificCredential>>,
    pub reset_service_specific_credential: Option<OpService<ResetServiceSpecificCredential>>,
}

pub type ContextKeyNameType = String;

#[derive(Debug, Default, Clone)]
pub struct Statement {
    pub start_position: Option<Position>,
    pub source_policy_id: Option<PolicyIdentifierType>,
    pub end_position: Option<Position>,
    pub source_policy_type: Option<PolicySourceType>,
}
impl Statement {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct UploadSshPublicKeyResponse {
    pub ssh_public_key: Option<SshPublicKey>,
}
impl UploadSshPublicKeyResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetLoginProfileRequest {
    pub user_name: Option<userNameType>,
}
impl GetLoginProfileRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetServiceLastAccessedDetailsRequest {
    pub job_id: Option<jobIdType>,
    pub marker: Option<markerType>,
    pub max_items: Option<maxItemsType>,
}
impl GetServiceLastAccessedDetailsRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListVirtualMfaDevicesRequest {
    pub assignment_status: Option<assignmentStatusType>,
    pub max_items: Option<maxItemsType>,
    pub marker: Option<markerType>,
}
impl ListVirtualMfaDevicesRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetOpenIdConnectProviderResponse {
    pub url: Option<OpenIdConnectProviderUrlType>,
    pub create_date: Option<dateType>,
    pub tags: Option<tagListType>,
    pub thumbprint_list: Option<thumbprintListType>,
    pub client_id_list: Option<clientIdListType>,
}
impl GetOpenIdConnectProviderResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type privateKeyType = String;

#[derive(Debug, Default, Clone)]
pub struct ChangePassword;
impl OperationShape for ChangePassword {
    const NAME: &'static str = "ChangePassword";
    type Input = ChangePasswordRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct DeleteOpenIdConnectProviderRequest {
    pub open_id_connect_provider_arn: Option<arnType>,
}
impl DeleteOpenIdConnectProviderRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ErrorDetails {
    pub code: Option<stringType>,
    pub message: Option<stringType>,
}
impl ErrorDetails {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct UpdateSamlProviderResponse {
    pub saml_provider_arn: Option<arnType>,
}
impl UpdateSamlProviderResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type ContextKeyValueListType = Vec<ContextKeyValueType>;

#[derive(Debug, Default, Clone)]
pub struct LoginProfile {
    pub user_name: Option<userNameType>,
    pub create_date: Option<dateType>,
    pub password_reset_required: Option<booleanType>,
}
impl LoginProfile {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type SamlProviderListType = Vec<SamlProviderListEntry>;

#[derive(Debug, Default, Clone)]
pub struct DeleteUser;
impl OperationShape for DeleteUser {
    const NAME: &'static str = "DeleteUser";
    type Input = DeleteUserRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct ListPolicies;
impl OperationShape for ListPolicies {
    const NAME: &'static str = "ListPolicies";
    type Input = ListPoliciesRequest;
    type Output = ListPoliciesResponse;
    type Error = ();
}

pub type markerType = String;

#[derive(Debug, Default, Clone)]
pub struct ListAttachedUserPoliciesRequest {
    pub marker: Option<markerType>,
    pub user_name: Option<userNameType>,
    pub path_prefix: Option<policyPathType>,
    pub max_items: Option<maxItemsType>,
}
impl ListAttachedUserPoliciesRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type serialNumberType = String;

pub type serviceUserName = String;

pub type policyEvaluationErrorMessage = String;

#[derive(Debug, Default, Clone)]
pub struct ServiceNotSupportedException {
    pub message: Option<serviceNotSupportedMessage>,
}
impl ServiceNotSupportedException {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct DeleteSshPublicKeyRequest {
    pub user_name: Option<userNameType>,
    pub ssh_public_key_id: Option<publicKeyIdType>,
}
impl DeleteSshPublicKeyRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListServiceSpecificCredentialsResponse {
    pub service_specific_credentials: Option<ServiceSpecificCredentialsListType>,
}
impl ListServiceSpecificCredentialsResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListInstanceProfileTagsResponse {
    pub is_truncated: Option<booleanType>,
    pub marker: Option<responseMarkerType>,
    pub tags: Option<tagListType>,
}
impl ListInstanceProfileTagsResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GenerateOrganizationsAccessReportResponse {
    pub job_id: Option<jobIdType>,
}
impl GenerateOrganizationsAccessReportResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct UpdateRoleDescription;
impl OperationShape for UpdateRoleDescription {
    const NAME: &'static str = "UpdateRoleDescription";
    type Input = UpdateRoleDescriptionRequest;
    type Output = UpdateRoleDescriptionResponse;
    type Error = ();
}

pub type credentialReportExpiredExceptionMessage = String;

#[derive(Debug, Default, Clone)]
pub struct AddUserToGroup;
impl OperationShape for AddUserToGroup {
    const NAME: &'static str = "AddUserToGroup";
    type Input = AddUserToGroupRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct CreateUserResponse {
    pub user: Option<User>,
}
impl CreateUserResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListEntitiesForPolicy;
impl OperationShape for ListEntitiesForPolicy {
    const NAME: &'static str = "ListEntitiesForPolicy";
    type Input = ListEntitiesForPolicyRequest;
    type Output = ListEntitiesForPolicyResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct ListServerCertificateTagsRequest {
    pub marker: Option<markerType>,
    pub max_items: Option<maxItemsType>,
    pub server_certificate_name: Option<serverCertificateNameType>,
}
impl ListServerCertificateTagsRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type integerType = i32;

#[derive(Debug, Default, Clone)]
pub struct TagUserRequest {
    pub user_name: Option<existingUserNameType>,
    pub tags: Option<tagListType>,
}
impl TagUserRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct DuplicateCertificateException {
    pub message: Option<duplicateCertificateMessage>,
}
impl DuplicateCertificateException {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct AccessKeyLastUsed {
    pub region: Option<stringType>,
    pub last_used_date: Option<dateType>,
    pub service_name: Option<stringType>,
}
impl AccessKeyLastUsed {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListAccessKeysRequest {
    pub user_name: Option<existingUserNameType>,
    pub marker: Option<markerType>,
    pub max_items: Option<maxItemsType>,
}
impl ListAccessKeysRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type ContextEntryListType = Vec<ContextEntry>;

#[derive(Debug, Default, Clone)]
pub struct UserDetail {
    pub attached_managed_policies: Option<attachedPoliciesListType>,
    pub arn: Option<arnType>,
    pub create_date: Option<dateType>,
    pub group_list: Option<groupNameListType>,
    pub path: Option<pathType>,
    pub tags: Option<tagListType>,
    pub user_id: Option<idType>,
    pub user_name: Option<userNameType>,
    pub user_policy_list: Option<policyDetailListType>,
    pub permissions_boundary: Option<AttachedPermissionsBoundary>,
}
impl UserDetail {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListServiceSpecificCredentials;
impl OperationShape for ListServiceSpecificCredentials {
    const NAME: &'static str = "ListServiceSpecificCredentials";
    type Input = ListServiceSpecificCredentialsRequest;
    type Output = ListServiceSpecificCredentialsResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct AccessDetail {
    pub region: Option<stringType>,
    pub service_name: Option<serviceNameType>,
    pub service_namespace: Option<serviceNamespaceType>,
    pub last_authenticated_time: Option<dateType>,
    pub entity_path: Option<organizationsEntityPathType>,
    pub total_authenticated_entities: Option<integerType>,
}
impl AccessDetail {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct CreateAccessKey;
impl OperationShape for CreateAccessKey {
    const NAME: &'static str = "CreateAccessKey";
    type Input = CreateAccessKeyRequest;
    type Output = CreateAccessKeyResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GenerateServiceLastAccessedDetailsResponse {
    pub job_id: Option<jobIdType>,
}
impl GenerateServiceLastAccessedDetailsResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct DeleteUserPermissionsBoundary;
impl OperationShape for DeleteUserPermissionsBoundary {
    const NAME: &'static str = "DeleteUserPermissionsBoundary";
    type Input = DeleteUserPermissionsBoundaryRequest;
    type Output = ();
    type Error = ();
}

pub type arnType = String;

pub type organizationsPolicyIdType = String;

pub type policyListType = Vec<Policy>;

#[derive(Debug, Default, Clone)]
pub struct GetUserPolicyRequest {
    pub policy_name: Option<policyNameType>,
    pub user_name: Option<existingUserNameType>,
}
impl GetUserPolicyRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type EvalDecisionSourceType = String;

#[derive(Debug, Default, Clone)]
pub struct GetServerCertificateResponse {
    pub server_certificate: Option<ServerCertificate>,
}
impl GetServerCertificateResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListInstanceProfileTags;
impl OperationShape for ListInstanceProfileTags {
    const NAME: &'static str = "ListInstanceProfileTags";
    type Input = ListInstanceProfileTagsRequest;
    type Output = ListInstanceProfileTagsResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct AddRoleToInstanceProfileRequest {
    pub instance_profile_name: Option<instanceProfileNameType>,
    pub role_name: Option<roleNameType>,
}
impl AddRoleToInstanceProfileRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct PutUserPermissionsBoundary;
impl OperationShape for PutUserPermissionsBoundary {
    const NAME: &'static str = "PutUserPermissionsBoundary";
    type Input = PutUserPermissionsBoundaryRequest;
    type Output = ();
    type Error = ();
}

pub type passwordPolicyViolationMessage = String;

#[derive(Debug, Default, Clone)]
pub struct GetPolicyVersionResponse {
    pub policy_version: Option<PolicyVersion>,
}
impl GetPolicyVersionResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type virtualMfaDeviceName = String;

#[derive(Debug, Default, Clone)]
pub struct Role {
    pub tags: Option<tagListType>,
    pub max_session_duration: Option<roleMaxSessionDurationType>,
    pub description: Option<roleDescriptionType>,
    pub role_last_used: Option<RoleLastUsed>,
    pub create_date: Option<dateType>,
    pub arn: Option<arnType>,
    pub path: Option<pathType>,
    pub assume_role_policy_document: Option<policyDocumentType>,
    pub role_id: Option<idType>,
    pub role_name: Option<roleNameType>,
    pub permissions_boundary: Option<AttachedPermissionsBoundary>,
}
impl Role {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListServerCertificatesRequest {
    pub marker: Option<markerType>,
    pub path_prefix: Option<pathPrefixType>,
    pub max_items: Option<maxItemsType>,
}
impl ListServerCertificatesRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type serviceFailureExceptionMessage = String;

#[derive(Debug, Default, Clone)]
pub struct ListOpenIdConnectProviderTagsRequest {
    pub marker: Option<markerType>,
    pub max_items: Option<maxItemsType>,
    pub open_id_connect_provider_arn: Option<arnType>,
}
impl ListOpenIdConnectProviderTagsRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct CreateServiceSpecificCredentialRequest {
    pub user_name: Option<userNameType>,
    pub service_name: Option<serviceName>,
}
impl CreateServiceSpecificCredentialRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type policyDetailListType = Vec<PolicyDetail>;

#[derive(Debug, Default, Clone)]
pub struct DetachGroupPolicy;
impl OperationShape for DetachGroupPolicy {
    const NAME: &'static str = "DetachGroupPolicy";
    type Input = DetachGroupPolicyRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct TagMfaDeviceRequest {
    pub tags: Option<tagListType>,
    pub serial_number: Option<serialNumberType>,
}
impl TagMfaDeviceRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetInstanceProfileRequest {
    pub instance_profile_name: Option<instanceProfileNameType>,
}
impl GetInstanceProfileRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct UpdateSigningCertificateRequest {
    pub status: Option<statusType>,
    pub user_name: Option<existingUserNameType>,
    pub certificate_id: Option<certificateIdType>,
}
impl UpdateSigningCertificateRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct UntagUser;
impl OperationShape for UntagUser {
    const NAME: &'static str = "UntagUser";
    type Input = UntagUserRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetContextKeysForCustomPolicyRequest {
    pub policy_input_list: Option<SimulationPolicyListType>,
}
impl GetContextKeysForCustomPolicyRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type invalidUserTypeMessage = String;

#[derive(Debug, Default, Clone)]
pub struct ListOpenIdConnectProviders;
impl OperationShape for ListOpenIdConnectProviders {
    const NAME: &'static str = "ListOpenIdConnectProviders";
    type Input = ListOpenIdConnectProvidersRequest;
    type Output = ListOpenIdConnectProvidersResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct ListVirtualMfaDevicesResponse {
    pub virtual_mfa_devices: Option<virtualMfaDeviceListType>,
    pub is_truncated: Option<booleanType>,
    pub marker: Option<responseMarkerType>,
}
impl ListVirtualMfaDevicesResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct DeleteInstanceProfile;
impl OperationShape for DeleteInstanceProfile {
    const NAME: &'static str = "DeleteInstanceProfile";
    type Input = DeleteInstanceProfileRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct ListRoleTagsResponse {
    pub marker: Option<responseMarkerType>,
    pub is_truncated: Option<booleanType>,
    pub tags: Option<tagListType>,
}
impl ListRoleTagsResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type serviceNamespaceListType = Vec<serviceNamespaceType>;

#[derive(Debug, Default, Clone)]
pub struct DeleteVirtualMfaDevice;
impl OperationShape for DeleteVirtualMfaDevice {
    const NAME: &'static str = "DeleteVirtualMfaDevice";
    type Input = DeleteVirtualMfaDeviceRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetAccessKeyLastUsed;
impl OperationShape for GetAccessKeyLastUsed {
    const NAME: &'static str = "GetAccessKeyLastUsed";
    type Input = GetAccessKeyLastUsedRequest;
    type Output = GetAccessKeyLastUsedResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct DeleteRole;
impl OperationShape for DeleteRole {
    const NAME: &'static str = "DeleteRole";
    type Input = DeleteRoleRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct DeleteSigningCertificateRequest {
    pub certificate_id: Option<certificateIdType>,
    pub user_name: Option<existingUserNameType>,
}
impl DeleteSigningCertificateRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GroupDetail {
    pub create_date: Option<dateType>,
    pub group_name: Option<groupNameType>,
    pub path: Option<pathType>,
    pub attached_managed_policies: Option<attachedPoliciesListType>,
    pub group_policy_list: Option<policyDetailListType>,
    pub group_id: Option<idType>,
    pub arn: Option<arnType>,
}
impl GroupDetail {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct CreateRoleRequest {
    pub permissions_boundary: Option<arnType>,
    pub role_name: Option<roleNameType>,
    pub assume_role_policy_document: Option<policyDocumentType>,
    pub description: Option<roleDescriptionType>,
    pub max_session_duration: Option<roleMaxSessionDurationType>,
    pub tags: Option<tagListType>,
    pub path: Option<pathType>,
}
impl CreateRoleRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum ReportFormatType {
    text_csv,
}
impl AsRef<str> for ReportFormatType {
    fn as_ref(&self) -> &str {
        match self {
            Self::text_csv => "text/csv",
        }
    }
}
impl TryFrom<&str> for ReportFormatType {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "text/csv" => Ok(Self::text_csv),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

pub type accountAliasListType = Vec<accountAliasType>;

#[derive(Debug, Default, Clone)]
pub struct GetAccountPasswordPolicyResponse {
    pub password_policy: Option<PasswordPolicy>,
}
impl GetAccountPasswordPolicyResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GenerateCredentialReportResponse {
    pub description: Option<ReportStateDescriptionType>,
    pub state: Option<ReportStateType>,
}
impl GenerateCredentialReportResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum ContextKeyTypeEnum {
    IP_LIST,
    BOOLEAN,
    NUMERIC_LIST,
    STRING,
    STRING_LIST,
    NUMERIC,
    BINARY,
    BOOLEAN_LIST,
    DATE,
    DATE_LIST,
    BINARY_LIST,
    IP,
}
impl AsRef<str> for ContextKeyTypeEnum {
    fn as_ref(&self) -> &str {
        match self {
            Self::IP_LIST => "ipList",
            Self::BOOLEAN => "boolean",
            Self::NUMERIC_LIST => "numericList",
            Self::STRING => "string",
            Self::STRING_LIST => "stringList",
            Self::NUMERIC => "numeric",
            Self::BINARY => "binary",
            Self::BOOLEAN_LIST => "booleanList",
            Self::DATE => "date",
            Self::DATE_LIST => "dateList",
            Self::BINARY_LIST => "binaryList",
            Self::IP => "ip",
        }
    }
}
impl TryFrom<&str> for ContextKeyTypeEnum {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "ipList" => Ok(Self::IP_LIST),
            "boolean" => Ok(Self::BOOLEAN),
            "numericList" => Ok(Self::NUMERIC_LIST),
            "string" => Ok(Self::STRING),
            "stringList" => Ok(Self::STRING_LIST),
            "numeric" => Ok(Self::NUMERIC),
            "binary" => Ok(Self::BINARY),
            "booleanList" => Ok(Self::BOOLEAN_LIST),
            "date" => Ok(Self::DATE),
            "dateList" => Ok(Self::DATE_LIST),
            "binaryList" => Ok(Self::BINARY_LIST),
            "ip" => Ok(Self::IP),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

pub type ConcurrentModificationMessage = String;

#[derive(Debug, Default, Clone)]
pub struct DeleteGroupPolicyRequest {
    pub group_name: Option<groupNameType>,
    pub policy_name: Option<policyNameType>,
}
impl DeleteGroupPolicyRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct TagOpenIdConnectProvider;
impl OperationShape for TagOpenIdConnectProvider {
    const NAME: &'static str = "TagOpenIdConnectProvider";
    type Input = TagOpenIdConnectProviderRequest;
    type Output = ();
    type Error = ();
}

pub type virtualMfaDeviceListType = Vec<VirtualMfaDevice>;

#[derive(Debug, Default, Clone)]
pub struct DetachUserPolicyRequest {
    pub policy_arn: Option<arnType>,
    pub user_name: Option<userNameType>,
}
impl DetachUserPolicyRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListUsersRequest {
    pub marker: Option<markerType>,
    pub path_prefix: Option<pathPrefixType>,
    pub max_items: Option<maxItemsType>,
}
impl ListUsersRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct TagSamlProvider;
impl OperationShape for TagSamlProvider {
    const NAME: &'static str = "TagSamlProvider";
    type Input = TagSamlProviderRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct SimulateCustomPolicy;
impl OperationShape for SimulateCustomPolicy {
    const NAME: &'static str = "SimulateCustomPolicy";
    type Input = SimulateCustomPolicyRequest;
    type Output = SimulatePolicyResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct ListServerCertificates;
impl OperationShape for ListServerCertificates {
    const NAME: &'static str = "ListServerCertificates";
    type Input = ListServerCertificatesRequest;
    type Output = ListServerCertificatesResponse;
    type Error = ();
}

pub type ColumnNumber = i32;

#[derive(Debug, Default, Clone)]
pub struct CreateOpenIdConnectProvider;
impl OperationShape for CreateOpenIdConnectProvider {
    const NAME: &'static str = "CreateOpenIdConnectProvider";
    type Input = CreateOpenIdConnectProviderRequest;
    type Output = CreateOpenIdConnectProviderResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct EvaluationResult {
    pub resource_specific_results: Option<ResourceSpecificResultListType>,
    pub eval_resource_name: Option<ResourceNameType>,
    pub eval_decision: Option<PolicyEvaluationDecisionType>,
    pub organizations_decision_detail: Option<OrganizationsDecisionDetail>,
    pub matched_statements: Option<StatementListType>,
    pub permissions_boundary_decision_detail: Option<PermissionsBoundaryDecisionDetail>,
    pub missing_context_values: Option<ContextKeyNamesResultListType>,
    pub eval_action_name: Option<ActionNameType>,
    pub eval_decision_details: Option<EvalDecisionDetailsType>,
}
impl EvaluationResult {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetPolicyVersion;
impl OperationShape for GetPolicyVersion {
    const NAME: &'static str = "GetPolicyVersion";
    type Input = GetPolicyVersionRequest;
    type Output = GetPolicyVersionResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct DeleteSshPublicKey;
impl OperationShape for DeleteSshPublicKey {
    const NAME: &'static str = "DeleteSshPublicKey";
    type Input = DeleteSshPublicKeyRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct ListSshPublicKeys;
impl OperationShape for ListSshPublicKeys {
    const NAME: &'static str = "ListSshPublicKeys";
    type Input = ListSshPublicKeysRequest;
    type Output = ListSshPublicKeysResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct DeleteUserPermissionsBoundaryRequest {
    pub user_name: Option<userNameType>,
}
impl DeleteUserPermissionsBoundaryRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetCredentialReport;
impl OperationShape for GetCredentialReport {
    const NAME: &'static str = "GetCredentialReport";
    type Input = ();
    type Output = GetCredentialReportResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct CreateOpenIdConnectProviderRequest {
    pub client_id_list: Option<clientIdListType>,
    pub tags: Option<tagListType>,
    pub url: Option<OpenIdConnectProviderUrlType>,
    pub thumbprint_list: Option<thumbprintListType>,
}
impl CreateOpenIdConnectProviderRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetSamlProviderRequest {
    pub saml_provider_arn: Option<arnType>,
}
impl GetSamlProviderRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct TagMfaDevice;
impl OperationShape for TagMfaDevice {
    const NAME: &'static str = "TagMfaDevice";
    type Input = TagMfaDeviceRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct UntagInstanceProfile;
impl OperationShape for UntagInstanceProfile {
    const NAME: &'static str = "UntagInstanceProfile";
    type Input = UntagInstanceProfileRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct ListUserTagsRequest {
    pub max_items: Option<maxItemsType>,
    pub marker: Option<markerType>,
    pub user_name: Option<existingUserNameType>,
}
impl ListUserTagsRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct UntagMfaDevice;
impl OperationShape for UntagMfaDevice {
    const NAME: &'static str = "UntagMfaDevice";
    type Input = UntagMfaDeviceRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct UntagServerCertificateRequest {
    pub tag_keys: Option<tagKeyListType>,
    pub server_certificate_name: Option<serverCertificateNameType>,
}
impl UntagServerCertificateRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct CreateVirtualMfaDevice;
impl OperationShape for CreateVirtualMfaDevice {
    const NAME: &'static str = "CreateVirtualMfaDevice";
    type Input = CreateVirtualMfaDeviceRequest;
    type Output = CreateVirtualMfaDeviceResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct UpdateOpenIdConnectProviderThumbprint;
impl OperationShape for UpdateOpenIdConnectProviderThumbprint {
    const NAME: &'static str = "UpdateOpenIdConnectProviderThumbprint";
    type Input = UpdateOpenIdConnectProviderThumbprintRequest;
    type Output = ();
    type Error = ();
}

pub type booleanType = bool;

pub type AccessDetails = Vec<AccessDetail>;

#[derive(Debug, Default, Clone)]
pub struct ListPoliciesResponse {
    pub is_truncated: Option<booleanType>,
    pub policies: Option<policyListType>,
    pub marker: Option<responseMarkerType>,
}
impl ListPoliciesResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetSamlProviderResponse {
    pub create_date: Option<dateType>,
    pub valid_until: Option<dateType>,
    pub saml_metadata_document: Option<SamlMetadataDocumentType>,
    pub tags: Option<tagListType>,
}
impl GetSamlProviderResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type invalidInputMessage = String;

#[derive(Debug, Default, Clone)]
pub struct PutUserPolicy;
impl OperationShape for PutUserPolicy {
    const NAME: &'static str = "PutUserPolicy";
    type Input = PutUserPolicyRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum statusType {
    Inactive,
    Active,
}
impl AsRef<str> for statusType {
    fn as_ref(&self) -> &str {
        match self {
            Self::Inactive => "Inactive",
            Self::Active => "Active",
        }
    }
}
impl TryFrom<&str> for statusType {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "Inactive" => Ok(Self::Inactive),
            "Active" => Ok(Self::Active),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct UpdateGroupRequest {
    pub group_name: Option<groupNameType>,
    pub new_group_name: Option<groupNameType>,
    pub new_path: Option<pathType>,
}
impl UpdateGroupRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListSamlProvidersRequest {}
impl ListSamlProvidersRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type publicKeyFingerprintType = String;

#[derive(Debug, Default, Clone)]
pub struct GetGroupResponse {
    pub group: Option<Group>,
    pub users: Option<userListType>,
    pub is_truncated: Option<booleanType>,
    pub marker: Option<responseMarkerType>,
}
impl GetGroupResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type existingUserNameType = String;

#[derive(Debug, Default, Clone)]
pub struct TagUser;
impl OperationShape for TagUser {
    const NAME: &'static str = "TagUser";
    type Input = TagUserRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct OrganizationsDecisionDetail {
    pub allowed_by_organizations: Option<booleanType>,
}
impl OrganizationsDecisionDetail {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type certificateListType = Vec<SigningCertificate>;

#[derive(Debug, Default, Clone)]
pub struct GetCredentialReportResponse {
    pub content: Option<ReportContentType>,
    pub generated_time: Option<dateType>,
    pub report_format: Option<ReportFormatType>,
}
impl GetCredentialReportResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ConcurrentModificationException {
    pub message: Option<ConcurrentModificationMessage>,
}
impl ConcurrentModificationException {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListGroupsForUserResponse {
    pub groups: Option<groupListType>,
    pub is_truncated: Option<booleanType>,
    pub marker: Option<responseMarkerType>,
}
impl ListGroupsForUserResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct DeleteAccountPasswordPolicy;
impl OperationShape for DeleteAccountPasswordPolicy {
    const NAME: &'static str = "DeleteAccountPasswordPolicy";
    type Input = ();
    type Output = ();
    type Error = ();
}

pub type groupNameType = String;

#[derive(Debug, Default, Clone)]
pub struct GetRoleResponse {
    pub role: Option<Role>,
}
impl GetRoleResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum EntityType {
    AWSManagedPolicy,
    Group,
    LocalManagedPolicy,
    Role,
    User,
}
impl AsRef<str> for EntityType {
    fn as_ref(&self) -> &str {
        match self {
            Self::AWSManagedPolicy => "AWSManagedPolicy",
            Self::Group => "Group",
            Self::LocalManagedPolicy => "LocalManagedPolicy",
            Self::Role => "Role",
            Self::User => "User",
        }
    }
}
impl TryFrom<&str> for EntityType {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "AWSManagedPolicy" => Ok(Self::AWSManagedPolicy),
            "Group" => Ok(Self::Group),
            "LocalManagedPolicy" => Ok(Self::LocalManagedPolicy),
            "Role" => Ok(Self::Role),
            "User" => Ok(Self::User),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ResetServiceSpecificCredentialRequest {
    pub service_specific_credential_id: Option<serviceSpecificCredentialId>,
    pub user_name: Option<userNameType>,
}
impl ResetServiceSpecificCredentialRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetServiceLastAccessedDetails;
impl OperationShape for GetServiceLastAccessedDetails {
    const NAME: &'static str = "GetServiceLastAccessedDetails";
    type Input = GetServiceLastAccessedDetailsRequest;
    type Output = GetServiceLastAccessedDetailsResponse;
    type Error = ();
}

pub type deleteConflictMessage = String;

#[derive(Debug, Default, Clone)]
pub struct CreateGroupResponse {
    pub group: Option<Group>,
}
impl CreateGroupResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetUserRequest {
    pub user_name: Option<existingUserNameType>,
}
impl GetUserRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListGroupPoliciesRequest {
    pub group_name: Option<groupNameType>,
    pub marker: Option<markerType>,
    pub max_items: Option<maxItemsType>,
}
impl ListGroupPoliciesRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct UpdateServiceSpecificCredentialRequest {
    pub user_name: Option<userNameType>,
    pub service_specific_credential_id: Option<serviceSpecificCredentialId>,
    pub status: Option<statusType>,
}
impl UpdateServiceSpecificCredentialRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type certificateIdType = String;

#[derive(Debug, Default, Clone)]
pub struct GetRole;
impl OperationShape for GetRole {
    const NAME: &'static str = "GetRole";
    type Input = GetRoleRequest;
    type Output = GetRoleResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct ListInstanceProfilesForRole;
impl OperationShape for ListInstanceProfilesForRole {
    const NAME: &'static str = "ListInstanceProfilesForRole";
    type Input = ListInstanceProfilesForRoleRequest;
    type Output = ListInstanceProfilesForRoleResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct KeyPairMismatchException {
    pub message: Option<keyPairMismatchMessage>,
}
impl KeyPairMismatchException {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetContextKeysForPolicyResponse {
    pub context_key_names: Option<ContextKeyNamesResultListType>,
}
impl GetContextKeysForPolicyResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct PutUserPolicyRequest {
    pub policy_name: Option<policyNameType>,
    pub user_name: Option<existingUserNameType>,
    pub policy_document: Option<policyDocumentType>,
}
impl PutUserPolicyRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListServerCertificateTagsResponse {
    pub is_truncated: Option<booleanType>,
    pub tags: Option<tagListType>,
    pub marker: Option<responseMarkerType>,
}
impl ListServerCertificateTagsResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type attachmentCountType = i32;

#[derive(Debug, Default, Clone)]
pub struct CreatePolicyVersionRequest {
    pub policy_arn: Option<arnType>,
    pub policy_document: Option<policyDocumentType>,
    pub set_as_default: Option<booleanType>,
}
impl CreatePolicyVersionRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListOpenIdConnectProvidersResponse {
    pub open_id_connect_provider_list: Option<OpenIdConnectProviderListType>,
}
impl ListOpenIdConnectProvidersResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ResyncMfaDeviceRequest {
    pub authentication_code1: Option<authenticationCodeType>,
    pub authentication_code2: Option<authenticationCodeType>,
    pub serial_number: Option<serialNumberType>,
    pub user_name: Option<existingUserNameType>,
}
impl ResyncMfaDeviceRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ServerCertificateMetadata {
    pub arn: Option<arnType>,
    pub expiration: Option<dateType>,
    pub server_certificate_id: Option<idType>,
    pub upload_date: Option<dateType>,
    pub path: Option<pathType>,
    pub server_certificate_name: Option<serverCertificateNameType>,
}
impl ServerCertificateMetadata {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct InvalidInputException {
    pub message: Option<invalidInputMessage>,
}
impl InvalidInputException {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct PolicyDetail {
    pub policy_document: Option<policyDocumentType>,
    pub policy_name: Option<policyNameType>,
}
impl PolicyDetail {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListRolePolicies;
impl OperationShape for ListRolePolicies {
    const NAME: &'static str = "ListRolePolicies";
    type Input = ListRolePoliciesRequest;
    type Output = ListRolePoliciesResponse;
    type Error = ();
}

pub type SamlMetadataDocumentType = String;

#[derive(Debug, Default, Clone)]
pub struct DeleteServerCertificateRequest {
    pub server_certificate_name: Option<serverCertificateNameType>,
}
impl DeleteServerCertificateRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListGroupsRequest {
    pub path_prefix: Option<pathPrefixType>,
    pub max_items: Option<maxItemsType>,
    pub marker: Option<markerType>,
}
impl ListGroupsRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type booleanObjectType = bool;

pub type jobIdType = String;

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum ReportStateType {
    COMPLETE,
    STARTED,
    INPROGRESS,
}
impl AsRef<str> for ReportStateType {
    fn as_ref(&self) -> &str {
        match self {
            Self::COMPLETE => "COMPLETE",
            Self::STARTED => "STARTED",
            Self::INPROGRESS => "INPROGRESS",
        }
    }
}
impl TryFrom<&str> for ReportStateType {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "COMPLETE" => Ok(Self::COMPLETE),
            "STARTED" => Ok(Self::STARTED),
            "INPROGRESS" => Ok(Self::INPROGRESS),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct DeactivateMfaDevice;
impl OperationShape for DeactivateMfaDevice {
    const NAME: &'static str = "DeactivateMfaDevice";
    type Input = DeactivateMfaDeviceRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct UpdateSamlProviderRequest {
    pub saml_metadata_document: Option<SamlMetadataDocumentType>,
    pub saml_provider_arn: Option<arnType>,
}
impl UpdateSamlProviderRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct UploadSigningCertificateRequest {
    pub user_name: Option<existingUserNameType>,
    pub certificate_body: Option<certificateBodyType>,
}
impl UploadSigningCertificateRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetGroupPolicy;
impl OperationShape for GetGroupPolicy {
    const NAME: &'static str = "GetGroupPolicy";
    type Input = GetGroupPolicyRequest;
    type Output = GetGroupPolicyResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetServiceLastAccessedDetailsResponse {
    pub job_completion_date: Option<dateType>,
    pub services_last_accessed: Option<ServicesLastAccessed>,
    pub is_truncated: Option<booleanType>,
    pub job_creation_date: Option<dateType>,
    pub job_type: Option<AccessAdvisorUsageGranularityType>,
    pub job_status: Option<jobStatusType>,
    pub marker: Option<responseMarkerType>,
    pub error: Option<ErrorDetails>,
}
impl GetServiceLastAccessedDetailsResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct DeleteOpenIdConnectProvider;
impl OperationShape for DeleteOpenIdConnectProvider {
    const NAME: &'static str = "DeleteOpenIdConnectProvider";
    type Input = DeleteOpenIdConnectProviderRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct ListGroupPolicies;
impl OperationShape for ListGroupPolicies {
    const NAME: &'static str = "ListGroupPolicies";
    type Input = ListGroupPoliciesRequest;
    type Output = ListGroupPoliciesResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct UntagSamlProviderRequest {
    pub saml_provider_arn: Option<arnType>,
    pub tag_keys: Option<tagKeyListType>,
}
impl UntagSamlProviderRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListMfaDevicesResponse {
    pub is_truncated: Option<booleanType>,
    pub mfa_devices: Option<mfaDeviceListType>,
    pub marker: Option<responseMarkerType>,
}
impl ListMfaDevicesResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetOpenIdConnectProviderRequest {
    pub open_id_connect_provider_arn: Option<arnType>,
}
impl GetOpenIdConnectProviderRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct SetSecurityTokenServicePreferencesRequest {
    pub global_endpoint_token_version: Option<globalEndpointTokenVersion>,
}
impl SetSecurityTokenServicePreferencesRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type groupDetailListType = Vec<GroupDetail>;

pub type ResourceHandlingOptionType = String;

pub type instanceProfileNameType = String;

#[derive(Debug, Default, Clone)]
pub struct RemoveRoleFromInstanceProfileRequest {
    pub instance_profile_name: Option<instanceProfileNameType>,
    pub role_name: Option<roleNameType>,
}
impl RemoveRoleFromInstanceProfileRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type PolicyIdentifierType = String;

#[derive(Debug, Default, Clone)]
pub struct ListSigningCertificates;
impl OperationShape for ListSigningCertificates {
    const NAME: &'static str = "ListSigningCertificates";
    type Input = ListSigningCertificatesRequest;
    type Output = ListSigningCertificatesResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct ListAttachedGroupPolicies;
impl OperationShape for ListAttachedGroupPolicies {
    const NAME: &'static str = "ListAttachedGroupPolicies";
    type Input = ListAttachedGroupPoliciesRequest;
    type Output = ListAttachedGroupPoliciesResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct EnableMfaDevice;
impl OperationShape for EnableMfaDevice {
    const NAME: &'static str = "EnableMfaDevice";
    type Input = EnableMfaDeviceRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct ListVirtualMfaDevices;
impl OperationShape for ListVirtualMfaDevices {
    const NAME: &'static str = "ListVirtualMfaDevices";
    type Input = ListVirtualMfaDevicesRequest;
    type Output = ListVirtualMfaDevicesResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetGroupRequest {
    pub max_items: Option<maxItemsType>,
    pub group_name: Option<groupNameType>,
    pub marker: Option<markerType>,
}
impl GetGroupRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListAccessKeys;
impl OperationShape for ListAccessKeys {
    const NAME: &'static str = "ListAccessKeys";
    type Input = ListAccessKeysRequest;
    type Output = ListAccessKeysResponse;
    type Error = ();
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum PolicySourceType {
    NONE,
    RESOURCE,
    USER,
    USER_MANAGED,
    ROLE,
    AWS_MANAGED,
    GROUP,
}
impl AsRef<str> for PolicySourceType {
    fn as_ref(&self) -> &str {
        match self {
            Self::NONE => "none",
            Self::RESOURCE => "resource",
            Self::USER => "user",
            Self::USER_MANAGED => "user-managed",
            Self::ROLE => "role",
            Self::AWS_MANAGED => "aws-managed",
            Self::GROUP => "group",
        }
    }
}
impl TryFrom<&str> for PolicySourceType {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "none" => Ok(Self::NONE),
            "resource" => Ok(Self::RESOURCE),
            "user" => Ok(Self::USER),
            "user-managed" => Ok(Self::USER_MANAGED),
            "role" => Ok(Self::ROLE),
            "aws-managed" => Ok(Self::AWS_MANAGED),
            "group" => Ok(Self::GROUP),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct AttachUserPolicy;
impl OperationShape for AttachUserPolicy {
    const NAME: &'static str = "AttachUserPolicy";
    type Input = AttachUserPolicyRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct UpdateGroup;
impl OperationShape for UpdateGroup {
    const NAME: &'static str = "UpdateGroup";
    type Input = UpdateGroupRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetSshPublicKey;
impl OperationShape for GetSshPublicKey {
    const NAME: &'static str = "GetSshPublicKey";
    type Input = GetSshPublicKeyRequest;
    type Output = GetSshPublicKeyResponse;
    type Error = ();
}

pub type EvaluationResultsListType = Vec<EvaluationResult>;

#[derive(Debug, Default, Clone)]
pub struct CreateUserRequest {
    pub user_name: Option<userNameType>,
    pub tags: Option<tagListType>,
    pub permissions_boundary: Option<arnType>,
    pub path: Option<pathType>,
}
impl CreateUserRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct UpdateRoleResponse {}
impl UpdateRoleResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetServiceLinkedRoleDeletionStatus;
impl OperationShape for GetServiceLinkedRoleDeletionStatus {
    const NAME: &'static str = "GetServiceLinkedRoleDeletionStatus";
    type Input = GetServiceLinkedRoleDeletionStatusRequest;
    type Output = GetServiceLinkedRoleDeletionStatusResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetServerCertificate;
impl OperationShape for GetServerCertificate {
    const NAME: &'static str = "GetServerCertificate";
    type Input = GetServerCertificateRequest;
    type Output = GetServerCertificateResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct ListAttachedRolePoliciesRequest {
    pub role_name: Option<roleNameType>,
    pub max_items: Option<maxItemsType>,
    pub marker: Option<markerType>,
    pub path_prefix: Option<policyPathType>,
}
impl ListAttachedRolePoliciesRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct UpdateUserRequest {
    pub new_path: Option<pathType>,
    pub new_user_name: Option<userNameType>,
    pub user_name: Option<existingUserNameType>,
}
impl UpdateUserRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct DetachRolePolicy;
impl OperationShape for DetachRolePolicy {
    const NAME: &'static str = "DetachRolePolicy";
    type Input = DetachRolePolicyRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct PutRolePolicy;
impl OperationShape for PutRolePolicy {
    const NAME: &'static str = "PutRolePolicy";
    type Input = PutRolePolicyRequest;
    type Output = ();
    type Error = ();
}

pub type SimulationPolicyListType = Vec<policyDocumentType>;

#[derive(Debug, Default, Clone)]
pub struct AttachedPolicy {
    pub policy_arn: Option<arnType>,
    pub policy_name: Option<policyNameType>,
}
impl AttachedPolicy {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListGroups;
impl OperationShape for ListGroups {
    const NAME: &'static str = "ListGroups";
    type Input = ListGroupsRequest;
    type Output = ListGroupsResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct GetAccountAuthorizationDetailsResponse {
    pub user_detail_list: Option<userDetailListType>,
    pub is_truncated: Option<booleanType>,
    pub policies: Option<ManagedPolicyDetailListType>,
    pub marker: Option<responseMarkerType>,
    pub group_detail_list: Option<groupDetailListType>,
    pub role_detail_list: Option<roleDetailListType>,
}
impl GetAccountAuthorizationDetailsResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetUserPolicy;
impl OperationShape for GetUserPolicy {
    const NAME: &'static str = "GetUserPolicy";
    type Input = GetUserPolicyRequest;
    type Output = GetUserPolicyResponse;
    type Error = ();
}

pub type invalidCertificateMessage = String;

#[derive(Debug, Default, Clone)]
pub struct PolicyUser {
    pub user_id: Option<idType>,
    pub user_name: Option<userNameType>,
}
impl PolicyUser {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type publicKeyMaterialType = String;

#[derive(Debug, Default, Clone)]
pub struct EntityTemporarilyUnmodifiableException {
    pub message: Option<entityTemporarilyUnmodifiableMessage>,
}
impl EntityTemporarilyUnmodifiableException {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type DeletionTaskIdType = String;

#[derive(Debug, Default, Clone)]
pub struct DeleteVirtualMfaDeviceRequest {
    pub serial_number: Option<serialNumberType>,
}
impl DeleteVirtualMfaDeviceRequest {
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

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum AccessAdvisorUsageGranularityType {
    SERVICE_LEVEL,
    ACTION_LEVEL,
}
impl AsRef<str> for AccessAdvisorUsageGranularityType {
    fn as_ref(&self) -> &str {
        match self {
            Self::SERVICE_LEVEL => "SERVICE_LEVEL",
            Self::ACTION_LEVEL => "ACTION_LEVEL",
        }
    }
}
impl TryFrom<&str> for AccessAdvisorUsageGranularityType {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "SERVICE_LEVEL" => Ok(Self::SERVICE_LEVEL),
            "ACTION_LEVEL" => Ok(Self::ACTION_LEVEL),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

pub type PolicyGroupListType = Vec<PolicyGroup>;

#[derive(Debug, Default, Clone)]
pub struct TagRole;
impl OperationShape for TagRole {
    const NAME: &'static str = "TagRole";
    type Input = TagRoleRequest;
    type Output = ();
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct ListSamlProviderTags;
impl OperationShape for ListSamlProviderTags {
    const NAME: &'static str = "ListSamlProviderTags";
    type Input = ListSamlProviderTagsRequest;
    type Output = ListSamlProviderTagsResponse;
    type Error = ();
}

pub type OpenIdConnectProviderUrlType = String;

#[derive(Debug, Default, Clone)]
pub struct UpdateLoginProfile;
impl OperationShape for UpdateLoginProfile {
    const NAME: &'static str = "UpdateLoginProfile";
    type Input = UpdateLoginProfileRequest;
    type Output = ();
    type Error = ();
}

pub type accessKeySecretType = String;

#[derive(Debug, Default, Clone)]
pub struct PutRolePolicyRequest {
    pub policy_name: Option<policyNameType>,
    pub policy_document: Option<policyDocumentType>,
    pub role_name: Option<roleNameType>,
}
impl PutRolePolicyRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct UpdateRole;
impl OperationShape for UpdateRole {
    const NAME: &'static str = "UpdateRole";
    type Input = UpdateRoleRequest;
    type Output = UpdateRoleResponse;
    type Error = ();
}

pub type thumbprintListType = Vec<thumbprintType>;

#[derive(Debug, Default, Clone)]
pub struct CreateOpenIdConnectProviderResponse {
    pub tags: Option<tagListType>,
    pub open_id_connect_provider_arn: Option<arnType>,
}
impl CreateOpenIdConnectProviderResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct TagServerCertificateRequest {
    pub server_certificate_name: Option<serverCertificateNameType>,
    pub tags: Option<tagListType>,
}
impl TagServerCertificateRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type ArnListType = Vec<arnType>;

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum policyScopeType {
    Local,
    AWS,
    All,
}
impl AsRef<str> for policyScopeType {
    fn as_ref(&self) -> &str {
        match self {
            Self::Local => "Local",
            Self::AWS => "AWS",
            Self::All => "All",
        }
    }
}
impl TryFrom<&str> for policyScopeType {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "Local" => Ok(Self::Local),
            "AWS" => Ok(Self::AWS),
            "All" => Ok(Self::All),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

pub type entityNameType = String;

#[derive(Debug, Default, Clone)]
pub struct SetDefaultPolicyVersionRequest {
    pub version_id: Option<policyVersionIdType>,
    pub policy_arn: Option<arnType>,
}
impl SetDefaultPolicyVersionRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListPolicyVersionsResponse {
    pub versions: Option<policyDocumentVersionListType>,
    pub is_truncated: Option<booleanType>,
    pub marker: Option<responseMarkerType>,
}
impl ListPolicyVersionsResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListRolesRequest {
    pub path_prefix: Option<pathPrefixType>,
    pub marker: Option<markerType>,
    pub max_items: Option<maxItemsType>,
}
impl ListRolesRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GetAccountPasswordPolicy;
impl OperationShape for GetAccountPasswordPolicy {
    const NAME: &'static str = "GetAccountPasswordPolicy";
    type Input = ();
    type Output = GetAccountPasswordPolicyResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct UpdateLoginProfileRequest {
    pub password: Option<passwordType>,
    pub password_reset_required: Option<booleanObjectType>,
    pub user_name: Option<userNameType>,
}
impl UpdateLoginProfileRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type authenticationCodeType = String;

pub type policyNameListType = Vec<policyNameType>;

#[derive(Debug, Default, Clone)]
pub struct InstanceProfile {
    pub instance_profile_id: Option<idType>,
    pub arn: Option<arnType>,
    pub path: Option<pathType>,
    pub create_date: Option<dateType>,
    pub roles: Option<roleListType>,
    pub tags: Option<tagListType>,
    pub instance_profile_name: Option<instanceProfileNameType>,
}
impl InstanceProfile {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListAttachedRolePolicies;
impl OperationShape for ListAttachedRolePolicies {
    const NAME: &'static str = "ListAttachedRolePolicies";
    type Input = ListAttachedRolePoliciesRequest;
    type Output = ListAttachedRolePoliciesResponse;
    type Error = ();
}

#[derive(Debug, Default, Clone)]
pub struct CreateInstanceProfile;
impl OperationShape for CreateInstanceProfile {
    const NAME: &'static str = "CreateInstanceProfile";
    type Input = CreateInstanceProfileRequest;
    type Output = CreateInstanceProfileResponse;
    type Error = ();
}

pub type clientIdType = String;

#[derive(Debug, Default, Clone)]
pub struct InvalidAuthenticationCodeException {
    pub message: Option<invalidAuthenticationCodeMessage>,
}
impl InvalidAuthenticationCodeException {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct DeleteRolePolicyRequest {
    pub role_name: Option<roleNameType>,
    pub policy_name: Option<policyNameType>,
}
impl DeleteRolePolicyRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type pathType = String;

#[derive(Debug, Default, Clone)]
pub struct ListInstanceProfilesResponse {
    pub marker: Option<responseMarkerType>,
    pub instance_profiles: Option<instanceProfileListType>,
    pub is_truncated: Option<booleanType>,
}
impl ListInstanceProfilesResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type tagKeyListType = Vec<tagKeyType>;

#[derive(Debug, Default, Clone)]
pub struct GetAccountSummaryResponse {
    pub summary_map: Option<summaryMapType>,
}
impl GetAccountSummaryResponse {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type customSuffixType = String;

#[derive(Debug, Default, Clone)]
pub struct AddClientIdToOpenIdConnectProviderRequest {
    pub open_id_connect_provider_arn: Option<arnType>,
    pub client_id: Option<clientIdType>,
}
impl AddClientIdToOpenIdConnectProviderRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListRoleTags;
impl OperationShape for ListRoleTags {
    const NAME: &'static str = "ListRoleTags";
    type Input = ListRoleTagsRequest;
    type Output = ListRoleTagsResponse;
    type Error = ();
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum PolicyEvaluationDecisionType {
    EXPLICIT_DENY,
    IMPLICIT_DENY,
    ALLOWED,
}
impl AsRef<str> for PolicyEvaluationDecisionType {
    fn as_ref(&self) -> &str {
        match self {
            Self::EXPLICIT_DENY => "explicitDeny",
            Self::IMPLICIT_DENY => "implicitDeny",
            Self::ALLOWED => "allowed",
        }
    }
}
impl TryFrom<&str> for PolicyEvaluationDecisionType {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "explicitDeny" => Ok(Self::EXPLICIT_DENY),
            "implicitDeny" => Ok(Self::IMPLICIT_DENY),
            "allowed" => Ok(Self::ALLOWED),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct UpdateRoleDescriptionRequest {
    pub description: Option<roleDescriptionType>,
    pub role_name: Option<roleNameType>,
}
impl UpdateRoleDescriptionRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

pub type credentialReportNotReadyExceptionMessage = String;

#[derive(Debug, Default, Clone)]
pub struct ListInstanceProfilesForRoleRequest {
    pub role_name: Option<roleNameType>,
    pub marker: Option<markerType>,
    pub max_items: Option<maxItemsType>,
}
impl ListInstanceProfilesForRoleRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct DeletePolicyVersion;
impl OperationShape for DeletePolicyVersion {
    const NAME: &'static str = "DeletePolicyVersion";
    type Input = DeletePolicyVersionRequest;
    type Output = ();
    type Error = ();
}

pub type keyPairMismatchMessage = String;

#[derive(Debug, Default, Clone)]
pub struct ServiceLastAccessed {
    pub last_authenticated: Option<dateType>,
    pub last_authenticated_region: Option<stringType>,
    pub service_name: Option<serviceNameType>,
    pub total_authenticated_entities: Option<integerType>,
    pub last_authenticated_entity: Option<arnType>,
    pub service_namespace: Option<serviceNamespaceType>,
    pub tracked_actions_last_accessed: Option<TrackedActionsLastAccessed>,
}
impl ServiceLastAccessed {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct PutUserPermissionsBoundaryRequest {
    pub permissions_boundary: Option<arnType>,
    pub user_name: Option<userNameType>,
}
impl PutUserPermissionsBoundaryRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Default, Clone)]
pub struct CredentialReportNotPresentException {
    pub message: Option<credentialReportNotPresentExceptionMessage>,
}
impl CredentialReportNotPresentException {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum sortKeyType {
    LAST_AUTHENTICATED_TIME_ASCENDING,
    LAST_AUTHENTICATED_TIME_DESCENDING,
    SERVICE_NAMESPACE_DESCENDING,
    SERVICE_NAMESPACE_ASCENDING,
}
impl AsRef<str> for sortKeyType {
    fn as_ref(&self) -> &str {
        match self {
            Self::LAST_AUTHENTICATED_TIME_ASCENDING => "LAST_AUTHENTICATED_TIME_ASCENDING",
            Self::LAST_AUTHENTICATED_TIME_DESCENDING => "LAST_AUTHENTICATED_TIME_DESCENDING",
            Self::SERVICE_NAMESPACE_DESCENDING => "SERVICE_NAMESPACE_DESCENDING",
            Self::SERVICE_NAMESPACE_ASCENDING => "SERVICE_NAMESPACE_ASCENDING",
        }
    }
}
impl TryFrom<&str> for sortKeyType {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "LAST_AUTHENTICATED_TIME_ASCENDING" => Ok(Self::LAST_AUTHENTICATED_TIME_ASCENDING),
            "LAST_AUTHENTICATED_TIME_DESCENDING" => Ok(Self::LAST_AUTHENTICATED_TIME_DESCENDING),
            "SERVICE_NAMESPACE_DESCENDING" => Ok(Self::SERVICE_NAMESPACE_DESCENDING),
            "SERVICE_NAMESPACE_ASCENDING" => Ok(Self::SERVICE_NAMESPACE_ASCENDING),
            _ => Err(format!("unknown enum value {}", s)),
        }
    }
}

pub type policyPathType = String;

pub type unmodifiableEntityMessage = String;

pub type roleNameType = String;

#[derive(Debug, Default, Clone)]
pub struct CreateServiceLinkedRoleRequest {
    pub custom_suffix: Option<customSuffixType>,
    pub aws_service_name: Option<groupNameType>,
    pub description: Option<roleDescriptionType>,
}
impl CreateServiceLinkedRoleRequest {
    pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
        anyhow::bail!("todo")
    }
}
