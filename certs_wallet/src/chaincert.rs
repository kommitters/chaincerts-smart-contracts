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
