#![cfg(test)]
use crate::certs_wallet::{self, OptionU64};
use crate::storage_types::{CertData, Info, Organization, Status};
use crate::{contract::CertIssuance, CertIssuanceClient};
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{vec, Address, Bytes, Env, IntoVal, Vec};

const WASM: &[u8] = include_bytes!("../../target/wasm32-unknown-unknown/release/certs_wallet.wasm");

fn create_wallet_contract(env: &Env, owner: &Address, id: &Bytes) -> certs_wallet::Client {
    let wallet = certs_wallet::Client::new(env, &env.register_contract_wasm(None, WASM));
    wallet.initialize(owner);
    wallet.add_organization(id);
    wallet
}

fn create_cert_issuance_contract_with_limit(
    e: &Env,
    limit: &Option<u32>,
    address_recipients: &Option<Vec<Address>>,
    organization: &Organization,
    administration_rules: &(bool, OptionU64),
) -> CertIssuanceClient {
    let cert_issuance =
        CertIssuanceClient::new(e, &e.register_contract(None, CertIssuance {}));
    cert_issuance.initialize(
        &("FileBase").into_val(e),
        &"ChaincertName".into_val(e),
        address_recipients,
        limit,
        administration_rules,
        organization,
    );
    cert_issuance
}

fn create_cert_issuance_contract_with_recipients(
    e: &Env,
    limit: &Option<u32>,
    address_recipients: &Option<Vec<Address>>,
    organization: &Organization,
    administration_rules: &(bool, OptionU64),
) -> CertIssuanceClient {
    let cert_issuance =
        CertIssuanceClient::new(e, &e.register_contract(None, CertIssuance {}));

    cert_issuance.initialize(
        &"FileBase".into_val(e),
        &"ChaincertName".into_val(e),
        address_recipients,
        limit,
        administration_rules,
        organization,
    );
    cert_issuance
}

fn create_random_recipients_address(e: &Env) -> Vec<Address> {
    let recipient_1 = Address::random(e);
    let recipient_2 = Address::random(e);
    let recipient_3 = Address::random(e);
    vec![e, recipient_1, recipient_2, recipient_3]
}

#[test]
fn test_create_cert_data() {
    let e: Env = Default::default();
    let id_chaincert: Bytes = "12345".into_val(&e);
    let status = Status::Unassigned;
    let distribution_date = OptionU64::Some(1711195200);
    let cert_data = CertData::new(
        id_chaincert.clone(),
        status.clone(),
        distribution_date.clone(),
    );
    assert_eq!(cert_data.id, id_chaincert);
    assert_eq!(cert_data.status, status);
    assert_eq!(cert_data.distribution_date, distribution_date);
}

#[test]
fn test_initialize_contract_with_recipients() {
    let e: Env = Default::default();
    let recipients: Option<Vec<Address>> = Option::Some(create_random_recipients_address(&e));
    let organization: Organization = Organization {
        admin: Address::random(&e),
        id: "12345".into_val(&e),
    };
    let administration_rules = (true, OptionU64::Some(1680091200));

    let cert_issuance: CertIssuanceClient = create_cert_issuance_contract_with_recipients(
        &e,
        &Option::None,
        &recipients,
        &organization,
        &administration_rules,
    );
    assert_eq!(cert_issuance.file_storage(), "FileBase".into_val(&e));
    assert_eq!(cert_issuance.name(), "ChaincertName".into_val(&e));
    assert!(cert_issuance.is_revocable());
    assert_eq!(
        cert_issuance.expiration_time(),
        OptionU64::Some(1680091200)
    );
    assert_eq!(cert_issuance.distribution_limit(), 3);
    assert_eq!(cert_issuance.supply(), 0);
    assert_eq!(cert_issuance.recipients().len(), 3);
}

