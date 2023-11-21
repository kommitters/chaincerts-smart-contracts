use crate::did;
use crate::error::ContractError;
use crate::issuer;
use crate::storage;

use crate::vault_trait::VaultTrait;
use soroban_sdk::{
    contract, contractimpl, contractmeta, panic_with_error, Address, Env, String, Vec,
};

const LEDGERS_THRESHOLD: u32 = 1;
const LEDGERS_TO_EXTEND: u32 = 535_000;

contractmeta!(key = "Description", val = "Smart contract for Vault",);

#[contract]
pub struct VaultContract;

#[contractimpl]
impl VaultTrait for VaultContract {
    fn initialize(e: Env, admin: Address, dids: Vec<String>) {
        if storage::has_admin(&e) {
            panic_with_error!(e, ContractError::AlreadyInitialized);
        }
        storage::write_admin(&e, &admin);

        did::set_initial_dids(&e, &dids);

        e.storage()
            .instance()
            .bump(LEDGERS_THRESHOLD, LEDGERS_TO_EXTEND);
    }

    fn authorize_issuer(e: Env, admin: Address, issuer: Address, did: String) {
        let contract_admin = storage::read_admin(&e);
        if contract_admin != admin {
            panic_with_error!(e, ContractError::NotAuthorized)
        }
        admin.require_auth();

        issuer::authorize_issuer(&e, &issuer, &did);
    }

    fn revoke_issuer(e: Env, admin: Address, issuer: Address, did: String) {
        let contract_admin = storage::read_admin(&e);
        if contract_admin != admin {
            panic_with_error!(e, ContractError::NotAuthorized)
        }
        admin.require_auth();

        issuer::revoke_issuer(&e, &issuer, &did)
    }
}
