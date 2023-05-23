//! Module Owner
//!
//! Module responsible of managing the wallet owner information.
use soroban_sdk::{contracttype, panic_with_error, Address, Env, String, Vec};

use crate::{error::ContractError, storage_types::DataKey};

#[derive(Clone, PartialEq, Eq, Debug)]
#[contracttype]
/// The DID `Service` information
pub struct Owner {
    pub authentications: Vec<String>,
    pub verification_methods: Vec<VerificationMethod>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
#[contracttype]
/// The DID `VerificationMethod` information
pub struct VerificationMethod {
    pub id: String,
    pub verification_method_type: String,
    pub controller: String,
    pub blockchain_account_id: Address,
}

const OWNER_KEY: DataKey = DataKey::Owner;

pub(crate) fn has_owner(env: &Env) -> bool {
    env.storage().has(&OWNER_KEY)
}

#[cfg(not(tarpaulin_include))]
pub(crate) fn _read_owner(env: &Env) -> Owner {
    env.storage().get_unchecked(&OWNER_KEY).unwrap()
}

pub(crate) fn write_owner(env: &Env, owner: &Owner) {
    env.storage().set(&OWNER_KEY, owner);
}

pub(crate) fn check_owner(env: &Env, address: &Address) -> bool {
    let owner: Owner = env.storage().get_unchecked(&OWNER_KEY).unwrap();
    let verification_methods = owner.verification_methods;
    verification_methods.iter().any(|verification_method| {
        let verification_method = verification_method.unwrap();
        verification_method.blockchain_account_id == address.clone()
    })
}

pub(crate) fn check_invocation_address(env: &Env, address: &Address) {
    if !check_owner(env, address) {
        panic_with_error!(env, ContractError::NotAuthorized);
    }
}