#[test]
fn test_initialize_with_limit_contract() {
    let e: Env = Default::default();
    let organization: Organization = Organization {
        admin: Address::random(&e),
        id: "12345".into_val(&e),
    };
    let distribution_limit: Option<u32> = Option::Some(6);
    let administration_rules = (true, OptionU64::None);

    let cert_issuance = create_cert_issuance_contract_with_limit(
        &e,
        &distribution_limit,
        &Option::None,
        &organization,
        &administration_rules,
    );
    assert_eq!(cert_issuance.file_storage(), "FileBase".into_val(&e));
    assert_eq!(cert_issuance.name(), "ChaincertName".into_val(&e));
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
        id: "12345".into_val(&e),
    };

    let administration_rules = (true, OptionU64::None);

    let cert_issuance = create_cert_issuance_contract_with_limit(
        &e,
        &Option::None,
        &Option::None,
        &organization,
        &administration_rules,
    );
    assert_eq!(cert_issuance.file_storage(), "FileBase".into_val(&e));
    assert_eq!(cert_issuance.name(), "ChaincertName".into_val(&e));
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
        id: "12345".into_val(&e),
    };
    let administration_rules_without_expiration_time = (true, OptionU64::None);

    let administration_rules_with_expiration_time = (true, OptionU64::Some(31556926));

    let distribution_limit: Option<u32> = Option::Some(6);

    let cert_issuance = create_cert_issuance_contract_with_limit(
        &e,
        &distribution_limit,
        &Option::None,
        &organization,
        &administration_rules_without_expiration_time,
    );

    let cert_issuance_2 = create_cert_issuance_contract_with_limit(
        &e,
        &distribution_limit,
        &Option::None,
        &organization,
        &administration_rules_with_expiration_time,
    );

    let info = Info {
        name: "ChaincertName".into_val(&e),
        revocable: true,
        expiration_time: OptionU64::None,
        distribution_limit: 6,
        supply: 0,
    };

    let info_2 = Info {
        name: "ChaincertName".into_val(&e),
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
    let address_recipient_1 = Address::random(&e);
    let wallet = create_wallet_contract(&e, &address_recipient_1, &"12345".into_val(&e));
    let organization: Organization = Organization {
        admin: Address::random(&e),
        id: "12345".into_val(&e),
    };
    let distribution_limit: Option<u32> = Option::Some(6);
    let administration_rules = (true, OptionU64::None);
    let cert_issuance = create_cert_issuance_contract_with_limit(
        &e,
        &distribution_limit,
        &Option::None,
        &organization,
        &administration_rules,
    );

    pub const CID1: &str = "QmdtyfTYbVS3K9iYqBPjXxn4mbB7aBvEjYGzYWnzRcMrEC";

    let distribution_date: u64 = 1679918400;
    cert_issuance.distribute(
        &organization.admin,
        &address_recipient_1,
        &wallet.contract_id,
        &CID1.into_val(&e),
        &distribution_date,
    );
    let recipients: soroban_sdk::Map<Address, CertData> = cert_issuance.recipients();
    let cert_data = recipients.get(address_recipient_1).unwrap().unwrap();

    assert_eq!(cert_data.status, Status::Distribute);
    assert_eq!(cert_issuance.supply(), 1);
    assert_eq!(recipients.len(), 1);
    assert_eq!(wallet.get_chaincerts().len(), 1);
}

#[test]
fn test_distribute_with_initial_recipients() {
    let e: Env = Default::default();
    let recipients = Option::Some(create_random_recipients_address(&e));
    let address_recipient_1 = recipients
        .clone()
        .expect("Vec of recipients")
        .get(0)
        .unwrap()
        .unwrap();
    let wallet = create_wallet_contract(&e, &address_recipient_1, &"12345".into_val(&e));
    let organization: Organization = Organization {
        admin: Address::random(&e),
        id: "12345".into_val(&e),
    };
    let distribution_date: u64 = 1679918400;
    pub const CID1: &str = "QmdtyfTYbVS3K9iYqBPjXxn4mbB7aBvEjYGzYWnzRcMrEC";
    let administration_rules = (true, OptionU64::Some(31556926));
    let cert_issuance = create_cert_issuance_contract_with_recipients(
        &e,
        &Option::None,
        &recipients,
        &organization,
        &administration_rules,
    );

    let mut recipients = cert_issuance.recipients();
    let mut cert_data = recipients.get(address_recipient_1.clone()).unwrap().unwrap();
    assert_eq!(cert_data.status, Status::Unassigned);

    cert_issuance.distribute(
        &organization.admin,
        &address_recipient_1,
        &wallet.contract_id,
        &CID1.into_val(&e),
        &distribution_date,
    );

    recipients = cert_issuance.recipients();
    cert_data = recipients.get(address_recipient_1).unwrap().unwrap();

    assert_eq!(cert_data.status, Status::Distribute);
    assert_eq!(cert_issuance.supply(), 1);
    assert_eq!(recipients.len(), 3);
    assert_eq!(wallet.get_chaincerts().len(), 1);
}

