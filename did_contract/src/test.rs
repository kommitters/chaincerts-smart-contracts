#![cfg(test)]
use crate::{
    did_document::{Metadata, Method, Service},
    option::OptionU64,
    owner::{Owner, VerificationMethod},
    DIDContract, DIDContractClient,
};
use soroban_sdk::{testutils::Address as _, vec, Address, Env, IntoVal, String, Vec};

fn create_wallet(
    e: &Env,
    owner: &Owner,
    context: &Vec<String>,
    verification_processes: &Vec<Method>,
    services: &Vec<Service>,
    metadata: &Metadata,
) -> DIDContractClient {
    let wallet = DIDContractClient::new(e, &e.register_contract(None, DIDContract {}));
    wallet.initialize(owner, context, verification_processes, services, metadata);
    wallet
}

struct DIDContractTest {
    env: Env,
    owner: Owner,
    owner_address: Address,
    wallet: DIDContractClient,
    credential_did: String,
    organizations: Vec<String>,
    cids: Vec<String>,
    context: Vec<String>,
    verification_processes: Vec<Method>,
    services: Vec<Service>,
    metadata: Metadata,
}

impl DIDContractTest {
    fn setup() -> Self {
        let env: Env = Default::default();
        let authentications = vec![&env, String::from_slice(&env, "did:chaincerts::ABC#key1")];
        let owner_address = Address::random(&env);
        let verification_method = VerificationMethod {
            id: String::from_slice(&env, "did:chaincerts:ABC123#key-1"),
            verification_method_type: String::from_slice(&env, "Ed25519VerificationKey2020"),
            controller: String::from_slice(&env, "did:chaincerts:ABC123"),
            blockchain_account_id: owner_address.clone(),
        };
        let verification_methods = vec![&env, verification_method];
        let owner = Owner {
            authentications,
            verification_methods,
        };
        let context = vec![
            &env,
            String::from_slice(&env, "https://www.w3.org/ns/did/v1"),
            String::from_slice(&env, "https://www.example.com/context/v1"),
        ];
        let method = Method {
            method_type: String::from_slice(&env, "otp"),
            verified: true,
            timestamp: 1684872059,
        };
        let verification_processes = vec![&env, method];
        let service = Service {
            service_type: String::from_slice(&env, "VerifiableCredentials"),
            service_endpoint: String::from_slice(&env, "https://did.chaincerts.co/ABC123"),
        };
        let services = vec![&env, service];
        let metadata = Metadata {
            created: 1684872059,
            updated: 1684872059,
            version: String::from_slice(&env, "1.0"),
        };
        let wallet = create_wallet(
            &env,
            &owner,
            &context,
            &verification_processes,
            &services,
            &metadata,
        );
        let credential_did: String = "did:chaincerts:abc123#credential-xyz123".into_val(&env);
        let issuer_id1: String = "did:chaincerts:ISSUER1".into_val(&env);
        let issuer_id2: String = "did:chaincerts:ISSUER2".into_val(&env);
        let organizations = vec![&env, issuer_id1, issuer_id2];
        let cid1: String = "QmdtyfTYbVS3K9iYqBPjXxn4mbB7aBvEjYGzYWnzRcMrEC".into_val(&env);
        let cids = vec![&env, cid1];

        DIDContractTest {
            env,
            owner,
            owner_address,
            wallet,
            credential_did,
            organizations,
            cids,
            context,
            verification_processes,
            services,
            metadata,
        }
    }
}

#[test]
fn test_successful_execution_of_wallet_capabilities() {
    let test = DIDContractTest::setup();
    let new_credential_did = "did:chaincerts:".into_val(&test.env);

    test.wallet.add_organization(
        &test.organizations.get_unchecked(0).unwrap(),
        &test.owner_address,
    );
    test.wallet.add_organization(
        &test.organizations.get_unchecked(1).unwrap(),
        &test.owner_address,
    );

    assert_eq!(
        test.wallet
            .get_access_control_list(&test.owner_address)
            .len(),
        2
    );

    test.wallet.deposit_chaincert(
        &test.credential_did,
        &test.organizations.get_unchecked(0).unwrap(),
        &1680105831,
        &OptionU64::Some(1711662757),
        &test.cids.get_unchecked(0).unwrap(),
    );

    test.wallet.deposit_chaincert(
        &new_credential_did,
        &test.organizations.get_unchecked(0).unwrap(),
        &1680205831,
        &OptionU64::None,
        &test.cids.get_unchecked(0).unwrap(),
    );

    assert_eq!(test.wallet.get_credentials().len(), 2);

    test.wallet
        .revoke_credential(&test.credential_did, &test.owner_address);

    test.wallet.remove_organization(
        &test.organizations.get_unchecked(0).unwrap(),
        &test.owner_address,
    );
    assert_eq!(
        test.wallet
            .get_access_control_list(&test.owner_address)
            .len(),
        1
    );
}

#[test]
#[should_panic(expected = "Status(ContractError(1))")]
fn test_initialize_an_already_initialized_wallet() {
    let test = DIDContractTest::setup();
    test.wallet.initialize(
        &test.owner,
        &test.context,
        &test.verification_processes,
        &test.services,
        &test.metadata,
    );
}

