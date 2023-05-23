//! Module Chaincert
//!
//! Module responsible of managing `Chaincerts` information and defining its corresponding struct.
use crate::{error::ContractError, option::OptionU64, storage_types::DataKey};
use soroban_sdk::{contracttype, map, panic_with_error, Address, Bytes, Env, Map, Vec};

const CHAINCERT_KEY: DataKey = DataKey::Chaincerts;

#[derive(Clone, PartialEq, Eq, Debug)]
#[contracttype]
/// The `Chaincert` information stored in the wallet
pub struct Chaincert {
    pub attestation: Bytes,
    /// Address of the issuance contract that distributed the `Chaincert`
    pub distributor_contract: Address,
    /// The id of the organization that distributed the `Chaincert`
    pub org_id: Bytes,
    /// The distribution date in Unix Timestamp format
    pub issuance_date: u64,
    /// The expiration date in Unix Timestamp format
    pub expiration_date: OptionU64,
    /// A logical indicator that lets know if a `Chaincert` is revoked or not
    pub revoked: bool,
}

impl Chaincert {
    fn new(
        attestation: Bytes,
        distributor_contract: Address,
        org_id: Bytes,
        issuance_date: u64,
        expiration_date: OptionU64,
        revoked: bool,
    ) -> Chaincert {
        Chaincert {
            attestation,
            distributor_contract,
            org_id,
            issuance_date,
            expiration_date,
            revoked,
        }
    }
}

pub(crate) fn deposit_chaincert(
    env: &Env,
    chaincert_did: Bytes,
    attestation: Bytes,
    distributor_contract: Address,
    org_id: Bytes,
    issuance_date: u64,
    expiration_date: OptionU64,
) {
    let chaincert = Chaincert::new(
        attestation,
        distributor_contract,
        org_id,
        issuance_date,
        expiration_date,
        false,
    );

    let chaincerts = match env.storage().get(&CHAINCERT_KEY) {
        Some(credential_map) => {
            let mut credential_map: Map<Bytes, Chaincert> = credential_map.unwrap();
            if !credential_map.contains_key(chaincert_did.clone()) {
                credential_map.set(chaincert_did, chaincert);
                credential_map
            } else {
                panic_with_error!(env, ContractError::ChaincertAlreadyInWallet)
            }
        }
        None => {
            let map: Map<Bytes, Chaincert> = map![env, (chaincert_did, chaincert)];
            map
        }
    };
    write_chaincerts(env, &chaincerts)
}

pub(crate) fn revoke_chaincert(env: &Env, credential_did: &Bytes) {
    match env.storage().get(&CHAINCERT_KEY) {
        Some(credential_map) => {
            let mut credential_map: Map<Bytes, Chaincert> = credential_map.unwrap();
            revoke_chaincert_from_map(
                env,
                &mut credential_map,
                credential_did,
            );
            write_chaincerts(env, &credential_map);
        }
        None => {
            panic_with_error!(env, ContractError::NoChaincerts)
        }
    };
}

pub(crate) fn get_chaincerts(env: &Env) -> Vec<Chaincert> {
    read_chaincerts(env).values()
}

fn revoke_chaincert_from_map(
    env: &Env,
    credential_map: &mut Map<Bytes, Chaincert>,
    credential_did: &Bytes,
) {
    match credential_map.get(credential_did.clone()) {
        Some(chaincert) => {
            let mut chaincert = chaincert.unwrap();
            chaincert.revoked = true;
            credential_map.set(credential_did.clone(), chaincert);
        }
        None => panic_with_error!(env, ContractError::ChaincertNotFound),
    }
}

fn read_chaincerts(env: &Env) -> Map<Bytes, Chaincert> {
    match env.storage().get(&CHAINCERT_KEY) {
        Some(cc) => cc.unwrap(),
        None => panic_with_error!(env, ContractError::NoChaincerts),
    }
}

fn write_chaincerts(env: &Env, certs: &Map<Bytes, Chaincert>) {
    env.storage().set(&CHAINCERT_KEY, certs)
}
