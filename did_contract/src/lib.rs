#![no_std]
mod authentication;
mod capability_invocation;
mod did_document;
mod error;
mod option;
mod storage_types;
mod verifiable_credential;

use crate::error::{ContractError, DIDContractError};
use authentication::VerificationMethod;
use capability_invocation::CapabilityInvocation;
use did_document::{DIDDocument, Method, Service};
use option::OptionAddress;
use soroban_sdk::{contract, contractimpl, panic_with_error, Address, Env, String, Vec};
use verifiable_credential::VerifiableCredential;

const LEDGERS_THRESHOLD: u32 = 1;
const LEDGERS_TO_LIVE: u32 = 520_000;

#[contract]
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
        public_add_cap: Option<CapabilityInvocation>,
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
        did_document::write_services(&env, &services);
        capability_invocation::write_capability_invocation(
            &env,
            &Vec::<CapabilityInvocation>::new(&env),
        );
        if let Some(cap) = public_add_cap {
            if cap.type_ != capability_invocation::CapType::PublicAdd {
                panic_with_error!(env, DIDContractError::InvalidCapabilityInvocation);
            }
            capability_invocation::add_capability(&env, &cap);
        } else {
            capability_invocation::write_public_add_cap(&env, false);
        }

        // The contract instance will be bumped to have a lifetime of ~1 month.
        // If the lifetime is already more than 1 month, this is a no-op.
        // This lifetime bump includes the contract instance itself and all entries in storage().instance()
        env.storage()
            .instance()
            .bump(LEDGERS_THRESHOLD, LEDGERS_TO_LIVE)
    }

    /// Add capability invocation
    pub fn add_capability(env: Env, address: Address, capability_invocation: CapabilityInvocation) {
        authentication::check_invocation_address(&env, &address);
        address.require_auth();
        capability_invocation::add_capability(&env, &capability_invocation)
    }

    /// Remove capability invocation
    pub fn remove_capability(env: Env, address: Address, cap_id: String) {
        authentication::check_invocation_address(&env, &address);
        address.require_auth();
        capability_invocation::remove_capability(&env, &cap_id)
    }

    /// Deposit a `VerifiableCredential` to the DID contract
    pub fn deposit_credential(env: Env, verifiable_credential: VerifiableCredential) {
        if !capability_invocation::has_public_add_cap(&env) {
            if let OptionAddress::Some(address) = capability_invocation::check_capability_to_deposit(
                &env,
                &verifiable_credential.issuer,
            ) {
                address.require_auth()
            };
        }

        verifiable_credential::deposit_credential(&env, verifiable_credential)
    }

    /// Self-revoke a Credential.
    pub fn revoke_credential(env: Env, address: Address, credential_did: String) {
        authentication::check_invocation_address(&env, &address);
        address.require_auth();
        verifiable_credential::revoke_credential(&env, &credential_did);
    }

    /// Get the list of the `VerifiableCredential` stored in the DID contract. Only the owner can call this function.
    pub fn get_credentials(env: Env, address: Address) -> Vec<VerifiableCredential> {
        authentication::check_invocation_address(&env, &address);
        address.require_auth();
        verifiable_credential::get_credentials(&env)
    }

    pub fn get_public_credentials(env: Env) -> Vec<VerifiableCredential> {
        verifiable_credential::get_public_credentials(&env)
    }

    pub fn get_shared_credentials(
        env: Env,
        address: Address,
        invoker: String,
    ) -> Vec<VerifiableCredential> {
        capability_invocation::check_capability_to_read_credentials(&env, &address, &invoker);
        address.require_auth();
        verifiable_credential::get_shared_credentials(&env, &invoker)
    }

    /// Get the capability invocation list
    pub fn get_capability_invocation(env: Env, address: Address) -> Vec<CapabilityInvocation> {
        authentication::check_invocation_address(&env, &address);
        address.require_auth();
        capability_invocation::read_capability_invocation(&env)
    }

    /// Return boolean indicating if the DID document has a public add capability
    pub fn has_public_add_cap(env: Env) -> bool {
        capability_invocation::has_public_add_cap(&env)
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
