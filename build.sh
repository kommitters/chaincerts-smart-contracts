#!/bin/sh
soroban contract build --package soroban-did-contract
soroban contract optimize --wasm target/wasm32-unknown-unknown/release/soroban_did_contract.wasm
soroban contract build
soroban contract optimize --wasm target/wasm32-unknown-unknown/release/deployer_contract.wasm
soroban contract optimize --wasm target/wasm32-unknown-unknown/release/vault_contract.wasm
soroban contract optimize --wasm target/wasm32-unknown-unknown/release/vc_issuance_contract.wasm
