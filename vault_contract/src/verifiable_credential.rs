use crate::storage;
use soroban_sdk::{contracttype, Address, Env, String, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiableCredential {
    pub id: String,
    pub data: String,
    pub issuance_contract: Address,
    pub issuer_did: String,
}

pub fn store_vc(e: &Env, id: String, data: String, issuance_contract: Address, issuer_did: String) {
    let mut vcs: Vec<VerifiableCredential> = storage::read_vcs(e);
    let new_vc: VerifiableCredential = VerifiableCredential {
        id,
        data,
        issuance_contract,
        issuer_did,
    };

    vcs.push_front(new_vc);
    storage::write_vcs(e, &vcs);
}
