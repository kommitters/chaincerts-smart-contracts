#![no_std]
mod chaincert;
mod option;
mod owner;
mod storage_types;
use soroban_sdk::{contractimpl, Address, Env};

pub struct Wallet;

#[contractimpl]
impl Wallet {
    pub fn initialize(env: Env, owner: Address) {
        if owner::has_owner(&env) {
            panic!("This wallet is already initialized");
        }
        owner::write_owner(&env, &owner);
    }
}

mod test;
