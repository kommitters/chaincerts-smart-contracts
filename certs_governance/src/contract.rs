//! Module Contract
//!
//! Module containing the main contract logic.
use crate::governance_trait::GovernanceTrait;
use crate::metadata::{
    create_receivers, read_distribution_limit, read_expiration_time, read_file_storage, read_name,
    read_revocable, write_distribution_limit, write_expiration_time, write_file_storage,
    write_name, write_receivers, write_revocable,
};
use crate::organization::{has_organization, read_organization_id, write_organization};
use crate::storage_types::{CertData, Organization};
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
    }

    #[cfg(not(tarpaulin_include))]
    fn distribute(
        _e: Env,
        _admin: Address,
        _receiver: Address,
        _wallet_contract_id: Bytes,
        _cid: Bytes,
        _distribution_date: u64,
    ) {
    }

    #[cfg(not(tarpaulin_include))]
    fn revoke(_e: Env, _admin: Address, _receiver: Address, _wallet_contract_id: Address) {}

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

    #[cfg(not(tarpaulin_include))]
    fn supply(_e: Env) -> u32 {
        0
    }

    #[cfg(not(tarpaulin_include))]
    fn receivers(e: Env) -> Map<Address, CertData> {
        Map::<Address, CertData>::new(&e)
    }
}
