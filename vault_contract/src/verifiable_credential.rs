use crate::storage;
use crate::vault::Vault;
use soroban_sdk::{contracttype, Address, Env, Map, String, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiableCredential {
    pub id: String,
    pub data: String,
    pub issuance_contract: Address,
}

pub fn store_vc(
    e: &Env,
    vaults: &mut Map<String, Vault>,
    id: String,
    data: String,
    issuance_contract: Address,
    recipient_did: String,
) {
    let mut vcs: Vec<VerifiableCredential> = vaults.get_unchecked(recipient_did.clone()).vcs;
    let new_vc: VerifiableCredential = VerifiableCredential {
        id,
        data,
        issuance_contract,
    };

    vcs.push_back(new_vc);

    vaults.set(
        recipient_did.clone(),
        Vault {
            did: recipient_did,
            revoked: false,
            vcs,
        },
    );

    storage::write_vaults(e, vaults);
}
