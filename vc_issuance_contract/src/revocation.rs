use crate::error::ContractError;
use crate::storage;
use soroban_sdk::{contracttype, panic_with_error, Env, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Revocation {
    pub vc_id: String,
    pub date: String,
}

pub fn revoke_vc(e: &Env, vc_id: String, date: String) {
    let mut revocations = storage::read_vcs_revocations(e);

    if revocations.contains_key(vc_id.clone()) {
        panic_with_error!(e, ContractError::VCAlreadyRevoked)
    }
    revocations.set(vc_id.clone(), Revocation { vc_id, date });
    storage::write_vcs_revocations(e, &revocations);
}
