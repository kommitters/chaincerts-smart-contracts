use soroban_sdk::{contracttype, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Revocation {
    pub vc_id: String,
    pub date: String,
}
