//! Module Chaincert
//!
//! Module responsible of managing `Chaincerts` information and defining its corresponding struct.
use crate::option::OptU64;
use soroban_sdk::{contracttype, Address, Bytes};

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
    #[cfg(not(tarpaulin_include))]
    fn _new(
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
