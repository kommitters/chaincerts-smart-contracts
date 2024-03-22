use crate::verifiable_credential::VerifiableCredential;
use soroban_sdk::{Address, BytesN, Env, String, Val, Vec};

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

    /// Retrieves the vcs.
    fn get_vcs(e: Env) -> Vec<VerifiableCredential>;

    /// Upgrades WASM code.
    fn upgrade(e: Env, new_wasm_hash: BytesN<32>);

    /// Returns the version of the contract.
    fn version(e: Env) -> String;
}
