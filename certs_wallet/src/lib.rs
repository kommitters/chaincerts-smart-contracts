#![no_std]
mod access_control_list;
mod chaincert;
mod option;
mod owner;
mod storage_types;
use option::OptU64;
use soroban_sdk::{contractimpl, Address, Bytes, Env};

pub struct Wallet;

#[contractimpl]
impl Wallet {
    pub fn initialize(env: Env, owner: Address) {
        if owner::has_owner(&env) {
            panic!("This wallet is already initialized");
        }
        owner::write_owner(&env, &owner);
    }

    /// Add organizations to the ACL
    pub fn add_org(env: Env, org_id: Bytes) {
        owner::read_owner(&env).require_auth();
        access_control_list::add_organization(&env, &org_id)
    }

    /// Remove organizations from the ACL
    pub fn rmv_org(env: Env, org_id: Bytes) {
        owner::read_owner(&env).require_auth();
        access_control_list::remove_organization(&env, &org_id)
    }

    /// Deposit a `Chaincert` to the wallet
    pub fn deposit_cc(
        env: Env,
        chaincert_id: Bytes,
        cid: Bytes,
        contract_distributor: Address,
        org_id: Bytes,
        distribution_date: u64,
        expiration_date: OptU64,
    ) {
        access_control_list::check_acl(&env, &org_id);
        contract_distributor.require_auth();
        chaincert::deposit_chaincert(
            &env,
            chaincert_id,
            cid,
            contract_distributor,
            org_id,
            distribution_date,
            expiration_date,
        )
    }

    /// Revoke a `Chaincert` from the wallet
    pub fn revoke_cc(env: Env, chaincert_id: Bytes, contract_distributor: Address, org_id: Bytes) {
        contract_distributor.require_auth();
        chaincert::revoke_chaincert(&env, &chaincert_id, &contract_distributor, &org_id);
    }
}

mod test;
