use crate::contract::{VCsContract, VCsContractClient};
use soroban_sdk::{testutils::Address as _, Address, Env,};

pub struct VCsContractTest<'a> {
    pub env: Env,
    pub admin: Address,
    pub contract: VCsContractClient<'a>,
}

impl<'a> VCsContractTest<'a> {
    pub fn setup() -> Self {
        let env: Env = Default::default();
        env.mock_all_auths();
        let admin = Address::random(&env);

        let contract = VCsContractClient::new(&env, &env.register_contract(None, VCsContract));
        VCsContractTest {
            env,
            admin,
            contract,
        }
    }
}
