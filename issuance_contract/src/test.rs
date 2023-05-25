#![cfg(test)]
use crate::did_contract::{self, OptionU64};
use crate::issuance_trait::{CredentialParams, CredentialStatus, DistributeCredential};
use crate::storage_types::{CredentialData, Info, Organization, RevokedCredential};
use crate::{contract::IssuanceContract, IssuanceContractClient};
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{vec, Address, Bytes, Env, IntoVal, Map, String, Vec};

const WASM: &[u8] = include_bytes!("../did_contract.wasm");

fn create_did_contract(env: &Env, owner: &Address, id: &Bytes) -> did_contract::Client {
    let wallet = did_contract::Client::new(env, &env.register_contract_wasm(None, WASM));
    wallet.initialize(owner);
    wallet.add_organization(id);
    wallet
}

fn create_issuance_contract(
    e: &Env,
    limit: &Option<u32>,
    recipient_dids: &Option<Vec<String>>,
    organization: &Organization,
    credential_params: &CredentialParams,
) -> IssuanceContractClient {
    let issuance_contract =
        IssuanceContractClient::new(e, &e.register_contract(None, IssuanceContract {}));

    issuance_contract.initialize(
        &"Issuance Contract Name".into_val(e),
        recipient_dids,
        limit,
        organization,
        credential_params,
    );
    issuance_contract
}

fn create_random_recipient_dids(e: &Env) -> Vec<String> {
    let recipient_1 = String::from_slice(e, "did:chaincerts:abc123");
    let recipient_2 = String::from_slice(e, "did:chaincerts:def123");
    let recipient_3 = String::from_slice(e, "did:chaincerts:ghi123");
    vec![e, recipient_1, recipient_2, recipient_3]
}

fn setup_initialized_and_distributed_contract() -> (
    Env,
    Organization,
    DistributeCredential,
    IssuanceContractClient,
) {
    let e: Env = Default::default();
    let recipient_address = Address::random(&e);
    let recipient_did = String::from_slice(&e, "did:chaincerts:abc123");
    let organization: Organization = Organization {
        admin: Address::random(&e),
        did: "did:chaincerts:org123".into_val(&e),
    };
    let wallet = create_did_contract(&e, &recipient_address, &organization.did);
    let distribution_limit: Option<u32> = Option::Some(6);
    let credential_params = CredentialParams {
        file_storage: "FileBase".into_val(&e),
        revocable: true,
        credential_type: String::from_slice(&e, "Work"),
        credential_title: String::from_slice(&e, "Software Engineer"),
    };
    let issuance_contract = create_issuance_contract(
        &e,
        &distribution_limit,
        &Option::None,
        &organization,
        &credential_params,
    );

    const ATTESTATION1: &str = "ipfs://QmdtyfTYbVS3K9iYqBPjXxn4mbB7aBvEjYGzYWnzRcMrEC";

    let verifiable_credential = DistributeCredential {
        did: "did:chaincerts:abc123#credential-xyz123".into_val(&e),
        id: "c8b875a2-3f5d-4a63-b1c8-791be9b01c02".into_val(&e),
        recipient_did,
        signature: String::from_slice(&e, "MEUCIFZ5o9zSYiC9d0hvN6V73Y8yBm9n3MF8Hj"),
        attestation: ATTESTATION1.into_val(&e),
        issuance_date: 1679918400,
        expiration_date: OptionU64::None,
    };

    issuance_contract.distribute(
        &organization.admin,
        &wallet.contract_id,
        &verifiable_credential,
    );
    (e, organization, verifiable_credential, issuance_contract)
}

