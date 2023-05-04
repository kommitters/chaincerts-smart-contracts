//! Module Storage Types
//!
//! Module where the DataKey of the contract and some necessary structs are defined.
use crate::cert_wallet::OptionU64;
use soroban_sdk::{contracttype, Address, Bytes};

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct Organization {
    pub id: Bytes,
    pub admin: Address,
}

#[contracttype]
#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Unassigned,
    Distribute,
    Revoked,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct CertData {
    pub id: Bytes,
    pub status: Status,
    pub distribution_date: OptionU64,
}

impl CertData {
    pub fn new(id: Bytes, status: Status, distribution_date: OptionU64) -> CertData {
        CertData {
            id,
            status,
            distribution_date,
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
    Recipients,         // Map <Address, CertData>
    Organization,      // Organization
    DistributionLimit, // u32
    Supply,            // u32
}
