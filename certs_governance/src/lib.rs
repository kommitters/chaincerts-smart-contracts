#![no_std]
mod contract;
mod error;
mod governance_trait;
mod metadata;
mod organization;
mod receivers;
mod storage_types;
mod test;
pub use crate::contract::CertGovernanceClient;