#[test]
fn test_create_cert_data() {
    let e: Env = Default::default();
    let did: Bytes = "did:chaincerts:abc123#credential-xyz123".into_val(&e);
    let issuance_date = 1711195200;
    let cert_data = CredentialData::new(
        did.clone(),
        String::from_slice(&e, "did:chaincerts:abc123"),
        String::from_slice(&e, "Work"),
        String::from_slice(&e, "Software Engineer"),
        issuance_date.clone(),
        OptionU64::None,
        String::from_slice(&e, "MEUCIFZ5o9zSYiC9d0hvN6V73Y8yBm9n3MF8Hj"),
    );
    assert_eq!(cert_data.did, did);
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

        credential_type: String::from_slice(&e, "Work"),
        credential_title: String::from_slice(&e, "Software Engineer"),
    };

    let issuance_contract: IssuanceContractClient = create_issuance_contract(
        &e,
        &Option::None,
        &recipients,
        &organization,
        &credential_params,
    );
    assert_eq!(issuance_contract.file_storage(), "FileBase".into_val(&e));
    assert_eq!(
        issuance_contract.name(),
        "Issuance Contract Name".into_val(&e)
    );
    assert!(issuance_contract.is_revocable());

    assert_eq!(issuance_contract.distribution_limit(), 3);
    assert_eq!(issuance_contract.supply(), 0);
    assert_eq!(issuance_contract.recipients().len(), 3);
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
        credential_type: String::from_slice(&e, "Work"),
        credential_title: String::from_slice(&e, "Software Engineer"),
    };

    let issuance_contract = create_issuance_contract(
        &e,
        &distribution_limit,
        &Option::None,
        &organization,
        &credential_params,
    );
    assert_eq!(issuance_contract.file_storage(), "FileBase".into_val(&e));
    assert_eq!(
        issuance_contract.name(),
        "Issuance Contract Name".into_val(&e)
    );
    assert!(issuance_contract.is_revocable());
    assert_eq!(issuance_contract.distribution_limit(), 6);
    assert_eq!(issuance_contract.supply(), 0);
    assert_eq!(issuance_contract.recipients().len(), 0);
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
        credential_type: String::from_slice(&e, "Work"),
        credential_title: String::from_slice(&e, "Software Engineer"),
    };

    let issuance_contract = create_issuance_contract(
        &e,
        &Option::None,
        &Option::None,
        &organization,
        &credential_params,
    );
    assert_eq!(issuance_contract.file_storage(), "FileBase".into_val(&e));
    assert_eq!(
        issuance_contract.name(),
        "Issuance Contract Name".into_val(&e)
    );
    assert!(issuance_contract.is_revocable());
    assert_eq!(issuance_contract.distribution_limit(), 10);
    assert_eq!(issuance_contract.supply(), 0);
    assert_eq!(issuance_contract.recipients().len(), 0);
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
        credential_type: String::from_slice(&e, "Work"),
        credential_title: String::from_slice(&e, "Software Engineer"),
    };

    let credential_params_with_expiration_time = CredentialParams {
        file_storage: "FileBase".into_val(&e),
        revocable: true,
        credential_type: String::from_slice(&e, "Work"),
        credential_title: String::from_slice(&e, "Software Engineer"),
    };

    let distribution_limit: Option<u32> = Option::Some(6);

    let issuance_contract = create_issuance_contract(
        &e,
        &distribution_limit,
        &Option::None,
        &organization,
        &credential_params_without_expiration_time,
    );

    let issuance_contract_2 = create_issuance_contract(
        &e,
        &distribution_limit,
        &Option::None,
        &organization,
        &credential_params_with_expiration_time,
    );

    let info = Info {
        name: "Issuance Contract Name".into_val(&e),
        revocable: true,
        distribution_limit: 6,
        supply: 0,
        credential_type: String::from_slice(&e, "Work"),
        credential_title: String::from_slice(&e, "Software Engineer"),
    };

    let info_2 = Info {
        name: "Issuance Contract Name".into_val(&e),
        revocable: true,
        distribution_limit: 6,
        supply: 0,
        credential_type: String::from_slice(&e, "Work"),
        credential_title: String::from_slice(&e, "Software Engineer"),
    };

    assert_eq!(issuance_contract.info(), info);
    assert_eq!(issuance_contract_2.info(), info_2);
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
    let wallet = create_did_contract(&e, &recipient1_address, &organization.did);
    let distribution_limit: Option<u32> = Option::Some(6);
    let credential_params = CredentialParams {
        file_storage: "FileBase".into_val(&e),
        revocable: true,
        credential_type: String::from_slice(&e, "Work"),
        credential_title: String::from_slice(&e, "Software Engineer"),
    };
    let issuance_contract = create_issuance_contract(
        &e,
        &distribution_limit,
        &Option::None,
        &organization,
        &credential_params,
    );

    const ATTESTATION1: &str = "ipfs://QmdtyfTYbVS3K9iYqBPjXxn4mbB7aBvEjYGzYWnzRcMrEC";

    let verifiable_credential = DistributeCredential {
        did: "did:chaincerts:abc123#credential-xyz123".into_val(&e),
        id: "c8b875a2-3f5d-4a63-b1c8-791be9b01c02".into_val(&e),
        recipient_did: recipient1_did,
        signature: String::from_slice(&e, "MEUCIFZ5o9zSYiC9d0hvN6V73Y8yBm9n3MF8Hj"),
        attestation: ATTESTATION1.into_val(&e),
        issuance_date: 1679918400,
        expiration_date: OptionU64::None,
    };

    issuance_contract.distribute(
        &organization.admin,
        &wallet.contract_id,
        &verifiable_credential,
    );
    let recipients: Map<String, Option<CredentialData>> = issuance_contract.recipients();

    assert_eq!(issuance_contract.supply(), 1);
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
    let wallet = create_did_contract(&e, &recipient1_address, &organization.did);
    const ATTESTATION1: &str = "ipfs://QmdtyfTYbVS3K9iYqBPjXxn4mbB7aBvEjYGzYWnzRcMrEC";
    let credential_params = CredentialParams {
        file_storage: "FileBase".into_val(&e),
        revocable: true,
        credential_type: String::from_slice(&e, "Work"),
        credential_title: String::from_slice(&e, "Software Engineer"),
    };
    let issuance_contract = create_issuance_contract(
        &e,
        &Option::None,
        &recipients,
        &organization,
        &credential_params,
    );

    let mut recipients = issuance_contract.recipients();
    let cert_data = recipients.get(recipient1_did.clone()).unwrap().unwrap();
    assert_eq!(cert_data, Option::None);

    let verifiable_credential = DistributeCredential {
        did: "did:chaincerts:abc123#credential-xyz123".into_val(&e),
        id: "c8b875a2-3f5d-4a63-b1c8-791be9b01c02".into_val(&e),
        recipient_did: recipient1_did,
        signature: String::from_slice(&e, "MEUCIFZ5o9zSYiC9d0hvN6V73Y8yBm9n3MF8Hj"),
        attestation: ATTESTATION1.into_val(&e),
        issuance_date: 1679918400,
        expiration_date: OptionU64::Some(31556926),
    };

    issuance_contract.distribute(
        &organization.admin,
        &wallet.contract_id,
        &verifiable_credential,
    );

    recipients = issuance_contract.recipients();

    assert_eq!(issuance_contract.supply(), 1);
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
    let wallet = create_did_contract(&e, &recipient_address, &organization.did);

    pub const ATTESTATION1: &str = "ipfs://QmdtyfTYbVS3K9iYqBPjXxn4mbB7aBvEjYGzYWnzRcMrEC";

    let credential_params = CredentialParams {
        file_storage: "FileBase".into_val(&e),
        revocable: true,
        credential_type: String::from_slice(&e, "Work"),
        credential_title: String::from_slice(&e, "Software Engineer"),
    };

    let issuance_contract = create_issuance_contract(
        &e,
        &Option::None,
        &recipients,
        &organization,
        &credential_params,
    );

    let verifiable_credential = DistributeCredential {
        did: "did:chaincerts:abc123#credential-xyz123".into_val(&e),
        id: "c8b875a2-3f5d-4a63-b1c8-791be9b01c02".into_val(&e),
        recipient_did,
        signature: String::from_slice(&e, "MEUCIFZ5o9zSYiC9d0hvN6V73Y8yBm9n3MF8Hj"),
        attestation: ATTESTATION1.into_val(&e),
        issuance_date: 1679918400,
        expiration_date: OptionU64::None,
    };

    let revocation_date: u64 = 1684875611;

    issuance_contract.distribute(
        &organization.admin,
        &wallet.contract_id,
        &verifiable_credential,
    );

    let recipients = issuance_contract.recipients();
    let cert_data = recipients
        .get(verifiable_credential.recipient_did.clone())
        .unwrap()
        .unwrap();
    assert!(cert_data.is_some());

    issuance_contract.revoke(
        &organization.admin,
        &verifiable_credential.recipient_did,
        &revocation_date,
    );

    let revoked_credentials = issuance_contract.revoked_credentials(&organization.admin);

    let credential_data = CredentialData {
        did: verifiable_credential.did,
        recipient_did: verifiable_credential.recipient_did,
        credential_type: credential_params.credential_type,
        credential_title: credential_params.credential_title,
        issuance_date: verifiable_credential.issuance_date,
        expiration_date: OptionU64::None,
        signature: verifiable_credential.signature,
    };

    let revoked_credential = RevokedCredential {
        credential_data,
        revocation_date,
    };

    assert_eq!(revoked_credentials.len(), 1);
    assert_eq!(
        revoked_credentials.get_unchecked(0).unwrap(),
        revoked_credential
    );
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
        credential_type: String::from_slice(&e, "Work"),
        credential_title: String::from_slice(&e, "Software Engineer"),
    };

    let issuance_contract: IssuanceContractClient = create_issuance_contract(
        &e,
        &Option::None,
        &recipient_dids,
        &organization,
        &credential_params,
    );

    issuance_contract.initialize(
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
        credential_type: String::from_slice(&e, "Work"),
        credential_title: String::from_slice(&e, "Software Engineer"),
    };

    let distribution_limit: Option<u32> = Option::Some(6);
    let issuance_contract = create_issuance_contract(
        &e,
        &distribution_limit,
        &Option::None,
        &organization,
        &credential_params,
    );

    issuance_contract.initialize(
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
    let wallet = create_did_contract(&e, &recipient_address, &organization.did);

    let credential_params = CredentialParams {
        file_storage: "FileBase".into_val(&e),
        revocable: true,
        credential_type: String::from_slice(&e, "Work"),
        credential_title: String::from_slice(&e, "Software Engineer"),
    };

    let issuance_contract = create_issuance_contract(
        &e,
        &Option::None,
        &recipient_dids,
        &organization,
        &credential_params,
    );

    let verifiable_credential = DistributeCredential {
        did: "did:chaincerts:abc123#credential-xyz123".into_val(&e),
        id: "c8b875a2-3f5d-4a63-b1c8-791be9b01c02".into_val(&e),
        recipient_did,
        signature: String::from_slice(&e, "MEUCIFZ5o9zSYiC9d0hvN6V73Y8yBm9n3MF8Hj"),
        attestation: ATTESTATION1.into_val(&e),
        issuance_date: 1679918400,
        expiration_date: OptionU64::None,
    };

    issuance_contract.distribute(&invalid_admin, &wallet.contract_id, &verifiable_credential);
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
    let wallet1 = create_did_contract(&e, &address1, &organization.did);
    let wallet2 = create_did_contract(&e, &address2, &organization.did);

    let credential_params = CredentialParams {
        file_storage: "FileBase".into_val(&e),
        revocable: true,
        credential_type: String::from_slice(&e, "Work"),
        credential_title: String::from_slice(&e, "Software Engineer"),
    };

    let distribution_limit = Option::Some(1);
    let issuance_contract = create_issuance_contract(
        &e,
        &distribution_limit,
        &Option::None,
        &organization,
        &credential_params,
    );
    const ATTESTATION1: &str = "ipfs://QmdtyfTYbVS3K9iYqBPjXxn4mbB7aBvEjYGzYWnzRcMrEC";
    const ATTESTATION2: &str = "ipfs://QmdtyfTYbVS3K9iYqBPjXxn4mbB7aBvEjYGzYWnzRcMrED";

    let mut verifiable_credential = DistributeCredential {
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
        expiration_date: OptionU64::None,
    };

    issuance_contract.distribute(
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
    issuance_contract.distribute(
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
    let wallet = create_did_contract(&e, &wallet_owner, &organization.did);

    let credential_params = CredentialParams {
        file_storage: "FileBase".into_val(&e),
        revocable: true,
        credential_type: String::from_slice(&e, "Work"),
        credential_title: String::from_slice(&e, "Software Engineer"),
    };

    let distribution_limit = Option::Some(3);
    let issuance_contract = create_issuance_contract(
        &e,
        &distribution_limit,
        &Option::None,
        &organization,
        &credential_params,
    );
    pub const ATTESTATION1: &str = "ipfs://QmdtyfTYbVS3K9iYqBPjXxn4mbB7aBvEjYGzYWnzRcMrEC";

    let verifiable_credential = DistributeCredential {
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
        expiration_date: OptionU64::None,
    };

    issuance_contract.distribute(
        &organization.admin,
        &wallet.contract_id,
        &verifiable_credential,
    );

    issuance_contract.distribute(
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
    let wallet = create_did_contract(&e, &recipient_address, &organization.did);

    pub const ATTESTATION1: &str = "ipfs://QmdtyfTYbVS3K9iYqBPjXxn4mbB7aBvEjYGzYWnzRcMrEC";

    let credential_params = CredentialParams {
        file_storage: "FileBase".into_val(&e),
        revocable: true,
        credential_type: String::from_slice(&e, "Work"),
        credential_title: String::from_slice(&e, "Software Engineer"),
    };

    let issuance_contract = create_issuance_contract(
        &e,
        &Option::None,
        &recipients,
        &organization,
        &credential_params,
    );

    let credential = DistributeCredential {
        did: "did:chaincerts:abc123#credential-xyz123".into_val(&e),
        id: "c8b875a2-3f5d-4a63-b1c8-791be9b01c02".into_val(&e),
        recipient_did,
        signature: String::from_slice(&e, "MEUCIFZ5o9zSYiC9d0hvN6V73Y8yBm9n3MF8Hj"),
        attestation: ATTESTATION1.into_val(&e),
        issuance_date: 1679918400,
        expiration_date: OptionU64::None,
    };

    let revocation_date: u64 = 1684875611;

    issuance_contract.distribute(&organization.admin, &wallet.contract_id, &credential);

    issuance_contract.revoke(
        &Address::random(&e),
        &credential.recipient_did,
        &revocation_date,
    );
}

#[test]
#[should_panic(expected = "Status(ContractError(7))")]
fn test_revoke_credential_data_none_error() {
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

    let credential_params = CredentialParams {
        file_storage: "FileBase".into_val(&e),
        revocable: true,
        credential_type: String::from_slice(&e, "Work"),
        credential_title: String::from_slice(&e, "Software Engineer"),
    };

    let revocation_date: u64 = 1684875611;

    let issuance_contract = create_issuance_contract(
        &e,
        &Option::None,
        &recipients,
        &organization,
        &credential_params,
    );

    issuance_contract.revoke(&organization.admin, &recipient_did, &revocation_date);
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
    let wallet = create_did_contract(&e, &recipient_address, &organization.did);
    const ATTESTATION1: &str = "ipfs://QmdtyfTYbVS3K9iYqBPjXxn4mbB7aBvEjYGzYWnzRcMrEC";

    let credential_params = CredentialParams {
        file_storage: "FileBase".into_val(&e),
        revocable: true,
        credential_type: String::from_slice(&e, "Work"),
        credential_title: String::from_slice(&e, "Software Engineer"),
    };

    let issuance_contract = create_issuance_contract(
        &e,
        &Option::None,
        &recipients,
        &organization,
        &credential_params,
    );

    let verifiable_credential = DistributeCredential {
        did: "did:chaincerts:abc123#credential-xyz123".into_val(&e),
        id: "c8b875a2-3f5d-4a63-b1c8-791be9b01c02".into_val(&e),
        recipient_did,
        signature: String::from_slice(&e, "MEUCIFZ5o9zSYiC9d0hvN6V73Y8yBm9n3MF8Hj"),
        attestation: ATTESTATION1.into_val(&e),
        issuance_date: 1679918400,
        expiration_date: OptionU64::None,
    };

    let revocation_date: u64 = 1684875611;

    issuance_contract.distribute(
        &organization.admin,
        &wallet.contract_id,
        &verifiable_credential,
    );

    issuance_contract.revoke(
        &organization.admin,
        &verifiable_credential.recipient_did,
        &revocation_date,
    );
    issuance_contract.revoke(
        &organization.admin,
        &verifiable_credential.recipient_did,
        &revocation_date,
    );
}

#[test]
#[should_panic(expected = "Status(ContractError(7))")]
fn test_revoke_no_revocable_cert() {
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

    let credential_params = CredentialParams {
        file_storage: "FileBase".into_val(&e),
        revocable: false,
        credential_type: String::from_slice(&e, "Work"),
        credential_title: String::from_slice(&e, "Software Engineer"),
    };

    let revocation_date: u64 = 1684875611;

    let issuance_contract = create_issuance_contract(
        &e,
        &Option::None,
        &recipients,
        &organization,
        &credential_params,
    );

    issuance_contract.revoke(&organization.admin, &recipient_did, &revocation_date);
}

#[test]
fn test_attest_valid() {
    let (e, organization, credential, issuance_contract) =
        setup_initialized_and_distributed_contract();

    let credential_status = CredentialStatus {
        status: String::from_slice(&e, "valid"),
        expiration_date: credential.expiration_date,
        revocation_date: OptionU64::None,
    };

    let attest = issuance_contract.attest(
        &credential.did,
        &organization.did,
        &credential.recipient_did,
        &credential.signature,
    );

    assert_eq!(attest, credential_status)
}

#[test]
fn test_attest_revoked() {
    let (e, organization, credential, issuance_contract) =
        setup_initialized_and_distributed_contract();

    let revocation_date: u64 = 1684875611;
    let credential_status = CredentialStatus {
        status: String::from_slice(&e, "revoked"),
        expiration_date: credential.expiration_date,
        revocation_date: OptionU64::Some(revocation_date),
    };

    issuance_contract.revoke(
        &organization.admin,
        &credential.recipient_did,
        &revocation_date,
    );

    let attest = issuance_contract.attest(
        &credential.did,
        &organization.did,
        &credential.recipient_did,
        &credential.signature,
    );

    assert_eq!(attest, credential_status)
}

#[test]
fn test_attest_with_invalid_credential() {
    let (e, organization, verifiable_credential, issuance_contract) =
        setup_initialized_and_distributed_contract();

    let credential_status = CredentialStatus {
        status: String::from_slice(&e, "invalid"),
        expiration_date: OptionU64::None,
        revocation_date: OptionU64::None,
    };

    let attest = issuance_contract.attest(
        &"did:chaincerts:abc123#credential-invalid".into_val(&e),
        &organization.did,
        &verifiable_credential.recipient_did,
        &verifiable_credential.signature,
    );

    assert_eq!(attest, credential_status)
}

#[test]
fn test_attest_with_invalid_issuer() {
    let (e, _organization, verifiable_credential, issuance_contract) =
        setup_initialized_and_distributed_contract();

    let credential_status = CredentialStatus {
        status: String::from_slice(&e, "invalid"),
        expiration_date: OptionU64::None,
        revocation_date: OptionU64::None,
    };

    let attest = issuance_contract.attest(
        &verifiable_credential.did,
        &"did:chaincerts:invalid-org-123".into_val(&e),
        &verifiable_credential.recipient_did,
        &verifiable_credential.signature,
    );

    assert_eq!(attest, credential_status)
}

#[test]
fn test_attest_with_invalid_recipient() {
    let (e, organization, verifiable_credential, issuance_contract) =
        setup_initialized_and_distributed_contract();

    let credential_status = CredentialStatus {
        status: String::from_slice(&e, "invalid"),
        expiration_date: OptionU64::None,
        revocation_date: OptionU64::None,
    };

    let attest = issuance_contract.attest(
        &verifiable_credential.did,
        &organization.did,
        &"did:chaincerts:invalid-recipient-123".into_val(&e),
        &verifiable_credential.signature,
    );

    assert_eq!(attest, credential_status)
}

#[test]
fn test_attest_with_invalid_signature() {
    let (e, organization, verifiable_credential, issuance_contract) =
        setup_initialized_and_distributed_contract();

    let credential_status = CredentialStatus {
        status: String::from_slice(&e, "invalid"),
        expiration_date: OptionU64::None,
        revocation_date: OptionU64::None,
    };

    let attest = issuance_contract.attest(
        &verifiable_credential.did,
        &organization.did,
        &verifiable_credential.recipient_did,
        &String::from_slice(&e, "Invalid signature"),
    );

    assert_eq!(attest, credential_status)
}
