use soroban_sdk::{contracttype, Address};

// Service types are defined in https://www.w3.org/TR/did-spec-registries/#service-types.

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Issuer {
    pub public_key: Address,
    pub is_revoked: bool,
}
