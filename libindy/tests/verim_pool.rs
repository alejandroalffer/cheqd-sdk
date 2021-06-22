#[macro_use]
extern crate derivative;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
mod utils;

use indyrs::ErrorCode;
use utils::{constants::*, verim_keys, verim_pool, verim_setup, verim_ledger, test, wallet, types::ResponseType};
use rand::prelude::*;
use rand::Rng;

mod high_cases {
    use super::*;

    #[cfg(test)]
    mod add {
        use super::*;

        #[test]
        fn test_add() {
            let pool_name = "test_pool";
            let result = verim_pool::add(&pool_name, "rpc_address", "chain_id").unwrap();
            test::cleanup_storage(&pool_name);
            println!("Data: {:?} ", result);
        }
    }

    #[cfg(test)]
    mod get_config {
        use super::*;

        #[test]
        fn test_get_config() {
            let pool_name = "test_pool";
            verim_pool::add(&pool_name, "rpc_address", "chain_id").unwrap();
            let result = verim_pool::get_config(&pool_name).unwrap();
            test::cleanup_storage(&pool_name);
            println!("Data: {:?} ", result);
        }
    }

    #[cfg(test)]
    mod broadcast_tx_commit {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_verim_pool")]
        fn test_broadcast_tx_commit() {
            let setup = verim_setup::VerimSetup::new();

            let (account_number, account_sequence) = setup.get_base_account_number_and_sequence(&setup.alice_account_id).unwrap();

            // Message
            let msg = verim_ledger::verim::build_msg_create_nym(
                "test-did",
                &setup.alice_account_id,
                "test-verkey",
                "test-alias",
                "test-role",
            ).unwrap();

            // Transaction
            let tx = verim_ledger::auth::build_tx(
                &setup.pool_alias, &setup.alice_key_alias, &msg, account_number, account_sequence, 300000, 0, "token", 100000, "memo",
            ).unwrap();

            //Wallet
            let (wallet_handle, _) = wallet::create_and_open_default_wallet(&config).unwrap();

            // Sign
            let signed = verim_keys::sign(wallet_handle, &setup.alice_key_alias, &tx).unwrap();

            // Broadcast
            let resp = verim_pool::broadcast_tx_commit(&setup.pool_alias, &signed).unwrap();

            assert!(true);
        }
    }

    #[cfg(test)]
    mod abci_query {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_verim_pool")]
        fn test_abci_query() {
            unimplemented!()
        }
    }

    #[cfg(test)]
    mod abci_info {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_verim_pool")]
        fn test_abci_info() {
            unimplemented!()
        }
    }
}
