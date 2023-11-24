use soroban_sdk::{contracttype, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VCRevocation {
    pub vc_id: String,
    pub date: String,
}
