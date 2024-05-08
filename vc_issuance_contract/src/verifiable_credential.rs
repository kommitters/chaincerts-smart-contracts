use crate::error::ContractError;
use crate::storage;
use soroban_sdk::{contracttype, panic_with_error, Env, String};

#[derive(PartialEq)]
#[contracttype]
pub enum VCStatus {
    Valid,
    Invalid,
    Revoked(String),
}

pub fn revoke_vc(e: &Env, vc_id: String, date: String) {
    let vc_status = storage::read_vc(e, &vc_id);

    if vc_status != VCStatus::Valid {
        panic_with_error!(e, ContractError::VCAlreadyRevoked)
    }
    storage::write_vc(e, &vc_id, &VCStatus::Revoked(date))
}
