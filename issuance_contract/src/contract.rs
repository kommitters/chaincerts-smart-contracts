//! Module Contract
//!
//! Module containing the main contract logic.
use crate::attest::{
    get_credential_data, get_revoked_credential, invalid_status, is_valid, revoked_status,
    valid_status,
};
use crate::did_contract::{self, OptionU64};
use crate::error::ContractError;
use crate::issuance_trait::{
    CredentialParams, CredentialStatus, IssuanceTrait, VerifiableCredential,
};
use crate::metadata::{
    increment_supply, read_credential_title, read_credential_type, read_distribution_limit,
    read_expiration_time, read_file_storage, read_name, read_revocable, read_revoked_credentials,
    read_supply, write_credential_title, write_credential_type, write_distribution_limit,
    write_expiration_time, write_file_storage, write_name, write_recipients, write_revocable,
    write_revoked_credentials, write_supply,
};
use crate::organization::{
    check_admin, has_organization, read_organization_did, write_organization,
};
use crate::recipients::{add_recipient, create_recipients, read_recipients};
use crate::storage_types::{CredentialData, Info, Organization, RevokedCredential};
use soroban_sdk::{contractimpl, panic_with_error, Address, Bytes, BytesN, Env, Map, String, Vec};

pub struct IssuanceContract;

#[contractimpl]
impl IssuanceTrait for IssuanceContract {
    /// Initialize the contract a list of recipients or with the limit of Chaincerts that can be distributed.
    fn initialize(
        e: Env,
        name: Bytes,
        recipients: Option<Vec<String>>,
        distribution_limit: Option<u32>,
        organization: Organization,
        credential_params: CredentialParams,
    ) {
        if has_organization(&e) {
            panic_with_error!(&e, ContractError::AlreadyInit);
        }
        write_organization(&e, organization);
        write_file_storage(&e, credential_params.file_storage);
        write_name(&e, name);
        write_revocable(&e, credential_params.revocable);
        write_revoked_credentials(&e, Map::<String, RevokedCredential>::new(&e));
        write_expiration_time(&e, credential_params.expiration_time);
        write_supply(&e, 0);
        write_credential_type(&e, credential_params.credential_type);
        write_credential_title(&e, credential_params.credential_title);

        define_limit_and_recipients(e, recipients, distribution_limit);
    }

    /// Distribute a Chaincert to a recipient.
    fn distribute(
        e: Env,
        admin: Address,
        wallet_contract_id: BytesN<32>,
        verifiable_credential: VerifiableCredential,
    ) {
        check_admin(&e, &admin);
        admin.require_auth();
        check_amount(&e);

        apply_distribution(e, wallet_contract_id, &verifiable_credential);
    }

    /// Revoke a Chaincert from a recipient.
    fn revoke(e: Env, admin: Address, recipient_did: String, revocation_date: u64) {
        check_revocable(&e);
        check_admin(&e, &admin);
        admin.require_auth();
        let mut recipients: Map<String, Option<CredentialData>> = read_recipients(&e);
        if let Some(credential_data) = recipients.get(recipient_did.clone()) {
            match credential_data.unwrap() {
                Some(data) => {
                    let mut revoked_credentials = read_revoked_credentials(&e);
                    let revoked_credential = RevokedCredential::new(data, revocation_date);
                    revoked_credentials.set(recipient_did.clone(), revoked_credential);
                    recipients.remove(recipient_did);
                    write_recipients(&e, recipients);
                    write_revoked_credentials(&e, revoked_credentials);
                }
                None => {
                    panic_with_error!(e, ContractError::NoRevocable);
                }
            }
        } else {
            panic_with_error!(e, ContractError::NoRevocable);
        }
    }

    /// Attest the authenticity and legitimacy of a credential.
    fn attest(
        e: Env,
        credential: Bytes,
        issuer: Bytes,
        recipient: String,
        signature: String,
    ) -> CredentialStatus {
        if issuer != read_organization_did(&e) {
            return invalid_status(&e);
        }

        if let Some(revoked_credential) = get_revoked_credential(&e, &recipient) {
            if is_valid(&revoked_credential.credential_data, &credential, &signature) {
                return revoked_status(&e, revoked_credential.revocation_date);
            }
        }

        if let Some(credential_data) = get_credential_data(&e, &recipient) {
            if is_valid(&credential_data, &credential, &signature) {
                return valid_status(&e);
            }
        }

        invalid_status(&e)
    }

    /// Get the Chaincert name.
    fn name(e: Env) -> Bytes {
        read_name(&e)
    }

    /// Get if the Chaincert can be revoked or not.
    fn is_revocable(e: Env) -> bool {
        read_revocable(&e)
    }

    /// Get the Chaincert expiration time (Epoch time).
    fn expiration_time(e: Env) -> OptionU64 {
        read_expiration_time(&e)
    }

    /// Get the maximum number of Chaincerts that can be distributed by this contract.
    fn distribution_limit(e: Env) -> u32 {
        read_distribution_limit(&e)
    }

    /// Get number of Chaincerts that have been distributed.
    fn supply(e: Env) -> u32 {
        read_supply(&e)
    }

