//! Module VerifiableCredentials
//!
//! Module responsible of managing `VerifiableCredentials` information and defining its corresponding struct.
use crate::{error::ContractError, option::OptionU64, storage_types::DataKey};
use soroban_sdk::{contracttype, map, panic_with_error, Env, Map, String, Vec};

const VERIFIABLE_CREDENTIAL_KEY: DataKey = DataKey::VerifiableCredentials;

#[derive(Clone, PartialEq, Eq, Debug)]
#[contracttype]
/// The `VerifiableCredentials` information stored in the did document
pub struct VerifiableCredentials {
    // Credential did
    pub id: String,
    /// The did of the organization that distributed the `VerifiableCredentials`
    pub issuer: String,
    /// The distribution date in Unix Timestamp format
    pub issuance_date: u64,
    /// The expiration date in Unix Timestamp format
    pub expiration_date: OptionU64,
    pub attestation: String,
    /// A logical indicator that lets know if a `VerifiableCredentials` is revoked or not
    pub revoked: bool,
}

impl VerifiableCredentials {
    fn new(
        id: String,
        issuer: String,
        issuance_date: u64,
        expiration_date: OptionU64,
        attestation: String,
        revoked: bool,
    ) -> VerifiableCredentials {
        VerifiableCredentials {
            id,
            attestation,
            issuer,
            issuance_date,
            expiration_date,
            revoked,
        }
    }
}

pub(crate) fn deposit_credential(
    env: &Env,
    credential_did: String,
    issuer: String,
    issuance_date: u64,
    expiration_date: OptionU64,
    attestation: String,
) {
    let credential = VerifiableCredentials::new(
        credential_did.clone(),
        issuer,
        issuance_date,
        expiration_date,
        attestation,
        false,
    );

    let credentials = match env.storage().get(&VERIFIABLE_CREDENTIAL_KEY) {
        Some(credential_map) => {
            let mut credential_map: Map<String, VerifiableCredentials> = credential_map.unwrap();
            if !credential_map.contains_key(credential_did.clone()) {
                credential_map.set(credential_did, credential);
                credential_map
            } else {
                panic_with_error!(env, ContractError::VerifiableCredentialAlreadyInWallet)
            }
        }
        None => {
            let map: Map<String, VerifiableCredentials> = map![env, (credential_did, credential)];
            map
        }
    };
    write_credentials(env, &credentials)
}

pub(crate) fn revoke_credential(env: &Env, credential_did: &String) {
    match env.storage().get(&VERIFIABLE_CREDENTIAL_KEY) {
        Some(credential_map) => {
            let mut credential_map: Map<String, VerifiableCredentials> = credential_map.unwrap();
            revoke_credential_from_map(env, &mut credential_map, credential_did);
            write_credentials(env, &credential_map);
        }
        None => {
            panic_with_error!(env, ContractError::NoVerifiableCredentials)
        }
    };
}

pub(crate) fn get_credentials(env: &Env) -> Vec<VerifiableCredentials> {
    read_credentials(env).values()
}

fn revoke_credential_from_map(
    env: &Env,
    credential_map: &mut Map<String, VerifiableCredentials>,
    credential_did: &String,
) {
    match credential_map.get(credential_did.clone()) {
        Some(credential) => {
            let mut credential = credential.unwrap();
            credential.revoked = true;
            credential_map.set(credential_did.clone(), credential);
        }
        None => panic_with_error!(env, ContractError::VerifiableCredentialNotFound),
    }
}

fn read_credentials(env: &Env) -> Map<String, VerifiableCredentials> {
    match env.storage().get(&VERIFIABLE_CREDENTIAL_KEY) {
        Some(credential) => credential.unwrap(),
        None => panic_with_error!(env, ContractError::NoVerifiableCredentials),
    }
}

fn write_credentials(env: &Env, certs: &Map<String, VerifiableCredentials>) {
    env.storage().set(&VERIFIABLE_CREDENTIAL_KEY, certs)
}
