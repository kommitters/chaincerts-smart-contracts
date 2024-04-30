use crate::test::setup::{create_vc, get_revoked_vc_map, get_valid_vc_map, VCIssuanceContractTest};
use soroban_sdk::{testutils::Address as _, Address, String};

#[test]
fn test_initialize_with_amount() {
    let VCIssuanceContractTest {
        env: _env,
        admin,
        vc_id: _,
        vc_data: _,
        issuer_did,
        contract,
    } = VCIssuanceContractTest::setup();

    contract.initialize(&admin, &issuer_did);
}

#[test]
fn test_initialize_without_amount() {
    let VCIssuanceContractTest {
        env: _env,
        admin,
        vc_id: _,
        vc_data: _,
        issuer_did,
        contract,
    } = VCIssuanceContractTest::setup();

    contract.initialize(&admin, &issuer_did);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #1)")]
fn test_initialize_an_already_initialized_contract() {
    let VCIssuanceContractTest {
        env: _env,
        admin,
        vc_id: _,
        vc_data: _,
        issuer_did,
        contract,
    } = VCIssuanceContractTest::setup();

    contract.initialize(&admin, &issuer_did);
    contract.initialize(&admin, &issuer_did);
}

#[test]
fn test_issue() {
    let VCIssuanceContractTest {
        env,
        admin,
        vc_id,
        vc_data,
        issuer_did,
        contract,
    } = VCIssuanceContractTest::setup();

    let vault_contract_id = create_vc(&env, &admin, &contract, &issuer_did);
    contract.issue(&vc_id, &vc_data, &vault_contract_id);
}

#[test]
fn test_revoke_vc() {
    let VCIssuanceContractTest {
        env,
        admin,
        vc_id,
        vc_data,
        issuer_did,
        contract,
    } = VCIssuanceContractTest::setup();
    let vault_contract_id = create_vc(&env, &admin, &contract, &issuer_did);
    let vc_id = contract.issue(&vc_id, &vc_data, &vault_contract_id);

    let date = String::from_str(&env, "2023-12-05T21:37:44.389Z");

    contract.revoke(&vc_id, &date);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #3)")]
fn test_revoke_vc_with_invalid_vc() {
    let VCIssuanceContractTest {
        env,
        admin,
        vc_id: _,
        vc_data: _,
        issuer_did,
        contract,
    } = VCIssuanceContractTest::setup();
    contract.initialize(&admin, &issuer_did);

    let vc_id = String::from_str(&env, "vc_id1");
    let date = String::from_str(&env, "2023-12-05T21:37:44.389Z");

    contract.revoke(&vc_id, &date);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #4)")]
fn test_revoke_vc_when_it_was_already_revoked() {
    let VCIssuanceContractTest {
        env,
        admin,
        vc_id,
        vc_data,
        issuer_did,
        contract,
    } = VCIssuanceContractTest::setup();
    let vault_contract_id = create_vc(&env, &admin, &contract, &issuer_did);
    let vc_id = contract.issue(&vc_id, &vc_data, &vault_contract_id);

    let date_1 = String::from_str(&env, "2023-12-05T21:37:44.389Z");
    let date_2 = String::from_str(&env, "2023-21-05T21:37:44.389Z");

    contract.revoke(&vc_id, &date_1);
    contract.revoke(&vc_id, &date_2);
}

#[test]
fn test_verify_vc() {
    let VCIssuanceContractTest {
        env,
        admin,
        vc_id,
        vc_data,
        issuer_did,
        contract,
    } = VCIssuanceContractTest::setup();
    let vault_contract_id = create_vc(&env, &admin, &contract, &issuer_did);
    let vc_id = contract.issue(&vc_id, &vc_data, &vault_contract_id);

    let valid_vc_map = get_valid_vc_map(&env);
    assert_eq!(contract.verify(&vc_id), valid_vc_map)
}

#[test]
fn test_verify_vc_with_revoked_vc() {
    let VCIssuanceContractTest {
        env,
        admin,
        vc_id,
        vc_data,
        issuer_did,
        contract,
    } = VCIssuanceContractTest::setup();
    let vault_contract_id = create_vc(&env, &admin, &contract, &issuer_did);
    let vc_id = contract.issue(&vc_id, &vc_data, &vault_contract_id);
    let date = String::from_str(&env, "2023-12-05T21:37:44.389Z");

    contract.revoke(&vc_id, &date);

    let revoked_vc_map = get_revoked_vc_map(&env, date);
    assert_eq!(contract.verify(&vc_id), revoked_vc_map)
}

#[test]
fn test_set_admin() {
    let VCIssuanceContractTest {
        env,
        admin,
        vc_id: _,
        vc_data: _,
        issuer_did,
        contract,
    } = VCIssuanceContractTest::setup();

    contract.initialize(&admin, &issuer_did);

    let new_admin = Address::generate(&env);

    contract.set_admin(&new_admin);
}

#[test]
fn test_version() {
    let VCIssuanceContractTest {
        env,
        admin: _,
        vc_id: _,
        vc_data: _,
        issuer_did: _,
        contract,
    } = VCIssuanceContractTest::setup();

    let pkg_version = env!("CARGO_PKG_VERSION");
    let expected_version = String::from_str(&env, pkg_version);
    assert_eq!(contract.version(), expected_version)
}