#[test]
#[should_panic(expected = "Status(ContractError(2))")]
fn test_when_invalid_address() {
    let test = DIDContractTest::setup();
    let invalid_address = Address::random(&test.env);

    test.wallet.add_organization(
        &test.organizations.get_unchecked(0).unwrap(),
        &invalid_address,
    );
}

#[test]
#[should_panic(expected = "Status(ContractError(4))")]
fn test_when_adding_an_already_added_org() {
    let test = DIDContractTest::setup();

    test.wallet.add_organization(
        &test.organizations.get_unchecked(0).unwrap(),
        &test.owner_address,
    );
    test.wallet.add_organization(
        &test.organizations.get_unchecked(0).unwrap(),
        &test.owner_address,
    );
}

#[test]
#[should_panic(expected = "Status(ContractError(6))")]
fn test_remove_organization_when_not_organizations_already_set() {
    let test = DIDContractTest::setup();
    test.wallet.remove_organization(
        &test.organizations.get_unchecked(0).unwrap(),
        &test.owner_address,
    );
}

#[test]
#[should_panic(expected = "Status(ContractError(8))")]
fn test_remove_organization_when_organization_not_found() {
    let test = DIDContractTest::setup();
    test.wallet.add_organization(
        &test.organizations.get_unchecked(0).unwrap(),
        &test.owner_address,
    );
    test.wallet.remove_organization(
        &test.organizations.get_unchecked(1).unwrap(),
        &test.owner_address,
    );
}

#[test]
#[should_panic(expected = "Status(ContractError(2))")]
fn test_deposit_chaincert_when_organization_is_not_in_the_acl() {
    let test = DIDContractTest::setup();

    test.wallet.add_organization(
        &test.organizations.get_unchecked(0).unwrap(),
        &test.owner_address,
    );

    test.wallet.deposit_chaincert(
        &test.credential_did,
        &test.organizations.get_unchecked(1).unwrap(),
        &1680105831,
        &OptionU64::Some(1711662757),
        &test.cids.get(0).unwrap().unwrap(),
    );
}

#[test]
#[should_panic(expected = "Status(ContractError(6))")]
fn test_deposit_chaincert_when_no_organizations_in_the_acl() {
    let test = DIDContractTest::setup();

    test.wallet.deposit_chaincert(
        &test.credential_did,
        &test.organizations.get_unchecked(1).unwrap(),
        &1680105831,
        &OptionU64::Some(1711662757),
        &test.cids.get(0).unwrap().unwrap(),
    );
}

#[test]
#[should_panic(expected = "Status(ContractError(9))")]
fn test_deposit_chaincert_chaincert_is_already_in_the_wallet() {
    let test = DIDContractTest::setup();

    test.wallet.add_organization(
        &test.organizations.get_unchecked(0).unwrap(),
        &test.owner_address,
    );
    test.wallet.add_organization(
        &test.organizations.get_unchecked(1).unwrap(),
        &test.owner_address,
    );

    test.wallet.deposit_chaincert(
        &test.credential_did,
        &test.organizations.get_unchecked(0).unwrap(),
        &1680105831,
        &OptionU64::Some(1711662757),
        &test.cids.get_unchecked(0).unwrap(),
    );

    test.wallet.deposit_chaincert(
        &test.credential_did,
        &test.organizations.get_unchecked(0).unwrap(),
        &1680105831,
        &OptionU64::Some(1711662757),
        &test.cids.get_unchecked(0).unwrap(),
    );
}

#[test]
#[should_panic(expected = "Status(ContractError(11))")]
fn test_revoke_credential_when_no_chaincerts_in_wallet() {
    let test = DIDContractTest::setup();

    test.wallet.add_organization(
        &test.organizations.get_unchecked(0).unwrap(),
        &test.owner_address,
    );
    test.wallet
        .revoke_credential(&test.credential_did, &test.owner_address)
}

#[test]
#[should_panic(expected = "Status(ContractError(10))")]
fn test_revoke_credential_when_chaincert_not_found() {
    let test = DIDContractTest::setup();
    let org1 = test.organizations.get_unchecked(0).unwrap();
    let new_chaincert: String = "CHAINCERT2".into_val(&test.env);

    test.wallet.add_organization(&org1, &test.owner_address);
    test.wallet.deposit_chaincert(
        &test.credential_did,
        &org1,
        &1680105831,
        &OptionU64::Some(1711662757),
        &test.cids.get(0).unwrap().unwrap(),
    );

    test.wallet
        .revoke_credential(&new_chaincert, &test.owner_address);
}

#[test]
#[should_panic(expected = "Status(ContractError(11))")]
fn test_request_chaincerts_when_no_chaincerts_set() {
    let test = DIDContractTest::setup();

    test.wallet.get_credentials();
}

#[test]
#[should_panic(expected = "Status(ContractError(6))")]
fn test_request_acl_when_no_organizations_set() {
    let test = DIDContractTest::setup();

    test.wallet.get_access_control_list(&test.owner_address);
}
