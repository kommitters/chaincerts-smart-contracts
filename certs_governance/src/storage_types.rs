//! Module Storage Types
//!
//! Module where the DataKey of the contract and some necessary structs are defined.
use soroban_sdk::{contracttype, Address, Bytes};

use crate::certs_wallet::OptU64;

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct Organization {
    pub id_org: Bytes,
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
    pub id_cert: Bytes,
    pub status: Status,
    pub dist_date: OptU64,
}

impl CertData {
    pub fn new(id_cert: Bytes, status: Status, dist_date: OptU64) -> CertData {
        CertData {
            id_cert,
            status,
            dist_date,
        }
    }
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct Info {
    pub name: Bytes,
    pub revocable: bool,
    pub exp_time: OptU64,
    pub dist_limit: u32,
    pub supply: u32,
}

#[derive(Clone, Debug)]
#[contracttype]
pub enum DataKey {
    FStorage,  // Bytes
    Name,      // Bytes
    Revocable, // bool
    ExpTime,   // Option <u64>
    Receivers, // Map <Address, CertData>
    Org,       // Organization
    DistLimit, // u32
    Supply,    // u32
}
