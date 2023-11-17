use soroban_sdk::{contracttype, String};

// Service types are defined in https://www.w3.org/TR/did-spec-registries/#service-types.

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Did {
    pub did: String,
    pub is_revoked: bool,
}
