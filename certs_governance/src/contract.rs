//! Module Contract
//!
//! Module containing the main contract logic.
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
use crate::storage_types::{CertData, Info, Opt, Organization, Status};
use soroban_sdk::{contractimpl, Address, Bytes, Env, Map, Vec};
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
            panic!("Already initialized");
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
            panic!("Already initialized");
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
        _wallet_contract_id: Bytes,
        _cid: Bytes,
        distribution_date: u64,
    ) {
        check_admin(&e, &admin);
        admin.require_auth();
        check_amount(&e);
        let _expiration_date: Option<u64> = expiration_date(&e, distribution_date);
        match read_receivers(&e).get(receiver.clone()) {
            Some(_cert_data) => {
                distribute_receiver(&e, &receiver, distribution_date);
                //TODO: distribute to wallet
            }
            None => {
                add_receiver(&e, &receiver);
                distribute_receiver(&e, &receiver, distribution_date);
                //TODO: distribute to wallet
            }
        };
    }

    fn revoke(e: Env, admin: Address, receiver: Address, _wallet_contract_id: Bytes) {
        check_admin(&e, &admin);
        admin.require_auth();
        let mut receivers: Map<Address, CertData> = read_receivers(&e);
        let mut cert_data: CertData = receivers.get(receiver.clone()).unwrap().unwrap();
        check_receiver_status_for_revoke(&cert_data);
        cert_data.status = Status::Revoked;
        receivers.set(receiver, cert_data);
        write_receivers(&e, receivers);
        //TODO: revoke from wallet
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

    fn org(e: Env) -> Bytes {
        read_organization_id(&e)
    }

    fn supply(e: Env) -> u32 {
        read_supply(&e)
    }

    fn receivers(e: Env) -> Map<Address, CertData> {
        read_receivers(&e)
    }

    fn info(e: Env) -> Info {
        let exp_time = match read_expiration_time(&e) {
            Some(value) => Opt::Some(value),
            None => Opt::None,
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
        panic!("It is not possible to issue more Chaincerts")
    }
}

/// Checks that the status of the CertData of the receiver to distribute is Unassigned.
fn check_receiver_status_for_distribute(receiver_data: &CertData) {
    if receiver_data.status != Status::Unassigned {
        panic!("Chaincert has already been issued to the entered address")
    }
}

/// Checks that the status of the CertData of the receiver to revoke is Distributed.
fn check_receiver_status_for_revoke(receiver_data: &CertData) {
    if receiver_data.status != Status::Distribute {
        panic!("Chaincert cannot be revoked")
    }
}

/// Makes the necessary storage changes to distribute.
fn distribute_receiver(e: &Env, address: &Address, dist_date: u64) {
    let mut receivers: Map<Address, CertData> = read_receivers(e);
    let mut cert_data: CertData = receivers.get(address.clone()).unwrap().unwrap();
    check_receiver_status_for_distribute(&cert_data);
    cert_data.status = Status::Distribute;
    cert_data.dist_date = Opt::Some(dist_date);
    receivers.set(address.clone(), cert_data);
    write_receivers(e, receivers);
    increment_supply(e);
}
