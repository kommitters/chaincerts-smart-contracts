use crate::test::setup::{create_vc, get_revoked_vc_map, get_valid_vc_map, VCIssuanceContractTest};
use soroban_sdk::{testutils::Address as _, Address, String};

#[test]
fn test_initialize_with_amount() {
    let VCIssuanceContractTest {
        env: _env,
        admin,
        amount,
        vc_data: _,
        issuer_did,
        contract,
    } = VCIssuanceContractTest::setup();

    contract.initialize(&admin, &issuer_did, &amount);
}

#[test]
fn test_initialize_without_amount() {
    let VCIssuanceContractTest {
        env: _env,
        admin,
        amount: _,
        vc_data: _,
        issuer_did,
        contract,
    } = VCIssuanceContractTest::setup();

    contract.initialize(&admin, &issuer_did, &None);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #3)")]
fn test_initialize_with_too_high_amount() {
    let VCIssuanceContractTest {
        env: _env,
        admin,
        amount: _,
        vc_data: _,
        issuer_did,
        contract,
    } = VCIssuanceContractTest::setup();
    let high_amount = Some(101);

    contract.initialize(&admin, &issuer_did, &high_amount);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #1)")]
fn test_initialize_an_already_initialized_contract() {
    let VCIssuanceContractTest {
        env: _env,
        admin,
        amount,
        vc_data: _,
        issuer_did,
        contract,
    } = VCIssuanceContractTest::setup();

    contract.initialize(&admin, &issuer_did, &amount);
    contract.initialize(&admin, &issuer_did, &amount);
}

#[test]
fn test_issue() {
    let VCIssuanceContractTest {
        env,
        admin,
        amount: _,
        vc_data,
        issuer_did,
        contract,
    } = VCIssuanceContractTest::setup();

    let vault_contract_id = create_vc(&env, &admin, &contract, &issuer_did, &None);
    contract.issue(&admin, &vc_data, &vault_contract_id);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #2)")]
fn test_issue_with_invalid_admin() {
    let VCIssuanceContractTest {
        env,
        admin,
        amount: _,
        vc_data,
        issuer_did,
        contract,
    } = VCIssuanceContractTest::setup();
    let invalid_admin = Address::generate(&env);

    let vault_contract_id = create_vc(&env, &admin, &contract, &issuer_did, &None);
    contract.issue(&invalid_admin, &vc_data, &vault_contract_id);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #6)")]
fn test_issue_when_amount_is_exceeded() {
    let VCIssuanceContractTest {
        env,
        admin,
        amount: _,
        vc_data,
        issuer_did,
        contract,
    } = VCIssuanceContractTest::setup();
    let vault_contract_id = create_vc(&env, &admin, &contract, &issuer_did, &Some(1));
    contract.issue(&admin, &vc_data, &vault_contract_id);
    contract.issue(&admin, &vc_data, &vault_contract_id);
}

#[test]
fn test_revoke_vc() {
    let VCIssuanceContractTest {
        env,
        admin,
        amount: _,
        vc_data,
        issuer_did,
        contract,
    } = VCIssuanceContractTest::setup();
    let vault_contract_id = create_vc(&env, &admin, &contract, &issuer_did, &None);
    let vc_id = contract.issue(&admin, &vc_data, &vault_contract_id);

    let date = String::from_str(&env, "2023-12-05T21:37:44.389Z");

    contract.revoke(&admin, &vc_id, &date);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #4)")]
fn test_revoke_vc_with_invalid_vc() {
    let VCIssuanceContractTest {
        env,
        admin,
        amount: _,
        vc_data: _,
        issuer_did,
        contract,
    } = VCIssuanceContractTest::setup();
    contract.initialize(&admin, &issuer_did, &Some(10));

    let vc_id = String::from_str(&env, "vc_id1");
    let date = String::from_str(&env, "2023-12-05T21:37:44.389Z");

    contract.revoke(&admin, &vc_id, &date);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #5)")]
fn test_revoke_vc_when_it_was_already_revoked() {
    let VCIssuanceContractTest {
        env,
        admin,
        amount: _,
        vc_data,
        issuer_did,
        contract,
    } = VCIssuanceContractTest::setup();
    let vault_contract_id = create_vc(&env, &admin, &contract, &issuer_did, &None);
    let vc_id = contract.issue(&admin, &vc_data, &vault_contract_id);

    let date_1 = String::from_str(&env, "2023-12-05T21:37:44.389Z");
    let date_2 = String::from_str(&env, "2023-21-05T21:37:44.389Z");

    contract.revoke(&admin, &vc_id, &date_1);
    contract.revoke(&admin, &vc_id, &date_2);
}

#[test]
fn test_verify_vc() {
    let VCIssuanceContractTest {
        env,
        admin,
        amount: _,
        vc_data,
        issuer_did,
        contract,
    } = VCIssuanceContractTest::setup();
    let vault_contract_id = create_vc(&env, &admin, &contract, &issuer_did, &None);
    let vc_id = contract.issue(&admin, &vc_data, &vault_contract_id);

    let valid_vc_map = get_valid_vc_map(&env);
    assert_eq!(contract.verify(&vc_id), valid_vc_map)
}

#[test]
fn test_verify_vc_with_revoked_vc() {
    let VCIssuanceContractTest {
        env,
        admin,
        amount: _,
        vc_data,
        issuer_did,
        contract,
    } = VCIssuanceContractTest::setup();
    let vault_contract_id = create_vc(&env, &admin, &contract, &issuer_did, &None);
    let vc_id = contract.issue(&admin, &vc_data, &vault_contract_id);
    let date = String::from_str(&env, "2023-12-05T21:37:44.389Z");

    contract.revoke(&admin, &vc_id, &date);

    let revoked_vc_map = get_revoked_vc_map(&env, date);
    assert_eq!(contract.verify(&vc_id), revoked_vc_map)
}
