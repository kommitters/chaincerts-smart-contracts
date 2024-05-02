use crate::verifiable_credential::VerifiableCredential;
use soroban_sdk::{contracttype, Address, Env, String, Vec};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,       // Address
    Did,         // String
    DidContract, // Address
    Revoked,     // Boolean
    Issuers,     // Vec<Address>
    VC(String),  // VerifiableCredential
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

pub fn write_did_contract(e: &Env, did_contract: &Address) {
    let key = DataKey::DidContract;
    e.storage().instance().set(&key, did_contract);
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

pub fn write_vc(e: &Env, vc_id: &String, vc: &VerifiableCredential) {
    let key = DataKey::VC(vc_id.clone());
    e.storage().persistent().set(&key, vc)
}
