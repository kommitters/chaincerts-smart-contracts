use crate::test::setup::VCIssuanceContractTest;
use soroban_sdk::{testutils::Address as _, Address, String};

use super::setup::create_vc;

#[test]
fn test_initialize_with_amount() {
    let VCIssuanceContractTest {
        env: _env,
        admin,
        amount,
        contract,
    } = VCIssuanceContractTest::setup();

    contract.initialize(&admin, &amount);
}

#[test]
fn test_initialize_without_amount() {
    let VCIssuanceContractTest {
        env: _env,
        admin,
        amount: _,
        contract,
    } = VCIssuanceContractTest::setup();

    contract.initialize(&admin, &None);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #3)")]
fn test_initialize_with_too_high_amount() {
    let VCIssuanceContractTest {
        env: _env,
        admin,
        amount: _,
        contract,
    } = VCIssuanceContractTest::setup();
    let high_amount = Some(101);

    contract.initialize(&admin, &high_amount);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #1)")]
fn test_initialize_an_already_initialized_contract() {
    let VCIssuanceContractTest {
        env: _env,
        admin,
        amount,
        contract,
    } = VCIssuanceContractTest::setup();

    contract.initialize(&admin, &amount);
    contract.initialize(&admin, &amount);
}

#[test]
fn test_issue() {
    let VCIssuanceContractTest {
        env,
        admin,
        amount: _,
        contract,
    } = VCIssuanceContractTest::setup();
    let vc_data = String::from_slice(&env, "eoZXggNeVDW2g5GeA0G2s0QJBn3SZWzWSE3fXM9V6IB5wWIfFJRxPrTLQRMHulCF62bVQNmZkj7zbSa39fVjAUTtfm6JMio75uMxoDlAN/Y");
    let recipient_did = String::from_slice(&env, "did:chaincerts:pe4t2r94dftr1n1gf6jikt6a");

    let vault_contract_id = create_vc(&env, &admin, &contract, &recipient_did);
    contract.issue(&admin, &vc_data, &recipient_did, &vault_contract_id);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #2)")]
fn test_issue_with_invalid_admin() {
    let VCIssuanceContractTest {
        env,
        admin,
        amount: _,
        contract,
    } = VCIssuanceContractTest::setup();
    let invalid_admin = Address::random(&env);

    let vc_data = String::from_slice(&env, "eoZXggNeVDW2g5GeA0G2s0QJBn3SZWzWSE3fXM9V6IB5wWIfFJRxPrTLQRMHulCF62bVQNmZkj7zbSa39fVjAUTtfm6JMio75uMxoDlAN/Y");
    let recipient_did = String::from_slice(&env, "did:chaincerts:pe4t2r94dftr1n1gf6jikt6a");

    let vault_contract_id = create_vc(&env, &admin, &contract, &recipient_did);
    contract.issue(&invalid_admin, &vc_data, &recipient_did, &vault_contract_id);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #4)")]
fn test_revoke_vc_with_invalid_vc() {
    let VCIssuanceContractTest {
        env,
        admin,
        amount: _,
        contract,
    } = VCIssuanceContractTest::setup();
    contract.initialize(&admin, &Some(10));

    let vc_id = String::from_slice(&env, "vc_id1");
    let date = String::from_slice(&env, "date1");

    contract.revoke(&admin, &vc_id, &date);
}

#[test]
fn test_revoke_vc() {
    let VCIssuanceContractTest {
        env,
        admin,
        amount: _,
        contract,
    } = VCIssuanceContractTest::setup();
    let vc_data = String::from_slice(&env, "vc_data");
    let recipient_did = String::from_slice(&env, "recipient_did");
    let vault_contract_id = create_vc(&env, &admin, &contract, &recipient_did);
    let vc_id = contract.issue(&admin, &vc_data, &recipient_did, &vault_contract_id);

    let date = String::from_slice(&env, "date1");

    contract.revoke(&admin, &vc_id, &date);
}

#[test]
fn test_verify_vc() {
    let VCIssuanceContractTest {
        env,
        admin,
        amount: _,
        contract,
    } = VCIssuanceContractTest::setup();
    let vc_data = String::from_slice(&env, "vc_data");
    let recipient_did = String::from_slice(&env, "recipient_did");
    let vault_contract_id = create_vc(&env, &admin, &contract, &recipient_did);
    let vc_id = contract.issue(&admin, &vc_data, &recipient_did, &vault_contract_id);

    assert!(contract.verify(&vc_id));
}

#[test]
fn test_verify_vc_with_revoked_vc() {
    let VCIssuanceContractTest {
        env,
        admin,
        amount: _,
        contract,
    } = VCIssuanceContractTest::setup();
    let vc_data = String::from_slice(&env, "vc_data");
    let recipient_did = String::from_slice(&env, "recipient_did");
    let vault_contract_id = create_vc(&env, &admin, &contract, &recipient_did);
    let vc_id = contract.issue(&admin, &vc_data, &recipient_did, &vault_contract_id);
    let date = String::from_slice(&env, "date");

    contract.revoke(&admin, &vc_id, &date);

    assert!(!contract.verify(&vc_id));
}
