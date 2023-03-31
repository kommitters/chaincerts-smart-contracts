//! Module Option
//!
//! Module to create new Option types if necessary since Rust Option doesn't works when using `contracttype` for now
//! as well as generics
use soroban_sdk::contracttype;

/// OptU64 basic implementation
#[contracttype]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum OptU64 {
    None,
    Some(u64),
}
