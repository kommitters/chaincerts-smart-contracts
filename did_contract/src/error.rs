use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    AlreadyInit = 1,
    NotAuthorized = 2,
    AlreadyInCapInvocation = 4,
    NoCapabilityInvocation = 6,
    CapabilityInvocationNotFound = 8,
    VerifiableCredentialAlreadyInDID = 9,
}

/// This is temporally since Error codes fails with error(Error(Value, InvalidInput)\nDebugInfo not available\n) when identifier > 9
/// https://github.com/stellar/rs-soroban-env/issues/953
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum DIDContractError {
    VerifiableCredentialNotFound = 0,
    NoVerifiableCredential = 1,
    CannotRemoveAuthentication = 2,
    CannotRemoveVerificationMethod = 3,
    InvalidCapabilityInvocation = 4,
}
