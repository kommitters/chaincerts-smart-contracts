#![no_std]
mod contract;
mod error;
mod issuance_trait;
mod metadata;
mod organization;
mod recipients;
mod storage_types;
mod cert_wallet {
    soroban_sdk::contractimport!(file = "./cert_wallet.wasm");
}
mod test;
pub use crate::contract::CertIssuanceClient;
