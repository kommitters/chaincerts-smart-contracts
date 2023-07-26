//! Module Attest
//!
//! Module that groups the functions required to attest a credential.
use soroban_sdk::{Env, Map, String};

use crate::{
    did_contract::OptionU64,
    issuance_trait::CredentialStatus,
    metadata::read_revoked_credentials,
    recipients::read_recipients,
    storage_types::{CredentialData, RevokedCredential},
};

pub fn get_credential_data(e: &Env, recipient: &String) -> Option<CredentialData> {
    let recipients_map: Map<String, Option<CredentialData>> = read_recipients(e);
    if let Some(recipient_data) = recipients_map.get(recipient.clone()) {
        return recipient_data;
    }
    None
}

pub fn get_revoked_credential(e: &Env, recipient: &String) -> Option<RevokedCredential> {
    let revoked_credentials: Map<String, RevokedCredential> = read_revoked_credentials(e);
    revoked_credentials.get(recipient.clone())
}

pub fn is_valid(data: &CredentialData, credential: &String, signature: &String) -> bool {
    data.signature == *signature && data.did == *credential
}

pub fn valid_status(e: &Env, credential_data: CredentialData) -> CredentialStatus {
    let expiration_date = credential_data.expiration_date;
    CredentialStatus {
        status: String::from_slice(e, "valid"),
        expiration_date,
        revocation_date: OptionU64::None,
    }
}

pub fn revoked_status(e: &Env, revoked_credential: RevokedCredential) -> CredentialStatus {
    let expiration_date = revoked_credential.credential_data.expiration_date;
    let revocation_date = revoked_credential.revocation_date;
    CredentialStatus {
        status: String::from_slice(e, "revoked"),
        expiration_date,
        revocation_date: OptionU64::Some(revocation_date),
    }
}

pub fn invalid_status(e: &Env) -> CredentialStatus {
    CredentialStatus {
        status: String::from_slice(e, "invalid"),
        expiration_date: OptionU64::None,
        revocation_date: OptionU64::None,
    }
}
