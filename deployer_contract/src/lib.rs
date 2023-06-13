#![no_std]

use soroban_sdk::{contractimpl, Address, BytesN, Env, RawVal, Symbol, Vec};

pub struct DeployerContract;

#[contractimpl]
impl DeployerContract {
    /// Deploy the contract WASM and after deployment invoke the `initialize` function
    /// of the contract with the given arguments.
    /// Returns the contract ID and result of the `initialize` function.
    pub fn deploy(
        env: Env,
        salt: BytesN<32>,
        wasm_id: BytesN<32>,
        init_args: Vec<RawVal>,
    ) -> (Address, RawVal) {
        // Deploy the contract using the installed WASM code with given hash.
        let contract_id = env.deployer().with_current_contract(&salt).deploy(&wasm_id);

        let init_fn = Symbol::new(&env, "initialize");
        // Invoke the init function with the given arguments.
        let res: RawVal = env.invoke_contract(&contract_id, &init_fn, init_args);

        (contract_id, res)
    }
}

mod test;
