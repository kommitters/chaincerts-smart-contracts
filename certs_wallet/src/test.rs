#![cfg(test)]

use crate::{Wallet, WalletClient};
use soroban_sdk::{testutils::Address as _, Address, Env};

fn create_wallet(e: &Env, owner: &Address) -> WalletClient {
    let wallet = WalletClient::new(e, &e.register_contract(None, Wallet {}));
    wallet.initialize(owner);
    wallet
}

#[test]
fn test_successful_execution_of_wallet_capabilities() {
    let env: Env = Default::default();
    let owner = Address::random(&env);
    create_wallet(&env, &owner);
}

#[test]
#[should_panic(expected = "This wallet is already initialized")]
fn test_initialize_an_already_initialized_wallet() {
    let env: Env = Default::default();
    let owner = Address::random(&env);
    let wallet = create_wallet(&env, &owner);
    wallet.initialize(&owner);
}
