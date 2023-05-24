#![no_std]
mod access_control_list;
mod authentication;
mod did_document;
mod error;
mod option;
mod storage_types;
mod verifiable_credential;

use crate::error::ContractError;
use authentication::VerificationMethod;
use did_document::{DIDDocument, Metadata, Method, Service};
use option::OptionU64;
use soroban_sdk::{contractimpl, panic_with_error, Address, Env, String, Vec};
use verifiable_credential::VerifiableCredential;

pub struct DIDContract;

#[contractimpl]
impl DIDContract {
    pub fn initialize(
        env: Env,
        id: String,
        authentication_params: (String, Address), //key_id, address
        context: Vec<String>,
        verification_processes: Vec<Method>,
        services: Vec<Service>,
        metadata: Metadata,
    ) {
        if authentication::has_authentication(&env) {
            panic_with_error!(env, ContractError::AlreadyInit);
        }
        did_document::write_id(&env, &id);
        authentication::write_authentication(
            &env,
            &authentication_params.0,
            &authentication_params.1,
        );
        did_document::write_context(&env, &context);
        did_document::write_verification_processes(&env, &verification_processes);
        did_document::write_metadata(&env, &metadata);
        did_document::write_services(&env, &services);
    }

    /// Add organizations to the ACL
    pub fn add_organization(env: Env, address: Address, issuer: String) {
        authentication::check_invocation_address(&env, &address);
        address.require_auth();
        access_control_list::add_organization(&env, &issuer)
    }

    /// Remove organizations from the ACL
    pub fn remove_organization(env: Env, address: Address, issuer: String) {
        authentication::check_invocation_address(&env, &address);
        address.require_auth();
        access_control_list::remove_organization(&env, &issuer)
    }

    /// Deposit a `VerifiableCredential` to the DID contract
    pub fn deposit_credential(
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
    pub fn revoke_credential(env: Env, address: Address, credential_did: String) {
        authentication::check_invocation_address(&env, &address);
        address.require_auth();
        verifiable_credential::revoke_credential(&env, &credential_did);
    }

    /// Get the list of the `VerifiableCredential` stored in the DID contract
    pub fn get_credentials(env: Env) -> Vec<VerifiableCredential> {
        verifiable_credential::get_credentials(&env)
    }

    /// Get the ACL stored in the DID contract
    pub fn get_access_control_list(env: Env, address: Address) -> Vec<String> {
        authentication::check_invocation_address(&env, &address);
        address.require_auth();
        access_control_list::get_access_control_list(&env)
    }

    /// Get DID document public data
    pub fn public_did_document(env: Env) -> DIDDocument {
        did_document::retrieve_public_did_document(&env)
    }

    /// Add a new authentication to the authentication list
    pub fn add_authentication(
        env: Env,
        address: Address,
        new_key_id: String,
        new_address: Address,
    ) {
        authentication::check_invocation_address(&env, &address);
        address.require_auth();
        authentication::write_authentication(&env, &new_key_id, &new_address)
    }

    /// Remove an authentication form the authentication list
    pub fn remove_authentication(env: Env, address: Address, key_id: String) {
        authentication::check_invocation_address(&env, &address);
        address.require_auth();
        authentication::remove_authentication(&env, &key_id)
    }

    /// Add a new verification_method to the verification_method list
    pub fn add_verification_method(
        env: Env,
        address: Address,
        key_id: String,
        new_address: Address,
    ) {
        authentication::check_invocation_address(&env, &address);
        address.require_auth();
        let controller = did_document::read_id(&env);
        let verification_method = VerificationMethod::new(&env, key_id, new_address, controller);
        authentication::write_verification_method(&env, verification_method)
    }

    /// Remove an authentication form the authentication list,
    /// also removes the associated authentication of the authentication list
    pub fn remove_verification_method(env: Env, address: Address, key_id: String) {
        authentication::check_invocation_address(&env, &address);
        address.require_auth();
        authentication::remove_verification_method(&env, &key_id)
    }
}

mod test;
