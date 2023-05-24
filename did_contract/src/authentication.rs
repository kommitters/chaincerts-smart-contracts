//! Module Authentication
//!
//! Module responsible of managing the DID contract authentication information.
use soroban_sdk::{contracttype, panic_with_error, vec, Address, Env, String, Vec};

use crate::{did_document, error::ContractError, storage_types::DataKey};

const AUTHENTICATION_KEY: DataKey = DataKey::Authentication;
const VER_METHODS_KEY: DataKey = DataKey::VerificationMethods;

#[derive(Clone, PartialEq, Eq, Debug)]
#[contracttype]
/// The DID `VerificationMethod` information
pub struct VerificationMethod {
    pub id: String,
    pub verification_method_type: String,
    pub controller: String,
    pub blockchain_account_id: Address,
}

impl VerificationMethod {
    pub fn new(env: &Env, id: String, blockchain_account_id: Address, controller: String) -> Self {
        VerificationMethod {
            id,
            verification_method_type: String::from_slice(env, "Ed25519VerificationKey2020"),
            controller,
            blockchain_account_id,
        }
    }
}

pub(crate) fn has_authentication(env: &Env) -> bool {
    env.storage().has(&AUTHENTICATION_KEY)
}

pub(crate) fn write_authentication(env: &Env, key_id: &String, address: &Address) {
    let controller = did_document::read_id(env);
    let verification_method =
        VerificationMethod::new(env, key_id.clone(), address.clone(), controller);

    let authentication = if has_authentication(env) {
        let mut authentications: Vec<String> =
            env.storage().get_unchecked(&AUTHENTICATION_KEY).unwrap();
        authentications.push_front(key_id.clone());
        authentications
    } else {
        vec![env, key_id.clone()]
    };

    env.storage().set(&AUTHENTICATION_KEY, &authentication);

    write_verification_method(env, verification_method);
}

pub(crate) fn remove_authentication(env: &Env, key_id: &String) {
    let mut authentications: Vec<String> =
        env.storage().get_unchecked(&AUTHENTICATION_KEY).unwrap();
    if authentications.len() <= 1 {
        panic_with_error!(env, ContractError::CannotRemoveAuthentication);
    }

    let key_id = key_id.clone();
    let index = authentications
        .iter()
        .position(|key| key.unwrap() == key_id);
    match index {
        Some(val) => {
            authentications.remove(val as u32);
            env.storage().set(&AUTHENTICATION_KEY, &authentications);
        }
        None => {
            panic_with_error!(env, ContractError::CannotRemoveAuthentication);
        }
    }
}

pub(crate) fn read_authentication(env: &Env) -> Vec<String> {
    env.storage().get_unchecked(&AUTHENTICATION_KEY).unwrap()
}

pub(crate) fn write_verification_method(env: &Env, verification_method: VerificationMethod) {
    let verification_methods = if env.storage().has(&VER_METHODS_KEY) {
        let mut verification_methods: Vec<VerificationMethod> =
            env.storage().get_unchecked(&VER_METHODS_KEY).unwrap();
        verification_methods.push_front(verification_method);
        verification_methods
    } else {
        vec![env, verification_method]
    };

    env.storage().set(&VER_METHODS_KEY, &verification_methods);
}

pub(crate) fn remove_verification_method(env: &Env, key_id: &String) {
    let key_id = key_id.clone();
    let mut verification_methods: Vec<VerificationMethod> =
        env.storage().get_unchecked(&VER_METHODS_KEY).unwrap();
    if verification_methods.len() <= 1 {
        panic_with_error!(env, ContractError::CannotRemoveVerificationMethod);
    }
    let index_option = verification_methods
        .iter()
        .position(|verification_method| verification_method.unwrap().id == key_id);
    match index_option {
        Some(index) => {
            // Check if the verification method is used in the authentication.
            // If so, remove it.
            let mut authentication = read_authentication(env);
            if let Some(index) = authentication
                .iter()
                .position(|auth| auth.unwrap() == key_id)
            {
                authentication.remove(index as u32);
                env.storage().set(&AUTHENTICATION_KEY, &authentication);
            };

            verification_methods.remove(index as u32);
            env.storage().set(&VER_METHODS_KEY, &verification_methods);
        }
        None => {
            panic_with_error!(env, ContractError::CannotRemoveVerificationMethod);
        }
    }
}

pub(crate) fn read_verification_method(env: &Env) -> Vec<VerificationMethod> {
    env.storage().get_unchecked(&VER_METHODS_KEY).unwrap()
}

pub(crate) fn check_authentication(env: &Env, address: &Address) -> bool {
    let verification_methods: Vec<VerificationMethod> =
        env.storage().get_unchecked(&VER_METHODS_KEY).unwrap();
    let authentication = read_authentication(env);

    verification_methods.iter().any(|verification_method| {
        let verification_method = verification_method.unwrap();
        verification_method.blockchain_account_id == address.clone()
            && authentication.contains(verification_method.id)
    })
}

pub(crate) fn check_invocation_address(env: &Env, address: &Address) {
    if !check_authentication(env, address) {
        panic_with_error!(env, ContractError::NotAuthorized);
    }
}
