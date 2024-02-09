use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    AlreadyInitialized = 1,
    NotAuthorized = 2,
    AmountLimitExceeded = 3,
    VCNotFound = 4,
    VCAlreadyRevoked = 5,
    IssuanceLimitExceeded = 6,
}
