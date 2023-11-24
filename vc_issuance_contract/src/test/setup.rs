use crate::contract::{VCIssuanceContract, VCIssuanceContractClient};
use soroban_sdk::{testutils::Address as _, Address, Env};

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
