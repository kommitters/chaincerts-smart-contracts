use crate::did::DidWithVCs;
use crate::verifiable_credential::VerifiableCredential;
use soroban_sdk::{Address, Env, Map, String, Vec};

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
        issuer_pk: Address,
        issuance_contract_address: Address,
    );

    /// Retrieves a verifiable credential using its unique identifier.
    fn get_vc(e: Env, vc_id: String) -> VerifiableCredential;

    /// Retrieves the list of verifiable credentials from the storage grouped by DID.
    fn list_vcs(e: Env) -> Map<String, DidWithVCs>;

    /// Revokes a DID given its DID URI.
    fn revoke_did(e: Env, admin: Address, did: String);

    /// Registers a new DID given a DID URI.
    fn register_did(e: Env, admin: Address, did: String);
}
