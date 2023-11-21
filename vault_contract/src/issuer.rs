use crate::{error::ContractError, storage};
use soroban_sdk::{contracttype, panic_with_error, Address, Env, Map};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Issuer {
    pub public_key: Address,
    pub is_revoked: bool,
}

pub fn add_issuer_to_issuers_map(e: &Env, issuer: &Address) -> Map<Address, Issuer> {
    let mut issuers = storage::read_issuers(e);

    issuers.set(
        issuer.clone(),
        Issuer {
            public_key: issuer.clone(),
            is_revoked: false,
        },
    );

    issuers
}

pub fn revoke_issuer_to_issuers_map(e: &Env, issuer: &Address) -> Map<Address, Issuer> {
    let mut issuers = storage::read_issuers(e);

    if issuers.contains_key(issuer.clone()) {
        issuers.set(
            issuer.clone(),
            Issuer {
                public_key: issuer.clone(),
                is_revoked: true,
            },
        )
    } else {
        panic_with_error!(e, ContractError::IssuerNotFound)
    }

    issuers
}
