use crate::revocation::Revocation;
use soroban_sdk::{contracttype, Address, Env, Map, String, Vec};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,       // Address
    IssuerDID,   // String
    Amount,      // U32
    VCs,         // Vec<String>
    Revocations, // Map<String, Revocation>
}

pub fn has_admin(e: &Env) -> bool {
    let key = DataKey::Admin;
    e.storage().instance().has(&key)
}

pub fn read_admin(e: &Env) -> Address {
    let key = DataKey::Admin;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_admin(e: &Env, id: &Address) {
    let key = DataKey::Admin;
    e.storage().instance().set(&key, id);
}

pub fn read_issuer_did(e: &Env) -> String {
    let key = DataKey::IssuerDID;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_issuer_did(e: &Env, issuer_did: &String) {
    let key = DataKey::IssuerDID;
    e.storage().instance().set(&key, issuer_did);
}

pub fn read_amount(e: &Env) -> u32 {
    let key = DataKey::Amount;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_amount(e: &Env, amount: &u32) {
    let key = DataKey::Amount;
    e.storage().instance().set(&key, amount)
}

pub fn write_vcs(e: &Env, vc: &Vec<String>) {
    let key = DataKey::VCs;
    e.storage().persistent().set(&key, vc)
}

pub fn read_vcs(e: &Env) -> Vec<String> {
    let key = DataKey::VCs;
    e.storage().persistent().get(&key).unwrap()
}

pub fn write_vcs_revocations(e: &Env, revocations: &Map<String, Revocation>) {
    let key = DataKey::Revocations;
    e.storage().persistent().set(&key, revocations)
}

pub fn read_vcs_revocations(e: &Env) -> Map<String, Revocation> {
    let key = DataKey::Revocations;
    e.storage().persistent().get(&key).unwrap()
}
