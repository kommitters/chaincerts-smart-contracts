//! Module DIDDocument
//!
//! Module responsible of managing the DID document data.
use crate::storage_types::DataKey;
use soroban_sdk::{contracttype, Env, String, Symbol, Vec};

#[derive(Clone, PartialEq, Eq, Debug)]
#[contracttype]
/// The `Method` verification process
pub struct Method {
    pub method_type: String,
    pub verified: bool,
    //Unix timestamp
    pub timestamp: u64,
}

#[derive(Clone, PartialEq, Eq, Debug)]
#[contracttype]
/// The DID `Service` information
pub struct Service {
    pub service_type: String,
    pub service_endpoint: String,
}

#[derive(Clone, PartialEq, Eq, Debug)]
#[contracttype]
/// The DID `Service` information
pub struct Metadata {
    /// Creation date in unix time stamp
    pub created: u64,
    /// Last update date in unix time stamp
    pub updated: u64,
    /// DID document version
    pub version: Symbol,
}

pub(crate) fn write_context(env: &Env, context: &Vec<String>) {
    let key: DataKey = DataKey::Context;
    env.storage().set(&key, context);
}

pub(crate) fn write_verification_processes(env: &Env, verification_processes: &Vec<Method>) {
    let key: DataKey = DataKey::VerificationProcesses;
    env.storage().set(&key, verification_processes);
}

pub(crate) fn write_services(env: &Env, services: &Vec<Service>) {
    let key: DataKey = DataKey::Services;
    env.storage().set(&key, services);
}

pub(crate) fn write_metadata(env: &Env, metadata: &Metadata) {
    let key: DataKey = DataKey::Metadata;
    env.storage().set(&key, metadata);
}
