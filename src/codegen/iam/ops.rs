#![allow(unused)]
#![allow(non_camel_case_types)]

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Ops {
    RemoveRoleFromInstanceProfile,
    SimulatePrincipalPolicy,
    UntagOpenIdConnectProvider,
    CreateLoginProfile,
    ListInstanceProfileTags,
    ListOpenIdConnectProviderTags,
    DeletePolicyVersion,
    ListAttachedRolePolicies,
    GetAccountAuthorizationDetails,
    DeleteSigningCertificate,
    AddRoleToInstanceProfile,
    CreateAccessKey,
    ListGroupsForUser,
    DeleteVirtualMfaDevice,
    GetAccessKeyLastUsed,
    UploadServerCertificate,
    GetRolePolicy,
    UpdateUser,
    DeleteUser,
    ListServiceSpecificCredentials,
    DeleteServiceLinkedRole,
    UpdateSigningCertificate,
    GetGroupPolicy,
    UntagUser,
    DeleteAccountPasswordPolicy,
    DeleteLoginProfile,
    DeleteServerCertificate,
    DeleteUserPermissionsBoundary,
    SetSecurityTokenServicePreferences,
    TagUser,
    ResetServiceSpecificCredential,
    GetPolicyVersion,
    TagSamlProvider,
    CreateGroup,
    AttachGroupPolicy,
    ListAttachedGroupPolicies,
    UpdateGroup,
    CreateOpenIdConnectProvider,
    GetServerCertificate,
    CreatePolicyVersion,
    GetAccountSummary,
    UpdateServiceSpecificCredential,
    DeleteRolePermissionsBoundary,
    CreateServiceLinkedRole,
    UntagPolicy,
    UpdateAccountPasswordPolicy,
    RemoveClientIdFromOpenIdConnectProvider,
    ListSamlProviderTags,
    UpdateAssumeRolePolicy,
    GetSamlProvider,
    CreateInstanceProfile,
    GetServiceLastAccessedDetailsWithEntities,
    GetUser,
    ListAccessKeys,
    CreateServiceSpecificCredential,
    EnableMfaDevice,
    UploadSigningCertificate,
    DeleteAccountAlias,
    TagInstanceProfile,
    ListGroupPolicies,
    ChangePassword,
    ListInstanceProfilesForRole,
    ListUsers,
    GetSshPublicKey,
    GenerateServiceLastAccessedDetails,
    ListOpenIdConnectProviders,
    UpdateRole,
    ListPolicyVersions,
    GetInstanceProfile,
    UntagSamlProvider,
    CreateRole,
    DetachGroupPolicy,
    PutRolePolicy,
    ListAttachedUserPolicies,
    DetachUserPolicy,
    ListPolicies,
    ListMfaDeviceTags,
    UpdateServerCertificate,
    TagRole,
    DeleteGroupPolicy,
    UpdateSshPublicKey,
    ListServerCertificateTags,
    GenerateOrganizationsAccessReport,
    DeleteInstanceProfile,
    DeleteSshPublicKey,
    TagOpenIdConnectProvider,
    SimulateCustomPolicy,
    PutUserPolicy,
    GetOrganizationsAccessReport,
    GetOpenIdConnectProvider,
    UpdateAccessKey,
    GetAccountPasswordPolicy,
    UploadSshPublicKey,
    CreatePolicy,
    ListEntitiesForPolicy,
    UntagInstanceProfile,
    UntagRole,
    ListPoliciesGrantingServiceAccess,
    GetUserPolicy,
    PutGroupPolicy,
    GetRole,
    UntagMfaDevice,
    UntagServerCertificate,
    ListSamlProviders,
    DeleteServiceSpecificCredential,
    DetachRolePolicy,
    UpdateOpenIdConnectProviderThumbprint,
    GetGroup,
    GetContextKeysForPrincipalPolicy,
    TagServerCertificate,
    GetLoginProfile,
    ListRoleTags,
    UpdateRoleDescription,
    CreateSamlProvider,
    CreateAccountAlias,
    AddClientIdToOpenIdConnectProvider,
    DeleteUserPolicy,
    DeleteSamlProvider,
    ResyncMfaDevice,
    ListInstanceProfiles,
    ListUserTags,
    ListPolicyTags,
    ListVirtualMfaDevices,
    ListGroups,
    GetContextKeysForCustomPolicy,
    TagPolicy,
    AttachUserPolicy,
    ListRolePolicies,
    PutRolePermissionsBoundary,
    ListSshPublicKeys,
    CreateUser,
    ListMfaDevices,
    RemoveUserFromGroup,
    DeleteGroup,
    GetCredentialReport,
    SetDefaultPolicyVersion,
    DeleteRole,
    DeletePolicy,
    CreateVirtualMfaDevice,
    ListAccountAliases,
    GetServiceLinkedRoleDeletionStatus,
    ListServerCertificates,
    ListRoles,
    GetServiceLastAccessedDetails,
    UpdateLoginProfile,
    ListSigningCertificates,
    PutUserPermissionsBoundary,
    DeleteAccessKey,
    UpdateSamlProvider,
    AttachRolePolicy,
    AddUserToGroup,
    TagMfaDevice,
    GetPolicy,
    DeactivateMfaDevice,
    DeleteOpenIdConnectProvider,
    ListUserPolicies,
    DeleteRolePolicy,
    GenerateCredentialReport,
}
#[doc = r" This macro calls a provided $macro for each operation to generate code per op."]
macro_rules! generate_ops_code {
    ($ macro : ident) => {
        $macro!(RemoveRoleFromInstanceProfile);
        $macro!(SimulatePrincipalPolicy);
        $macro!(UntagOpenIdConnectProvider);
        $macro!(CreateLoginProfile);
        $macro!(ListInstanceProfileTags);
        $macro!(ListOpenIdConnectProviderTags);
        $macro!(DeletePolicyVersion);
        $macro!(ListAttachedRolePolicies);
        $macro!(GetAccountAuthorizationDetails);
        $macro!(DeleteSigningCertificate);
        $macro!(AddRoleToInstanceProfile);
        $macro!(CreateAccessKey);
        $macro!(ListGroupsForUser);
        $macro!(DeleteVirtualMfaDevice);
        $macro!(GetAccessKeyLastUsed);
        $macro!(UploadServerCertificate);
        $macro!(GetRolePolicy);
        $macro!(UpdateUser);
        $macro!(DeleteUser);
        $macro!(ListServiceSpecificCredentials);
        $macro!(DeleteServiceLinkedRole);
        $macro!(UpdateSigningCertificate);
        $macro!(GetGroupPolicy);
        $macro!(UntagUser);
        $macro!(DeleteAccountPasswordPolicy);
        $macro!(DeleteLoginProfile);
        $macro!(DeleteServerCertificate);
        $macro!(DeleteUserPermissionsBoundary);
        $macro!(SetSecurityTokenServicePreferences);
        $macro!(TagUser);
        $macro!(ResetServiceSpecificCredential);
        $macro!(GetPolicyVersion);
        $macro!(TagSamlProvider);
        $macro!(CreateGroup);
        $macro!(AttachGroupPolicy);
        $macro!(ListAttachedGroupPolicies);
        $macro!(UpdateGroup);
        $macro!(CreateOpenIdConnectProvider);
        $macro!(GetServerCertificate);
        $macro!(CreatePolicyVersion);
        $macro!(GetAccountSummary);
        $macro!(UpdateServiceSpecificCredential);
        $macro!(DeleteRolePermissionsBoundary);
        $macro!(CreateServiceLinkedRole);
        $macro!(UntagPolicy);
        $macro!(UpdateAccountPasswordPolicy);
        $macro!(RemoveClientIdFromOpenIdConnectProvider);
        $macro!(ListSamlProviderTags);
        $macro!(UpdateAssumeRolePolicy);
        $macro!(GetSamlProvider);
        $macro!(CreateInstanceProfile);
        $macro!(GetServiceLastAccessedDetailsWithEntities);
        $macro!(GetUser);
        $macro!(ListAccessKeys);
        $macro!(CreateServiceSpecificCredential);
        $macro!(EnableMfaDevice);
        $macro!(UploadSigningCertificate);
        $macro!(DeleteAccountAlias);
        $macro!(TagInstanceProfile);
        $macro!(ListGroupPolicies);
        $macro!(ChangePassword);
        $macro!(ListInstanceProfilesForRole);
        $macro!(ListUsers);
        $macro!(GetSshPublicKey);
        $macro!(GenerateServiceLastAccessedDetails);
        $macro!(ListOpenIdConnectProviders);
        $macro!(UpdateRole);
        $macro!(ListPolicyVersions);
        $macro!(GetInstanceProfile);
        $macro!(UntagSamlProvider);
        $macro!(CreateRole);
        $macro!(DetachGroupPolicy);
        $macro!(PutRolePolicy);
        $macro!(ListAttachedUserPolicies);
        $macro!(DetachUserPolicy);
        $macro!(ListPolicies);
        $macro!(ListMfaDeviceTags);
        $macro!(UpdateServerCertificate);
        $macro!(TagRole);
        $macro!(DeleteGroupPolicy);
        $macro!(UpdateSshPublicKey);
        $macro!(ListServerCertificateTags);
        $macro!(GenerateOrganizationsAccessReport);
        $macro!(DeleteInstanceProfile);
        $macro!(DeleteSshPublicKey);
        $macro!(TagOpenIdConnectProvider);
        $macro!(SimulateCustomPolicy);
        $macro!(PutUserPolicy);
        $macro!(GetOrganizationsAccessReport);
        $macro!(GetOpenIdConnectProvider);
        $macro!(UpdateAccessKey);
        $macro!(GetAccountPasswordPolicy);
        $macro!(UploadSshPublicKey);
        $macro!(CreatePolicy);
        $macro!(ListEntitiesForPolicy);
        $macro!(UntagInstanceProfile);
        $macro!(UntagRole);
        $macro!(ListPoliciesGrantingServiceAccess);
        $macro!(GetUserPolicy);
        $macro!(PutGroupPolicy);
        $macro!(GetRole);
        $macro!(UntagMfaDevice);
        $macro!(UntagServerCertificate);
        $macro!(ListSamlProviders);
        $macro!(DeleteServiceSpecificCredential);
        $macro!(DetachRolePolicy);
        $macro!(UpdateOpenIdConnectProviderThumbprint);
        $macro!(GetGroup);
        $macro!(GetContextKeysForPrincipalPolicy);
        $macro!(TagServerCertificate);
        $macro!(GetLoginProfile);
        $macro!(ListRoleTags);
        $macro!(UpdateRoleDescription);
        $macro!(CreateSamlProvider);
        $macro!(CreateAccountAlias);
        $macro!(AddClientIdToOpenIdConnectProvider);
        $macro!(DeleteUserPolicy);
        $macro!(DeleteSamlProvider);
        $macro!(ResyncMfaDevice);
        $macro!(ListInstanceProfiles);
        $macro!(ListUserTags);
        $macro!(ListPolicyTags);
        $macro!(ListVirtualMfaDevices);
        $macro!(ListGroups);
        $macro!(GetContextKeysForCustomPolicy);
        $macro!(TagPolicy);
        $macro!(AttachUserPolicy);
        $macro!(ListRolePolicies);
        $macro!(PutRolePermissionsBoundary);
        $macro!(ListSshPublicKeys);
        $macro!(CreateUser);
        $macro!(ListMfaDevices);
        $macro!(RemoveUserFromGroup);
        $macro!(DeleteGroup);
        $macro!(GetCredentialReport);
        $macro!(SetDefaultPolicyVersion);
        $macro!(DeleteRole);
        $macro!(DeletePolicy);
        $macro!(CreateVirtualMfaDevice);
        $macro!(ListAccountAliases);
        $macro!(GetServiceLinkedRoleDeletionStatus);
        $macro!(ListServerCertificates);
        $macro!(ListRoles);
        $macro!(GetServiceLastAccessedDetails);
        $macro!(UpdateLoginProfile);
        $macro!(ListSigningCertificates);
        $macro!(PutUserPermissionsBoundary);
        $macro!(DeleteAccessKey);
        $macro!(UpdateSamlProvider);
        $macro!(AttachRolePolicy);
        $macro!(AddUserToGroup);
        $macro!(TagMfaDevice);
        $macro!(GetPolicy);
        $macro!(DeactivateMfaDevice);
        $macro!(DeleteOpenIdConnectProvider);
        $macro!(ListUserPolicies);
        $macro!(DeleteRolePolicy);
        $macro!(GenerateCredentialReport);
    };
}
#[doc = r" This macro calls a provided $macro for each operation to generate item per op."]
macro_rules ! generate_ops_items { ($ macro : ident) => { $ macro ! (RemoveRoleFromInstanceProfile) , $ macro ! (SimulatePrincipalPolicy) , $ macro ! (UntagOpenIdConnectProvider) , $ macro ! (CreateLoginProfile) , $ macro ! (ListInstanceProfileTags) , $ macro ! (ListOpenIdConnectProviderTags) , $ macro ! (DeletePolicyVersion) , $ macro ! (ListAttachedRolePolicies) , $ macro ! (GetAccountAuthorizationDetails) , $ macro ! (DeleteSigningCertificate) , $ macro ! (AddRoleToInstanceProfile) , $ macro ! (CreateAccessKey) , $ macro ! (ListGroupsForUser) , $ macro ! (DeleteVirtualMfaDevice) , $ macro ! (GetAccessKeyLastUsed) , $ macro ! (UploadServerCertificate) , $ macro ! (GetRolePolicy) , $ macro ! (UpdateUser) , $ macro ! (DeleteUser) , $ macro ! (ListServiceSpecificCredentials) , $ macro ! (DeleteServiceLinkedRole) , $ macro ! (UpdateSigningCertificate) , $ macro ! (GetGroupPolicy) , $ macro ! (UntagUser) , $ macro ! (DeleteAccountPasswordPolicy) , $ macro ! (DeleteLoginProfile) , $ macro ! (DeleteServerCertificate) , $ macro ! (DeleteUserPermissionsBoundary) , $ macro ! (SetSecurityTokenServicePreferences) , $ macro ! (TagUser) , $ macro ! (ResetServiceSpecificCredential) , $ macro ! (GetPolicyVersion) , $ macro ! (TagSamlProvider) , $ macro ! (CreateGroup) , $ macro ! (AttachGroupPolicy) , $ macro ! (ListAttachedGroupPolicies) , $ macro ! (UpdateGroup) , $ macro ! (CreateOpenIdConnectProvider) , $ macro ! (GetServerCertificate) , $ macro ! (CreatePolicyVersion) , $ macro ! (GetAccountSummary) , $ macro ! (UpdateServiceSpecificCredential) , $ macro ! (DeleteRolePermissionsBoundary) , $ macro ! (CreateServiceLinkedRole) , $ macro ! (UntagPolicy) , $ macro ! (UpdateAccountPasswordPolicy) , $ macro ! (RemoveClientIdFromOpenIdConnectProvider) , $ macro ! (ListSamlProviderTags) , $ macro ! (UpdateAssumeRolePolicy) , $ macro ! (GetSamlProvider) , $ macro ! (CreateInstanceProfile) , $ macro ! (GetServiceLastAccessedDetailsWithEntities) , $ macro ! (GetUser) , $ macro ! (ListAccessKeys) , $ macro ! (CreateServiceSpecificCredential) , $ macro ! (EnableMfaDevice) , $ macro ! (UploadSigningCertificate) , $ macro ! (DeleteAccountAlias) , $ macro ! (TagInstanceProfile) , $ macro ! (ListGroupPolicies) , $ macro ! (ChangePassword) , $ macro ! (ListInstanceProfilesForRole) , $ macro ! (ListUsers) , $ macro ! (GetSshPublicKey) , $ macro ! (GenerateServiceLastAccessedDetails) , $ macro ! (ListOpenIdConnectProviders) , $ macro ! (UpdateRole) , $ macro ! (ListPolicyVersions) , $ macro ! (GetInstanceProfile) , $ macro ! (UntagSamlProvider) , $ macro ! (CreateRole) , $ macro ! (DetachGroupPolicy) , $ macro ! (PutRolePolicy) , $ macro ! (ListAttachedUserPolicies) , $ macro ! (DetachUserPolicy) , $ macro ! (ListPolicies) , $ macro ! (ListMfaDeviceTags) , $ macro ! (UpdateServerCertificate) , $ macro ! (TagRole) , $ macro ! (DeleteGroupPolicy) , $ macro ! (UpdateSshPublicKey) , $ macro ! (ListServerCertificateTags) , $ macro ! (GenerateOrganizationsAccessReport) , $ macro ! (DeleteInstanceProfile) , $ macro ! (DeleteSshPublicKey) , $ macro ! (TagOpenIdConnectProvider) , $ macro ! (SimulateCustomPolicy) , $ macro ! (PutUserPolicy) , $ macro ! (GetOrganizationsAccessReport) , $ macro ! (GetOpenIdConnectProvider) , $ macro ! (UpdateAccessKey) , $ macro ! (GetAccountPasswordPolicy) , $ macro ! (UploadSshPublicKey) , $ macro ! (CreatePolicy) , $ macro ! (ListEntitiesForPolicy) , $ macro ! (UntagInstanceProfile) , $ macro ! (UntagRole) , $ macro ! (ListPoliciesGrantingServiceAccess) , $ macro ! (GetUserPolicy) , $ macro ! (PutGroupPolicy) , $ macro ! (GetRole) , $ macro ! (UntagMfaDevice) , $ macro ! (UntagServerCertificate) , $ macro ! (ListSamlProviders) , $ macro ! (DeleteServiceSpecificCredential) , $ macro ! (DetachRolePolicy) , $ macro ! (UpdateOpenIdConnectProviderThumbprint) , $ macro ! (GetGroup) , $ macro ! (GetContextKeysForPrincipalPolicy) , $ macro ! (TagServerCertificate) , $ macro ! (GetLoginProfile) , $ macro ! (ListRoleTags) , $ macro ! (UpdateRoleDescription) , $ macro ! (CreateSamlProvider) , $ macro ! (CreateAccountAlias) , $ macro ! (AddClientIdToOpenIdConnectProvider) , $ macro ! (DeleteUserPolicy) , $ macro ! (DeleteSamlProvider) , $ macro ! (ResyncMfaDevice) , $ macro ! (ListInstanceProfiles) , $ macro ! (ListUserTags) , $ macro ! (ListPolicyTags) , $ macro ! (ListVirtualMfaDevices) , $ macro ! (ListGroups) , $ macro ! (GetContextKeysForCustomPolicy) , $ macro ! (TagPolicy) , $ macro ! (AttachUserPolicy) , $ macro ! (ListRolePolicies) , $ macro ! (PutRolePermissionsBoundary) , $ macro ! (ListSshPublicKeys) , $ macro ! (CreateUser) , $ macro ! (ListMfaDevices) , $ macro ! (RemoveUserFromGroup) , $ macro ! (DeleteGroup) , $ macro ! (GetCredentialReport) , $ macro ! (SetDefaultPolicyVersion) , $ macro ! (DeleteRole) , $ macro ! (DeletePolicy) , $ macro ! (CreateVirtualMfaDevice) , $ macro ! (ListAccountAliases) , $ macro ! (GetServiceLinkedRoleDeletionStatus) , $ macro ! (ListServerCertificates) , $ macro ! (ListRoles) , $ macro ! (GetServiceLastAccessedDetails) , $ macro ! (UpdateLoginProfile) , $ macro ! (ListSigningCertificates) , $ macro ! (PutUserPermissionsBoundary) , $ macro ! (DeleteAccessKey) , $ macro ! (UpdateSamlProvider) , $ macro ! (AttachRolePolicy) , $ macro ! (AddUserToGroup) , $ macro ! (TagMfaDevice) , $ macro ! (GetPolicy) , $ macro ! (DeactivateMfaDevice) , $ macro ! (DeleteOpenIdConnectProvider) , $ macro ! (ListUserPolicies) , $ macro ! (DeleteRolePolicy) , $ macro ! (GenerateCredentialReport) , } }
#[doc = r" This macro matches a variable of Ops type and expands the provided $macro"]
#[doc = r" for each operation to generate code handler per op."]
macro_rules! generate_ops_match {
    ($ macro : ident , $ op : expr) => {
        match ($op) {
            ops::Ops::RemoveRoleFromInstanceProfile => $macro!(RemoveRoleFromInstanceProfile),
            ops::Ops::SimulatePrincipalPolicy => $macro!(SimulatePrincipalPolicy),
            ops::Ops::UntagOpenIdConnectProvider => $macro!(UntagOpenIdConnectProvider),
            ops::Ops::CreateLoginProfile => $macro!(CreateLoginProfile),
            ops::Ops::ListInstanceProfileTags => $macro!(ListInstanceProfileTags),
            ops::Ops::ListOpenIdConnectProviderTags => $macro!(ListOpenIdConnectProviderTags),
            ops::Ops::DeletePolicyVersion => $macro!(DeletePolicyVersion),
            ops::Ops::ListAttachedRolePolicies => $macro!(ListAttachedRolePolicies),
            ops::Ops::GetAccountAuthorizationDetails => $macro!(GetAccountAuthorizationDetails),
            ops::Ops::DeleteSigningCertificate => $macro!(DeleteSigningCertificate),
            ops::Ops::AddRoleToInstanceProfile => $macro!(AddRoleToInstanceProfile),
            ops::Ops::CreateAccessKey => $macro!(CreateAccessKey),
            ops::Ops::ListGroupsForUser => $macro!(ListGroupsForUser),
            ops::Ops::DeleteVirtualMfaDevice => $macro!(DeleteVirtualMfaDevice),
            ops::Ops::GetAccessKeyLastUsed => $macro!(GetAccessKeyLastUsed),
            ops::Ops::UploadServerCertificate => $macro!(UploadServerCertificate),
            ops::Ops::GetRolePolicy => $macro!(GetRolePolicy),
            ops::Ops::UpdateUser => $macro!(UpdateUser),
            ops::Ops::DeleteUser => $macro!(DeleteUser),
            ops::Ops::ListServiceSpecificCredentials => $macro!(ListServiceSpecificCredentials),
            ops::Ops::DeleteServiceLinkedRole => $macro!(DeleteServiceLinkedRole),
            ops::Ops::UpdateSigningCertificate => $macro!(UpdateSigningCertificate),
            ops::Ops::GetGroupPolicy => $macro!(GetGroupPolicy),
            ops::Ops::UntagUser => $macro!(UntagUser),
            ops::Ops::DeleteAccountPasswordPolicy => $macro!(DeleteAccountPasswordPolicy),
            ops::Ops::DeleteLoginProfile => $macro!(DeleteLoginProfile),
            ops::Ops::DeleteServerCertificate => $macro!(DeleteServerCertificate),
            ops::Ops::DeleteUserPermissionsBoundary => $macro!(DeleteUserPermissionsBoundary),
            ops::Ops::SetSecurityTokenServicePreferences => {
                $macro!(SetSecurityTokenServicePreferences)
            }
            ops::Ops::TagUser => $macro!(TagUser),
            ops::Ops::ResetServiceSpecificCredential => $macro!(ResetServiceSpecificCredential),
            ops::Ops::GetPolicyVersion => $macro!(GetPolicyVersion),
            ops::Ops::TagSamlProvider => $macro!(TagSamlProvider),
            ops::Ops::CreateGroup => $macro!(CreateGroup),
            ops::Ops::AttachGroupPolicy => $macro!(AttachGroupPolicy),
            ops::Ops::ListAttachedGroupPolicies => $macro!(ListAttachedGroupPolicies),
            ops::Ops::UpdateGroup => $macro!(UpdateGroup),
            ops::Ops::CreateOpenIdConnectProvider => $macro!(CreateOpenIdConnectProvider),
            ops::Ops::GetServerCertificate => $macro!(GetServerCertificate),
            ops::Ops::CreatePolicyVersion => $macro!(CreatePolicyVersion),
            ops::Ops::GetAccountSummary => $macro!(GetAccountSummary),
            ops::Ops::UpdateServiceSpecificCredential => $macro!(UpdateServiceSpecificCredential),
            ops::Ops::DeleteRolePermissionsBoundary => $macro!(DeleteRolePermissionsBoundary),
            ops::Ops::CreateServiceLinkedRole => $macro!(CreateServiceLinkedRole),
            ops::Ops::UntagPolicy => $macro!(UntagPolicy),
            ops::Ops::UpdateAccountPasswordPolicy => $macro!(UpdateAccountPasswordPolicy),
            ops::Ops::RemoveClientIdFromOpenIdConnectProvider => {
                $macro!(RemoveClientIdFromOpenIdConnectProvider)
            }
            ops::Ops::ListSamlProviderTags => $macro!(ListSamlProviderTags),
            ops::Ops::UpdateAssumeRolePolicy => $macro!(UpdateAssumeRolePolicy),
            ops::Ops::GetSamlProvider => $macro!(GetSamlProvider),
            ops::Ops::CreateInstanceProfile => $macro!(CreateInstanceProfile),
            ops::Ops::GetServiceLastAccessedDetailsWithEntities => {
                $macro!(GetServiceLastAccessedDetailsWithEntities)
            }
            ops::Ops::GetUser => $macro!(GetUser),
            ops::Ops::ListAccessKeys => $macro!(ListAccessKeys),
            ops::Ops::CreateServiceSpecificCredential => $macro!(CreateServiceSpecificCredential),
            ops::Ops::EnableMfaDevice => $macro!(EnableMfaDevice),
            ops::Ops::UploadSigningCertificate => $macro!(UploadSigningCertificate),
            ops::Ops::DeleteAccountAlias => $macro!(DeleteAccountAlias),
            ops::Ops::TagInstanceProfile => $macro!(TagInstanceProfile),
            ops::Ops::ListGroupPolicies => $macro!(ListGroupPolicies),
            ops::Ops::ChangePassword => $macro!(ChangePassword),
            ops::Ops::ListInstanceProfilesForRole => $macro!(ListInstanceProfilesForRole),
            ops::Ops::ListUsers => $macro!(ListUsers),
            ops::Ops::GetSshPublicKey => $macro!(GetSshPublicKey),
            ops::Ops::GenerateServiceLastAccessedDetails => {
                $macro!(GenerateServiceLastAccessedDetails)
            }
            ops::Ops::ListOpenIdConnectProviders => $macro!(ListOpenIdConnectProviders),
            ops::Ops::UpdateRole => $macro!(UpdateRole),
            ops::Ops::ListPolicyVersions => $macro!(ListPolicyVersions),
            ops::Ops::GetInstanceProfile => $macro!(GetInstanceProfile),
            ops::Ops::UntagSamlProvider => $macro!(UntagSamlProvider),
            ops::Ops::CreateRole => $macro!(CreateRole),
            ops::Ops::DetachGroupPolicy => $macro!(DetachGroupPolicy),
            ops::Ops::PutRolePolicy => $macro!(PutRolePolicy),
            ops::Ops::ListAttachedUserPolicies => $macro!(ListAttachedUserPolicies),
            ops::Ops::DetachUserPolicy => $macro!(DetachUserPolicy),
            ops::Ops::ListPolicies => $macro!(ListPolicies),
            ops::Ops::ListMfaDeviceTags => $macro!(ListMfaDeviceTags),
            ops::Ops::UpdateServerCertificate => $macro!(UpdateServerCertificate),
            ops::Ops::TagRole => $macro!(TagRole),
            ops::Ops::DeleteGroupPolicy => $macro!(DeleteGroupPolicy),
            ops::Ops::UpdateSshPublicKey => $macro!(UpdateSshPublicKey),
            ops::Ops::ListServerCertificateTags => $macro!(ListServerCertificateTags),
            ops::Ops::GenerateOrganizationsAccessReport => {
                $macro!(GenerateOrganizationsAccessReport)
            }
            ops::Ops::DeleteInstanceProfile => $macro!(DeleteInstanceProfile),
            ops::Ops::DeleteSshPublicKey => $macro!(DeleteSshPublicKey),
            ops::Ops::TagOpenIdConnectProvider => $macro!(TagOpenIdConnectProvider),
            ops::Ops::SimulateCustomPolicy => $macro!(SimulateCustomPolicy),
            ops::Ops::PutUserPolicy => $macro!(PutUserPolicy),
            ops::Ops::GetOrganizationsAccessReport => $macro!(GetOrganizationsAccessReport),
            ops::Ops::GetOpenIdConnectProvider => $macro!(GetOpenIdConnectProvider),
            ops::Ops::UpdateAccessKey => $macro!(UpdateAccessKey),
            ops::Ops::GetAccountPasswordPolicy => $macro!(GetAccountPasswordPolicy),
            ops::Ops::UploadSshPublicKey => $macro!(UploadSshPublicKey),
            ops::Ops::CreatePolicy => $macro!(CreatePolicy),
            ops::Ops::ListEntitiesForPolicy => $macro!(ListEntitiesForPolicy),
            ops::Ops::UntagInstanceProfile => $macro!(UntagInstanceProfile),
            ops::Ops::UntagRole => $macro!(UntagRole),
            ops::Ops::ListPoliciesGrantingServiceAccess => {
                $macro!(ListPoliciesGrantingServiceAccess)
            }
            ops::Ops::GetUserPolicy => $macro!(GetUserPolicy),
            ops::Ops::PutGroupPolicy => $macro!(PutGroupPolicy),
            ops::Ops::GetRole => $macro!(GetRole),
            ops::Ops::UntagMfaDevice => $macro!(UntagMfaDevice),
            ops::Ops::UntagServerCertificate => $macro!(UntagServerCertificate),
            ops::Ops::ListSamlProviders => $macro!(ListSamlProviders),
            ops::Ops::DeleteServiceSpecificCredential => $macro!(DeleteServiceSpecificCredential),
            ops::Ops::DetachRolePolicy => $macro!(DetachRolePolicy),
            ops::Ops::UpdateOpenIdConnectProviderThumbprint => {
                $macro!(UpdateOpenIdConnectProviderThumbprint)
            }
            ops::Ops::GetGroup => $macro!(GetGroup),
            ops::Ops::GetContextKeysForPrincipalPolicy => $macro!(GetContextKeysForPrincipalPolicy),
            ops::Ops::TagServerCertificate => $macro!(TagServerCertificate),
            ops::Ops::GetLoginProfile => $macro!(GetLoginProfile),
            ops::Ops::ListRoleTags => $macro!(ListRoleTags),
            ops::Ops::UpdateRoleDescription => $macro!(UpdateRoleDescription),
            ops::Ops::CreateSamlProvider => $macro!(CreateSamlProvider),
            ops::Ops::CreateAccountAlias => $macro!(CreateAccountAlias),
            ops::Ops::AddClientIdToOpenIdConnectProvider => {
                $macro!(AddClientIdToOpenIdConnectProvider)
            }
            ops::Ops::DeleteUserPolicy => $macro!(DeleteUserPolicy),
            ops::Ops::DeleteSamlProvider => $macro!(DeleteSamlProvider),
            ops::Ops::ResyncMfaDevice => $macro!(ResyncMfaDevice),
            ops::Ops::ListInstanceProfiles => $macro!(ListInstanceProfiles),
            ops::Ops::ListUserTags => $macro!(ListUserTags),
            ops::Ops::ListPolicyTags => $macro!(ListPolicyTags),
            ops::Ops::ListVirtualMfaDevices => $macro!(ListVirtualMfaDevices),
            ops::Ops::ListGroups => $macro!(ListGroups),
            ops::Ops::GetContextKeysForCustomPolicy => $macro!(GetContextKeysForCustomPolicy),
            ops::Ops::TagPolicy => $macro!(TagPolicy),
            ops::Ops::AttachUserPolicy => $macro!(AttachUserPolicy),
            ops::Ops::ListRolePolicies => $macro!(ListRolePolicies),
            ops::Ops::PutRolePermissionsBoundary => $macro!(PutRolePermissionsBoundary),
            ops::Ops::ListSshPublicKeys => $macro!(ListSshPublicKeys),
            ops::Ops::CreateUser => $macro!(CreateUser),
            ops::Ops::ListMfaDevices => $macro!(ListMfaDevices),
            ops::Ops::RemoveUserFromGroup => $macro!(RemoveUserFromGroup),
            ops::Ops::DeleteGroup => $macro!(DeleteGroup),
            ops::Ops::GetCredentialReport => $macro!(GetCredentialReport),
            ops::Ops::SetDefaultPolicyVersion => $macro!(SetDefaultPolicyVersion),
            ops::Ops::DeleteRole => $macro!(DeleteRole),
            ops::Ops::DeletePolicy => $macro!(DeletePolicy),
            ops::Ops::CreateVirtualMfaDevice => $macro!(CreateVirtualMfaDevice),
            ops::Ops::ListAccountAliases => $macro!(ListAccountAliases),
            ops::Ops::GetServiceLinkedRoleDeletionStatus => {
                $macro!(GetServiceLinkedRoleDeletionStatus)
            }
            ops::Ops::ListServerCertificates => $macro!(ListServerCertificates),
            ops::Ops::ListRoles => $macro!(ListRoles),
            ops::Ops::GetServiceLastAccessedDetails => $macro!(GetServiceLastAccessedDetails),
            ops::Ops::UpdateLoginProfile => $macro!(UpdateLoginProfile),
            ops::Ops::ListSigningCertificates => $macro!(ListSigningCertificates),
            ops::Ops::PutUserPermissionsBoundary => $macro!(PutUserPermissionsBoundary),
            ops::Ops::DeleteAccessKey => $macro!(DeleteAccessKey),
            ops::Ops::UpdateSamlProvider => $macro!(UpdateSamlProvider),
            ops::Ops::AttachRolePolicy => $macro!(AttachRolePolicy),
            ops::Ops::AddUserToGroup => $macro!(AddUserToGroup),
            ops::Ops::TagMfaDevice => $macro!(TagMfaDevice),
            ops::Ops::GetPolicy => $macro!(GetPolicy),
            ops::Ops::DeactivateMfaDevice => $macro!(DeactivateMfaDevice),
            ops::Ops::DeleteOpenIdConnectProvider => $macro!(DeleteOpenIdConnectProvider),
            ops::Ops::ListUserPolicies => $macro!(ListUserPolicies),
            ops::Ops::DeleteRolePolicy => $macro!(DeleteRolePolicy),
            ops::Ops::GenerateCredentialReport => $macro!(GenerateCredentialReport),
        }
    };
}
pub(crate) use generate_ops_code;
pub(crate) use generate_ops_items;
pub(crate) use generate_ops_match;
