use soroban_sdk::{Address, Env, Map, String};

pub trait VCIssuanceTrait {
    /// Initializes the Verifiable Credentials Issuance Contract by setting the admin, the issuer_did and an optional amount.
    fn initialize(e: Env, admin: Address, issuer_did: String, amount: Option<u32>);

    /// Issues a new Verifiable Credential and returns the Verifiable Credential id
    fn issue(e: Env, admin: Address, vc_data: String, vault_contract: Address) -> String;

    /// Verifies if the Verifiable Credential is not revoked
    fn verify(e: Env, vc_id: String) -> Map<String, String>;

    /// Revokes a Verifiable Credential
    fn revoke(e: Env, admin: Address, vc_id: String, date: String);
}
