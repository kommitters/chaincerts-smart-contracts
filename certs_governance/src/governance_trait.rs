//! Module GovernanceTrait
//!
//! Interface that defines the behavior of a Governance contract.
use soroban_sdk::{Address, Bytes, BytesN, Env, Map, Vec};

use crate::{storage_types::{CertData, GovernanceRules, Info, Organization}, certs_wallet::OptU64};
pub trait GovernanceTrait {
    /// Initialize the contract a list of receivers or with the limit of Chaincerts that can be distributed.
    fn initialize(
        e: Env,
        file_storage: Bytes,
        name: Bytes,
        receivers: Option<Vec<Address>>,
        distribution_limit: Option<u32>,
        // governance_rules: GovernanceRules,
        governance_rules: (bool, OptU64),
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
    fn is_revocable(e: Env) -> bool;

    /// Get the Chaincert expiration time (Unix time).
    fn expiration_time(e: Env) -> OptU64;

    /// Get the maximum number of Chaincerts that can be distributed by this contract.
    fn distribution_limit(e: Env) -> u32;

    /// Get number of Chaincerts that have been distributed.
    fn supply(e: Env) -> u32;

    /// Get the type of decentralized storage service.
    fn file_storage(e: Env) -> Bytes;

    /// Get the receivers data in the contract.
    fn receivers(e: Env) -> Map<Address, CertData>;

    // Get all relevant contract data.
    fn info(e: Env) -> Info;
}
