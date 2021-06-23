#![allow(dead_code, unused_macros)]
use crate::utils::{verim_keys, verim_pool, verim_ledger::auth};
use serde_json::Value;
use super::test;
use super::{logger, wallet, WalletHandle};
use indyrs::IndyError;

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

fn config(name: &str) -> String {
    json!({ "id": name }).to_string()
}

pub struct VerimSetup {
    pub name: String,
    pub pool_alias: String,
    pub key_alias: String,
    pub account_id: String,
    // Base58-encoded SEC1-encoded secp256k1 ECDSA key
    pub pub_key: String,
    pub wallet_handle: WalletHandle,
}

impl VerimSetup {
    pub fn new() -> VerimSetup {
        let name = setup();
        let config = config(&name);
        let (wallet_handle, _) = wallet::create_and_open_default_wallet(&config).unwrap();

        // Create account key
        let key_alias = "alice";
        let (account_id, pub_key) = VerimSetup::create_key(wallet_handle, key_alias, "alice").unwrap();

        // Pool
        verim_pool::add(&name, "http://localhost:26657", "verimnode");

        let setup = VerimSetup {
            name: name.clone(),
            pool_alias: name,
            key_alias: key_alias.to_string(),
            account_id,
            pub_key,
            wallet_handle
        };

        setup
    }

    pub fn create_key(wallet_handle: WalletHandle, alias: &str, mnemonic: &str) -> Result<(String, String), IndyError> {
        let key = verim_keys::add_from_mnemonic(wallet_handle, alias, mnemonic).unwrap();
        println!("Verim setup. Create key: {}", key);
        let key: Value = serde_json::from_str(&key).unwrap();
        Ok((key["account_id"].as_str().unwrap().to_string(), key["pub_key"].as_str().unwrap().to_string()))
    }

    pub fn get_base_account_number_and_sequence(&self, account_id: &str) -> Result<(u64, u64), IndyError> {
        let req = auth::build_query_account(account_id).unwrap();
        let resp = verim_pool::abci_query(&self.pool_alias, &req).unwrap();
        let resp = auth::parse_query_account_resp(&resp).unwrap();

        println!("Verim setup. Get account: {}", resp);

        let resp: Value = serde_json::from_str(&resp).unwrap();
        let base_account = resp["account"].as_object().unwrap()["base_account"].as_object().unwrap();
        let account_number = base_account["account_number"].as_u64().unwrap();
        let account_sequence = base_account["sequence"].as_u64().unwrap();

        Ok((account_number, account_sequence))
    }
}

impl Drop for VerimSetup {
    fn drop(&mut self) {
        tear_down(&self.name, self.wallet_handle);
    }
}
