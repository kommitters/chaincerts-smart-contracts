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
    VerifiableCredentialNotFound = 10,
    NoVerifiableCredential = 11,
    CannotRemoveAuthentication = 12,
    CannotRemoveVerificationMethod = 13,
    InvalidCapabilityInvocation = 14,
}
