use std::{mem, sync::Once};

use indy_utils::crypto::hash::hash;
use indyrs::{future::Future, ledger, verim_ledger, IndyError, PoolHandle, WalletHandle};
use lazy_static::lazy_static;

use crate::utils::{anoncreds, blob_storage, constants::*, did, pool, timeout, wallet};

pub fn build_msg_create_nym(
    did: &str,
    creator: &str,
    verkey: &str,
    alias: &str,
    role: &str,
) -> Result<Vec<u8>, IndyError> {
    verim_ledger::build_msg_create_nym(did, creator, verkey, alias, role).wait()
}

pub fn build_msg_update_nym(
    did: &str,
    creator: &str,
    verkey: &str,
    alias: &str,
    role: &str,
    id: u64,
) -> Result<Vec<u8>, IndyError> {
    verim_ledger::build_msg_update_nym(did, creator, verkey, alias, role, id).wait()
}

pub fn build_msg_delete_nym(creator: &str, id: u64) -> Result<Vec<u8>, IndyError> {
    verim_ledger::build_msg_delete_nym(creator, id).wait()
}
