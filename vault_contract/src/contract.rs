use crate::error::ContractError;
use crate::storage;
use crate::vault_trait::VaultTrait;
use soroban_sdk::{
    contract, contractimpl, contractmeta, panic_with_error, Address, Env, Map, String, Vec,
};

const LEDGERS_THRESHOLD: u32 = 1;
const LEDGERS_TO_EXTEND: u32 = 535_000;

contractmeta!(
    key = "Description",
    val = "Smart contract for decentralized identifiers (DIDs)",
);

#[contract]
pub struct VaultContract;

#[contractimpl]
impl VaultTrait for VaultContract {
    fn initialize(e: Env, admin: Address, dids: Vec<String>) {
        if storage::has_admin(&e) {
            panic_with_error!(e, ContractError::AlreadyInitialized);
        }
        storage::write_admin(&e, &admin);
        set_dids(&e, &dids);
        set_issuers(&e);

        e.storage()
            .instance()
            .bump(LEDGERS_THRESHOLD, LEDGERS_TO_EXTEND);
    }

    fn authorize_issuer(e: Env, admin: Address, issuer: Address) {
        let contract_admin = storage::read_admin(&e);
        if contract_admin != admin {
            panic_with_error!(e, ContractError::NotAuthorized)
        }
        admin.require_auth();
        storage::write_issuer(&e, &issuer);
    }

    fn revoke_issuer(e: Env, admin: Address, issuer: Address) {
        let contract_admin = storage::read_admin(&e);
        if contract_admin != admin {
            panic_with_error!(e, ContractError::NotAuthorized)
        }
        admin.require_auth();
        storage::revoke_issuer(&e, &issuer);
    }
}

fn set_dids(e: &Env, dids: &Vec<String>) {
    if dids.is_empty() {
        panic_with_error!(e, ContractError::EmptyDID);
    }
    storage::write_dids(e, dids);
}

fn set_issuers(e: &Env) {
    let issuers = Map::new(e);
    storage::write_issuers(e, &issuers);
}
