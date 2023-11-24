use soroban_sdk::{Address, Env, String};

pub trait VCsTrait {
    /// Initializes the Verifiable Credentials by setting the admin and an optional amount.
    fn initialize(e: Env, admin: Address, amount: Option<u32>);

    fn issue(
        e: Env,
        admin: Address,
        recipient_did: String,
        vc_data: String,
        storage_address: Address,
    ) -> String;
}
