use crate::vault::Vault;
use soroban_sdk::{Address, Env, String, Vec};

pub trait VaultTrait {
    /// Initializes the Vault Contract by setting the admin and the initial DIDs.
    fn initialize(e: Env, admin: Address, dids: Vec<String>);

    /// Authorizes an issuer adding it to the issuers map.
    fn authorize_issuer(e: Env, admin: Address, issuer: Address, did: String);

    /// Revokes an issuer setting its is_revoked property to true.
    fn revoke_issuer(e: Env, admin: Address, issuer: Address, did: String);

    /// Stores the verifiable credential.
    fn store_vc(
        e: Env,
        vc_id: String,
        vc_data: String,
        recipient_did: String,
        issuer: Address,
        issuance_contract: Address,
    );

    /// Retrieves a vault using its unique identifier.
    fn get_vault(e: Env, vc_id: String) -> Vault;

    /// Retrieves all the vaults in a vec.
    fn list_vaults(e: Env) -> Vec<Vault>;

    /// Revokes a vault given its DID URI.
    fn revoke_vault(e: Env, admin: Address, did: String);

    /// Registers a new vault given a DID URI.
    fn register_vault(e: Env, admin: Address, did: String);
}
