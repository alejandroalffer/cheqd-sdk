#[macro_use]
extern crate derivative;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

#[macro_use]
mod utils;

use utils::{verim_keys, verim_ledger, verim_pool, verim_setup};
use serde_json::Value;

mod high_cases {
    use super::*;

    #[cfg(test)]
    mod create_nym {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_verim_pool")]
        fn test_create_nym() {
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

            let result: Value = serde_json::from_str(&query_resp_parsed).unwrap();
            let expected_result: Value = json!({
                "nym": {
                    "creator": setup.account_id,
                    "id": tx_resp["id"].as_u64().unwrap(),
                    "alias": "test-alias",
                    "verkey": "test-verkey",
                    "did": "test-did",
                    "role": "test-role",
                }
            });

            assert_eq!(expected_result, result);
        }
    }

    #[cfg(test)]
    mod update_nym {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_verim_pool")]
        fn test_update_nym() {
            let setup = verim_setup::VerimSetup::new();
            ///// Transaction sending

            let (account_number, account_sequence) = setup.get_base_account_number_and_sequence(&setup.account_id).unwrap();

            // Message
            let msg1 = verim_ledger::verim::build_msg_create_nym(
                "test-did",
                &setup.account_id,
                "test-verkey",
                "test-alias",
                "test-role",
            ).unwrap();

            // Transaction
            let tx1 = verim_ledger::auth::build_tx(
                &setup.pool_alias, &setup.pub_key, &msg1, account_number, account_sequence, 300000, 0u64, "token", 39090, "memo",
            ).unwrap();

            // Signature
            let signed1 = verim_keys::sign(setup.wallet_handle, &setup.key_alias, &tx1).unwrap();

            // Broadcast
            let resp1 = verim_pool::broadcast_tx_commit(&setup.pool_alias, &signed1).unwrap();

            // Parse the response
            let tx_resp_parsed1 = verim_ledger::verim::parse_msg_create_nym_resp(&resp1).unwrap();
            println!("Old tx response: {:?}", tx_resp_parsed1);
            let tx_resp1: Value = serde_json::from_str(&tx_resp_parsed1).unwrap();

            // Message for updating
            let msg2 = verim_ledger::verim::build_msg_update_nym(
                "test-did-update",
                &setup.account_id,
                "test-verkey-update",
                "test-alias-update",
                "test-role-update",
                tx_resp1["id"].as_u64().unwrap()
            ).unwrap();

            // Transaction for updating
            let tx2 = verim_ledger::auth::build_tx(
                &setup.pool_alias, &setup.pub_key, &msg2, account_number, account_sequence+1, 300000, 0u64, "token", 39090, "memo",
            ).unwrap();

            // Signature for updating
            let signed2 = verim_keys::sign(setup.wallet_handle, &setup.key_alias, &tx2).unwrap();

            // Broadcast for updating
            let resp2 = verim_pool::broadcast_tx_commit(&setup.pool_alias, &signed2).unwrap();

            // Parse the response of updating
            let tx_resp_parsed2 = verim_ledger::verim::parse_msg_update_nym_resp(&resp2).unwrap();
            println!("New tx response: {:?}", tx_resp_parsed2);

            assert_eq!("{}", tx_resp_parsed2);
        }
    }

    #[cfg(test)]
    mod delete_nym {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_verim_pool")]
        fn test_delete_nym() {
            let setup = verim_setup::VerimSetup::new();
            ///// Transaction sending

            let (account_number, account_sequence) = setup.get_base_account_number_and_sequence(&setup.account_id).unwrap();

            // Message
            let msg1 = verim_ledger::verim::build_msg_create_nym(
                "test-did",
                &setup.account_id,
                "test-verkey",
                "test-alias",
                "test-role",
            ).unwrap();

            // Transaction
            let tx1 = verim_ledger::auth::build_tx(
                &setup.pool_alias, &setup.pub_key, &msg1, account_number, account_sequence, 300000, 0u64, "token", 39090, "memo",
            ).unwrap();

            // Signature
            let signed1 = verim_keys::sign(setup.wallet_handle, &setup.key_alias, &tx1).unwrap();

            // Broadcast
            let resp1 = verim_pool::broadcast_tx_commit(&setup.pool_alias, &signed1).unwrap();

            // Parse the response
            let tx_resp_parsed1 = verim_ledger::verim::parse_msg_create_nym_resp(&resp1).unwrap();
            println!("Old tx response: {:?}", tx_resp_parsed1);
            let tx_resp1: Value = serde_json::from_str(&tx_resp_parsed1).unwrap();

            // Message for updating
            let msg2 = verim_ledger::verim::build_msg_delete_nym(
                &setup.account_id, tx_resp1["id"].as_u64().unwrap()
            ).unwrap();

            // Transaction for updating
            let tx2 = verim_ledger::auth::build_tx(
                &setup.pool_alias, &setup.pub_key, &msg2, account_number, account_sequence+1, 300000, 0u64, "token", 39090, "memo",
            ).unwrap();

            // Signature for updating
            let signed2 = verim_keys::sign(setup.wallet_handle, &setup.key_alias, &tx2).unwrap();

            // Broadcast for updating
            let resp2 = verim_pool::broadcast_tx_commit(&setup.pool_alias, &signed2).unwrap();

            // Parse the response of updating
            let tx_resp_parsed2 = verim_ledger::verim::parse_msg_delete_nym_resp(&resp2).unwrap();
            println!("New tx response: {:?}", tx_resp_parsed2);
            assert_eq!("{}", tx_resp_parsed2);
        }
    }

