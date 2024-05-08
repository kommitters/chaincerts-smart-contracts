use super::setup::{did_context, get_vc_setup, VCVaultContractTest};
use crate::did_contract;
use crate::test::setup::VaultContractTest;
use soroban_sdk::{testutils::Address as _, vec, Address, String};

#[test]
fn test_initialize() {
    let VaultContractTest {
        env,
        admin,
        issuer: _issuer,
        did_init_args,
        did_wasm_hash,
        salt,
        contract,
    } = VaultContractTest::setup();

    let context = did_context(&env);

    let (contract_id, did_document_val) =
        contract.initialize(&admin, &did_wasm_hash, &did_init_args, &salt);

    let client = did_contract::Client::new(&env, &contract_id);
    let did_document = client.get_did();

    assert!(did_document_val.is_object());
    assert!(did_document.context == context);
    assert!(contract_id.to_string().len() > 0)
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #1)")]
fn test_initialize_an_already_initialized_contract() {
    let VaultContractTest {
        env: _,
        admin,
        issuer: _issuer,
        did_init_args,
        did_wasm_hash,
        salt,
        contract,
    } = VaultContractTest::setup();

    contract.initialize(&admin, &did_wasm_hash, &did_init_args, &salt);
    contract.initialize(&admin, &did_wasm_hash, &did_init_args, &salt);
}

#[test]
fn test_authorize_issuer() {
    let VaultContractTest {
        env: _env,
        admin,
        issuer,
        did_init_args,
        did_wasm_hash,
        salt,
        contract,
    } = VaultContractTest::setup();

    contract.initialize(&admin, &did_wasm_hash, &did_init_args, &salt);
    contract.authorize_issuer(&issuer);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #3)")]
fn test_authorize_issuer_with_already_authorized_issuer() {
    let VaultContractTest {
        env: _,
        admin,
        issuer,
        did_init_args,
        did_wasm_hash,
        salt,
        contract,
    } = VaultContractTest::setup();

    contract.initialize(&admin, &did_wasm_hash, &did_init_args, &salt);
    contract.authorize_issuer(&issuer);
    contract.authorize_issuer(&issuer);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #4)")]
fn test_authorize_issuer_with_revoked_vault() {
    let VaultContractTest {
        env: _,
        admin,
        issuer,
        did_init_args,
        did_wasm_hash,
        salt,
        contract,
    } = VaultContractTest::setup();

    contract.initialize(&admin, &did_wasm_hash, &did_init_args, &salt);
    contract.revoke_vault();
    contract.authorize_issuer(&issuer);
}

#[test]
fn test_authorize_issuers() {
    let VaultContractTest {
        env,
        admin,
        issuer,
        did_init_args,
        did_wasm_hash,
        salt,
        contract,
    } = VaultContractTest::setup();
    let issuers = vec![&env, issuer.clone()];

    contract.initialize(&admin, &did_wasm_hash, &did_init_args, &salt);
    contract.authorize_issuers(&issuers);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #4)")]
fn test_authorize_issuers_with_revoked_vault() {
    let VaultContractTest {
        env,
        admin,
        issuer,
        did_init_args,
        did_wasm_hash,
        salt,
        contract,
    } = VaultContractTest::setup();
    let issuers = vec![&env, issuer.clone()];

    contract.initialize(&admin, &did_wasm_hash, &did_init_args, &salt);
    contract.revoke_vault();
    contract.authorize_issuers(&issuers);
}

#[test]
fn test_revoke_issuer() {
    let VaultContractTest {
        env: _env,
        admin,
        issuer,
        did_init_args,
        did_wasm_hash,
        salt,
        contract,
    } = VaultContractTest::setup();

    contract.initialize(&admin, &did_wasm_hash, &did_init_args, &salt);
    contract.authorize_issuer(&issuer);
    contract.revoke_issuer(&issuer);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #2)")]
fn test_revoke_issuer_when_issuer_is_not_found() {
    let VaultContractTest {
        env,
        admin,
        issuer,
        did_init_args,
        did_wasm_hash,
        salt,
        contract,
    } = VaultContractTest::setup();

    contract.initialize(&admin, &did_wasm_hash, &did_init_args, &salt);
    contract.authorize_issuer(&issuer);

    let invalid_issuer = Address::generate(&env);
    contract.revoke_issuer(&invalid_issuer);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #4)")]
fn test_revoke_issuer_with_revoked_vault() {
    let VaultContractTest {
        env: _,
        admin,
        issuer,
        did_init_args,
        did_wasm_hash,
        salt,
        contract,
    } = VaultContractTest::setup();

    contract.initialize(&admin, &did_wasm_hash, &did_init_args, &salt);
    contract.revoke_vault();
    contract.revoke_issuer(&issuer);
}

#[test]
fn test_store_vc() {
    let VaultContractTest {
        env,
        admin,
        issuer,
        did_init_args,
        did_wasm_hash,
        salt,
        contract,
    } = VaultContractTest::setup();

    let VCVaultContractTest {
        vc_id,
        vc_data,
        issuance_contract_address,
        issuer_did,
    } = get_vc_setup(&env);

    contract.initialize(&admin, &did_wasm_hash, &did_init_args, &salt);
    contract.authorize_issuer(&issuer);
    contract.store_vc(
        &vc_id,
        &vc_data,
        &issuer,
        &issuer_did,
        &issuance_contract_address,
    )
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #2)")]
fn test_store_vc_with_empty_issuers() {
    let VaultContractTest {
        env,
        admin,
        issuer,
        did_init_args,
        did_wasm_hash,
        salt,
        contract,
    } = VaultContractTest::setup();

    let VCVaultContractTest {
        vc_id,
        vc_data,
        issuance_contract_address,
        issuer_did,
    } = get_vc_setup(&env);

    contract.initialize(&admin, &did_wasm_hash, &did_init_args, &salt);
    contract.store_vc(
        &vc_id,
        &vc_data,
        &issuer,
        &issuer_did,
        &issuance_contract_address,
    )
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #2)")]
fn test_store_vc_with_issuer_not_found() {
    let VaultContractTest {
        env,
        admin,
        issuer,
        did_init_args,
        did_wasm_hash,
        salt,
        contract,
    } = VaultContractTest::setup();

    let invalid_issuer = Address::generate(&env);

    let VCVaultContractTest {
        vc_id,
        vc_data,
        issuance_contract_address,
        issuer_did,
    } = get_vc_setup(&env);

    contract.initialize(&admin, &did_wasm_hash, &did_init_args, &salt);
    contract.authorize_issuer(&issuer);
    contract.store_vc(
        &vc_id,
        &vc_data,
        &invalid_issuer,
        &issuer_did,
        &issuance_contract_address,
    )
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #2)")]
fn test_store_vc_with_revoked_issuer() {
    let VaultContractTest {
        env,
        admin,
        issuer,
        did_init_args,
        did_wasm_hash,
        salt,
        contract,
    } = VaultContractTest::setup();

    let VCVaultContractTest {
        vc_id,
        vc_data,
        issuance_contract_address,
        issuer_did,
    } = get_vc_setup(&env);

    contract.initialize(&admin, &did_wasm_hash, &did_init_args, &salt);
    contract.authorize_issuer(&issuer);
    contract.revoke_issuer(&issuer);

    contract.store_vc(
        &vc_id,
        &vc_data,
        &issuer,
        &issuer_did,
        &issuance_contract_address,
    )
}

#[test]
fn test_revoke_vault() {
    let VaultContractTest {
        env: _,
        admin,
        issuer: _,
        did_init_args,
        did_wasm_hash,
        salt,
        contract,
    } = VaultContractTest::setup();

    contract.initialize(&admin, &did_wasm_hash, &did_init_args, &salt);
    contract.revoke_vault();
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #5)")]
fn test_migrate_should_fail_without_vcs() {
    let VaultContractTest {
        env: _,
        admin,
        issuer: _,
        did_init_args,
        did_wasm_hash,
        salt,
        contract,
    } = VaultContractTest::setup();

    contract.initialize(&admin, &did_wasm_hash, &did_init_args, &salt);
    contract.migrate();
}

#[test]
fn test_set_admin() {
    let VaultContractTest {
        env,
        admin,
        issuer: _issuer,
        did_init_args,
        did_wasm_hash,
        salt,
        contract,
    } = VaultContractTest::setup();

    contract.initialize(&admin, &did_wasm_hash, &did_init_args, &salt);

    let new_admin = Address::generate(&env);

    contract.set_admin(&new_admin);
}

#[test]
fn test_version() {
    let VaultContractTest {
        env,
        admin,
        issuer: _issuer,
        did_init_args,
        did_wasm_hash,
        salt,
        contract,
    } = VaultContractTest::setup();

    contract.initialize(&admin, &did_wasm_hash, &did_init_args, &salt);
    let pkg_version = env!("CARGO_PKG_VERSION");
    let expected_version = String::from_str(&env, pkg_version);
    assert_eq!(contract.version(), expected_version)
}
