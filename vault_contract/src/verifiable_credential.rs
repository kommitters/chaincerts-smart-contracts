use crate::storage;
use soroban_sdk::{contracttype, Address, Env, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiableCredential {
    pub id: String,
    pub data: String,
    pub issuance_contract: Address,
    pub issuer_did: String,
}

pub fn store_vc(e: &Env, id: String, data: String, issuance_contract: Address, issuer_did: String) {
    let new_vc: VerifiableCredential = VerifiableCredential {
        id: id.clone(),
        data,
        issuance_contract,
        issuer_did,
    };

    storage::write_vc(e, &id, &new_vc);
}
