#![cfg(test)]
use crate::cert_wallet::{self, OptionU64};
use crate::issuance_trait::{CredentialParams, VerifiableCredential};
use crate::storage_types::{CredentialData, Info, Organization, Status};
use crate::{contract::CertIssuance, CertIssuanceClient};
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{vec, Address, Bytes, Env, IntoVal, Map, String, Vec};

const WASM: &[u8] = include_bytes!("../../target/wasm32-unknown-unknown/release/cert_wallet.wasm");

fn create_wallet_contract(env: &Env, owner: &Address, id: &Bytes) -> cert_wallet::Client {
    let wallet = cert_wallet::Client::new(env, &env.register_contract_wasm(None, WASM));
    wallet.initialize(owner);
    wallet.add_organization(id);
    wallet
}

fn create_cert_issuance(
    e: &Env,
    limit: &Option<u32>,
    recipient_dids: &Option<Vec<String>>,
    organization: &Organization,
    credential_params: &CredentialParams,
) -> CertIssuanceClient {
    let cert_issuance = CertIssuanceClient::new(e, &e.register_contract(None, CertIssuance {}));

    cert_issuance.initialize(
        &"Issuance Contract Name".into_val(e),
        recipient_dids,
        limit,
        organization,
        credential_params,
    );
    cert_issuance
}

fn create_random_recipient_dids(e: &Env) -> Vec<String> {
    let recipient_1 = String::from_slice(e, "did:chaincerts:abc123");
    let recipient_2 = String::from_slice(e, "did:chaincerts:def123");
    let recipient_3 = String::from_slice(e, "did:chaincerts:ghi123");
    vec![e, recipient_1, recipient_2, recipient_3]
}

#[test]
fn test_create_cert_data() {
    let e: Env = Default::default();
    let did: Bytes = "did:chaincerts:abc123#credential-xyz123".into_val(&e);
    let status = Status::Distributed;
    let issuance_date = OptionU64::Some(1711195200);
    let cert_data = CredentialData::new(
        did.clone(),
        status.clone(),
        String::from_slice(&e, "did:chaincerts:abc123"),
        String::from_slice(&e, "Work"),
        String::from_slice(&e, "Software Engineer"),
        issuance_date.clone(),
        String::from_slice(&e, "MEUCIFZ5o9zSYiC9d0hvN6V73Y8yBm9n3MF8Hj"),
    );
    assert_eq!(cert_data.did, did);
    assert_eq!(cert_data.status, status);
    assert_eq!(cert_data.issuance_date, issuance_date);
}

#[test]
fn test_initialize_contract_with_recipients() {
    let e: Env = Default::default();
    let recipients: Option<Vec<String>> = Option::Some(create_random_recipient_dids(&e));
    let organization: Organization = Organization {
        admin: Address::random(&e),
        did: "did:chaincerts:org123".into_val(&e),
    };

    let credential_params = CredentialParams {
        file_storage: "FileBase".into_val(&e),
        revocable: true,
        expiration_time: OptionU64::Some(1680091200),
        credential_type: String::from_slice(&e, "Work"),
        credential_title: String::from_slice(&e, "Software Engineer"),
    };

    let cert_issuance: CertIssuanceClient = create_cert_issuance(
        &e,
        &Option::None,
        &recipients,
        &organization,
        &credential_params,
    );
    assert_eq!(cert_issuance.file_storage(), "FileBase".into_val(&e));
    assert_eq!(cert_issuance.name(), "Issuance Contract Name".into_val(&e));
    assert!(cert_issuance.is_revocable());
    assert_eq!(cert_issuance.expiration_time(), OptionU64::Some(1680091200));
    assert_eq!(cert_issuance.distribution_limit(), 3);
    assert_eq!(cert_issuance.supply(), 0);
    assert_eq!(cert_issuance.recipients().len(), 3);
}

