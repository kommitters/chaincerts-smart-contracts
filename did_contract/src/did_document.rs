//! Module DIDDocument
//!
//! Module responsible of managing the DID document data.
use crate::{
    authentication::{read_authentication, read_verification_method, VerificationMethod},
    option::OptionMethodService,
    storage_types::DataKey,
};
use soroban_sdk::{contracttype, Env, String, Vec};

#[derive(Clone, PartialEq, Eq, Debug)]
#[contracttype]
/// `DIDDocument` public info
pub struct DIDDocument {
    pub context: Vec<String>,
    pub id: String,
    pub verification_method: Vec<VerificationMethod>,
    pub authentication: Vec<String>,
    pub services: Vec<Service>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
#[contracttype]
/// The verification process `Method`
pub struct Method {
    pub type_: String,
    pub verified: bool,
    /// Unix timestamp
    pub timestamp: u64,
    pub service: OptionMethodService,
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug)]
#[contracttype]
/// The verification process `MethodService`
pub struct MethodService {
    pub name: String,
    pub url: String,
    pub proof: String,
}

#[derive(Clone, PartialEq, Eq, Debug)]
#[contracttype]
/// The DID `Service` information
pub struct Service {
    pub type_: String,
    pub service_endpoint: String,
}

pub(crate) fn read_id(env: &Env) -> String {
    let key: DataKey = DataKey::Id;
    env.storage().get_unchecked(&key).unwrap()
}

pub(crate) fn write_id(env: &Env, id: &String) {
    let key: DataKey = DataKey::Id;
    env.storage().set(&key, id);
}

pub(crate) fn write_context(env: &Env, context: &Vec<String>) {
    let key: DataKey = DataKey::Context;
    env.storage().set(&key, context);
}

pub(crate) fn read_context(env: &Env) -> Vec<String> {
    let key: DataKey = DataKey::Context;
    env.storage().get_unchecked(&key).unwrap()
}

pub(crate) fn write_verification_processes(env: &Env, verification_processes: &Vec<Method>) {
    let key: DataKey = DataKey::VerificationProcesses;
    env.storage().set(&key, verification_processes);
}

pub(crate) fn write_services(env: &Env, services: &Vec<Service>) {
    let key: DataKey = DataKey::Services;
    env.storage().set(&key, services);
}

pub(crate) fn read_services(env: &Env) -> Vec<Service> {
    let key: DataKey = DataKey::Services;
    env.storage().get_unchecked(&key).unwrap()
}

pub(crate) fn retrieve_public_did_document(env: &Env) -> DIDDocument {
    let context = read_context(env);
    let id = read_id(env);
    let verification_method = read_verification_method(env);
    let authentication = read_authentication(env);
    let services = read_services(env);

    DIDDocument {
        context,
        id,
        verification_method,
        authentication,
        services,
    }
}
