use crate::did_contract::DIDDocument;
use crate::error::ContractError;
use crate::issuer;
use crate::storage;
use crate::vault;
use crate::vault::Vault;
use crate::vault_trait::VaultTrait;
use crate::verifiable_credential;
use soroban_sdk::{
    contract, contractimpl, contractmeta, panic_with_error, Address, BytesN, Env, IntoVal, Map,
    String, Symbol, Val, Vec,
};

// MAXIMUM ENTRY TTL:
// 31 days, 12 ledger close per minute.
// (12 * 60 * 24 * 31) - 1
const LEDGERS_TO_EXTEND: u32 = 535_679;

contractmeta!(
    key = "Description",
    val = "Smart contract for Chaincerts Vault",
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

        vault::set_initial_vaults(&e, &dids);

        e.storage()
            .instance()
            .extend_ttl(LEDGERS_TO_EXTEND, LEDGERS_TO_EXTEND);
    }

    fn authorize_issuers(e: Env, admin: Address, issuers: Vec<Address>, did: String) {
        validate_admin(&e, admin);

        let vaults = storage::read_vaults(&e);
        validate_vault(&e, &vaults, &did);

        issuer::authorize_issuers(&e, &issuers, &did);
    }

    fn authorize_issuer(e: Env, admin: Address, issuer: Address, did: String) {
        validate_admin(&e, admin);

        let vaults = storage::read_vaults(&e);
        validate_vault(&e, &vaults, &did);

        issuer::authorize_issuer(&e, &issuer, &did);
    }

    fn revoke_issuer(e: Env, admin: Address, issuer: Address, did: String) {
        validate_admin(&e, admin);

        let vaults = storage::read_vaults(&e);
        validate_vault(&e, &vaults, &did);

        issuer::revoke_issuer(&e, &issuer, &did)
    }

    fn store_vc(
        e: Env,
        vc_id: String,
        vc_data: String,
        recipient_did: String,
        issuer: Address,
        issuance_contract: Address,
    ) {
        let mut vaults = storage::read_vaults(&e);
        validate_vault(&e, &vaults, &recipient_did);

        validate_issuer(&e, &issuer, &recipient_did, &vc_data, &issuance_contract);

        verifiable_credential::store_vc(
            &e,
            &mut vaults,
            vc_id,
            vc_data,
            issuance_contract,
            recipient_did,
        );
    }

    fn register_vault(
        e: Env,
        admin: Address,
        did_wasm_hash: BytesN<32>,
        did_init_args: Vec<Val>,
        salt: BytesN<32>,
    ) -> (Address, Val) {
        validate_admin(&e, admin.clone());

        let (did_contract_address, did_document) =
            deploy_and_initialize_did(&e, salt, did_wasm_hash, did_init_args);
        let did_uri = did_document.id.clone();

        let mut vaults = storage::read_vaults(&e);
        vaults.set(
            did_uri.clone(),
            Vault {
                did: did_uri,
                revoked: false,
                vcs: Vec::new(&e),
            },
        );

        storage::write_vaults(&e, &vaults);
        (did_contract_address, did_document.into_val(&e))
    }

    fn register_vault_with_did(e: Env, admin: Address, did: String) {
        validate_admin(&e, admin);
        let mut vaults = storage::read_vaults(&e);

        if vault::is_registered(&vaults, &did) {
            panic_with_error!(e, ContractError::VaultAlreadyRegistered)
        }

        vaults.set(
            did.clone(),
            Vault {
                did,
                revoked: false,
                vcs: Vec::new(&e),
            },
        );

        storage::write_vaults(&e, &vaults)
    }

    fn revoke_vault(e: Env, admin: Address, did: String) {
        validate_admin(&e, admin);
        let mut vaults = storage::read_vaults(&e);

        if !vault::is_registered(&vaults, &did) {
            panic_with_error!(e, ContractError::VaultNotFound)
        }

        let vault = vaults.get_unchecked(did.clone());

        vaults.set(
            did.clone(),
            Vault {
                revoked: true,
                ..vault
            },
        );

        storage::write_vaults(&e, &vaults);
    }

    fn get_vault(e: Env, did: String) -> Vault {
        let vaults = storage::read_vaults(&e);

        match vaults.get(did) {
            Some(vault) => vault,
            None => panic_with_error!(&e, ContractError::VaultNotFound),
        }
    }

    fn list_vaults(e: Env) -> Vec<Vault> {
        let vaults = storage::read_vaults(&e);
        vaults.values()
    }
}

fn validate_admin(e: &Env, admin: Address) {
    let contract_admin = storage::read_admin(e);
    if contract_admin != admin {
        panic_with_error!(e, ContractError::NotAuthorized)
    }
    admin.require_auth();
}

fn validate_vault(e: &Env, vaults: &Map<String, Vault>, did: &String) {
    if !vault::is_registered(vaults, did) {
        panic_with_error!(e, ContractError::VaultNotFound)
    }

    if vault::is_revoked(vaults, did) {
        panic_with_error!(e, ContractError::VaultRevoked)
    }
}

fn validate_issuer(
    e: &Env,
    issuer: &Address,
    did: &String,
    vc_data: &String,
    issuance_contract: &Address,
) {
    let issuers: Vec<Address> = storage::read_issuers(e, did);

    if !issuer::is_authorized(&issuers, issuer) {
        panic_with_error!(e, ContractError::IssuerNotAuthorized)
    }

    issuer.require_auth_for_args(
        (
            vc_data.clone(),
            did.clone(),
            issuer.clone(),
            issuance_contract.clone(),
        )
            .into_val(e),
    );
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