#[test]
fn test_initialize_with_limit_contract() {
    let e: Env = Default::default();
    let organization: Organization = Organization {
        admin: Address::random(&e),
        did: "did:chaincerts:org123".into_val(&e),
    };
    let distribution_limit: Option<u32> = Option::Some(6);

    let credential_params = CredentialParams {
        file_storage: "FileBase".into_val(&e),
        revocable: true,
        expiration_time: OptionU64::None,
        credential_type: String::from_slice(&e, "Work"),
        credential_title: String::from_slice(&e, "Software Engineer"),
    };

    let cert_issuance = create_cert_issuance(
        &e,
        &distribution_limit,
        &Option::None,
        &organization,
        &credential_params,
    );
    assert_eq!(cert_issuance.file_storage(), "FileBase".into_val(&e));
    assert_eq!(cert_issuance.name(), "Issuance Contract Name".into_val(&e));
    assert!(cert_issuance.is_revocable());
    assert_eq!(cert_issuance.expiration_time(), OptionU64::None);
    assert_eq!(cert_issuance.distribution_limit(), 6);
    assert_eq!(cert_issuance.supply(), 0);
    assert_eq!(cert_issuance.recipients().len(), 0);
}

#[test]
fn test_initialize_without_limit_contract_and_recipients() {
    let e: Env = Default::default();
    let organization: Organization = Organization {
        admin: Address::random(&e),
        did: "did:chaincerts:org123".into_val(&e),
    };

    let credential_params = CredentialParams {
        file_storage: "FileBase".into_val(&e),
        revocable: true,
        expiration_time: OptionU64::None,
        credential_type: String::from_slice(&e, "Work"),
        credential_title: String::from_slice(&e, "Software Engineer"),
    };

    let cert_issuance = create_cert_issuance(
        &e,
        &Option::None,
        &Option::None,
        &organization,
        &credential_params,
    );
    assert_eq!(cert_issuance.file_storage(), "FileBase".into_val(&e));
    assert_eq!(cert_issuance.name(), "Issuance Contract Name".into_val(&e));
    assert!(cert_issuance.is_revocable());
    assert_eq!(cert_issuance.expiration_time(), OptionU64::None);
    assert_eq!(cert_issuance.distribution_limit(), 10);
    assert_eq!(cert_issuance.supply(), 0);
    assert_eq!(cert_issuance.recipients().len(), 0);
}

#[test]
fn test_get_contract_info() {
    let e: Env = Default::default();
    let organization: Organization = Organization {
        admin: Address::random(&e),
        did: "did:chaincerts:org123".into_val(&e),
    };

    let credential_params_without_expiration_time = CredentialParams {
        file_storage: "FileBase".into_val(&e),
        revocable: true,
        expiration_time: OptionU64::None,
        credential_type: String::from_slice(&e, "Work"),
        credential_title: String::from_slice(&e, "Software Engineer"),
    };

    let credential_params_with_expiration_time = CredentialParams {
        file_storage: "FileBase".into_val(&e),
        revocable: true,
        expiration_time: OptionU64::Some(31556926),
        credential_type: String::from_slice(&e, "Work"),
        credential_title: String::from_slice(&e, "Software Engineer"),
    };

    let distribution_limit: Option<u32> = Option::Some(6);

    let cert_issuance = create_cert_issuance(
        &e,
        &distribution_limit,
        &Option::None,
        &organization,
        &credential_params_without_expiration_time,
    );

    let cert_issuance_2 = create_cert_issuance(
        &e,
        &distribution_limit,
        &Option::None,
        &organization,
        &credential_params_with_expiration_time,
    );

    let info = Info {
        name: "Issuance Contract Name".into_val(&e),
        revocable: true,
        expiration_time: OptionU64::None,
        distribution_limit: 6,
        supply: 0,
    };

    let info_2 = Info {
        name: "Issuance Contract Name".into_val(&e),
        revocable: true,
        expiration_time: OptionU64::Some(31556926),
        distribution_limit: 6,
        supply: 0,
    };

    assert_eq!(cert_issuance.info(), info);
    assert_eq!(cert_issuance_2.info(), info_2);
}

