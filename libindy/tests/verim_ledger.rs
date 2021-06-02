#[macro_use]
extern crate derivative;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

#[macro_use]
mod utils;

use utils::{constants::*, cosmos_keys, cosmos_pool, verim_ledger};

mod high_cases {
    use super::*;

    #[cfg(test)]
    mod verim_ledger_transactions {
        use super::*;

        #[test]
        fn build_msg_create_nym() {
            let did = "0";
            let creator = "1";
            let role = "some_role";
            let alias = "some_alias";
            let verkey = "some_verkey";
            verim_ledger::build_msg_create_nym(did, creator, verkey, alias, role);
        }

        #[test]
        fn build_msg_update_nym() {
            let did = "0";
            let creator = "1";
            let role = "some_role";
            let alias = "some_alias";
            let verkey = "some_verkey";
            let id = 2;
            verim_ledger::build_msg_update_nym(did, creator, verkey, alias, role, id);
        }

        #[test]
        fn build_msg_delete_nym() {
            let creator = "1";
            let id = 2;
            verim_ledger::build_msg_delete_nym(creator, id);
        }
    }

    #[cfg(test)]
    mod verim_ledger_queries {
        use super::*;

        #[test]
        fn test_build_query_get_nym() {
            verim_ledger::build_query_get_nym(1);
        }

        #[test]
        fn test_build_query_all_nym() {
            println!("QQQQ {:?}", verim_ledger::build_query_all_nym());
        }
    }

    #[cfg(test)]
    mod e2e {
        use super::*;
        use serde_json::Value;

        #[test]
        fn test_create_nym() {
            ///// Transaction sending

            // Create key
            let alice = cosmos_keys::add_from_mnemonic("alice", "alice").unwrap();
            println!("Alice's account: {}", alice);
            let alice: Value = serde_json::from_str(&alice).unwrap();

            // Pool
            let pool_alias = "test_pool";
            let pool =
                cosmos_pool::add(pool_alias, "http://localhost:26657", "verimcosmos").unwrap();
            println!("Pool config: {}", pool);

            // Message
            let msg = verim_ledger::build_msg_create_nym(
                "test-did",
                alice["account_id"].as_str().unwrap(),
                "test-verkey",
                "test-alias",
                "test-role",
            )
            .unwrap();

            // Transaction
            let tx = cosmos_pool::build_tx(
                pool_alias, "alice", &msg, 9, 4, 300000, 0u64, "token", 39090, "memo",
            )
            .unwrap();

            // Signature
            let signed = cosmos_keys::sign("alice", &tx).unwrap();

            // Broadcast
            let resp = cosmos_pool::broadcast_tx_commit(pool_alias, &signed).unwrap();

            // Parse the response
            let tx_resp_parsed = verim_ledger::parse_msg_create_nym_resp(&resp).unwrap();
            println!("Tx response: {:?}", tx_resp_parsed);
            let tx_resp: Value = serde_json::from_str(&tx_resp_parsed).unwrap();

            ///// Querying

            let query = verim_ledger::build_query_get_nym(tx_resp["id"].as_u64().unwrap()).unwrap();

            let query_resp = cosmos_pool::abci_query(pool_alias, &query).unwrap();
            let query_resp_parsed = verim_ledger::parse_query_get_nym_resp(&query_resp).unwrap();

            println!("Query response: {:?}", query_resp_parsed);

            assert!(true)
        }

        #[test]
        fn test_query_all_nym(){
            ///// Transaction sending

            // Create key
            let alice = cosmos_keys::add_from_mnemonic("alice", "alice").unwrap();
            println!("Alice's account: {}", alice);
            let alice: Value = serde_json::from_str(&alice).unwrap();

            // Pool
            let pool_alias = "test_pool";
            let pool =
                cosmos_pool::add(pool_alias, "http://localhost:26657", "verimcosmos").unwrap();
            println!("Pool config: {}", pool);

            // Message #1
            let msg = verim_ledger::build_msg_create_nym(
                "test-did-1",
                alice["account_id"].as_str().unwrap(),
                "test-verkey-1",
                "test-alias-1",
                "test-role-1",
            )
                .unwrap();

            // Transaction #1
            let tx = cosmos_pool::build_tx(
                pool_alias, "alice", &msg, 9, 10, 300000, 0u64, "token", 39090, "memo",
            )
                .unwrap();

            // Signature #1
            let signed = cosmos_keys::sign("alice", &tx).unwrap();

            // Broadcast #1
            let resp = cosmos_pool::broadcast_tx_commit(pool_alias, &signed).unwrap();

            // Parse the response #1
            let tx_resp_parsed = verim_ledger::parse_msg_create_nym_resp(&resp).unwrap();
            println!("Tx response: {:?}", tx_resp_parsed);
            let tx_resp: Value = serde_json::from_str(&tx_resp_parsed).unwrap();

            // Message #2
            let msg = verim_ledger::build_msg_create_nym(
                "test-did-2",
                alice["account_id"].as_str().unwrap(),
                "test-verkey-2",
                "test-alias-2",
                "test-role-2",
            )
                .unwrap();

            // Transaction #2
            let tx = cosmos_pool::build_tx(
                pool_alias, "alice", &msg, 9, 11, 300000, 0u64, "token", 39090, "memo",
            )
                .unwrap();

            // Signature #2
            let signed = cosmos_keys::sign("alice", &tx).unwrap();

            // Broadcast #2
            let resp = cosmos_pool::broadcast_tx_commit(pool_alias, &signed).unwrap();

            // Parse the response #2
            let tx_resp_parsed = verim_ledger::parse_msg_create_nym_resp(&resp).unwrap();
            println!("Tx response: {:?}", tx_resp_parsed);
            let tx_resp: Value = serde_json::from_str(&tx_resp_parsed).unwrap();

            ///// Querying

            let query = verim_ledger::build_query_all_nym().unwrap();

            let query_resp = cosmos_pool::abci_query(pool_alias, &query).unwrap();
            let query_resp_parsed = verim_ledger::parse_query_all_nym_resp(&query_resp).unwrap();

            println!("Query response: {:?}", query_resp_parsed);

            assert!(true)
        }



    }
}
