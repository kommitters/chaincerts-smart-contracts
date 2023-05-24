//! Module Option
//!
//! Module to create new Option types if necessary since Rust Option doesn't works when using `contracttype` for now
//! as well as generics. Replace this logic with generic Option when supported
use soroban_sdk::contracttype;

use crate::did_document::MethodService;

/// OptU64 basic implementation
#[contracttype]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum OptionU64 {
    None,
    Some(u64),
}

#[contracttype]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum OptionMethodService {
    None,
    Some(MethodService),
}
