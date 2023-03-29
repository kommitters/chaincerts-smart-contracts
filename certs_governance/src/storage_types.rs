//! Module Storage Types
//!
//! Module where the DataKey of the contract and some necessary structs are defined.
use soroban_sdk::{contracttype, Address, Bytes};

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct Organization {
    pub id_org: Bytes,
    pub admin: Address,
}

impl Organization {
    pub fn new(id_org: Bytes, admin: Address) -> Organization {
        Organization { id_org, admin }
    }
}

#[contracttype]
#[derive(Debug, Clone, PartialEq)]
pub enum Opt {
    None,
    Some(u64),
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
    pub dist_date: Opt,
}

impl CertData {
    pub fn new(id_cert: Bytes, status: Status, dist_date: Opt) -> CertData {
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
    pub exp_time: Opt,
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
