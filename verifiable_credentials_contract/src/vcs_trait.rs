use soroban_sdk::{Address, Env};

pub trait VCsTrait {
    /// Initializes the Verifiable Credentials by setting the admin and an optional amount.
    fn initialize(e: Env, admin: Address, amount: Option<u32>);
}