#[test]
fn test_distribute_with_distribution_limit_contract() {
    let e: Env = Default::default();
    let recipient1_address = Address::random(&e);
    let recipient1_did = String::from_slice(&e, "did:chaincerts:abc123");
    let organization: Organization = Organization {
        admin: Address::random(&e),
        did: "did:chaincerts:org123".into_val(&e),
    };
    let wallet = create_wallet_contract(&e, &recipient1_address, &organization.did);
    let distribution_limit: Option<u32> = Option::Some(6);
    let credential_params = CredentialParams {
        file_storage: "FileBase".into_val(&e),
        revocable: true,
        expiration_time: OptionU64::None,
        credential_type: String::from_slice(&e, "Work"),
        credential_title: String::from_slice(&e, "Software Engineer"),
    };
    let cert_issuance = create_cert_issuance(
        &e,
        &distribution_limit,
        &Option::None,
        &organization,
        &credential_params,
    );

    const ATTESTATION1: &str = "ipfs://QmdtyfTYbVS3K9iYqBPjXxn4mbB7aBvEjYGzYWnzRcMrEC";

    let verificable_credential = VerifiableCredential {
        did: "did:chaincerts:abc123#credential-xyz123".into_val(&e),
        id: "c8b875a2-3f5d-4a63-b1c8-791be9b01c02".into_val(&e),
        recipient_did: recipient1_did,
        signature: String::from_slice(&e, "MEUCIFZ5o9zSYiC9d0hvN6V73Y8yBm9n3MF8Hj"),
        attestation: ATTESTATION1.into_val(&e),
        issuance_date: 1679918400,
    };

    cert_issuance.distribute(
        &organization.admin,
        &wallet.contract_id,
        &verificable_credential,
    );
    let recipients: Map<String, Option<CredentialData>> = cert_issuance.recipients();
    let cert_data = recipients
        .get(verificable_credential.recipient_did)
        .unwrap()
        .unwrap()
        .unwrap();

    assert_eq!(cert_data.status, Status::Distributed);
    assert_eq!(cert_issuance.supply(), 1);
    assert_eq!(recipients.len(), 1);
    assert_eq!(wallet.get_chaincerts().len(), 1);
}

#[test]
fn test_distribute_with_initial_recipients() {
    let e: Env = Default::default();
    let recipients = Option::Some(create_random_recipient_dids(&e));
    let recipient1_address = Address::random(&e);
    let recipient1_did = recipients
        .clone()
        .expect("Vec of recipients")
        .get(0)
        .unwrap()
        .unwrap();
    let organization: Organization = Organization {
        admin: Address::random(&e),
        did: "did:chaincerts:org123".into_val(&e),
    };
    let wallet = create_wallet_contract(&e, &recipient1_address, &organization.did);
    const ATTESTATION1: &str = "ipfs://QmdtyfTYbVS3K9iYqBPjXxn4mbB7aBvEjYGzYWnzRcMrEC";
    let credential_params = CredentialParams {
        file_storage: "FileBase".into_val(&e),
        revocable: true,
        expiration_time: OptionU64::Some(31556926),
        credential_type: String::from_slice(&e, "Work"),
        credential_title: String::from_slice(&e, "Software Engineer"),
    };
    let cert_issuance = create_cert_issuance(
        &e,
        &Option::None,
        &recipients,
        &organization,
        &credential_params,
    );

    let mut recipients = cert_issuance.recipients();
    let mut cert_data = recipients.get(recipient1_did.clone()).unwrap().unwrap();
    assert_eq!(cert_data, Option::None);

    let verifiable_credential = VerifiableCredential {
        did: "did:chaincerts:abc123#credential-xyz123".into_val(&e),
        id: "c8b875a2-3f5d-4a63-b1c8-791be9b01c02".into_val(&e),
        recipient_did: recipient1_did,
        signature: String::from_slice(&e, "MEUCIFZ5o9zSYiC9d0hvN6V73Y8yBm9n3MF8Hj"),
        attestation: ATTESTATION1.into_val(&e),
        issuance_date: 1679918400,
    };

    cert_issuance.distribute(
        &organization.admin,
        &wallet.contract_id,
        &verifiable_credential,
    );

    recipients = cert_issuance.recipients();
    cert_data = recipients
        .get(verifiable_credential.recipient_did)
        .unwrap()
        .unwrap();

    assert_eq!(cert_data.unwrap().status, Status::Distributed);
    assert_eq!(cert_issuance.supply(), 1);
    assert_eq!(recipients.len(), 3);
    assert_eq!(wallet.get_chaincerts().len(), 1);
}

