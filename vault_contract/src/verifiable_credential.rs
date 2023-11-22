use soroban_sdk::{contracttype, Address, Env, String};

use crate::storage;

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
    let vc = VerifiableCredential {
        id: vc_id.clone(),
        data: vc_data.clone(),
        holder_did: recipient_did.clone(),
        issuance_contract: issuance_contract.clone(),
    };
    let mut vcs = storage::read_vcs(e);
    let mut dids = storage::read_dids(e);
    let mut did = dids.get_unchecked(recipient_did.clone());

    did.vcs.push_back(vc_id.clone());
    dids.set(recipient_did.clone(), did);
    storage::write_dids(e, &dids);

    vcs.set(vc_id.clone(), vc);
    storage::write_vcs(e, &vcs);
}
