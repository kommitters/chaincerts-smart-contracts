use crate::contract::{VaultContract, VaultContractClient};
use crate::did_contract::{
    self, Service, ServiceType, VerificationMethodEntry, VerificationMethodType,
    VerificationRelationship,
};
use soroban_sdk::{testutils::Address as _, vec, Address, BytesN, Env, FromVal, String, Val, Vec};

pub struct VaultContractTest<'a> {
    pub env: Env,
    pub admin: Address,
    pub did: String,
    pub dids: Vec<String>,
    pub issuer: Address,
    pub contract: VaultContractClient<'a>,
}

impl<'a> VaultContractTest<'a> {
    pub fn setup() -> Self {
        let env: Env = Default::default();
        env.mock_all_auths();
        let admin = Address::generate(&env);
        let did = String::from_str(&env, "did:chaincerts:5ppl9sm47frl0tpj7g3lp6eo");
        let dids = vec![&env, did.clone()];
        let issuer = Address::generate(&env);

        let contract = VaultContractClient::new(&env, &env.register_contract(None, VaultContract));
        VaultContractTest {
            env,
            admin,
            did,
            dids,
            issuer,
            contract,
        }
    }
}

pub struct VCVaultContractTest {
    pub vc_id: String,
    pub vc_data: String,
    pub issuance_contract_address: Address,
    pub issuer_did: String,
}

pub fn get_vc_setup(env: &Env) -> VCVaultContractTest {
    let vc_id = String::from_str(env, "vc_id");
    let vc_data = String::from_str(env, "vc_data");
    let issuance_contract_address = Address::generate(env);
    let issuer_did = String::from_str(env, "did:chaincerts:7dotwpyzo2weqj6oto6liic6");

    VCVaultContractTest {
        vc_id,
        vc_data,
        issuance_contract_address,
        issuer_did,
    }
}

pub struct DIDContractTest {
    pub did_init_args: Vec<Val>,
    pub did_wasm_hash: BytesN<32>,
    pub salt: BytesN<32>,
    pub context: Vec<String>,
}

pub fn get_did_contract_setup(env: &Env, admin: &Address) -> DIDContractTest {
    let salt = BytesN::from_array(env, &[0; 32]);
    let did_wasm_hash = env.deployer().upload_contract_wasm(did_contract::WASM);

    DIDContractTest {
        did_init_args: did_init_args(env, admin),
        did_wasm_hash,
        salt,
        context: did_context(env),
    }
}

pub fn did_init_args(e: &Env, admin: &Address) -> Vec<Val> {
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

    let did_method = String::from_str(e, "chaincerts");
    vec![
        e,
        Val::from_val(e, &admin),
        Val::from_val(e, &did_method),
        Val::from_val(e, &did_context(e)),
        Val::from_val(e, &verification_methods),
        Val::from_val(e, &services),
    ]
}

fn did_context(e: &Env) -> Vec<String> {
    vec![
        e,
        String::from_str(e, "https://www.w3.org/ns/did/v1"),
        String::from_str(e, "https://w3id.org/security/suites/ed25519-2020/v1"),
        String::from_str(e, "https://w3id.org/security/suites/x25519-2020/v1"),
    ]
}
