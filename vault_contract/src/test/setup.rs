use crate::contract::{VaultContract, VaultContractClient};
use soroban_sdk::{testutils::Address as _, vec, Address, Env, String, Vec};

pub struct VaultContractTest<'a> {
    pub env: Env,
    pub admin: Address,
    pub dids: Vec<String>,
    pub issuer: Address,
    pub contract: VaultContractClient<'a>,
}

impl<'a> VaultContractTest<'a> {
    pub fn setup() -> Self {
        let env: Env = Default::default();
        env.mock_all_auths();
        let admin = Address::random(&env);
        let dids = vec![
            &env,
            (String::from_slice(&env, "did:chaincerts:5ppl9sm47frl0tpj7g3lp6eo")),
        ];
        let issuer = Address::random(&env);

        let contract = VaultContractClient::new(&env, &env.register_contract(None, VaultContract));
        VaultContractTest {
            env,
            admin,
            dids,
            issuer,
            contract,
        }
    }
}
