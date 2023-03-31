//! Module AccessControlList
//!
//! Module responsible of managing the ACL that allows organizations to deposit `Chaincerts` to a wallet
use soroban_sdk::{panic_with_error, vec, Bytes, Env, Vec};

use crate::error::ContractError;

use super::storage_types::DataKey;

const ACL_KEY: DataKey = DataKey::Acl;

pub(crate) fn get_acl(env: &Env) -> Vec<Bytes> {
    match env.storage().get(&ACL_KEY) {
        Some(acl) => acl.unwrap(),
        None => {
            panic_with_error!(env, ContractError::NoOrganizationsInACL)
        }
    }
}

pub(crate) fn add_organization(env: &Env, org_id: &Bytes) {
    let acl = match env.storage().get(&ACL_KEY) {
        Some(acl) => {
            let mut access_list: Vec<Bytes> = acl.unwrap();
            if !is_organization_in_acl(org_id, &access_list) {
                access_list.push_front(org_id.clone());
                access_list
            } else {
                panic_with_error!(env, ContractError::AlreadyInACL)
            }
        }
        None => {
            let access_list: Vec<Bytes> = vec![env, org_id.clone()];
            access_list
        }
    };
    env.storage().set(&ACL_KEY, &acl)
}

pub(crate) fn remove_organization(env: &Env, org_id: &Bytes) {
    match env.storage().get(&ACL_KEY) {
        Some(acl) => {
            let mut access_list: Vec<Bytes> = acl.unwrap();
            remove_from_acl(env, org_id, &mut access_list);
            env.storage().set(&ACL_KEY, &access_list)
        }
        None => {
            panic_with_error!(env, ContractError::NoOrganizationsInACL)
        }
    }
}

pub(crate) fn check_acl(env: &Env, org_id: &Bytes) {
    match env.storage().get(&ACL_KEY) {
        Some(acl) => {
            let access_list: Vec<Bytes> = acl.unwrap();
            for org in access_list.iter() {
                if org.unwrap() == org_id.clone() {
                    return;
                }
            }
            panic_with_error!(env, ContractError::NotAuthorized)
        }
        None => {
            panic_with_error!(env, ContractError::NoOrganizationsInACL)
        }
    }
}

fn remove_from_acl(env: &Env, org_id: &Bytes, access_list: &mut Vec<Bytes>) {
    for (index, org) in access_list.iter().enumerate() {
        if org.unwrap() == org_id.clone() {
            access_list.remove(index as u32).unwrap();
            return;
        }
    }
    panic_with_error!(env, ContractError::OrganizationNotFound)
}

fn is_organization_in_acl(org_id: &Bytes, access_list: &Vec<Bytes>) -> bool {
    for org in access_list.iter() {
        if org.unwrap() == org_id.clone() {
            return true;
        }
    }
    false
}
