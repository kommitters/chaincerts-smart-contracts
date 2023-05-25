//! Module VerifiableCredential
//!
//! Module responsible of managing `VerifiableCredential` information and defining its corresponding struct.
use crate::{
    capability_invocation::{read_capability_invocation, CapType},
    error::ContractError,
    option::{OptionString, OptionU64},
    storage_types::DataKey,
};
use soroban_sdk::{contracttype, map, panic_with_error, Env, Map, String, Vec};
const VERIFIABLE_CREDENTIAL_KEY: DataKey = DataKey::VerifiableCredential;

#[derive(Clone, PartialEq, Eq, Debug)]
#[contracttype]
/// The `VerifiableCredential` information stored in the did document
pub struct VerifiableCredential {
    // Credential did
    pub id: String,
    /// The did of the organization that distributed the `VerifiableCredential`
    pub issuer: String,
    /// The distribution date in Unix Timestamp format
    pub issuance_date: u64,
    /// The expiration date in Unix Timestamp format
    pub expiration_date: OptionU64,
    /// Content identifier
    pub attestation: String,
    /// A logical indicator that lets know if a `VerifiableCredential` is self-revoked or not
    pub revoked: bool,
}

impl VerifiableCredential {
    fn new(
        id: String,
        issuer: String,
        issuance_date: u64,
        expiration_date: OptionU64,
        attestation: String,
        revoked: bool,
    ) -> VerifiableCredential {
        VerifiableCredential {
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
    let credential = VerifiableCredential::new(
        credential_did.clone(),
        issuer,
        issuance_date,
        expiration_date,
        attestation,
        false,
    );

    let credentials = match env.storage().get(&VERIFIABLE_CREDENTIAL_KEY) {
        Some(credential_map) => {
            let mut credential_map: Map<String, VerifiableCredential> = credential_map.unwrap();
            if !credential_map.contains_key(credential_did.clone()) {
                credential_map.set(credential_did, credential);
                credential_map
            } else {
                panic_with_error!(env, ContractError::VerifiableCredentialAlreadyInWallet)
            }
        }
        None => {
            let map: Map<String, VerifiableCredential> = map![env, (credential_did, credential)];
            map
        }
    };
    write_credentials(env, &credentials)
}

pub(crate) fn revoke_credential(env: &Env, credential_did: &String) {
    match env.storage().get(&VERIFIABLE_CREDENTIAL_KEY) {
        Some(credential_map) => {
            let mut credential_map: Map<String, VerifiableCredential> = credential_map.unwrap();
            revoke_credential_from_map(env, &mut credential_map, credential_did);
            write_credentials(env, &credential_map);
        }
        None => {
            panic_with_error!(env, ContractError::NoVerifiableCredential)
        }
    };
}

pub(crate) fn get_credentials(env: &Env) -> Vec<VerifiableCredential> {
    read_credentials(env).values()
}

pub(crate) fn get_public_credentials(env: &Env) -> Vec<VerifiableCredential> {
    let mut credentials: Vec<VerifiableCredential> = Vec::new(env);

    let verifiable_credentials_map = read_credentials(env);

    // Look for the capability invocation type and get the credential (did)
    let capability_invocation = read_capability_invocation(env);
    for cap in capability_invocation {
        let cap = cap.unwrap();
        if cap.type_ == CapType::PublicRead {
            if let OptionString::Some(credential_did) = cap.credential {
                let verifiable_credential = verifiable_credentials_map
                    .get_unchecked(credential_did)
                    .unwrap();

                credentials.push_front(verifiable_credential);
            }
        }
    }

    credentials
}

pub(crate) fn get_shared_credentials(env: &Env, invoker: &String) -> Vec<VerifiableCredential> {
    let mut credentials: Vec<VerifiableCredential> = Vec::new(env);

    let verifiable_credentials_map = read_credentials(env);

    // Look for the capability invocation type and get the credential (did)
    let capability_invocation = read_capability_invocation(env);
    for cap in capability_invocation {
        let cap = cap.unwrap();
        if cap.type_ == CapType::ReadCredential {
            if let (OptionString::Some(credential_did), OptionString::Some(cap_invoker)) =
                (cap.credential, cap.invoker)
            {
                if cap_invoker == invoker.clone() {
                    let verifiable_credential = verifiable_credentials_map
                        .get_unchecked(credential_did)
                        .unwrap();
                    credentials.push_front(verifiable_credential)
                }
            }
        }
    }

    credentials
}

fn revoke_credential_from_map(
    env: &Env,
    credential_map: &mut Map<String, VerifiableCredential>,
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

fn read_credentials(env: &Env) -> Map<String, VerifiableCredential> {
    match env.storage().get(&VERIFIABLE_CREDENTIAL_KEY) {
        Some(credential) => credential.unwrap(),
        None => panic_with_error!(env, ContractError::NoVerifiableCredential),
    }
}

fn write_credentials(env: &Env, certs: &Map<String, VerifiableCredential>) {
    env.storage().set(&VERIFIABLE_CREDENTIAL_KEY, certs)
}
