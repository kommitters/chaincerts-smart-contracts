#![cfg(test)]

use crate::{Wallet, WalletClient};
use soroban_sdk::{testutils::Address as _, vec, Address, Bytes, Env, IntoVal, Vec};

fn create_wallet(e: &Env, owner: &Address) -> WalletClient {
    let wallet = WalletClient::new(e, &e.register_contract(None, Wallet {}));
    wallet.initialize(owner);
    wallet
}

struct ChaincertWalletTest {
    owner: Address,
    wallet: WalletClient,
    organizations: Vec<Bytes>,
}

impl ChaincertWalletTest {
    fn setup() -> Self {
        let env: Env = Default::default();
        let owner = Address::random(&env);
        let wallet = create_wallet(&env, &owner);
        let org_id1: Bytes = "ORG1".into_val(&env);
        let org_id2: Bytes = "ORG2".into_val(&env);
        let organizations = vec![&env, org_id1, org_id2];

        ChaincertWalletTest {
            owner,
            wallet,
            organizations,
        }
    }
}

#[test]
fn test_successful_execution_of_wallet_capabilities() {
    let test = ChaincertWalletTest::setup();

    test.wallet
        .add_org(&test.organizations.get_unchecked(0).unwrap());
    test.wallet
        .add_org(&test.organizations.get_unchecked(1).unwrap());
    test.wallet
        .rmv_org(&test.organizations.get_unchecked(0).unwrap());
}

#[test]
#[should_panic(expected = "This wallet is already initialized")]
fn test_initialize_an_already_initialized_wallet() {
    let test = ChaincertWalletTest::setup();
    test.wallet.initialize(&test.owner);
}

#[test]
#[should_panic(expected = "The organization is already on the ACL")]
fn test_when_adding_an_already_added_org() {
    let test = ChaincertWalletTest::setup();

    test.wallet
        .add_org(&test.organizations.get_unchecked(0).unwrap());
    test.wallet
        .add_org(&test.organizations.get_unchecked(0).unwrap());
}

#[test]
#[should_panic(expected = "There are no organizations in the ACL")]
fn test_remove_organization_when_not_organizations_already_set() {
    let test = ChaincertWalletTest::setup();
    test.wallet
        .rmv_org(&test.organizations.get_unchecked(0).unwrap());
}

#[test]
#[should_panic(expected = "The organization doesn't exist in the ACL")]
fn test_remove_organization_when_organization_not_found() {
    let test = ChaincertWalletTest::setup();
    test.wallet
        .add_org(&test.organizations.get_unchecked(0).unwrap());
    test.wallet
        .rmv_org(&test.organizations.get_unchecked(1).unwrap());
}
