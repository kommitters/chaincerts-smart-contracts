#![cfg(test)]
use crate::certs_wallet::{self, OptionU64};
use crate::storage_types::{CertData, Info, Organization, Status};
use crate::{contract::CertGovernance, CertGovernanceClient};
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{vec, Address, Bytes, Env, IntoVal, Vec};

const WASM: &[u8] = include_bytes!("../../target/wasm32-unknown-unknown/release/certs_wallet.wasm");

fn create_wallet_contract(env: &Env, owner: &Address, id: &Bytes) -> certs_wallet::Client {
    let wallet = certs_wallet::Client::new(env, &env.register_contract_wasm(None, WASM));
    wallet.initialize(owner);
    wallet.add_organization(id);
    wallet
}

fn create_cert_governance_contract_with_limit(
    e: &Env,
    limit: &Option<u32>,
    address_receivers: &Option<Vec<Address>>,
    organization: &Organization,
    governance_rules: &(bool, OptionU64),
) -> CertGovernanceClient {
    let cert_governance =
        CertGovernanceClient::new(e, &e.register_contract(None, CertGovernance {}));
    cert_governance.initialize(
        &("FileBase").into_val(e),
        &"ChaincertName".into_val(e),
        address_receivers,
        limit,
        governance_rules,
        organization,
    );
    cert_governance
}

fn create_cert_governance_contract_with_receivers(
    e: &Env,
    limit: &Option<u32>,
    address_receivers: &Option<Vec<Address>>,
    organization: &Organization,
    governance_rules: &(bool, OptionU64),
) -> CertGovernanceClient {
    let cert_governance =
        CertGovernanceClient::new(e, &e.register_contract(None, CertGovernance {}));

    cert_governance.initialize(
        &"FileBase".into_val(e),
        &"ChaincertName".into_val(e),
        address_receivers,
        limit,
        governance_rules,
        organization,
    );
    cert_governance
}

