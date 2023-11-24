use crate::vc_revocation::VCRevocation;
use soroban_sdk::{contracttype, Address, Env, Map, String, Vec};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,                 // Address
    Amount,                // U32
    VerifiableCredentials, // Vec<String>
    RevocationsList,       // Map<String, Revocation>
}

pub fn has_admin(e: &Env) -> bool {
    let key = DataKey::Admin;
    e.storage().instance().has(&key)
}

pub fn write_admin(e: &Env, id: &Address) {
    let key = DataKey::Admin;
    e.storage().instance().set(&key, id);
}

pub fn write_amount(e: &Env, amount: &u32) {
    let key = DataKey::Amount;
    e.storage().instance().set(&key, amount)
}

pub fn write_vcs(e: &Env, vc: &Vec<String>) {
    let key = DataKey::VerifiableCredentials;
    e.storage().instance().set(&key, vc)
}

pub fn write_vcs_revocations(e: &Env, revocations: &Map<String, VCRevocation>) {
    let key = DataKey::RevocationsList;
    e.storage().instance().set(&key, revocations)
}
