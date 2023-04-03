//! Module Contract
//!
//! Module containing the main contract logic.
use crate::certs_wallet::{self, OptU64};
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
    fn init_w_r(
        e: Env,
        file_storage: Bytes,
        name: Bytes,
        revocable: bool,
        expiration_time: Option<u64>,
        receivers: Vec<Address>,
        organization: Organization,
    ) {
        if has_organization(&e) {
            panic_with_error!(&e, ContractError::AlreadyInit);
        }
        write_organization(&e, organization);
        write_file_storage(&e, file_storage);
        write_name(&e, name);
        write_revocable(&e, revocable);
        write_expiration_time(&e, expiration_time);
        write_distribution_limit(&e, receivers.len());
        create_receivers(&e, receivers);
        write_supply(&e, 0);
    }

    fn init_w_l(
        e: Env,
        file_storage: Bytes,
        name: Bytes,
        revocable: bool,
        expiration_time: Option<u64>,
        distribution_limit: u32,
        organization: Organization,
    ) {
        if has_organization(&e) {
            panic_with_error!(&e, ContractError::AlreadyInit);
        }
        write_organization(&e, organization);
        write_file_storage(&e, file_storage);
        write_name(&e, name);
        write_revocable(&e, revocable);
        write_expiration_time(&e, expiration_time);
        write_distribution_limit(&e, distribution_limit);
        write_receivers(&e, Map::<Address, CertData>::new(&e));
        write_supply(&e, 0);
    }

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

    fn revoke(e: Env, admin: Address, receiver: Address, wallet_contract_id: BytesN<32>) {
        check_revocable(&e);
        check_admin(&e, &admin);
        admin.require_auth();
        let mut receivers: Map<Address, CertData> = read_receivers(&e);
        let mut cert_data: CertData = receivers.get(receiver.clone()).unwrap().unwrap();
        check_receiver_status_for_revoke(&e, &cert_data);

        revoke_from_wallet(&e, wallet_contract_id, &cert_data.id_cert);
        cert_data.status = Status::Revoked;
        receivers.set(receiver, cert_data);
        write_receivers(&e, receivers);
    }

    fn name(e: Env) -> Bytes {
        read_name(&e)
    }

    fn revocable(e: Env) -> bool {
        read_revocable(&e)
    }

    fn exp_time(e: Env) -> Option<u64> {
        read_expiration_time(&e)
    }

    fn dist_limit(e: Env) -> u32 {
        read_distribution_limit(&e)
    }

    fn f_storage(e: Env) -> Bytes {
        read_file_storage(&e)
    }

    fn supply(e: Env) -> u32 {
        read_supply(&e)
    }

    fn receivers(e: Env) -> Map<Address, CertData> {
        read_receivers(&e)
    }

    fn info(e: Env) -> Info {
        let exp_time = match read_expiration_time(&e) {
            Some(value) => OptU64::Some(value),
            None => OptU64::None,
        };
        Info {
            name: read_name(&e),
            revocable: read_revocable(&e),
            exp_time,
            dist_limit: read_distribution_limit(&e),
            supply: read_supply(&e),
        }
    }
}

/// Calculates the expiration date of a distributed Chaincert (using Unix time).
fn expiration_date(e: &Env, distribution_date: u64) -> Option<u64> {
    read_expiration_time(e).map(|exp_time| distribution_date + exp_time)
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
        cert_data.id_cert.clone(),
        cid,
        distribution_date,
    );

    cert_data.status = Status::Distribute;
    cert_data.dist_date = OptU64::Some(distribution_date);
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
    let expiration_date: Option<u64> = expiration_date(e, distribution_date);
    let org_id = read_organization_id(e);
    wallet_client.deposit_cc(
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
    wallet_client.revoke_cc(chaincert_id, &distributor_contract, &org_id);
}
