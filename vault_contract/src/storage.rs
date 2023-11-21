use crate::{did::Did, issuer::Issuer};
use soroban_sdk::{contracttype, Address, Env, Map, String};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,   // Address
    Dids,    // Vec<String>
    Issuers, // Map<Address, Issuer>
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

pub fn write_dids(e: &Env, dids: &Map<String, Did>) {
    let key = DataKey::Dids;
    e.storage().instance().set(&key, dids);
}

pub fn read_issuers(e: &Env) -> Map<Address, Issuer> {
    let key = DataKey::Issuers;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_issuers(e: &Env, issuers: &Map<Address, Issuer>) {
    let key = DataKey::Issuers;
    e.storage().instance().set(&key, issuers);
}
