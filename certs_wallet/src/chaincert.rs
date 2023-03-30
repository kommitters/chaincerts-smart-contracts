//! Module Chaincert
//!
//! Module responsible of managing `Chaincerts` information and defining its corresponding struct.
use crate::{option::OptU64, storage_types::DataKey};
use soroban_sdk::{contracttype, map, Address, Bytes, Env, Map};

const CHAINCERT_KEY: DataKey = DataKey::Chaincerts;

#[derive(Clone, PartialEq, Eq, Debug)]
#[contracttype]
/// The `Chaincert` information stored in the wallet
pub struct Chaincert {
    pub cid: Bytes,
    /// Address of the governance contract that distributed the `Chaincert`
    pub cont_dist: Address,
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
        cont_dist: Address,
        org_id: Bytes,
        dist_date: u64,
        exp_date: OptU64,
        revoked: bool,
    ) -> Chaincert {
        Chaincert {
            cid,
            cont_dist,
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
        Some(cc_map) => {
            let mut cc_map: Map<Bytes, Chaincert> = cc_map.unwrap();
            if !cc_map.contains_key(chaincert_id.clone()) {
                cc_map.set(chaincert_id, chaincert);
                cc_map
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

fn write_chaincerts(env: &Env, certs: &Map<Bytes, Chaincert>) {
    env.storage().set(&CHAINCERT_KEY, certs)
}
