use crate::error::ContractError;
use crate::storage;

use crate::vc_issuance_trait::VCIssuanceTrait;
use soroban_sdk::{contract, contractimpl, contractmeta, panic_with_error, Address, Env, Map, Vec};

const LEDGERS_THRESHOLD: u32 = 1;
const LEDGERS_TO_EXTEND: u32 = 535_000;
const DEFAULT_AMOUNT: u32 = 20;
const MAX_AMOUNT: u32 = 100;

contractmeta!(
    key = "Description",
    val = "Smart Contract to issue, transfer, verify, and revoke Verifiable Credentials (VCs).",
);

#[contract]
pub struct VCIssuanceContract;

#[contractimpl]
impl VCIssuanceTrait for VCIssuanceContract {
    fn initialize(e: Env, admin: Address, amount: Option<u32>) {
        if storage::has_admin(&e) {
            panic_with_error!(e, ContractError::AlreadyInitialized);
        }
        if amount.map_or(false, |a| a > MAX_AMOUNT) {
            panic_with_error!(e, ContractError::AmountLimitExceeded);
        }

        storage::write_admin(&e, &admin);
        storage::write_amount(&e, &amount.unwrap_or(DEFAULT_AMOUNT));

        // set initial empty values
        storage::write_vcs(&e, &Vec::new(&e));
        storage::write_vcs_revocations(&e, &Map::new(&e));

        e.storage()
            .instance()
            .bump(LEDGERS_THRESHOLD, LEDGERS_TO_EXTEND);
    }
}
