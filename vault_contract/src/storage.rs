use crate::issuer::Issuer;
use crate::vault::Vault;
use soroban_sdk::{contracttype, Address, Env, Map, String};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,           // Address
    Issuers(String), // Map<Address, Issuer>
    Vaults,          // Map<Vault>
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

pub fn read_issuers(e: &Env, did: &String) -> Map<Address, Issuer> {
    let key = DataKey::Issuers(did.clone());
    e.storage().instance().get(&key).unwrap_or(Map::new(e))
}

pub fn write_issuers(e: &Env, issuers: &Map<Address, Issuer>, did: &String) {
    let key = DataKey::Issuers(did.clone());
    e.storage().instance().set(&key, issuers)
}

pub fn read_vaults(e: &Env) -> Map<String, Vault> {
    let key = DataKey::Vaults;
    e.storage().instance().get(&key).unwrap_or(Map::new(e))
}

pub fn write_vaults(e: &Env, vaults: &Map<String, Vault>) {
    let key = DataKey::Vaults;
    e.storage().instance().set(&key, vaults)
}
