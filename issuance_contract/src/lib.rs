#![no_std]
mod contract;
mod error;
mod issuance_trait;
mod metadata;
mod organization;
mod recipients;
mod storage_types;
mod did_contract {
    soroban_sdk::contractimport!(file = "./did_contract.wasm");
}
mod test;
pub use crate::contract::IssuanceContractClient;
