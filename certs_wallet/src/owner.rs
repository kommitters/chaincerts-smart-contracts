//! Module Owner
//!
//! Module responsible of managing the wallet owner information.
use soroban_sdk::{Address, Env};

use crate::storage_types::DataKey;

const OWNER_KEY: DataKey = DataKey::Owner;

pub(crate) fn has_owner(env: &Env) -> bool {
    env.storage().has(&OWNER_KEY)
}

pub(crate) fn read_owner(env: &Env) -> Address {
    env.storage().get_unchecked(&OWNER_KEY).unwrap()
}

pub(crate) fn write_owner(env: &Env, owner: &Address) {
    env.storage().set(&OWNER_KEY, owner);
}
