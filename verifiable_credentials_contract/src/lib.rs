#![no_std]
mod base32;
mod contract;
mod error;
mod revocation;
mod storage;
mod vcs_trait;
mod verifiable_credential;
mod vault_contract {
    soroban_sdk::contractimport!(file = "../../target/wasm32-unknown-unknown/release/vault_contract.wasm");
}

#[cfg(test)]
mod test;
