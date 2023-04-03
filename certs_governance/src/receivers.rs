//! Module Receivers
//!
//! Module to create and read the contract receivers.
use soroban_sdk::{map, Address, Bytes, Env, Map, Vec};
use uuid::Uuid;

use crate::storage_types::{CertData, DataKey, Opt, Status};

pub fn read_receivers(e: &Env) -> Map<Address, CertData> {
    let key = DataKey::Receivers;
    e.storage().get_unchecked(&key).unwrap()
}

pub fn create_receivers(e: &Env, receivers_address: Vec<Address>) {
    let mut receivers: Map<Address, CertData> = map![e];
    receivers_address.iter().for_each(|receiver| {
        let address: Address = receiver.unwrap();
        let id_cert = create_unique_id(e, &address);
        let chaincert_data = CertData::new(id_cert, Status::Unassigned, Opt::None);
        receivers.set(address, chaincert_data);
    });
    let key = DataKey::Receivers;
    e.storage().set(&key, &receivers)
}

pub fn add_receiver(e: &Env, address: &Address) {
    let mut receivers = read_receivers(e);
    let id_cert = create_unique_id(e, address);
    let cert_data = CertData::new(id_cert, Status::Unassigned, Opt::None);
    receivers.set(address.clone(), cert_data);
    let key = DataKey::Receivers;
    e.storage().set(&key, &receivers);
}

fn create_unique_id(e: &Env, address: &Address) -> Bytes {
    let contract_address_bytes = e.current_contract_address().to_raw().get_payload();
    let receiver_bytes = address.to_raw().get_payload();
    let uniq_bytes = (contract_address_bytes + receiver_bytes).to_be_bytes();
    let uuid = Uuid::new_v5(&Uuid::NAMESPACE_DNS, &uniq_bytes);
    let id_cert: Bytes = Bytes::from_slice(e, uuid.as_bytes());
    id_cert
}
