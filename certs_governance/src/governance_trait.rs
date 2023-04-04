//! Module GovernanceTrait
//!
//! Interface that defines the behavior of a Governance contract.
use soroban_sdk::{Address, Bytes, BytesN, Env, Map, Vec};

use crate::storage_types::{CertData, Info, Organization};
pub trait GovernanceTrait {
    /// Initialize the contract with a list of receivers.
    fn init_w_r(
        e: Env,
        file_storage: Bytes,
        name: Bytes,
        revocable: bool,
        expiration_time: Option<u64>,
        receivers: Vec<Address>,
        organization: Organization,
    );

    /// Initialize the contract with the limit of Chaincerts that can be distributed.
    fn init_w_l(
        e: Env,
        file_storage: Bytes,
        name: Bytes,
        revocable: bool,
        expiration_time: Option<u64>,
        distribution_limit: u32,
        organization: Organization,
    );

    /// Distribute a Chaincert to a receiver.
    fn distribute(
        e: Env,
        admin: Address,
        receiver: Address,
        wallet_contract_id: BytesN<32>,
        cid: Bytes,
        distribution_date: u64,
    );

    /// Revoke a Chaincert from a receiver.
    fn revoke(e: Env, admin: Address, receiver: Address, wallet_contract_id: BytesN<32>);

    /// Get the Chaincert name.
    fn name(e: Env) -> Bytes;

    /// Get if the Chaincert can be revoked or not.
    fn revocable(e: Env) -> bool;

    /// Get the Chaincert expiration time (Unix time).
    fn exp_time(e: Env) -> Option<u64>;

    /// Get the maximum number of Chaincerts that can be distributed by this contract.
    fn dist_limit(e: Env) -> u32;

    /// Get number of Chaincerts that have been distributed.
    fn supply(e: Env) -> u32;

    /// Get the type of decentralized storage service.
    fn f_storage(e: Env) -> Bytes;

    /// Get the receivers data in the contract.
    fn receivers(e: Env) -> Map<Address, CertData>;

    // Get all relevant contract data.
    fn info(e: Env) -> Info;
}
