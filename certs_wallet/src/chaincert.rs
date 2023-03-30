//! Module Chaincert
//!
//! Module responsible of managing `Chaincerts` information and defining its corresponding struct.
use crate::{option::OptU64, storage_types::DataKey};
use soroban_sdk::{contracttype, map, Address, Bytes, Env, Map, Vec};

const CHAINCERT_KEY: DataKey = DataKey::Chaincerts;

#[derive(Clone, PartialEq, Eq, Debug)]
#[contracttype]
/// The `Chaincert` information stored in the wallet
pub struct Chaincert {
    pub cid: Bytes,
    /// Address of the governance contract that distributed the `Chaincert`
    pub dist_cont: Address,
    /// The id of the organization that distributed the `Chaincert`
    pub org_id: Bytes,
    /// The distribution date in Unix Timestamp format
    pub dist_date: u64,
    /// The expiration date in Unix Timestamp format
    pub exp_date: OptU64,
    /// A logical indicator that lets know if a `Chaincert` is revoked or not
    pub revoked: bool,
}

impl Chaincert {
    fn new(
        cid: Bytes,
        dist_cont: Address,
        org_id: Bytes,
        dist_date: u64,
        exp_date: OptU64,
        revoked: bool,
    ) -> Chaincert {
        Chaincert {
            cid,
            dist_cont,
            org_id,
            dist_date,
            exp_date,
            revoked,
        }
    }
}

pub(crate) fn deposit_chaincert(
    env: &Env,
    chaincert_id: Bytes,
    cid: Bytes,
    distributor_contract: Address,
    org_id: Bytes,
    distribution_date: u64,
    expiration_date: OptU64,
) {
    let chaincert = Chaincert::new(
        cid,
        distributor_contract,
        org_id,
        distribution_date,
        expiration_date,
        false,
    );

    let chaincerts = match env.storage().get(&CHAINCERT_KEY) {
        Some(chaincert_map) => {
            let mut chaincert_map: Map<Bytes, Chaincert> = chaincert_map.unwrap();
            if !chaincert_map.contains_key(chaincert_id.clone()) {
                chaincert_map.set(chaincert_id, chaincert);
                chaincert_map
            } else {
                panic!("The chaincert is already deposited in the wallet")
            }
        }
        None => {
            let map: Map<Bytes, Chaincert> = map![env, (chaincert_id, chaincert)];
            map
        }
    };

    write_chaincerts(env, &chaincerts)
}

pub(crate) fn revoke_chaincert(
    env: &Env,
    chaincert_id: &Bytes,
    distributor_contract: &Address,
    org_id: &Bytes,
) {
    match env.storage().get(&CHAINCERT_KEY) {
        Some(chaincert_map) => {
            let mut chaincert_map: Map<Bytes, Chaincert> = chaincert_map.unwrap();
            remove_chaincert_from_map(
                &mut chaincert_map,
                chaincert_id,
                distributor_contract,
                org_id,
            );
            write_chaincerts(env, &chaincert_map);
        }
        None => {
            panic!("This wallet doesn't own any `chaincert` for the moment")
        }
    };
}

pub(crate) fn get_chaincerts(env: &Env) -> Vec<Chaincert> {
    read_chaincerts(env).values()
}

fn remove_chaincert_from_map(
    chaincert_map: &mut Map<Bytes, Chaincert>,
    chaincert_id: &Bytes,
    distributor_contract: &Address,
    org_id: &Bytes,
) {
    match chaincert_map.get(chaincert_id.clone()) {
        Some(chaincert) => {
            let mut chaincert = chaincert.unwrap();
            if chaincert.dist_cont == distributor_contract.clone()
                && chaincert.org_id == org_id.clone()
            {
                chaincert.revoked = true;
                chaincert_map.set(chaincert_id.clone(), chaincert);
            } else {
                panic!("Not Authorized");
            }
        }
        None => panic!("The chaincert doesn't exist"),
    }
}

fn read_chaincerts(env: &Env) -> Map<Bytes, Chaincert> {
    match env.storage().get(&CHAINCERT_KEY) {
        Some(cc) => cc.unwrap(),
        None => panic!("This wallet doesn't own any `chaincert` for the moment"),
    }
}

fn write_chaincerts(env: &Env, certs: &Map<Bytes, Chaincert>) {
    env.storage().set(&CHAINCERT_KEY, certs)
}
