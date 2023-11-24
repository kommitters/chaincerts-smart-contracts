use soroban_sdk::{Address, Env};

pub trait VCIssuanceTrait {
    /// Initializes the Verifiable Credentials Issuance Contract by setting the admin and an optional amount.
    fn initialize(e: Env, admin: Address, amount: Option<u32>);
}
