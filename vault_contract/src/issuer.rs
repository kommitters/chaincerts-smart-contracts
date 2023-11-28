use crate::error::ContractError;
use crate::storage;
use soroban_sdk::{contracttype, panic_with_error, Address, Env, Map, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Issuer {
    pub public_key: Address,
    pub is_revoked: bool,
}

pub fn authorize_issuer(e: &Env, issuer_pk: &Address, did: &String) {
    let mut issuers: Map<Address, Issuer> = storage::read_issuers(e, did);

    issuers.set(
        issuer_pk.clone(),
        Issuer {
            public_key: issuer_pk.clone(),
            is_revoked: false,
        },
    );

    storage::write_issuers(e, &issuers, did);
}

pub fn revoke_issuer(e: &Env, issuer: &Address, did: &String) {
    let mut issuers = storage::read_issuers(e, did);

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

    storage::write_issuers(e, &issuers, did);
}

pub fn is_registered(issuers: &Map<Address, Issuer>, issuer: &Address) -> bool {
    issuers.contains_key(issuer.clone())
}

pub fn is_revoked(issuers: &Map<Address, Issuer>, issuer: &Address) -> bool {
    issuers.get_unchecked(issuer.clone()).is_revoked
}