#[test]
fn test_revoke_chaincert() {
    let e: Env = Default::default();
    let recipients: Option<Vec<Address>> = Option::Some(create_random_recipients_address(&e));
    let recipient_address = recipients
        .clone()
        .expect("Vec of recipients")
        .get(0)
        .unwrap()
        .unwrap();
    let wallet = create_wallet_contract(&e, &recipient_address, &"12345".into_val(&e));
    let organization: Organization = Organization {
        admin: Address::random(&e),
        id: "12345".into_val(&e),
    };

    let administration_rules = (true, OptionU64::None);

    let distribution_date: u64 = 1679918400;
    pub const CID1: &str = "QmdtyfTYbVS3K9iYqBPjXxn4mbB7aBvEjYGzYWnzRcMrEC";

    let cert_issuance = create_cert_issuance_contract_with_recipients(
        &e,
        &Option::None,
        &recipients,
        &organization,
        &administration_rules,
    );

    cert_issuance.distribute(
        &organization.admin,
        &recipient_address,
        &wallet.contract_id,
        &CID1.into_val(&e),
        &distribution_date,
    );

    let mut recipients = cert_issuance.recipients();
    let mut cert_data = recipients.get(recipient_address.clone()).unwrap().unwrap();
    assert_eq!(cert_data.status, Status::Distribute);

    cert_issuance.revoke(&organization.admin, &recipient_address, &wallet.contract_id);

    recipients = cert_issuance.recipients();
    cert_data = recipients.get(recipient_address).unwrap().unwrap();
    assert_eq!(cert_data.status, Status::Revoked);

    let chaincert = wallet.get_chaincerts().get(0).unwrap().unwrap();
    assert!(chaincert.revoked);
}

#[test]
#[should_panic(expected = "Status(ContractError(1))")]
fn test_initialize_contract_with_recipients_error() {
    let e: Env = Default::default();
    let recipients: Option<Vec<Address>> = Option::Some(create_random_recipients_address(&e));
    let organization: Organization = Organization {
        admin: Address::random(&e),
        id: "12345".into_val(&e),
    };

    let administration_rules = (true, OptionU64::None);

    let cert_issuance: CertIssuanceClient = create_cert_issuance_contract_with_recipients(
        &e,
        &Option::None,
        &recipients,
        &organization,
        &administration_rules,
    );

    cert_issuance.initialize(
        &"FileBase".into_val(&e),
        &"ChaincertName".into_val(&e),
        &recipients,
        &Option::None,
        &administration_rules,
        &organization,
    )
}

#[test]
#[should_panic(expected = "Status(ContractError(1))")]
fn test_initialize_with_limit_contract_error() {
    let e: Env = Default::default();
    let organization: Organization = Organization {
        admin: Address::random(&e),
        id: "12345".into_val(&e),
    };

    let administration_rules = (true, OptionU64::None);

    let distribution_limit: Option<u32> = Option::Some(6);
    let cert_issuance = create_cert_issuance_contract_with_limit(
        &e,
        &distribution_limit,
        &Option::None,
        &organization,
        &administration_rules,
    );

    cert_issuance.initialize(
        &"FileBase".into_val(&e),
        &"ChaincertName".into_val(&e),
        &Option::None,
        &distribution_limit,
        &administration_rules,
        &organization,
    )
}

#[test]
#[should_panic(expected = "Status(ContractError(2))")]
fn test_distribute_admin_error() {
    pub const CID1: &str = "QmdtyfTYbVS3K9iYqBPjXxn4mbB7aBvEjYGzYWnzRcMrEC";
    let e: Env = Default::default();
    let admin = Address::random(&e);
    let recipients = Option::Some(create_random_recipients_address(&e));
    let recipient_address = recipients
        .clone()
        .expect("Vec of recipients")
        .get(0)
        .unwrap()
        .unwrap();
    let wallet = create_wallet_contract(&e, &recipient_address, &"12345".into_val(&e));
    let organization: Organization = Organization {
        admin: Address::random(&e),
        id: "12345".into_val(&e),
    };

    let administration_rules = (true, OptionU64::None);

    let distribution_date: u64 = 1679918400;
    let cert_issuance = create_cert_issuance_contract_with_recipients(
        &e,
        &Option::None,
        &recipients,
        &organization,
        &administration_rules,
    );

    cert_issuance.distribute(
        &admin,
        &recipient_address,
        &wallet.contract_id,
        &CID1.into_val(&e),
        &distribution_date,
    );
}

