#![allow(dead_code, unused_macros)]

use indyrs::IndyError;
use serde_json::Value;

use crate::utils::{verim_keys, verim_ledger, verim_ledger::auth, verim_pool};

use super::{logger, wallet, WalletHandle};
use super::test;

fn setup() -> String {
    let name = crate::utils::rand_utils::get_rand_string(10);
    test::cleanup_storage(&name);
    logger::set_default_logger();
    name
}

fn tear_down(name: &str, wallet_handle: WalletHandle) {
    wallet::close_wallet(wallet_handle).unwrap();
    test::cleanup_storage(name);
}

fn wallet_config(name: &str) -> String {
    json!({ "id": name }).to_string()
}

pub struct VerimSetup {
    pub name: String,
    pub pool_alias: String,
    pub key_alias: String,
    pub account_id: String,
    pub pub_key: String,
    pub wallet_handle: WalletHandle,
}

impl VerimSetup {
    pub fn new() -> VerimSetup {
        let name = setup();

        // Wallet
        let wallet_config = wallet_config(&name);
        let (wallet_handle, _) = wallet::create_and_open_default_wallet(&wallet_config).unwrap();

        // Account
        let key_alias = "alice";
        let (account_id, pub_key) = VerimSetup::create_key(wallet_handle, key_alias, "alice").unwrap();

        // Pool
        verim_pool::add(&name, "http://localhost:26657", "verimnode").unwrap();

        let setup = VerimSetup {
            name: name.clone(),
            pool_alias: name,
            key_alias: key_alias.to_string(),
            account_id,
            pub_key,
            wallet_handle,
        };

        setup
    }

    pub fn create_key(wallet_handle: WalletHandle, alias: &str, mnemonic: &str) -> Result<(String, String), IndyError> {
        let key = verim_keys::add_from_mnemonic(wallet_handle, alias, mnemonic).unwrap();
        let key: Value = serde_json::from_str(&key).unwrap();
        println!("Verim setup. Create key: {:?}", key);

        let account_id = key["account_id"].as_str().unwrap().to_string();
        let pub_key = key["pub_key"].as_str().unwrap().to_string();
        Ok((account_id, pub_key))
    }

    pub fn get_base_account_number_and_sequence(&self, account_id: &str) -> Result<(u64, u64), IndyError> {
        let req = auth::build_query_account(account_id).unwrap();
        let resp = verim_pool::abci_query(&self.pool_alias, &req).unwrap();
        let resp = auth::parse_query_account_resp(&resp).unwrap();
        println!("Verim setup. Get account: {:?}", resp);

        let resp: Value = serde_json::from_str(&resp).unwrap();
        let account = resp["account"].as_object().unwrap();
        let base_account = account["base_account"].as_object().unwrap();
        let account_number = base_account["account_number"].as_u64().unwrap();
        let account_sequence = base_account["sequence"].as_u64().unwrap();

        Ok((account_number, account_sequence))
    }

    pub fn build_and_sign_and_broadcast_tx(&self, msg: &[u8]) -> Result<String, IndyError> {
        // Get account info
        let (account_number, account_sequence) = self.get_base_account_number_and_sequence(&self.account_id)?;

        // Tx
        // TODO: Set correct timeout height using abci info query
        let tx = verim_ledger::auth::build_tx(
            &self.pool_alias, &self.pub_key, &msg, account_number, account_sequence, 300000, 0u64, "token", 39090, "memo",
        )?;

        // Sign
        let signed = verim_keys::sign(self.wallet_handle, &self.key_alias, &tx)?;

        // Broadcast
        let resp = verim_pool::broadcast_tx_commit(&self.pool_alias, &signed)?;

        Ok(resp)
    }
}

impl Drop for VerimSetup {
    fn drop(&mut self) {
        tear_down(&self.name, self.wallet_handle);
    }
}
