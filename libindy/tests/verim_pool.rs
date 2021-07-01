#[macro_use]
extern crate derivative;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
mod utils;

use utils::{verim_keys, verim_pool, verim_setup, verim_ledger, test};
use serde_json::Value;

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

            let (account_number, account_sequence) = setup.get_base_account_number_and_sequence(&setup.account_id).unwrap();

            // Message
            let msg = verim_ledger::verim::build_msg_create_nym(
                "test-did",
                &setup.account_id,
                "test-verkey",
                "test-alias",
                "test-role",
            ).unwrap();

            // Transaction
            let tx = verim_ledger::auth::build_tx(
                &setup.pool_alias, &setup.pub_key, &msg, account_number, account_sequence, 300000, 0, "token", 100000, "memo",
            ).unwrap();

            // Sign
            let signed = verim_keys::sign(setup.wallet_handle, &setup.key_alias, &tx).unwrap();

            // Broadcast
            verim_pool::broadcast_tx_commit(&setup.pool_alias, &signed).unwrap();

            assert!(true);
        }
    }

    #[cfg(test)]
    mod abci_query {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_verim_pool")]
        fn test_abci_query() {
            let setup = verim_setup::VerimSetup::new();
            ///// Transaction sending

            let (account_number, account_sequence) = setup.get_base_account_number_and_sequence(&setup.account_id).unwrap();

            // Message
            let msg = verim_ledger::verim::build_msg_create_nym(
                "test-did",
                &setup.account_id,
                "test-verkey",
                "test-alias",
                "test-role",
            ).unwrap();

            // Transaction
            let tx = verim_ledger::auth::build_tx(
                &setup.pool_alias, &setup.pub_key, &msg, account_number, account_sequence, 300000, 0u64, "token", 39090, "memo",
            ).unwrap();

            // Signature
            let signed = verim_keys::sign(setup.wallet_handle, &setup.key_alias, &tx).unwrap();

            // Broadcast
            let resp = verim_pool::broadcast_tx_commit(&setup.pool_alias, &signed).unwrap();

            // Parse the response
            let tx_resp_parsed = verim_ledger::verim::parse_msg_create_nym_resp(&resp).unwrap();
            println!("Tx response: {:?}", tx_resp_parsed);
            let tx_resp: Value = serde_json::from_str(&tx_resp_parsed).unwrap();

            ///// Querying

            let query = verim_ledger::verim::build_query_get_nym(tx_resp["id"].as_u64().unwrap()).unwrap();

            let query_resp = verim_pool::abci_query(&setup.pool_alias, &query).unwrap();
            let query_resp_parsed = verim_ledger::verim::parse_query_get_nym_resp(&query_resp).unwrap();
            println!("Query response: {:?}", query_resp_parsed);

            assert!(true);
        }
    }

    #[cfg(test)]
    mod abci_info {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_verim_pool")]
        fn test_abci_info() {
            let setup = verim_setup::VerimSetup::new();
            let query_resp = verim_pool::abci_info(&setup.pool_alias).unwrap();
            println!("Query response: {:?}", query_resp);

            assert!(true);
        }
    }
}
