use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    AlreadyInit = 1,
    NotAuthorized = 2,
    AlreadyInACL = 4,
    NoOrganizationsInACL = 6,
    OrganizationNotFound = 8,
    VerifiableCredentialAlreadyInWallet = 9,
    VerifiableCredentialNotFound = 10,
    NoVerifiableCredential = 11,
    CannotRemoveAuthentication = 12,
    CannotRemoveVerificationMethod = 13,
}
