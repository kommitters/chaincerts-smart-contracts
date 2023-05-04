//! Module Contract
//!
//! Module containing the main contract logic.
use crate::cert_wallet::{self, OptionU64};
use crate::error::ContractError;
use crate::issuance_trait::IssuanceTrait;
use crate::metadata::{
    increment_supply, read_distribution_limit, read_expiration_time, read_file_storage, read_name,
    read_revocable, read_supply, write_distribution_limit, write_expiration_time,
    write_file_storage, write_name, write_recipients, write_revocable, write_supply,
};
use crate::organization::{
    check_admin, has_organization, read_organization_id, write_organization,
};
use crate::recipients::{add_recipient, create_recipients, read_recipients};
use crate::storage_types::{CertData, Info, Organization, Status};
use soroban_sdk::{contractimpl, panic_with_error, Address, Bytes, BytesN, Env, Map, Vec};
pub struct CertIssuance;

#[contractimpl]
impl IssuanceTrait for CertIssuance {
    /// Initialize the contract a list of recipients or with the limit of Chaincerts that can be distributed.
    fn initialize(
        e: Env,
        file_storage: Bytes,
        name: Bytes,
        recipients: Option<Vec<Address>>,
        distribution_limit: Option<u32>,
        administration_rules: (bool, OptionU64), // (revocable, expiration_time)
        organization: Organization,
    ) {
        if has_organization(&e) {
            panic_with_error!(&e, ContractError::AlreadyInit);
        }
        write_organization(&e, organization);
        write_file_storage(&e, file_storage);
        write_name(&e, name);
        write_revocable(&e, administration_rules.0);
        write_expiration_time(&e, administration_rules.1);
        write_supply(&e, 0);

        define_limit_and_recipients(e, recipients, distribution_limit);
    }

    /// Distribute a Chaincert to a recipient.
    fn distribute(
        e: Env,
        admin: Address,
        recipient: Address,
        wallet_contract_id: BytesN<32>,
        cid: Bytes,
        distribution_date: u64,
    ) {
        check_admin(&e, &admin);
        admin.require_auth();
        check_amount(&e);

        apply_distribution(e, recipient, wallet_contract_id, cid, distribution_date);
    }

    /// Revoke a Chaincert from a holder.
    fn revoke(e: Env, admin: Address, holder: Address, wallet_contract_id: BytesN<32>) {
        check_revocable(&e);
        check_admin(&e, &admin);
        admin.require_auth();
        let mut recipients: Map<Address, CertData> = read_recipients(&e);
        let mut cert_data: CertData = recipients.get(holder.clone()).unwrap().unwrap();
        check_recipient_status_for_revoke(&e, &cert_data);

        revoke_from_wallet(&e, wallet_contract_id, &cert_data.id);
        cert_data.status = Status::Revoked;
        recipients.set(holder, cert_data);
        write_recipients(&e, recipients);
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
    fn recipients(e: Env) -> Map<Address, CertData> {
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
        }
    }
}

/// Defines recipients and distribution_limit depending on the received ones.
fn apply_distribution(
    e: Env,
    recipient: Address,
    wallet_contract_id: BytesN<32>,
    cid: Bytes,
    distribution_date: u64,
) {
    match read_recipients(&e).get(recipient.clone()) {
        Some(_) => {
            distribute_to_recipient(&e, &recipient, distribution_date, wallet_contract_id, cid);
        }
        None => {
            add_recipient(&e, &recipient);
            distribute_to_recipient(&e, &recipient, distribution_date, wallet_contract_id, cid);
        }
    };
}

/// Defines recipients and distribution_limit depending on the received ones.
fn define_limit_and_recipients(
    e: Env,
    recipients: Option<Vec<Address>>,
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
            write_recipients(&e, Map::<Address, CertData>::new(&e));
        }
    };
}

/// Calculates the expiration date of a distributed Chaincert (using Epoch Unix Timestamp, and Epoch time).
fn expiration_date(e: &Env, distribution_date: u64) -> OptionU64 {
    match read_expiration_time(e) {
        OptionU64::Some(value) => OptionU64::Some(distribution_date + value),
        OptionU64::None => OptionU64::None,
    }
}

/// Checks that no more chain certificates are issued than allowed by the distribution limit.
fn check_amount(e: &Env) {
    if read_supply(e) >= read_distribution_limit(e) {
        panic_with_error!(e, ContractError::LimitReached);
    }
}

/// Checks that the status of the CertData of the recipient_data to distribute is Unassigned.
fn check_recipient_status_for_distribute(e: &Env, recipient_data: &CertData) {
    if recipient_data.status != Status::Unassigned {
        panic_with_error!(e, ContractError::AlreadyIssued);
    }
}

/// Checks that the status of the CertData of the recipient_data to revoke is Distributed.
fn check_recipient_status_for_revoke(e: &Env, recipient_data: &CertData) {
    if recipient_data.status != Status::Distribute {
        panic_with_error!(e, ContractError::NoRevocable);
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
    address: &Address,
    distribution_date: u64,
    wallet_contract_id: BytesN<32>,
    cid: Bytes,
) {
    let mut recipients: Map<Address, CertData> = read_recipients(e);
    let mut cert_data: CertData = recipients.get(address.clone()).unwrap().unwrap();
    check_recipient_status_for_distribute(e, &cert_data);

    deposit_to_wallet(
        e,
        wallet_contract_id,
        cert_data.id.clone(),
        cid,
        distribution_date,
    );

    cert_data.status = Status::Distribute;
    cert_data.distribution_date = OptionU64::Some(distribution_date);
    recipients.set(address.clone(), cert_data);
    write_recipients(e, recipients);
    increment_supply(e);
}

/// Invokes a wallet contract to make a chaincert deposit.
fn deposit_to_wallet(
    e: &Env,
    wallet_contract_id: BytesN<32>,
    chaincert_id: Bytes,
    cid: Bytes,
    distribution_date: u64,
) {
    let wallet_client = cert_wallet::Client::new(e, &wallet_contract_id);
    let distributor_contract = e.current_contract_address();
    let expiration_date: OptionU64 = expiration_date(e, distribution_date);
    let org_id = read_organization_id(e);
    wallet_client.deposit_chaincert(
        &chaincert_id,
        &cid,
        &distributor_contract,
        &org_id,
        &distribution_date,
        &expiration_date,
    );
}

/// Invokes a wallet contract to execute a chaincert revocation.
fn revoke_from_wallet(e: &Env, wallet_contract_id: BytesN<32>, chaincert_id: &Bytes) {
    let wallet_client = cert_wallet::Client::new(e, &wallet_contract_id);
    let distributor_contract = e.current_contract_address();
    let org_id = read_organization_id(e);
    wallet_client.revoke_chaincert(chaincert_id, &distributor_contract, &org_id);
}
