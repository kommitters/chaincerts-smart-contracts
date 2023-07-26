#![cfg(test)]
use crate::{
    authentication::VerificationMethod,
    capability_invocation::{CapType, CapabilityInvocation},
    did_document::{DIDDocument, Method, Service},
    option::{OptionAddress, OptionMethodService, OptionString, OptionU64},
    verifiable_credential::{CredentialSubject, VerifiableCredential},
    DIDContract, DIDContractClient,
};
use soroban_sdk::{testutils::Address as _, vec, Address, Env, IntoVal, String, Vec};

fn create_did_contract<'a>(
    e: &Env,
    id: &String,
    authentication_params: &(String, Address),
    context: &Vec<String>,
    verification_processes: &Vec<Method>,
    services: &Vec<Service>,
    public_add_cap: &Option<CapabilityInvocation>,
) -> DIDContractClient<'a> {
    let did_contract = DIDContractClient::new(e, &e.register_contract(None, DIDContract {}));
    did_contract.initialize(
        id,
        authentication_params,
        context,
        verification_processes,
        services,
        public_add_cap,
    );
    did_contract
}

struct DIDContractTest<'a> {
    env: Env,
    id: String,
    authentication: String,
    authentication_address: Address,
    authentication_params: (String, Address),
    did_contract: DIDContractClient<'a>,
    credential_did: String,
    public_credential_did: String,
    capability_invocations: Vec<CapabilityInvocation>,
    cids: Vec<String>,
    context: Vec<String>,
    verification_processes: Vec<Method>,
    services: Vec<Service>,
    public_add_cap: Option<CapabilityInvocation>,
    shared_address: Address,
    credential_subject: CredentialSubject,
}

