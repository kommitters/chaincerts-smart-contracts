use self::did_contract::{
    Service, ServiceType, VerificationMethodEntry, VerificationMethodType, VerificationRelationship,
};
use crate::contract::{VCIssuanceContract, VCIssuanceContractClient};
use soroban_sdk::{
    map, testutils::Address as _, vec, Address, BytesN, Env, FromVal, Map, String, Val, Vec,
};

mod did_contract {
    soroban_sdk::contractimport!(
        file = "../target/wasm32-unknown-unknown/release/soroban_did_contract.wasm"
    );
}
mod vault_contract {
    soroban_sdk::contractimport!(
        file = "../target/wasm32-unknown-unknown/release/vault_contract.wasm"
    );
}

pub struct VCIssuanceContractTest<'a> {
    pub env: Env,
    pub admin: Address,
    pub amount: Option<u32>,
    pub vc_id: String,
    pub vc_data: String,
    pub issuer_did: String,
    pub contract: VCIssuanceContractClient<'a>,
}

impl<'a> VCIssuanceContractTest<'a> {
    pub fn setup() -> Self {
        let env: Env = Default::default();
        env.mock_all_auths();
        let admin = Address::generate(&env);
        let contract =
            VCIssuanceContractClient::new(&env, &env.register_contract(None, VCIssuanceContract));
        let amount = Some(10);
        let vc_id = String::from_str(&env, "iwvkdjquj3fscmafrgeeqblw");
        let vc_data = String::from_str(&env, "eoZXggNeVDW2g5GeA0G2s0QJBn3SZWzWSE3fXM9V6IB5wWIfFJRxPrTLQRMHulCF62bVQNmZkj7zbSa39fVjAUTtfm6JMio75uMxoDlAN/Y");
        let issuer_did = String::from_str(&env, "did:chaincerts:7dotwpyzo2weqj6oto6liic6");

        VCIssuanceContractTest {
            env,
            admin,
            amount,
            vc_id,
            vc_data,
            issuer_did,
            contract,
        }
    }
}

pub fn create_vc(
    env: &Env,
    admin: &Address,
    contract: &VCIssuanceContractClient,
    issuer_did: &String,
    amount: &Option<u32>,
) -> Address {
    let vault_admin = Address::generate(env);
    let vault_contract_address = env.register_contract_wasm(None, vault_contract::WASM);
    let vault_client = vault_contract::Client::new(env, &vault_contract_address);

    let did_wasm_hash = env.deployer().upload_contract_wasm(did_contract::WASM);
    let did_init_args = build_did_init_args(env, admin);
    let salt = BytesN::from_array(env, &[0; 32]);

    vault_client.initialize(&vault_admin, &did_wasm_hash, &did_init_args, &salt);
    vault_client.authorize_issuer(admin);

    contract.initialize(admin, issuer_did, amount);
    vault_contract_address
}

pub fn get_revoked_vc_map(env: &Env, date: String) -> Map<String, String> {
    let status_str = String::from_str(env, "status");
    let since_str = String::from_str(env, "since");
    let revoked_str = String::from_str(env, "revoked");

    map![env, (status_str, revoked_str), (since_str, date)]
}

pub fn get_valid_vc_map(env: &Env) -> Map<String, String> {
    let status_str = String::from_str(env, "status");
    let valid_str = String::from_str(env, "valid");

    map![env, (status_str, valid_str)]
}

fn build_did_init_args(e: &Env, admin: &Address) -> Vec<Val> {
    let verification_methods = vec![
        e,
        VerificationMethodEntry {
            id: String::from_str(e, "keys-1"),
            type_: VerificationMethodType::Ed25519VerificationKey2020,
            public_key_multibase: String::from_str(
                e,
                "z6MkgpAN9rsVPXJ6DrrvxcsGzKwjdkVdvjNtbQsRiLfsqmuQ",
            ),
            controller: String::from_str(e, ""),
            verification_relationships: vec![
                e,
                VerificationRelationship::Authentication,
                VerificationRelationship::AssertionMethod,
            ],
        },
    ];

    let services: Vec<Service> = vec![
        &e,
        Service {
            id: String::from_str(e, "chaincerts"),
            type_: ServiceType::LinkedDomains,
            service_endpoint: String::from_str(e, "https://chaincerts.co"),
        },
    ];

    let context: Vec<String> = vec![
        e,
        String::from_str(e, "https://www.w3.org/ns/did/v1"),
        String::from_str(e, "https://w3id.org/security/suites/ed25519-2020/v1"),
        String::from_str(e, "https://w3id.org/security/suites/x25519-2020/v1"),
    ];

    let did_method = String::from_str(e, "chaincerts");
    vec![
        e,
        Val::from_val(e, &admin),
        Val::from_val(e, &did_method),
        Val::from_val(e, &context),
        Val::from_val(e, &verification_methods),
        Val::from_val(e, &services),
    ]
}
