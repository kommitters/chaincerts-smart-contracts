#![cfg(test)]

use crate::{option::OptU64, Wallet, WalletClient};
use soroban_sdk::{testutils::Address as _, vec, Address, Bytes, Env, IntoVal, Vec};

fn create_wallet(e: &Env, owner: &Address) -> WalletClient {
    let wallet = WalletClient::new(e, &e.register_contract(None, Wallet {}));
    wallet.initialize(owner);
    wallet
}

struct ChaincertWalletTest {
    env: Env,
    owner: Address,
    distributor_contract: Address,
    wallet: WalletClient,
    chaincert_id: Bytes,
    organizations: Vec<Bytes>,
    cids: Vec<Bytes>,
}

impl ChaincertWalletTest {
    fn setup() -> Self {
        let env: Env = Default::default();
        let owner = Address::random(&env);
        let distributor_contract = Address::random(&env);
        let wallet = create_wallet(&env, &owner);
        let chaincert_id: Bytes = "CHAINCERT1".into_val(&env);
        let org_id1: Bytes = "ORG1".into_val(&env);
        let org_id2: Bytes = "ORG2".into_val(&env);
        let organizations = vec![&env, org_id1, org_id2];
        let cid1: Bytes = "QmdtyfTYbVS3K9iYqBPjXxn4mbB7aBvEjYGzYWnzRcMrEC".into_val(&env);
        let cids = vec![&env, cid1];

        ChaincertWalletTest {
            env,
            owner,
            distributor_contract,
            wallet,
            chaincert_id,
            organizations,
            cids,
        }
    }
}

#[test]
fn test_successful_execution_of_wallet_capabilities() {
    let test = ChaincertWalletTest::setup();
    let new_chiancert_id = "CHAINCERT2".into_val(&test.env);

    test.wallet
        .add_org(&test.organizations.get_unchecked(0).unwrap());
    test.wallet
        .add_org(&test.organizations.get_unchecked(1).unwrap());

    test.wallet.deposit_cc(
        &test.chaincert_id,
        &test.cids.get_unchecked(0).unwrap(),
        &test.distributor_contract,
        &test.organizations.get_unchecked(0).unwrap(),
        &1680105831,
        &OptU64::Some(1711662757),
    );

    test.wallet.deposit_cc(
        &new_chiancert_id,
        &test.cids.get_unchecked(0).unwrap(),
        &test.distributor_contract,
        &test.organizations.get_unchecked(0).unwrap(),
        &1680205831,
        &OptU64::Some(1711662757),
    );

    test.wallet.revoke_cc(
        &test.chaincert_id,
        &test.distributor_contract,
        &test.organizations.get_unchecked(0).unwrap(),
    );

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
#[should_panic(expected = "The organization is already in the ACL")]
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

#[test]
#[should_panic(expected = "Not Authorized")]
fn test_deposit_chaincert_when_organization_is_not_in_the_acl() {
    let test = ChaincertWalletTest::setup();

    test.wallet
        .add_org(&test.organizations.get_unchecked(0).unwrap());

    test.wallet.deposit_cc(
        &test.chaincert_id,
        &test.cids.get(0).unwrap().unwrap(),
        &test.distributor_contract,
        &test.organizations.get_unchecked(1).unwrap(),
        &1680105831,
        &OptU64::Some(1711662757),
    );
}

#[test]
#[should_panic(expected = "There are no organizations in the ACL")]
fn test_deposit_chaincert_when_no_organizations_in_the_acl() {
    let test = ChaincertWalletTest::setup();

    test.wallet.deposit_cc(
        &test.chaincert_id,
        &test.cids.get(0).unwrap().unwrap(),
        &test.distributor_contract,
        &test.organizations.get_unchecked(1).unwrap(),
        &1680105831,
        &OptU64::Some(1711662757),
    );
}

#[test]
#[should_panic(expected = "The chaincert is already deposited in the wallet")]
fn test_deposit_chaincert_chaincert_is_already_in_the_wallet() {
    let test = ChaincertWalletTest::setup();

    test.wallet
        .add_org(&test.organizations.get_unchecked(0).unwrap());
    test.wallet
        .add_org(&test.organizations.get_unchecked(1).unwrap());

    test.wallet.deposit_cc(
        &test.chaincert_id,
        &test.cids.get_unchecked(0).unwrap(),
        &test.distributor_contract,
        &test.organizations.get_unchecked(0).unwrap(),
        &1680105831,
        &OptU64::Some(1711662757),
    );

    test.wallet.deposit_cc(
        &test.chaincert_id,
        &test.cids.get_unchecked(0).unwrap(),
        &test.distributor_contract,
        &test.organizations.get_unchecked(0).unwrap(),
        &1680105831,
        &OptU64::Some(1711662757),
    );
}

#[test]
#[should_panic(expected = "This wallet doesn't own any `chaincert` for the moment")]
fn test_revoke_chaincert_when_no_chaincerts_in_wallet() {
    let test = ChaincertWalletTest::setup();

    test.wallet
        .add_org(&test.organizations.get_unchecked(0).unwrap());
    test.wallet.revoke_cc(
        &test.chaincert_id,
        &test.distributor_contract,
        &test.organizations.get_unchecked(0).unwrap(),
    )
}

#[test]
#[should_panic(expected = "The chaincert doesn't exist")]
fn test_revoke_chaincert_when_chaincert_not_found() {
    let test = ChaincertWalletTest::setup();
    let org1 = test.organizations.get_unchecked(0).unwrap();
    let new_chaincert: Bytes = "CHAINCERT2".into_val(&test.env);

    test.wallet.add_org(&org1);
    test.wallet.deposit_cc(
        &test.chaincert_id,
        &test.cids.get(0).unwrap().unwrap(),
        &test.distributor_contract,
        &org1,
        &1680105831,
        &OptU64::Some(1711662757),
    );

    test.wallet
        .revoke_cc(&new_chaincert, &test.distributor_contract, &org1);
}

#[test]
#[should_panic(expected = "Not Authorized")]
fn test_revoke_chaincert_when_not_authorized_contract_or_organization() {
    let test = ChaincertWalletTest::setup();
    let org1 = test.organizations.get_unchecked(0).unwrap();
    let org2 = test.organizations.get_unchecked(1).unwrap();

    test.wallet.add_org(&org1);
    test.wallet.deposit_cc(
        &test.chaincert_id,
        &test.cids.get(0).unwrap().unwrap(),
        &test.distributor_contract,
        &org1,
        &1680105831,
        &OptU64::Some(1711662757),
    );

    test.wallet
        .revoke_cc(&test.chaincert_id, &test.distributor_contract, &org2);
}
