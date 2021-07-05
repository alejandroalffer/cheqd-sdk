use indy::IndyError;
use indy::future::Future;
use indy::{WalletHandle};
use indy::verim_keys;

pub struct VerimPool {}

impl VerimPool {
    pub fn add_random(wallet_handle: WalletHandle, alias: &str) -> Result<String, IndyError> {
        verim_keys::add_random(wallet_handle, alias).wait()
    }

    pub fn add_from_mnemonic(wallet_handle: WalletHandle, alias: &str, mnemonic: &str) -> Result<String, IndyError> {
        verim_keys::add_from_mnemonic(wallet_handle, alias, mnemonic).wait()
    }

    pub fn get_info(wallet_handle: WalletHandle, alias: &str) -> Result<String, IndyError> {
        verim_keys::get_info(wallet_handle, alias).wait()
    }

    pub fn sign(wallet_handle: WalletHandle, alias: &str, tx: &[u8]) -> Result<Vec<u8>, IndyError> {
        verim_keys::sign(wallet_handle, alias, tx).wait()
    }
}