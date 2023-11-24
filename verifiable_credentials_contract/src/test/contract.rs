use crate::test::setup::VCsContractTest;
use crate::vault_contract;
use soroban_sdk::{testutils::Address as _, vec, Address, String};

#[test]
fn test_initialize_with_amount() {
    let VCsContractTest {
        env: _env,
        admin,
        amount,
        contract,
    } = VCsContractTest::setup();

    contract.initialize(&admin, &amount);
}

#[test]
fn test_initialize_without_amount() {
    let VCsContractTest {
        env: _env,
        admin,
        amount: _,
        contract,
    } = VCsContractTest::setup();

    contract.initialize(&admin, &None);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #3)")]
fn test_initialize_with_too_high_amount() {
    let VCsContractTest {
        env: _env,
        admin,
        amount: _,
        contract,
    } = VCsContractTest::setup();
    let high_amount = Some(101);

    contract.initialize(&admin, &high_amount);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #1)")]
fn test_initialize_an_already_initialized_contract() {
    let VCsContractTest {
        env: _env,
        admin,
        amount,
        contract,
    } = VCsContractTest::setup();

    contract.initialize(&admin, &amount);
    contract.initialize(&admin, &amount);
}

#[test]
fn test_issue() {
    let VCsContractTest {
        env,
        admin,
        amount,
        contract,
    } = VCsContractTest::setup();

    let vc_data = String::from_slice(&env, "vc_data");
    let recipient_did = String::from_slice(&env, "recipient_did");
    let vault_admin = Address::random(&env);

    let vault_contract_id = env.register_contract_wasm(None, vault_contract::WASM);
    let vault_client = vault_contract::Client::new(&env, &vault_contract_id);
    let dids = vec![&env, recipient_did.clone()];

    vault_client.initialize(&vault_admin, &dids);
    vault_client.authorize_issuer(&vault_admin, &admin, &recipient_did);

    contract.initialize(&admin, &amount);
    contract.issue(&admin, &vc_data, &recipient_did, &vault_contract_id);
}
