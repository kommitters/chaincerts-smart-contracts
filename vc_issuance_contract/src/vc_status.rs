use soroban_sdk::{contracttype, String};

#[derive(PartialEq)]
#[contracttype]
pub enum Status {
    Valid,
    Invalid,
    Revoked(String),
}
