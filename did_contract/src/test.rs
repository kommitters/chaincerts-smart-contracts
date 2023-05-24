#![cfg(test)]
use crate::{
    authentication::VerificationMethod,
    did_document::{DIDDocument, Metadata, Method, Service},
    option::{OptionMethodService, OptionU64},
    DIDContract, DIDContractClient,
};
use soroban_sdk::{testutils::Address as _, vec, Address, Env, IntoVal, String, Vec};

fn create_did_contract(
    e: &Env,
    id: &String,
    authentication_params: &(String, Address),
    context: &Vec<String>,
    verification_processes: &Vec<Method>,
    services: &Vec<Service>,
    metadata: &Metadata,
) -> DIDContractClient {
    let did_contract = DIDContractClient::new(e, &e.register_contract(None, DIDContract {}));
    did_contract.initialize(
        id,
        authentication_params,
        context,
        verification_processes,
        services,
        metadata,
    );
    did_contract
}

struct DIDContractTest {
    env: Env,
    id: String,
    authentication: String,
    authentication_address: Address,
    did_contract: DIDContractClient,
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
        let id = String::from_slice(&env, "did:chaincerts::ABC");
        let authentication = String::from_slice(&env, "did:chaincerts::ABC#key1");
        let authentication_address = Address::random(&env);
        let context = vec![
            &env,
            String::from_slice(&env, "https://www.w3.org/ns/did/v1"),
            String::from_slice(&env, "https://www.example.com/context/v1"),
        ];
        let method = Method {
            method_type: String::from_slice(&env, "otp"),
            verified: true,
            timestamp: 1684872059,
            service: OptionMethodService::None,
        };
        let verification_processes = vec![&env, method];
        let service = Service {
            service_type: String::from_slice(&env, "VerifiableCredential"),
            service_endpoint: String::from_slice(&env, "https://did.chaincerts.co/ABC123"),
        };
        let services = vec![&env, service];
        let metadata = Metadata {
            created: 1684872059,
            updated: 1684872059,
            version: String::from_slice(&env, "1.0"),
        };
        let did_contract = create_did_contract(
            &env,
            &id,
            &(authentication.clone(), authentication_address.clone()),
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
            id,
            authentication,
            authentication_address,
            did_contract,
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
fn test_successful_execution_of_did_contract_capabilities() {
    let test = DIDContractTest::setup();
    let new_credential_did = "did:chaincerts:".into_val(&test.env);

    test.did_contract.add_organization(
        &test.organizations.get_unchecked(0).unwrap(),
        &test.authentication_address,
    );
    test.did_contract.add_organization(
        &test.organizations.get_unchecked(1).unwrap(),
        &test.authentication_address,
    );

    assert_eq!(
        test.did_contract
            .get_access_control_list(&test.authentication_address)
            .len(),
        2
    );

    test.did_contract.deposit_credential(
        &test.credential_did,
        &test.organizations.get_unchecked(0).unwrap(),
        &1680105831,
        &OptionU64::Some(1711662757),
        &test.cids.get_unchecked(0).unwrap(),
    );

    test.did_contract.deposit_credential(
        &new_credential_did,
        &test.organizations.get_unchecked(0).unwrap(),
        &1680205831,
        &OptionU64::None,
        &test.cids.get_unchecked(0).unwrap(),
    );

    assert_eq!(test.did_contract.get_credentials().len(), 2);

    test.did_contract
        .revoke_credential(&test.credential_did, &test.authentication_address);

    test.did_contract.remove_organization(
        &test.organizations.get_unchecked(0).unwrap(),
        &test.authentication_address,
    );
    assert_eq!(
        test.did_contract
            .get_access_control_list(&test.authentication_address)
            .len(),
        1
    );
}

#[test]
fn test_retrieve_did_public_document() {
    let test = DIDContractTest::setup();

    let verifiable_method = VerificationMethod {
        id: test.authentication.clone(),
        verification_method_type: String::from_slice(&test.env, "Ed25519VerificationKey2020"),
        controller: test.id.clone(),
        blockchain_account_id: test.authentication_address,
    };

    let public_did_document = DIDDocument {
        context: test.context,
        id: test.id,
        verification_method: vec![&test.env, verifiable_method],
        authentication: vec![&test.env, test.authentication],
        services: test.services,
        metadata: test.metadata.clone(),
    };

    test.did_contract.public_did_document();
    assert_eq!(test.did_contract.public_did_document(), public_did_document)
}

#[test]
#[should_panic(expected = "Status(ContractError(1))")]
fn test_initialize_an_already_initialized_did_contract() {
    let test = DIDContractTest::setup();
    test.did_contract.initialize(
        &test.id,
        &(test.authentication, test.authentication_address),
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

    test.did_contract.get_access_control_list(&invalid_address);
}

#[test]
#[should_panic(expected = "Status(ContractError(4))")]
fn test_when_adding_an_already_added_org() {
    let test = DIDContractTest::setup();

    test.did_contract.add_organization(
        &test.organizations.get_unchecked(0).unwrap(),
        &test.authentication_address,
    );
    test.did_contract.add_organization(
        &test.organizations.get_unchecked(0).unwrap(),
        &test.authentication_address,
    );
}

#[test]
#[should_panic(expected = "Status(ContractError(6))")]
fn test_remove_organization_when_not_organizations_already_set() {
    let test = DIDContractTest::setup();
    test.did_contract.remove_organization(
        &test.organizations.get_unchecked(0).unwrap(),
        &test.authentication_address,
    );
}

#[test]
#[should_panic(expected = "Status(ContractError(8))")]
fn test_remove_organization_when_organization_not_found() {
    let test = DIDContractTest::setup();
    test.did_contract.add_organization(
        &test.organizations.get_unchecked(0).unwrap(),
        &test.authentication_address,
    );
    test.did_contract.remove_organization(
        &test.organizations.get_unchecked(1).unwrap(),
        &test.authentication_address,
    );
}

#[test]
#[should_panic(expected = "Status(ContractError(2))")]
fn test_deposit_credential_when_organization_is_not_in_the_acl() {
    let test = DIDContractTest::setup();

    test.did_contract.add_organization(
        &test.organizations.get_unchecked(0).unwrap(),
        &test.authentication_address,
    );

    test.did_contract.deposit_credential(
        &test.credential_did,
        &test.organizations.get_unchecked(1).unwrap(),
        &1680105831,
        &OptionU64::Some(1711662757),
        &test.cids.get(0).unwrap().unwrap(),
    );
}

#[test]
#[should_panic(expected = "Status(ContractError(6))")]
fn test_deposit_credential_when_no_organizations_in_the_acl() {
    let test = DIDContractTest::setup();

    test.did_contract.deposit_credential(
        &test.credential_did,
        &test.organizations.get_unchecked(1).unwrap(),
        &1680105831,
        &OptionU64::Some(1711662757),
        &test.cids.get(0).unwrap().unwrap(),
    );
}

#[test]
#[should_panic(expected = "Status(ContractError(9))")]
fn test_deposit_credential_chaincert_is_already_in_the_did_contract() {
    let test = DIDContractTest::setup();

    test.did_contract.add_organization(
        &test.organizations.get_unchecked(0).unwrap(),
        &test.authentication_address,
    );
    test.did_contract.add_organization(
        &test.organizations.get_unchecked(1).unwrap(),
        &test.authentication_address,
    );

    test.did_contract.deposit_credential(
        &test.credential_did,
        &test.organizations.get_unchecked(0).unwrap(),
        &1680105831,
        &OptionU64::Some(1711662757),
        &test.cids.get_unchecked(0).unwrap(),
    );

    test.did_contract.deposit_credential(
        &test.credential_did,
        &test.organizations.get_unchecked(0).unwrap(),
        &1680105831,
        &OptionU64::Some(1711662757),
        &test.cids.get_unchecked(0).unwrap(),
    );
}

#[test]
#[should_panic(expected = "Status(ContractError(11))")]
fn test_revoke_credential_when_no_chaincerts_in_did_contract() {
    let test = DIDContractTest::setup();

    test.did_contract.add_organization(
        &test.organizations.get_unchecked(0).unwrap(),
        &test.authentication_address,
    );
    test.did_contract
        .revoke_credential(&test.credential_did, &test.authentication_address)
}

#[test]
#[should_panic(expected = "Status(ContractError(10))")]
fn test_revoke_credential_when_chaincert_not_found() {
    let test = DIDContractTest::setup();
    let org1 = test.organizations.get_unchecked(0).unwrap();
    let new_chaincert: String = "CHAINCERT2".into_val(&test.env);

    test.did_contract
        .add_organization(&org1, &test.authentication_address);
    test.did_contract.deposit_credential(
        &test.credential_did,
        &org1,
        &1680105831,
        &OptionU64::Some(1711662757),
        &test.cids.get(0).unwrap().unwrap(),
    );

    test.did_contract
        .revoke_credential(&new_chaincert, &test.authentication_address);
}

#[test]
#[should_panic(expected = "Status(ContractError(11))")]
fn test_request_chaincerts_when_no_chaincerts_set() {
    let test = DIDContractTest::setup();

    test.did_contract.get_credentials();
}

#[test]
#[should_panic(expected = "Status(ContractError(6))")]
fn test_request_acl_when_no_organizations_set() {
    let test = DIDContractTest::setup();

    test.did_contract
        .get_access_control_list(&test.authentication_address);
}
