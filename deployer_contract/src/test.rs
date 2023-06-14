#![cfg(test)]

use crate::{DeployerContract, DeployerContractClient};
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{vec, Address, BytesN, Env, FromVal, RawVal, String, Vec};

use self::contract::{
    CapabilityInvocation, DIDDocument, Method, OptionMethodService, Service, VerificationMethod,
};

// The contract that will be deployed by the deployer contract.
mod contract {
    soroban_sdk::contractimport!(file = "./did_contract.wasm");
}

extern crate std;

#[test]
fn test() {
    let env = Env::default();
    let client = DeployerContractClient::new(&env, &env.register_contract(None, DeployerContract));

    // Install the WASM code to be deployed from the deployer contract.
    let wasm_id = env.install_contract_wasm(contract::WASM);

    // Deploy contract using deployer, and include an init function to call.
    let salt = BytesN::from_array(&env, &[0; 32]);
    let address = Address::random(&env);
    let init_args = did_init_args(&env, &address);
    let (contract_id, init_result) = client.deploy(&salt, &wasm_id, &init_args);
    assert!(init_result.is_void());

    let expected_did_document = DIDDocument {
        id: String::from_slice(&env, "did:chaincerts:ABC123"),
        authentication: vec![
            &env,
            String::from_slice(&env, "did:chaincerts:ABC123#key-1"),
        ],
        context: vec![
            &env,
            String::from_slice(&env, "https://www.w3.org/ns/did/v1"),
            String::from_slice(&env, "https://www.example.com/context/v1"),
        ],
        services: vec![
            &env,
            Service {
                type_: String::from_slice(&env, "VerifiableCredential"),
                service_endpoint: String::from_slice(&env, "https://did.chaincerts.co/ABC123"),
            },
        ],
        verification_method: vec![
            &env,
            VerificationMethod {
                id: String::from_slice(&env, "did:chaincerts:ABC123#key-1"),
                type_: String::from_slice(&env, "Ed25519VerificationKey2020"),
                controller: String::from_slice(&env, "did:chaincerts:ABC123"),
                blockchain_account_id: address,
            },
        ],
    };
    // Invoke contract to check that it is initialized correctly.
    let client = contract::Client::new(&env, &contract_id);

    let did_document = client.public_did_document();
    assert_eq!(did_document, expected_did_document);
}

fn did_init_args(env: &Env, address: &Address) -> Vec<RawVal> {
    let id = String::from_slice(env, "did:chaincerts:ABC123");
    let authentication_params = (
        String::from_slice(env, "did:chaincerts:ABC123#key-1"),
        address,
    );
    let context = vec![
        env,
        String::from_slice(env, "https://www.w3.org/ns/did/v1"),
        String::from_slice(env, "https://www.example.com/context/v1"),
    ];
    let method = Method {
        type_: String::from_slice(env, "otp"),
        verified: true,
        timestamp: 1684872059,
        service: OptionMethodService::None,
    };
    let verification_processes = vec![env, method];
    let service = Service {
        type_: String::from_slice(env, "VerifiableCredential"),
        service_endpoint: String::from_slice(env, "https://did.chaincerts.co/ABC123"),
    };
    let services = vec![env, service];
    let public_add_cap: Option<CapabilityInvocation> = Option::None;

    vec![
        env,
        RawVal::from_val(env, &id),
        RawVal::from_val(env, &authentication_params),
        RawVal::from_val(env, &context),
        RawVal::from_val(env, &verification_processes),
        RawVal::from_val(env, &services),
        RawVal::from_val(env, &public_add_cap),
    ]
}
