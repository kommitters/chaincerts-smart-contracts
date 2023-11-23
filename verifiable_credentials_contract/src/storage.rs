use crate::revocation::Revocation;
use soroban_sdk::{contracttype, Address, Env, Map, String, Vec};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,       // Address
    Amount,      // U32
    VCs,         // Vec<String>
    Revocations, // Map<String, Revocation>
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
    let key = DataKey::VCs;
    e.storage().instance().set(&key, vc)
}

pub fn write_revocations(e: &Env, revocations: &Map<String, Revocation>) {
    let key = DataKey::Revocations;
    e.storage().instance().set(&key, revocations)
}
