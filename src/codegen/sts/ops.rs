#![allow(unused)]
#![allow(non_camel_case_types)]

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Ops {
    GetAccessKeyInfo,
    DecodeAuthorizationMessage,
    AssumeRoleWithSaml,
    GetCallerIdentity,
    AssumeRole,
    GetSessionToken,
    AssumeRoleWithWebIdentity,
    GetFederationToken,
}
#[doc = r" This macro calls a provided $macro for each operation to generate code per op."]
macro_rules! generate_ops_code {
    ($ macro : ident) => {
        $macro!(GetAccessKeyInfo);
        $macro!(DecodeAuthorizationMessage);
        $macro!(AssumeRoleWithSaml);
        $macro!(GetCallerIdentity);
        $macro!(AssumeRole);
        $macro!(GetSessionToken);
        $macro!(AssumeRoleWithWebIdentity);
        $macro!(GetFederationToken);
    };
}
#[doc = r" This macro calls a provided $macro for each operation to generate item per op."]
macro_rules ! generate_ops_items { ($ macro : ident) => { $ macro ! (GetAccessKeyInfo) , $ macro ! (DecodeAuthorizationMessage) , $ macro ! (AssumeRoleWithSaml) , $ macro ! (GetCallerIdentity) , $ macro ! (AssumeRole) , $ macro ! (GetSessionToken) , $ macro ! (AssumeRoleWithWebIdentity) , $ macro ! (GetFederationToken) , } }
#[doc = r" This macro matches a variable of Ops type and expands the provided $macro"]
#[doc = r" for each operation to generate code handler per op."]
macro_rules! generate_ops_match {
    ($ macro : ident , $ op : expr) => {
        match ($op) {
            ops::Ops::GetAccessKeyInfo => $macro!(GetAccessKeyInfo),
            ops::Ops::DecodeAuthorizationMessage => $macro!(DecodeAuthorizationMessage),
            ops::Ops::AssumeRoleWithSaml => $macro!(AssumeRoleWithSaml),
            ops::Ops::GetCallerIdentity => $macro!(GetCallerIdentity),
            ops::Ops::AssumeRole => $macro!(AssumeRole),
            ops::Ops::GetSessionToken => $macro!(GetSessionToken),
            ops::Ops::AssumeRoleWithWebIdentity => $macro!(AssumeRoleWithWebIdentity),
            ops::Ops::GetFederationToken => $macro!(GetFederationToken),
        }
    };
}
pub(crate) use generate_ops_code;
pub(crate) use generate_ops_items;
pub(crate) use generate_ops_match;
