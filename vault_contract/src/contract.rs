use crate::did_contract::DIDDocument;
use crate::error::ContractError;
use crate::issuer;
use crate::storage;
use crate::vault_trait::VaultTrait;
use crate::verifiable_credential;
use crate::verifiable_credential::VerifiableCredential;
use soroban_sdk::{
    contract, contractimpl, contractmeta, panic_with_error, Address, BytesN, Env, IntoVal, String,
    Symbol, Val, Vec,
};

contractmeta!(
    key = "Description",
    val = "Smart contract for Chaincerts Vault",
);

#[contract]
pub struct VaultContract;

#[contractimpl]
impl VaultTrait for VaultContract {
    fn initialize(
        e: Env,
        admin: Address,
        did_wasm_hash: BytesN<32>,
        did_init_args: Vec<Val>,
        salt: BytesN<32>,
    ) -> (Address, Val) {
        if storage::has_admin(&e) {
            panic_with_error!(e, ContractError::AlreadyInitialized);
        }
        let (did_contract_address, did_document) =
            deploy_and_initialize_did(&e, salt, did_wasm_hash, did_init_args);
        let did_uri = did_document.id.clone();

        storage::write_admin(&e, &admin);
        storage::write_did(&e, &did_uri);
        storage::write_revoked(&e, &false);
        storage::write_issuers(&e, &Vec::new(&e));
        storage::write_vcs(&e, &Vec::new(&e));

        storage::extend_ttl_to_instance(&e);
        storage::extend_ttl_to_persistent(&e);
        (did_contract_address, did_document.into_val(&e))
    }

    fn authorize_issuers(e: Env, admin: Address, issuers: Vec<Address>) {
        validate_admin(&e, admin);
        validate_vault_revoked(&e);

        issuer::authorize_issuers(&e, &issuers);
    }

    fn authorize_issuer(e: Env, admin: Address, issuer: Address) {
        validate_admin(&e, admin);
        validate_vault_revoked(&e);

        issuer::authorize_issuer(&e, &issuer);
    }

    fn revoke_issuer(e: Env, admin: Address, issuer: Address) {
        validate_admin(&e, admin);
        validate_vault_revoked(&e);

        issuer::revoke_issuer(&e, &issuer)
    }

    fn store_vc(
        e: Env,
        vc_id: String,
        vc_data: String,
        issuer: Address,
        issuer_did: String,
        issuance_contract: Address,
    ) {
        validate_vault_revoked(&e);
        validate_issuer(&e, &issuer, &vc_data, &issuance_contract);

        verifiable_credential::store_vc(&e, vc_id, vc_data, issuance_contract, issuer_did);
    }

    fn revoke_vault(e: Env, admin: Address) {
        validate_admin(&e, admin);
        validate_vault_revoked(&e);

        storage::write_revoked(&e, &true);
    }

    fn get_vcs(e: Env) -> Vec<VerifiableCredential> {
        storage::read_vcs(&e)
    }
}

fn validate_admin(e: &Env, admin: Address) {
    let contract_admin = storage::read_admin(e);
    if contract_admin != admin {
        panic_with_error!(e, ContractError::NotAuthorized)
    }
    admin.require_auth();
}

fn validate_issuer(e: &Env, issuer: &Address, vc_data: &String, issuance_contract: &Address) {
    let issuers: Vec<Address> = storage::read_issuers(e);

    if !issuer::is_authorized(&issuers, issuer) {
        panic_with_error!(e, ContractError::IssuerNotAuthorized)
    }

    issuer.require_auth_for_args(
        (vc_data.clone(), issuer.clone(), issuance_contract.clone()).into_val(e),
    );
}

fn validate_vault_revoked(e: &Env) {
    let vault_revoked: bool = storage::read_revoked(e);
    if vault_revoked {
        panic_with_error!(e, ContractError::VaultRevoked)
    }
}

fn deploy_and_initialize_did(
    e: &Env,
    salt: BytesN<32>,
    did_wasm_hash: BytesN<32>,
    did_init_args: Vec<Val>,
) -> (Address, DIDDocument) {
    let init_fn = Symbol::new(e, "initialize");
    let did_contract_address = e
        .deployer()
        .with_current_contract(salt)
        .deploy(did_wasm_hash);
    let did_document: DIDDocument =
        e.invoke_contract(&did_contract_address, &init_fn, did_init_args);

    (did_contract_address, did_document)
}
