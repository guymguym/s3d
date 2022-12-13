#![allow(unused)]
#![allow(non_camel_case_types)]

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Ops {
    DeleteAccountAlias,
    UpdateAssumeRolePolicy,
    DeleteGroup,
    GetOrganizationsAccessReport,
    DeleteSamlProvider,
    GetAccountSummary,
    GetAccountAuthorizationDetails,
    TagPolicy,
    GenerateOrganizationsAccessReport,
    GetGroup,
    UpdateServerCertificate,
    ListOpenIdConnectProviderTags,
    ListInstanceProfiles,
    CreateLoginProfile,
    ListPoliciesGrantingServiceAccess,
    ResyncMfaDevice,
    PutRolePermissionsBoundary,
    UpdateSigningCertificate,
    GetContextKeysForCustomPolicy,
    ListServerCertificateTags,
    RemoveRoleFromInstanceProfile,
    ResetServiceSpecificCredential,
    GetLoginProfile,
    ListPolicyVersions,
    AddRoleToInstanceProfile,
    CreateAccountAlias,
    CreateServiceSpecificCredential,
    DeleteUserPolicy,
    GetOpenIdConnectProvider,
    PutGroupPolicy,
    AttachGroupPolicy,
    ListMfaDevices,
    CreateGroup,
    GenerateCredentialReport,
    ListAccountAliases,
    ListMfaDeviceTags,
    UntagRole,
    ListRoles,
    GenerateServiceLastAccessedDetails,
    GetRolePolicy,
    CreateSamlProvider,
    GetServiceLastAccessedDetailsWithEntities,
    UploadSshPublicKey,
    UploadServerCertificate,
    RemoveClientIdFromOpenIdConnectProvider,
    UntagPolicy,
    RemoveUserFromGroup,
    AddClientIdToOpenIdConnectProvider,
    GetSamlProvider,
    CreateRole,
    DeleteServiceLinkedRole,
    GetUser,
    ListUserPolicies,
    UntagOpenIdConnectProvider,
    DeleteRolePermissionsBoundary,
    ListAttachedUserPolicies,
    UpdateAccountPasswordPolicy,
    UpdateServiceSpecificCredential,
    GetPolicy,
    DeleteAccessKey,
    SimulatePrincipalPolicy,
    ListSamlProviders,
    DeleteRolePolicy,
    DeleteGroupPolicy,
    DeleteServerCertificate,
    ListGroupsForUser,
    SetDefaultPolicyVersion,
    TagInstanceProfile,
    GetInstanceProfile,
    GetContextKeysForPrincipalPolicy,
    UpdateSshPublicKey,
    UploadSigningCertificate,
    TagServerCertificate,
    DeletePolicy,
    ListPolicyTags,
    ListUserTags,
    DetachUserPolicy,
    CreateUser,
    SetSecurityTokenServicePreferences,
    UntagSamlProvider,
    AttachRolePolicy,
    UntagServerCertificate,
    CreateServiceLinkedRole,
    UpdateSamlProvider,
    ListUsers,
    DeleteLoginProfile,
    UpdateAccessKey,
    CreatePolicyVersion,
    DeleteSigningCertificate,
    CreatePolicy,
    DeleteServiceSpecificCredential,
    UpdateUser,
    ChangePassword,
    DeleteUser,
    ListPolicies,
    UpdateRoleDescription,
    AddUserToGroup,
    ListEntitiesForPolicy,
    ListServiceSpecificCredentials,
    CreateAccessKey,
    DeleteUserPermissionsBoundary,
    ListInstanceProfileTags,
    PutUserPermissionsBoundary,
    DetachGroupPolicy,
    UntagUser,
    ListOpenIdConnectProviders,
    DeleteInstanceProfile,
    DeleteVirtualMfaDevice,
    GetAccessKeyLastUsed,
    DeleteRole,
    TagOpenIdConnectProvider,
    TagSamlProvider,
    SimulateCustomPolicy,
    ListServerCertificates,
    CreateOpenIdConnectProvider,
    GetPolicyVersion,
    DeleteSshPublicKey,
    ListSshPublicKeys,
    GetCredentialReport,
    TagMfaDevice,
    UntagInstanceProfile,
    UntagMfaDevice,
    CreateVirtualMfaDevice,
    UpdateOpenIdConnectProviderThumbprint,
    PutUserPolicy,
    TagUser,
    DeleteAccountPasswordPolicy,
    GetServiceLastAccessedDetails,
    GetRole,
    ListInstanceProfilesForRole,
    ListRolePolicies,
    DeactivateMfaDevice,
    GetGroupPolicy,
    DeleteOpenIdConnectProvider,
    ListGroupPolicies,
    ListSigningCertificates,
    ListAttachedGroupPolicies,
    EnableMfaDevice,
    ListVirtualMfaDevices,
    ListAccessKeys,
    AttachUserPolicy,
    UpdateGroup,
    GetSshPublicKey,
    GetServiceLinkedRoleDeletionStatus,
    GetServerCertificate,
    DetachRolePolicy,
    PutRolePolicy,
    ListGroups,
    GetUserPolicy,
    TagRole,
    ListSamlProviderTags,
    UpdateLoginProfile,
    UpdateRole,
    GetAccountPasswordPolicy,
    ListAttachedRolePolicies,
    CreateInstanceProfile,
    ListRoleTags,
    DeletePolicyVersion,
}
#[doc = r" This macro calls a provided $macro for each operation to generate code per op."]
macro_rules! generate_ops_code {
    ($ macro : ident) => {
        $macro!(DeleteAccountAlias);
        $macro!(UpdateAssumeRolePolicy);
        $macro!(DeleteGroup);
        $macro!(GetOrganizationsAccessReport);
        $macro!(DeleteSamlProvider);
        $macro!(GetAccountSummary);
        $macro!(GetAccountAuthorizationDetails);
        $macro!(TagPolicy);
        $macro!(GenerateOrganizationsAccessReport);
        $macro!(GetGroup);
        $macro!(UpdateServerCertificate);
        $macro!(ListOpenIdConnectProviderTags);
        $macro!(ListInstanceProfiles);
        $macro!(CreateLoginProfile);
        $macro!(ListPoliciesGrantingServiceAccess);
        $macro!(ResyncMfaDevice);
        $macro!(PutRolePermissionsBoundary);
        $macro!(UpdateSigningCertificate);
        $macro!(GetContextKeysForCustomPolicy);
        $macro!(ListServerCertificateTags);
        $macro!(RemoveRoleFromInstanceProfile);
        $macro!(ResetServiceSpecificCredential);
        $macro!(GetLoginProfile);
        $macro!(ListPolicyVersions);
        $macro!(AddRoleToInstanceProfile);
        $macro!(CreateAccountAlias);
        $macro!(CreateServiceSpecificCredential);
        $macro!(DeleteUserPolicy);
        $macro!(GetOpenIdConnectProvider);
        $macro!(PutGroupPolicy);
        $macro!(AttachGroupPolicy);
        $macro!(ListMfaDevices);
        $macro!(CreateGroup);
        $macro!(GenerateCredentialReport);
        $macro!(ListAccountAliases);
        $macro!(ListMfaDeviceTags);
        $macro!(UntagRole);
        $macro!(ListRoles);
        $macro!(GenerateServiceLastAccessedDetails);
        $macro!(GetRolePolicy);
        $macro!(CreateSamlProvider);
        $macro!(GetServiceLastAccessedDetailsWithEntities);
        $macro!(UploadSshPublicKey);
        $macro!(UploadServerCertificate);
        $macro!(RemoveClientIdFromOpenIdConnectProvider);
        $macro!(UntagPolicy);
        $macro!(RemoveUserFromGroup);
        $macro!(AddClientIdToOpenIdConnectProvider);
        $macro!(GetSamlProvider);
        $macro!(CreateRole);
        $macro!(DeleteServiceLinkedRole);
        $macro!(GetUser);
        $macro!(ListUserPolicies);
        $macro!(UntagOpenIdConnectProvider);
        $macro!(DeleteRolePermissionsBoundary);
        $macro!(ListAttachedUserPolicies);
        $macro!(UpdateAccountPasswordPolicy);
        $macro!(UpdateServiceSpecificCredential);
        $macro!(GetPolicy);
        $macro!(DeleteAccessKey);
        $macro!(SimulatePrincipalPolicy);
        $macro!(ListSamlProviders);
        $macro!(DeleteRolePolicy);
        $macro!(DeleteGroupPolicy);
        $macro!(DeleteServerCertificate);
        $macro!(ListGroupsForUser);
        $macro!(SetDefaultPolicyVersion);
        $macro!(TagInstanceProfile);
        $macro!(GetInstanceProfile);
        $macro!(GetContextKeysForPrincipalPolicy);
        $macro!(UpdateSshPublicKey);
        $macro!(UploadSigningCertificate);
        $macro!(TagServerCertificate);
        $macro!(DeletePolicy);
        $macro!(ListPolicyTags);
        $macro!(ListUserTags);
        $macro!(DetachUserPolicy);
        $macro!(CreateUser);
        $macro!(SetSecurityTokenServicePreferences);
        $macro!(UntagSamlProvider);
        $macro!(AttachRolePolicy);
        $macro!(UntagServerCertificate);
        $macro!(CreateServiceLinkedRole);
        $macro!(UpdateSamlProvider);
        $macro!(ListUsers);
        $macro!(DeleteLoginProfile);
        $macro!(UpdateAccessKey);
        $macro!(CreatePolicyVersion);
        $macro!(DeleteSigningCertificate);
        $macro!(CreatePolicy);
        $macro!(DeleteServiceSpecificCredential);
        $macro!(UpdateUser);
        $macro!(ChangePassword);
        $macro!(DeleteUser);
        $macro!(ListPolicies);
        $macro!(UpdateRoleDescription);
        $macro!(AddUserToGroup);
        $macro!(ListEntitiesForPolicy);
        $macro!(ListServiceSpecificCredentials);
        $macro!(CreateAccessKey);
        $macro!(DeleteUserPermissionsBoundary);
        $macro!(ListInstanceProfileTags);
        $macro!(PutUserPermissionsBoundary);
        $macro!(DetachGroupPolicy);
        $macro!(UntagUser);
        $macro!(ListOpenIdConnectProviders);
        $macro!(DeleteInstanceProfile);
        $macro!(DeleteVirtualMfaDevice);
        $macro!(GetAccessKeyLastUsed);
        $macro!(DeleteRole);
        $macro!(TagOpenIdConnectProvider);
        $macro!(TagSamlProvider);
        $macro!(SimulateCustomPolicy);
        $macro!(ListServerCertificates);
        $macro!(CreateOpenIdConnectProvider);
        $macro!(GetPolicyVersion);
        $macro!(DeleteSshPublicKey);
        $macro!(ListSshPublicKeys);
        $macro!(GetCredentialReport);
        $macro!(TagMfaDevice);
        $macro!(UntagInstanceProfile);
        $macro!(UntagMfaDevice);
        $macro!(CreateVirtualMfaDevice);
        $macro!(UpdateOpenIdConnectProviderThumbprint);
        $macro!(PutUserPolicy);
        $macro!(TagUser);
        $macro!(DeleteAccountPasswordPolicy);
        $macro!(GetServiceLastAccessedDetails);
        $macro!(GetRole);
        $macro!(ListInstanceProfilesForRole);
        $macro!(ListRolePolicies);
        $macro!(DeactivateMfaDevice);
        $macro!(GetGroupPolicy);
        $macro!(DeleteOpenIdConnectProvider);
        $macro!(ListGroupPolicies);
        $macro!(ListSigningCertificates);
        $macro!(ListAttachedGroupPolicies);
        $macro!(EnableMfaDevice);
        $macro!(ListVirtualMfaDevices);
        $macro!(ListAccessKeys);
        $macro!(AttachUserPolicy);
        $macro!(UpdateGroup);
        $macro!(GetSshPublicKey);
        $macro!(GetServiceLinkedRoleDeletionStatus);
        $macro!(GetServerCertificate);
        $macro!(DetachRolePolicy);
        $macro!(PutRolePolicy);
        $macro!(ListGroups);
        $macro!(GetUserPolicy);
        $macro!(TagRole);
        $macro!(ListSamlProviderTags);
        $macro!(UpdateLoginProfile);
        $macro!(UpdateRole);
        $macro!(GetAccountPasswordPolicy);
        $macro!(ListAttachedRolePolicies);
        $macro!(CreateInstanceProfile);
        $macro!(ListRoleTags);
        $macro!(DeletePolicyVersion);
    };
}
#[doc = r" This macro calls a provided $macro for each operation to generate item per op."]
macro_rules ! generate_ops_items { ($ macro : ident) => { $ macro ! (DeleteAccountAlias) , $ macro ! (UpdateAssumeRolePolicy) , $ macro ! (DeleteGroup) , $ macro ! (GetOrganizationsAccessReport) , $ macro ! (DeleteSamlProvider) , $ macro ! (GetAccountSummary) , $ macro ! (GetAccountAuthorizationDetails) , $ macro ! (TagPolicy) , $ macro ! (GenerateOrganizationsAccessReport) , $ macro ! (GetGroup) , $ macro ! (UpdateServerCertificate) , $ macro ! (ListOpenIdConnectProviderTags) , $ macro ! (ListInstanceProfiles) , $ macro ! (CreateLoginProfile) , $ macro ! (ListPoliciesGrantingServiceAccess) , $ macro ! (ResyncMfaDevice) , $ macro ! (PutRolePermissionsBoundary) , $ macro ! (UpdateSigningCertificate) , $ macro ! (GetContextKeysForCustomPolicy) , $ macro ! (ListServerCertificateTags) , $ macro ! (RemoveRoleFromInstanceProfile) , $ macro ! (ResetServiceSpecificCredential) , $ macro ! (GetLoginProfile) , $ macro ! (ListPolicyVersions) , $ macro ! (AddRoleToInstanceProfile) , $ macro ! (CreateAccountAlias) , $ macro ! (CreateServiceSpecificCredential) , $ macro ! (DeleteUserPolicy) , $ macro ! (GetOpenIdConnectProvider) , $ macro ! (PutGroupPolicy) , $ macro ! (AttachGroupPolicy) , $ macro ! (ListMfaDevices) , $ macro ! (CreateGroup) , $ macro ! (GenerateCredentialReport) , $ macro ! (ListAccountAliases) , $ macro ! (ListMfaDeviceTags) , $ macro ! (UntagRole) , $ macro ! (ListRoles) , $ macro ! (GenerateServiceLastAccessedDetails) , $ macro ! (GetRolePolicy) , $ macro ! (CreateSamlProvider) , $ macro ! (GetServiceLastAccessedDetailsWithEntities) , $ macro ! (UploadSshPublicKey) , $ macro ! (UploadServerCertificate) , $ macro ! (RemoveClientIdFromOpenIdConnectProvider) , $ macro ! (UntagPolicy) , $ macro ! (RemoveUserFromGroup) , $ macro ! (AddClientIdToOpenIdConnectProvider) , $ macro ! (GetSamlProvider) , $ macro ! (CreateRole) , $ macro ! (DeleteServiceLinkedRole) , $ macro ! (GetUser) , $ macro ! (ListUserPolicies) , $ macro ! (UntagOpenIdConnectProvider) , $ macro ! (DeleteRolePermissionsBoundary) , $ macro ! (ListAttachedUserPolicies) , $ macro ! (UpdateAccountPasswordPolicy) , $ macro ! (UpdateServiceSpecificCredential) , $ macro ! (GetPolicy) , $ macro ! (DeleteAccessKey) , $ macro ! (SimulatePrincipalPolicy) , $ macro ! (ListSamlProviders) , $ macro ! (DeleteRolePolicy) , $ macro ! (DeleteGroupPolicy) , $ macro ! (DeleteServerCertificate) , $ macro ! (ListGroupsForUser) , $ macro ! (SetDefaultPolicyVersion) , $ macro ! (TagInstanceProfile) , $ macro ! (GetInstanceProfile) , $ macro ! (GetContextKeysForPrincipalPolicy) , $ macro ! (UpdateSshPublicKey) , $ macro ! (UploadSigningCertificate) , $ macro ! (TagServerCertificate) , $ macro ! (DeletePolicy) , $ macro ! (ListPolicyTags) , $ macro ! (ListUserTags) , $ macro ! (DetachUserPolicy) , $ macro ! (CreateUser) , $ macro ! (SetSecurityTokenServicePreferences) , $ macro ! (UntagSamlProvider) , $ macro ! (AttachRolePolicy) , $ macro ! (UntagServerCertificate) , $ macro ! (CreateServiceLinkedRole) , $ macro ! (UpdateSamlProvider) , $ macro ! (ListUsers) , $ macro ! (DeleteLoginProfile) , $ macro ! (UpdateAccessKey) , $ macro ! (CreatePolicyVersion) , $ macro ! (DeleteSigningCertificate) , $ macro ! (CreatePolicy) , $ macro ! (DeleteServiceSpecificCredential) , $ macro ! (UpdateUser) , $ macro ! (ChangePassword) , $ macro ! (DeleteUser) , $ macro ! (ListPolicies) , $ macro ! (UpdateRoleDescription) , $ macro ! (AddUserToGroup) , $ macro ! (ListEntitiesForPolicy) , $ macro ! (ListServiceSpecificCredentials) , $ macro ! (CreateAccessKey) , $ macro ! (DeleteUserPermissionsBoundary) , $ macro ! (ListInstanceProfileTags) , $ macro ! (PutUserPermissionsBoundary) , $ macro ! (DetachGroupPolicy) , $ macro ! (UntagUser) , $ macro ! (ListOpenIdConnectProviders) , $ macro ! (DeleteInstanceProfile) , $ macro ! (DeleteVirtualMfaDevice) , $ macro ! (GetAccessKeyLastUsed) , $ macro ! (DeleteRole) , $ macro ! (TagOpenIdConnectProvider) , $ macro ! (TagSamlProvider) , $ macro ! (SimulateCustomPolicy) , $ macro ! (ListServerCertificates) , $ macro ! (CreateOpenIdConnectProvider) , $ macro ! (GetPolicyVersion) , $ macro ! (DeleteSshPublicKey) , $ macro ! (ListSshPublicKeys) , $ macro ! (GetCredentialReport) , $ macro ! (TagMfaDevice) , $ macro ! (UntagInstanceProfile) , $ macro ! (UntagMfaDevice) , $ macro ! (CreateVirtualMfaDevice) , $ macro ! (UpdateOpenIdConnectProviderThumbprint) , $ macro ! (PutUserPolicy) , $ macro ! (TagUser) , $ macro ! (DeleteAccountPasswordPolicy) , $ macro ! (GetServiceLastAccessedDetails) , $ macro ! (GetRole) , $ macro ! (ListInstanceProfilesForRole) , $ macro ! (ListRolePolicies) , $ macro ! (DeactivateMfaDevice) , $ macro ! (GetGroupPolicy) , $ macro ! (DeleteOpenIdConnectProvider) , $ macro ! (ListGroupPolicies) , $ macro ! (ListSigningCertificates) , $ macro ! (ListAttachedGroupPolicies) , $ macro ! (EnableMfaDevice) , $ macro ! (ListVirtualMfaDevices) , $ macro ! (ListAccessKeys) , $ macro ! (AttachUserPolicy) , $ macro ! (UpdateGroup) , $ macro ! (GetSshPublicKey) , $ macro ! (GetServiceLinkedRoleDeletionStatus) , $ macro ! (GetServerCertificate) , $ macro ! (DetachRolePolicy) , $ macro ! (PutRolePolicy) , $ macro ! (ListGroups) , $ macro ! (GetUserPolicy) , $ macro ! (TagRole) , $ macro ! (ListSamlProviderTags) , $ macro ! (UpdateLoginProfile) , $ macro ! (UpdateRole) , $ macro ! (GetAccountPasswordPolicy) , $ macro ! (ListAttachedRolePolicies) , $ macro ! (CreateInstanceProfile) , $ macro ! (ListRoleTags) , $ macro ! (DeletePolicyVersion) , } }
#[doc = r" This macro matches a variable of Ops type and expands the provided $macro"]
#[doc = r" for each operation to generate code handler per op."]
macro_rules! generate_ops_match {
    ($ macro : ident , $ op : expr) => {
        match ($op) {
            ops::Ops::DeleteAccountAlias => $macro!(DeleteAccountAlias),
            ops::Ops::UpdateAssumeRolePolicy => $macro!(UpdateAssumeRolePolicy),
            ops::Ops::DeleteGroup => $macro!(DeleteGroup),
            ops::Ops::GetOrganizationsAccessReport => $macro!(GetOrganizationsAccessReport),
            ops::Ops::DeleteSamlProvider => $macro!(DeleteSamlProvider),
            ops::Ops::GetAccountSummary => $macro!(GetAccountSummary),
            ops::Ops::GetAccountAuthorizationDetails => $macro!(GetAccountAuthorizationDetails),
            ops::Ops::TagPolicy => $macro!(TagPolicy),
            ops::Ops::GenerateOrganizationsAccessReport => {
                $macro!(GenerateOrganizationsAccessReport)
            }
            ops::Ops::GetGroup => $macro!(GetGroup),
            ops::Ops::UpdateServerCertificate => $macro!(UpdateServerCertificate),
            ops::Ops::ListOpenIdConnectProviderTags => $macro!(ListOpenIdConnectProviderTags),
            ops::Ops::ListInstanceProfiles => $macro!(ListInstanceProfiles),
            ops::Ops::CreateLoginProfile => $macro!(CreateLoginProfile),
            ops::Ops::ListPoliciesGrantingServiceAccess => {
                $macro!(ListPoliciesGrantingServiceAccess)
            }
            ops::Ops::ResyncMfaDevice => $macro!(ResyncMfaDevice),
            ops::Ops::PutRolePermissionsBoundary => $macro!(PutRolePermissionsBoundary),
            ops::Ops::UpdateSigningCertificate => $macro!(UpdateSigningCertificate),
            ops::Ops::GetContextKeysForCustomPolicy => $macro!(GetContextKeysForCustomPolicy),
            ops::Ops::ListServerCertificateTags => $macro!(ListServerCertificateTags),
            ops::Ops::RemoveRoleFromInstanceProfile => $macro!(RemoveRoleFromInstanceProfile),
            ops::Ops::ResetServiceSpecificCredential => $macro!(ResetServiceSpecificCredential),
            ops::Ops::GetLoginProfile => $macro!(GetLoginProfile),
            ops::Ops::ListPolicyVersions => $macro!(ListPolicyVersions),
            ops::Ops::AddRoleToInstanceProfile => $macro!(AddRoleToInstanceProfile),
            ops::Ops::CreateAccountAlias => $macro!(CreateAccountAlias),
            ops::Ops::CreateServiceSpecificCredential => $macro!(CreateServiceSpecificCredential),
            ops::Ops::DeleteUserPolicy => $macro!(DeleteUserPolicy),
            ops::Ops::GetOpenIdConnectProvider => $macro!(GetOpenIdConnectProvider),
            ops::Ops::PutGroupPolicy => $macro!(PutGroupPolicy),
            ops::Ops::AttachGroupPolicy => $macro!(AttachGroupPolicy),
            ops::Ops::ListMfaDevices => $macro!(ListMfaDevices),
            ops::Ops::CreateGroup => $macro!(CreateGroup),
            ops::Ops::GenerateCredentialReport => $macro!(GenerateCredentialReport),
            ops::Ops::ListAccountAliases => $macro!(ListAccountAliases),
            ops::Ops::ListMfaDeviceTags => $macro!(ListMfaDeviceTags),
            ops::Ops::UntagRole => $macro!(UntagRole),
            ops::Ops::ListRoles => $macro!(ListRoles),
            ops::Ops::GenerateServiceLastAccessedDetails => {
                $macro!(GenerateServiceLastAccessedDetails)
            }
            ops::Ops::GetRolePolicy => $macro!(GetRolePolicy),
            ops::Ops::CreateSamlProvider => $macro!(CreateSamlProvider),
            ops::Ops::GetServiceLastAccessedDetailsWithEntities => {
                $macro!(GetServiceLastAccessedDetailsWithEntities)
            }
            ops::Ops::UploadSshPublicKey => $macro!(UploadSshPublicKey),
            ops::Ops::UploadServerCertificate => $macro!(UploadServerCertificate),
            ops::Ops::RemoveClientIdFromOpenIdConnectProvider => {
                $macro!(RemoveClientIdFromOpenIdConnectProvider)
            }
            ops::Ops::UntagPolicy => $macro!(UntagPolicy),
            ops::Ops::RemoveUserFromGroup => $macro!(RemoveUserFromGroup),
            ops::Ops::AddClientIdToOpenIdConnectProvider => {
                $macro!(AddClientIdToOpenIdConnectProvider)
            }
            ops::Ops::GetSamlProvider => $macro!(GetSamlProvider),
            ops::Ops::CreateRole => $macro!(CreateRole),
            ops::Ops::DeleteServiceLinkedRole => $macro!(DeleteServiceLinkedRole),
            ops::Ops::GetUser => $macro!(GetUser),
            ops::Ops::ListUserPolicies => $macro!(ListUserPolicies),
            ops::Ops::UntagOpenIdConnectProvider => $macro!(UntagOpenIdConnectProvider),
            ops::Ops::DeleteRolePermissionsBoundary => $macro!(DeleteRolePermissionsBoundary),
            ops::Ops::ListAttachedUserPolicies => $macro!(ListAttachedUserPolicies),
            ops::Ops::UpdateAccountPasswordPolicy => $macro!(UpdateAccountPasswordPolicy),
            ops::Ops::UpdateServiceSpecificCredential => $macro!(UpdateServiceSpecificCredential),
            ops::Ops::GetPolicy => $macro!(GetPolicy),
            ops::Ops::DeleteAccessKey => $macro!(DeleteAccessKey),
            ops::Ops::SimulatePrincipalPolicy => $macro!(SimulatePrincipalPolicy),
            ops::Ops::ListSamlProviders => $macro!(ListSamlProviders),
            ops::Ops::DeleteRolePolicy => $macro!(DeleteRolePolicy),
            ops::Ops::DeleteGroupPolicy => $macro!(DeleteGroupPolicy),
            ops::Ops::DeleteServerCertificate => $macro!(DeleteServerCertificate),
            ops::Ops::ListGroupsForUser => $macro!(ListGroupsForUser),
            ops::Ops::SetDefaultPolicyVersion => $macro!(SetDefaultPolicyVersion),
            ops::Ops::TagInstanceProfile => $macro!(TagInstanceProfile),
            ops::Ops::GetInstanceProfile => $macro!(GetInstanceProfile),
            ops::Ops::GetContextKeysForPrincipalPolicy => $macro!(GetContextKeysForPrincipalPolicy),
            ops::Ops::UpdateSshPublicKey => $macro!(UpdateSshPublicKey),
            ops::Ops::UploadSigningCertificate => $macro!(UploadSigningCertificate),
            ops::Ops::TagServerCertificate => $macro!(TagServerCertificate),
            ops::Ops::DeletePolicy => $macro!(DeletePolicy),
            ops::Ops::ListPolicyTags => $macro!(ListPolicyTags),
            ops::Ops::ListUserTags => $macro!(ListUserTags),
            ops::Ops::DetachUserPolicy => $macro!(DetachUserPolicy),
            ops::Ops::CreateUser => $macro!(CreateUser),
            ops::Ops::SetSecurityTokenServicePreferences => {
                $macro!(SetSecurityTokenServicePreferences)
            }
            ops::Ops::UntagSamlProvider => $macro!(UntagSamlProvider),
            ops::Ops::AttachRolePolicy => $macro!(AttachRolePolicy),
            ops::Ops::UntagServerCertificate => $macro!(UntagServerCertificate),
            ops::Ops::CreateServiceLinkedRole => $macro!(CreateServiceLinkedRole),
            ops::Ops::UpdateSamlProvider => $macro!(UpdateSamlProvider),
            ops::Ops::ListUsers => $macro!(ListUsers),
            ops::Ops::DeleteLoginProfile => $macro!(DeleteLoginProfile),
            ops::Ops::UpdateAccessKey => $macro!(UpdateAccessKey),
            ops::Ops::CreatePolicyVersion => $macro!(CreatePolicyVersion),
            ops::Ops::DeleteSigningCertificate => $macro!(DeleteSigningCertificate),
            ops::Ops::CreatePolicy => $macro!(CreatePolicy),
            ops::Ops::DeleteServiceSpecificCredential => $macro!(DeleteServiceSpecificCredential),
            ops::Ops::UpdateUser => $macro!(UpdateUser),
            ops::Ops::ChangePassword => $macro!(ChangePassword),
            ops::Ops::DeleteUser => $macro!(DeleteUser),
            ops::Ops::ListPolicies => $macro!(ListPolicies),
            ops::Ops::UpdateRoleDescription => $macro!(UpdateRoleDescription),
            ops::Ops::AddUserToGroup => $macro!(AddUserToGroup),
            ops::Ops::ListEntitiesForPolicy => $macro!(ListEntitiesForPolicy),
            ops::Ops::ListServiceSpecificCredentials => $macro!(ListServiceSpecificCredentials),
            ops::Ops::CreateAccessKey => $macro!(CreateAccessKey),
            ops::Ops::DeleteUserPermissionsBoundary => $macro!(DeleteUserPermissionsBoundary),
            ops::Ops::ListInstanceProfileTags => $macro!(ListInstanceProfileTags),
            ops::Ops::PutUserPermissionsBoundary => $macro!(PutUserPermissionsBoundary),
            ops::Ops::DetachGroupPolicy => $macro!(DetachGroupPolicy),
            ops::Ops::UntagUser => $macro!(UntagUser),
            ops::Ops::ListOpenIdConnectProviders => $macro!(ListOpenIdConnectProviders),
            ops::Ops::DeleteInstanceProfile => $macro!(DeleteInstanceProfile),
            ops::Ops::DeleteVirtualMfaDevice => $macro!(DeleteVirtualMfaDevice),
            ops::Ops::GetAccessKeyLastUsed => $macro!(GetAccessKeyLastUsed),
            ops::Ops::DeleteRole => $macro!(DeleteRole),
            ops::Ops::TagOpenIdConnectProvider => $macro!(TagOpenIdConnectProvider),
            ops::Ops::TagSamlProvider => $macro!(TagSamlProvider),
            ops::Ops::SimulateCustomPolicy => $macro!(SimulateCustomPolicy),
            ops::Ops::ListServerCertificates => $macro!(ListServerCertificates),
            ops::Ops::CreateOpenIdConnectProvider => $macro!(CreateOpenIdConnectProvider),
            ops::Ops::GetPolicyVersion => $macro!(GetPolicyVersion),
            ops::Ops::DeleteSshPublicKey => $macro!(DeleteSshPublicKey),
            ops::Ops::ListSshPublicKeys => $macro!(ListSshPublicKeys),
            ops::Ops::GetCredentialReport => $macro!(GetCredentialReport),
            ops::Ops::TagMfaDevice => $macro!(TagMfaDevice),
            ops::Ops::UntagInstanceProfile => $macro!(UntagInstanceProfile),
            ops::Ops::UntagMfaDevice => $macro!(UntagMfaDevice),
            ops::Ops::CreateVirtualMfaDevice => $macro!(CreateVirtualMfaDevice),
            ops::Ops::UpdateOpenIdConnectProviderThumbprint => {
                $macro!(UpdateOpenIdConnectProviderThumbprint)
            }
            ops::Ops::PutUserPolicy => $macro!(PutUserPolicy),
            ops::Ops::TagUser => $macro!(TagUser),
            ops::Ops::DeleteAccountPasswordPolicy => $macro!(DeleteAccountPasswordPolicy),
            ops::Ops::GetServiceLastAccessedDetails => $macro!(GetServiceLastAccessedDetails),
            ops::Ops::GetRole => $macro!(GetRole),
            ops::Ops::ListInstanceProfilesForRole => $macro!(ListInstanceProfilesForRole),
            ops::Ops::ListRolePolicies => $macro!(ListRolePolicies),
            ops::Ops::DeactivateMfaDevice => $macro!(DeactivateMfaDevice),
            ops::Ops::GetGroupPolicy => $macro!(GetGroupPolicy),
            ops::Ops::DeleteOpenIdConnectProvider => $macro!(DeleteOpenIdConnectProvider),
            ops::Ops::ListGroupPolicies => $macro!(ListGroupPolicies),
            ops::Ops::ListSigningCertificates => $macro!(ListSigningCertificates),
            ops::Ops::ListAttachedGroupPolicies => $macro!(ListAttachedGroupPolicies),
            ops::Ops::EnableMfaDevice => $macro!(EnableMfaDevice),
            ops::Ops::ListVirtualMfaDevices => $macro!(ListVirtualMfaDevices),
            ops::Ops::ListAccessKeys => $macro!(ListAccessKeys),
            ops::Ops::AttachUserPolicy => $macro!(AttachUserPolicy),
            ops::Ops::UpdateGroup => $macro!(UpdateGroup),
            ops::Ops::GetSshPublicKey => $macro!(GetSshPublicKey),
            ops::Ops::GetServiceLinkedRoleDeletionStatus => {
                $macro!(GetServiceLinkedRoleDeletionStatus)
            }
            ops::Ops::GetServerCertificate => $macro!(GetServerCertificate),
            ops::Ops::DetachRolePolicy => $macro!(DetachRolePolicy),
            ops::Ops::PutRolePolicy => $macro!(PutRolePolicy),
            ops::Ops::ListGroups => $macro!(ListGroups),
            ops::Ops::GetUserPolicy => $macro!(GetUserPolicy),
            ops::Ops::TagRole => $macro!(TagRole),
            ops::Ops::ListSamlProviderTags => $macro!(ListSamlProviderTags),
            ops::Ops::UpdateLoginProfile => $macro!(UpdateLoginProfile),
            ops::Ops::UpdateRole => $macro!(UpdateRole),
            ops::Ops::GetAccountPasswordPolicy => $macro!(GetAccountPasswordPolicy),
            ops::Ops::ListAttachedRolePolicies => $macro!(ListAttachedRolePolicies),
            ops::Ops::CreateInstanceProfile => $macro!(CreateInstanceProfile),
            ops::Ops::ListRoleTags => $macro!(ListRoleTags),
            ops::Ops::DeletePolicyVersion => $macro!(DeletePolicyVersion),
        }
    };
}
pub(crate) use generate_ops_code;
pub(crate) use generate_ops_items;
pub(crate) use generate_ops_match;
