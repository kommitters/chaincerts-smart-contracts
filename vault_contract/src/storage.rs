use crate::verifiable_credential::VerifiableCredential;
use soroban_sdk::{contracttype, Address, Env, String, Vec};

// MAXIMUM ENTRY TTL:
// 31 days, 12 ledger close per minute.
// (12 * 60 * 24 * 31) - 1
const LEDGERS_TO_EXTEND: u32 = 535_679;

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,   // Address
    Did,     // String
    Revoked, // Boolean
    Issuers, // Vec<Address>
    VCs,     // Vec<VerifiableCredential>
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

pub fn write_did(e: &Env, did: &String) {
    let key = DataKey::Did;
    e.storage().instance().set(&key, did);
}

pub fn read_revoked(e: &Env) -> bool {
    let key = DataKey::Revoked;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_revoked(e: &Env, revoked: &bool) {
    let key = DataKey::Revoked;
    e.storage().instance().set(&key, revoked);
}

pub fn read_issuers(e: &Env) -> Vec<Address> {
    let key = DataKey::Issuers;
    e.storage().persistent().get(&key).unwrap()
}

pub fn write_issuers(e: &Env, issuers: &Vec<Address>) {
    let key = DataKey::Issuers;
    e.storage().persistent().set(&key, issuers)
}

pub fn read_vcs(e: &Env) -> Vec<VerifiableCredential> {
    let key = DataKey::VCs;
    e.storage().persistent().get(&key).unwrap()
}

pub fn write_vcs(e: &Env, vcs: &Vec<VerifiableCredential>) {
    let key = DataKey::VCs;
    e.storage().persistent().set(&key, vcs)
}

pub fn extend_ttl_to_instance(e: &Env) {
    e.storage()
        .instance()
        .extend_ttl(LEDGERS_TO_EXTEND, LEDGERS_TO_EXTEND);
}

pub fn extend_ttl_to_persistent(e: &Env) {
    let vcs_key = DataKey::VCs;
    let issuers_key = DataKey::Issuers;

    e.storage()
        .persistent()
        .extend_ttl(&vcs_key, LEDGERS_TO_EXTEND, LEDGERS_TO_EXTEND);
    e.storage()
        .persistent()
        .extend_ttl(&issuers_key, LEDGERS_TO_EXTEND, LEDGERS_TO_EXTEND);
}
