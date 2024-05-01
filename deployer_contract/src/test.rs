#![cfg(test)]
use crate::{DeployerContract, DeployerContractClient};
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{vec, Address, BytesN, Env, FromVal, String, Val, Vec};
// The contract that will be deployed by the deployer contract.
mod contract {
    soroban_sdk::contractimport!(
        file = "../target/wasm32-unknown-unknown/release/vc_issuance_contract.wasm"
    );
}

extern crate std;

#[test]
fn test_from_contract() {
    let env = Env::default();
    let client = DeployerContractClient::new(&env, &env.register_contract(None, DeployerContract));

    // Install the WASM code to be deployed from the deployer contract.
    let wasm_id = env.deployer().upload_contract_wasm(contract::WASM);

    // Deploy contract using deployer, and include an init function to call.
    let salt = BytesN::from_array(&env, &[0; 32]);
    let init_fn_args = vc_issuance_init_args(&env);
    let (_contract_address, init_result) =
        client.deploy(&client.address, &wasm_id, &salt, &init_fn_args);
    assert!(init_result.is_void());
}

#[test]
fn test_deploy_from_address() {
    let env = Env::default();
    let deployer_client =
        DeployerContractClient::new(&env, &env.register_contract(None, DeployerContract));

    // Upload the Wasm to be deployed from the deployer contract.
    // This can also be called from within a contract if needed.
    let wasm_hash = env.deployer().upload_contract_wasm(contract::WASM);

    // Define a deployer address that needs to authorize the deployment.
    let deployer = Address::generate(&env);

    // Deploy contract using deployer, and include an init function to call.
    let salt = BytesN::from_array(&env, &[0; 32]);
    let init_fn_args = vc_issuance_init_args(&env);
    env.mock_all_auths();
    let (_contract_address, init_result) =
        deployer_client.deploy(&deployer, &wasm_hash, &salt, &init_fn_args);

    assert!(init_result.is_void());
}

fn vc_issuance_init_args(env: &Env) -> Vec<Val> {
    let admin = Address::generate(env);
    let issuer_did = String::from_str(env, "did:chaincerts:3mtjfbxad3wzh7qa4w5f7q4h");
    vec![
        env,
        Val::from_val(env, &admin),
        Val::from_val(env, &issuer_did),
    ]
}
