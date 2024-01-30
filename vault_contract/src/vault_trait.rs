use crate::vault::Vault;
use soroban_sdk::{Address, BytesN, Env, String, Val, Vec};

pub trait VaultTrait {
    /// Initializes the vault contract by setting the admin and creating a vault for each DID.
    fn initialize(e: Env, admin: Address, dids: Vec<String>);

    /// Authorizes a list of issuers for a vault.
    fn authorize_issuers(e: Env, admin: Address, issuers: Vec<Address>, did: String);

    /// Authorizes an issuer for a vault.
    fn authorize_issuer(e: Env, admin: Address, issuer: Address, did: String);

    /// Revokes an issuer for a vault.
    fn revoke_issuer(e: Env, admin: Address, issuer: Address, did: String);

    /// Stores a verifiable credential in the recipient's vault.
    fn store_vc(
        e: Env,
        vc_id: String,
        vc_data: String,
        recipient_did: String,
        issuer: Address,
        issuer_did: String,
        issuance_contract: Address,
    );

    /// Deploys a DID and registers a vault using the deployed DID.
    fn register_vault(
        e: Env,
        admin: Address,
        did_wasm_hash: BytesN<32>,
        did_init_args: Vec<Val>,
        salt: BytesN<32>,
    ) -> (Address, Val);

    /// Registers a vault given its DID.
    fn register_vault_with_did(e: Env, admin: Address, did: String);

    /// Revokes a vault given its DID.
    fn revoke_vault(e: Env, admin: Address, did: String);

    /// Retrieves a vault given its DID.
    fn get_vault(e: Env, did: String) -> Vault;

    /// Retrieves the list of vaults.
    fn list_vaults(e: Env) -> Vec<Vault>;
}
