use crate::test::setup::VaultContractTest;
use soroban_sdk::{testutils::Address as _, vec, Address};

#[test]
fn test_inititialize() {
    let VaultContractTest {
        env: _env,
        admin,
        did: _did,
        dids,
        issuer: _issuer,
        contract,
    } = VaultContractTest::setup();
    contract.initialize(&admin, &dids);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #1)")]
fn test_initialize_an_already_initialized_contract() {
    let VaultContractTest {
        env: _env,
        admin,
        did: _did,
        dids,
        issuer: _issuer,
        contract,
    } = VaultContractTest::setup();

    contract.initialize(&admin, &dids);
    contract.initialize(&admin, &dids);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #3)")]
fn test_initialize_with_empty_dids() {
    let VaultContractTest {
        env,
        admin,
        dids: _dids,
        did: _did,
        issuer: _issuer,
        contract,
    } = VaultContractTest::setup();

    let empty_dids = vec![&env];
    contract.initialize(&admin, &empty_dids);
}

#[test]
fn test_authorize_issuer() {
    let VaultContractTest {
        env: _env,
        admin,
        did,
        dids,
        issuer,
        contract,
    } = VaultContractTest::setup();

    contract.initialize(&admin, &dids);
    contract.authorize_issuer(&admin, &issuer, &did);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #2)")]
fn test_authorize_issuer_with_invalid_admin() {
    let VaultContractTest {
        env,
        admin,
        did,
        dids,
        issuer,
        contract,
    } = VaultContractTest::setup();

    let invalid_admin = Address::random(&env);
    contract.initialize(&admin, &dids);

    contract.authorize_issuer(&invalid_admin, &issuer, &did);
}

#[test]
fn test_revoke_issuer() {
    let VaultContractTest {
        env: _env,
        admin,
        did,
        dids,
        issuer,
        contract,
    } = VaultContractTest::setup();

    contract.initialize(&admin, &dids);
    contract.authorize_issuer(&admin, &issuer, &did);
    contract.revoke_issuer(&admin, &issuer, &did);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #2)")]
fn test_revoke_issuer_with_invalid_admin() {
    let VaultContractTest {
        env,
        admin,
        did,
        dids,
        issuer,
        contract,
    } = VaultContractTest::setup();

    contract.initialize(&admin, &dids);
    contract.authorize_issuer(&admin, &issuer, &did);

    let invalid_admin = Address::random(&env);
    contract.revoke_issuer(&invalid_admin, &issuer, &did);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #4)")]
fn test_revoke_issuer_when_issuer_is_not_found() {
    let VaultContractTest {
        env,
        admin,
        did,
        dids,
        issuer,
        contract,
    } = VaultContractTest::setup();

    contract.initialize(&admin, &dids);
    contract.authorize_issuer(&admin, &issuer, &did);

    let invalid_issuer = Address::random(&env);
    contract.revoke_issuer(&admin, &invalid_issuer, &did);
}
