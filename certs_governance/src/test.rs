#![cfg(test)]
extern crate std;
use crate::storage_types::{CertData, Info, Opt, Organization, Status};
use crate::{contract::CertGovernance, CertGovernanceClient};
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{vec, Address, Bytes, Env, IntoVal, Vec};

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
    addess_receivers: &Vec<Address>,
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
        addess_receivers,
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
    let distribution_date = Opt::Some(1711195200);
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
fn test_create_organization() {
    let e: Env = Default::default();
    let id_org: Bytes = "12345".into_val(&e);
    let admin: Address = Address::random(&e);
    let organization = Organization::new(id_org.clone(), admin.clone());
    assert_eq!(organization.id_org, id_org);
    assert_eq!(organization.admin, admin);
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
    assert_eq!(cert_governance.org(), "12345".into_val(&e));
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
    assert_eq!(cert_governance.org(), "12345".into_val(&e));
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
        exp_time: Opt::None,
        dist_limit: 6,
        supply: 0,
    };

    let info_2 = Info {
        name: "ChaincertName".into_val(&e),
        revocable: true,
        exp_time: Opt::Some(31556926),
        dist_limit: 6,
        supply: 0,
    };

    assert_eq!(cert_governance.info(), info);
    assert_eq!(cert_governance_2.info(), info_2);
}

#[test]
#[should_panic(expected = "Already initialized")]
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
fn test_distribute_with_distribution_limit_contract() {
    let e: Env = Default::default();
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
    let address_receiver_1 = Address::random(&e);
    let distribution_date: u64 = 1679918400;
    cert_governance.distribute(
        &organization.admin,
        &address_receiver_1,
        &"wallet_contract_id".into_val(&e),
        &CID1.into_val(&e),
        &distribution_date,
    );
    let receivers = cert_governance.receivers();
    let cert_data = receivers.get(address_receiver_1).unwrap().unwrap();

    assert_eq!(cert_data.status, Status::Distribute);
    assert_eq!(cert_governance.supply(), 1);
    assert_eq!(receivers.len(), 1);
}

#[test]
fn test_distribute_with_initial_receivers() {
    let e: Env = Default::default();
    let receivers = create_random_receivers_address(&e);
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

    let address_receiver_1 = receivers.get(0).unwrap().unwrap();

    let mut receivers = cert_governance.receivers();
    let mut cert_data = receivers.get(address_receiver_1.clone()).unwrap().unwrap();
    assert_eq!(cert_data.status, Status::Unassigned);

    cert_governance.distribute(
        &organization.admin,
        &address_receiver_1,
        &"wallet_contract_id".into_val(&e),
        &CID1.into_val(&e),
        &distribution_date,
    );

    receivers = cert_governance.receivers();
    cert_data = receivers.get(address_receiver_1).unwrap().unwrap();

    assert_eq!(cert_data.status, Status::Distribute);
    assert_eq!(cert_governance.supply(), 1);
    assert_eq!(receivers.len(), 3);
}

#[test]
#[should_panic(expected = "Already initialized")]
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
#[should_panic(expected = "Does not have administrator permissions")]
fn test_distribute_admin_error() {
    pub const CID1: &str = "QmdtyfTYbVS3K9iYqBPjXxn4mbB7aBvEjYGzYWnzRcMrEC";
    let e: Env = Default::default();
    let admin = Address::random(&e);
    let receivers = create_random_receivers_address(&e);
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
        &receivers.get(0).unwrap().unwrap(),
        &"wallet_contract_id".into_val(&e),
        &CID1.into_val(&e),
        &distribution_date,
    );
}

#[test]
#[should_panic(expected = "It is not possible to issue more Chaincerts")]
fn distribute_over_amount_error() {
    let e: Env = Default::default();
    let receivers = create_random_receivers_address(&e);
    let organization: Organization = Organization {
        admin: Address::random(&e),
        id_org: "12345".into_val(&e),
    };
    let distribution_date: u64 = 1679918400;
    let distribution_limit = 1;
    let chaincert = create_cert_governance_contract_with_limit(
        &e,
        &distribution_limit,
        &organization,
        &true,
        &Option::None,
    );
    pub const CID1: &str = "QmdtyfTYbVS3K9iYqBPjXxn4mbB7aBvEjYGzYWnzRcMrEC";
    pub const CID2: &str = "QmdtyfTYbVS3K9iYqBPjXxn4mbB7aBvEjYGzYWnzRcMrED";

    chaincert.distribute(
        &organization.admin,
        &receivers.get(0).unwrap().unwrap(),
        &"wallet_contract_id".into_val(&e),
        &CID1.into_val(&e),
        &distribution_date,
    );

    chaincert.distribute(
        &organization.admin,
        &receivers.get(1).unwrap().unwrap(),
        &"wallet_contract_id".into_val(&e),
        &CID2.into_val(&e),
        &distribution_date,
    );
}

#[test]
#[should_panic(expected = "Chaincert has already been issued to the entered address")]
fn test_distribute_status_error() {
    let e: Env = Default::default();
    let receivers = create_random_receivers_address(&e);
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
        &"wallet_contract_id".into_val(&e),
        &CID1.into_val(&e),
        &distribution_date,
    );

    cert_governance.distribute(
        &organization.admin,
        &receivers.get(0).unwrap().unwrap(),
        &"wallet_contract_id".into_val(&e),
        &CID1.into_val(&e),
        &distribution_date,
    );
}
