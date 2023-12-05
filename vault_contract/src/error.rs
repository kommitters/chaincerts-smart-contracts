use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    AlreadyInitialized = 1,
    NotAuthorized = 2,
    EmptyDIDs = 3,
    IssuerNotFound = 4,
    IssuerRevoked = 5,
    VaultNotFound = 6,
    VaultRevoked = 7,
    VaultAlreadyRegistered = 8,
}
