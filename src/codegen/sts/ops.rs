#![allow(unused)]
#![allow(non_camel_case_types)]

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Ops {
    GetFederationToken,
    GetAccessKeyInfo,
    GetCallerIdentity,
    AssumeRoleWithSaml,
    AssumeRole,
    AssumeRoleWithWebIdentity,
    GetSessionToken,
    DecodeAuthorizationMessage,
}
#[doc = r" This macro calls a provided $macro for each operation to generate code per op."]
macro_rules! generate_ops_code {
    ($ macro : ident) => {
        $macro!(GetFederationToken);
        $macro!(GetAccessKeyInfo);
        $macro!(GetCallerIdentity);
        $macro!(AssumeRoleWithSaml);
        $macro!(AssumeRole);
        $macro!(AssumeRoleWithWebIdentity);
        $macro!(GetSessionToken);
        $macro!(DecodeAuthorizationMessage);
    };
}
#[doc = r" This macro calls a provided $macro for each operation to generate item per op."]
macro_rules ! generate_ops_items { ($ macro : ident) => { $ macro ! (GetFederationToken) , $ macro ! (GetAccessKeyInfo) , $ macro ! (GetCallerIdentity) , $ macro ! (AssumeRoleWithSaml) , $ macro ! (AssumeRole) , $ macro ! (AssumeRoleWithWebIdentity) , $ macro ! (GetSessionToken) , $ macro ! (DecodeAuthorizationMessage) , } }
#[doc = r" This macro matches a variable of Ops type and expands the provided $macro"]
#[doc = r" for each operation to generate code handler per op."]
macro_rules! generate_ops_match {
    ($ macro : ident , $ op : expr) => {
        match ($op) {
            ops::Ops::GetFederationToken => $macro!(GetFederationToken),
            ops::Ops::GetAccessKeyInfo => $macro!(GetAccessKeyInfo),
            ops::Ops::GetCallerIdentity => $macro!(GetCallerIdentity),
            ops::Ops::AssumeRoleWithSaml => $macro!(AssumeRoleWithSaml),
            ops::Ops::AssumeRole => $macro!(AssumeRole),
            ops::Ops::AssumeRoleWithWebIdentity => $macro!(AssumeRoleWithWebIdentity),
            ops::Ops::GetSessionToken => $macro!(GetSessionToken),
            ops::Ops::DecodeAuthorizationMessage => $macro!(DecodeAuthorizationMessage),
        }
    };
}
pub(crate) use generate_ops_code;
pub(crate) use generate_ops_items;
pub(crate) use generate_ops_match;
