//! Module MetaData
//!
//! Module for obtaining and modifying the metadata fields.
use crate::{
    storage_types::{CredentialData, DataKey, RevokedCredential},
};
use soroban_sdk::{Bytes, Env, Map, String};

pub fn read_file_storage(e: &Env) -> Bytes {
    let key = DataKey::FileStorage;
    e.storage().get_unchecked(&key).unwrap()
}

pub fn write_file_storage(e: &Env, file_storage: Bytes) {
    let key = DataKey::FileStorage;
    e.storage().set(&key, &file_storage);
}

pub fn read_name(e: &Env) -> Bytes {
    let key = DataKey::Name;
    e.storage().get_unchecked(&key).unwrap()
}

pub fn write_name(e: &Env, name: Bytes) {
    let key = DataKey::Name;
    e.storage().set(&key, &name)
}

pub fn read_revocable(e: &Env) -> bool {
    let key = DataKey::Revocable;
    e.storage().get_unchecked(&key).unwrap()
}

pub fn write_revocable(e: &Env, revocable: bool) {
    let key = DataKey::Revocable;
    e.storage().set(&key, &revocable)
}

pub fn read_revoked_credentials(e: &Env) -> Map<String, RevokedCredential> {
    let key = DataKey::RevokedCredentials;
    e.storage().get_unchecked(&key).unwrap()
}

pub fn write_revoked_credentials(e: &Env, revoked_credentials: Map<String, RevokedCredential>) {
    let key = DataKey::RevokedCredentials;
    e.storage().set(&key, &revoked_credentials)
}

pub fn write_recipients(e: &Env, recipients: Map<String, Option<CredentialData>>) {
    let key = DataKey::Recipients;
    e.storage().set(&key, &recipients)
}

pub fn read_distribution_limit(e: &Env) -> u32 {
    let key = DataKey::DistributionLimit;
    e.storage().get_unchecked(&key).unwrap()
}

pub fn write_distribution_limit(e: &Env, distribution_limit: u32) {
    let key = DataKey::DistributionLimit;
    e.storage().set(&key, &distribution_limit)
}

pub fn write_supply(e: &Env, supply: u32) {
    let key = DataKey::Supply;
    e.storage().set(&key, &supply)
}

pub fn read_supply(e: &Env) -> u32 {
    let key = DataKey::Supply;
    e.storage().get_unchecked(&key).unwrap()
}

pub fn increment_supply(e: &Env) {
    write_supply(e, read_supply(e) + 1);
}

pub fn write_credential_title(e: &Env, credential_title: String) {
    let key = DataKey::CredentialTitle;
    e.storage().set(&key, &credential_title);
}

pub fn read_credential_title(e: &Env) -> String {
    let key = DataKey::CredentialTitle;
    e.storage().get_unchecked(&key).unwrap()
}

pub fn write_credential_type(e: &Env, credential_type: String) {
    let key = DataKey::CredentialType;
    e.storage().set(&key, &credential_type);
}

pub fn read_credential_type(e: &Env) -> String {
    let key = DataKey::CredentialType;
    e.storage().get_unchecked(&key).unwrap()
}
