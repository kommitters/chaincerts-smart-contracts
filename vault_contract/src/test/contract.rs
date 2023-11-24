use crate::test::setup::VaultContractTest;
use soroban_sdk::{testutils::Address as _, vec, Address, String};

use super::setup::{get_vc_setup, VCVaultContractTest};

#[test]
fn test_initialize() {
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
#[should_panic(expected = "HostError: Error(Contract, #6)")]
fn test_authorize_issuer_with_not_registered_did() {
    let VaultContractTest {
        env,
        admin,
        did: _did,
        dids,
        issuer,
        contract,
    } = VaultContractTest::setup();
    contract.initialize(&admin, &dids);
    let invalid_did = String::from_slice(&env, "did:chaincerts:3mtjfbxad3wzh7qa4w5f7q4h");

    contract.authorize_issuer(&admin, &issuer, &invalid_did);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #5)")]
fn test_authorize_issuer_with_revoked_did() {
    let VaultContractTest {
        env: _,
        admin,
        did,
        dids,
        issuer,
        contract,
    } = VaultContractTest::setup();

    contract.initialize(&admin, &dids);
    contract.revoke_did(&admin, &did);
    contract.authorize_issuer(&admin, &issuer, &did);
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

#[test]
#[should_panic(expected = "HostError: Error(Contract, #6)")]
fn test_revoke_issuer_with_not_registered_did() {
    let VaultContractTest {
        env,
        admin,
        did: _did,
        dids,
        issuer,
        contract,
    } = VaultContractTest::setup();
    contract.initialize(&admin, &dids);
    let invalid_did = String::from_slice(&env, "did:chaincerts:3mtjfbxad3wzh7qa4w5f7q4h");

    contract.revoke_issuer(&admin, &issuer, &invalid_did);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #5)")]
fn test_revoke_issuer_with_revoked_did() {
    let VaultContractTest {
        env: _,
        admin,
        did,
        dids,
        issuer,
        contract,
    } = VaultContractTest::setup();

    contract.initialize(&admin, &dids);
    contract.revoke_did(&admin, &did);
    contract.revoke_issuer(&admin, &issuer, &did);
}

#[test]
fn test_store_vc() {
    let VaultContractTest {
        env,
        admin,
        did,
        dids,
        issuer,
        contract,
    } = VaultContractTest::setup();

    let VCVaultContractTest {
        vc_id,
        vc_data,
        issuance_contract_address,
    } = get_vc_setup(&env);

    contract.initialize(&admin, &dids);
    contract.authorize_issuer(&admin, &issuer, &did);
    contract.store_vc(&vc_id, &vc_data, &did, &issuer, &issuance_contract_address)
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #4)")]
fn test_store_vc_with_empty_issuers() {
    let VaultContractTest {
        env,
        admin,
        did,
        dids,
        issuer,
        contract,
    } = VaultContractTest::setup();

    let VCVaultContractTest {
        vc_id,
        vc_data,
        issuance_contract_address,
    } = get_vc_setup(&env);

    contract.initialize(&admin, &dids);
    contract.store_vc(&vc_id, &vc_data, &did, &issuer, &issuance_contract_address)
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #4)")]
fn test_store_vc_with_issuer_not_found() {
    let VaultContractTest {
        env,
        admin,
        did,
        dids,
        issuer,
        contract,
    } = VaultContractTest::setup();

    let invalid_issuer = Address::random(&env);

    let VCVaultContractTest {
        vc_id,
        vc_data,
        issuance_contract_address,
    } = get_vc_setup(&env);

    contract.initialize(&admin, &dids);
    contract.authorize_issuer(&admin, &issuer, &did);
    contract.store_vc(
        &vc_id,
        &vc_data,
        &did,
        &invalid_issuer,
        &issuance_contract_address,
    )
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #7)")]
fn test_store_vc_with_revoked_issuer() {
    let VaultContractTest {
        env,
        admin,
        did,
        dids,
        issuer,
        contract,
    } = VaultContractTest::setup();

    let VCVaultContractTest {
        vc_id,
        vc_data,
        issuance_contract_address,
    } = get_vc_setup(&env);

    contract.initialize(&admin, &dids);
    contract.authorize_issuer(&admin, &issuer, &did);
    contract.revoke_issuer(&admin, &issuer, &did);

    contract.store_vc(&vc_id, &vc_data, &did, &issuer, &issuance_contract_address)
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #6)")]
fn test_store_vc_with_did_not_found() {
    let VaultContractTest {
        env,
        admin,
        did,
        dids,
        issuer,
        contract,
    } = VaultContractTest::setup();
    let invalid_did = String::from_slice(&env, "did:chaincerts:3mtjfbxad3wzh7qa4w5f7q4h");

    let VCVaultContractTest {
        vc_id,
        vc_data,
        issuance_contract_address,
    } = get_vc_setup(&env);

    contract.initialize(&admin, &dids);
    contract.authorize_issuer(&admin, &issuer, &did);
    contract.store_vc(
        &vc_id,
        &vc_data,
        &invalid_did,
        &issuer,
        &issuance_contract_address,
    )
}

#[test]
fn test_get_vc() {
    let VaultContractTest {
        env,
        admin,
        did,
        dids,
        issuer,
        contract,
    } = VaultContractTest::setup();

    let VCVaultContractTest {
        vc_id,
        vc_data,
        issuance_contract_address,
    } = get_vc_setup(&env);

    contract.initialize(&admin, &dids);
    contract.authorize_issuer(&admin, &issuer, &did);
    contract.store_vc(&vc_id, &vc_data, &did, &issuer, &issuance_contract_address);
    let vc = contract.get_vc(&vc_id);

    assert_eq!(vc_id, vc.id);
    assert_eq!(vc_data, vc.data);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #8)")]
fn test_get_vc_not_found() {
    let VaultContractTest {
        env,
        admin,
        did,
        dids,
        issuer,
        contract,
    } = VaultContractTest::setup();

    let VCVaultContractTest {
        vc_id,
        vc_data,
        issuance_contract_address,
    } = get_vc_setup(&env);

    contract.initialize(&admin, &dids);
    contract.authorize_issuer(&admin, &issuer, &did);
    contract.store_vc(&vc_id, &vc_data, &did, &issuer, &issuance_contract_address);

    let vc_id2 = String::from_slice(&env, "vc_id2");
    contract.get_vc(&vc_id2);
}

#[test]
fn test_list_vcs() {
    let VaultContractTest {
        env,
        admin,
        did,
        dids: _dids,
        issuer,
        contract,
    } = VaultContractTest::setup();

    let VCVaultContractTest {
        vc_id,
        vc_data,
        issuance_contract_address,
    } = get_vc_setup(&env);

    let vc_id2 = String::from_slice(&env, "vc_id2");
    let vc_id3 = String::from_slice(&env, "vc_id3");
    let did2 = String::from_slice(&env, "did:chaincerts:3mtjfbxad3wzh7qa4w5f7q4h");
    let dids = vec![&env, did.clone(), did2.clone()];

    contract.initialize(&admin, &dids);
    contract.authorize_issuer(&admin, &issuer, &did);
    contract.authorize_issuer(&admin, &issuer, &did2);

    contract.store_vc(&vc_id, &vc_data, &did, &issuer, &issuance_contract_address);
    contract.store_vc(&vc_id2, &vc_data, &did, &issuer, &issuance_contract_address);
    contract.store_vc(
        &vc_id3,
        &vc_data,
        &did2,
        &issuer,
        &issuance_contract_address,
    );

    let dids_with_vcs = contract.list_vcs();
    let did_with_vc1 = dids_with_vcs.get_unchecked(did.clone());
    let did_with_vc2 = dids_with_vcs.get_unchecked(did2.clone());

    assert_eq!(dids_with_vcs.len(), 2);
    assert_eq!(did_with_vc1.did, did);
    assert_eq!(did_with_vc2.did, did2);
    assert_eq!(did_with_vc1.vcs.len(), 2);
    assert_eq!(did_with_vc2.vcs.len(), 1);
}

#[test]
fn test_register_did() {
    let VaultContractTest {
        env,
        admin,
        did: _,
        dids,
        issuer: _,
        contract,
    } = VaultContractTest::setup();
    let did2 = String::from_slice(&env, "did:chaincerts:3mtjfbxad3wzh7qa4w5f7q4h");

    contract.initialize(&admin, &dids);
    contract.register_did(&admin, &did2);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #9)")]
fn test_register_did_with_duplicated_did() {
    let VaultContractTest {
        env: _,
        admin,
        did: duplicated_did,
        dids,
        issuer: _,
        contract,
    } = VaultContractTest::setup();

    contract.initialize(&admin, &dids);
    contract.register_did(&admin, &duplicated_did);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #2)")]
fn test_register_did_with_invalid_admin() {
    let VaultContractTest {
        env,
        admin,
        did: _,
        dids,
        issuer: _,
        contract,
    } = VaultContractTest::setup();
    let did2 = String::from_slice(&env, "did:chaincerts:3mtjfbxad3wzh7qa4w5f7q4h");
    let invalid_admin = Address::random(&env);

    contract.initialize(&admin, &dids);
    contract.register_did(&invalid_admin, &did2);
}

#[test]
fn test_revoke_did() {
    let VaultContractTest {
        env: _,
        admin,
        did,
        dids,
        issuer: _,
        contract,
    } = VaultContractTest::setup();

    contract.initialize(&admin, &dids);
    contract.revoke_did(&admin, &did);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #2)")]
fn test_revoke_did_with_invalid_admin() {
    let VaultContractTest {
        env,
        admin,
        did,
        dids,
        issuer: _,
        contract,
    } = VaultContractTest::setup();
    let invalid_admin = Address::random(&env);

    contract.initialize(&admin, &dids);
    contract.revoke_did(&invalid_admin, &did);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #6)")]
fn test_revoke_did_with_no_registered_did() {
    let VaultContractTest {
        env,
        admin,
        did: _,
        dids,
        issuer: _,
        contract,
    } = VaultContractTest::setup();
    let invalid_did = String::from_slice(&env, "did:chaincerts:3mtjfbxad3wzh7qa4w5f7q4h");

    contract.initialize(&admin, &dids);
    contract.revoke_did(&admin, &invalid_did);
}
