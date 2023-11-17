#![no_std]
#![allow(dead_code)] // Added to remove warnings for unused functions

mod contract;
mod did;
mod error;
mod issuer;
mod storage;
mod vault_trait;

#[cfg(test)]
mod test;
