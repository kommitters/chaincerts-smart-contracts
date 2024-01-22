use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    AlreadyInitialized = 1,
    NotAuthorized = 2,
    EmptyDIDs = 3,
    IssuerNotAuthorized = 4,
    VaultNotFound = 5,
    VaultRevoked = 6,
    VaultAlreadyRegistered = 7,
}
