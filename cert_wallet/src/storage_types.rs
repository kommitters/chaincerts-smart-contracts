//! Module StorageTypes
//!
//! Module that defines the set of keys that can be used to access and store data within the contract.
use soroban_sdk::contracttype;

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Owner,
    /// Access Control List
    AccessControlList,
    /// A map that stores the Chaincerts, identified by a chaincert_id `Map<Bytes, Chaincert>`
    Chaincerts,
}
