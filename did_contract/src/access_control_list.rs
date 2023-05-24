//! Module AccessControlList
//!
//! Module responsible of managing the ACL that allows organizations to deposit `VerifiableCredentials` to a DID contract
use soroban_sdk::{panic_with_error, vec, Env, String, Vec};

use crate::error::ContractError;

use super::storage_types::DataKey;

const ACL_KEY: DataKey = DataKey::AccessControlList;

pub(crate) fn get_access_control_list(env: &Env) -> Vec<String> {
    match env.storage().get(&ACL_KEY) {
        Some(acl) => acl.unwrap(),
        None => {
            panic_with_error!(env, ContractError::NoOrganizationsInACL)
        }
    }
}

pub(crate) fn add_organization(env: &Env, org_id: &String) {
    let acl = match env.storage().get(&ACL_KEY) {
        Some(acl) => {
            let mut access_list: Vec<String> = acl.unwrap();
            if !is_organization_in_access_control_list(org_id, &access_list) {
                access_list.push_front(org_id.clone());
                access_list
            } else {
                panic_with_error!(env, ContractError::AlreadyInACL)
            }
        }
        None => {
            let access_list: Vec<String> = vec![env, org_id.clone()];
            access_list
        }
    };
    env.storage().set(&ACL_KEY, &acl)
}

pub(crate) fn remove_organization(env: &Env, org_id: &String) {
    match env.storage().get(&ACL_KEY) {
        Some(acl) => {
            let mut access_list: Vec<String> = acl.unwrap();
            remove_from_access_control_list(env, org_id, &mut access_list);
            env.storage().set(&ACL_KEY, &access_list)
        }
        None => {
            panic_with_error!(env, ContractError::NoOrganizationsInACL)
        }
    }
}

pub(crate) fn check_access_control_list(env: &Env, org_id: &String) {
    match env.storage().get(&ACL_KEY) {
        Some(acl) => {
            let access_list: Vec<String> = acl.unwrap();
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

fn remove_from_access_control_list(env: &Env, org_id: &String, access_list: &mut Vec<String>) {
    for (index, org) in access_list.iter().enumerate() {
        if org.unwrap() == org_id.clone() {
            access_list.remove(index as u32).unwrap();
            return;
        }
    }
    panic_with_error!(env, ContractError::OrganizationNotFound)
}

fn is_organization_in_access_control_list(org_id: &String, access_list: &Vec<String>) -> bool {
    for org in access_list.iter() {
        if org.unwrap() == org_id.clone() {
            return true;
        }
    }
    false
}
