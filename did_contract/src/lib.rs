#![no_std]
mod access_control_list;
mod chaincert;
mod error;
mod option;
mod owner;
mod storage_types;
use crate::error::ContractError;
use chaincert::Chaincert;
use option::OptionU64;
use soroban_sdk::{contractimpl, panic_with_error, Address, Bytes, Env, Vec};

pub struct DIDContract;

#[contractimpl]
impl DIDContract {
    pub fn initialize(env: Env, owner: Address) {
        if owner::has_owner(&env) {
            panic_with_error!(env, ContractError::AlreadyInit);
        }
        owner::write_owner(&env, &owner);
    }

    /// Add organizations to the ACL
    pub fn add_organization(env: Env, org_id: Bytes) {
        owner::read_owner(&env).require_auth();
        access_control_list::add_organization(&env, &org_id)
    }

    /// Remove organizations from the ACL
    pub fn remove_organization(env: Env, org_id: Bytes) {
        owner::read_owner(&env).require_auth();
        access_control_list::remove_organization(&env, &org_id)
    }

    /// Deposit a `Chaincert` to the wallet
    pub fn deposit_chaincert(
        env: Env,
        credential_did: Bytes,
        attestation: Bytes,
        distributor_contract: Address,
        org_id: Bytes,
        issuance_date: u64,
        expiration_date: OptionU64,
    ) {
        access_control_list::check_access_control_list(&env, &org_id);
        distributor_contract.require_auth();
        chaincert::deposit_chaincert(
            &env,
            credential_did,
            attestation,
            distributor_contract,
            org_id,
            issuance_date,
            expiration_date,
        )
    }

    /// Self-revoke a Credential.
    pub fn revoke_credential(
        env: Env,
        credential_did: Bytes,
    ) {
        owner::read_owner(&env).require_auth();
        chaincert::revoke_chaincert(&env, &credential_did);
    }

    /// Get the list of the `Chaincerts` stored in the wallet
    pub fn get_chaincerts(env: Env) -> Vec<Chaincert> {
        chaincert::get_chaincerts(&env)
    }

    /// Get the ACL stored in the wallet
    pub fn get_access_control_list(env: Env) -> Vec<Bytes> {
        owner::read_owner(&env).require_auth();
        access_control_list::get_access_control_list(&env)
    }
}

mod test;
