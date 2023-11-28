use crate::contract::{VCIssuanceContract, VCIssuanceContractClient};
use crate::vault_contract;
use soroban_sdk::{map, testutils::Address as _, vec, Address, Env, Map, String};

pub struct VCIssuanceContractTest<'a> {
    pub env: Env,
    pub admin: Address,
    pub amount: Option<u32>,
    pub vc_data: String,
    pub recipient_did: String,
    pub contract: VCIssuanceContractClient<'a>,
}

impl<'a> VCIssuanceContractTest<'a> {
    pub fn setup() -> Self {
        let env: Env = Default::default();
        env.mock_all_auths();
        let admin = Address::random(&env);
        let contract =
            VCIssuanceContractClient::new(&env, &env.register_contract(None, VCIssuanceContract));
        let amount = Some(10);
        let vc_data = String::from_slice(&env, "eoZXggNeVDW2g5GeA0G2s0QJBn3SZWzWSE3fXM9V6IB5wWIfFJRxPrTLQRMHulCF62bVQNmZkj7zbSa39fVjAUTtfm6JMio75uMxoDlAN/Y");
        let recipient_did = String::from_slice(&env, "did:chaincerts:pe4t2r94dftr1n1gf6jikt6a");

        VCIssuanceContractTest {
            env,
            admin,
            amount,
            vc_data,
            recipient_did,
            contract,
        }
    }
}

pub fn create_vc(
    env: &Env,
    admin: &Address,
    contract: &VCIssuanceContractClient,
    recipient_did: &String,
) -> Address {
    let vault_admin = Address::random(env);

    let vault_contract_id = env.register_contract_wasm(None, vault_contract::WASM);
    let vault_client = vault_contract::Client::new(env, &vault_contract_id);
    let dids = vec![env, recipient_did.clone()];

    vault_client.initialize(&vault_admin, &dids);
    vault_client.authorize_issuer(&vault_admin, admin, recipient_did);

    contract.initialize(admin, &None);
    vault_contract_id
}

pub fn get_revoked_vc_map(env: &Env, date: String) -> Map<String, String> {
    let status_str = String::from_slice(env, "status");
    let since_str = String::from_slice(env, "Since");
    let revoked_str = String::from_slice(env, "Revoked");

    map![env, (status_str, revoked_str), (since_str, date)]
}

pub fn get_valid_vc_map(env: &Env) -> Map<String, String> {
    let status_str = String::from_slice(env, "status");
    let valid_str = String::from_slice(env, "Valid");

    map![env, (status_str, valid_str)]
}