#[test]
fn test_revoke_chaincert() {
    let e: Env = Default::default();
    let recipients: Option<Vec<String>> = Option::Some(create_random_recipient_dids(&e));
    let recipient_address = Address::random(&e);
    let recipient_did = recipients
        .clone()
        .expect("Vec of recipients")
        .get(0)
        .unwrap()
        .unwrap();
    let organization: Organization = Organization {
        admin: Address::random(&e),
        did: "did:chaincerts:org123".into_val(&e),
    };
    let wallet = create_wallet_contract(&e, &recipient_address, &organization.did);

    pub const ATTESTATION1: &str = "ipfs://QmdtyfTYbVS3K9iYqBPjXxn4mbB7aBvEjYGzYWnzRcMrEC";

    let credential_params = CredentialParams {
        file_storage: "FileBase".into_val(&e),
        revocable: true,
        expiration_time: OptionU64::None,
        credential_type: String::from_slice(&e, "Work"),
        credential_title: String::from_slice(&e, "Software Engineer"),
    };

    let cert_issuance = create_cert_issuance(
        &e,
        &Option::None,
        &recipients,
        &organization,
        &credential_params,
    );

    let verifiable_credential = VerifiableCredential {
        did: "did:chaincerts:abc123#credential-xyz123".into_val(&e),
        id: "c8b875a2-3f5d-4a63-b1c8-791be9b01c02".into_val(&e),
        recipient_did,
        signature: String::from_slice(&e, "MEUCIFZ5o9zSYiC9d0hvN6V73Y8yBm9n3MF8Hj"),
        attestation: ATTESTATION1.into_val(&e),
        issuance_date: 1679918400,
    };

    cert_issuance.distribute(
        &organization.admin,
        &wallet.contract_id,
        &verifiable_credential,
    );

    let mut recipients = cert_issuance.recipients();
    let mut cert_data = recipients
        .get(verifiable_credential.recipient_did.clone())
        .unwrap()
        .unwrap()
        .unwrap();
    assert_eq!(cert_data.status, Status::Distributed);

    cert_issuance.revoke(
        &organization.admin,
        &verifiable_credential.recipient_did,
        &wallet.contract_id,
    );

    recipients = cert_issuance.recipients();
    cert_data = recipients
        .get(verifiable_credential.recipient_did)
        .unwrap()
        .unwrap()
        .unwrap();
    assert_eq!(cert_data.status, Status::Revoked);

    let chaincert = wallet.get_chaincerts().get(0).unwrap().unwrap();
    assert!(chaincert.revoked);
}

#[test]
#[should_panic(expected = "Status(ContractError(1))")]
fn test_initialize_contract_with_recipients_error() {
    let e: Env = Default::default();
    let recipient_dids: Option<Vec<String>> = Option::Some(create_random_recipient_dids(&e));
    let organization: Organization = Organization {
        admin: Address::random(&e),
        did: "did:chaincerts:org123".into_val(&e),
    };

    let credential_params = CredentialParams {
        file_storage: "FileBase".into_val(&e),
        revocable: true,
        expiration_time: OptionU64::None,
        credential_type: String::from_slice(&e, "Work"),
        credential_title: String::from_slice(&e, "Software Engineer"),
    };

    let cert_issuance: CertIssuanceClient = create_cert_issuance(
        &e,
        &Option::None,
        &recipient_dids,
        &organization,
        &credential_params,
    );

    cert_issuance.initialize(
        &"Issuance Contract Name".into_val(&e),
        &recipient_dids,
        &Option::None,
        &organization,
        &credential_params,
    );
}

#[test]
#[should_panic(expected = "Status(ContractError(1))")]
fn test_initialize_with_limit_contract_error() {
    let e: Env = Default::default();
    let organization: Organization = Organization {
        admin: Address::random(&e),
        did: "did:chaincerts:org123".into_val(&e),
    };

    let credential_params = CredentialParams {
        file_storage: "FileBase".into_val(&e),
        revocable: true,
        expiration_time: OptionU64::None,
        credential_type: String::from_slice(&e, "Work"),
        credential_title: String::from_slice(&e, "Software Engineer"),
    };

    let distribution_limit: Option<u32> = Option::Some(6);
    let cert_issuance = create_cert_issuance(
        &e,
        &distribution_limit,
        &Option::None,
        &organization,
        &credential_params,
    );

    cert_issuance.initialize(
        &"Issuance Contract Name".into_val(&e),
        &Option::None,
        &distribution_limit,
        &organization,
        &credential_params,
    );
}

