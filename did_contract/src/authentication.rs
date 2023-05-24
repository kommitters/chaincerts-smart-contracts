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
    fn new(env: &Env, id: String, blockchain_account_id: Address, controller: String) -> Self {
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

    let authentication = vec![env, key_id.clone()];

    env.storage().set(&AUTHENTICATION_KEY, &authentication);

    write_verification_method(env, verification_method);
}

pub(crate) fn read_authentication(env: &Env) -> Vec<String> {
    env.storage().get_unchecked(&AUTHENTICATION_KEY).unwrap()
}

pub(crate) fn write_verification_method(env: &Env, verification_method: VerificationMethod) {
    let verification_methods: Vec<VerificationMethod> = vec![env, verification_method];

    env.storage().set(&VER_METHODS_KEY, &verification_methods);
}

pub(crate) fn read_verification_method(env: &Env) -> Vec<VerificationMethod> {
    env.storage().get_unchecked(&VER_METHODS_KEY).unwrap()
}

pub(crate) fn check_authentication(env: &Env, address: &Address) -> bool {
    let verification_methods: Vec<VerificationMethod> =
        env.storage().get_unchecked(&VER_METHODS_KEY).unwrap();

    verification_methods.iter().any(|verification_method| {
        let verification_method = verification_method.unwrap();
        verification_method.blockchain_account_id == address.clone()
    })
}

pub(crate) fn check_invocation_address(env: &Env, address: &Address) {
    if !check_authentication(env, address) {
        panic_with_error!(env, ContractError::NotAuthorized);
    }
}
