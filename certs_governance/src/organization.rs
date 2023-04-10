//! Module Organization
//!
//! Module to obtain and modify the organization, and verify its existence in the contract.
use crate::{
    error::ContractError,
    storage_types::{DataKey, Organization},
};
use soroban_sdk::{panic_with_error, Address, Bytes, Env};

pub fn has_organization(e: &Env) -> bool {
    let key = DataKey::Org;
    e.storage().has(&key)
}

pub fn write_organization(e: &Env, org: Organization) {
    let key = DataKey::Org;
    e.storage().set(&key, &org);
}

pub fn read_organization_id(e: &Env) -> Bytes {
    let key = DataKey::Org;
    let organization: Organization = e.storage().get_unchecked(&key).unwrap();
    organization.org_id
}

fn read_organization_admin(e: &Env) -> Address {
    let key = DataKey::Org;
    let organization: Organization = e.storage().get_unchecked(&key).unwrap();
    organization.admin
}

pub fn check_admin(e: &Env, admin: &Address) {
    if admin != &read_organization_admin(e) {
        panic_with_error!(e, ContractError::NotAuthorized);
    }
}
