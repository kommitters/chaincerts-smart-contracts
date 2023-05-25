//! Module IssuanceTrait
//!
//! Interface that defines the behavior of a Issuance contract.
use soroban_sdk::{contracttype, Address, BytesN, Env, Map, String, Vec};

use crate::{
    did_contract::OptionU64,
    storage_types::{CredentialData, Info, Organization, RevokedCredential},
};

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct CredentialParams {
    pub file_storage: String,
    pub revocable: bool,
    pub credential_type: String,
    pub credential_title: String,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct DistributeCredential {
    pub did: String,
    pub id: String,
    pub recipient_did: String,
    pub signature: String,
    pub attestation: String,
    pub issuance_date: u64,
    pub expiration_date: OptionU64,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct CredentialStatus {
    pub status: String,
    pub expiration_date: OptionU64,
    pub revocation_date: OptionU64,
}

pub trait IssuanceTrait {
    /// Initialize the contract a list of recipients or with the limit of Credentials that can be distributed.
    fn initialize(
        e: Env,
        name: String,
        recipients: Option<Vec<String>>,
        distribution_limit: Option<u32>,
        organization: Organization,
        credential_params: CredentialParams,
    );

    /// Distribute a Credential to a recipient.
    fn distribute(
        e: Env,
        admin: Address,
        did_contract_id: BytesN<32>,
        verifiable_credential: DistributeCredential,
    );

    /// Revoke a Credential from a recipient.
    fn revoke(e: Env, admin: Address, recipient: String, revocation_date: u64);

    /// Attest the authenticity and legitimacy of a credential.
    fn attest(
        e: Env,
        credential: String,
        issuer: String,
        recipient: String,
        signature: String,
    ) -> CredentialStatus;

    /// Get the Credential name.
    fn name(e: Env) -> String;

    /// Get if the Credential can be revoked or not.
    fn is_revocable(e: Env) -> bool;

    /// Get the maximum number of Credentials that can be distributed by this contract.
    fn distribution_limit(e: Env) -> u32;

    /// Get number of Credentials that have been distributed.
    fn supply(e: Env) -> u32;

    /// Get the type of decentralized storage service.
    fn file_storage(e: Env) -> String;

    /// Get the recipients data in the contract.
    fn recipients(e: Env) -> Map<String, Option<CredentialData>>;

    /// Get all relevant contract data.
    fn info(e: Env) -> Info;

    /// Get all revoked credentials.
    fn revoked_credentials(e: Env, admin: Address) -> Vec<RevokedCredential>;
}
