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
    verim_ledger::verim::build_msg_create_nym(did, creator, verkey, alias, role).wait()
}

pub fn parse_msg_create_nym_resp(commit_resp: &str) -> Result<String, IndyError> {
    verim_ledger::verim::parse_msg_create_nym_resp(commit_resp).wait()
}

pub fn build_msg_update_nym(
    did: &str,
    creator: &str,
    verkey: &str,
    alias: &str,
    role: &str,
    id: u64,
) -> Result<Vec<u8>, IndyError> {
    verim_ledger::verim::build_msg_update_nym(did, creator, verkey, alias, role, id).wait()
}

pub fn parse_msg_update_nym_resp(commit_resp: &str) -> Result<String, IndyError> {
    verim_ledger::verim::parse_msg_update_nym_resp(commit_resp).wait()
}

pub fn build_msg_delete_nym(creator: &str, id: u64) -> Result<Vec<u8>, IndyError> {
    verim_ledger::verim::build_msg_delete_nym(creator, id).wait()
}

pub fn parse_msg_delete_nym_resp(commit_resp: &str) -> Result<String, IndyError> {
    verim_ledger::verim::parse_msg_delete_nym_resp(commit_resp).wait()
}

pub fn build_query_get_nym(id: u64) -> Result<String, IndyError> {
    verim_ledger::verim::build_query_get_nym(id).wait()
}

pub fn parse_query_get_nym_resp(query_resp: &str) -> Result<String, IndyError> {
    verim_ledger::verim::parse_query_get_nym_resp(query_resp).wait()
}

pub fn build_query_all_nym() -> Result<String, IndyError> {
    verim_ledger::verim::build_query_all_nym().wait()
}

pub fn parse_query_all_nym_resp(query_resp: &str) -> Result<String, IndyError> {
    verim_ledger::verim::parse_query_all_nym_resp(query_resp).wait()
}
