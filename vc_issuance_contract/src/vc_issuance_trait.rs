use soroban_sdk::{Address, Env, String};

pub trait VCIssuanceTrait {
    /// Initializes the Verifiable Credentials Issuance Contract by setting the admin and an optional amount.
    fn initialize(e: Env, admin: Address, amount: Option<u32>);

    /// Issues a new Verifiable Credential and returns the Verifiable Credential id
    fn issue(
        e: Env,
        admin: Address,
        recipient_did: String,
        vc_data: String,
        storage_address: Address,
    ) -> String;
}
