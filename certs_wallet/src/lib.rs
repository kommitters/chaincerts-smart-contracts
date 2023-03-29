#![no_std]
mod access_control_list;
mod chaincert;
mod option;
mod owner;
mod storage_types;
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
}

mod test;
