use crate::error::ContractError;
use crate::storage;
use crate::verifiable_credential::VerifiableCredential;
use soroban_sdk::{contracttype, panic_with_error, Env, Map, String, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Vault {
    pub did: String,
    pub revoked: bool,
    pub vcs: Vec<VerifiableCredential>,
}

pub fn set_initial_vaults(e: &Env, dids: &Vec<String>) {
    if dids.is_empty() {
        panic_with_error!(e, ContractError::EmptyDIDs);
    }

    let mut vaults: Map<String, Vault> = Map::new(e);

    for did in dids.iter() {
        vaults.set(
            did.clone(),
            Vault {
                did: did.clone(),
                revoked: false,
                vcs: Vec::new(e),
            },
        )
    }

    storage::write_vaults(e, &vaults);
}

pub fn is_registered(vaults: &Map<String, Vault>, vault_did: &String) -> bool {
    vaults.contains_key(vault_did.clone())
}

pub fn is_revoked(vaults: &Map<String, Vault>, vault_did: &String) -> bool {
    vaults.get_unchecked(vault_did.clone()).revoked
}
