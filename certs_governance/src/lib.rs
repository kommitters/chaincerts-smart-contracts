#![no_std]
mod contract;
mod error;
mod governance_trait;
mod metadata;
mod organization;
mod recipients;
mod storage_types;
mod certs_wallet {
    soroban_sdk::contractimport!(file = "./certs_wallet.wasm");
}
mod test;
pub use crate::contract::CertGovernanceClient;
