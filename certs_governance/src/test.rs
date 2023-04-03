#![cfg(test)]
use crate::certs_wallet::{self, OptU64};
use crate::storage_types::{CertData, Info, Organization, Status};
use crate::{contract::CertGovernance, CertGovernanceClient};
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{vec, Address, Bytes, Env, IntoVal, Vec};

const WASM: &[u8] = include_bytes!("../../target/wasm32-unknown-unknown/release/certs_wallet.wasm");

fn create_wallet_contract(env: &Env, owner: &Address, org_id: &Bytes) -> certs_wallet::Client {
    let wallet = certs_wallet::Client::new(env, &env.register_contract_wasm(None, WASM));
    wallet.initialize(owner);
    wallet.add_org(org_id);
    wallet
}

fn create_cert_governance_contract_with_limit(
    e: &Env,
    limit: &u32,
    organization: &Organization,
    revocable: &bool,
    expiration_time: &Option<u64>,
) -> CertGovernanceClient {
    let cert_governance =
        CertGovernanceClient::new(e, &e.register_contract(None, CertGovernance {}));
    cert_governance.init_w_l(
        &("FileBase").into_val(e),
        &"ChaincertName".into_val(e),
        revocable,
        expiration_time,
        limit,
        organization,
    );
    cert_governance
}

