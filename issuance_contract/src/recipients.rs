//! Module Recipients
//!
//! Module to create and read the contract recipients.
use soroban_sdk::{map, Env, Map, String, Vec};

use crate::storage_types::{CredentialData, DataKey};

pub fn read_recipients(e: &Env) -> Map<String, Option<CredentialData>> {
    let key = DataKey::Recipients;
    e.storage().instance().get(&key).unwrap()
}

pub fn create_recipients(e: &Env, recipient_dids: Vec<String>) {
    let mut recipients: Map<String, Option<CredentialData>> = map![e];
    recipient_dids.iter().for_each(|recipient| {
        recipients.set(recipient, Option::None);
    });
    let key = DataKey::Recipients;
    e.storage().instance().set(&key, &recipients)
}

pub fn add_recipient(e: &Env, recipient_did: &String) {
    let key = DataKey::Recipients;
    let mut recipients = read_recipients(e);
    recipients.set(recipient_did.clone(), Option::None);
    e.storage().instance().set(&key, &recipients);
}
