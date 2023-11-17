use crate::{did::Did, error::ContractError, issuer::Issuer};
use soroban_sdk::{contracttype, panic_with_error, Address, Env, Map, String, Vec};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,   // Address
    Dids,    // String
    Issuers, // Address
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

pub fn write_dids(e: &Env, dids: &Vec<String>) {
    let key = DataKey::Dids;

    let mut dids_map: Map<String, Did> = Map::new(e);

    for did in dids.iter() {
        dids_map.set(
            did.clone(),
            Did {
                did: did.clone(),
                is_revoked: false,
            },
        )
    }
    e.storage().instance().set(&key, &dids_map);
}

pub fn read_issuers(e: &Env) -> Map<Address, Issuer> {
    let key = DataKey::Issuers;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_issuers(e: &Env, issuers: &Map<Address, Issuer>) {
    let key = DataKey::Issuers;
    e.storage().instance().set(&key, issuers);
}

pub fn write_issuer(e: &Env, issuer: &Address) {
    let mut issuers = read_issuers(e);
    issuers.set(
        issuer.clone(),
        Issuer {
            public_key: issuer.clone(),
            is_revoked: false,
        },
    );
    write_issuers(e, &issuers);
}

pub fn revoke_issuer(e: &Env, issuer: &Address) {
    let mut issuers = read_issuers(e);

    if issuers.contains_key(issuer.clone()) {
        issuers.set(
            issuer.clone(),
            Issuer {
                public_key: issuer.clone(),
                is_revoked: true,
            },
        )
    } else {
        panic_with_error!(e, ContractError::IssuerNotFound)
    }
    write_issuers(e, &issuers);
}
