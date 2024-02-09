#![no_std]
mod contract;
mod error;
mod issuer;
mod storage;
mod vault_trait;
mod verifiable_credential;
mod did_contract {
    soroban_sdk::contractimport!(
        file = "../target/wasm32-unknown-unknown/release/soroban_did_contract.optimized.wasm"
    );
}

#[cfg(test)]
mod test;
