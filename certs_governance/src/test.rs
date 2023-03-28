#![cfg(test)]
extern crate std;
use crate::storage_types::{CertData, Opt, Organization, Status};
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{Address, Bytes, Env, IntoVal};

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
