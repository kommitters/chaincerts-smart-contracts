#![no_std]
mod contract;
mod error;
mod issuer;
mod storage;
mod vault;
mod vault_trait;
mod verifiable_credential;
mod did_contract {
    soroban_sdk::contractimport!(file = "./soroban_did_contract.wasm");
}

#[cfg(test)]
mod test;
