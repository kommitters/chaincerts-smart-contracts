use crate::did;
use crate::did::{Did, DidWithVCs};
use crate::error::ContractError;
use crate::issuer;
use crate::issuer::Issuer;
use crate::storage;
use crate::verifiable_credential;
use crate::verifiable_credential::VerifiableCredential;

use crate::vault_trait::VaultTrait;
use soroban_sdk::{
    contract, contractimpl, contractmeta, panic_with_error, Address, Env, IntoVal, Map, String, Vec,
};

const LEDGERS_THRESHOLD: u32 = 1;
const LEDGERS_TO_EXTEND: u32 = 535_000;

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

        did::set_initial_dids(&e, &dids);

        e.storage()
            .instance()
            .bump(LEDGERS_THRESHOLD, LEDGERS_TO_EXTEND);
    }

    fn authorize_issuer(e: Env, admin: Address, issuer: Address, did: String) {
        validate_admin(&e, admin);
        validate_did(&e, &did);

        issuer::authorize_issuer(&e, &issuer, &did);
    }

    fn revoke_issuer(e: Env, admin: Address, issuer: Address, did: String) {
        validate_admin(&e, admin);
        validate_did(&e, &did);

        issuer::revoke_issuer(&e, &issuer, &did)
    }

    fn store_vc(
        e: Env,
        vc_id: String,
        vc_data: String,
        recipient_did: String,
        issuer_pk: Address,
        issuance_contract_address: Address,
    ) {
        validate_did(&e, &recipient_did);
        validate_issuer(&e, &issuer_pk, &recipient_did);
        issuer_pk.require_auth_for_args(
            (
                vc_data.clone(),
                recipient_did.clone(),
                issuer_pk.clone(),
                issuance_contract_address.clone(),
            )
                .into_val(&e),
        );

        verifiable_credential::store_vc(
            &e,
            &vc_id,
            &vc_data,
            &issuance_contract_address,
            &recipient_did,
        );
    }

    fn get_vc(e: Env, vc_id: String) -> VerifiableCredential {
        let vcs = storage::read_vcs(&e);

        match vcs.get(vc_id) {
            Some(vc) => vc,
            None => panic_with_error!(&e, ContractError::VCNotFound),
        }
    }

    fn list_vcs(e: Env) -> Map<String, DidWithVCs> {
        let vcs = storage::read_vcs(&e);
        let dids = storage::read_dids(&e);
        let mut dids_with_vcs = Map::new(&e);

        for (did, did_struct) in dids {
            let mut did_vcs = Vec::new(&e);
            for vc in did_struct.vcs {
                did_vcs.push_front(vcs.get_unchecked(vc));
            }

            dids_with_vcs.set(
                did.clone(),
                DidWithVCs {
                    did: did.clone(),
                    is_revoked: did_struct.is_revoked,
                    vcs: did_vcs,
                },
            )
        }

        dids_with_vcs
    }

    fn revoke_did(e: Env, admin: Address, did: String) {
        validate_admin(&e, admin);
        let mut dids = storage::read_dids(&e);
        if !did::is_registered(&dids, &did) {
            panic_with_error!(e, ContractError::DidNotFound)
        }

        let did_struct = dids.get_unchecked(did.clone());
        dids.set(
            did.clone(),
            Did {
                did: did.clone(),
                is_revoked: true,
                vcs: did_struct.vcs,
            },
        );
        storage::write_dids(&e, &dids);
    }

    fn register_did(e: Env, admin: Address, did: String) {
        validate_admin(&e, admin);
        let mut dids = storage::read_dids(&e);

        if did::is_registered(&dids, &did) {
            panic_with_error!(e, ContractError::DuplicatedDID)
        }

        dids.set(
            did.clone(),
            Did {
                did: did.clone(),
                is_revoked: false,
                vcs: Vec::new(&e),
            },
        );
        storage::write_dids(&e, &dids)
    }
}

fn validate_admin(e: &Env, admin: Address) {
    let contract_admin = storage::read_admin(e);
    if contract_admin != admin {
        panic_with_error!(e, ContractError::NotAuthorized)
    }
    admin.require_auth();
}

fn validate_did(e: &Env, did: &String) {
    let dids = storage::read_dids(e);

    if !did::is_registered(&dids, did) {
        panic_with_error!(e, ContractError::DidNotFound)
    }
    if did::is_revoked(&dids, did) {
        panic_with_error!(e, ContractError::DidRevoked)
    }
}

fn validate_issuer(e: &Env, issuer: &Address, did: &String) {
    let issuers: Map<Address, Issuer> = storage::read_issuers(e, did);

    if !issuer::is_registered(&issuers, issuer) {
        panic_with_error!(e, ContractError::IssuerNotFound)
    }
    if issuer::is_revoked(&issuers, issuer) {
        panic_with_error!(e, ContractError::IssuerRevoked)
    }
}
