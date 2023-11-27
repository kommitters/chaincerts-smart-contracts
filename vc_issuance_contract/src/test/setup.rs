use crate::contract::{VCIssuanceContract, VCIssuanceContractClient};
use crate::vault_contract;
use soroban_sdk::{testutils::Address as _, vec, Address, Env, String};

pub struct VCIssuanceContractTest<'a> {
    pub env: Env,
    pub admin: Address,
    pub amount: Option<u32>,
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

        VCIssuanceContractTest {
            env,
            admin,
            amount,
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
