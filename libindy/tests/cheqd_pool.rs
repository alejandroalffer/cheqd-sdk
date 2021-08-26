#[macro_use]
extern crate derivative;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
mod utils;

use utils::{cheqd_keys, cheqd_pool, cheqd_setup, cheqd_ledger, test};
use serde_json::Value;

mod high_cases {
    use super::*;

    #[cfg(test)]
    mod add {
        use super::*;

        #[test]
        fn test_add() {
            let pool_name = "test_pool";
            let result = cheqd_pool::add(&pool_name, "rpc_address", "chain_id").unwrap();
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
            cheqd_pool::add(&pool_name, "rpc_address", "chain_id").unwrap();
            let result = cheqd_pool::get_config(&pool_name).unwrap();
            test::cleanup_storage(&pool_name);
            println!("Data: {:?} ", result);
        }
    }

    #[cfg(test)]
    mod broadcast_tx_commit {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_cheqd_pool")]
        fn test_broadcast_tx_commit() {
            let setup = cheqd_setup::CheqdSetup::new();

            let (account_number, account_sequence) = setup.get_base_account_number_and_sequence(&setup.account_id).unwrap();

            // Message
            let msg = cheqd_ledger::cheqd::build_msg_create_nym(
                "test-did",
                &setup.account_id,
                "test-verkey",
                "test-alias",
                "test-role",
            ).unwrap();

            // Transaction
            let tx = cheqd_ledger::auth::build_tx(
                &setup.pool_alias, &setup.pub_key, &msg, account_number, account_sequence, 300000, 0, "cheq", setup.get_timeout_height(), "memo",
            ).unwrap();

            // Sign
            let signed = cheqd_keys::sign(setup.wallet_handle, &setup.key_alias, &tx).unwrap();

            // Broadcast
            cheqd_pool::broadcast_tx_commit(&setup.pool_alias, &signed).unwrap();

            assert!(true);
        }
    }

    #[cfg(test)]
    mod abci_query {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_cheqd_pool")]
        fn test_abci_query() {
            let setup = cheqd_setup::CheqdSetup::new();
            ///// Transaction sending

            let (account_number, account_sequence) = setup.get_base_account_number_and_sequence(&setup.account_id).unwrap();

            // Message
            let msg = cheqd_ledger::cheqd::build_msg_create_nym(
                "test-did",
                &setup.account_id,
                "test-verkey",
                "test-alias",
                "test-role",
            ).unwrap();

            // Transaction
            let tx = cheqd_ledger::auth::build_tx(
                &setup.pool_alias, &setup.pub_key, &msg, account_number, account_sequence, 300000, 0u64, "cheq", setup.get_timeout_height(), "memo",
            ).unwrap();

            // Signature
            let signed = cheqd_keys::sign(setup.wallet_handle, &setup.key_alias, &tx).unwrap();

            // Broadcast
            let resp = cheqd_pool::broadcast_tx_commit(&setup.pool_alias, &signed).unwrap();

            // Parse the response
            let tx_resp_parsed = cheqd_ledger::cheqd::parse_msg_create_nym_resp(&resp).unwrap();
            println!("Tx response: {:?}", tx_resp_parsed);
            let tx_resp: Value = serde_json::from_str(&tx_resp_parsed).unwrap();

            ///// Querying

            let query = cheqd_ledger::cheqd::build_query_get_nym(tx_resp["id"].as_u64().unwrap()).unwrap();

            let query_resp = cheqd_pool::abci_query(&setup.pool_alias, &query).unwrap();
            let query_resp_parsed = cheqd_ledger::cheqd::parse_query_get_nym_resp(&query_resp).unwrap();
            println!("Query response: {:?}", query_resp_parsed);

            assert!(true);
        }
    }

    #[cfg(test)]
    mod abci_info {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_cheqd_pool")]
        fn test_abci_info() {
            let setup = cheqd_setup::CheqdSetup::new();
            let query_resp = cheqd_pool::abci_info(&setup.pool_alias).unwrap();
            println!("Query response: {:?}", query_resp);

            assert!(true);
        }
    }
}