#[test]
#[should_panic(expected = "Status(ContractError(3))")]
fn test_distribute_limit_error() {
    let e: Env = Default::default();
    let recipients = Option::Some(create_random_recipients_address(&e));
    let wallet1 = create_wallet_contract(
        &e,
        &recipients
            .clone()
            .expect("Vec of recipients")
            .get(0)
            .unwrap()
            .unwrap(),
        &"12345".into_val(&e),
    );
    let wallet2 = create_wallet_contract(
        &e,
        &recipients
            .clone()
            .expect("Vec of recipients")
            .get(1)
            .unwrap()
            .unwrap(),
        &"12345".into_val(&e),
    );

    let organization: Organization = Organization {
        admin: Address::random(&e),
        id: "12345".into_val(&e),
    };

    let administration_rules = (true, OptionU64::None);

    let distribution_date: u64 = 1679918400;
    let distribution_limit = Option::Some(1);
    let cert_issuance = create_cert_issuance_contract_with_limit(
        &e,
        &distribution_limit,
        &Option::None,
        &organization,
        &administration_rules,
    );
    pub const CID1: &str = "QmdtyfTYbVS3K9iYqBPjXxn4mbB7aBvEjYGzYWnzRcMrEC";
    pub const CID2: &str = "QmdtyfTYbVS3K9iYqBPjXxn4mbB7aBvEjYGzYWnzRcMrED";

    cert_issuance.distribute(
        &organization.admin,
        &recipients
            .clone()
            .expect("Vec of recipients")
            .get(0)
            .unwrap()
            .unwrap(),
        &wallet1.contract_id,
        &CID1.into_val(&e),
        &distribution_date,
    );

    cert_issuance.distribute(
        &organization.admin,
        &recipients
            .expect("Vec of recipients")
            .get(1)
            .unwrap()
            .unwrap(),
        &wallet2.contract_id,
        &CID2.into_val(&e),
        &distribution_date,
    );
}

#[test]
#[should_panic(expected = "Status(ContractError(5))")]
fn test_distribute_status_error() {
    let e: Env = Default::default();
    let recipients = Option::Some(create_random_recipients_address(&e));
    let wallet = create_wallet_contract(
        &e,
        &recipients
            .clone()
            .expect("Vec of recipients")
            .get(0)
            .unwrap()
            .unwrap(),
        &"12345".into_val(&e),
    );
    let organization: Organization = Organization {
        admin: Address::random(&e),
        id: "12345".into_val(&e),
    };

    let administration_rules = (true, OptionU64::None);

    let distribution_date: u64 = 1679918400;
    let distribution_limit = Option::Some(3);
    let cert_issuance = create_cert_issuance_contract_with_limit(
        &e,
        &distribution_limit,
        &Option::None,
        &organization,
        &administration_rules,
    );
    pub const CID1: &str = "QmdtyfTYbVS3K9iYqBPjXxn4mbB7aBvEjYGzYWnzRcMrEC";

    cert_issuance.distribute(
        &organization.admin,
        &recipients
            .clone()
            .expect("Vec of recipients")
            .get(0)
            .unwrap()
            .unwrap(),
        &wallet.contract_id,
        &CID1.into_val(&e),
        &distribution_date,
    );

    cert_issuance.distribute(
        &organization.admin,
        &recipients
            .expect("Vec of recipients")
            .get(0)
            .unwrap()
            .unwrap(),
        &wallet.contract_id,
        &CID1.into_val(&e),
        &distribution_date,
    );
}

