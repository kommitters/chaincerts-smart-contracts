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
    ChaincertAlreadyInWallet = 9,
    ChaincertNotFound = 10,
    NoChaincerts = 11,
}