    #[cfg(test)]
    mod get_nym {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_verim_pool")]
        fn test_get_nym() {
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

            let result: Value = serde_json::from_str(&query_resp_parsed).unwrap();
            let expected_result: Value = json!({
                "nym": {
                    "creator": setup.account_id,
                    "id": tx_resp["id"].as_u64().unwrap(),
                    "alias": "test-alias",
                    "verkey": "test-verkey",
                    "did": "test-did",
                    "role": "test-role",
                }
            });

            assert_eq!(expected_result, result);
        }
    }

    #[cfg(test)]
    mod all_nym {
        use super::*;

        #[test]
        #[cfg(feature = "local_nodes_verim_pool")]
        fn test_all_nym() {
            let setup = verim_setup::VerimSetup::new();
            ///// Transaction sending

            let (account_number, account_sequence) = setup.get_base_account_number_and_sequence(&setup.account_id).unwrap();

            // First message
            let msg = verim_ledger::verim::build_msg_create_nym(
                "test-did-1",
                &setup.account_id,
                "test-verkey-1",
                "test-alias-1",
                "test-role-1",
            ).unwrap();

            // First transaction
            let tx = verim_ledger::auth::build_tx(
                &setup.pool_alias, &setup.pub_key, &msg, account_number, account_sequence, 300000, 0u64, "token", 39090, "memo",
            ).unwrap();

            // First signature
            let signed = verim_keys::sign(setup.wallet_handle, &setup.key_alias, &tx).unwrap();

            // First broadcast
            let resp = verim_pool::broadcast_tx_commit(&setup.pool_alias, &signed).unwrap();

            // First parse the response
            let tx_resp_parsed = verim_ledger::verim::parse_msg_create_nym_resp(&resp).unwrap();
            println!("Tx response: {:?}", tx_resp_parsed);
            let tx_resp1: Value = serde_json::from_str(&tx_resp_parsed).unwrap();

            // Second message
            let msg = verim_ledger::verim::build_msg_create_nym(
                "test-did-2",
                &setup.account_id,
                "test-verkey-2",
                "test-alias-2",
                "test-role-2",
            ).unwrap();

            // Second transaction
            let tx = verim_ledger::auth::build_tx(
                &setup.pool_alias, &setup.pub_key, &msg, account_number, account_sequence+1, 300000, 0u64, "token", 39090, "memo",
            ).unwrap();

            // Second signature
            let signed = verim_keys::sign(setup.wallet_handle, &setup.key_alias, &tx).unwrap();

            // Second broadcast
            let resp = verim_pool::broadcast_tx_commit(&setup.pool_alias, &signed).unwrap();

            // Second parse the response
            let tx_resp_parsed = verim_ledger::verim::parse_msg_create_nym_resp(&resp).unwrap();
            println!("Tx response: {:?}", tx_resp_parsed);
            let tx_resp2: Value = serde_json::from_str(&tx_resp_parsed).unwrap();

            ///// Querying

            let query = verim_ledger::verim::build_query_all_nym().unwrap();
            let query_resp = verim_pool::abci_query(&setup.pool_alias, &query).unwrap();
            let query_resp_parsed = verim_ledger::verim::parse_query_all_nym_resp(&query_resp).unwrap();
            println!("Query response: {:?}", query_resp_parsed);

            let result: Value = serde_json::from_str(&query_resp_parsed).unwrap();
            let result_nym = result["nym"].as_array().unwrap();
            let result_pag = result["pagination"].as_object().unwrap();

            let expected_result1: Value = json!({
                    "creator": setup.account_id,
                    "id": tx_resp1["id"].as_u64().unwrap(),
                    "alias": "test-alias-1",
                    "verkey": "test-verkey-1",
                    "did": "test-did-1",
                    "role": "test-role-1",
            });

            let expected_result2: Value = json!({
                    "creator": setup.account_id,
                    "id": tx_resp2["id"].as_u64().unwrap(),
                    "alias": "test-alias-2",
                    "verkey": "test-verkey-2",
                    "did": "test-did-2",
                    "role": "test-role-2",
            });

            assert!(result_nym.contains(&expected_result1));
            assert!(result_nym.contains(&expected_result2));
        }
    }
}
