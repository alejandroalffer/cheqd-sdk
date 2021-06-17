#[macro_use]
extern crate derivative;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

#[macro_use]
mod utils;

use utils::{verim_keys, verim_ledger, verim_pool, verim_setup};

mod high_cases {
    use super::*;

    #[cfg(test)]
    mod create_nym {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_verim_pool")]
        fn test_create_nym() {
            unimplemented!()
        }
    }


    #[cfg(test)]
    mod update_nym {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_verim_pool")]
        fn test_update_nym() {
            unimplemented!()
        }
    }

    #[cfg(test)]
    mod delete_nym {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_verim_pool")]
        fn test_delete_nym() {
            unimplemented!()
        }
    }

    #[cfg(test)]
    mod get_nym {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_verim_pool")]
        fn test_get_nym() {
            unimplemented!()
        }
    }

    #[cfg(test)]
    mod all_nym {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_verim_pool")]
        fn test_all_nym() {
            unimplemented!()
        }
    }

    // TODO: Remove
    #[cfg(test)]
    mod e2e {
        use super::*;
        use serde_json::Value;

        #[test]
        fn test_create_nym() {
            let setup = verim_setup::VerimSetup::new();
            ///// Transaction sending

            let (account_number, account_sequence) = setup.get_base_account_number_and_sequence(&setup.alice_account_id).unwrap();

            // Message
            let msg = verim_ledger::verim::build_msg_create_nym(
                "test-did",
                &setup.alice_account_id,
                "test-verkey",
                "test-alias",
                "test-role",
            )
            .unwrap();

            // Transaction
            let tx = verim_ledger::auth::build_tx(
                &setup.pool_alias, &setup.alice_key_alias, &msg, account_number, account_sequence, 300000, 0u64, "token", 39090, "memo",
            )
            .unwrap();

            // Signature
            let signed = verim_keys::sign(&setup.alice_key_alias, &tx).unwrap();

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

            assert!(true)
        }

        #[test]
        fn test_query_all_nym(){
            let setup = verim_setup::VerimSetup::new();
            ///// Transaction sending

            let (account_number, account_sequence) = setup.get_base_account_number_and_sequence(&setup.alice_account_id).unwrap();

            // Message #1
            let msg1 = verim_ledger::verim::build_msg_create_nym(
                "test-did-1",
                &setup.alice_account_id,
                "test-verkey-1",
                "test-alias-1",
                "test-role-1",
            ).unwrap();

            // Transaction #1
            let tx1 = verim_ledger::auth::build_tx(
                &setup.pool_alias, &setup.alice_key_alias, &msg1, account_number, account_sequence, 300000, 0u64, "token", 39090, "memo",
            ).unwrap();

            // Signature #1
            let signed1 = verim_keys::sign(&setup.alice_key_alias, &tx1).unwrap();

            // Broadcast #1
            let resp1 = verim_pool::broadcast_tx_commit(&setup.pool_alias, &signed1).unwrap();

            // Parse the response #1
            let tx_resp_parsed1 = verim_ledger::verim::parse_msg_create_nym_resp(&resp1).unwrap();
            println!("Tx response: {:?}", tx_resp_parsed1);
            let tx_resp1: Value = serde_json::from_str(&tx_resp_parsed1).unwrap();

            // Message #2
            let msg2 = verim_ledger::verim::build_msg_create_nym(
                "test-did-2",
                &setup.alice_account_id,
                "test-verkey-2",
                "test-alias-2",
                "test-role-2",
            ).unwrap();

            // Transaction #2
            let tx2 = verim_ledger::auth::build_tx(
                &setup.pool_alias, &setup.alice_key_alias, &msg2, account_number, account_sequence+1, 300000, 0u64, "token", 39090, "memo",
            ).unwrap();

            // Signature #2
            let signed2 = verim_keys::sign(&setup.alice_key_alias, &tx2).unwrap();

            // Broadcast #2
            let resp2 = verim_pool::broadcast_tx_commit(&setup.pool_alias, &signed2).unwrap();

            // Parse the response #2
            let tx_resp_parsed2 = verim_ledger::verim::parse_msg_create_nym_resp(&resp2).unwrap();
            println!("Tx response: {:?}", tx_resp_parsed2);
            let tx_resp2: Value = serde_json::from_str(&tx_resp_parsed2).unwrap();

            ///// Querying

            let query = verim_ledger::verim::build_query_all_nym().unwrap();

            let resp = verim_pool::abci_query(&setup.pool_alias, &query).unwrap();
            let resp = verim_ledger::verim::parse_query_all_nym_resp(&resp).unwrap();
            let resp: Value = serde_json::from_str(&resp).unwrap();

            println!("Query response: {:?}", resp);

            let nym_list = resp.get("nym").unwrap().as_array().unwrap();
            let expected_nym1 = &json!({"creator": &setup.alice_account_id,"id": account_sequence, "alias":"test-alias-1","verkey":"test-verkey-1","did":"test-did-1","role":"test-role-1"});
            let expected_nym2 = &json!({"creator": &setup.alice_account_id,"id": account_sequence+1, "alias":"test-alias-2","verkey":"test-verkey-2","did":"test-did-2","role":"test-role-2"});

            assert!(nym_list.contains(expected_nym1));
            assert!(nym_list.contains(expected_nym2));
        }

    }
}
