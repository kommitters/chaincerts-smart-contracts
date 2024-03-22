use crate::storage;
use crate::vc_issuance_trait::VCIssuanceTrait;
use crate::verifiable_credential;
use crate::{error::ContractError, revocation};
use soroban_sdk::{
    contract, contractimpl, contractmeta, map, panic_with_error, vec, Address, BytesN, Env,
    FromVal, Map, String, Symbol, Val, Vec,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const DEFAULT_AMOUNT: u32 = 20;
const MAX_AMOUNT: u32 = 200;

contractmeta!(
    key = "Description",
    val = "Smart Contract to issue, transfer, verify, and revoke Verifiable Credentials (VCs).",
);

#[contract]
pub struct VCIssuanceContract;

#[contractimpl]
impl VCIssuanceTrait for VCIssuanceContract {
    fn initialize(e: Env, admin: Address, issuer_did: String, amount: Option<u32>) {
        if storage::has_admin(&e) {
            panic_with_error!(e, ContractError::AlreadyInitialized);
        }
        if amount.is_some() && amount.unwrap() > MAX_AMOUNT {
            panic_with_error!(e, ContractError::AmountLimitExceeded);
        }

        storage::write_admin(&e, &admin);
        storage::write_issuer_did(&e, &issuer_did);
        storage::write_amount(&e, &amount.unwrap_or(DEFAULT_AMOUNT));

        // set initial empty values
        storage::write_vcs(&e, &Vec::new(&e));
        storage::write_vcs_revocations(&e, &Map::new(&e));

        storage::extend_ttl_to_instance(&e);
        storage::extend_ttl_to_persistent(&e);
    }

    fn issue(e: Env, vc_data: String, vault_contract: Address) -> String {
        let admin = validate_admin(&e);

        let vcs = storage::read_vcs(&e);
        validate_vc_amount(&e, &vcs);

        let vc_id = verifiable_credential::generate_id(&e);
        let contract_address = e.current_contract_address();
        let issuer_did = storage::read_issuer_did(&e);

        let store_vc_args = vec![
            &e,
            Val::from_val(&e, &vc_id),
            Val::from_val(&e, &vc_data),
            Val::from_val(&e, &admin),
            Val::from_val(&e, &issuer_did),
            Val::from_val(&e, &contract_address),
        ];

        let store_vc_fn = Symbol::new(&e, "store_vc");
        e.invoke_contract::<()>(&vault_contract, &store_vc_fn, store_vc_args);

        verifiable_credential::add_vc(&e, &vc_id, vcs);

        vc_id
    }

    fn verify(e: Env, vc_id: String) -> Map<String, String> {
        validate_vc(&e, &vc_id);
        let revocations = storage::read_vcs_revocations(&e);

        let status_str = String::from_str(&e, "status");
        let since_str = String::from_str(&e, "since");
        let revoked_str = String::from_str(&e, "revoked");
        let valid_str = String::from_str(&e, "valid");

        match revocations.get(vc_id) {
            Some(revocation) => map![&e, (status_str, revoked_str), (since_str, revocation.date)],
            None => map![&e, (status_str, valid_str)],
        }
    }

    fn revoke(e: Env, vc_id: String, date: String) {
        validate_admin(&e);
        validate_vc(&e, &vc_id);

        revocation::revoke_vc(&e, vc_id, date);
    }

    fn upgrade(e: Env, new_wasm_hash: BytesN<32>) {
        let admin = storage::read_admin(&e);
        admin.require_auth();

        e.deployer().update_current_contract_wasm(new_wasm_hash);
    }

    fn version(e: Env) -> String {
        String::from_str(&e, VERSION)
    }
}

fn validate_admin(e: &Env) -> Address {
    let contract_admin = storage::read_admin(e);
    contract_admin.require_auth();

    contract_admin
}

fn validate_vc_amount(e: &Env, vcs: &Vec<String>) {
    let amount = storage::read_amount(e);
    if amount == vcs.len() {
        panic_with_error!(e, ContractError::IssuanceLimitExceeded);
    }
}

fn validate_vc(e: &Env, vc_id: &String) {
    let vcs = storage::read_vcs(e);

    if !vcs.contains(vc_id) {
        panic_with_error!(e, ContractError::VCNotFound)
    }
}