    /// Get the type of decentralized storage service.
    fn file_storage(e: Env) -> Bytes {
        read_file_storage(&e)
    }

    /// Get the recipients data in the contract.
    fn recipients(e: Env) -> Map<String, Option<CredentialData>> {
        read_recipients(&e)
    }

    /// Get all relevant contract data.
    fn info(e: Env) -> Info {
        Info {
            name: read_name(&e),
            revocable: read_revocable(&e),
            expiration_time: read_expiration_time(&e),
            distribution_limit: read_distribution_limit(&e),
            supply: read_supply(&e),
            credential_type: read_credential_type(&e),
            credential_title: read_credential_title(&e),
        }
    }

    /// Get all revoked credentials.
    fn revoked_credentials(e: Env, admin: Address) -> Vec<RevokedCredential> {
        check_admin(&e, &admin);
        admin.require_auth();
        read_revoked_credentials(&e).values()
    }
}

/// Defines recipients and distribution_limit depending on the received ones.
fn apply_distribution(
    e: Env,
    wallet_contract_id: BytesN<32>,
    verifiable_credential: &VerifiableCredential,
) {
    match read_recipients(&e).get(verifiable_credential.recipient_did.clone()) {
        Some(_) => {
            distribute_to_recipient(&e, wallet_contract_id, verifiable_credential);
        }
        None => {
            add_recipient(&e, &verifiable_credential.recipient_did);
            distribute_to_recipient(&e, wallet_contract_id, verifiable_credential);
        }
    };
}

/// Defines recipients and distribution_limit depending on the received ones.
fn define_limit_and_recipients(
    e: Env,
    recipients: Option<Vec<String>>,
    distribution_limit: Option<u32>,
) {
    match recipients {
        Some(recipients) => {
            write_distribution_limit(&e, recipients.len());
            create_recipients(&e, recipients);
        }
        None => {
            if let Some(distribution_limit) = distribution_limit {
                write_distribution_limit(&e, distribution_limit);
            } else {
                write_distribution_limit(&e, 10);
            }
            write_recipients(&e, Map::<String, Option<CredentialData>>::new(&e));
        }
    };
}

/// Calculates the expiration date of a distributed Chaincert (using Epoch Unix Timestamp, and Epoch time).
fn expiration_date(e: &Env, issuance_date: u64) -> OptionU64 {
    match read_expiration_time(e) {
        OptionU64::Some(value) => OptionU64::Some(issuance_date + value),
        OptionU64::None => OptionU64::None,
    }
}

/// Checks that no more chain certificates are issued than allowed by the distribution limit.
fn check_amount(e: &Env) {
    if read_supply(e) >= read_distribution_limit(e) {
        panic_with_error!(e, ContractError::LimitReached);
    }
}

/// Checks that the status of the CredentialData of the recipient_data to distribute is Unassigned.
fn check_recipient_status_for_distribute(e: &Env, recipient_data: &Option<CredentialData>) {
    if !matches!(recipient_data, None) {
        panic_with_error!(e, ContractError::AlreadyIssued);
    }
}

fn check_revocable(e: &Env) {
    if !read_revocable(e) {
        panic_with_error!(e, ContractError::NoRevocable);
    }
}

/// Deposit a chaincert to a wallet and makes the necessary storage changes when successful.
fn distribute_to_recipient(
    e: &Env,
    wallet_contract_id: BytesN<32>,
    verifiable_credential: &VerifiableCredential,
) {
    let mut recipients: Map<String, Option<CredentialData>> = read_recipients(e);
    let mut credential_data: Option<CredentialData> = recipients
        .get(verifiable_credential.recipient_did.clone())
        .unwrap()
        .unwrap();
    check_recipient_status_for_distribute(e, &credential_data);
    let credential_type = read_credential_type(e);
    let credential_title = read_credential_title(e);

    deposit_to_wallet(e, wallet_contract_id, verifiable_credential);

    credential_data = Some(CredentialData::new(
        verifiable_credential.did.clone(),
        verifiable_credential.recipient_did.clone(),
        credential_type,
        credential_title,
        OptionU64::Some(verifiable_credential.issuance_date),
        verifiable_credential.signature.clone(),
    ));

    recipients.set(verifiable_credential.recipient_did.clone(), credential_data);
    write_recipients(e, recipients);
    increment_supply(e);
}

/// Invokes a wallet contract to make a chaincert deposit.
fn deposit_to_wallet(
    e: &Env,
    wallet_contract_id: BytesN<32>,
    verifiable_credential: &VerifiableCredential,
) {
    let wallet_client = did_contract::Client::new(e, &wallet_contract_id);
    let distributor_contract = e.current_contract_address();
    let expiration_date: OptionU64 = expiration_date(e, verifiable_credential.issuance_date);
    let org_did = read_organization_did(e);
    wallet_client.deposit_chaincert(
        &verifiable_credential.did,
        &verifiable_credential.attestation,
        &distributor_contract,
        &org_did,
        &verifiable_credential.issuance_date,
        &expiration_date,
    );
}
