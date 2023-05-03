//! Module IssuanceTrait
//!
//! Interface that defines the behavior of a Issuance contract.
use soroban_sdk::{Address, Bytes, BytesN, Env, Map, Vec};

use crate::{
    certs_wallet::OptionU64,
    storage_types::{CertData, Info, Organization},
};
pub trait IssuanceTrait {
    /// Initialize the contract a list of recipients or with the limit of Chaincerts that can be distributed.
    fn initialize(
        e: Env,
        file_storage: Bytes,
        name: Bytes,
        recipients: Option<Vec<Address>>,
        distribution_limit: Option<u32>,
        distribution_rules: (bool, OptionU64),
        organization: Organization,
    );

    /// Distribute a Chaincert to a recipient.
    fn distribute(
        e: Env,
        admin: Address,
        recipient: Address,
        wallet_contract_id: BytesN<32>,
        cid: Bytes,
        distribution_date: u64,
    );

    /// Revoke a Chaincert from a recipient.
    fn revoke(e: Env, admin: Address, recipient: Address, wallet_contract_id: BytesN<32>);

    /// Get the Chaincert name.
    fn name(e: Env) -> Bytes;

    /// Get if the Chaincert can be revoked or not.
    fn is_revocable(e: Env) -> bool;

    /// Get the Chaincert expiration time (Unix time).
    fn expiration_time(e: Env) -> OptionU64;

    /// Get the maximum number of Chaincerts that can be distributed by this contract.
    fn distribution_limit(e: Env) -> u32;

    /// Get number of Chaincerts that have been distributed.
    fn supply(e: Env) -> u32;

    /// Get the type of decentralized storage service.
    fn file_storage(e: Env) -> Bytes;

    /// Get the recipients data in the contract.
    fn recipients(e: Env) -> Map<Address, CertData>;

    // Get all relevant contract data.
    fn info(e: Env) -> Info;
}
