#![no_std]
mod base32;
mod contract;
mod error;
mod revocation;
mod storage;
mod vc_issuance_trait;
mod verifiable_credential;

#[cfg(test)]
mod test;