impl<'a> DIDContractTest<'a> {
    fn setup() -> Self {
        let env: Env = Default::default();
        env.mock_all_auths();
        let id = String::from_slice(&env, "did:chaincerts::ABC123");
        let authentication = String::from_slice(&env, "did:chaincerts:ABC123#key1");
        let authentication_address = Address::random(&env);
        let authentication_params = (authentication.clone(), authentication_address.clone());
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
        let public_add_cap = Option::None;
        let did_contract = create_did_contract(
            &env,
            &id,
            &authentication_params,
            &context,
            &verification_processes,
            &services,
            &public_add_cap,
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
        let credential_subject = CredentialSubject::new(
            String::from_slice(&env, "c8b875a2-3f5d-4a63-b1c8-791be9b01c02"),
            String::from_slice(&env, "Test"),
            String::from_slice(&env, "Test subject"),
        );

        DIDContractTest {
            env,
            id,
            authentication,
            authentication_address,
            authentication_params,
            did_contract,
            credential_did,
            public_credential_did,
            capability_invocations,
            cids,
            context,
            verification_processes,
            services,
            public_add_cap,
            shared_address,
            credential_subject,
        }
    }
}

#[test]
fn test_successful_execution_of_did_contract_capabilities() {
    let test = DIDContractTest::setup();
    let new_credential_did = String::from_slice(&test.env, "did:chaincerts:");
    let issuer = String::from_slice(&test.env, "did:chaincerts:ISSUER1");
    let verifiable_credential1 = VerifiableCredential {
        id: test.credential_did.clone(),
        issuer: issuer.clone(),
        issuance_date: 1680105831,
        expiration_date: OptionU64::Some(1711662757),
        credential_subject: test.credential_subject.clone(),
        attestation: test.cids.get_unchecked(0),
        revoked: false,
    };
    let verifiable_credential2 = VerifiableCredential {
        id: new_credential_did,
        issuer,
        issuance_date: 1680205831,
        expiration_date: OptionU64::None,
        credential_subject: test.credential_subject,
        attestation: test.cids.get_unchecked(0),
        revoked: false,
    };
    test.did_contract.add_capability(
        &test.authentication_address,
        &test.capability_invocations.get_unchecked(0),
    );
    test.did_contract.add_capability(
        &test.authentication_address,
        &test.capability_invocations.get_unchecked(1),
    );

    assert_eq!(
        test.did_contract
            .get_capability_invocation(&test.authentication_address)
            .len(),
        2
    );

    test.did_contract
        .deposit_credential(&verifiable_credential1);

    test.did_contract
        .deposit_credential(&verifiable_credential2);

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
        &test.capability_invocations.get_unchecked(0).id,
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
    let verifiable_credential1 = VerifiableCredential {
        id: test.credential_did,
        issuer: invoker.clone(),
        issuance_date: 1680105831,
        expiration_date: OptionU64::Some(1711662757),
        credential_subject: test.credential_subject.clone(),
        attestation: test.cids.get_unchecked(0),
        revoked: false,
    };
    let verifiable_credential2 = VerifiableCredential {
        id: test.public_credential_did,
        issuer: invoker.clone(),
        issuance_date: 1680205831,
        expiration_date: OptionU64::None,
        credential_subject: test.credential_subject,
        attestation: test.cids.get_unchecked(0),
        revoked: false,
    };

    test.did_contract.add_capability(
        &test.authentication_address,
        &test.capability_invocations.get_unchecked(0),
    );
    test.did_contract.add_capability(
        &test.authentication_address,
        &test.capability_invocations.get_unchecked(1),
    );
    test.did_contract.add_capability(
        &test.authentication_address,
        &test.capability_invocations.get_unchecked(2),
    );
    test.did_contract.add_capability(
        &test.authentication_address,
        &test.capability_invocations.get_unchecked(3),
    );

    test.did_contract
        .deposit_credential(&verifiable_credential1);

    test.did_contract
        .deposit_credential(&verifiable_credential2);

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
#[should_panic(expected = "HostError: Error(Contract, #1)")]
fn test_initialize_an_already_initialized_did_contract() {
    let test = DIDContractTest::setup();
    test.did_contract.initialize(
        &test.id,
        &(test.authentication, test.authentication_address),
        &test.context,
        &test.verification_processes,
        &test.services,
        &test.public_add_cap,
    );
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #2)")]
fn test_when_invalid_address() {
    let test = DIDContractTest::setup();
    let invalid_address = Address::random(&test.env);

    test.did_contract
        .get_capability_invocation(&invalid_address);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #4)")]
fn test_when_adding_an_already_added_org() {
    let test = DIDContractTest::setup();

    test.did_contract.add_capability(
        &test.authentication_address,
        &test.capability_invocations.get_unchecked(0),
    );
    test.did_contract.add_capability(
        &test.authentication_address,
        &test.capability_invocations.get_unchecked(0),
    );
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #6")]
fn test_remove_capability_when_no_capability_invocations_already_set() {
    let test = DIDContractTest::setup();
    test.did_contract.remove_capability(
        &test.authentication_address,
        &test.capability_invocations.get_unchecked(0).id,
    );
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #8)")]
fn test_remove_capability_when_organization_not_found() {
    let test = DIDContractTest::setup();
    test.did_contract.add_capability(
        &test.authentication_address,
        &test.capability_invocations.get_unchecked(0),
    );
    test.did_contract.remove_capability(
        &test.authentication_address,
        &test.capability_invocations.get_unchecked(1).id,
    );
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #2)")]
fn test_deposit_credential_when_not_share_cap_set() {
    let test = DIDContractTest::setup();
    let verifiable_credential1 = VerifiableCredential {
        id: test.credential_did,
        issuer: test.capability_invocations.get_unchecked(1).id,
        issuance_date: 1680105831,
        expiration_date: OptionU64::Some(1711662757),
        credential_subject: test.credential_subject,
        attestation: test.cids.get_unchecked(0),
        revoked: false,
    };

    test.did_contract.add_capability(
        &test.authentication_address,
        &test.capability_invocations.get_unchecked(0),
    );

    test.did_contract
        .deposit_credential(&verifiable_credential1);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #2)")]
fn test_deposit_credential_when_no_shared_cap_set() {
    let test = DIDContractTest::setup();
    let issuer = String::from_slice(&test.env, "did:chaincerts:ISSUER1");

    test.did_contract.add_capability(
        &test.authentication_address,
        &test.capability_invocations.get_unchecked(0),
    );

    test.did_contract
        .get_shared_credentials(&Address::random(&test.env), &issuer);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #9)")]
fn test_deposit_credential_already_in_the_did_contract() {
    let test = DIDContractTest::setup();
    let issuer = String::from_slice(&test.env, "did:chaincerts:ISSUER1");
    let verifiable_credential1 = VerifiableCredential {
        id: test.credential_did,
        issuer,
        issuance_date: 1680105831,
        expiration_date: OptionU64::Some(1711662757),
        credential_subject: test.credential_subject,
        attestation: test.cids.get_unchecked(0),
        revoked: false,
    };

    test.did_contract.add_capability(
        &test.authentication_address,
        &test.capability_invocations.get_unchecked(0),
    );
    test.did_contract.add_capability(
        &test.authentication_address,
        &test.capability_invocations.get_unchecked(1),
    );

    test.did_contract
        .deposit_credential(&verifiable_credential1);

    test.did_contract
        .deposit_credential(&verifiable_credential1);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #1)")]
fn test_revoke_credential_when_no_chaincerts_in_did_contract() {
    let test = DIDContractTest::setup();

    test.did_contract.add_capability(
        &test.authentication_address,
        &test.capability_invocations.get_unchecked(0),
    );
    test.did_contract
        .revoke_credential(&test.authentication_address, &test.credential_did)
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #0)")]
fn test_revoke_credential_when_chaincert_not_found() {
    let test = DIDContractTest::setup();
    let issuer_org1 = test.capability_invocations.get_unchecked(0);
    let issuer = String::from_slice(&test.env, "did:chaincerts:ISSUER1");
    let new_chaincert: String = "did:chaincerts:abc123#credential-invalid".into_val(&test.env);
    let verifiable_credential1 = VerifiableCredential {
        id: test.credential_did,
        issuer,
        issuance_date: 1680105831,
        expiration_date: OptionU64::Some(1711662757),
        credential_subject: test.credential_subject,
        attestation: test.cids.get_unchecked(0),
        revoked: false,
    };

    test.did_contract
        .add_capability(&test.authentication_address, &issuer_org1);
    test.did_contract
        .deposit_credential(&verifiable_credential1);

    test.did_contract
        .revoke_credential(&test.authentication_address, &new_chaincert);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #1)")]
fn test_request_chaincerts_when_no_chaincerts_set() {
    let test = DIDContractTest::setup();

    test.did_contract
        .get_credentials(&test.authentication_address);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #2)")]
fn test_remove_authentication_with_only_one_authentication() {
    let test = DIDContractTest::setup();

    test.did_contract
        .remove_authentication(&test.authentication_address, &test.authentication);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #2)")]
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
#[should_panic(expected = "HostError: Error(Contract, #3)")]
fn test_remove_verification_method_with_only_one_authentication() {
    let test = DIDContractTest::setup();

    test.did_contract
        .remove_verification_method(&test.authentication_address, &test.authentication);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #3)")]
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
#[should_panic(expected = "HostError: Error(Contract, #4)")]
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
#[should_panic(expected = "HostError: Error(Contract, #4)")]
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
#[should_panic(expected = "HostError: Error(Contract, #4)")]
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

#[test]
fn test_initialize_with_public_add_cap() {
    let test = DIDContractTest::setup();
    let public_add_cap = Option::Some(CapabilityInvocation {
        id: String::from_slice(&test.env, "did:chaincerts:ABC123#capability-1"),
        type_: CapType::PublicAdd,
        invoker: OptionString::None,
        invoker_address: OptionAddress::None,
        credential: OptionString::None,
    });
    let did_contract = create_did_contract(
        &test.env,
        &test.id,
        &test.authentication_params,
        &test.context,
        &test.verification_processes,
        &test.services,
        &public_add_cap,
    );

    // Check that the public add cap is set in storage
    assert!(did_contract.has_public_add_cap());

    // Check that the public add cap is added to the CapabilityInvocations
    let caps = did_contract.get_capability_invocation(&test.authentication_address);
    let first_cap = caps.get_unchecked(0);
    assert_eq!(first_cap, public_add_cap.unwrap());

    // Let's remove the public add cap
    did_contract.remove_capability(&test.authentication_address, &first_cap.id);
    assert!(!did_contract.has_public_add_cap());
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #4)")]
fn test_initialize_with_invalid_public_add_cap() {
    let test = DIDContractTest::setup();
    let invalid_public_add_cap = Option::Some(CapabilityInvocation {
        id: String::from_slice(&test.env, "did:chaincerts:ABC123#capability-1"),
        type_: CapType::AddCredential,
        invoker: OptionString::None,
        invoker_address: OptionAddress::None,
        credential: OptionString::None,
    });
    create_did_contract(
        &test.env,
        &test.id,
        &test.authentication_params,
        &test.context,
        &test.verification_processes,
        &test.services,
        &invalid_public_add_cap,
    );
}
