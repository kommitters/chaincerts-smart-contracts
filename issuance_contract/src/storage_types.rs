//! Module Storage Types
//!
//! Module where the DataKey of the contract and some necessary structs are defined.
use crate::did_contract::OptionU64;
use soroban_sdk::{contracttype, Address, String};

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct Organization {
    pub did: String,
    pub admin: Address,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct CredentialData {
    pub did: String,
    pub recipient_did: String,
    pub credential_type: String,
    pub credential_title: String,
    pub issuance_date: u64,
    pub expiration_date: OptionU64,
    pub signature: String,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct RevokedCredential {
    pub credential_data: CredentialData,
    pub revocation_date: u64,
}

impl CredentialData {
    pub fn new(
        did: String,
        recipient_did: String,
        credential_type: String,
        credential_title: String,
        issuance_date: u64,
        expiration_date: OptionU64,
        signature: String,
    ) -> CredentialData {
        CredentialData {
            did,
            recipient_did,
            credential_type,
            credential_title,
            issuance_date,
            expiration_date,
            signature,
        }
    }
}

impl RevokedCredential {
    pub fn new(credential_data: CredentialData, revocation_date: u64) -> RevokedCredential {
        RevokedCredential {
            credential_data,
            revocation_date,
        }
    }
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct Info {
    pub name: String,
    pub revocable: bool,
    pub distribution_limit: u32,
    pub supply: u32,
    pub credential_type: String,
    pub credential_title: String,
}

#[derive(Clone, Debug)]
#[contracttype]
pub enum DataKey {
    FileStorage,        // String
    Name,               // String
    Revocable,          // bool
    RevokedCredentials, // Vec<CredentialData>
    Recipients,         // Map <String, Option<CredentialData>>
    Organization,       // Organization
    DistributionLimit,  // u32
    Supply,             // u32
    CredentialTitle,    // String
    CredentialType,     // String
}
