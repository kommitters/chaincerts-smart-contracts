use crate::contract::{VaultContract, VaultContractClient};
use soroban_sdk::{testutils::Address as _, vec, Address, Env, String, Vec};

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
        let admin = Address::random(&env);
        let did = String::from_slice(&env, "did:chaincerts:5ppl9sm47frl0tpj7g3lp6eo");
        let dids = vec![&env, did.clone()];
        let issuer = Address::random(&env);

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
}

pub fn get_vc_setup(env: &Env) -> VCVaultContractTest {
    let vc_id = String::from_slice(env, "vc_id");
    let vc_data = String::from_slice(env, "vc_data");
    let issuance_contract_address = Address::random(env);
    VCVaultContractTest {
        vc_id,
        vc_data,
        issuance_contract_address,
    }
}
