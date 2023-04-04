//! Module error
//!
//! Module that groups the errors within the contract and assigns them a code
use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    AlreadyInit = 1,
    NotAuthorized = 2,
    LimitReached = 3,
    AlreadyIssued = 5,
    NoRevocable = 7,
}
