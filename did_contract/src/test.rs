#![cfg(test)]
use crate::{
    authentication::VerificationMethod,
    capability_invocation::{CapType, CapabilityInvocation},
    did_document::{DIDDocument, Metadata, Method, Service},
    option::{OptionAddress, OptionMethodService, OptionString, OptionU64},
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
    public_credential_did: String,
    capability_invocations: Vec<CapabilityInvocation>,
    cids: Vec<String>,
    context: Vec<String>,
    verification_processes: Vec<Method>,
    services: Vec<Service>,
    metadata: Metadata,
    shared_address: Address,
}

impl DIDContractTest {
    fn setup() -> Self {
        let env: Env = Default::default();
        let id = String::from_slice(&env, "did:chaincerts::ABC123");
        let authentication = String::from_slice(&env, "did:chaincerts:ABC123#key1");
        let authentication_address = Address::random(&env);
        let shared_address = Address::random(&env);
        let context = vec![
            &env,
            String::from_slice(&env, "https://www.w3.org/ns/did/v1"),
            String::from_slice(&env, "https://www.example.com/context/v1"),
        ];
        let method = Method {
            type_: String::from_slice(&env, "otp"),
            verified: true,
            timestamp: 1684872059,
            service: OptionMethodService::None,
        };
        let verification_processes = vec![&env, method];
        let service = Service {
            type_: String::from_slice(&env, "VerifiableCredential"),
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
        let credential_did: String = "did:chaincerts:ABC123#credential-xyz123".into_val(&env);
        let public_credential_did: String =
            "did:chaincerts:ABC123#credential-dfg123".into_val(&env);
        let issuer_id1: String = "did:chaincerts:ISSUER1".into_val(&env);
        let issuer_id2: String = "did:chaincerts:ISSUER2".into_val(&env);
        let cap1 = CapabilityInvocation {
            id: String::from_slice(&env, "did:chaincerts:ABC123#capability-1"),
            type_: CapType::AddCredential,
            invoker: OptionString::Some(issuer_id1.clone()),
            invoker_address: OptionAddress::Some(Address::random(&env)),
            credential: OptionString::None,
        };
        let cap2 = CapabilityInvocation {
            id: String::from_slice(&env, "did:chaincerts:ABC123#capability-2"),
            type_: CapType::AddCredential,
            invoker: OptionString::Some(issuer_id2),
            invoker_address: OptionAddress::Some(Address::random(&env)),
            credential: OptionString::None,
        };
        let cap3_read = CapabilityInvocation {
            id: String::from_slice(&env, "did:chaincerts:ABC123#capability-3"),
            type_: CapType::ReadCredential,
            invoker: OptionString::Some(issuer_id1),
            invoker_address: OptionAddress::Some(shared_address.clone()),
            credential: OptionString::Some(credential_did.clone()),
        };
        let cap4_public_read = CapabilityInvocation {
            id: String::from_slice(&env, "did:chaincerts:ABC123#capability-4"),
            type_: CapType::PublicRead,
            invoker: OptionString::None,
            invoker_address: OptionAddress::None,
            credential: OptionString::Some(public_credential_did.clone()),
        };
        let capability_invocations = vec![&env, cap1, cap2, cap3_read, cap4_public_read];
        let cid1: String = "QmdtyfTYbVS3K9iYqBPjXxn4mbB7aBvEjYGzYWnzRcMrEC".into_val(&env);
        let cids = vec![&env, cid1];

        DIDContractTest {
            env,
            id,
            authentication,
            authentication_address,
            did_contract,
            credential_did,
            public_credential_did,
            capability_invocations,
            cids,
            context,
            verification_processes,
            services,
            metadata,
            shared_address,
        }
    }
}

#[test]
fn test_successful_execution_of_did_contract_capabilities() {
    let test = DIDContractTest::setup();
    let new_credential_did = "did:chaincerts:".into_val(&test.env);
    let issuer = String::from_slice(&test.env, "did:chaincerts:ISSUER1");

    test.did_contract.add_capability(
        &test.authentication_address,
        &test.capability_invocations.get_unchecked(0).unwrap(),
    );
    test.did_contract.add_capability(
        &test.authentication_address,
        &test.capability_invocations.get_unchecked(1).unwrap(),
    );

    assert_eq!(
        test.did_contract
            .get_capability_invocation(&test.authentication_address)
            .len(),
        2
    );

    test.did_contract.deposit_credential(
        &test.credential_did,
        &issuer,
        &1680105831,
        &OptionU64::Some(1711662757),
        &test.cids.get_unchecked(0).unwrap(),
    );

    test.did_contract.deposit_credential(
        &new_credential_did,
        &issuer,
        &1680205831,
        &OptionU64::None,
        &test.cids.get_unchecked(0).unwrap(),
    );

    assert_eq!(
        test.did_contract
            .get_credentials(&test.authentication_address)
            .len(),
        2
    );

    test.did_contract
        .revoke_credential(&test.authentication_address, &test.credential_did);

    test.did_contract.remove_capability(
        &test.authentication_address,
        &test.capability_invocations.get_unchecked(0).unwrap().id,
    );
    assert_eq!(
        test.did_contract
            .get_capability_invocation(&test.authentication_address)
            .len(),
        1
    );

    let key_id = String::from_slice(&test.env, "did:chaincerts::ABC#key2");
    test.did_contract.add_authentication(
        &test.authentication_address,
        &key_id,
        &Address::random(&test.env),
    );
    assert_eq!(
        test.did_contract.public_did_document().authentication.len(),
        2
    );

    test.did_contract
        .remove_authentication(&test.authentication_address, &key_id);
    assert_eq!(
        test.did_contract.public_did_document().authentication.len(),
        1
    );

    test.did_contract.add_verification_method(
        &test.authentication_address,
        &key_id,
        &Address::random(&test.env),
    );
    assert_eq!(
        test.did_contract
            .public_did_document()
            .verification_method
            .len(),
        3
    );

    test.did_contract
        .remove_verification_method(&test.authentication_address, &key_id);
    assert_eq!(
        test.did_contract
            .public_did_document()
            .verification_method
            .len(),
        2
    );
}

#[test]
fn test_public_and_shared_credential_capability() {
    let test = DIDContractTest::setup();
    let invoker = String::from_slice(&test.env, "did:chaincerts:ISSUER1");

    test.did_contract.add_capability(
        &test.authentication_address,
        &test.capability_invocations.get_unchecked(0).unwrap(),
    );
    test.did_contract.add_capability(
        &test.authentication_address,
        &test.capability_invocations.get_unchecked(1).unwrap(),
    );
    test.did_contract.add_capability(
        &test.authentication_address,
        &test.capability_invocations.get_unchecked(2).unwrap(),
    );
    test.did_contract.add_capability(
        &test.authentication_address,
        &test.capability_invocations.get_unchecked(3).unwrap(),
    );

    test.did_contract.deposit_credential(
        &test.credential_did,
        &invoker,
        &1680105831,
        &OptionU64::Some(1711662757),
        &test.cids.get_unchecked(0).unwrap(),
    );

    test.did_contract.deposit_credential(
        &test.public_credential_did,
        &invoker,
        &1680205831,
        &OptionU64::None,
        &test.cids.get_unchecked(0).unwrap(),
    );

    let public_credentials = test.did_contract.get_public_credentials();
    let shared_credentials = test
        .did_contract
        .get_shared_credentials(&test.shared_address, &invoker);

    assert_eq!(public_credentials.len(), 1);
    assert_eq!(shared_credentials.len(), 1);
}

#[test]
fn test_retrieve_did_public_document() {
    let test = DIDContractTest::setup();

    let verifiable_method = VerificationMethod {
        id: test.authentication.clone(),
        type_: String::from_slice(&test.env, "Ed25519VerificationKey2020"),
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
fn test_remove_verification_method_when_remove_verification_with_auth() {
    let test = DIDContractTest::setup();

    let key_id = String::from_slice(&test.env, "did:chaincerts::ABC#key2");

    test.did_contract.add_authentication(
        &test.authentication_address,
        &key_id,
        &Address::random(&test.env),
    );

    test.did_contract
        .remove_verification_method(&test.authentication_address, &key_id)
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

    test.did_contract
        .get_capability_invocation(&invalid_address);
}

#[test]
#[should_panic(expected = "Status(ContractError(4))")]
fn test_when_adding_an_already_added_org() {
    let test = DIDContractTest::setup();

    test.did_contract.add_capability(
        &test.authentication_address,
        &test.capability_invocations.get_unchecked(0).unwrap(),
    );
    test.did_contract.add_capability(
        &test.authentication_address,
        &test.capability_invocations.get_unchecked(0).unwrap(),
    );
}

#[test]
#[should_panic(expected = "Status(ContractError(6))")]
fn test_remove_capability_when_no_capability_invocations_already_set() {
    let test = DIDContractTest::setup();
    test.did_contract.remove_capability(
        &test.authentication_address,
        &test.capability_invocations.get_unchecked(0).unwrap().id,
    );
}

#[test]
#[should_panic(expected = "Status(ContractError(8))")]
fn test_remove_capability_when_organization_not_found() {
    let test = DIDContractTest::setup();
    test.did_contract.add_capability(
        &test.authentication_address,
        &test.capability_invocations.get_unchecked(0).unwrap(),
    );
    test.did_contract.remove_capability(
        &test.authentication_address,
        &test.capability_invocations.get_unchecked(1).unwrap().id,
    );
}

#[test]
#[should_panic(expected = "Status(ContractError(2))")]
fn test_deposit_credential_when_not_share_cap_set() {
    let test = DIDContractTest::setup();

    test.did_contract.add_capability(
        &test.authentication_address,
        &test.capability_invocations.get_unchecked(0).unwrap(),
    );

    test.did_contract.deposit_credential(
        &test.credential_did,
        &test.capability_invocations.get_unchecked(1).unwrap().id,
        &1680105831,
        &OptionU64::Some(1711662757),
        &test.cids.get(0).unwrap().unwrap(),
    );
}

#[test]
#[should_panic(expected = "Status(ContractError(2))")]
fn test_deposit_credential_when_no_shared_cap_set() {
    let test = DIDContractTest::setup();
    let issuer = String::from_slice(&test.env, "did:chaincerts:ISSUER1");

    test.did_contract.add_capability(
        &test.authentication_address,
        &test.capability_invocations.get_unchecked(0).unwrap(),
    );

    test.did_contract
        .get_shared_credentials(&Address::random(&test.env), &issuer);
}

#[test]
#[should_panic(expected = "Status(ContractError(9))")]
fn test_deposit_credential_already_in_the_did_contract() {
    let test = DIDContractTest::setup();
    let issuer = String::from_slice(&test.env, "did:chaincerts:ISSUER1");

    test.did_contract.add_capability(
        &test.authentication_address,
        &test.capability_invocations.get_unchecked(0).unwrap(),
    );
    test.did_contract.add_capability(
        &test.authentication_address,
        &test.capability_invocations.get_unchecked(1).unwrap(),
    );

    test.did_contract.deposit_credential(
        &test.credential_did,
        &issuer,
        &1680105831,
        &OptionU64::Some(1711662757),
        &test.cids.get_unchecked(0).unwrap(),
    );

    test.did_contract.deposit_credential(
        &test.credential_did,
        &issuer,
        &1680105831,
        &OptionU64::Some(1711662757),
        &test.cids.get_unchecked(0).unwrap(),
    );
}

#[test]
#[should_panic(expected = "Status(ContractError(11))")]
fn test_revoke_credential_when_no_chaincerts_in_did_contract() {
    let test = DIDContractTest::setup();

    test.did_contract.add_capability(
        &test.authentication_address,
        &test.capability_invocations.get_unchecked(0).unwrap(),
    );
    test.did_contract
        .revoke_credential(&test.authentication_address, &test.credential_did)
}

#[test]
#[should_panic(expected = "Status(ContractError(10))")]
fn test_revoke_credential_when_chaincert_not_found() {
    let test = DIDContractTest::setup();
    let issuer_org1 = test.capability_invocations.get_unchecked(0).unwrap();
    let issuer = String::from_slice(&test.env, "did:chaincerts:ISSUER1");
    let new_chaincert: String = "did:chaincerts:abc123#credential-invalid".into_val(&test.env);

    test.did_contract
        .add_capability(&test.authentication_address, &issuer_org1);
    test.did_contract.deposit_credential(
        &test.credential_did,
        &issuer,
        &1680105831,
        &OptionU64::Some(1711662757),
        &test.cids.get(0).unwrap().unwrap(),
    );

    test.did_contract
        .revoke_credential(&test.authentication_address, &new_chaincert);
}

#[test]
#[should_panic(expected = "Status(ContractError(11))")]
fn test_request_chaincerts_when_no_chaincerts_set() {
    let test = DIDContractTest::setup();

    test.did_contract
        .get_credentials(&test.authentication_address);
}

#[test]
#[should_panic(expected = "Status(ContractError(12))")]
fn test_remove_authentication_with_only_one_authentication() {
    let test = DIDContractTest::setup();

    test.did_contract
        .remove_authentication(&test.authentication_address, &test.authentication);
}

#[test]
#[should_panic(expected = "Status(ContractError(12))")]
fn test_remove_authentication_with_non_existent_key() {
    let test = DIDContractTest::setup();
    let key_id = String::from_slice(&test.env, "did:chaincerts::ABC#key2");
    let invalid_key_id = String::from_slice(&test.env, "did:chaincerts::ABC#key_invalid");

    test.did_contract.add_authentication(
        &test.authentication_address,
        &key_id,
        &Address::random(&test.env),
    );

    test.did_contract
        .remove_authentication(&test.authentication_address, &invalid_key_id);
}

#[test]
#[should_panic(expected = "Status(ContractError(13))")]
fn test_remove_verification_method_with_only_one_authentication() {
    let test = DIDContractTest::setup();

    test.did_contract
        .remove_verification_method(&test.authentication_address, &test.authentication);
}

#[test]
#[should_panic(expected = "Status(ContractError(13))")]
fn test_remove_verification_method_with_non_existent_key() {
    let test = DIDContractTest::setup();
    let key_id = String::from_slice(&test.env, "did:chaincerts::ABC#key2");
    let invalid_key_id = String::from_slice(&test.env, "did:chaincerts::ABC#key_invalid");

    test.did_contract.add_verification_method(
        &test.authentication_address,
        &key_id,
        &Address::random(&test.env),
    );

    test.did_contract
        .remove_verification_method(&test.authentication_address, &invalid_key_id);
}

#[test]
#[should_panic(expected = "Status(ContractError(14))")]
fn test_add_invalid_public_read_cap() {
    let test = DIDContractTest::setup();
    let invalid_cap = CapabilityInvocation {
        id: String::from_slice(&test.env, "did:chaincerts:ABC123#capability-11"),
        type_: CapType::PublicRead,
        invoker: OptionString::None,
        invoker_address: OptionAddress::Some(Address::random(&test.env)),
        credential: OptionString::Some(test.public_credential_did),
    };

    test.did_contract
        .add_capability(&test.authentication_address, &invalid_cap);
}
#[test]
#[should_panic(expected = "Status(ContractError(14))")]
fn test_add_invalid_read_credential_cap() {
    let test = DIDContractTest::setup();
    let invalid_cap = CapabilityInvocation {
        id: String::from_slice(&test.env, "did:chaincerts:ABC123#capability-11"),
        type_: CapType::ReadCredential,
        invoker: OptionString::None,
        invoker_address: OptionAddress::None,
        credential: OptionString::None,
    };

    test.did_contract
        .add_capability(&test.authentication_address, &invalid_cap);
}
#[test]
#[should_panic(expected = "Status(ContractError(14))")]
fn test_add_invalid_add_credential_cap() {
    let test = DIDContractTest::setup();
    let invalid_cap = CapabilityInvocation {
        id: String::from_slice(&test.env, "did:chaincerts:ABC123#capability-11"),
        type_: CapType::AddCredential,
        invoker: OptionString::None,
        invoker_address: OptionAddress::None,
        credential: OptionString::None,
    };

    test.did_contract
        .add_capability(&test.authentication_address, &invalid_cap);
}
