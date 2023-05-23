#![no_std]
mod access_control_list;
mod did_document;
mod error;
mod option;
mod owner;
mod storage_types;
mod verifiable_credential;

use crate::error::ContractError;
use did_document::{Metadata, Method, Service};
use option::OptionU64;
use owner::Owner;
use soroban_sdk::{contractimpl, panic_with_error, Address, Env, String, Vec};
use verifiable_credential::VerifiableCredentials;

pub struct DIDContract;

#[contractimpl]
impl DIDContract {
    pub fn initialize(
        env: Env,
        owner: Owner,
        context: Vec<String>,
        verification_processes: Vec<Method>,
        services: Vec<Service>,
        metadata: Metadata,
    ) {
        if owner::has_owner(&env) {
            panic_with_error!(env, ContractError::AlreadyInit);
        }
        owner::write_owner(&env, &owner);
        did_document::write_context(&env, &context);
        did_document::write_verification_processes(&env, &verification_processes);
        did_document::write_services(&env, &services);
        did_document::write_metadata(&env, &metadata);
    }

    /// Add organizations to the ACL
    pub fn add_organization(env: Env, issuer: String, address: Address) {
        owner::check_invocation_address(&env, &address);
        address.require_auth();
        access_control_list::add_organization(&env, &issuer)
    }

    /// Remove organizations from the ACL
    pub fn remove_organization(env: Env, issuer: String, address: Address) {
        owner::check_invocation_address(&env, &address);
        address.require_auth();
        access_control_list::remove_organization(&env, &issuer)
    }

    /// Deposit a `VerifiableCredentials` to the wallet
    pub fn deposit_chaincert(
        env: Env,
        credential_did: String,
        issuer: String,
        issuance_date: u64,
        expiration_date: OptionU64,
        attestation: String,
    ) {
        access_control_list::check_access_control_list(&env, &issuer);
        verifiable_credential::deposit_credential(
            &env,
            credential_did,
            issuer,
            issuance_date,
            expiration_date,
            attestation,
        )
    }

    /// Self-revoke a Credential.
    pub fn revoke_credential(env: Env, credential_did: String, address: Address) {
        owner::check_invocation_address(&env, &address);
        address.require_auth();
        verifiable_credential::revoke_credential(&env, &credential_did);
    }

    /// Get the list of the `VerifiableCredentials` stored in the wallet
    pub fn get_credentials(env: Env) -> Vec<VerifiableCredentials> {
        verifiable_credential::get_credentials(&env)
    }

    /// Get the ACL stored in the wallet
    pub fn get_access_control_list(env: Env, address: Address) -> Vec<String> {
        owner::check_invocation_address(&env, &address);
        address.require_auth();
        access_control_list::get_access_control_list(&env)
    }
}

mod test;
