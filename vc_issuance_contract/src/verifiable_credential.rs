use soroban_sdk::{Env, String};

use crate::base32;
use crate::storage;

pub fn add_vc(e: &Env, vc_id: &String) {
    let mut vcs = storage::read_vcs(e);
    vcs.push_front(vc_id.clone());

    storage::write_vcs(e, &vcs);
}

pub fn generate_id(e: &Env) -> String {
    let random_bytes: [u8; 15] = get_random_bytes(e);
    let mut id = [0u8; 24];

    base32::encode(&mut id, &random_bytes);

    let str_id = core::str::from_utf8(id.as_ref()).unwrap();

    String::from_slice(e, str_id)
}

fn get_random_bytes(e: &Env) -> [u8; 15] {
    let mut random_bytes = [0u8; 15];

    for byte in &mut random_bytes {
        *byte = e.prng().u64_in_range(0..256) as u8;
    }

    random_bytes
}
