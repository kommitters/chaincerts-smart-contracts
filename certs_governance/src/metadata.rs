//! Module MetaData
//!
//! Module for obtaining and modifying the metadata fields.
use crate::storage_types::{CertData, DataKey};
use soroban_sdk::{Address, Bytes, Env, Map};

pub fn read_file_storage(e: &Env) -> Bytes {
    let key = DataKey::FStorage;
    e.storage().get_unchecked(&key).unwrap()
}

pub fn write_file_storage(e: &Env, file_storage: Bytes) {
    let key = DataKey::FStorage;
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

pub fn read_expiration_time(e: &Env) -> Option<u64> {
    let key = DataKey::ExpTime;
    e.storage().get_unchecked(&key).unwrap()
}

pub fn write_expiration_time(e: &Env, expiration_time: Option<u64>) {
    let key = DataKey::ExpTime;
    e.storage().set(&key, &expiration_time)
}

pub fn write_receivers(e: &Env, receivers: Map<Address, CertData>) {
    let key = DataKey::Receivers;
    e.storage().set(&key, &receivers)
}

pub fn read_distribution_limit(e: &Env) -> u32 {
    let key = DataKey::DistLimit;
    e.storage().get_unchecked(&key).unwrap()
}

pub fn write_distribution_limit(e: &Env, distribution_limit: u32) {
    let key = DataKey::DistLimit;
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
