use soroban_sdk::{Address, Env, String, Vec};

pub trait VaultTrait {
    /// Initializes the Vault Contract by setting the admin and the did.
    fn initialize(e: Env, admin: Address, dids: Vec<String>);

    /// Authorize an issuer creating a list of issuers and storing them.
    fn authorize_issuer(e: Env, admin: Address, issuer: Address);

    /// Revoke an issuer deleting it from the list of issuers.
    fn revoke_issuer(e: Env, admin: Address, issuer: Address);
}
