use soroban_sdk::{Env, String, Vec};

use crate::storage;

pub fn add_vc(e: &Env, vc_id: &String, mut vcs: Vec<String>) {
    vcs.push_front(vc_id.clone());

    storage::write_vcs(e, &vcs);
}
