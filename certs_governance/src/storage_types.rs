//! Module Storage Types
//!
//! Module where the DataKey of the contract and some necessary structs are defined.
use crate::certs_wallet::OptU64;
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
    pub distribution_date: OptU64,
}

impl CertData {
    pub fn new(id: Bytes, status: Status, distribution_date: OptU64) -> CertData {
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
    pub expiration_time: OptU64,
    pub distribution_limit: u32,
    pub supply: u32,
}

#[derive(Clone, Debug)]
#[contracttype]
pub enum DataKey {
    FileStorage,       // Bytes
    Name,              // Bytes
    Revocable,         // bool
    ExpirationTime,    // Option <u64>
    Receivers,         // Map <Address, CertData>
    Organization,      // Organization
    DistributionLimit, // u32
    Supply,            // u32
}
