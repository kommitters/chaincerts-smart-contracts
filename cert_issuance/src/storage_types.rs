//! Module Storage Types
//!
//! Module where the DataKey of the contract and some necessary structs are defined.
use crate::cert_wallet::OptionU64;
use soroban_sdk::{contracttype, Address, Bytes, String};

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct Organization {
    pub did: Bytes,
    pub admin: Address,
}

#[contracttype]
#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Distributed,
    Revoked,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct CredentialData {
    pub did: Bytes,
    pub status: Status,
    pub credential_type: String,
    pub credential_title: String,
    pub issuance_date: OptionU64,
    pub signature: String,
}

impl CredentialData {
    pub fn new(
        did: Bytes,
        status: Status,
        credential_type: String,
        credential_title: String,
        issuance_date: OptionU64,
        signature: String,
    ) -> CredentialData {
        CredentialData {
            did,
            status,
            credential_type,
            credential_title,
            issuance_date,
            signature,
        }
    }
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct Info {
    pub name: Bytes,
    pub revocable: bool,
    pub expiration_time: OptionU64,
    pub distribution_limit: u32,
    pub supply: u32,
}

#[derive(Clone, Debug)]
#[contracttype]
pub enum DataKey {
    FileStorage,       // Bytes
    Name,              // Bytes
    Revocable,         // bool
    ExpirationTime,    // OptionU64
    Recipients,        // Map <Address, Option<CredentialData>>
    Organization,      // Organization
    DistributionLimit, // u32
    Supply,            // u32
    CredentialTitle,   // String
    CredentialType,    // String
}
