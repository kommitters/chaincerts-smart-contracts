use crate::verifiable_credential::VCStatus;
use soroban_sdk::{contracttype, Address, Env, Map, String, Vec};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,       // Address
    IssuerDID,   // String
    VC(String),  // VCStatus
    Revocations, // Map<String, Revocation>
    VCs,         // Vec<VerifiableCredential>
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Revocation {
    pub vc_id: String,
    pub date: String,
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

pub fn write_vc(e: &Env, vc_id: &String, status: &VCStatus) {
    let key = DataKey::VC(vc_id.clone());
    e.storage().persistent().set(&key, status)
}

pub fn read_vc(e: &Env, vc_id: &String) -> VCStatus {
    let key = DataKey::VC(vc_id.clone());
    e.storage()
        .persistent()
        .get(&key)
        .unwrap_or(VCStatus::Invalid)
}

pub fn read_old_vcs(e: &Env) -> Option<Vec<String>> {
    let key = DataKey::VCs;
    e.storage().persistent().get(&key)
}

pub fn remove_old_vcs(e: &Env) {
    let key = DataKey::VCs;
    e.storage().persistent().remove(&key);
}

pub fn read_old_revocations(e: &Env) -> Map<String, Revocation> {
    let key = DataKey::Revocations;
    e.storage().persistent().get(&key).unwrap()
}

pub fn remove_old_revocations(e: &Env) {
    let key = DataKey::Revocations;
    e.storage().persistent().remove(&key);
}
