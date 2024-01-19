use crate::error::ContractError;
use crate::storage;
use soroban_sdk::{panic_with_error, Address, Env, String, Vec};

pub fn authorize_issuer(e: &Env, issuer: &Address, did: &String) {
    let mut issuers: Vec<Address> = storage::read_issuers(e, did);

    issuers.push_front(issuer.clone());

    storage::write_issuers(e, &issuers, did);
}

pub fn set_authorized_issuers(e: &Env, issuers: &Vec<Address>, did: &String) {
    storage::write_issuers(e, issuers, did);
}

pub fn revoke_issuer(e: &Env, issuer: &Address, did: &String) {
    let mut issuers = storage::read_issuers(e, did);

    if let Some(issuer_index) = issuers.first_index_of(issuer) {
        issuers.remove(issuer_index);
    } else {
        panic_with_error!(e, ContractError::IssuerNotFound)
    }

    storage::write_issuers(e, &issuers, did);
}

pub fn is_authorized(issuers: &Vec<Address>, issuer: &Address) -> bool {
    issuers.contains(issuer.clone())
}
