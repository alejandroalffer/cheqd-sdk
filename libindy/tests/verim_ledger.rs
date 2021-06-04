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
        fn test_build_query_all_nym() { verim_ledger::build_query_all_nym(); }
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
            let alice_account_id = alice["account_id"].as_str().unwrap();

            // Pool
            let pool_alias = "test_pool";
            let pool =
                cosmos_pool::add(pool_alias, "http://localhost:26657", "verimcosmos").unwrap();
            println!("Pool config: {}", pool);

            // Message #1
            let msg1 = verim_ledger::build_msg_create_nym(
                "test-did-1",
                alice_account_id,
                "test-verkey-1",
                "test-alias-1",
                "test-role-1",
            )
                .unwrap();

            // Transaction #1
            let tx1 = cosmos_pool::build_tx(
                pool_alias, "alice", &msg1, 9, 6, 300000, 0u64, "token", 39090, "memo",
            )
                .unwrap();

            // Signature #1
            let signed1 = cosmos_keys::sign("alice", &tx1).unwrap();

            // Broadcast #1
            let resp1 = cosmos_pool::broadcast_tx_commit(pool_alias, &signed1).unwrap();

            // Parse the response #1
            let tx_resp_parsed1 = verim_ledger::parse_msg_create_nym_resp(&resp1).unwrap();
            println!("Tx response: {:?}", tx_resp_parsed1);
            let tx_resp1: Value = serde_json::from_str(&tx_resp_parsed1).unwrap();

            // Message #2
            let msg2 = verim_ledger::build_msg_create_nym(
                "test-did-2",
                alice_account_id,
                "test-verkey-2",
                "test-alias-2",
                "test-role-2",
            )
                .unwrap();

            // Transaction #2
            let tx2 = cosmos_pool::build_tx(
                pool_alias, "alice", &msg2, 9, 7, 300000, 0u64, "token", 39090, "memo",
            )
                .unwrap();

            // Signature #2
            let signed2 = cosmos_keys::sign("alice", &tx2).unwrap();

            // Broadcast #2
            let resp2 = cosmos_pool::broadcast_tx_commit(pool_alias, &signed2).unwrap();

            // Parse the response #2
            let tx_resp_parsed2 = verim_ledger::parse_msg_create_nym_resp(&resp2).unwrap();
            println!("Tx response: {:?}", tx_resp_parsed2);
            let tx_resp2: Value = serde_json::from_str(&tx_resp_parsed2).unwrap();

            ///// Querying

            let query = verim_ledger::build_query_all_nym().unwrap();

            let resp = cosmos_pool::abci_query(pool_alias, &query).unwrap();
            let resp = verim_ledger::parse_query_all_nym_resp(&resp).unwrap();
            let resp: Value = serde_json::from_str(&resp).unwrap();

            println!("Query response: {:?}", resp);

            let nym_list = resp.get("nym").unwrap().as_array().unwrap();
            let expected_nym1 = &json!({"creator": alice_account_id,"id": tx_resp1.get("id").unwrap().as_i64().unwrap(), "alias":"test-alias-1","verkey":"test-verkey-1","did":"test-did-1","role":"test-role-1"});
            let expected_nym2 = &json!({"creator": alice_account_id,"id": tx_resp2.get("id").unwrap().as_i64().unwrap(), "alias":"test-alias-2","verkey":"test-verkey-2","did":"test-did-2","role":"test-role-2"});

            assert!(nym_list.contains(expected_nym1));
            assert!(nym_list.contains(expected_nym2));
        }

    }
}
