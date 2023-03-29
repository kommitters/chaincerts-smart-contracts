//! Module StorageTypes
//!
//! Module that defines the set of keys that can be used to access and store data within the contract.
use soroban_sdk::contracttype;

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Owner,
    /// Access Control List
    Acl,
    /// A map that stores a Chaincert by a given chaincert_id `Map<Bytes, Chaincert>`
    Chaincerts,
}
