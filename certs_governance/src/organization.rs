//! Module Organization
//!
//! Module to obtain and modify the organization, and verify its existence in the contract.
use crate::storage_types::{DataKey, Organization};
use soroban_sdk::{Bytes, Env};

pub fn has_organization(e: &Env) -> bool {
    let key = DataKey::Org;
    e.storage().has(&key)
}

pub fn read_organization_id(e: &Env) -> Bytes {
    let key = DataKey::Org;
    let organization: Organization = e.storage().get_unchecked(&key).unwrap();
    organization.id_org
}

pub fn write_organization(e: &Env, org: Organization) {
    let key = DataKey::Org;
    e.storage().set(&key, &org);
}