#[test]
#[should_panic(expected = "Status(ContractError(2))")]
fn test_distribute_admin_error() {
    const ATTESTATION1: &str = "ipfs://QmdtyfTYbVS3K9iYqBPjXxn4mbB7aBvEjYGzYWnzRcMrEC";
    let e: Env = Default::default();
    let invalid_admin = Address::random(&e);
    let recipient_address = Address::random(&e);
    let recipient_dids = Option::Some(create_random_recipient_dids(&e));
    let recipient_did = recipient_dids
        .clone()
        .expect("Vec of recipients")
        .get(0)
        .unwrap()
        .unwrap();
    let organization: Organization = Organization {
        admin: Address::random(&e),
        did: "did:chaincerts:org123".into_val(&e),
    };
    let wallet = create_wallet_contract(&e, &recipient_address, &organization.did);

    let credential_params = CredentialParams {
        file_storage: "FileBase".into_val(&e),
        revocable: true,
        expiration_time: OptionU64::None,
        credential_type: String::from_slice(&e, "Work"),
        credential_title: String::from_slice(&e, "Software Engineer"),
    };

    let cert_issuance = create_cert_issuance(
        &e,
        &Option::None,
        &recipient_dids,
        &organization,
        &credential_params,
    );

    let verifiable_credential = VerifiableCredential {
        did: "did:chaincerts:abc123#credential-xyz123".into_val(&e),
        id: "c8b875a2-3f5d-4a63-b1c8-791be9b01c02".into_val(&e),
        recipient_did,
        signature: String::from_slice(&e, "MEUCIFZ5o9zSYiC9d0hvN6V73Y8yBm9n3MF8Hj"),
        attestation: ATTESTATION1.into_val(&e),
        issuance_date: 1679918400,
    };

    cert_issuance.distribute(&invalid_admin, &wallet.contract_id, &verifiable_credential);
}

#[test]
#[should_panic(expected = "Status(ContractError(3))")]
fn test_distribute_limit_error() {
    let e: Env = Default::default();
    let recipients = Option::Some(create_random_recipient_dids(&e));
    let address1 = Address::random(&e);
    let address2 = Address::random(&e);
    let organization: Organization = Organization {
        admin: Address::random(&e),
        did: "did:chaincerts:org123".into_val(&e),
    };
    let wallet1 = create_wallet_contract(&e, &address1, &organization.did);
    let wallet2 = create_wallet_contract(&e, &address2, &organization.did);

    let credential_params = CredentialParams {
        file_storage: "FileBase".into_val(&e),
        revocable: true,
        expiration_time: OptionU64::None,
        credential_type: String::from_slice(&e, "Work"),
        credential_title: String::from_slice(&e, "Software Engineer"),
    };

    let distribution_limit = Option::Some(1);
    let cert_issuance = create_cert_issuance(
        &e,
        &distribution_limit,
        &Option::None,
        &organization,
        &credential_params,
    );
    const ATTESTATION1: &str = "ipfs://QmdtyfTYbVS3K9iYqBPjXxn4mbB7aBvEjYGzYWnzRcMrEC";
    const ATTESTATION2: &str = "ipfs://QmdtyfTYbVS3K9iYqBPjXxn4mbB7aBvEjYGzYWnzRcMrED";

    let mut verifiable_credential = VerifiableCredential {
        did: "did:chaincerts:abc123#credential-xyz123".into_val(&e),
        id: "c8b875a2-3f5d-4a63-b1c8-791be9b01c02".into_val(&e),
        recipient_did: recipients
            .clone()
            .expect("Vec of recipients")
            .get(0)
            .unwrap()
            .unwrap(),
        signature: String::from_slice(&e, "MEUCIFZ5o9zSYiC9d0hvN6V73Y8yBm9n3MF8Hj"),
        attestation: ATTESTATION1.into_val(&e),
        issuance_date: 1679918400,
    };

    cert_issuance.distribute(
        &organization.admin,
        &wallet1.contract_id,
        &verifiable_credential,
    );

    verifiable_credential.recipient_did = recipients
        .expect("Vec of recipients")
        .get(1)
        .unwrap()
        .unwrap();
    verifiable_credential.attestation = ATTESTATION2.into_val(&e);
    cert_issuance.distribute(
        &organization.admin,
        &wallet2.contract_id,
        &verifiable_credential,
    );
}

