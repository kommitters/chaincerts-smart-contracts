//! Module IssuanceTrait
//!
//! Interface that defines the behavior of a Issuance contract.
use soroban_sdk::{contracttype, Address, Bytes, BytesN, Env, Map, String, Vec};

use crate::{
    cert_wallet::OptionU64,
    storage_types::{CredentialData, Info, Organization},
};

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct CredentialParams {
    pub file_storage: Bytes,
    pub revocable: bool,
    pub expiration_time: OptionU64,
    pub credential_type: String,
    pub credential_title: String,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct VerifiableCredential {
    pub did: Bytes,
    pub id: Bytes,
    pub recipient_did: String,
    pub signature: String,
    pub attestation: Bytes,
    pub issuance_date: u64,
}

pub trait IssuanceTrait {
    /// Initialize the contract a list of recipients or with the limit of Chaincerts that can be distributed.
    fn initialize(
        e: Env,
        name: Bytes,
        recipients: Option<Vec<String>>,
        distribution_limit: Option<u32>,
        organization: Organization,
        credential_params: CredentialParams,
    );

    /// Distribute a Chaincert to a recipient.
    fn distribute(
        e: Env,
        admin: Address,
        wallet_contract_id: BytesN<32>,
        verifiable_credential: VerifiableCredential,
    );

    /// Revoke a Chaincert from a recipient.
    fn revoke(e: Env, admin: Address, recipient: String, wallet_contract_id: BytesN<32>);

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
    fn recipients(e: Env) -> Map<String, Option<CredentialData>>;

    // Get all relevant contract data.
    fn info(e: Env) -> Info;
}
