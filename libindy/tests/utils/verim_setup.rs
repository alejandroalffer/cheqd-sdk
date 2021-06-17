#![allow(dead_code, unused_macros)]
use crate::utils::{verim_keys, verim_pool, auth, verim_ledger};
use indy_api_types::errors::IndyResult;
use indyrs::{
    ErrorCode, IndyError, PoolHandle, WalletHandle, INVALID_POOL_HANDLE, INVALID_WALLET_HANDLE,
};
use serde_json::Value;
use std::{borrow::Borrow, future::Future};
use super::environment;
use super::test;

fn setup() -> String {
    let name = crate::utils::rand_utils::get_rand_string(10);
    test::cleanup_storage(&name);
    name
}

fn tear_down(name: &str) {
    test::cleanup_storage(name);
}

pub struct VerimSetup {
    pub name: String,
    pub pool_alias: String,
    pub alice_key_alias: String,
    pub alice_account_id: String,
    pub bob_key_alias: String,
    pub bob_account_id: String,
}

impl VerimSetup {
    pub fn new() -> VerimSetup {
        let name = setup();

        // Create Alice key
        let alice_alias = "alice";
        let alice_account_address = VerimSetup::create_key(alice_alias, "alice").unwrap();

        // Create Bob key
        let bob_alias = "bob";
        let bob_account_address = VerimSetup::create_key(bob_alias, "bob").unwrap();

        let setup = VerimSetup {
            name,
            pool_alias: "test_pool".to_string(),
            alice_key_alias: alice_alias.to_string(),
            alice_account_id: alice_account_address.to_string(),
            bob_key_alias: bob_alias.to_string(),
            bob_account_id: bob_account_address.to_string(),
        };

        // Pool
        let pool_alias = "test_pool";
        let pool = verim_pool::add(pool_alias, "http://localhost:26657", "verim").unwrap();
        println!("Verim setup. Pool config: {}", pool);

        setup
    }

    pub fn create_key(alias: &str, mnemonic: &str) -> Result<String, IndyError> {
        let key = verim_keys::add_from_mnemonic(alias, mnemonic).unwrap();
        println!("Verim setup. Create key: {}", key);
        let key: Value = serde_json::from_str(&key).unwrap();
        Ok(key["account_id"].as_str().unwrap().to_string())
    }

    pub fn get_base_account_number_and_sequence(&self, account_id: &str) -> Result<(u64, u64), IndyError> {
        let req = auth::build_query_cosmos_auth_account(account_id).unwrap();
        let resp = verim_pool::abci_query(&self.pool_alias, &req).unwrap();
        let resp = auth::parse_query_cosmos_auth_account_resp(&resp).unwrap();

        println!("Get account: {}", resp);

        let resp: Value = serde_json::from_str(&resp).unwrap();
        let base_account = resp["account"].as_object().unwrap()["base_account"].as_object().unwrap();
        let account_number = base_account["account_number"].as_u64().unwrap();
        let account_sequence = base_account["sequence"].as_u64().unwrap();

        Ok((account_number, account_sequence))
    }
}

impl Drop for VerimSetup {
    fn drop(&mut self) {
        // if self.wallet_handle != INVALID_WALLET_HANDLE {
        //     wallet::close_and_delete_wallet(self.wallet_handle, &self.wallet_config).unwrap();
        // }
        //
        // if self.pool_handle != INVALID_POOL_HANDLE {
        //     pool::close(self.pool_handle).unwrap();
        // }

        tear_down(&self.name);
    }
}