#[test]
#[should_panic(expected = "Status(ContractError(5))")]
fn test_distribute_status_error() {
    let e: Env = Default::default();
    let wallet_owner = Address::random(&e);
    let recipients = Option::Some(create_random_recipient_dids(&e));
    let organization: Organization = Organization {
        admin: Address::random(&e),
        did: "did:chaincerts:org123".into_val(&e),
    };
    let wallet = create_wallet_contract(&e, &wallet_owner, &organization.did);

    let credential_params = CredentialParams {
        file_storage: "FileBase".into_val(&e),
        revocable: true,
        expiration_time: OptionU64::None,
        credential_type: String::from_slice(&e, "Work"),
        credential_title: String::from_slice(&e, "Software Engineer"),
    };

    let distribution_limit = Option::Some(3);
    let cert_issuance = create_cert_issuance(
        &e,
        &distribution_limit,
        &Option::None,
        &organization,
        &credential_params,
    );
    pub const ATTESTATION1: &str = "ipfs://QmdtyfTYbVS3K9iYqBPjXxn4mbB7aBvEjYGzYWnzRcMrEC";

    let verifiable_credential = VerifiableCredential {
        did: "did:chaincerts:abc123#credential-xyz123".into_val(&e),
        id: "c8b875a2-3f5d-4a63-b1c8-791be9b01c02".into_val(&e),
        recipient_did: recipients
            .expect("Vec of recipients")
            .get(0)
            .unwrap()
            .unwrap(),
        signature: String::from_slice(&e, "MEUCIFZ5o9zSYiC9d0hvN6V73Y8yBm9n3MF8Hj"),
        attestation: ATTESTATION1.into_val(&e),
        issuance_date: 1679918400,
    };

    cert_issuance.distribute(
        &organization.admin,
        &wallet.contract_id,
        &verifiable_credential,
    );

    cert_issuance.distribute(
        &organization.admin,
        &wallet.contract_id,
        &verifiable_credential,
    );
}

#[test]
#[should_panic(expected = "Status(ContractError(2))")]
fn test_revoke_admin_error() {
    let e: Env = Default::default();
    let recipients: Option<Vec<String>> = Option::Some(create_random_recipient_dids(&e));
    let recipient_address = Address::random(&e);
    let recipient_did = recipients
        .clone()
        .expect("Vec of recipients")
        .get(0)
        .unwrap()
        .unwrap();
    let organization: Organization = Organization {
        admin: Address::random(&e),
        did: "did:chaincerts:org123".into_val(&e),
    };
    let wallet = create_wallet_contract(&e, &recipient_address, &organization.did);

    pub const ATTESTATION1: &str = "ipfs://QmdtyfTYbVS3K9iYqBPjXxn4mbB7aBvEjYGzYWnzRcMrEC";

    let credential_params = CredentialParams {
        file_storage: "FileBase".into_val(&e),
        revocable: true,
        expiration_time: OptionU64::None,
        credential_type: String::from_slice(&e, "Work"),
        credential_title: String::from_slice(&e, "Software Engineer"),
    };

    let cert_issuance = create_cert_issuance(
        &e,
        &Option::None,
        &recipients,
        &organization,
        &credential_params,
    );

    let verifiable_credential = VerifiableCredential {
        did: "did:chaincerts:abc123#credential-xyz123".into_val(&e),
        id: "c8b875a2-3f5d-4a63-b1c8-791be9b01c02".into_val(&e),
        recipient_did,
        signature: String::from_slice(&e, "MEUCIFZ5o9zSYiC9d0hvN6V73Y8yBm9n3MF8Hj"),
        attestation: ATTESTATION1.into_val(&e),
        issuance_date: 1679918400,
    };

    cert_issuance.distribute(
        &organization.admin,
        &wallet.contract_id,
        &verifiable_credential,
    );

    cert_issuance.revoke(
        &Address::random(&e),
        &verifiable_credential.recipient_did,
        &wallet.contract_id,
    );
}

