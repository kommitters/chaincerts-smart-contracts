//! Module MetaData
//!
//! Module for obtaining and modifying the metadata fields.
use crate::storage_types::{CertData, DataKey, Opt, Status};
use soroban_sdk::{map, Address, Bytes, Env, Map, Vec};
use uuid::Uuid;

pub fn read_file_storage(e: &Env) -> Bytes {
    let key = DataKey::FStorage;
    e.storage().get_unchecked(&key).unwrap()
}

pub fn write_file_storage(e: &Env, d: Bytes) {
    let key = DataKey::FStorage;
    e.storage().set(&key, &d);
}

pub fn read_name(e: &Env) -> Bytes {
    let key = DataKey::Name;
    e.storage().get_unchecked(&key).unwrap()
}

pub fn write_name(e: &Env, d: Bytes) {
    let key = DataKey::Name;
    e.storage().set(&key, &d)
}

pub fn read_revocable(e: &Env) -> bool {
    let key = DataKey::Revocable;
    e.storage().get_unchecked(&key).unwrap()
}

pub fn write_revocable(e: &Env, d: bool) {
    let key = DataKey::Revocable;
    e.storage().set(&key, &d)
}

pub fn read_expiration_time(e: &Env) -> Option<u64> {
    let key = DataKey::ExpTime;
    e.storage().get_unchecked(&key).unwrap()
}

pub fn write_expiration_time(e: &Env, d: Option<u64>) {
    let key = DataKey::ExpTime;
    e.storage().set(&key, &d)
}

pub fn create_receivers(e: &Env, d: Vec<Address>) {
    let mut receivers: Map<Address, CertData> = map![e];
    d.iter().for_each(|receiver| {
        let address: Address = receiver.unwrap();
        let bytes = address.to_raw().get_payload().to_be_bytes();
        let uuid = Uuid::new_v5(&Uuid::NAMESPACE_DNS, &bytes);
        let id_cert = Bytes::from_slice(e, uuid.as_bytes());
        let chaincert_data = CertData::new(id_cert, Status::Unassigned, Opt::None);
        receivers.set(address, chaincert_data);
    });
    let key = DataKey::Receivers;
    e.storage().set(&key, &receivers)
}

pub fn write_receivers(e: &Env, d: Map<Address, CertData>) {
    let key = DataKey::Receivers;
    e.storage().set(&key, &d)
}

pub fn read_distribution_limit(e: &Env) -> u32 {
    let key = DataKey::DistLimit;
    e.storage().get_unchecked(&key).unwrap()
}

pub fn write_distribution_limit(e: &Env, d: u32) {
    let key = DataKey::DistLimit;
    e.storage().set(&key, &d)
}
