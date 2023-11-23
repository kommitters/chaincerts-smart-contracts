use soroban_sdk::{contracttype, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Revocation {
    pub id: String,
    pub date: String,
}
