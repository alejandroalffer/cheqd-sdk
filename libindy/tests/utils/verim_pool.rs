#![allow(dead_code, unused_macros)]

use indyrs::{ErrorCode, PoolHandle, WalletHandle, INVALID_POOL_HANDLE, INVALID_WALLET_HANDLE};

pub mod callback;

#[path = "../../indy-utils/src/environment.rs"]
pub mod environment;

pub mod anoncreds;
pub mod blob_storage;
pub mod constants;
pub mod crypto;
pub mod did;
pub mod ledger;
pub mod non_secrets;
pub mod pairwise;
pub mod pool;
pub mod results;
pub mod types;
pub mod wallet;
//pub mod payments;
pub mod cache;
pub mod logger;
pub mod rand_utils;
pub mod metrics;
pub mod verim_ledger;

#[macro_use]
#[allow(unused_macros)]
#[path = "../../indy-utils/src/test.rs"]
pub mod test;

pub mod timeout;

#[path = "../../indy-utils/src/sequence.rs"]
pub mod sequence;

#[macro_use]
#[allow(unused_macros)]
#[path = "../../indy-utils/src/ctypes.rs"]
pub mod ctypes;

#[macro_use]
#[path = "../../src/utils/qualifier.rs"]
pub mod qualifier;

#[path = "../../indy-utils/src/inmem_wallet.rs"]
pub mod inmem_wallet;

#[path = "../../indy-utils/src/wql.rs"]
pub mod wql;

#[path = "../../src/domain/mod.rs"]
pub mod domain;

fn setup() -> String {
    let name = crate::utils::rand_utils::get_rand_string(10);
    test::cleanup_storage(&name);
    logger::set_default_logger();
    name
}

fn tear_down(name: &str) {
    test::cleanup_storage(name);
}

pub struct VerimSetup {
    pub name: String,
    pub did: String,
    pub verkey: String,
}

impl VerimSetup {
    pub fn empty() -> VerimSetup {
        let name = setup();
        VerimSetup {
            name,
            did: String::new(),
            verkey: String::new(),
        }
    }

    pub fn wallet() -> VerimSetup {
        let name = setup();
        let (wallet_handle, wallet_config) = wallet::create_and_open_default_wallet(&name).unwrap();
        VerimSetup {
            name,
            did: String::new(),
            verkey: String::new(),
        }
    }

    // pub fn plugged_wallet() -> Setup {
    //     let name = setup();
    //     let (wallet_handle, wallet_config) = wallet::create_and_open_plugged_wallet().unwrap();
    //     Setup {
    //         name,
    //         wallet_config,
    //         wallet_handle,
    //         pool_handle: INVALID_POOL_HANDLE,
    //         did: String::new(),
    //         verkey: String::new(),
    //     }
    // }

    pub fn pool() -> VerimSetup {
        let name = setup();
        let pool_handle = pool::create_and_open_pool_ledger(&name).unwrap();
        VerimSetup {
            name,
            did: String::new(),
            verkey: String::new(),
        }
    }

    pub fn wallet_and_pool() -> VerimSetup {
        let name = setup();
        let (wallet_handle, wallet_config) = wallet::create_and_open_default_wallet(&name).unwrap();
        let pool_handle = pool::create_and_open_pool_ledger(&name).unwrap();
        VerimSetup {
            name,
            did: String::new(),
            verkey: String::new(),
        }
    }

    pub fn new_identity() -> VerimSetup {
        let mut setup = VerimSetup::wallet_and_pool();
        let (did, verkey) = did::create_store_and_publish_did(
            setup.wallet_handle,
            setup.pool_handle,
            "TRUSTEE",
            None,
        )
            .unwrap();
        setup.did = did;
        setup.verkey = verkey;
        setup
    }

    pub fn did() -> VerimSetup {
        let name = setup();
        let (wallet_handle, wallet_config) = wallet::create_and_open_default_wallet(&name).unwrap();
        let (did, verkey) = did::create_and_store_my_did(wallet_handle, None).unwrap();
        VerimSetup {
            name,
            wallet_config,
            wallet_handle,
            pool_handle: 0,
            did,
            verkey,
        }
    }
}

impl Drop for VerimSetup {
    fn drop(&mut self) {
        if self.wallet_handle != INVALID_WALLET_HANDLE {
            wallet::close_and_delete_wallet(self.wallet_handle, &self.wallet_config).unwrap();
        }

        if self.pool_handle != INVALID_POOL_HANDLE {
            pool::close(self.pool_handle).unwrap();
        }

        tear_down(&self.name);
    }
}
