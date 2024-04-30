use soroban_sdk::{Address, BytesN, Env, Map, String};

pub trait VCIssuanceTrait {
    /// Initializes the Verifiable Credentials Issuance Contract by setting the admin and the issuer_did.
    fn initialize(e: Env, admin: Address, issuer_did: String);

    /// Issues a new Verifiable Credential and returns the Verifiable Credential id
    fn issue(e: Env, vc_id: String, vc_data: String, vault_contract: Address) -> String;

    /// Verifies if the Verifiable Credential is not revoked
    fn verify(e: Env, vc_id: String) -> Map<String, String>;

    /// Revokes a Verifiable Credential
    fn revoke(e: Env, vc_id: String, date: String);

    /// Sets the new contract admin.
    fn set_admin(e: Env, new_admin: Address);

    /// Upgrades WASM code.
    fn upgrade(e: Env, new_wasm_hash: BytesN<32>);

    /// Returns the version of the contract.
    fn version(e: Env) -> String;
}
