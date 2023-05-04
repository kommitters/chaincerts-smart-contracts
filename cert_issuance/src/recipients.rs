//! Module Recipients
//!
//! Module to create and read the contract recipients.
use soroban_sdk::{map, Address, Bytes, Env, Map, Vec};
use uuid::Uuid;

use crate::{
    cert_wallet::OptionU64,
    storage_types::{CertData, DataKey, Status},
};

pub fn read_recipients(e: &Env) -> Map<Address, CertData> {
    let key = DataKey::Recipients;
    e.storage().get_unchecked(&key).unwrap()
}

pub fn create_recipients(e: &Env, recipients_address: Vec<Address>) {
    let mut recipients: Map<Address, CertData> = map![e];
    recipients_address.iter().for_each(|recipient| {
        let address: Address = recipient.unwrap();
        let id_cert = create_unique_id(e, &address);
        let chaincert_data = CertData::new(id_cert, Status::Unassigned, OptionU64::None);
        recipients.set(address, chaincert_data);
    });
    let key = DataKey::Recipients;
    e.storage().set(&key, &recipients)
}

pub fn add_recipient(e: &Env, address: &Address) {
    let mut recipients = read_recipients(e);
    let id_cert = create_unique_id(e, address);
    let cert_data = CertData::new(id_cert, Status::Unassigned, OptionU64::None);
    recipients.set(address.clone(), cert_data);
    let key = DataKey::Recipients;
    e.storage().set(&key, &recipients);
}

fn create_unique_id(e: &Env, address: &Address) -> Bytes {
    let contract_address_bytes = e.current_contract_address().to_raw().get_payload();
    let recipient_bytes = address.to_raw().get_payload();
    let uniq_bytes = (contract_address_bytes + recipient_bytes).to_be_bytes();
    let uuid = Uuid::new_v5(&Uuid::NAMESPACE_DNS, &uniq_bytes);
    let id_cert: Bytes = Bytes::from_slice(e, uuid.as_bytes());
    id_cert
}
