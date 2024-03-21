use super::setup::{did_context, get_vc_setup, VCVaultContractTest};
use crate::did_contract;
use crate::test::setup::VaultContractTest;
use crate::verifiable_credential::VerifiableCredential;
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
    contract.authorize_issuer(&admin, &issuer);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #2)")]
fn test_authorize_issuer_with_invalid_admin() {
    let VaultContractTest {
        env,
        admin,
        issuer,
        did_init_args,
        did_wasm_hash,
        salt,
        contract,
    } = VaultContractTest::setup();

    let invalid_admin = Address::generate(&env);
    contract.initialize(&admin, &did_wasm_hash, &did_init_args, &salt);

    contract.authorize_issuer(&invalid_admin, &issuer);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #4)")]
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
    contract.authorize_issuer(&admin, &issuer);
    contract.authorize_issuer(&admin, &issuer);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #5)")]
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
    contract.revoke_vault(&admin);
    contract.authorize_issuer(&admin, &issuer);
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
    contract.authorize_issuers(&admin, &issuers);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #2)")]
fn test_authorize_issuers_with_invalid_admin() {
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
    let invalid_admin = Address::generate(&env);

    contract.initialize(&admin, &did_wasm_hash, &did_init_args, &salt);
    contract.authorize_issuers(&invalid_admin, &issuers);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #5)")]
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
    contract.revoke_vault(&admin);
    contract.authorize_issuers(&admin, &issuers);
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
    contract.authorize_issuer(&admin, &issuer);
    contract.revoke_issuer(&admin, &issuer);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #2)")]
fn test_revoke_issuer_with_invalid_admin() {
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
    contract.authorize_issuer(&admin, &issuer);

    let invalid_admin = Address::generate(&env);
    contract.revoke_issuer(&invalid_admin, &issuer);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #3)")]
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
    contract.authorize_issuer(&admin, &issuer);

    let invalid_issuer = Address::generate(&env);
    contract.revoke_issuer(&admin, &invalid_issuer);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #5)")]
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
    contract.revoke_vault(&admin);
    contract.revoke_issuer(&admin, &issuer);
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
    contract.authorize_issuer(&admin, &issuer);
    contract.store_vc(
        &vc_id,
        &vc_data,
        &issuer,
        &issuer_did,
        &issuance_contract_address,
    )
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #3)")]
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
#[should_panic(expected = "HostError: Error(Contract, #3)")]
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
    contract.authorize_issuer(&admin, &issuer);
    contract.store_vc(
        &vc_id,
        &vc_data,
        &invalid_issuer,
        &issuer_did,
        &issuance_contract_address,
    )
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #3)")]
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
    contract.authorize_issuer(&admin, &issuer);
    contract.revoke_issuer(&admin, &issuer);

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
    contract.revoke_vault(&admin);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #2)")]
fn test_revoke_vault_with_invalid_admin() {
    let VaultContractTest {
        env,
        admin,
        issuer: _,
        did_init_args,
        did_wasm_hash,
        salt,
        contract,
    } = VaultContractTest::setup();
    let invalid_admin = Address::generate(&env);

    contract.initialize(&admin, &did_wasm_hash, &did_init_args, &salt);
    contract.revoke_vault(&invalid_admin);
}

#[test]
fn test_get_vcs() {
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

    let vc_id_2 = String::from_str(&env, "vc_id2");

    let vc_1 = VerifiableCredential {
        id: vc_id.clone(),
        data: vc_data.clone(),
        issuance_contract: issuance_contract_address.clone(),
        issuer_did: issuer_did.clone(),
    };

    let vc_2 = VerifiableCredential {
        id: vc_id_2.clone(),
        data: vc_data.clone(),
        issuance_contract: issuance_contract_address.clone(),
        issuer_did: issuer_did.clone(),
    };

    contract.initialize(&admin, &did_wasm_hash, &did_init_args, &salt);
    contract.authorize_issuer(&admin, &issuer);
    contract.store_vc(
        &vc_id,
        &vc_data,
        &issuer,
        &issuer_did,
        &issuance_contract_address,
    );
    contract.store_vc(
        &vc_id_2,
        &vc_data,
        &issuer,
        &issuer_did,
        &issuance_contract_address,
    );
    let vcs = contract.get_vcs();

    assert_eq!(vcs.len(), 2);
    assert_eq!(vcs.get_unchecked(1), vc_1);
    assert_eq!(vcs.get_unchecked(0), vc_2);
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
