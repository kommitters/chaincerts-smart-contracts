//! Module StorageTypes
//!
//! Module that defines the set of keys that can be used to access and store data within the contract.
use soroban_sdk::contracttype;

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Id,
    /// Vec<String> with the authentication keys
    Authentication,
    /// Vec<VerificationMethod> with the verification methods of the DID
    VerificationMethods,
    /// Capability Invocation List
    CapabilityInvocation,
    /// A map that stores the VerifiableCredential, identified by a credential_id `Map<String, VerifiableCredential>`
    VerifiableCredential,
    /// Vec<String> that stores DID context urls
    Context,
    /// Vec<Method> that stores DID verification processes
    VerificationProcesses,
    /// Vec<Service> that stored DID services
    Services,
    /// Stores the DID `Metadata`
    Metadata,
}
