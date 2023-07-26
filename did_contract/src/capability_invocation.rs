//! Module CapabilityInvocation
//!
//! Module responsible of managing the CapabilityInvocation list
//! that specifies allowed actions.
use soroban_sdk::{contracttype, panic_with_error, Address, Env, String, Vec};

use crate::{
    error::{ContractError, DIDContractError},
    option::{OptionAddress, OptionString},
};

use super::storage_types::DataKey;

const CAP_INVOCATION_KEY: DataKey = DataKey::CapabilityInvocation;

#[derive(Clone, PartialEq, Eq, Debug)]
#[contracttype]
/// The DID `CapabilityInvocation` information
pub struct CapabilityInvocation {
    pub id: String,
    pub type_: CapType,
    pub invoker: OptionString,
    pub invoker_address: OptionAddress,
    pub credential: OptionString,
}

#[contracttype]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum CapType {
    PublicAdd,
    AddCredential,
    ReadCredential,
    PublicRead,
}

pub(crate) fn write_capability_invocation(env: &Env, cap_list: &Vec<CapabilityInvocation>) {
    env.storage().instance().set(&CAP_INVOCATION_KEY, cap_list)
}

pub(crate) fn read_capability_invocation(env: &Env) -> Vec<CapabilityInvocation> {
    env.storage().instance().get(&CAP_INVOCATION_KEY).unwrap()
}

pub(crate) fn write_public_add_cap(env: &Env, public_add: bool) {
    env.storage()
        .instance()
        .set(&DataKey::PublicAdd, &public_add)
}

pub(crate) fn has_public_add_cap(env: &Env) -> bool {
    env.storage().instance().get(&DataKey::PublicAdd).unwrap()
}

pub(crate) fn add_capability(env: &Env, capability: &CapabilityInvocation) {
    let mut cap_invocation: Vec<CapabilityInvocation> =
        env.storage().instance().get(&CAP_INVOCATION_KEY).unwrap();

    if is_valid_capability(env, capability)
        && !is_capability_in_list(&capability.id, &cap_invocation)
    {
        cap_invocation.push_front(capability.clone());
        if capability.type_ == CapType::PublicAdd {
            write_public_add_cap(env, true);
        }
        write_capability_invocation(env, &cap_invocation);
    } else {
        panic_with_error!(env, ContractError::AlreadyInCapInvocation)
    }
}

pub(crate) fn remove_capability(env: &Env, cap_id: &String) {
    let mut cap_list: Vec<CapabilityInvocation> =
        env.storage().instance().get(&CAP_INVOCATION_KEY).unwrap();
    if cap_list.is_empty() {
        panic_with_error!(env, ContractError::NoCapabilityInvocation);
    }
    remove_from_cap_invocation_list(env, cap_id, &mut cap_list);
    env.storage().instance().set(&CAP_INVOCATION_KEY, &cap_list)
}

pub(crate) fn check_capability_to_deposit(env: &Env, invoker: &String) -> OptionAddress {
    let cap_invocation: Vec<CapabilityInvocation> =
        env.storage().instance().get(&CAP_INVOCATION_KEY).unwrap();

    for cap in cap_invocation.iter() {
        if let OptionString::Some(cap_invoker) = cap.invoker {
            if cap_invoker == invoker.clone() && cap.type_ == CapType::AddCredential {
                return cap.invoker_address;
            }
        }
    }
    panic_with_error!(env, ContractError::NotAuthorized)
}

pub(crate) fn check_capability_to_read_credentials(env: &Env, address: &Address, invoker: &String) {
    let cap_list: Vec<CapabilityInvocation> =
        env.storage().instance().get(&CAP_INVOCATION_KEY).unwrap();
    for cap in cap_list.iter() {
        if let (OptionString::Some(cap_invoker), OptionAddress::Some(cap_address)) =
            (cap.invoker, cap.invoker_address)
        {
            if cap_address == address.clone()
                && cap_invoker == invoker.clone()
                && cap.type_ == CapType::ReadCredential
            {
                return;
            }
        }
    }
    panic_with_error!(env, ContractError::NotAuthorized)
}

pub(crate) fn is_valid_capability(env: &Env, capability_invocation: &CapabilityInvocation) -> bool {
    match capability_invocation.type_ {
        CapType::PublicAdd => {
            if capability_invocation.invoker == OptionString::None
                && capability_invocation.invoker_address == OptionAddress::None
                && capability_invocation.credential == OptionString::None
            {
                return true;
            }
        }
        CapType::AddCredential => {
            if capability_invocation.invoker != OptionString::None
                && capability_invocation.invoker_address != OptionAddress::None
                && capability_invocation.credential == OptionString::None
            {
                return true;
            }
        }
        CapType::ReadCredential => {
            if capability_invocation.invoker != OptionString::None
                && capability_invocation.invoker_address != OptionAddress::None
                && capability_invocation.credential != OptionString::None
            {
                return true;
            }
        }
        CapType::PublicRead => {
            if capability_invocation.invoker == OptionString::None
                && capability_invocation.invoker_address == OptionAddress::None
                && capability_invocation.credential != OptionString::None
            {
                return true;
            }
        }
    }

    panic_with_error!(env, DIDContractError::InvalidCapabilityInvocation)
}

fn remove_from_cap_invocation_list(
    env: &Env,
    cap_id: &String,
    cap_invocation: &mut Vec<CapabilityInvocation>,
) {
    let index = cap_invocation
        .iter()
        .position(|cap| cap.id == cap_id.clone());
    match index {
        Some(val) => {
            let cap = cap_invocation.get_unchecked(val as u32);
            if cap.type_ == CapType::PublicAdd {
                write_public_add_cap(env, false);
            }
            cap_invocation.remove(val as u32).unwrap();
        }
        None => panic_with_error!(env, ContractError::CapabilityInvocationNotFound),
    }
}

fn is_capability_in_list(cap_id: &String, cap_invocation: &Vec<CapabilityInvocation>) -> bool {
    cap_invocation.iter().any(|cap| cap.id == cap_id.clone())
}