fn create_random_receivers_address(e: &Env) -> Vec<Address> {
    let receiver_1 = Address::random(e);
    let receiver_2 = Address::random(e);
    let receiver_3 = Address::random(e);
    vec![e, receiver_1, receiver_2, receiver_3]
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
fn test_initialize_contract_with_receivers() {
    let e: Env = Default::default();
    let receivers: Option<Vec<Address>> = Option::Some(create_random_receivers_address(&e));
    let organization: Organization = Organization {
        admin: Address::random(&e),
        id: "12345".into_val(&e),
    };
    let governance_rules = (true, OptionU64::Some(1680091200));

    let cert_governance: CertGovernanceClient = create_cert_governance_contract_with_receivers(
        &e,
        &Option::None,
        &receivers,
        &organization,
        &governance_rules,
    );
    assert_eq!(cert_governance.file_storage(), "FileBase".into_val(&e));
    assert_eq!(cert_governance.name(), "ChaincertName".into_val(&e));
    assert!(cert_governance.is_revocable());
    assert_eq!(
        cert_governance.expiration_time(),
        OptionU64::Some(1680091200)
    );
    assert_eq!(cert_governance.distribution_limit(), 3);
    assert_eq!(cert_governance.supply(), 0);
    assert_eq!(cert_governance.receivers().len(), 3);
}

#[test]
fn test_initialize_with_limit_contract() {
    let e: Env = Default::default();
    let organization: Organization = Organization {
        admin: Address::random(&e),
        id: "12345".into_val(&e),
    };
    let distribution_limit: Option<u32> = Option::Some(6);
    let governance_rules = (true, OptionU64::None);

    let cert_governance = create_cert_governance_contract_with_limit(
        &e,
        &distribution_limit,
        &Option::None,
        &organization,
        &governance_rules,
    );
    assert_eq!(cert_governance.file_storage(), "FileBase".into_val(&e));
    assert_eq!(cert_governance.name(), "ChaincertName".into_val(&e));
    assert!(cert_governance.is_revocable());
    assert_eq!(cert_governance.expiration_time(), OptionU64::None);
    assert_eq!(cert_governance.distribution_limit(), 6);
    assert_eq!(cert_governance.supply(), 0);
    assert_eq!(cert_governance.receivers().len(), 0);
}

#[test]
fn test_initialize_without_limit_contract_and_receivers() {
    let e: Env = Default::default();
    let organization: Organization = Organization {
        admin: Address::random(&e),
        id: "12345".into_val(&e),
    };

    let governance_rules = (true, OptionU64::None);

    let cert_governance = create_cert_governance_contract_with_limit(
        &e,
        &Option::None,
        &Option::None,
        &organization,
        &governance_rules,
    );
    assert_eq!(cert_governance.file_storage(), "FileBase".into_val(&e));
    assert_eq!(cert_governance.name(), "ChaincertName".into_val(&e));
    assert!(cert_governance.is_revocable());
    assert_eq!(cert_governance.expiration_time(), OptionU64::None);
    assert_eq!(cert_governance.distribution_limit(), 10);
    assert_eq!(cert_governance.supply(), 0);
    assert_eq!(cert_governance.receivers().len(), 0);
}

#[test]
fn test_get_contract_info() {
    let e: Env = Default::default();
    let organization: Organization = Organization {
        admin: Address::random(&e),
        id: "12345".into_val(&e),
    };
    let governance_rules_without_expiration_time = (true, OptionU64::None);

    let governance_rules_with_expiration_time = (true, OptionU64::Some(31556926));

    let distribution_limit: Option<u32> = Option::Some(6);

    let cert_governance = create_cert_governance_contract_with_limit(
        &e,
        &distribution_limit,
        &Option::None,
        &organization,
        &governance_rules_without_expiration_time,
    );

    let cert_governance_2 = create_cert_governance_contract_with_limit(
        &e,
        &distribution_limit,
        &Option::None,
        &organization,
        &governance_rules_with_expiration_time,
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

    assert_eq!(cert_governance.info(), info);
    assert_eq!(cert_governance_2.info(), info_2);
}

#[test]
fn test_distribute_with_distribution_limit_contract() {
    let e: Env = Default::default();
    let address_receiver_1 = Address::random(&e);
    let wallet = create_wallet_contract(&e, &address_receiver_1, &"12345".into_val(&e));
    let organization: Organization = Organization {
        admin: Address::random(&e),
        id: "12345".into_val(&e),
    };
    let distribution_limit: Option<u32> = Option::Some(6);
    let governance_rules = (true, OptionU64::None);
    let cert_governance = create_cert_governance_contract_with_limit(
        &e,
        &distribution_limit,
        &Option::None,
        &organization,
        &governance_rules,
    );

    pub const CID1: &str = "QmdtyfTYbVS3K9iYqBPjXxn4mbB7aBvEjYGzYWnzRcMrEC";

    let distribution_date: u64 = 1679918400;
    cert_governance.distribute(
        &organization.admin,
        &address_receiver_1,
        &wallet.contract_id,
        &CID1.into_val(&e),
        &distribution_date,
    );
    let receivers = cert_governance.receivers();
    let cert_data = receivers.get(address_receiver_1).unwrap().unwrap();

    assert_eq!(cert_data.status, Status::Distribute);
    assert_eq!(cert_governance.supply(), 1);
    assert_eq!(receivers.len(), 1);
    assert_eq!(wallet.get_chaincerts().len(), 1);
}

#[test]
fn test_distribute_with_initial_receivers() {
    let e: Env = Default::default();
    let receivers = Option::Some(create_random_receivers_address(&e));
    let address_receiver_1 = receivers
        .clone()
        .expect("Vec of receivers")
        .get(0)
        .unwrap()
        .unwrap();
    let wallet = create_wallet_contract(&e, &address_receiver_1, &"12345".into_val(&e));
    let organization: Organization = Organization {
        admin: Address::random(&e),
        id: "12345".into_val(&e),
    };
    let distribution_date: u64 = 1679918400;
    pub const CID1: &str = "QmdtyfTYbVS3K9iYqBPjXxn4mbB7aBvEjYGzYWnzRcMrEC";
    let governance_rules = (true, OptionU64::Some(31556926));
    let cert_governance = create_cert_governance_contract_with_receivers(
        &e,
        &Option::None,
        &receivers,
        &organization,
        &governance_rules,
    );

    let mut receivers = cert_governance.receivers();
    let mut cert_data = receivers.get(address_receiver_1.clone()).unwrap().unwrap();
    assert_eq!(cert_data.status, Status::Unassigned);

    cert_governance.distribute(
        &organization.admin,
        &address_receiver_1,
        &wallet.contract_id,
        &CID1.into_val(&e),
        &distribution_date,
    );

    receivers = cert_governance.receivers();
    cert_data = receivers.get(address_receiver_1).unwrap().unwrap();

    assert_eq!(cert_data.status, Status::Distribute);
    assert_eq!(cert_governance.supply(), 1);
    assert_eq!(receivers.len(), 3);
    assert_eq!(wallet.get_chaincerts().len(), 1);
}

#[test]
fn test_revoke_chaincert() {
    let e: Env = Default::default();
    let receivers: Option<Vec<Address>> = Option::Some(create_random_receivers_address(&e));
    let receiver_address = receivers
        .clone()
        .expect("Vec of receivers")
        .get(0)
        .unwrap()
        .unwrap();
    let wallet = create_wallet_contract(&e, &receiver_address, &"12345".into_val(&e));
    let organization: Organization = Organization {
        admin: Address::random(&e),
        id: "12345".into_val(&e),
    };

    let governance_rules = (true, OptionU64::None);

    let distribution_date: u64 = 1679918400;
    pub const CID1: &str = "QmdtyfTYbVS3K9iYqBPjXxn4mbB7aBvEjYGzYWnzRcMrEC";

    let cert_governance = create_cert_governance_contract_with_receivers(
        &e,
        &Option::None,
        &receivers,
        &organization,
        &governance_rules,
    );

    cert_governance.distribute(
        &organization.admin,
        &receiver_address,
        &wallet.contract_id,
        &CID1.into_val(&e),
        &distribution_date,
    );

    let mut receivers = cert_governance.receivers();
    let mut cert_data = receivers.get(receiver_address.clone()).unwrap().unwrap();
    assert_eq!(cert_data.status, Status::Distribute);

    cert_governance.revoke(&organization.admin, &receiver_address, &wallet.contract_id);

    receivers = cert_governance.receivers();
    cert_data = receivers.get(receiver_address).unwrap().unwrap();
    assert_eq!(cert_data.status, Status::Revoked);

    let chaincert = wallet.get_chaincerts().get(0).unwrap().unwrap();
    assert!(chaincert.revoked);
}

#[test]
#[should_panic(expected = "Status(ContractError(1))")]
fn test_initialize_contract_with_receivers_error() {
    let e: Env = Default::default();
    let receivers: Option<Vec<Address>> = Option::Some(create_random_receivers_address(&e));
    let organization: Organization = Organization {
        admin: Address::random(&e),
        id: "12345".into_val(&e),
    };

    let governance_rules = (true, OptionU64::None);

    let cert_governance: CertGovernanceClient = create_cert_governance_contract_with_receivers(
        &e,
        &Option::None,
        &receivers,
        &organization,
        &governance_rules,
    );

    cert_governance.initialize(
        &"FileBase".into_val(&e),
        &"ChaincertName".into_val(&e),
        &receivers,
        &Option::None,
        &governance_rules,
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

    let governance_rules = (true, OptionU64::None);

    let distribution_limit: Option<u32> = Option::Some(6);
    let cert_governance = create_cert_governance_contract_with_limit(
        &e,
        &distribution_limit,
        &Option::None,
        &organization,
        &governance_rules,
    );

    cert_governance.initialize(
        &"FileBase".into_val(&e),
        &"ChaincertName".into_val(&e),
        &Option::None,
        &distribution_limit,
        &governance_rules,
        &organization,
    )
}

#[test]
#[should_panic(expected = "Status(ContractError(2))")]
fn test_distribute_admin_error() {
    pub const CID1: &str = "QmdtyfTYbVS3K9iYqBPjXxn4mbB7aBvEjYGzYWnzRcMrEC";
    let e: Env = Default::default();
    let admin = Address::random(&e);
    let receivers = Option::Some(create_random_receivers_address(&e));
    let receiver_address = receivers
        .clone()
        .expect("Vec of receivers")
        .get(0)
        .unwrap()
        .unwrap();
    let wallet = create_wallet_contract(&e, &receiver_address, &"12345".into_val(&e));
    let organization: Organization = Organization {
        admin: Address::random(&e),
        id: "12345".into_val(&e),
    };

    let governance_rules = (true, OptionU64::None);

    let distribution_date: u64 = 1679918400;
    let cert_governance = create_cert_governance_contract_with_receivers(
        &e,
        &Option::None,
        &receivers,
        &organization,
        &governance_rules,
    );

    cert_governance.distribute(
        &admin,
        &receiver_address,
        &wallet.contract_id,
        &CID1.into_val(&e),
        &distribution_date,
    );
}

#[test]
#[should_panic(expected = "Status(ContractError(3))")]
fn test_distribute_limit_error() {
    let e: Env = Default::default();
    let receivers = Option::Some(create_random_receivers_address(&e));
    let wallet1 = create_wallet_contract(
        &e,
        &receivers
            .clone()
            .expect("Vec of receivers")
            .get(0)
            .unwrap()
            .unwrap(),
        &"12345".into_val(&e),
    );
    let wallet2 = create_wallet_contract(
        &e,
        &receivers
            .clone()
            .expect("Vec of receivers")
            .get(1)
            .unwrap()
            .unwrap(),
        &"12345".into_val(&e),
    );

    let organization: Organization = Organization {
        admin: Address::random(&e),
        id: "12345".into_val(&e),
    };

    let governance_rules = (true, OptionU64::None);

    let distribution_date: u64 = 1679918400;
    let distribution_limit = Option::Some(1);
    let cert_governance = create_cert_governance_contract_with_limit(
        &e,
        &distribution_limit,
        &Option::None,
        &organization,
        &governance_rules,
    );
    pub const CID1: &str = "QmdtyfTYbVS3K9iYqBPjXxn4mbB7aBvEjYGzYWnzRcMrEC";
    pub const CID2: &str = "QmdtyfTYbVS3K9iYqBPjXxn4mbB7aBvEjYGzYWnzRcMrED";

    cert_governance.distribute(
        &organization.admin,
        &receivers
            .clone()
            .expect("Vec of receivers")
            .get(0)
            .unwrap()
            .unwrap(),
        &wallet1.contract_id,
        &CID1.into_val(&e),
        &distribution_date,
    );

    cert_governance.distribute(
        &organization.admin,
        &receivers
            .expect("Vec of receivers")
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
    let receivers = Option::Some(create_random_receivers_address(&e));
    let wallet = create_wallet_contract(
        &e,
        &receivers
            .clone()
            .expect("Vec of receivers")
            .get(0)
            .unwrap()
            .unwrap(),
        &"12345".into_val(&e),
    );
    let organization: Organization = Organization {
        admin: Address::random(&e),
        id: "12345".into_val(&e),
    };

    let governance_rules = (true, OptionU64::None);

    let distribution_date: u64 = 1679918400;
    let distribution_limit = Option::Some(3);
    let cert_governance = create_cert_governance_contract_with_limit(
        &e,
        &distribution_limit,
        &Option::None,
        &organization,
        &governance_rules,
    );
    pub const CID1: &str = "QmdtyfTYbVS3K9iYqBPjXxn4mbB7aBvEjYGzYWnzRcMrEC";

    cert_governance.distribute(
        &organization.admin,
        &receivers
            .clone()
            .expect("Vec of receivers")
            .get(0)
            .unwrap()
            .unwrap(),
        &wallet.contract_id,
        &CID1.into_val(&e),
        &distribution_date,
    );

    cert_governance.distribute(
        &organization.admin,
        &receivers
            .expect("Vec of receivers")
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
    let receivers: Option<Vec<Address>> = Option::Some(create_random_receivers_address(&e));
    let receiver_address = receivers
        .clone()
        .expect("Vec of receivers")
        .get(0)
        .unwrap()
        .unwrap();
    let wallet = create_wallet_contract(&e, &receiver_address, &"12345".into_val(&e));
    let organization: Organization = Organization {
        admin: Address::random(&e),
        id: "12345".into_val(&e),
    };

    let governance_rules = (true, OptionU64::None);

    let distribution_date: u64 = 1679918400;
    pub const CID1: &str = "QmdtyfTYbVS3K9iYqBPjXxn4mbB7aBvEjYGzYWnzRcMrEC";
    let cert_governance = create_cert_governance_contract_with_receivers(
        &e,
        &Option::None,
        &receivers,
        &organization,
        &governance_rules,
    );

    cert_governance.distribute(
        &organization.admin,
        &receiver_address,
        &wallet.contract_id,
        &CID1.into_val(&e),
        &distribution_date,
    );

    cert_governance.revoke(&Address::random(&e), &receiver_address, &wallet.contract_id);
}

#[test]
#[should_panic(expected = "Status(ContractError(7))")]
fn test_revoke_status_unassigned_error() {
    let e: Env = Default::default();
    let receivers: Option<Vec<Address>> = Option::Some(create_random_receivers_address(&e));
    let receiver_address = receivers
        .clone()
        .expect("Vec of receivers")
        .get(0)
        .unwrap()
        .unwrap();
    let wallet = create_wallet_contract(&e, &receiver_address, &"12345".into_val(&e));
    let organization: Organization = Organization {
        admin: Address::random(&e),
        id: "12345".into_val(&e),
    };

    let governance_rules = (true, OptionU64::None);

    let cert_governance = create_cert_governance_contract_with_receivers(
        &e,
        &Option::None,
        &receivers,
        &organization,
        &governance_rules,
    );

    cert_governance.revoke(&organization.admin, &receiver_address, &wallet.contract_id);
}

#[test]
#[should_panic(expected = "Status(ContractError(7))")]
fn test_revoke_status_revoked_error() {
    let e: Env = Default::default();
    let receivers: Option<Vec<Address>> = Option::Some(create_random_receivers_address(&e));
    let receiver_address = receivers
        .clone()
        .expect("Vec of receivers")
        .get(0)
        .unwrap()
        .unwrap();
    let wallet = create_wallet_contract(&e, &receiver_address, &"12345".into_val(&e));
    let organization: Organization = Organization {
        admin: Address::random(&e),
        id: "12345".into_val(&e),
    };

    let governance_rules = (true, OptionU64::None);

    let distribution_date: u64 = 1679918400;
    pub const CID1: &str = "QmdtyfTYbVS3K9iYqBPjXxn4mbB7aBvEjYGzYWnzRcMrEC";
    let cert_governance = create_cert_governance_contract_with_receivers(
        &e,
        &Option::None,
        &receivers,
        &organization,
        &governance_rules,
    );
    cert_governance.distribute(
        &organization.admin,
        &receiver_address,
        &wallet.contract_id,
        &CID1.into_val(&e),
        &distribution_date,
    );

    cert_governance.revoke(&organization.admin, &receiver_address, &wallet.contract_id);
    cert_governance.revoke(&organization.admin, &receiver_address, &wallet.contract_id);
}

#[test]
#[should_panic(expected = "Status(ContractError(7))")]
fn test_revoke_no_revocable_cert() {
    let e: Env = Default::default();
    let receivers: Option<Vec<Address>> = Option::Some(create_random_receivers_address(&e));
    let receiver_address = receivers
        .clone()
        .expect("Vec of receivers")
        .get(0)
        .unwrap()
        .unwrap();
    let wallet = create_wallet_contract(&e, &receiver_address, &"12345".into_val(&e));
    let organization: Organization = Organization {
        admin: Address::random(&e),
        id: "12345".into_val(&e),
    };

    let governance_rules = (false, OptionU64::None);

    let cert_governance = create_cert_governance_contract_with_receivers(
        &e,
        &Option::None,
        &receivers,
        &organization,
        &governance_rules,
    );

    let receiver_address = receivers
        .expect("Vec of receivers")
        .get(0)
        .unwrap()
        .unwrap();

    cert_governance.revoke(&organization.admin, &receiver_address, &wallet.contract_id);
}
