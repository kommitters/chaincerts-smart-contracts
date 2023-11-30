use crate::error::ContractError;
use crate::revocation::Revocation;
use crate::storage;
use crate::vault_contract;
use crate::vc_issuance_trait::VCIssuanceTrait;
use crate::verifiable_credential;
use soroban_sdk::{
    contract, contractimpl, contractmeta, map, panic_with_error, Address, Env, Map, String, Vec,
};

const LEDGERS_THRESHOLD: u32 = 1;
const LEDGERS_TO_EXTEND: u32 = 535_000;
const DEFAULT_AMOUNT: u32 = 20;
const MAX_AMOUNT: u32 = 100;

contractmeta!(
    key = "Description",
    val = "Smart Contract to issue, transfer, verify, and revoke Verifiable Credentials (VCs).",
);

#[contract]
pub struct VCIssuanceContract;

#[contractimpl]
impl VCIssuanceTrait for VCIssuanceContract {
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
        storage::write_vcs_revocations(&e, &Map::new(&e));

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
        let contract_address = e.current_contract_address();

        let client = vault_contract::Client::new(&e, &storage_address);
        client.store_vc(&vc_id, &vc_data, &recipient_did, &admin, &contract_address);
        verifiable_credential::add_vc(&e, &vc_id);

        vc_id
    }

    fn verify(e: Env, vc_id: String) -> Map<String, String> {
        validate_vc(&e, &vc_id);
        let revocations = storage::read_vcs_revocations(&e);

        let status_str = String::from_slice(&e, "status");
        let since_str = String::from_slice(&e, "since");
        let revoked_str = String::from_slice(&e, "revoked");
        let valid_str = String::from_slice(&e, "valid");

        match revocations.get(vc_id) {
            Some(revocation) => map![&e, (status_str, revoked_str), (since_str, revocation.date)],
            None => map![&e, (status_str, valid_str)],
        }
    }

    fn revoke(e: Env, admin: Address, vc_id: String, date: String) {
        validate_admin(&e, &admin);
        validate_vc(&e, &vc_id);

        let mut revocations = storage::read_vcs_revocations(&e);

        revocations.set(
            vc_id.clone(),
            Revocation {
                vc_id: vc_id.clone(),
                date,
            },
        );

        storage::write_vcs_revocations(&e, &revocations);
    }
}

fn validate_admin(e: &Env, admin: &Address) {
    let contract_admin = storage::read_admin(e);
    if contract_admin != admin.clone() {
        panic_with_error!(e, ContractError::NotAuthorized)
    }
    admin.require_auth();
}

fn validate_vc(e: &Env, vc_id: &String) {
    let vcs = storage::read_vcs(e);

    if !vcs.contains(vc_id) {
        panic_with_error!(e, ContractError::VCNotFound)
    }
}
