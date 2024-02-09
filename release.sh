#!/bin/sh
# Config testnet in local.
soroban config network add testnet \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase "Test SDF Network ; September 2015"

# Generate key to sign the transactions.
soroban keys generate chaincerts_sc_source --network testnet

# Install and deploy contracts.
soroban contract install \
  --wasm target/wasm32-unknown-unknown/release/vault_contract.optimized.wasm \
  --source chaincerts_sc_source \
  --network testnet

soroban contract install \
  --wasm target/wasm32-unknown-unknown/release/vc_issuance_contract.optimized.wasm \
  --source chaincerts_sc_source \
  --network testnet

soroban contract install \
  --wasm target/wasm32-unknown-unknown/release/soroban_did_contract.optimized.wasm \
  --source chaincerts_sc_source \
  --network testnet

soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/deployer_contract.optimized.wasm \
  --source chaincerts_sc_source \
  --network testnet
