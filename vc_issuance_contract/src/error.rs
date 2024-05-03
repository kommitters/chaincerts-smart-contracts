use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    AlreadyInitialized = 1,
    VCNotFound = 2,
    VCAlreadyRevoked = 3,
    VCSAlreadyMigrated = 4,
}
