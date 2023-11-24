#![no_std]
mod base32;
mod contract;
mod error;
mod revocation;
mod storage;
mod vc_issuance_trait;
mod verifiable_credential;
mod vault_contract {
    soroban_sdk::contractimport!(file = "./vault_contract.wasm");
}

#[cfg(test)]
mod test;