fn create_cert_governance_contract_with_receivers(
    e: &Env,
    address_receivers: &Vec<Address>,
    organization: &Organization,
    revocable: &bool,
    expiration_time: &Option<u64>,
) -> CertGovernanceClient {
    let cert_governance =
        CertGovernanceClient::new(e, &e.register_contract(None, CertGovernance {}));

    cert_governance.init_w_r(
        &"FileBase".into_val(e),
        &"ChaincertName".into_val(e),
        revocable,
        expiration_time,
        address_receivers,
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
    let distribution_date = OptU64::Some(1711195200);
    let cert_data = CertData::new(
        id_chaincert.clone(),
        status.clone(),
        distribution_date.clone(),
    );
    assert_eq!(cert_data.id_cert, id_chaincert);
    assert_eq!(cert_data.status, status);
    assert_eq!(cert_data.dist_date, distribution_date);
}

#[test]
fn test_initialize_contract_with_receivers() {
    let e: Env = Default::default();
    let receivers: Vec<Address> = create_random_receivers_address(&e);
    let organization: Organization = Organization {
        admin: Address::random(&e),
        id_org: "12345".into_val(&e),
    };

    let cert_governance: CertGovernanceClient = create_cert_governance_contract_with_receivers(
        &e,
        &receivers,
        &organization,
        &true,
        &Option::Some(1680091200),
    );
    assert_eq!(cert_governance.f_storage(), "FileBase".into_val(&e));
    assert_eq!(cert_governance.name(), "ChaincertName".into_val(&e));
    assert!(cert_governance.revocable());
    assert_eq!(cert_governance.exp_time(), Option::Some(1680091200));
    assert_eq!(cert_governance.dist_limit(), 3);
    assert_eq!(cert_governance.supply(), 0);
    assert_eq!(cert_governance.receivers().len(), 3);
}

#[test]
fn test_initialize_with_limit_contract() {
    let e: Env = Default::default();
    let organization: Organization = Organization {
        admin: Address::random(&e),
        id_org: "12345".into_val(&e),
    };
    let distribution_limit: u32 = 6;

    let cert_governance = create_cert_governance_contract_with_limit(
        &e,
        &distribution_limit,
        &organization,
        &true,
        &Option::None,
    );
    assert_eq!(cert_governance.f_storage(), "FileBase".into_val(&e));
    assert_eq!(cert_governance.name(), "ChaincertName".into_val(&e));
    assert!(cert_governance.revocable());
    assert_eq!(cert_governance.exp_time(), Option::None);
    assert_eq!(cert_governance.dist_limit(), 6);
    assert_eq!(cert_governance.supply(), 0);
    assert_eq!(cert_governance.receivers().len(), 0);
}

#[test]
fn test_get_contract_info() {
    let e: Env = Default::default();
    let organization: Organization = Organization {
        admin: Address::random(&e),
        id_org: "12345".into_val(&e),
    };
    let distribution_limit: u32 = 6;

    let cert_governance = create_cert_governance_contract_with_limit(
        &e,
        &distribution_limit,
        &organization,
        &true,
        &Option::None,
    );

    let cert_governance_2 = create_cert_governance_contract_with_limit(
        &e,
        &distribution_limit,
        &organization,
        &true,
        &Option::Some(31556926),
    );

    let info = Info {
        name: "ChaincertName".into_val(&e),
        revocable: true,
        exp_time: OptU64::None,
        dist_limit: 6,
        supply: 0,
    };

    let info_2 = Info {
        name: "ChaincertName".into_val(&e),
        revocable: true,
        exp_time: OptU64::Some(31556926),
        dist_limit: 6,
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
        id_org: "12345".into_val(&e),
    };
    let distribution_limit = 6;
    let cert_governance = create_cert_governance_contract_with_limit(
        &e,
        &distribution_limit,
        &organization,
        &true,
        &Option::None,
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
    assert_eq!(wallet.get_ccs().len(), 1);
}

#[test]
fn test_distribute_with_initial_receivers() {
    let e: Env = Default::default();
    let receivers = create_random_receivers_address(&e);
    let address_receiver_1 = receivers.get(0).unwrap().unwrap();
    let wallet = create_wallet_contract(&e, &address_receiver_1, &"12345".into_val(&e));
    let organization: Organization = Organization {
        admin: Address::random(&e),
        id_org: "12345".into_val(&e),
    };
    let distribution_date: u64 = 1679918400;
    pub const CID1: &str = "QmdtyfTYbVS3K9iYqBPjXxn4mbB7aBvEjYGzYWnzRcMrEC";

    let cert_governance = create_cert_governance_contract_with_receivers(
        &e,
        &receivers,
        &organization,
        &true,
        &Option::Some(31556926),
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
    assert_eq!(wallet.get_ccs().len(), 1);
}

#[test]
fn test_revoke_chaincert() {
    let e: Env = Default::default();
    let receivers: Vec<Address> = create_random_receivers_address(&e);
    let receiver_address = receivers.get(0).unwrap().unwrap();
    let wallet = create_wallet_contract(&e, &receiver_address, &"12345".into_val(&e));
    let organization: Organization = Organization {
        admin: Address::random(&e),
        id_org: "12345".into_val(&e),
    };

    let distribution_date: u64 = 1679918400;
    pub const CID1: &str = "QmdtyfTYbVS3K9iYqBPjXxn4mbB7aBvEjYGzYWnzRcMrEC";

    let cert_governance = create_cert_governance_contract_with_receivers(
        &e,
        &receivers,
        &organization,
        &true,
        &Option::None,
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

    let chaincert = wallet.get_ccs().get(0).unwrap().unwrap();
    assert!(chaincert.revoked);
}

#[test]
#[should_panic(expected = "Status(ContractError(1))")]
fn test_initialize_contract_with_receivers_error() {
    let e: Env = Default::default();
    let receivers: Vec<Address> = create_random_receivers_address(&e);
    let organization: Organization = Organization {
        admin: Address::random(&e),
        id_org: "12345".into_val(&e),
    };
    let cert_governance: CertGovernanceClient = create_cert_governance_contract_with_receivers(
        &e,
        &receivers,
        &organization,
        &true,
        &Option::None,
    );

    cert_governance.init_w_r(
        &"FileBase".into_val(&e),
        &"ChaincertName".into_val(&e),
        &false,
        &Option::None,
        &vec![&e],
        &organization,
    )
}

#[test]
#[should_panic(expected = "Status(ContractError(1))")]
fn test_initialize_with_limit_contract_error() {
    let e: Env = Default::default();
    let organization: Organization = Organization {
        admin: Address::random(&e),
        id_org: "12345".into_val(&e),
    };
    let distribution_limit: u32 = 6;
    let cert_governance = create_cert_governance_contract_with_limit(
        &e,
        &distribution_limit,
        &organization,
        &true,
        &Option::None,
    );

    cert_governance.init_w_l(
        &"FileBase".into_val(&e),
        &"ChaincertName".into_val(&e),
        &true,
        &Option::None,
        &6,
        &organization,
    )
}

#[test]
#[should_panic(expected = "Status(ContractError(2))")]
fn test_distribute_admin_error() {
    pub const CID1: &str = "QmdtyfTYbVS3K9iYqBPjXxn4mbB7aBvEjYGzYWnzRcMrEC";
    let e: Env = Default::default();
    let admin = Address::random(&e);
    let receivers = create_random_receivers_address(&e);
    let receiver_address = receivers.get(0).unwrap().unwrap();
    let wallet = create_wallet_contract(&e, &receiver_address, &"12345".into_val(&e));
    let organization: Organization = Organization {
        admin: Address::random(&e),
        id_org: "12345".into_val(&e),
    };
    let distribution_date: u64 = 1679918400;
    let cert_governance = create_cert_governance_contract_with_receivers(
        &e,
        &receivers,
        &organization,
        &true,
        &Option::None,
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
    let receivers = create_random_receivers_address(&e);
    let wallet1 = create_wallet_contract(
        &e,
        &receivers.get(0).unwrap().unwrap(),
        &"12345".into_val(&e),
    );
    let wallet2 = create_wallet_contract(
        &e,
        &receivers.get(1).unwrap().unwrap(),
        &"12345".into_val(&e),
    );

    let organization: Organization = Organization {
        admin: Address::random(&e),
        id_org: "12345".into_val(&e),
    };
    let distribution_date: u64 = 1679918400;
    let distribution_limit = 1;
    let cert_governance = create_cert_governance_contract_with_limit(
        &e,
        &distribution_limit,
        &organization,
        &true,
        &Option::None,
    );
    pub const CID1: &str = "QmdtyfTYbVS3K9iYqBPjXxn4mbB7aBvEjYGzYWnzRcMrEC";
    pub const CID2: &str = "QmdtyfTYbVS3K9iYqBPjXxn4mbB7aBvEjYGzYWnzRcMrED";

    cert_governance.distribute(
        &organization.admin,
        &receivers.get(0).unwrap().unwrap(),
        &wallet1.contract_id,
        &CID1.into_val(&e),
        &distribution_date,
    );

    cert_governance.distribute(
        &organization.admin,
        &receivers.get(1).unwrap().unwrap(),
        &wallet2.contract_id,
        &CID2.into_val(&e),
        &distribution_date,
    );
}

#[test]
#[should_panic(expected = "Status(ContractError(5))")]
fn test_distribute_status_error() {
    let e: Env = Default::default();
    let receivers = create_random_receivers_address(&e);
    let wallet = create_wallet_contract(
        &e,
        &receivers.get(0).unwrap().unwrap(),
        &"12345".into_val(&e),
    );
    let organization: Organization = Organization {
        admin: Address::random(&e),
        id_org: "12345".into_val(&e),
    };
    let distribution_date: u64 = 1679918400;
    let distribution_limit = 3;
    let cert_governance = create_cert_governance_contract_with_limit(
        &e,
        &distribution_limit,
        &organization,
        &true,
        &Option::None,
    );
    pub const CID1: &str = "QmdtyfTYbVS3K9iYqBPjXxn4mbB7aBvEjYGzYWnzRcMrEC";

    cert_governance.distribute(
        &organization.admin,
        &receivers.get(0).unwrap().unwrap(),
        &wallet.contract_id,
        &CID1.into_val(&e),
        &distribution_date,
    );

    cert_governance.distribute(
        &organization.admin,
        &receivers.get(0).unwrap().unwrap(),
        &wallet.contract_id,
        &CID1.into_val(&e),
        &distribution_date,
    );
}

#[test]
#[should_panic(expected = "Status(ContractError(2))")]
fn test_revoke_admin_error() {
    let e: Env = Default::default();
    let receivers: Vec<Address> = create_random_receivers_address(&e);
    let receiver_address = receivers.get(0).unwrap().unwrap();
    let wallet = create_wallet_contract(&e, &receiver_address, &"12345".into_val(&e));
    let organization: Organization = Organization {
        admin: Address::random(&e),
        id_org: "12345".into_val(&e),
    };
    let distribution_date: u64 = 1679918400;
    pub const CID1: &str = "QmdtyfTYbVS3K9iYqBPjXxn4mbB7aBvEjYGzYWnzRcMrEC";
    let cert_governance = create_cert_governance_contract_with_receivers(
        &e,
        &receivers,
        &organization,
        &true,
        &Option::None,
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
    let receivers: Vec<Address> = create_random_receivers_address(&e);
    let receiver_address = receivers.get(0).unwrap().unwrap();
    let wallet = create_wallet_contract(&e, &receiver_address, &"12345".into_val(&e));
    let organization: Organization = Organization {
        admin: Address::random(&e),
        id_org: "12345".into_val(&e),
    };
    let cert_governance = create_cert_governance_contract_with_receivers(
        &e,
        &receivers,
        &organization,
        &true,
        &Option::None,
    );

    cert_governance.revoke(&organization.admin, &receiver_address, &wallet.contract_id);
}

#[test]
#[should_panic(expected = "Status(ContractError(7))")]
fn test_revoke_status_revoked_error() {
    let e: Env = Default::default();
    let receivers: Vec<Address> = create_random_receivers_address(&e);
    let receiver_address = receivers.get(0).unwrap().unwrap();
    let wallet = create_wallet_contract(&e, &receiver_address, &"12345".into_val(&e));
    let organization: Organization = Organization {
        admin: Address::random(&e),
        id_org: "12345".into_val(&e),
    };
    let distribution_date: u64 = 1679918400;
    pub const CID1: &str = "QmdtyfTYbVS3K9iYqBPjXxn4mbB7aBvEjYGzYWnzRcMrEC";
    let cert_governance = create_cert_governance_contract_with_receivers(
        &e,
        &receivers,
        &organization,
        &true,
        &Option::None,
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
    let receivers: Vec<Address> = create_random_receivers_address(&e);
    let organization: Organization = Organization {
        admin: Address::random(&e),
        id_org: "12345".into_val(&e),
    };
    let cert_governance = create_cert_governance_contract_with_receivers(
        &e,
        &receivers,
        &organization,
        &false,
        &Option::None,
    );
    let wallet_contract_id: Bytes = "wallet_contract_id".into_val(&e);
    let receiver_address = receivers.get(0).unwrap().unwrap();

    cert_governance.revoke(&organization.admin, &receiver_address, &wallet_contract_id);
}
