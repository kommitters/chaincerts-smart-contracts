use soroban_sdk::{Address, BytesN, Env, String, Val, Vec};

#[allow(dead_code)]
pub trait VaultTrait {
    /// Initializes the vault contract by setting the admin and deploying the DID.
    fn initialize(
        e: Env,
        admin: Address,
        did_wasm_hash: BytesN<32>,
        did_init_args: Vec<Val>,
        salt: BytesN<32>,
    ) -> (Address, Val);

    /// Authorizes a list of issuers.
    fn authorize_issuers(e: Env, issuers: Vec<Address>);

    /// Authorizes an issuer for a vault.
    fn authorize_issuer(e: Env, issuer: Address);

    /// Revokes an issuer for a vault.
    fn revoke_issuer(e: Env, issuer: Address);

    /// Stores a verifiable credential in the vault.
    fn store_vc(
        e: Env,
        vc_id: String,
        vc_data: String,
        issuer: Address,
        issuer_did: String,
        issuance_contract: Address,
    );

    /// Revokes the vault.
    fn revoke_vault(e: Env);

    /// Migrates the VCs from being stored in a single vector to multiple vectors.
    fn migrate(e: Env);

    /// Sets the new contract admin.
    fn set_admin(e: Env, new_admin: Address);

    /// Upgrades WASM code.
    fn upgrade(e: Env, new_wasm_hash: BytesN<32>);

    /// Returns the version of the contract.
    fn version(e: Env) -> String;
}
