use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    AlreadyInitialized = 1,
    NotAuthorized = 2,
    IssuerNotAuthorized = 3,
    IssuerAlreadyAuthorized = 4,
    VaultRevoked = 5,
}
