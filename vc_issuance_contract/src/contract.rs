use crate::storage;
use crate::vc_issuance_trait::VCIssuanceTrait;
use crate::verifiable_credential::VCStatus;
use crate::{error::ContractError, verifiable_credential};
use soroban_sdk::{
    contract, contractimpl, contractmeta, map, panic_with_error, vec, Address, BytesN, Env,
    FromVal, Map, String, Symbol, Val,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

contractmeta!(
    key = "Description",
    val = "Smart Contract to issue, transfer, verify, and revoke Verifiable Credentials (VCs).",
);

#[contract]
pub struct VCIssuanceContract;

#[contractimpl]
impl VCIssuanceTrait for VCIssuanceContract {
    fn initialize(e: Env, admin: Address, issuer_did: String) {
        if storage::has_admin(&e) {
            panic_with_error!(e, ContractError::AlreadyInitialized);
        }

        storage::write_admin(&e, &admin);
        storage::write_issuer_did(&e, &issuer_did);
    }

    fn issue(e: Env, vc_id: String, vc_data: String, vault_contract: Address) -> String {
        let admin = validate_admin(&e);

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
        storage::write_vc(&e, &vc_id, &VCStatus::Valid);

        vc_id
    }

    fn verify(e: Env, vc_id: String) -> Map<String, String> {
        let vc_status = storage::read_vc(&e, &vc_id);

        let status_str = String::from_str(&e, "status");
        let since_str = String::from_str(&e, "since");
        let revoked_str = String::from_str(&e, "revoked");
        let valid_str = String::from_str(&e, "valid");
        let invalid_str = String::from_str(&e, "invalid");

        match vc_status {
            VCStatus::Invalid => map![&e, (status_str, invalid_str)],
            VCStatus::Valid => map![&e, (status_str, valid_str)],
            VCStatus::Revoked(revocation_date) => map![&e, (status_str, revoked_str), (since_str, revocation_date)]
        }
    }

    fn revoke(e: Env, vc_id: String, date: String) {
        validate_admin(&e);
        validate_vc(&e, &vc_id);

        verifiable_credential::revoke_vc(&e, vc_id, date);
    }

    fn upgrade(e: Env, new_wasm_hash: BytesN<32>) {
        let admin = storage::read_admin(&e);
        admin.require_auth();

        e.deployer().update_current_contract_wasm(new_wasm_hash);
    }

    fn set_admin(e: Env, new_admin: Address) {
        validate_admin(&e);

        storage::write_admin(&e, &new_admin);
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

fn validate_vc(e: &Env, vc_id: &String) {
    let vc_status = storage::read_vc(e, vc_id);

    if vc_status == VCStatus::Invalid {
        panic_with_error!(e, ContractError::VCNotFound)
    }
}