#[test]
#[should_panic(expected = "Status(ContractError(7))")]
fn test_revoke_credential_data_none_error() {
    let e: Env = Default::default();
    let recipients: Option<Vec<String>> = Option::Some(create_random_recipient_dids(&e));
    let recipient_address = Address::random(&e);
    let recipient_did = recipients
        .clone()
        .expect("Vec of recipients")
        .get(0)
        .unwrap()
        .unwrap();
    let org_did = "did:chaincerts:org123".into_val(&e);
    let wallet = create_wallet_contract(&e, &recipient_address, &org_did);
    let organization: Organization = Organization {
        admin: Address::random(&e),
        did: "did:chaincerts:org123".into_val(&e),
    };

    let credential_params = CredentialParams {
        file_storage: "FileBase".into_val(&e),
        revocable: true,
        expiration_time: OptionU64::None,
        credential_type: String::from_slice(&e, "Work"),
        credential_title: String::from_slice(&e, "Software Engineer"),
    };

    let cert_issuance = create_cert_issuance(
        &e,
        &Option::None,
        &recipients,
        &organization,
        &credential_params,
    );

    cert_issuance.revoke(&organization.admin, &recipient_did, &wallet.contract_id);
}

#[test]
#[should_panic(expected = "Status(ContractError(7))")]
fn test_revoke_status_revoked_error() {
    let e: Env = Default::default();
    let recipients: Option<Vec<String>> = Option::Some(create_random_recipient_dids(&e));
    let recipient_did = recipients
        .clone()
        .expect("Vec of recipients")
        .get(0)
        .unwrap()
        .unwrap();

    let organization: Organization = Organization {
        admin: Address::random(&e),
        did: "did:chaincerts:org123".into_val(&e),
    };
    let recipient_address = Address::random(&e);
    let wallet = create_wallet_contract(&e, &recipient_address, &organization.did);
    const ATTESTATION1: &str = "ipfs://QmdtyfTYbVS3K9iYqBPjXxn4mbB7aBvEjYGzYWnzRcMrEC";

    let credential_params = CredentialParams {
        file_storage: "FileBase".into_val(&e),
        revocable: true,
        expiration_time: OptionU64::None,
        credential_type: String::from_slice(&e, "Work"),
        credential_title: String::from_slice(&e, "Software Engineer"),
    };

    let cert_issuance = create_cert_issuance(
        &e,
        &Option::None,
        &recipients,
        &organization,
        &credential_params,
    );

    let verifiable_credential = VerifiableCredential {
        did: "did:chaincerts:abc123#credential-xyz123".into_val(&e),
        id: "c8b875a2-3f5d-4a63-b1c8-791be9b01c02".into_val(&e),
        recipient_did,
        signature: String::from_slice(&e, "MEUCIFZ5o9zSYiC9d0hvN6V73Y8yBm9n3MF8Hj"),
        attestation: ATTESTATION1.into_val(&e),
        issuance_date: 1679918400,
    };

    cert_issuance.distribute(
        &organization.admin,
        &wallet.contract_id,
        &verifiable_credential,
    );

    cert_issuance.revoke(
        &organization.admin,
        &verifiable_credential.recipient_did,
        &wallet.contract_id,
    );
    cert_issuance.revoke(
        &organization.admin,
        &verifiable_credential.recipient_did,
        &wallet.contract_id,
    );
}

#[test]
#[should_panic(expected = "Status(ContractError(7))")]
fn test_revoke_no_revocable_cert() {
    let e: Env = Default::default();
    let recipients: Option<Vec<String>> = Option::Some(create_random_recipient_dids(&e));
    let recipient_address = Address::random(&e);
    let recipient_did = recipients
        .clone()
        .expect("Vec of recipients")
        .get(0)
        .unwrap()
        .unwrap();
    let wallet = create_wallet_contract(&e, &recipient_address, &"12345".into_val(&e));
    let organization: Organization = Organization {
        admin: Address::random(&e),
        did: "did:chaincerts:org123".into_val(&e),
    };

    let credential_params = CredentialParams {
        file_storage: "FileBase".into_val(&e),
        revocable: false,
        expiration_time: OptionU64::None,
        credential_type: String::from_slice(&e, "Work"),
        credential_title: String::from_slice(&e, "Software Engineer"),
    };

    let cert_issuance = create_cert_issuance(
        &e,
        &Option::None,
        &recipients,
        &organization,
        &credential_params,
    );

    cert_issuance.revoke(&organization.admin, &recipient_did, &wallet.contract_id);
}