#[test]
#[should_panic(expected = "Status(ContractError(2))")]
fn test_revoke_admin_error() {
    let e: Env = Default::default();
    let recipients: Option<Vec<Address>> = Option::Some(create_random_recipients_address(&e));
    let recipient_address = recipients
        .clone()
        .expect("Vec of recipients")
        .get(0)
        .unwrap()
        .unwrap();
    let wallet = create_wallet_contract(&e, &recipient_address, &"12345".into_val(&e));
    let organization: Organization = Organization {
        admin: Address::random(&e),
        id: "12345".into_val(&e),
    };

    let administration_rules = (true, OptionU64::None);

    let distribution_date: u64 = 1679918400;
    pub const CID1: &str = "QmdtyfTYbVS3K9iYqBPjXxn4mbB7aBvEjYGzYWnzRcMrEC";
    let cert_issuance = create_cert_issuance_contract_with_recipients(
        &e,
        &Option::None,
        &recipients,
        &organization,
        &administration_rules,
    );

    cert_issuance.distribute(
        &organization.admin,
        &recipient_address,
        &wallet.contract_id,
        &CID1.into_val(&e),
        &distribution_date,
    );

    cert_issuance.revoke(&Address::random(&e), &recipient_address, &wallet.contract_id);
}

#[test]
#[should_panic(expected = "Status(ContractError(7))")]
fn test_revoke_status_unassigned_error() {
    let e: Env = Default::default();
    let recipients: Option<Vec<Address>> = Option::Some(create_random_recipients_address(&e));
    let recipient_address = recipients
        .clone()
        .expect("Vec of recipients")
        .get(0)
        .unwrap()
        .unwrap();
    let wallet = create_wallet_contract(&e, &recipient_address, &"12345".into_val(&e));
    let organization: Organization = Organization {
        admin: Address::random(&e),
        id: "12345".into_val(&e),
    };

    let administration_rules = (true, OptionU64::None);

    let cert_issuance = create_cert_issuance_contract_with_recipients(
        &e,
        &Option::None,
        &recipients,
        &organization,
        &administration_rules,
    );

    cert_issuance.revoke(&organization.admin, &recipient_address, &wallet.contract_id);
}

#[test]
#[should_panic(expected = "Status(ContractError(7))")]
fn test_revoke_status_revoked_error() {
    let e: Env = Default::default();
    let recipients: Option<Vec<Address>> = Option::Some(create_random_recipients_address(&e));
    let recipient_address = recipients
        .clone()
        .expect("Vec of recipients")
        .get(0)
        .unwrap()
        .unwrap();
    let wallet = create_wallet_contract(&e, &recipient_address, &"12345".into_val(&e));
    let organization: Organization = Organization {
        admin: Address::random(&e),
        id: "12345".into_val(&e),
    };

    let administration_rules = (true, OptionU64::None);

    let distribution_date: u64 = 1679918400;
    pub const CID1: &str = "QmdtyfTYbVS3K9iYqBPjXxn4mbB7aBvEjYGzYWnzRcMrEC";
    let cert_issuance = create_cert_issuance_contract_with_recipients(
        &e,
        &Option::None,
        &recipients,
        &organization,
        &administration_rules,
    );
    cert_issuance.distribute(
        &organization.admin,
        &recipient_address,
        &wallet.contract_id,
        &CID1.into_val(&e),
        &distribution_date,
    );

    cert_issuance.revoke(&organization.admin, &recipient_address, &wallet.contract_id);
    cert_issuance.revoke(&organization.admin, &recipient_address, &wallet.contract_id);
}

#[test]
#[should_panic(expected = "Status(ContractError(7))")]
fn test_revoke_no_revocable_cert() {
    let e: Env = Default::default();
    let recipients: Option<Vec<Address>> = Option::Some(create_random_recipients_address(&e));
    let recipient_address = recipients
        .clone()
        .expect("Vec of recipients")
        .get(0)
        .unwrap()
        .unwrap();
    let wallet = create_wallet_contract(&e, &recipient_address, &"12345".into_val(&e));
    let organization: Organization = Organization {
        admin: Address::random(&e),
        id: "12345".into_val(&e),
    };

    let administration_rules = (false, OptionU64::None);

    let cert_issuance = create_cert_issuance_contract_with_recipients(
        &e,
        &Option::None,
        &recipients,
        &organization,
        &administration_rules,
    );

    let recipient_address = recipients
        .expect("Vec of recipients")
        .get(0)
        .unwrap()
        .unwrap();

    cert_issuance.revoke(&organization.admin, &recipient_address, &wallet.contract_id);
}
