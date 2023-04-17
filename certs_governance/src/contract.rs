//! Module Contract
//!
//! Module containing the main contract logic.
use crate::certs_wallet::{self, OptionU64};
use crate::error::ContractError;
use crate::governance_trait::GovernanceTrait;
use crate::metadata::{
    increment_supply, read_distribution_limit, read_expiration_time, read_file_storage, read_name,
    read_revocable, read_supply, write_distribution_limit, write_expiration_time,
    write_file_storage, write_name, write_receivers, write_revocable, write_supply,
};
use crate::organization::{
    check_admin, has_organization, read_organization_id, write_organization,
};
use crate::receivers::{add_receiver, create_receivers, read_receivers};
use crate::storage_types::{CertData, Info, Organization, Status};
use soroban_sdk::{contractimpl, panic_with_error, Address, Bytes, BytesN, Env, Map, Vec};
pub struct CertGovernance;

#[contractimpl]
impl GovernanceTrait for CertGovernance {
    /// Initialize the contract a list of receivers or with the limit of Chaincerts that can be distributed.
    fn initialize(
        e: Env,
        file_storage: Bytes,
        name: Bytes,
        receivers: Option<Vec<Address>>,
        distribution_limit: Option<u32>,
        governance_rules: (bool, OptionU64), // (revocable, expiration_time)
        organization: Organization,
    ) {
        if has_organization(&e) {
            panic_with_error!(&e, ContractError::AlreadyInit);
        }
        write_organization(&e, organization);
        write_file_storage(&e, file_storage);
        write_name(&e, name);
        write_revocable(&e, governance_rules.0);
        write_expiration_time(&e, governance_rules.1);
        write_supply(&e, 0);

        define_limit_and_receivers(e, receivers, distribution_limit);
    }

    /// Distribute a Chaincert to a receiver.
    fn distribute(
        e: Env,
        admin: Address,
        receiver: Address,
        wallet_contract_id: BytesN<32>,
        cid: Bytes,
        distribution_date: u64,
    ) {
        check_admin(&e, &admin);
        admin.require_auth();
        check_amount(&e);

        apply_distribution(e, receiver, wallet_contract_id, cid, distribution_date);
    }

    /// Revoke a Chaincert from a holder.
    fn revoke(e: Env, admin: Address, holder: Address, wallet_contract_id: BytesN<32>) {
        check_revocable(&e);
        check_admin(&e, &admin);
        admin.require_auth();
        let mut receivers: Map<Address, CertData> = read_receivers(&e);
        let mut cert_data: CertData = receivers.get(holder.clone()).unwrap().unwrap();
        check_receiver_status_for_revoke(&e, &cert_data);

        revoke_from_wallet(&e, wallet_contract_id, &cert_data.id);
        cert_data.status = Status::Revoked;
        receivers.set(holder, cert_data);
        write_receivers(&e, receivers);
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

    /// Get the receivers data in the contract.
    fn receivers(e: Env) -> Map<Address, CertData> {
        read_receivers(&e)
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

/// Defines receivers and distribution_limit depending on the received ones.
fn apply_distribution(
    e: Env,
    receiver: Address,
    wallet_contract_id: BytesN<32>,
    cid: Bytes,
    distribution_date: u64,
) {
    match read_receivers(&e).get(receiver.clone()) {
        Some(_) => {
            distribute_receiver(&e, &receiver, distribution_date, wallet_contract_id, cid);
        }
        None => {
            add_receiver(&e, &receiver);
            distribute_receiver(&e, &receiver, distribution_date, wallet_contract_id, cid);
        }
    };
}

/// Defines receivers and distribution_limit depending on the received ones.
fn define_limit_and_receivers(
    e: Env,
    receivers: Option<Vec<Address>>,
    distribution_limit: Option<u32>,
) {
    match receivers {
        Some(receivers) => {
            write_distribution_limit(&e, receivers.len());
            create_receivers(&e, receivers);
        }
        None => {
            if let Some(distribution_limit) = distribution_limit {
                write_distribution_limit(&e, distribution_limit);
            } else {
                write_distribution_limit(&e, 10);
            }
            write_receivers(&e, Map::<Address, CertData>::new(&e));
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

/// Checks that the status of the CertData of the receiver to distribute is Unassigned.
fn check_receiver_status_for_distribute(e: &Env, receiver_data: &CertData) {
    if receiver_data.status != Status::Unassigned {
        panic_with_error!(e, ContractError::AlreadyIssued);
    }
}

/// Checks that the status of the CertData of the receiver to revoke is Distributed.
fn check_receiver_status_for_revoke(e: &Env, receiver_data: &CertData) {
    if receiver_data.status != Status::Distribute {
        panic_with_error!(e, ContractError::NoRevocable);
    }
}

fn check_revocable(e: &Env) {
    if !read_revocable(e) {
        panic_with_error!(e, ContractError::NoRevocable);
    }
}

/// Deposit a chaincert to a wallet and makes the necessary storage changes when successful.
fn distribute_receiver(
    e: &Env,
    address: &Address,
    distribution_date: u64,
    wallet_contract_id: BytesN<32>,
    cid: Bytes,
) {
    let mut receivers: Map<Address, CertData> = read_receivers(e);
    let mut cert_data: CertData = receivers.get(address.clone()).unwrap().unwrap();
    check_receiver_status_for_distribute(e, &cert_data);

    deposit_to_wallet(
        e,
        wallet_contract_id,
        cert_data.id.clone(),
        cid,
        distribution_date,
    );

    cert_data.status = Status::Distribute;
    cert_data.distribution_date = OptionU64::Some(distribution_date);
    receivers.set(address.clone(), cert_data);
    write_receivers(e, receivers);
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
    let wallet_client = certs_wallet::Client::new(e, &wallet_contract_id);
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
    let wallet_client = certs_wallet::Client::new(e, &wallet_contract_id);
    let distributor_contract = e.current_contract_address();
    let org_id = read_organization_id(e);
    wallet_client.revoke_chaincert(chaincert_id, &distributor_contract, &org_id);
}
