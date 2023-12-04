use crate::storage;
use crate::vault::Vault;
use soroban_sdk::{contracttype, Address, Env, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiableCredential {
    pub id: String,
    pub data: String,
    pub holder_did: String,
    pub issuance_contract: Address,
}

pub fn store_vc(
    e: &Env,
    vc_id: &String,
    vc_data: &String,
    issuance_contract: &Address,
    recipient_did: &String,
) {
    let mut vaults = storage::read_vaults(e);
    let mut vcs = vaults.get_unchecked(recipient_did.clone()).vcs;
    let new_vc = VerifiableCredential {
        id: vc_id.clone(),
        data: vc_data.clone(),
        holder_did: recipient_did.clone(),
        issuance_contract: issuance_contract.clone(),
    };

    vcs.push_back(new_vc);

    vaults.set(
        recipient_did.clone(),
        Vault {
            did: recipient_did.clone(),
            revoked: false,
            vcs,
        },
    );

    storage::write_vaults(e, &vaults);
}
