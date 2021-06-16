use indyrs::{verim_keys, future::Future, IndyError, PoolHandle, WalletHandle};

use crate::utils::{constants::DEFAULT_METHOD_NAME, ledger, pool, types::ResponseType};

pub fn add_random(alias: &str) -> Result<String, IndyError> {
    verim_keys::add_random(alias).wait()
}

pub fn add_from_mnemonic(alias: &str, mnemonic: &str) -> Result<String, IndyError> {
    verim_keys::add_from_mnemonic(alias, mnemonic).wait()
}

pub fn get_info(alias: &str) -> Result<String, IndyError> {
    verim_keys::get_info(alias).wait()
}

pub fn sign(alias: &str, tx: &[u8]) -> Result<Vec<u8>, IndyError> {
    verim_keys::sign(alias, tx).wait()
}
