use soroban_sdk::{Address, Env, String, Vec};

pub trait VaultTrait {
    /// Initializes the Vault Contract by setting the admin and the did.
    fn initialize(e: Env, admin: Address, dids: Vec<String>);

    /// Authorize an issuer adding it to the issuers map.
    fn authorize_issuer(e: Env, admin: Address, issuer: Address);

    /// Revoke an issuer setting its is_revoked property to true.
    fn revoke_issuer(e: Env, admin: Address, issuer: Address);
}
