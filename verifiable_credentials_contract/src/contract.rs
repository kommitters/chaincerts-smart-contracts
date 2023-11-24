use crate::storage;
use crate::vault_contract;
use crate::vcs_trait::VCsTrait;
use crate::{error::ContractError, verifiable_credential};
use soroban_sdk::{
    contract, contractimpl, contractmeta, panic_with_error, Address, Env, Map, String, Vec,
};

const LEDGERS_THRESHOLD: u32 = 1;
const LEDGERS_TO_EXTEND: u32 = 535_000;
const DEFAULT_AMOUNT: u32 = 20;
const MAX_AMOUNT: u32 = 100;

contractmeta!(
    key = "Description",
    val = "Smart contract for Verifiable Credentials",
);

#[contract]
pub struct VCsContract;

#[contractimpl]
impl VCsTrait for VCsContract {
    fn initialize(e: Env, admin: Address, amount: Option<u32>) {
        if storage::has_admin(&e) {
            panic_with_error!(e, ContractError::AlreadyInitialized);
        }
        if amount.map_or(false, |a| a > MAX_AMOUNT) {
            panic_with_error!(e, ContractError::AmountLimitExceeded);
        }

        storage::write_admin(&e, &admin);
        storage::write_amount(&e, &amount.unwrap_or(DEFAULT_AMOUNT));

        // set initial empty values
        storage::write_vcs(&e, &Vec::new(&e));
        storage::write_revocations(&e, &Map::new(&e));

        e.storage()
            .instance()
            .bump(LEDGERS_THRESHOLD, LEDGERS_TO_EXTEND);
    }

    fn issue(
        e: Env,
        admin: Address,
        vc_data: String,
        recipient_did: String,
        storage_address: Address,
    ) -> String {
        validate_admin(&e, &admin);

        let vc_id = verifiable_credential::generate_id(&e);

        let client = vault_contract::Client::new(&e, &storage_address);
        client.store_vc(&vc_id, &vc_data, &recipient_did, &admin, &storage_address);
        verifiable_credential::add_vc(&e, &vc_id);

        vc_id
    }
}

fn validate_admin(e: &Env, admin: &Address) {
    let contract_admin = storage::read_admin(e);
    if contract_admin != admin.clone() {
        panic_with_error!(e, ContractError::NotAuthorized)
    }
    admin.require_auth();
}
