use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    AlreadyInitialized = 1,
    VCNotFound = 3,
    VCAlreadyRevoked = 4,
    IssuanceLimitExceeded = 5,
}
